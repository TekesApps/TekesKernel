#!/usr/bin/env python3
from __future__ import annotations

import json
import hashlib
import plistlib
import re
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
FIX = ROOT / "fixtures/deployment"
SAFE = 9_007_199_254_740_991
EXITS = {0, 64, 65, 66, 69, 74, 75}
READINESS = {
    "boot-incomplete": "Startup has not completed",
    "server-draining": "Endpoint is draining",
}
BOOTSTRAP = {
    "already-running": "Storage root already has an owner",
    "invalid-install": "Installed files are invalid",
    "unsupported-filesystem": "Storage filesystem is unsupported",
    "invalid-config": "Configuration is invalid",
    "corrupt-ledger": "Durable state is corrupt",
    "protocol-mismatch": "Protocol compatibility check failed",
    "required-broker-unavailable": "Required broker is unavailable",
    "selector-mismatch": "Selected build does not match process attribution",
    "listener-unavailable": "Fixed loopback listener is unavailable",
    "endpoint-credential-unavailable": "Endpoint credential is unavailable",
    "io": "Bootstrap I/O failed",
}
GATE_MAP = {
    71: {"clean-install", "duplicate-instance", "unsupported-filesystem", "cloud-managed-storage", "corrupt-tail", "locked-config"},
    72: {"reboot-start", "reboot-target-user-login", "selector-sigkill-recovery", "live-drain"},
    73: {"publication-power-loss", "first-activation-power-loss", "selector-update-power-loss", "upgrade-downgrade-gate", "installer-install-recovery"},
    74: {"crash-loop-rollback", "non-attributable-failure-matrix", "promotion-window-boundary"},
    75: {"observability-redaction"},
    76: {"archive-preservation", "uninstall-retains-data", "installer-uninstall-recovery"},
}


class Bad(Exception):
    pass


def pairs(items):
    result = {}
    for key, value in items:
        if key in result:
            raise Bad(f"duplicate key {key}")
        result[key] = value
    return result


def canonical(value: Any) -> bytes:
    return json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode()


def read(path: Path, allow_lf: bool = False) -> Any:
    raw = path.read_bytes()
    if allow_lf:
        if not raw.endswith(b"\n") or raw.endswith(b"\n\n"):
            raise Bad(f"{path}: canonical-plus-LF artifact must have exactly one final LF")
        body = raw[:-1]
    else:
        body = raw
    try:
        value = json.loads(body, object_pairs_hook=pairs, parse_constant=lambda value: (_ for _ in ()).throw(Bad(value)))
    except (json.JSONDecodeError, UnicodeDecodeError, Bad) as error:
        raise Bad(f"{path}: invalid JSON: {error}") from error
    if body != canonical(value):
        raise Bad(f"{path}: not RFC-8785 canonical for this integer/string corpus")
    check_json(value, str(path))
    return value


def check_json(value: Any, where: str) -> None:
    if value is None or isinstance(value, (str, bool)):
        return
    if isinstance(value, int) and not isinstance(value, bool):
        if abs(value) > SAFE:
            raise Bad(f"{where}: unsafe integer")
        return
    if isinstance(value, float):
        raise Bad(f"{where}: floating oracle value is not frozen")
    if isinstance(value, list):
        for index, item in enumerate(value):
            check_json(item, f"{where}[{index}]")
        return
    if isinstance(value, dict):
        for key, item in value.items():
            if not isinstance(key, str):
                raise Bad(f"{where}: non-string key")
            check_json(item, f"{where}.{key}")
        return
    raise Bad(f"{where}: unsupported JSON type")


def selection(value: Any, where: str) -> None:
    if not isinstance(value, dict) or set(value) not in ({"generation"}, {"current", "generation"}, {"current", "previous", "generation"}):
        raise Bad(f"{where}: bad selection fields")
    if not isinstance(value["generation"], int) or value["generation"] < 0:
        raise Bad(f"{where}: bad generation")
    for key in ("current", "previous"):
        if key in value and (not isinstance(value[key], str) or not value[key]):
            raise Bad(f"{where}: bad {key}")


def readiness(value: Any, where: str) -> None:
    if not isinstance(value, dict) or set(value) not in ({"ready"}, {"ready", "code"}):
        raise Bad(f"{where}: bad readiness fields")
    if not isinstance(value["ready"], bool):
        raise Bad(f"{where}: ready is not bool")
    if value["ready"] and "code" in value:
        raise Bad(f"{where}: ready state has error code")
    closed_codes = set(READINESS) | set(BOOTSTRAP) | {
        "launch-failed", "readiness-crash-loop", "service-absent"
    }
    if not value["ready"] and value.get("code") not in closed_codes:
        raise Bad(f"{where}: unknown readiness code")


def validate_setup(value: Any, case: str, gate: int) -> None:
    if set(value) != {"format", "case", "gate", "platform", "initial"} or value["format"] != 1 or value["case"] != case or value["gate"] != gate:
        raise Bad(f"{case}: setup envelope")
    platform = value["platform"]
    if set(platform) != {"os", "minimum", "architecture", "filesystem"} or platform["os"] != "macos" or platform["minimum"] != "15.0" or platform["architecture"] != "aarch64":
        raise Bad(f"{case}: platform mismatch")
    initial = value["initial"]
    allowed = {"current", "previous", "generation", "service", "threads", "observation"}
    if not isinstance(initial, dict) or not set(initial) <= allowed or not {"generation", "service", "threads"} <= set(initial):
        raise Bad(f"{case}: initial shape")
    selection({key: initial[key] for key in ("current", "previous", "generation") if key in initial}, f"{case}.initial")
    if initial["service"] not in {"absent", "stopped", "running"} or not isinstance(initial["threads"], list) or len(initial["threads"]) != len(set(initial["threads"])):
        raise Bad(f"{case}: initial service/threads")
    if "observation" in initial and initial["observation"] not in {"candidate-pending", "rollback-window", "promoted"}:
        raise Bad(f"{case}: initial observation")


def validate_steps(value: Any, case: str) -> None:
    keys = {"command", "fault_points", "expected_exit", "expected_selection", "expected_readiness"}
    if set(value) != keys:
        raise Bad(f"{case}: steps fields")
    commands = value["command"]
    exits = value["expected_exit"]
    if not isinstance(commands, list) or not commands or len(commands) != len(exits):
        raise Bad(f"{case}: commands/exits do not align")
    if any(not isinstance(argv, list) or not argv or not all(isinstance(part, str) and part for part in argv) for argv in commands):
        raise Bad(f"{case}: command is not argv")
    if any(code not in EXITS for code in exits):
        raise Bad(f"{case}: exit outside closed taxonomy")
    faults = value["fault_points"]
    if not isinstance(faults, list) or len(faults) != len(set(faults)) or not all(isinstance(item, str) and item for item in faults):
        raise Bad(f"{case}: fault points")
    selection(value["expected_selection"], f"{case}.steps.selection")
    readiness(value["expected_readiness"], f"{case}.steps.readiness")


def validate_expected(value: Any, case: str, gate: int) -> None:
    keys = {"format", "case", "gate", "selection", "readiness", "filesystem", "processes", "preserved", "observability"}
    if set(value) != keys or value["format"] != 1 or value["case"] != case or value["gate"] != gate:
        raise Bad(f"{case}: expected envelope")
    selection(value["selection"], f"{case}.expected.selection")
    readiness(value["readiness"], f"{case}.expected.readiness")
    fs = value["filesystem"]
    if not isinstance(fs, dict) or set(fs) != {"present", "absent", "modes"} or set(fs["present"]) & set(fs["absent"]):
        raise Bad(f"{case}: filesystem result")
    if any(mode not in {"0600", "0644", "0700", "0755"} for mode in fs["modes"].values()):
        raise Bad(f"{case}: invalid mode")
    processes = value["processes"]
    if set(processes) != {"selectors", "supervisors", "listeners", "orphans"} or any(not isinstance(v, int) or v < 0 for v in processes.values()):
        raise Bad(f"{case}: process result")
    obs = value["observability"]
    if set(obs) != {"codes", "forbidden"} or not isinstance(obs["codes"], list) or not isinstance(obs["forbidden"], list):
        raise Bad(f"{case}: observability result")


def main() -> int:
    registry = read(FIX / "cases.canonical.json", allow_lf=True)
    declared = set(registry["macos"])
    if len(declared) != 22 or declared != set().union(*GATE_MAP.values()):
        raise Bad("registry and Gate 71-76 map disagree")
    top_expected = {
        "README.md", "COVERAGE.md", "cases.canonical.json",
        "selector.canonical.json", "selector-current.canonical.json",
        "selector-previous.canonical.json", "bundle-manifest.canonical.json",
        "selector-manifest.canonical.json", "install-layout.canonical.json",
        "compatibility.canonical.json", "authority-registry.canonical.json",
        "endpoint-credential.canonical.json",
        "install-identity.canonical.json", "signing-identity.canonical.json",
        "installer-state-machine.canonical.json", "failure-attribution.canonical.json",
        "readiness.canonical.json", "bootstrap-status.canonical.json",
        "publication-state-machine.canonical.json", "observability.canonical.json",
        "production-uat-contract.canonical.json", "provisioning-profiles.canonical.json",
        "com.tekes.kernel.supervisor.plist", "cases", "support-bundle",
    }
    if {path.name for path in FIX.iterdir()} != top_expected:
        raise Bad("deployment top-level oracle set differs from the closed set")
    fixture_manifest = json.loads((ROOT / "fixtures/manifest.json").read_bytes())
    if (
        not isinstance(fixture_manifest, dict)
        or fixture_manifest.get("version") != 1
        or not isinstance(fixture_manifest.get("corpora"), dict)
    ):
        raise Bad("fixture manifest envelope")
    manifest_deployment = fixture_manifest["corpora"].get("deployment")
    disk_deployment = sorted(
        path.relative_to(FIX).as_posix() for path in FIX.rglob("*") if path.is_file()
    )
    if manifest_deployment != disk_deployment:
        missing = sorted(set(disk_deployment) - set(manifest_deployment or []))
        extra = sorted(set(manifest_deployment or []) - set(disk_deployment))
        raise Bad(
            "fixtures/manifest.json deployment parity mismatch "
            f"missing={missing} extra={extra}"
        )
    provisioning = read(FIX / "provisioning-profiles.canonical.json", allow_lf=True)
    if provisioning != {
        "access_group_authorization": "each-required-group-or-team-wildcard",
        "container": "application-bundle",
        "format": 1,
        "profiles": [
            {
                "actor": "client-uat",
                "application_identifier": "TEKESAPP01.com.tekesapps.TekesUI",
                "bundle_identifier": "com.tekesapps.TekesUI",
                "embedded_path": "Contents/embedded.provisionprofile",
                "required_access_groups": ["TEKESAPP01.com.tekes.shared.endpoint"],
            },
            {
                "actor": "production-uat",
                "application_identifier": "TEKESAPP01.com.tekes.kernel.installer",
                "bundle_identifier": "com.tekes.kernel.installer",
                "embedded_path": "Contents/embedded.provisionprofile",
                "required_access_groups": [
                    "TEKESAPP01.com.tekes.kernel.production-uat",
                    "TEKESAPP01.com.tekes.kernel.provider-secrets",
                    "TEKESAPP01.com.tekes.shared.endpoint",
                ],
            },
            {
                "actor": "supervisor",
                "application_identifier": "TEKESAPP01.com.tekes.kernel.supervisor",
                "bundle_identifier": "com.tekes.kernel.supervisor",
                "embedded_path": "Contents/embedded.provisionprofile",
                "required_access_groups": [
                    "TEKESAPP01.com.tekes.kernel.provider-secrets",
                    "TEKESAPP01.com.tekes.shared.endpoint",
                ],
            },
        ],
        "validation": [
            "cms-decode", "not-expired", "team-identifier",
            "application-identifier", "access-group-authorization",
            "runtime-admission",
        ],
    }:
        raise Bad("provisioning profile/application-bundle contract mismatch")
    production_uat = read(FIX / "production-uat-contract.canonical.json", allow_lf=True)
    expected_runner_argv = [
        "--mode", "prepare", "--gate", "72|76", "--fixtures", "ABSOLUTE", "--selector", "ABSOLUTE",
        "--supervisor", "ABSOLUTE", "--worker", "ABSOLUTE", "--helper", "ABSOLUTE",
        "--client-uat", "ABSOLUTE", "--origin", "http://127.0.0.1:7347",
        "--release-version", "1.0.0", "--selector-conformance", "ABSOLUTE",
    ]
    expected_client_argv = [
        "--protocol", "tekes-kernel-production-uat", "--gate", "72|76",
        "--operation", "UUID", "--phase", "PHASE", "--origin", "http://127.0.0.1:7347",
        "--release-version", "1.0.0", "--install-root", "ABSOLUTE",
        "--storage-root", "ABSOLUTE", "--selector", "ABSOLUTE",
    ]
    expected_supervisor_argv = [
        "--install-root", "ABSOLUTE", "--storage-root", "ABSOLUTE",
        "--listen", "127.0.0.1:7347",
        "--selected-version", "1.0.0", "--selector-generation", "1",
        "--launch-id", "1-1-0123456789abcdef0123456789abcdef",
        "--manifest-sha256", "0" * 64, "--bootstrap-status-fd", "3",
        "--authority-registry-sha256",
        "f1f084f11ee379fd19ff0b62db653e245a088f7055f2146a5e1adf49decf5469",
        "--launcher-lifetime-fd", "4",
    ]
    if production_uat != {
        "client_argv": expected_client_argv,
        "client_phases": [
            "post-reboot", "session", "provider-initial", "provider-rotated",
            "provider-revoked",
        ],
        "client_probe": {
            "argv": ["--describe-contract"],
            "result": {
                "format": 1,
                "identifier": "com.tekesapps.TekesUI",
                "protocol": "tekes-kernel-production-uat",
                "release_version": "1.0.0",
            },
        },
        "client_phase_argv_suffix": {
            "post-reboot": [],
            "provider-initial": ["--session-id", "UUID"],
            "provider-revoked": ["--session-id", "UUID"],
            "provider-rotated": ["--session-id", "UUID"],
            "session": [],
        },
        "client_result_fields": {
            "post-reboot": ["format", "gate", "phase", "client_driver", "endpoint_ready"],
            "provider-active": ["format", "gate", "phase", "models_ready", "provider_advertised"],
            "provider-revoked": ["format", "gate", "phase", "provider_advertised", "typed_not_ready"],
            "session": ["format", "gate", "phase", "archive", "credential_acl", "retained_data", "session_id", "unarchive"],
        },
        "client_rpc_ids": {
            "archive": "uat-<operation>-archive",
            "create": "<operation>",
            "prompt": "uat-<operation>-prompt",
            "unarchive": "uat-<operation>-unarchive",
            "workspace": "uat-<operation>-workspace",
        },
        "format": 1,
        "operation_argv": {
            "acknowledge": ["--mode", "acknowledge", "--operation", "UUID"],
            "consume": ["--mode", "consume", "--operation", "UUID"],
            "resume": ["--mode", "resume", "--operation", "UUID"],
            "verify": ["--mode", "verify", "--operation", "UUID"],
        },
        "operation_exits": {
            "acknowledge": 0,
            "consume": 0,
            "describe": 0,
            "prepare": 75,
            "resume": 0,
            "verify": 0,
        },
        "operation_phases": {
            "prepare": ["preparing", "installing", "crash-proved", "prepared"],
            "resume": [
                "resume-process-proved", "resume-client-ready", "resume-session-proved",
                "resume-canary-attested", "resume-provider-initial",
                "resume-provider-rotated", "resume-provider-revoked", "resumed",
            ],
            "consume": [
                "consume-prepared", "consume-service-stopped",
                "consume-credentials-deleted", "consume-plist-removed",
                "consume-binaries-removed", "consume-resume-agent-removed",
                "consume-data-verified", "consumed",
            ],
        },
        "protocol": "tekes-kernel-production-uat",
        "runner_argv": expected_runner_argv,
        "supervisor_argv": expected_supervisor_argv,
        "uat_profile": {"endpoint": "http://127.0.0.1:7348/v1", "model_id": "model-uat", "provider_id": "provider-uat", "workspace_id": "deployment-uat"},
    }:
        raise Bad("production UAT closed argv/protocol oracle mismatch")
    if len(production_uat["supervisor_argv"]) != 20:
        raise Bad("production UAT must freeze all ten supervisor option/value pairs")
    contract = registry["artifact_contract"]
    if contract["required_step_fields"] != ["command", "fault_points", "expected_exit", "expected_selection", "expected_readiness"]:
        raise Bad("registry step fields changed")

    wanted = {FIX / "cases" / f"{case}.{part}.canonical.json" for case in declared for part in ("setup", "steps", "expected")}
    actual = set((FIX / "cases").glob("*.canonical.json"))
    if wanted != actual:
        raise Bad(f"case registry/disk mismatch missing={sorted(wanted-actual)} extra={sorted(actual-wanted)}")

    for gate, cases in GATE_MAP.items():
        for case in cases:
            one = read(FIX / f"cases/{case}.setup.canonical.json")
            two = read(FIX / f"cases/{case}.steps.canonical.json")
            three = read(FIX / f"cases/{case}.expected.canonical.json")
            validate_setup(one, case, gate)
            validate_steps(two, case)
            validate_expected(three, case, gate)
            if two["expected_selection"] != three["selection"] or two["expected_readiness"] != three["readiness"]:
                raise Bad(f"{case}: steps/expected outcome mismatch")
            commands = two["command"]
            if three["processes"]["listeners"] == 0 and ["deployment-harness", "GET", "/health/ready"] in commands:
                raise Bad(f"{case}: zero-listener outcome cannot assert HTTP readiness")
            code = three["readiness"].get("code")
            if code in BOOTSTRAP and ["tekes-selector", "--install-root", "ROOT", "status"] not in commands:
                raise Bad(f"{case}: pre-listener failure is not read through selector status")
            if code == "service-absent" and ["deployment-harness", "verify-service-absent"] not in commands:
                raise Bad(f"{case}: service absence is not established by the install harness")
            if three["processes"]["supervisors"] or three["processes"]["listeners"]:
                if three["processes"]["selectors"] != 1:
                    raise Bad(f"{case}: supervisor/listener lacks resident selector owner")

    selector_path = "selector/bin/tekes-selector"
    for case in ("clean-install", "reboot-start"):
        expected = read(FIX / f"cases/{case}.expected.canonical.json")
        if selector_path not in expected["filesystem"]["present"] or expected["filesystem"]["modes"].get(selector_path) != "0755":
            raise Bad(f"{case}: stable selector launch artifact is not asserted")
    uninstall = read(FIX / "cases/uninstall-retains-data.expected.canonical.json")
    if selector_path not in uninstall["filesystem"]["absent"]:
        raise Bad("uninstall-retains-data: selector artifact is not removed")
    publication_steps = read(FIX / "cases/publication-power-loss.steps.canonical.json")
    publication_commands = publication_steps["command"]
    required_upgrade_order = [
        ["deployment-harness", "launchctl-bootout"],
        ["tekes-selector", "--install-root", "ROOT", "activate", "--version", "2.0.0"],
        ["deployment-harness", "launchctl-bootstrap"],
    ]
    positions = [publication_commands.index(item) for item in required_upgrade_order]
    if positions != sorted(positions):
        raise Bad("publication-power-loss: bootout/activate/bootstrap order is not closed")
    if "launch-requested" in publication_steps["fault_points"]:
        raise Bad("publication-power-loss: external launch leaked into selection transaction")
    first_activation = read(FIX / "cases/first-activation-power-loss.expected.canonical.json")
    if "previous" in first_activation["selection"] or first_activation["selection"] != {"current": "1.0.0", "generation": 1}:
        raise Bad("first activation does not freeze absent previous selection")
    selector_update_steps = read(FIX / "cases/selector-update-power-loss.steps.canonical.json")
    if selector_update_steps["fault_points"] != ["selector-offline-conformance", "selector-temp-synced", "selector-verified", "selector-renamed", "selector-directory-synced"] or ["deployment-harness", "verify-selector-offline-conformance"] not in selector_update_steps["command"]:
        raise Bad("selector update crash boundaries mismatch")
    crash_loop_steps = read(FIX / "cases/crash-loop-rollback.steps.canonical.json")
    if read(FIX / "cases/crash-loop-rollback.setup.canonical.json")["initial"].get("observation") != "rollback-window":
        raise Bad("crash-loop rollback does not begin inside rollback window")
    flat_crash_loop = [part for argv in crash_loop_steps["command"] for part in argv]
    if "observe" in flat_crash_loop or "rollback" in flat_crash_loop or ["deployment-harness", "inject-three-independent-startup-failures"] not in crash_loop_steps["command"]:
        raise Bad("crash-loop rollback still depends on an external observe/rollback actor")
    attribution = read(FIX / "failure-attribution.canonical.json")
    expected_attributable = ["canary-attribution-failed", "canary-timeout", "invalid-install", "protocol-mismatch", "readiness-timeout", "selector-mismatch", "unexpected-child-exit"]
    expected_environment = ["already-running", "corrupt-ledger", "endpoint-credential-unavailable", "invalid-config", "io", "listener-unavailable", "required-broker-unavailable", "unsupported-filesystem"]
    promotion_contract = {"comparator": "failure-commit-now-less-than-window-close", "in_flight": "retain-rollback-eligibility-until-pre-boundary-attempt-terminal", "requires_ready_child": True, "resets_consecutive_failures": True}
    if attribution != {"attributable": expected_attributable, "environment_or_data": expected_environment, "format": 1, "promotion": promotion_contract, "retry_seconds": [1, 2, 4, 8, 30], "threshold": 3}:
        raise Bad("failure attribution/retry oracle mismatch")
    non_attributable = read(FIX / "cases/non-attributable-failure-matrix.steps.canonical.json")
    if non_attributable["expected_selection"] != {"current": "2.0.0", "previous": "1.0.0", "generation": 2} or non_attributable["fault_points"] != ["invalid-config-not-counted", "endpoint-credential-unavailable-not-counted", "listener-unavailable-not-counted", "candidate-failure-count-remains-zero"]:
        raise Bad("environment failure matrix can spuriously rollback")
    promotion = read(FIX / "cases/promotion-window-boundary.steps.canonical.json")
    if promotion["expected_selection"] != {"current": "2.0.0", "previous": "1.0.0", "generation": 2} or promotion["fault_points"] != ["attributable-failure-before-window-counted", "in-flight-at-boundary-retains-rollback-eligibility", "ready-completion-promotes", "promotion-resets-counter", "post-promotion-failures-do-not-rollback"] or ["deployment-harness", "verify-promotion-counter-zero"] not in promotion["command"]:
        raise Bad("promotion boundary can collapse or extend the rollback window")
    for case, version, mode in (("clean-install", "1.0.0", "install"), ("first-activation-power-loss", "1.0.0", "install"), ("publication-power-loss", "2.0.0", "upgrade")):
        commands = read(FIX / f"cases/{case}.steps.canonical.json")["command"]
        attest = ["tekes-selector", "--install-root", "ROOT", "attest-canary", "--version", version, "--session", "018f0000-0000-7000-8000-0000000000ca", "--run", "CANARY_RUN", "--storage-root", "THREADS"]
        if ["deployment-harness", "submit-canary", mode] not in commands or attest not in commands:
            raise Bad(f"{case}: explicit Client canary/selector attestation is missing")
    clean_commands = read(FIX / "cases/clean-install.steps.canonical.json")["command"]
    if ["deployment-harness", "run-signed-installer-install"] not in clean_commands or ["client", "--driver", ".tekes", "--base-url", "http://127.0.0.1:7347", "--credential", "keychain", "host.describe"] not in clean_commands:
        raise Bad("clean-install: real Client fixed-origin discovery is not frozen")
    uninstall_commands = read(FIX / "cases/uninstall-retains-data.steps.canonical.json")["command"]
    if ["deployment-harness", "verify-keychain-endpoint-credential-absent"] not in uninstall_commands:
        raise Bad("uninstall: endpoint credential deletion is not frozen")
    install_recovery = read(FIX / "cases/installer-install-recovery.steps.canonical.json")
    if ["deployment-harness", "rerun-same-installer-install"] not in install_recovery["command"] or ["deployment-harness", "rerun-same-installer-rotation"] not in install_recovery["command"]:
        raise Bad("installer install/credential recovery is not frozen")
    uninstall_recovery = read(FIX / "cases/installer-uninstall-recovery.steps.canonical.json")
    if ["deployment-harness", "launchd-start-with-open-installer-operation"] not in uninstall_recovery["command"] or ["deployment-harness", "rerun-same-installer-uninstall"] not in uninstall_recovery["command"]:
        raise Bad("installer uninstall recovery is not frozen")
    idle_reboot = read(FIX / "cases/reboot-target-user-login.steps.canonical.json")
    if read(FIX / "cases/reboot-target-user-login.setup.canonical.json")["initial"].get("observation") != "promoted":
        raise Bad("ordinary reboot does not begin from promoted state")
    idle_parts = [part for argv in idle_reboot["command"] for part in argv]
    if (
        "target-user-login-without-tekes-gui" not in idle_parts
        or "submit-canary" in idle_parts
        or "attest-canary" in idle_parts
        or "target-user-login-no-canary" not in idle_reboot["fault_points"]
    ):
        raise Bad("ordinary post-login LaunchAgent reboot incorrectly requires a canary")
    selector_kill = read(FIX / "cases/selector-sigkill-recovery.steps.canonical.json")
    if ["deployment-harness", "sigkill-selector"] not in selector_kill["command"] or selector_kill["fault_points"] != ["selector-sigkill", "lifetime-fd-eof", "predecessor-root-lock-busy", "child-drained", "root-lock-released"]:
        raise Bad("selector SIGKILL lifetime-fd recovery is not frozen")

    selector = read(FIX / "selector.canonical.json")
    if set(selector) != {"commands", "current", "error_rows", "exits", "format", "observation", "operational", "previous", "replies", "state_guards"}:
        raise Bad("selector oracle top-level fields")
    command_names = ["describe-conformance", "stage", "activate", "rollback", "status", "recover", "attest-canary", "serve", "update-selector", "prune"]
    if selector["format"] != 1 or [item["name"] for item in selector["commands"]] != command_names:
        raise Bad("selector command surface mismatch")
    if selector["commands"][0]["argv"] != ["describe-conformance"]:
        raise Bad("selector conformance command argv mismatch")
    if selector["commands"][7]["argv"] != ["--install-root", "<absolute>", "serve", "--storage-root", "<absolute>", "--listen", "127.0.0.1:7347"]:
        raise Bad("selector serve fixed-origin argv mismatch")
    if set(selector["exits"].values()) != EXITS:
        raise Bad("selector exit taxonomy mismatch")
    if selector["current"]["generation"] != selector["previous"]["generation"]:
        raise Bad("selector state generations differ")
    if selector["current"]["selection"] == selector["previous"]["selection"]:
        raise Bad("current and previous selections collapse")
    error_codes = [item["code"] for item in selector["error_rows"]]
    if error_codes != ["usage", "invalid-bundle", "invalid-state", "unavailable", "io", "corruption"]:
        raise Bad("selector error row order/set mismatch")
    if any(set(item) != {"code", "details", "message"} or not isinstance(item["details"], dict) for item in selector["error_rows"]):
        raise Bad("selector error row shape")
    expected_error_exits = {
        "usage": 64,
        "invalid-bundle": 65,
        "invalid-state": 66,
        "unavailable": 69,
        "io": 74,
        "corruption": 75,
    }
    for row in selector["error_rows"]:
        stderr = canonical({"error": row}) + b"\n"
        if not stderr.endswith(b"\n") or stderr.count(b"\n") != 1:
            raise Bad(f"selector {row['code']} stderr is not exactly one canonical JSONL record")
        if selector["exits"].get(row["code"]) != expected_error_exits[row["code"]]:
            raise Bad(f"selector {row['code']} exit mismatch")
    replies = selector["replies"]
    if set(replies) != {"stage", "activate", "rollback", "status", "recover", "attest_canary", "conformance", "update_selector", "prune"}:
        raise Bad("selector success reply set mismatch")
    for name in ("stage", "activate", "rollback", "recover", "prune"):
        if replies[name].get("format") != 1 or replies[name].get("operation") != name:
            raise Bad(f"selector {name} reply mismatch")
    if set(replies["prune"]) != {"format", "operation", "removed"} or not all(
        isinstance(version, str) and version for version in replies["prune"]["removed"]
    ):
        raise Bad("selector prune reply mismatch")
    if replies["update_selector"].get("operation") != "update-selector" or replies["update_selector"].get("format") != 1:
        raise Bad("selector update reply mismatch")
    conformance = replies["conformance"]
    if (
        set(conformance) != {"architecture", "conformance_sha256", "format", "operation", "version"}
        or conformance["architecture"] != "aarch64"
        or conformance["format"] != 1
        or conformance["operation"] != "describe-conformance"
        or not isinstance(conformance["version"], str)
        or not conformance["version"]
        or not re.fullmatch(r"[0-9a-f]{64}", conformance["conformance_sha256"])
    ):
        raise Bad("selector conformance reply mismatch")
    if replies["attest_canary"] != {"format": 1, "operation": "attest-canary", "run": "canary-0002", "session": "018f0000-0000-7000-8000-0000000000ca", "version": "2.0.0"}:
        raise Bad("selector canary attestation reply mismatch")
    observation = selector["observation"]
    observation_keys = {"format", "version", "generation", "manifest_sha256", "attempt", "launch_id", "started_at", "deadline_at", "canary_required", "canary_deadline_at", "rollback_eligible", "window_closes_at", "consecutive_failures", "last_code", "state", "canary_run", "canary_session"}
    if set(observation) != observation_keys or observation["format"] != 1 or observation["state"] not in {"pending", "ready", "failed", "promoted"}:
        raise Bad("selector observation schema mismatch")
    if observation["attempt"] < 1 or observation["consecutive_failures"] < 0 or not re.fullmatch(r"[0-9a-f]{64}", observation["manifest_sha256"]) or not re.fullmatch(r"[1-9][0-9]*-[1-9][0-9]*-[0-9a-f]{32}", observation["launch_id"]):
        raise Bad("selector observation values")
    if observation["canary_session"] != "018f0000-0000-7000-8000-0000000000ca" or observation["canary_run"] != "canary-0002":
        raise Bad("selector observation does not retain canary session/run pair")
    if observation["started_at"] != "2026-08-28T00:00:00.000000000Z" or observation["deadline_at"] != "2026-08-28T00:00:30.000000000Z" or observation["canary_deadline_at"] != "2026-08-28T00:02:10.000000000Z" or observation["window_closes_at"] != "2026-08-28T00:05:10.000000000Z" or observation["canary_required"] is not True or observation["rollback_eligible"] is not True:
        raise Bad("selector observation constants mismatch")
    if replies["status"].get("observation") != observation or replies["status"].get("service") != "running":
        raise Bad("selector status reply does not freeze observation bytes")
    if set(selector["state_guards"]) != {
        "offline_mutation_child_group_running",
        "rollback_after_promotion",
    }:
        raise Bad("selector state guard set mismatch")
    offline_guard = selector["state_guards"]["offline_mutation_child_group_running"]
    if (
        offline_guard.get("exit") != 66
        or offline_guard.get("error", {}).get("code") != "invalid-state"
        or offline_guard.get("error", {}).get("details") != {"state": "child-group-running"}
    ):
        raise Bad("selector offline child-group guard mismatch")
    rollback_guard = selector["state_guards"].get("rollback_after_promotion")
    expected_guard = {"error": {"code": "invalid-state", "details": {"state": "rollback-window-closed"}, "message": "Selector state does not permit the operation"}, "exit": 66}
    if rollback_guard != expected_guard:
        raise Bad("post-promotion rollback is not closed")
    operational = selector["operational"]
    if set(operational) != {"codes", "failure_rows", "prelaunch_status", "rotation"}:
        raise Bad("selector operational registry envelope")
    expected_selector_codes = {
        "orphan-owner-timeout": ("selector", "error", "empty", ["deadline_ms", "generation", "manifest_sha256", "root_lock", "version"]),
        "promotion-complete": ("child", "info", "complete-child", ["from_state"]),
        "readiness-crash-loop": ("child", "error", "complete-child", ["classification"]),
        "rollback-complete": ("child", "warn", "complete-child-plus-operation", ["from", "reason", "to"]),
        "selector-child-launch": ("child", "info", "complete-child", ["canary_required"]),
        "selector-predecessor-draining": ("selector", "warn", "empty", ["root_lock"]),
        "selector-prelaunch-cleared": ("selector", "info", "empty", ["root_lock"]),
        "selector-recovered": ("selector", "info", "operation", ["operation", "phase"]),
        "selector-update-complete": ("selector", "info", "empty", ["sha256", "version"]),
    }
    actual_selector_codes = {}
    for row in operational["codes"]:
        if set(row) != {"build_authority", "code", "correlation", "fields", "severity"}:
            raise Bad("selector operational code row shape")
        code = row["code"]
        if code in actual_selector_codes:
            raise Bad("selector operational code duplicates")
        actual_selector_codes[code] = (
            row["build_authority"],
            row["severity"],
            row["correlation"],
            row["fields"],
        )
    if actual_selector_codes != expected_selector_codes:
        raise Bad("selector operational code registry mismatch")
    if operational["failure_rows"] != {
        "attributable": {"correlation": "complete-child", "fields": ["classification"], "severity": "error"},
        "environment": {"correlation": "complete-child", "fields": ["classification"], "severity": "warn"},
    }:
        raise Bad("selector operational failure rows mismatch")
    if operational["prelaunch_status"] != {
        "clear_code": "selector-prelaunch-cleared",
        "source": "rotated-log-generations-oldest-to-newest",
        "timeout_code": "orphan-owner-timeout",
    }:
        raise Bad("selector prelaunch status projection mismatch")
    if operational["rotation"] != {
        "bytes": 10 * 1024 * 1024,
        "full_sync_each_line": True,
        "retain": 5,
    }:
        raise Bad("selector operational rotation mismatch")

    bundle_path = FIX / "bundle-manifest.canonical.json"
    bundle = read(bundle_path, allow_lf=True)
    if set(bundle) != {"format", "version", "minimum_os", "architectures", "files", "compatibility", "signing"} or bundle["format"] != 1:
        raise Bad("bundle manifest envelope")
    if not re.fullmatch(r"[A-Z0-9]{10}", bundle["signing"].get("team_id", "")):
        raise Bad("bundle manifest team id")
    registry_path = FIX / "authority-registry.canonical.json"
    authority_registry = read(registry_path, allow_lf=True)
    if set(authority_registry) != {"format", "profile", "authorities"} or authority_registry["format"] != 1 or authority_registry["profile"] != "v1":
        raise Bad("authority registry envelope")
    authority_rows = authority_registry["authorities"]
    authority_names = [row.get("authority") for row in authority_rows]
    if authority_names != sorted(authority_names) or len(authority_names) != len(set(authority_names)):
        raise Bad("authority registry order/set")
    # Registry bytes/digests are runtime compatibility identities. A documentation
    # move must not mint a new identity: resolve its historical source through
    # the section migration map and require every destination to remain present.
    document_moves = json.loads((ROOT / "docs/history/document-map.json").read_text())

    def contract_document_exists(contract):
        if (ROOT / contract).is_file():
            return True
        destinations = {row["destination"] for row in document_moves if row["source"] == contract}
        return bool(destinations) and all((ROOT / dest).is_file() for dest in destinations)

    if any(set(row) != {"authority", "contract", "writer_profile"} or row["writer_profile"] != "v1" or not contract_document_exists(row["contract"]) for row in authority_rows):
        raise Bad("authority registry row")
    required_authorities = {
        "config-authority",
        "endpoint-management-carriers", "endpoint-projection-journals",
        "instruction-snapshots", "launch-bindings",
        "memory-goals-tool-state-jobs", "provider-sealed-adopt-assets",
        "provider-secret-generation-authority",
        "rewrite-operations", "seed-snapshot-carriers", "semantic-thread-ledgers",
        "tool-control-records", "tool-runtime-state", "worker-control",
    }
    if set(authority_names) != required_authorities:
        raise Bad("authority registry is not the closed Slice 10 writer inventory")
    authority_registry_sha = hashlib.sha256(registry_path.read_bytes()).hexdigest()
    compat_keys = {"authority_registry_sha256", "reader_profile", "writer_profile"}
    compatibility = bundle["compatibility"]
    if set(compatibility) != compat_keys:
        raise Bad("bundle compatibility shape")
    if compatibility != {"authority_registry_sha256": authority_registry_sha, "reader_profile": "v1", "writer_profile": "v1"}:
        raise Bad("bundle does not bind exact v1 authority registry")
    expected_bundle_files = {
        "apps/TekesKernelSupervisor.app/Contents/Info.plist": "0644",
        "apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor": "0755",
        "apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json": "0644",
        "apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources": "0644",
        "apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile": "0644",
        "bin/tekes-helper": "0755",
        "bin/tekes-worker": "0755",
    }
    paths = [item["path"] for item in bundle["files"]]
    if paths != sorted(paths) or paths != sorted(expected_bundle_files):
        raise Bad("bundle file set/order")
    for item in bundle["files"]:
        if set(item) != {"path", "bytes", "sha256", "mode"} or item["mode"] != expected_bundle_files[item["path"]] or not re.fullmatch(r"[0-9a-f]{64}", item["sha256"]):
            raise Bad("bundle file row")
    if bundle["signing"]["requirement"] != "anchor apple generic and identifier com.tekes.kernel.supervisor":
        raise Bad("bundle signing requirement is not the supervisor app requirement")
    manifest_sha = hashlib.sha256(bundle_path.read_bytes()).hexdigest()
    current_state = read(FIX / "selector-current.canonical.json", allow_lf=True)
    previous_state = read(FIX / "selector-previous.canonical.json", allow_lf=True)
    if current_state != selector["current"] or previous_state != selector["previous"]:
        raise Bad("selector current/previous standalone state mismatch")
    if current_state["selection"]["manifest_sha256"] != manifest_sha:
        raise Bad("current selection does not bind the bundle manifest bytes")

    compatibility_oracle = read(FIX / "compatibility.canonical.json")
    if set(compatibility_oracle) != {"format", "accepted", "rejected"} or compatibility_oracle["format"] != 1:
        raise Bad("compatibility oracle envelope")
    accepted = compatibility_oracle["accepted"]
    candidate = accepted["candidate"]
    previous = accepted["previous"]
    if set(accepted) != {"candidate", "previous"} or candidate != compatibility or previous != compatibility:
        raise Bad("rollback-eligible compatibility is not exact equality")
    rejected = compatibility_oracle["rejected"]
    if [row.get("reason") for row in rejected] != ["authority-registry-digest-mismatch", "profile-mismatch", "unknown-authority"]:
        raise Bad("compatibility rejection registry mismatch")
    if any(row.get("code") != "invalid-bundle" for row in rejected):
        raise Bad("compatibility rejection does not fail closed")
    if rejected[0].get("candidate", {}).get("authority_registry_sha256") == authority_registry_sha:
        raise Bad("digest-mismatch rejection is not mismatched")
    if rejected[1].get("candidate") == compatibility or rejected[1].get("candidate", {}).get("reader_profile") == "v1":
        raise Bad("profile-mismatch rejection is not mismatched")
    if rejected[2] != {"authority": "unknown-durable-authority", "code": "invalid-bundle", "reason": "unknown-authority"}:
        raise Bad("unknown authority does not fail closed")

    selector_manifest = read(FIX / "selector-manifest.canonical.json", allow_lf=True)
    if set(selector_manifest) != {"format", "version", "minimum_os", "architecture", "conformance_sha256", "file", "signing"} or selector_manifest["format"] != 1 or not re.fullmatch(r"[0-9a-f]{64}", selector_manifest["conformance_sha256"]):
        raise Bad("selector update manifest envelope")
    selector_file = selector_manifest["file"]
    if set(selector_file) != {"path", "bytes", "sha256", "mode"} or selector_file["path"] != "selector/bin/tekes-selector" or selector_file["mode"] != "0755" or not re.fullmatch(r"[0-9a-f]{64}", selector_file["sha256"]):
        raise Bad("selector update manifest file row")
    if replies["update_selector"]["sha256"] != selector_file["sha256"] or replies["update_selector"]["version"] != selector_manifest["version"]:
        raise Bad("selector update reply does not bind manifest")

    install_identity_path = FIX / "install-identity.canonical.json"
    install_identity = read(install_identity_path, allow_lf=True)
    identity_keys = {"format", "team_id", "access_group", "installer_requirement", "client_requirement", "selector_requirement", "supervisor_requirement"}
    if set(install_identity) != identity_keys or install_identity["format"] != 1 or install_identity["team_id"] != bundle["signing"]["team_id"] or install_identity["access_group"] != install_identity["team_id"] + ".com.tekes.shared.endpoint":
        raise Bad("immutable installation identity mismatch")
    if selector_manifest["signing"].get("team_id") != install_identity["team_id"] or selector_manifest["signing"].get("requirement") != install_identity["selector_requirement"]:
        raise Bad("selector update escapes installation signing identity")
    if any(not value.startswith("anchor apple generic and identifier ") for key, value in install_identity.items() if key.endswith("_requirement")):
        raise Bad("installation designated requirement")
    install_identity_sha = hashlib.sha256(install_identity_path.read_bytes()).hexdigest()

    signing_identity = read(FIX / "signing-identity.canonical.json", allow_lf=True)
    provider_secret_group = f"{install_identity['team_id']}.com.tekes.kernel.provider-secrets"
    accepted_signing = {"access_group": install_identity["access_group"], "bundle_team_id": install_identity["team_id"], "client_access_group": install_identity["access_group"], "client_team_id": install_identity["team_id"], "installer_access_group": install_identity["access_group"], "installer_production_uat_access_group": f"{install_identity['team_id']}.com.tekes.kernel.production-uat", "installer_provider_secret_access_group": provider_secret_group, "installer_team_id": install_identity["team_id"], "selector_team_id": install_identity["team_id"], "supervisor_access_group": install_identity["access_group"], "supervisor_provider_secret_access_group": provider_secret_group}
    rejected_signing_actors = ["candidate-bundle", "candidate-supervisor-entitlement", "candidate-supervisor-provider-secret-entitlement", "selector-update", "client-entitlement", "client-provider-secret-entitlement", "installer-entitlement", "installer-provider-secret-entitlement"]
    rejected_signing_codes = ["invalid-bundle", "invalid-install", "invalid-install", "invalid-bundle", "invalid-install", "invalid-install", "invalid-install", "invalid-install"]
    if signing_identity.get("format") != 1 or signing_identity.get("accepted") != accepted_signing or [row.get("actor") for row in signing_identity.get("rejected", [])] != rejected_signing_actors or [row.get("code") for row in signing_identity["rejected"]] != rejected_signing_codes:
        raise Bad("signing identity acceptance/rejection oracle mismatch")

    credential = read(FIX / "endpoint-credential.canonical.json", allow_lf=True)
    expected_credential = {
        "access_group": bundle["signing"]["team_id"] + ".com.tekes.shared.endpoint",
        "account": "loopback-bearer", "bytes": 32,
        "client_driver": ".tekes", "encoding": "base64url-no-padding",
        "format": 1, "install_identity_sha256": install_identity_sha,
        "origin": "http://127.0.0.1:7347",
        "readers": ["signed-tekes-client", "signed-tekes-supervisor"],
        "rotation": "installer-bootout-atomic-replace", "selector_access": False,
        "service": "com.tekes.kernel.endpoint", "uninstall": "delete",
        "upgrade": "retain",
    }
    if credential != expected_credential:
        raise Bad("endpoint credential/discovery oracle mismatch")

    installer_machine = read(FIX / "installer-state-machine.canonical.json")
    installer_phases = {
        "install": ["prepared", "identity-published", "credential-published", "bundles-published", "selection-published", "plist-published", "service-started", "closed"],
        "rotate_credential": ["prepared", "service-stopped", "credential-replaced", "service-started", "closed"],
        "uninstall": ["prepared", "service-stopped", "credential-deleted", "plist-removed", "binaries-removed", "closed"],
    }
    if set(installer_machine) != {"format", "install", "rotate_credential", "uninstall", "operation_shapes", "closed_retry"} or installer_machine["format"] != 1 or any(installer_machine[key] != phases for key, phases in installer_phases.items()):
        raise Bad("installer state-machine phases")
    installer_shapes = installer_machine["operation_shapes"]
    if set(installer_shapes) != {"install", "rotate_credential", "uninstall"} or any(set(row) != {"format", "install_identity_sha256", "op_id", "phase", "request_sha256", "type"} or row["install_identity_sha256"] != install_identity_sha or not re.fullmatch(r"[0-9a-f]{64}", row["request_sha256"]) for row in installer_shapes.values()):
        raise Bad("installer operation shape/identity binding")
    installer_closed = installer_machine["closed_retry"]
    if installer_closed.get("install_identity_sha256") != install_identity_sha or installer_closed.get("request_sha256") != installer_shapes["uninstall"]["request_sha256"] or installer_closed.get("response") != {"format": 1, "operation": "uninstall", "service": "absent"}:
        raise Bad("installer closed retry authority")

    layout = read(FIX / "install-layout.canonical.json")
    rows = {item["path"]: item for item in layout["paths"]}
    base = "USER_HOME/Library/Application Support/Tekes/Kernel"
    required_paths = {
        base: {"kind": "directory", "mode": "0700", "path": base},
        f"{base}/bundles": {"kind": "directory", "mode": "0700", "path": f"{base}/bundles"},
        f"{base}/bundles/2.0.0": {"kind": "directory", "mode": "0700", "path": f"{base}/bundles/2.0.0"},
        f"{base}/bundles/2.0.0/apps": {"kind": "directory", "mode": "0700", "path": f"{base}/bundles/2.0.0/apps"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app": {"kind": "directory", "mode": "0700", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents": {"kind": "directory", "mode": "0700", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/Info.plist": {"kind": "file", "mode": "0644", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/Info.plist"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/MacOS": {"kind": "directory", "mode": "0700", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/MacOS"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor": {"kind": "file", "mode": "0755", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/MacOS/tekes-supervisor"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/Resources": {"kind": "directory", "mode": "0700", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/Resources"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json": {"kind": "file", "mode": "0644", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/Resources/WebClientManifest.canonical.json"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/_CodeSignature": {"kind": "directory", "mode": "0700", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/_CodeSignature"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources": {"kind": "file", "mode": "0644", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/_CodeSignature/CodeResources"},
        f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile": {"kind": "file", "mode": "0644", "path": f"{base}/bundles/2.0.0/apps/TekesKernelSupervisor.app/Contents/embedded.provisionprofile"},
        f"{base}/bundles/2.0.0/bin": {"kind": "directory", "mode": "0700", "path": f"{base}/bundles/2.0.0/bin"},
        f"{base}/bundles/2.0.0/bin/tekes-helper": {"kind": "file", "mode": "0755", "path": f"{base}/bundles/2.0.0/bin/tekes-helper"},
        f"{base}/bundles/2.0.0/bin/tekes-worker": {"kind": "file", "mode": "0755", "path": f"{base}/bundles/2.0.0/bin/tekes-worker"},
        f"{base}/bundles/2.0.0/manifest.canonical.json": {"kind": "file", "mode": "0600", "path": f"{base}/bundles/2.0.0/manifest.canonical.json"},
        f"{base}/selector": {"kind": "directory", "mode": "0700", "path": f"{base}/selector"},
        f"{base}/selector/.lock": {"kind": "file", "mode": "0600", "path": f"{base}/selector/.lock"},
        f"{base}/selector/.service-lock": {"kind": "file", "mode": "0600", "path": f"{base}/selector/.service-lock"},
        f"{base}/selector/active": {"kind": "symlink", "path": f"{base}/selector/active", "target": "../bundles/2.0.0"},
        f"{base}/selector/bin": {"kind": "directory", "mode": "0700", "path": f"{base}/selector/bin"},
        f"{base}/selector/bin/tekes-selector": {"kind": "file", "mode": "0755", "path": f"{base}/selector/bin/tekes-selector"},
        f"{base}/selector/current.json": {"kind": "file", "mode": "0600", "path": f"{base}/selector/current.json"},
        f"{base}/selector/observations": {"kind": "directory", "mode": "0700", "path": f"{base}/selector/observations"},
        f"{base}/selector/operations": {"kind": "directory", "mode": "0700", "path": f"{base}/selector/operations"},
        f"{base}/selector/previous.json": {"kind": "file", "mode": "0600", "path": f"{base}/selector/previous.json"},
        "USER_HOME/.agents": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents"},
        "USER_HOME/.agents/settings": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/settings"},
        "USER_HOME/.agents/workspaces": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/workspaces"},
        "USER_HOME/.agents/jobs": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/jobs"},
        "USER_HOME/.agents/config": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/config"},
        "USER_HOME/.agents/credential-state": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/credential-state"},
        "USER_HOME/.agents/credential-state/.lock": {"kind": "file", "mode": "0600", "path": "USER_HOME/.agents/credential-state/.lock"},
        "USER_HOME/.agents/credential-state/provider-secret-generations.json": {"kind": "file", "mode": "0600", "path": "USER_HOME/.agents/credential-state/provider-secret-generations.json"},
        "USER_HOME/.agents/threads": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/threads"},
        "USER_HOME/.agents/threads/.root-lock": {"kind": "file", "mode": "0600", "path": "USER_HOME/.agents/threads/.root-lock"},
        "USER_HOME/Library/Application Support/Tekes/Installer": {"kind": "directory", "mode": "0700", "path": "USER_HOME/Library/Application Support/Tekes/Installer"},
        "USER_HOME/Library/Application Support/Tekes/Installer/.lock": {"kind": "file", "mode": "0600", "path": "USER_HOME/Library/Application Support/Tekes/Installer/.lock"},
        "USER_HOME/Library/Application Support/Tekes/Installer/install-identity.json": {"kind": "file", "mode": "0600", "path": "USER_HOME/Library/Application Support/Tekes/Installer/install-identity.json"},
        "USER_HOME/Library/Application Support/Tekes/Installer/operation.json": {"kind": "file", "mode": "0600", "path": "USER_HOME/Library/Application Support/Tekes/Installer/operation.json"},
        "USER_HOME/.agents/cache": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/cache"},
        "USER_HOME/.agents/logs": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/logs"},
        "USER_HOME/.agents/logs/kernel": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/logs/kernel"},
        "USER_HOME/.agents/runtime": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/runtime"},
        "USER_HOME/.agents/runtime/composer": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/runtime/composer"},
        "USER_HOME/.agents/skills": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/skills"},
        "USER_HOME/.agents/client": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/client"},
        "USER_HOME/.agents/not-in-project": {"kind": "directory", "mode": "0700", "path": "USER_HOME/.agents/not-in-project"},
        "USER_HOME/Library/LaunchAgents/com.tekes.kernel.supervisor.plist": {"kind": "file", "mode": "0644", "path": "USER_HOME/Library/LaunchAgents/com.tekes.kernel.supervisor.plist"},
    }
    if rows != required_paths or len(layout["paths"]) != len(required_paths):
        raise Bad("install layout differs from the complete deployment contract")

    plist_raw = (FIX / "com.tekes.kernel.supervisor.plist").read_bytes()
    plist = plistlib.loads(plist_raw)
    plist_keys = {"Label", "Program", "ProgramArguments", "RunAtLoad", "KeepAlive", "ProcessType", "StandardErrorPath", "StandardOutPath"}
    if set(plist) != plist_keys or plist.get("Label") != "com.tekes.kernel.supervisor" or plist.get("RunAtLoad") is not True or plist.get("KeepAlive") is not True or plist.get("ProcessType") != "Background":
        raise Bad("launchd ownership fields")
    if "EnvironmentVariables" in plist or plist.get("Program") != "@SELECTOR@":
        raise Bad("launchd program/environment")
    if plist.get("ProgramArguments") != ["@SELECTOR@", "--install-root", "@INSTALL_ROOT@", "serve", "--storage-root", "@STORAGE_ROOT@", "--listen", "127.0.0.1:7347"]:
        raise Bad("launchd argv")
    if plist.get("StandardOutPath") != "@STDOUT@" or plist.get("StandardErrorPath") != "@STDERR@":
        raise Bad("launchd log paths")
    for token in ("@SELECTOR@", "@INSTALL_ROOT@", "@STORAGE_ROOT@", "@STDOUT@", "@STDERR@"):
        if token.encode() not in plist_raw:
            raise Bad(f"launchd template omits {token}")

    ready = read(FIX / "readiness.canonical.json")
    rows = {item["code"]: item["message"] for item in ready["not_ready"]}
    if rows != READINESS or ready["ready"] != {"status": 200, "body": {"build": "2.0.0", "generation": 2, "ready": True}}:
        raise Bad("readiness oracle mismatch")
    for item in ready["not_ready"]:
        expected_body = {"ready": False, "error": {"code": item["code"], "message": item["message"], "details": {}}}
        if item["status"] != 503 or item["body"] != expected_body:
            raise Bad(f"readiness error body mismatch for {item['code']}")

    bootstrap = read(FIX / "bootstrap-status.canonical.json")
    failure_rows = {item["code"]: item["message"] for item in bootstrap["failures"]}
    if set(bootstrap) != {"format", "failures", "listener_bound", "failed"} or bootstrap["format"] != 1 or failure_rows != BOOTSTRAP:
        raise Bad("bootstrap status failure registry mismatch")
    listener_bound = bootstrap["listener_bound"]
    if set(listener_bound) != {"format", "launch_id", "selection", "state"} or listener_bound["format"] != 1 or listener_bound["state"] != "listener-bound" or listener_bound["selection"] != current_state["selection"] or not re.fullmatch(r"[1-9][0-9]*-[1-9][0-9]*-[0-9a-f]{32}", listener_bound["launch_id"]):
        raise Bad("bootstrap listener-bound oracle mismatch")
    failed_bootstrap = bootstrap["failed"]
    if failed_bootstrap != {"code": "invalid-config", "format": 1, "launch_id": "2-1-fedcba9876543210fedcba9876543210", "selection": current_state["selection"], "state": "failed"}:
        raise Bad("bootstrap failed record oracle mismatch")

    machine = read(FIX / "publication-state-machine.canonical.json")
    stage = ["prepared", "copied", "verified", "bundle-published", "closed"]
    switch = ["prepared", "link-published", "previous-published", "current-published", "closed"]
    if set(machine) != {"activate", "closed_cli_retry", "closed_serve_recovery", "corruption_exit", "decision_of_record", "format", "operation_shapes", "rollback", "stage"} or machine["stage"] != stage or machine["activate"] != switch or machine["rollback"] != switch or machine["decision_of_record"] != "active-link-after-link-published" or machine["corruption_exit"] != 75:
        raise Bad("publication recovery machine mismatch")
    shapes = machine["operation_shapes"]
    if set(shapes) != {"stage", "activate_first", "rollback_cli", "rollback_auto"}:
        raise Bad("selector operation actor matrix")
    if set(shapes["stage"]) != {"actor", "command_sha256", "format", "generation", "op_id", "phase", "to", "type"} or shapes["stage"]["type"] != "stage" or shapes["stage"]["actor"] != "cli":
        raise Bad("stage operation field presence mismatch")
    if set(shapes["activate_first"]) != {"actor", "command_sha256", "format", "generation", "op_id", "phase", "to", "type"} or shapes["activate_first"]["type"] != "activate" or shapes["activate_first"]["actor"] != "cli" or shapes["activate_first"]["generation"] != 1:
        raise Bad("first activation operation field presence mismatch")
    if set(shapes["rollback_cli"]) != {"actor", "command_sha256", "format", "from", "generation", "op_id", "phase", "reason", "to", "type"} or shapes["rollback_cli"]["actor"] != "cli":
        raise Bad("CLI rollback operation field presence mismatch")
    if set(shapes["rollback_auto"]) != {"actor", "command_sha256", "format", "from", "generation", "launch_id", "op_id", "phase", "reason", "to", "type"} or shapes["rollback_auto"]["actor"] != "serve" or not re.fullmatch(r"[1-9][0-9]*-[1-9][0-9]*-[0-9a-f]{32}", shapes["rollback_auto"]["launch_id"]):
        raise Bad("automatic rollback operation field presence mismatch")
    if any(not re.fullmatch(r"[0-9a-f]{64}", row["command_sha256"]) for row in shapes.values()):
        raise Bad("selector operation command digest")
    if shapes["stage"]["to"] != current_state["selection"] or shapes["rollback_cli"]["from"] != current_state["selection"] or shapes["rollback_auto"]["from"] != current_state["selection"]:
        raise Bad("operation examples do not bind selected manifest")
    closed_retry = machine["closed_cli_retry"]
    if (closed_retry.get("actor") != "cli" or closed_retry.get("command_sha256") != shapes["rollback_cli"]["command_sha256"] or
            closed_retry.get("generation") != closed_retry.get("response", {}).get("current", {}).get("generation") or closed_retry.get("response", {}).get("operation") != "rollback"):
        raise Bad("closed selector operation is not an exact retry authority")
    closed_serve = machine["closed_serve_recovery"]
    if closed_serve != {"actor": "serve", "command_sha256": shapes["rollback_auto"]["command_sha256"], "generation": 3, "launch_id": shapes["rollback_auto"]["launch_id"]} or "response" in closed_serve:
        raise Bad("automatic rollback recovery fabricates CLI response")

    observability = read(FIX / "observability.canonical.json")
    if observability["metric_label_keys"] != ["classification", "component"] or observability["support_files"] != ["logs.jsonl", "manifest.canonical.json", "metrics.canonical.json"]:
        raise Bad("observability bounded labels/support files")
    expected_file_safety = {"mode": "0600", "nofollow": True, "owner": "effective-uid", "regular": True, "rotation_serialized": True, "same_inode": True, "scope": "supervisor.jsonl"}
    expected_append_failure = {"fallback": {"code": "operational-log-unavailable", "format": 1, "message": "Operational log append failed"}, "health": "not-ready", "metric_readiness": 0}
    if observability["log"].get("file_safety") != expected_file_safety or observability["log"].get("append_failure") != expected_append_failure:
        raise Bad("operational log path/failure contract mismatch")
    required_metric_names = [
        "readiness", "active_workers", "threads_parked", "threads_running",
        "threads_settled", "provider_leases", "append_latency_ms",
        "barrier_latency_ms", "tail_repairs_total", "corruptions_total",
        "provider_outcomes_total", "tool_outcomes_total",
        "endpoint_requests_total", "endpoint_stream_pressure",
        "sweep_duration_ms", "restarts_total", "rollbacks_total",
    ]
    if observability["metric_names"] != required_metric_names:
        raise Bad("metric registry incomplete")
    field_registry = observability["log"].get("field_keys_by_code")
    if not isinstance(field_registry, dict) or not field_registry or any(keys != sorted(set(keys)) for keys in field_registry.values()):
        raise Bad("log field registry is not closed and canonical")
    if observability["log"].get("selector_registry") != "selector.canonical.json#/operational":
        raise Bad("observability selector registry reference mismatch")
    selector_field_registry = {
        row["code"]: row["fields"] for row in operational["codes"]
    }
    selector_correlation_registry = {
        row["code"]: row["correlation"] for row in operational["codes"]
    }
    selector_severity_registry = {
        row["code"]: row["severity"] for row in operational["codes"]
    }
    for code in expected_attributable:
        selector_field_registry.setdefault(code, operational["failure_rows"]["attributable"]["fields"])
        selector_correlation_registry.setdefault(code, operational["failure_rows"]["attributable"]["correlation"])
        selector_severity_registry.setdefault(code, operational["failure_rows"]["attributable"]["severity"])
    for code in expected_environment + ["launcher-interrupted"]:
        selector_field_registry.setdefault(code, operational["failure_rows"]["environment"]["fields"])
        selector_correlation_registry.setdefault(code, operational["failure_rows"]["environment"]["correlation"])
        selector_severity_registry.setdefault(code, operational["failure_rows"]["environment"]["severity"])
    correlation_keys = observability["log"].get("correlation_keys")
    expected_correlation_keys = ["attempt", "event_seq", "generation", "launch_id", "manifest_sha256", "operation_id", "request_id", "run", "session_id"]
    if correlation_keys != expected_correlation_keys:
        raise Bad("log correlation registry mismatch")
    selection_codes = observability["log"].get("selection_correlated_codes")
    if not isinstance(selection_codes, list) or selection_codes != sorted(set(selection_codes)):
        raise Bad("selection-correlated code registry mismatch")
    expected_codes = set()
    for case in declared:
        case_result = read(FIX / f"cases/{case}.expected.canonical.json")
        expected_codes.update(case_result["observability"]["codes"])
    required_log_codes = expected_codes | set(BOOTSTRAP) | {"launch-failed", "launcher-interrupted"}
    all_log_codes = set(field_registry) | set(selector_field_registry)
    if not required_log_codes <= all_log_codes:
        raise Bad(f"observability codes missing field registry rows: {sorted(required_log_codes-all_log_codes)}")
    forbidden = observability["forbidden_canaries"]
    result = read(FIX / "cases/observability-redaction.expected.canonical.json")
    if result["observability"]["forbidden"] != forbidden:
        raise Bad("redaction canary registry mismatch")

    support = FIX / "support-bundle"
    support_names = {"manifest.canonical.json", "logs.jsonl", "metrics.canonical.json"}
    if {path.name for path in support.iterdir()} != support_names:
        raise Bad("support bundle file set mismatch")
    support_manifest = read(support / "manifest.canonical.json", allow_lf=True)
    entries = support_manifest["files"]
    if [item["path"] for item in entries] != ["logs.jsonl", "metrics.canonical.json"]:
        raise Bad("support manifest path order")
    for item in entries:
        payload = (support / item["path"]).read_bytes()
        if item["bytes"] != len(payload) or item["sha256"] != hashlib.sha256(payload).hexdigest():
            raise Bad(f"support manifest digest mismatch for {item['path']}")
    log_lines = (support / "logs.jsonl").read_bytes().splitlines()
    if not log_lines:
        raise Bad("support logs are empty")
    for line in log_lines:
        record = json.loads(line, object_pairs_hook=pairs)
        if line != canonical(record) or set(record) != set(observability["log"]["required"]):
            raise Bad("support log schema/canonical mismatch")
        is_selector = record["component"] == "selector"
        allowed = (
            selector_field_registry.get(record["code"])
            if is_selector
            else field_registry.get(record["code"])
        )
        if allowed is None or sorted(record["fields"]) != allowed:
            raise Bad("support log uses undeclared fields")
        if not set(record["correlation"]) <= set(correlation_keys):
            raise Bad("support log uses undeclared correlation fields")
        selector_shape = selector_correlation_registry.get(record["code"]) if is_selector else None
        if record["code"] in selection_codes or selector_shape in {"complete-child", "complete-child-plus-operation"}:
            launch = record["correlation"]
            required_launch = {"launch_id", "attempt", "generation", "manifest_sha256"}
            if not required_launch <= set(launch) or not re.fullmatch(r"[1-9][0-9]*-[1-9][0-9]*-[0-9a-f]{32}", launch["launch_id"]) or launch["attempt"] < 1 or launch["generation"] < 1 or not re.fullmatch(r"[0-9a-f]{64}", launch["manifest_sha256"]):
                raise Bad("support log omits frozen launch attribution")
        if is_selector:
            if record["severity"] != selector_severity_registry.get(record["code"]):
                raise Bad("support selector log severity mismatch")
            if selector_shape == "empty" and record["correlation"]:
                raise Bad("support selector lifecycle log has child correlation")
            if selector_shape == "operation" and set(record["correlation"]) != {"operation_id"}:
                raise Bad("support selector operation log correlation mismatch")
            if selector_shape == "complete-child-plus-operation" and "operation_id" not in record["correlation"]:
                raise Bad("support selector rollback lacks operation correlation")
    support_codes = [json.loads(line)["code"] for line in log_lines]
    if support_codes != result["observability"]["codes"]:
        raise Bad("support log bytes do not realize the redaction case codes")
    launch_ids = [json.loads(line)["correlation"].get("launch_id") for line in log_lines if "launch_id" in json.loads(line)["correlation"]]
    if len(set(launch_ids)) != 1:
        raise Bad("support log case does not bind one frozen launch identity")
    metric_snapshot = read(support / "metrics.canonical.json", allow_lf=True)
    if set(metric_snapshot) != {"v", "ts", "metrics"} or metric_snapshot["v"] != 1:
        raise Bad("support metric snapshot envelope")
    metric_names = [metric.get("name") for metric in metric_snapshot["metrics"]]
    if metric_names != sorted(set(metric_names)) or not metric_names:
        raise Bad("support metric rows are empty, duplicated or not name-sorted")
    for metric in metric_snapshot["metrics"]:
        if (
            set(metric) != {"name", "type", "value", "labels"}
            or metric["type"] not in {"gauge", "counter", "histogram"}
            or isinstance(metric["value"], bool)
            or not isinstance(metric["value"], (int, float))
            or metric["value"] < 0
            or metric["name"] not in observability["metric_names"]
            or not set(metric["labels"]) <= set(observability["metric_label_keys"])
            or any(not isinstance(value, str) for value in metric["labels"].values())
        ):
            raise Bad("support metric name/labels escape registry")
    support_bytes = b"".join(path.read_bytes() for path in support.iterdir())
    for canary in forbidden:
        if canary.encode() in support_bytes:
            raise Bad(f"support bundle leaks canary {canary}")

    coverage = (FIX / "COVERAGE.md").read_text()
    for gate, cases in GATE_MAP.items():
        if f"Gate {gate}" not in coverage:
            raise Bad(f"coverage omits Gate {gate}")
        for case in cases:
            if f"`{case}`" not in coverage:
                raise Bad(f"coverage omits {case}")

    spec = (ROOT / "spec/deployment.md").read_text().lower()
    if any(word in spec for word in ("cargo crate", "rust enum", "rust struct")):
        raise Bad("deployment contract leaked implementation language")

    print(f"deployment fixtures: {len(declared)} cases, {len(actual)} case artifacts, Gates 71-76 and deployment oracles covered")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Bad as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(1)

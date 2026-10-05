#!/usr/bin/env python3
"""Run real provider/worker turns, retaining evidence for every matrix cell.

Credentials stay in child environment, never argv or evidence. Missing credentials,
zero dispatched attempts, timeouts, and missing final answers cannot pass.
"""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import shutil

ROOT = Path(__file__).resolve().parents[1]
SCENARIOS = {
    "legacy_simple_task": ("Call tool.task to start tasks:\n- Task_Simple: query the capital of China, write result to docs/0.md", "Beijing", "task,apply_patch,read"),
    "legacy_simple_output": ("What is the capital of Japan?", "Tokyo", ""),
    "legacy_compile_python": ("compile and run src/1.py, src/2.py and src/3.py.", "", "shell"),
    "legacy_weather_skill": ("Load and follow the weather skill. Format this supplied observation; do not fetch or invent any other weather data: Shanghai, 28°C, light rain, humidity 81%, observed at 2026-08-22 06:00 Asia/Shanghai.", "上海天气｜28°C｜小雨｜湿度81%｜观测时间 2026-08-22 06:00 Asia/Shanghai", "skill_explorer,skill"),
    "skill_discovery": ("Use skill_explorer to find the live proof skill, then call skill using the exact offered name. Follow the loaded skill instruction to produce your final answer. Do not guess the answer before loading it.", "CF_SKILL_BODY_VERIFIED_7291", "skill_explorer,skill"),
    "validation_repair_final": ("This is a validation repair test. The final deliverables are proof.txt containing KERNEL_FILE_OK and a final answer exactly KERNEL_FILE_OK, without extra prose. To exercise validation: before any validation.feedback, create the correct file with apply_patch, read it, but deliberately submit the incorrect candidate answer WRONG_FINAL. Do not fix that candidate early. After validation.feedback arrives, preserve the correct file and repair the final answer to exactly KERNEL_FILE_OK in this same session.", "KERNEL_FILE_OK", "apply_patch,read"),
    "legacy_write": ("Use apply_patch to create docs/live-smoke.md. The file content must be exactly LIVE_OK with no trailing newline. After the file is written, read it, then provide the final answer exactly LIVE_OK_DONE.", "LIVE_OK_DONE", "apply_patch,read"),
    "legacy_think_three": ("Solve this in THREE separate reasoning steps. For EACH step you MUST FIRST make its own separate call to the think tool, then move on. The thought strings must begin with these exact distinct markers in order: STEP_1_CLOSING_SPEED, STEP_2_MEETING_TIME, STEP_3_DISTANCE_CHECK. Do not repeat a prior think argument. Only AFTER all three successful think results, give the final numeric answer. Trains: X leaves city A toward B at 60 km/h; Y leaves city B (300 km from A) toward A at 40 km/h; both start at the same time. Step 1 (think): compute the combined closing speed. Step 2 (think): compute the time to meet from 300 km and that speed. Step 3 (think): sanity-check each train's distance at that time. Then state the meeting time in hours. Return only the number 3 as the final answer.", "3", "think"),
    "legacy_text": ("Reply with exactly this token and nothing else:\nLIVE_OK_DONE", "LIVE_OK_DONE", ""),
    "text": ("Reply with exactly KERNEL_LIVE_OK.", "KERNEL_LIVE_OK", ""),
    "goal_resume": ("This is a goal continuation test. In the first turn, do not call any tool and give the final answer ROUND_ONE. On the automatically resumed turn, call set_goal_state with state complete and then give the final answer GOAL_LIVE_OK. Do not use the completion marker in the first turn.", "GOAL_LIVE_OK", ""),
    "tool_continuation": ("First call the think tool with thought 'live continuation check'. After the tool result, reply exactly KERNEL_TOOL_OK.", "KERNEL_TOOL_OK", "think"),
    "repeated_context_get": ("This is a repeated context_get test. Call context_get with record_ids [1] and reason [audit] exactly THREE consecutive times with identical arguments. Omit optional arguments or keep them identical. Issue one call per response, waiting for each result before the next call. After the third successful result, call no more tools and finish with exactly KERNEL_CONTEXT_REPEAT_OK.", "KERNEL_CONTEXT_REPEAT_OK", "context_get"),
    "repeated_read": ("This is a repeated identical read test. Read repeated.txt exactly THREE consecutive times using identical path, offset 1, and limit 10. Make one read call per response and wait for its result before the next call. The file already exists in the workspace. After the third successful result, call no more tools and reply exactly KERNEL_REPEATED_OK.", "KERNEL_REPEATED_OK", "read"),
    "parallel_tools": ("This is a parallel tool batch test. In ONE response, issue TWO independent think calls, one with thought LEFT and one with thought RIGHT. Do not wait for either result before emitting the other call. After both tools return, reply exactly KERNEL_PARALLEL_OK.", "KERNEL_PARALLEL_OK", "think"),
    "shell_continuation": ("Use the shell tool to run 'sleep 1; printf KERNEL_SHELL_OK'. Set command to that string, program and args to JSON null (not the string null), steps to [], failure_policy to stop_on_error, max_duration_ms to 10000, writable_paths and artifact_outputs to []. Wait for the result, then reply with only that exact marker.", "KERNEL_SHELL_OK", "shell"),
    "shell_artifact": ("This is a shell artifact version test. Use shell command printf KERNEL_FILE_OK > proof.txt with program and args null, steps [], failure_policy stop_on_error, writable_paths [proof.txt], artifact_outputs [{path: proof.txt}], max_duration_ms 10000, and the workspace directory as working_directory. Read proof.txt afterward and reply exactly KERNEL_FILE_OK.", "KERNEL_FILE_OK", "shell,read"),
    "queued_validation": ("This is a queued validation test. Use apply_patch to create proof.txt containing KERNEL_FILE_OK in the workspace. Then read it. Only after reading it, return exactly KERNEL_FILE_OK.", "KERNEL_FILE_OK", "apply_patch,read"),
    "write_read": ("Use apply_patch to create proof.txt containing KERNEL_FILE_OK in the workspace. Then use read to inspect it. Only after reading it, return that exact marker.", "KERNEL_FILE_OK", "apply_patch,read"),
    "validation_repair": ("The final deliverable must be proof.txt containing exactly KERNEL_FILE_OK. This is a validation repair test: before any validation.feedback exists, deliberately create the first draft containing exactly WRONG and answer exactly KERNEL_DRAFT. Do not repair it early; the validator must reject this draft against the final deliverable requirement. When validation.feedback arrives, repair proof.txt in this same session using apply_patch, read it to verify, and answer exactly KERNEL_FILE_OK.", "KERNEL_FILE_OK", "apply_patch,read"),
    "memory": ("First use note to stage a project-scoped answer candidate with fact the live test marker is KERNEL_MEMORY_OK and commit later. Then use memorize to add that fact, consuming the actual candidate ID returned by note. Finally use recall with the exact saved key and confirm the retrieved fact. Only after all three tools succeed, reply exactly KERNEL_MEMORY_OK.", "KERNEL_MEMORY_OK", "note,memorize,recall"),
}

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--providers", default="all", help="comma-separated provider id substrings")
    parser.add_argument("--scenarios", default=",".join(name for name in SCENARIOS if name != "goal_resume"))
    parser.add_argument("--config", type=Path, default=os.environ.get("TEKES_PROVIDERS_CONFIG"), required="TEKES_PROVIDERS_CONFIG" not in os.environ, help="providers.json (default: $TEKES_PROVIDERS_CONFIG)")
    parser.add_argument("--secret-name", help="explicit live keys field for an isolated provider run")
    parser.add_argument("--keys", type=Path, default=os.environ.get("TEKES_LIVE_KEYS_FILE"), required="TEKES_LIVE_KEYS_FILE" not in os.environ, help="Key file (default: $TEKES_LIVE_KEYS_FILE)")
    parser.add_argument("--timeout", type=int, default=240, help="per-scenario wall timeout in seconds")
    parser.add_argument("--turn-wall-timeout", type=int, default=180, help="worker turn wall limit in seconds")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--toolchain-root", type=Path, action="append", default=[], help="Explicit read-only toolchain installation")
    parser.add_argument("--goal", action="store_true", help="Bind an independent goal for each live scenario")
    args = parser.parse_args()
    toolchain_roots = sorted({str(path.resolve(strict=True)) for path in args.toolchain_root})
    if any(not Path(path).is_dir() or ':' in path for path in toolchain_roots):
        parser.error("toolchain roots must be directories without PATH separators")
    if args.timeout <= 0 or args.turn_wall_timeout <= 0: parser.error("timeouts must be positive")
    output = args.output or ROOT/"target/live-kernel"/datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    output.mkdir(parents=True, exist_ok=False)
    secrets = dict(re.findall(r'^\s*"([\w]+)"\s*:\s*"([^"\r\n]+)"', args.keys.read_text(), re.M)) if args.keys.exists() else {}
    providers = json.loads(args.config.read_text())["providers"]
    selected = [p for p in providers if args.providers == "all" or any(part in p["id"] for part in args.providers.split(','))]
    if not selected:
        raise SystemExit("No matching provider; zero tests is not success")
    scenarios = args.scenarios.split(',')
    for scenario in scenarios:
        if scenario not in SCENARIOS:
            raise SystemExit(f"Unknown scenario: {scenario}")
    if "goal_resume" in scenarios and not args.goal:
        parser.error("goal_resume requires --goal")
    helpers = {}
    for package, name in [("tools", "tekes-helper"), ("workspace-service", "tekes-workspace-service")]:
        built = subprocess.run(["cargo", "build", "-p", package, "--bin", name,
            "--locked", "--message-format=json"], cwd=ROOT, check=True,
            stdout=subprocess.PIPE, text=True)
        paths = {entry["executable"] for line in built.stdout.splitlines()
            if line.strip() and (entry := json.loads(line)).get("reason") == "compiler-artifact"
            and entry.get("executable") and entry.get("target", {}).get("name") == name}
        if len(paths) != 1:
            raise SystemExit(f"Expected one Cargo executable for {name}, received {len(paths)}")
        helpers[name] = Path(paths.pop())
    build = subprocess.run(["cargo","test","-p","tekes-worker","--bin","tekes-worker","--no-run","--locked","--message-format=json"], cwd=ROOT, check=True, stdout=subprocess.PIPE, text=True)
    artifacts = [json.loads(line) for line in build.stdout.splitlines() if line.strip()]
    binaries = {entry["executable"] for entry in artifacts
        if entry.get("reason") == "compiler-artifact" and entry.get("executable")
        and entry.get("target", {}).get("name") == "tekes-worker"
        and entry.get("profile", {}).get("test") is True}
    if len(binaries) != 1:
        raise SystemExit(f"Expected one Cargo test executable, received {len(binaries)}")
    binary = Path(binaries.pop())
    frozen = output/"runtime"
    frozen.mkdir()
    shutil.copy2(binary, frozen/"tekes-worker-live-test")
    for name, path in helpers.items():
        shutil.copy2(path, frozen/name)
    binary = frozen/"tekes-worker-live-test"
    binary_sha256 = hashlib.sha256(binary.read_bytes()).hexdigest()
    results = []
    for provider in selected:
        dialect = provider["dialect"]
        field = args.secret_name or ("cloudflare" if provider["endpoint_owner"] == "cloudflare" else {"deepseek_responses_v1":"deepSeekKey", "glm_chat_v1":"glm", "kimi_chat_v1":"kimi", "google_generation_v1":"googleKey", "openai_responses_v1":"openAIKey", "anthropic_messages_v1":"anthropicKey"}.get(dialect, ""))
        key = os.environ.get("TEKES_KERNEL_LIVE_KEY") or secrets.get(field)
        for scenario in scenarios:
            cell = output/provider["id"]/scenario
            cell.mkdir(parents=True)
            row = {"provider":provider["id"], "model":provider["models"][0]["id"], "scenario":scenario, "artifact":str(cell), "binary_sha256":binary_sha256, "toolchain_roots":toolchain_roots}
            if not key:
                row["status"] = "blocked_missing_credential"
            else:
                config_path = cell/"provider.json"
                config_path.write_text(json.dumps(provider, indent=2))
                prompt, expected, tools = SCENARIOS[scenario]
                env = dict(os.environ, TEKES_KERNEL_LIVE_MAX_WALL_SECONDS=str(args.turn_wall_timeout), TEKES_KERNEL_LIVE_PROVIDER=str(config_path), TEKES_KERNEL_LIVE_KEY=key, TEKES_KERNEL_LIVE_SKILL_FIXTURE=scenario, TEKES_KERNEL_LIVE_PROMPT=prompt, TEKES_KERNEL_LIVE_EXPECT=expected, TEKES_KERNEL_LIVE_TOOLS=tools, TEKES_KERNEL_LIVE_ARTIFACT=str(cell), TEKES_KERNEL_LIVE_GOAL="1" if args.goal else "0")
                env["TEKES_KERNEL_LIVE_TOOLCHAIN_ROOTS"] = json.dumps(toolchain_roots)
                try:
                    run = subprocess.run([str(binary),"provider_context_tests::live_kernel_turn","--exact","--ignored","--nocapture"], cwd=ROOT, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=args.timeout)
                    # Defense in depth: redact every loaded credential before writing logs.
                    log = run.stdout.decode(errors="replace")
                    for secret in [key, *secrets.values()]:
                        if len(secret) > 8:
                            log = log.replace(secret, "[REDACTED]")
                    (cell/"test.log").write_text(log)
                    row["status"] = "passed" if run.returncode == 0 and "1 passed" in log and (cell/"result.json").exists() else "failed"
                except subprocess.TimeoutExpired as error:
                    log = (error.stdout or b"").decode(errors="replace")
                    for secret in [key, *secrets.values()]:
                        if len(secret) > 8:
                            log = log.replace(secret, "[REDACTED]")
                    (cell/"test.log").write_text(log + f"\nRunner timeout after {args.timeout} seconds.\n")
                    row["status"] = "failed_timeout"
                    row["partial_ledger"] = str(cell/"runtime-thread"/"main.jsonl")
            results.append(row)
            (output/"matrix.json").write_text(json.dumps(results, indent=2))
            print(f"{row['provider']} {scenario}: {row['status']}", flush=True)
    print(f"Evidence: {output}")
    return 0 if all(row["status"] == "passed" for row in results) else 1

if __name__ == "__main__":
    raise SystemExit(main())

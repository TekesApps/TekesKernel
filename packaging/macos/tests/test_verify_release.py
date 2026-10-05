import importlib.util
import os
from pathlib import Path
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "verify-release.py"
SPEC = importlib.util.spec_from_file_location("verify_release", SCRIPT)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class SelectorConformanceProbeTests(unittest.TestCase):
    MANIFEST = {
        "architecture": "aarch64",
        "conformance_sha256": "9" * 64,
        "version": "0.1.0-rc.5",
    }

    def selector(self, directory: str, body: str) -> Path:
        path = Path(directory) / "tekes-selector"
        path.write_text("#!/bin/sh\n" + body)
        path.chmod(0o755)
        return path

    def test_exact_canonical_probe_is_accepted(self) -> None:
        reply = MODULE.canonical(
            {
                "architecture": "aarch64",
                "conformance_sha256": "9" * 64,
                "format": 1,
                "operation": "describe-conformance",
                "version": "0.1.0-rc.5",
            }
        ).decode()
        with tempfile.TemporaryDirectory() as directory:
            selector = self.selector(directory, f"printf '%s\\n' '{reply}'\n")
            MODULE.verify_selector_conformance_probe(selector, self.MANIFEST)

    def test_missing_embedded_conformance_is_rejected(self) -> None:
        reply = MODULE.canonical(
            {
                "error": {
                    "code": "invalid-state",
                    "details": {"state": "conformance-evidence-missing"},
                    "message": "Selector state does not permit the operation",
                }
            }
        ).decode()
        with tempfile.TemporaryDirectory() as directory:
            selector = self.selector(directory, f"printf '%s\\n' '{reply}'\nexit 66\n")
            with self.assertRaisesRegex(MODULE.InvalidRelease, "differs from release manifest"):
                MODULE.verify_selector_conformance_probe(selector, self.MANIFEST)

    def test_unknown_reply_field_is_rejected(self) -> None:
        reply = MODULE.canonical(
            {
                "architecture": "aarch64",
                "conformance_sha256": "9" * 64,
                "format": 1,
                "operation": "describe-conformance",
                "unexpected": True,
                "version": "0.1.0-rc.5",
            }
        ).decode()
        with tempfile.TemporaryDirectory() as directory:
            selector = self.selector(directory, f"printf '%s\\n' '{reply}'\n")
            with self.assertRaisesRegex(MODULE.InvalidRelease, "differs from release manifest"):
                MODULE.verify_selector_conformance_probe(selector, self.MANIFEST)

    def test_probe_receives_no_inherited_environment(self) -> None:
        reply = MODULE.canonical(
            {
                "architecture": "aarch64",
                "conformance_sha256": "9" * 64,
                "format": 1,
                "operation": "describe-conformance",
                "version": "0.1.0-rc.5",
            }
        ).decode()
        with tempfile.TemporaryDirectory() as directory:
            selector = self.selector(
                directory,
                f'[ -z "$TEKES_MALICIOUS_ENV" ] || exit 70\nprintf \'%s\\n\' \'{reply}\'\n',
            )
            os.environ["TEKES_MALICIOUS_ENV"] = "must-not-cross"
            try:
                MODULE.verify_selector_conformance_probe(selector, self.MANIFEST)
            finally:
                del os.environ["TEKES_MALICIOUS_ENV"]


if __name__ == "__main__":
    unittest.main()

#!/usr/bin/env python3
import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures" / "plugins"


def read_canonical(path: Path):
    raw = path.read_bytes()
    value = json.loads(raw)
    canonical = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":")).encode() + b"\n"
    if raw != canonical:
        raise SystemExit(f"noncanonical plugin fixture: {path}")
    return value


def main():
    lifecycle = read_canonical(FIXTURES / "lifecycle.canonical.json")
    manifest = read_canonical(FIXTURES / "tekes-computer-use-manifest.canonical.json")
    corpus = json.loads((ROOT / "fixtures" / "manifest.json").read_bytes())["corpora"]
    if corpus.get("plugins") != [
        "lifecycle.canonical.json", "tekes-computer-use-manifest.canonical.json"
    ]:
        raise SystemExit("fixtures/manifest.json plugin parity mismatch")
    if lifecycle["gates"] != {
        "81": "hermetic-package-carrier-compatibility",
        "82": "hermetic-signature-trust-grants-fail-closed",
        "83": "lifecycle-and-collision-ownership",
        "84": "transaction-recovery",
        "85": "typed-management-inert-projection",
    } or lifecycle["projection"] != {"launchable": False} or lifecycle["executable_relation"] != {
        "generation": "packageDigest",
        "reference": ["pluginId", "componentId"],
    } or lifecycle["qualification"] != {
        "hermetic": "carrier-only",
        "reference": "requires-explicit-archive-and-macos-codesign",
    }:
        raise SystemExit("Slice 12 lifecycle fixture drift")
    if manifest["id"] != "com.tekes.computer-use" or manifest["version"] != "0.1.5":
        raise SystemExit("Computer Use identity drift")
    if [component["id"] for component in manifest["components"]] != [
        "computer-use", "computer-use-guidance", "computer-use-assets"
    ]:
        raise SystemExit("Computer Use component projection drift")
    print("Slice 12 hermetic plugin carrier fixtures: passed")


if __name__ == "__main__":
    main()

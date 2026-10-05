#!/usr/bin/env python3
"""Dump one real SWE-bench-Verified instance into swe-bench-verified-smoke.json.

The fixture ships as a REPLACE_ME template on purpose — we don't commit a fabricated
commit SHA. Populate it from the real dataset before running the smoke:

    pip install datasets
    python3 fetch_instance.py                         # first Verified instance
    python3 fetch_instance.py astropy__astropy-12907  # a specific instance id

Pick a cheap, fast-installing instance for the smoke (a small pure-Python repo resolves
Docker setup far quicker than numpy/scipy-heavy ones).
"""
import json
import sys

from datasets import load_dataset  # type: ignore

FIELDS = [
    "instance_id",
    "repo",
    "base_commit",
    "problem_statement",
    "patch",
    "test_patch",
    "FAIL_TO_PASS",
    "PASS_TO_PASS",
    "environment_setup_commit",
    "version",
]


def main() -> None:
    wanted = sys.argv[1] if len(sys.argv) > 1 else None
    ds = load_dataset("princeton-nlp/SWE-bench_Verified", split="test")

    row = None
    if wanted:
        for candidate in ds:
            if candidate["instance_id"] == wanted:
                row = candidate
                break
        if row is None:
            sys.exit(f"instance_id {wanted!r} not found in SWE-bench_Verified")
    else:
        row = ds[0]

    out = {key: row.get(key) for key in FIELDS}
    with open("swe-bench-verified-smoke.json", "w") as handle:
        json.dump(out, handle, indent=2)
    print(f"wrote swe-bench-verified-smoke.json for {out['instance_id']}")


if __name__ == "__main__":
    main()

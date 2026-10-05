#!/usr/bin/env python3
"""Run one production prepare and accept only exit-75 canonical evidence."""

from __future__ import annotations

import json
import os
import signal
import subprocess
import sys
import uuid


def main() -> int:
    if len(sys.argv) < 3 or sys.argv[1] not in {"72", "76"}:
        print("usage: check-production-prepare.py 72|76 COMMAND [ARG...]", file=sys.stderr)
        return 64
    gate = int(sys.argv[1])
    process = subprocess.Popen(
        sys.argv[2:],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        start_new_session=True,
    )
    try:
        stdout, stderr = process.communicate(timeout=600)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait()
        return 124
    if process.returncode != 75 or stderr:
        sys.stderr.buffer.write(stderr)
        if process.returncode != 75:
            print(
                f"production prepare runner exited {process.returncode}, expected 75",
                file=sys.stderr,
            )
        return 1
    try:
        value = json.loads(stdout)
        canonical = json.dumps(
            value, ensure_ascii=False, sort_keys=True, separators=(",", ":")
        ).encode() + b"\n"
        operation = str(uuid.UUID(value["operation"]))
        pre_boot_session = str(uuid.UUID(value["pre_boot_session"]))
    except (KeyError, TypeError, ValueError, json.JSONDecodeError):
        print("invalid production prepare evidence", file=sys.stderr)
        return 1
    if (
        stdout != canonical
        or set(value)
        != {"format", "gate", "operation", "phase", "pre_boot_session", "resume_label"}
        or value["format"] != 1
        or value["gate"] != gate
        or value["phase"] != "reboot-required"
        or value["operation"] != operation
        or value["pre_boot_session"] != pre_boot_session
        or value["resume_label"] != f"com.tekes.kernel.production-uat.{operation}"
    ):
        print("production prepare evidence differs from the closed contract", file=sys.stderr)
        return 1
    sys.stdout.buffer.write(stdout)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

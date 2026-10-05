#!/usr/bin/env python3
"""Probe the installed supervisor's real fd3 bootstrap/build attribution."""

from __future__ import annotations

import argparse
import json
import os
import signal
import subprocess
from pathlib import Path


REGISTRY_SHA256 = "f1f084f11ee379fd19ff0b62db653e245a088f7055f2146a5e1adf49decf5469"


def canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--supervisor", required=True, type=Path)
    parser.add_argument("--install-root", required=True, type=Path)
    parser.add_argument("--storage-root", required=True, type=Path)
    parser.add_argument("--selected-version", required=True)
    parser.add_argument("--manifest-sha256", required=True)
    args = parser.parse_args()
    if (
        not args.supervisor.is_absolute()
        or not args.install_root.is_absolute()
        or not args.storage_root.is_absolute()
    ):
        parser.error("supervisor, install root, and storage root must be absolute")
    if args.storage_root.name != "threads" or (args.storage_root / "threads").exists():
        parser.error("storage root must be the exact non-nested threads root")

    status_read, status_write = os.pipe()
    lifetime_read, lifetime_write = os.pipe()

    def install_fds() -> None:
        os.dup2(status_write, 3)
        os.dup2(lifetime_read, 4)

    command = [
        str(args.supervisor),
        "--install-root", str(args.install_root),
        "--storage-root", str(args.storage_root),
        "--listen", "127.0.0.1:7347",
        "--selected-version", args.selected_version,
        "--selector-generation", "1",
        "--launch-id", "1-1-00000000000000000000000000000000",
        "--manifest-sha256", args.manifest_sha256,
        "--bootstrap-status-fd", "3",
        "--authority-registry-sha256", REGISTRY_SHA256,
        "--launcher-lifetime-fd", "4",
    ]
    process = subprocess.Popen(
        command,
        stdin=subprocess.DEVNULL,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        close_fds=False,
        preexec_fn=install_fds,
        start_new_session=True,
    )
    os.close(status_write)
    os.close(lifetime_read)
    os.close(lifetime_write)
    try:
        _, stderr = process.communicate(timeout=40)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        _, stderr = process.communicate()
        raise SystemExit(f"supervisor bootstrap timed out: {stderr.decode(errors='replace')}")
    with os.fdopen(status_read, "rb") as stream:
        bootstrap = stream.read()
    if not bootstrap.endswith(b"\n") or bootstrap.count(b"\n") != 1:
        raise SystemExit("supervisor did not emit exactly one fd3 JSONL row")
    try:
        status = json.loads(bootstrap[:-1])
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise SystemExit(f"invalid supervisor bootstrap row: {error}") from error
    if canonical(status) + b"\n" != bootstrap:
        raise SystemExit("supervisor bootstrap row is not canonical-plus-LF")
    if status.get("selection", {}).get("version") != args.selected_version:
        raise SystemExit("supervisor bootstrap selection version differs")
    if status.get("code") == "selector-mismatch":
        raise SystemExit(
            "supervisor embedded build differs from selected version: "
            + stderr.decode(errors="replace")
        )
    print(json.dumps(
        {"bootstrap": status.get("state"), "embedded_version": args.selected_version, "format": 1},
        sort_keys=True,
        separators=(",", ":"),
    ))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

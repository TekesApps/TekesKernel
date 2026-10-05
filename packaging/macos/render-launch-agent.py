#!/usr/bin/env python3
"""Render the closed Slice-10 LaunchAgent template without ambient inputs."""

from __future__ import annotations

import argparse
import os
import plistlib
import tempfile
from pathlib import Path, PurePosixPath


TOKEN_COUNTS = {
    "@SELECTOR@": 2,
    "@INSTALL_ROOT@": 1,
    "@STORAGE_ROOT@": 1,
    "@STDOUT@": 1,
    "@STDERR@": 1,
}


def absolute_lexical(value: str) -> str:
    path = PurePosixPath(value)
    if not value.startswith("/") or value != path.as_posix() or value != "/" and value.endswith("/"):
        raise ValueError(f"path is not absolute lexical normal form: {value!r}")
    if any(part in {"", ".", ".."} for part in path.parts[1:]):
        raise ValueError(f"path has a forbidden component: {value!r}")
    return value


def render(template: bytes, substitutions: dict[str, str]) -> bytes:
    for token, count in TOKEN_COUNTS.items():
        encoded = token.encode()
        if template.count(encoded) != count:
            raise ValueError(f"template must contain {token} exactly {count} time(s)")
        template = template.replace(encoded, substitutions[token].encode())
    if any(token.encode() in template for token in TOKEN_COUNTS):
        raise ValueError("unresolved LaunchAgent token")

    plist = plistlib.loads(template)
    expected_keys = {
        "Label", "Program", "ProgramArguments", "RunAtLoad", "KeepAlive",
        "ProcessType", "StandardErrorPath", "StandardOutPath",
    }
    if set(plist) != expected_keys:
        raise ValueError("rendered LaunchAgent has fields outside the closed registry")
    selector = substitutions["@SELECTOR@"]
    if plist["Program"] != selector or plist["ProgramArguments"] != [
        selector,
        "--install-root", substitutions["@INSTALL_ROOT@"],
        "serve",
        "--storage-root", substitutions["@STORAGE_ROOT@"],
        "--listen", "127.0.0.1:7347",
    ]:
        raise ValueError("rendered LaunchAgent argv differs from the deployment contract")
    if (
        plist["Label"] != "com.tekes.kernel.supervisor"
        or plist["RunAtLoad"] is not True
        or plist["KeepAlive"] is not True
        or plist["ProcessType"] != "Background"
        or plist["StandardOutPath"] != substitutions["@STDOUT@"]
        or plist["StandardErrorPath"] != substitutions["@STDERR@"]
    ):
        raise ValueError("rendered LaunchAgent ownership/lifetime fields differ")
    return template


def publish(path: Path, payload: bytes) -> None:
    path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
    descriptor, temporary = tempfile.mkstemp(prefix=".launch-agent.", dir=path.parent)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(payload)
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(temporary, 0o644)
        os.replace(temporary, path)
        directory = os.open(path.parent, os.O_RDONLY | os.O_DIRECTORY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        try:
            os.unlink(temporary)
        except FileNotFoundError:
            pass


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--template", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--install-root", required=True)
    parser.add_argument("--storage-root", required=True)
    parser.add_argument("--selector", required=True)
    parser.add_argument("--stdout", required=True)
    parser.add_argument("--stderr", required=True)
    args = parser.parse_args()

    substitutions = {
        "@INSTALL_ROOT@": absolute_lexical(args.install_root),
        "@STORAGE_ROOT@": absolute_lexical(args.storage_root),
        "@SELECTOR@": absolute_lexical(args.selector),
        "@STDOUT@": absolute_lexical(args.stdout),
        "@STDERR@": absolute_lexical(args.stderr),
    }
    publish(args.output, render(args.template.read_bytes(), substitutions))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

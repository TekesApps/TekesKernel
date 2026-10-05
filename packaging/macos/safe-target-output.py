#!/usr/bin/env python3
"""Validate or atomically publish a new artifact below one trusted root.

Every existing path component is opened relative to a trusted directory fd with
O_NOFOLLOW. Missing parent directories are created through that fd. The final
name must not exist, including as a dangling symlink.
"""

from __future__ import annotations

import argparse
import errno
import os
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(message)


def open_artifact_root(path: Path, flags: int) -> int:
    descriptor = os.open("/", flags)
    try:
        components = path.parts[1:]
        for index, component in enumerate(components):
            try:
                child = os.open(component, flags, dir_fd=descriptor)
            except FileNotFoundError:
                if index != len(components) - 1:
                    fail("artifact-root parent does not exist")
                os.mkdir(component, 0o700, dir_fd=descriptor)
                child = os.open(component, flags, dir_fd=descriptor)
            except OSError as error:
                if error.errno in (errno.ELOOP, errno.ENOTDIR):
                    fail("artifact root contains a symlink or non-directory")
                raise
            os.close(descriptor)
            descriptor = child
        return descriptor
    except BaseException:
        os.close(descriptor)
        raise


def open_destination(repository: Path, artifact_root: Path, output: str) -> tuple[int, str]:
    if not output.startswith("/") or os.path.abspath(output) != output:
        fail("output must be absolute and normalized")
    if not artifact_root.is_absolute() or artifact_root != Path(os.path.abspath(artifact_root)):
        fail("artifact root must be absolute and normalized")
    if artifact_root in (Path("/"), repository, repository.parent):
        fail("artifact root is too broad")
    relative = os.path.relpath(output, artifact_root)
    parts = Path(relative).parts
    if not parts or relative == "." or parts[0] == ".." or any(part in ("", ".", "..") for part in parts):
        fail("output must be a descendant of artifact root")

    flags = os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC | os.O_NOFOLLOW
    descriptor = open_artifact_root(artifact_root, flags)
    try:
        for component in parts[:-1]:
            try:
                child = os.open(component, flags, dir_fd=descriptor)
            except FileNotFoundError:
                os.mkdir(component, 0o700, dir_fd=descriptor)
                child = os.open(component, flags, dir_fd=descriptor)
            except OSError as error:
                if error.errno in (errno.ELOOP, errno.ENOTDIR):
                    fail("output parent contains a symlink or non-directory")
                raise
            os.close(descriptor)
            descriptor = child
        try:
            os.stat(parts[-1], dir_fd=descriptor, follow_symlinks=False)
        except FileNotFoundError:
            return descriptor, parts[-1]
        fail("output already exists")
    except BaseException:
        os.close(descriptor)
        raise


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", choices=("validate", "publish"))
    parser.add_argument("--root", required=True)
    parser.add_argument("--artifact-root")
    parser.add_argument("--output", required=True)
    parser.add_argument("--source")
    arguments = parser.parse_args()
    root = Path(arguments.root).resolve(strict=True)
    artifact_root = Path(arguments.artifact_root) if arguments.artifact_root else root / "target"
    descriptor, name = open_destination(root, artifact_root, arguments.output)
    try:
        if arguments.mode == "publish":
            if not arguments.source or not os.path.isabs(arguments.source):
                fail("publish source must be absolute")
            os.rename(arguments.source, name, dst_dir_fd=descriptor)
            os.fsync(descriptor)
        elif arguments.source is not None:
            fail("validate does not accept a source")
    finally:
        os.close(descriptor)


if __name__ == "__main__":
    main()

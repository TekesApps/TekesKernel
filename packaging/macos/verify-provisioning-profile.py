#!/usr/bin/env python3
"""Validate the restricted-entitlement provisioning profile used by a macOS app."""

from __future__ import annotations

import argparse
import datetime as dt
import plistlib
import re
import subprocess
import sys
from pathlib import Path


TEAM = re.compile(r"[A-Z0-9]{10}")
IDENTIFIER = re.compile(r"[A-Za-z0-9][A-Za-z0-9.-]{0,254}")


class InvalidProfile(Exception):
    pass


def decoded_profile(path: Path) -> dict[str, object]:
    result = subprocess.run(
        ["/usr/bin/security", "cms", "-D", "-i", str(path)],
        capture_output=True,
        check=False,
    )
    if result.returncode != 0:
        raise InvalidProfile("profile is not an Apple-signed CMS document")
    try:
        value = plistlib.loads(result.stdout)
    except (ValueError, plistlib.InvalidFileException) as error:
        raise InvalidProfile("profile payload is not a plist") from error
    if not isinstance(value, dict):
        raise InvalidProfile("profile payload is not a dictionary")
    return value


def validate(
    path: Path,
    team_id: str,
    identifier: str,
    access_groups: set[str],
) -> None:
    if not path.is_absolute() or not path.is_file() or path.is_symlink():
        raise InvalidProfile("profile must be an absolute regular file")
    value = decoded_profile(path)
    teams = value.get("TeamIdentifier")
    if not isinstance(teams, list) or team_id not in teams:
        raise InvalidProfile("profile team does not match the installation team")
    expiration = value.get("ExpirationDate")
    if not isinstance(expiration, dt.datetime):
        raise InvalidProfile("profile has no expiration date")
    now = dt.datetime.now(tz=dt.timezone.utc)
    if expiration.tzinfo is None:
        expiration = expiration.replace(tzinfo=dt.timezone.utc)
    if expiration <= now:
        raise InvalidProfile("profile is expired")
    entitlements = value.get("Entitlements")
    if not isinstance(entitlements, dict):
        raise InvalidProfile("profile has no entitlement dictionary")
    application_identifier = f"{team_id}.{identifier}"
    if entitlements.get("com.apple.application-identifier") not in {
        application_identifier, f"{team_id}.*"
    }:
        raise InvalidProfile("profile application identifier mismatch")
    if entitlements.get("com.apple.developer.team-identifier") != team_id:
        raise InvalidProfile("profile entitlement team mismatch")
    groups = entitlements.get("keychain-access-groups")
    if not isinstance(groups, list) or not all(isinstance(group, str) for group in groups):
        raise InvalidProfile("profile has no Keychain access-group grant")
    allowed = set(groups)
    if f"{team_id}.*" in allowed:
        return
    if not access_groups.issubset(allowed):
        raise InvalidProfile("profile does not grant every requested Keychain access group")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--profile", required=True, type=Path)
    parser.add_argument("--team-id", required=True)
    parser.add_argument("--identifier", required=True)
    parser.add_argument("--access-group", action="append", default=[])
    args = parser.parse_args()
    if not TEAM.fullmatch(args.team_id):
        parser.error("team id must be ten uppercase ASCII letters/digits")
    if not IDENTIFIER.fullmatch(args.identifier):
        parser.error("invalid application identifier")
    expected_prefix = f"{args.team_id}."
    groups = set(args.access_group)
    if any(not group.startswith(expected_prefix) for group in groups):
        parser.error("access groups must belong to the installation team")
    validate(args.profile, args.team_id, args.identifier, groups)
    print(f"verified provisioning profile: {args.identifier}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (InvalidProfile, OSError) as error:
        print(f"error: {error}", file=sys.stderr)
        raise SystemExit(65)

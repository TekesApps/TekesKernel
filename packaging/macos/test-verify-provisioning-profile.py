#!/usr/bin/env python3
import datetime as dt
import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location("profile_verifier", Path(__file__).with_name("verify-provisioning-profile.py"))
verifier = importlib.util.module_from_spec(spec)
spec.loader.exec_module(verifier)


class ProfileGrants(unittest.TestCase):
    def test_xcode_wildcard_preserves_team_expiry_and_keychain_checks(self):
        team = "FFJQX338NY"
        identifier = "com.tekes.kernel.supervisor"
        group = f"{team}.com.tekes.shared.endpoint"
        value = {
            "TeamIdentifier": [team],
            "ExpirationDate": dt.datetime.now(dt.timezone.utc) + dt.timedelta(days=1),
            "Entitlements": {
                "com.apple.application-identifier": f"{team}.*",
                "com.apple.developer.team-identifier": team,
                "keychain-access-groups": [f"{team}.*"],
            },
        }
        with tempfile.NamedTemporaryFile() as file, patch.object(verifier, "decoded_profile", return_value=value):
            def validate():
                verifier.validate(Path(file.name), team, identifier, {group})
            validate()
            for grant in ["OTHERTEAM1.*", "*", f"{team}.com.other", f"{team}.com.tekes.*"]:
                value["Entitlements"]["com.apple.application-identifier"] = grant
                with self.assertRaises(verifier.InvalidProfile):
                    validate()
            value["Entitlements"]["com.apple.application-identifier"] = f"{team}.{identifier}"
            validate()
            value["Entitlements"]["keychain-access-groups"] = []
            with self.assertRaises(verifier.InvalidProfile):
                validate()
            value["Entitlements"]["keychain-access-groups"] = [f"{team}.*"]
            value["ExpirationDate"] = dt.datetime.now(dt.timezone.utc) - dt.timedelta(days=1)
            with self.assertRaises(verifier.InvalidProfile):
                validate()


if __name__ == "__main__":
    unittest.main()

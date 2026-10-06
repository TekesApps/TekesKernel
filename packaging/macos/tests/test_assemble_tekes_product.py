import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "assemble-tekes-product.py"
SPEC = importlib.util.spec_from_file_location("assemble_tekes_product", SCRIPT)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ProductAssemblerTests(unittest.TestCase):
    def test_contract_is_the_fixture_the_installer_embeds(self) -> None:
        fixture = SCRIPT.parent / "product-installer/contract.canonical.json"
        self.assertEqual(MODULE.load_contract(), fixture.read_bytes())
        # Keep the assembler's view tied to the installer's argument parser.
        source = (SCRIPT.parents[2] / "crates/product-installer/src/lib.rs").read_text()
        parsed = sorted(re.findall(r'^\s*"([a-z-]+)" => Operation::', source, re.MULTILINE))
        self.assertTrue(parsed)
        self.assertEqual(json.loads(fixture.read_bytes())["operations"], parsed)

    def test_contract_shape_errors_are_rejected(self) -> None:
        valid = json.loads(MODULE.load_contract())
        cases = [
            {**valid, "format": True},
            {**valid, "identifier": "com.example.installer"},
            {**valid, "operations": list(reversed(valid["operations"]))},
            {**valid, "operations": []},
            {**valid, "extra": 1},
        ]
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "contract.canonical.json"
            for case in cases:
                path.write_bytes(MODULE.canonical(case))
                with self.subTest(case=case), self.assertRaises(MODULE.InvalidProduct):
                    MODULE.load_contract(path)

    def test_symlink_is_rejected_before_product_publication(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            (root / "regular").write_bytes(b"bytes")
            (root / "alias").symlink_to(root / "regular")
            with self.assertRaises(MODULE.InvalidProduct):
                MODULE.reject_symlinks(root)

    def test_release_extra_file_fails_before_signature_tools_run(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            (root / "bundle").mkdir()
            (root / "selector").mkdir()
            (root / "install-identity.canonical.json").write_text("{}\n")
            (root / "unexpected").write_bytes(b"forbidden")
            with self.assertRaisesRegex(MODULE.InvalidProduct, "differs from the deployment contract"):
                MODULE.verify_release(root, root / "evidence", SCRIPT.parents[2])


if __name__ == "__main__":
    unittest.main()

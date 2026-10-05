import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


SCRIPT = Path(__file__).resolve().parents[1] / "assemble-tekes-product.py"
SPEC = importlib.util.spec_from_file_location("assemble_tekes_product", SCRIPT)
assert SPEC and SPEC.loader
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ProductAssemblerTests(unittest.TestCase):
    def test_contract_fixture_is_exact_canonical_bytes(self) -> None:
        fixture = SCRIPT.parent / "product-installer/contract.canonical.json"
        self.assertEqual(fixture.read_bytes(), MODULE.canonical(MODULE.CONTRACT))

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

    def test_contract_format_is_integer_not_boolean(self) -> None:
        contract = json.loads(MODULE.canonical(MODULE.CONTRACT))
        self.assertIs(type(contract["format"]), int)
        self.assertEqual(contract["format"], 1)


if __name__ == "__main__":
    unittest.main()

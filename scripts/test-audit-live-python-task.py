#!/usr/bin/env python3
import pathlib
import subprocess
import sys
import tempfile
import unittest

AUDIT = pathlib.Path(__file__).with_name('audit-live-python-task.py')

class PythonArtifactAuditTests(unittest.TestCase):
    def run_fixture(self, hardcoded, package=False):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            src = root/'workspace/src'
            src.mkdir(parents=True)
            (src/'1.py').write_text("def get_country_and_capital():\n    return 'China', 'Beijing'\n")
            sentence = "return 'Beijing is the capital of China.'" if hardcoded else "country, capital = importlib.import_module('1').get_country_and_capital()\n    return f'{capital} is the capital of {country}.'"
            (src/'2.py').write_text('import importlib\ndef make_capital_sentence():\n    '+sentence+'\n')
            (src/'3.py').write_text("import importlib\ndef make_capital_report():\n    country, capital = importlib.import_module('1').get_country_and_capital()\n    return country + ': ' + importlib.import_module('2').make_capital_sentence()\n")
            if package:
                for number in (2, 3):
                    path = src/(str(number)+'.py')
                    path.write_text(path.read_text().replace("import_module('1')", "import_module('src.1')").replace("import_module('2')", "import_module('src.2')"))
            result = subprocess.run([sys.executable,str(AUDIT),str(root)],capture_output=True,text=True,timeout=15)
            return result.returncode, (root/'python-execution.log').read_text()

    def test_real_dependency_calls_pass(self):
        code, log = self.run_fixture(False)
        self.assertEqual(code, 0, log)

    def test_workspace_package_imports_pass(self):
        code, log = self.run_fixture(False, package=True)
        self.assertEqual(code, 0, log)

    def test_hardcoded_sentence_without_dependency_fails(self):
        code, log = self.run_fixture(True)
        self.assertNotEqual(code, 0)
        self.assertIn('sentence did not call source function', log)

if __name__ == '__main__':
    unittest.main()

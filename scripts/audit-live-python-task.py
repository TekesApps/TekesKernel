#!/usr/bin/env python3
"""Execute the three legacy Python artifacts and verify their cross-file calls."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

CHECK = r'''
import importlib.util, json, pathlib, sys
root = pathlib.Path(sys.argv[1]).resolve()
country, capital = sys.argv[2:4]
expected_sentence = f'{capital} is the capital of {country}.'
sys.path[:0] = [str(root.parent), str(root)]
def load(number):
    spec = importlib.util.spec_from_file_location('audit_' + str(number), root / (str(number) + '.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module
modules = [load(n) for n in (1, 2, 3)]
value = modules[0].get_country_and_capital()
assert country in str(value) and capital in str(value), value
seen = set()
def trace(frame, event, arg):
    if event == 'call':
        seen.add((str(pathlib.Path(frame.f_code.co_filename).resolve()), frame.f_code.co_name))
def called(number, name):
    return (str(root / (str(number) + '.py')), name) in seen
sys.setprofile(trace)
sentence = modules[1].make_capital_sentence()
sys.setprofile(None)
assert sentence == expected_sentence, sentence
assert called(1, 'get_country_and_capital'), 'sentence did not call source function'
seen.clear()
sys.setprofile(trace)
report = modules[2].make_capital_report()
sys.setprofile(None)
assert isinstance(report, str) and capital in report and country in report, report
assert called(1, 'get_country_and_capital') and called(2, 'make_capital_sentence'), 'report did not call both dependencies'
print(json.dumps({'status':'passed','sentence':sentence,'report':report}))
'''

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('run', type=Path)
    parser.add_argument('--country', default='China')
    parser.add_argument('--capital', default='Beijing')
    args = parser.parse_args()
    root = args.run.resolve()
    result = subprocess.run([sys.executable, '-I', '-B', '-c', CHECK, str(root/'workspace/src'), args.country, args.capital],
                            cwd=root/'workspace', env={'PATH':os.defpath},
                            capture_output=True, text=True, timeout=10)
    (root/'python-execution.log').write_text(result.stdout + result.stderr)
    if result.returncode:
        raise SystemExit(result.returncode)
    (root/'python-execution.json').write_text(json.dumps(json.loads(result.stdout.strip().splitlines()[-1]),indent=2)+'\n')
    print('Python artifacts and dependency calls passed')

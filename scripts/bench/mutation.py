#!/usr/bin/env python3
"""Mutation score of each run's own test suite against its own implementation module.

usage: mutation.py [--module weighted_ttl_cache.py] <run-dir>...

Each mutant flips one comparison, swaps +/-, and/or, drops a `not`, or swaps a 0/1
constant in <run>/workspace/<module>; the run's own `python3.14 -m unittest` suite
kills it by failing. Duplicate mutants are skipped. Writes mutation.json next to
the first run directory.
"""
import ast
import copy
from concurrent.futures import ProcessPoolExecutor
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile


SWAP = {ast.Lt: ast.LtE, ast.LtE: ast.Lt, ast.Gt: ast.GtE, ast.GtE: ast.Gt,
        ast.Eq: ast.NotEq, ast.NotEq: ast.Eq, ast.In: ast.NotIn, ast.NotIn: ast.In,
        ast.Is: ast.IsNot, ast.IsNot: ast.Is, ast.Add: ast.Sub, ast.Sub: ast.Add,
        ast.And: ast.Or, ast.Or: ast.And}


def sites(tree):
    """Yield (index, description) for every mutable node, in a stable walk order."""
    out = []
    for i, node in enumerate(ast.walk(tree)):
        if isinstance(node, ast.Compare):
            for j, op in enumerate(node.ops):
                if type(op) in SWAP:
                    out.append((i, 'cmp', j))
        elif isinstance(node, (ast.BinOp, ast.AugAssign)) and type(node.op) in SWAP:
            out.append((i, 'bin', 0))
        elif isinstance(node, ast.BoolOp) and type(node.op) in SWAP:
            out.append((i, 'bool', 0))
        elif isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.Not):
            out.append((i, 'not', 0))
        elif isinstance(node, ast.Constant) and type(node.value) is int and node.value in (0, 1):
            out.append((i, 'const', 0))
    return out


class DropNot(ast.NodeTransformer):
    def __init__(self, target):
        self.target = target

    def visit_UnaryOp(self, node):
        self.generic_visit(node)
        return node.operand if node is self.target else node


def mutate(source, site):
    tree = ast.parse(source)
    index, kind, j = site
    node = list(ast.walk(tree))[index]
    if kind == 'cmp':
        node.ops[j] = SWAP[type(node.ops[j])]()
    elif kind in ('bin', 'bool'):
        node.op = SWAP[type(node.op)]()
    elif kind == 'const':
        node.value = 1 - node.value
    elif kind == 'not':
        tree = DropNot(node).visit(tree)
    return ast.unparse(ast.fix_missing_locations(tree))


def run_tests(directory):
    try:
        result = subprocess.run(['python3.14', '-m', 'unittest', '-q'], cwd=directory,
                                capture_output=True, text=True, timeout=60)
        return result.returncode != 0
    except subprocess.TimeoutExpired:
        return True


def score(workspace, module):
    source = (workspace/module).read_text()
    tree = ast.parse(source)
    baseline = ast.unparse(tree)
    with tempfile.TemporaryDirectory() as d:
        for path in workspace.glob('*.py'):
            shutil.copy(path, d)
        (Path(d)/module).write_text(baseline)
        if run_tests(d):
            return {'error': 'suite fails on the unmutated module'}
        killed = survived = 0
        seen = {baseline}
        for site in sites(tree):
            mutant = mutate(source, site)
            if mutant in seen:
                continue
            seen.add(mutant)
            (Path(d)/module).write_text(mutant)
            if run_tests(d):
                killed += 1
            else:
                survived += 1
    total = killed+survived
    return {'mutants': total, 'killed': killed, 'score': round(100*killed/total, 1) if total else None}


def job(item):
    run, module = item
    return Path(run).name, score(Path(run)/'workspace', module)


if __name__ == '__main__':
    args = sys.argv[1:]
    module = 'weighted_ttl_cache.py'
    if args[:1] == ['--module']:
        module, args = args[1], args[2:]
    sys.argv = [sys.argv[0]] + args
    with ProcessPoolExecutor(max_workers=6) as pool:
        results = dict(pool.map(job, [(run, module) for run in sys.argv[1:]]))
    for name, value in results.items():
        print(name, value)
    Path(sys.argv[1]).parent.joinpath('mutation.json').write_text(json.dumps(results, indent=2)+'\n')

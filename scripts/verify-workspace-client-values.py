#!/usr/bin/env python3
"""Decode Kernel carrier results with read-only current Client DTO sources.
This checks value compatibility, not Client endpoint routing or UI integration.
"""
import argparse
import json
from pathlib import Path
import subprocess
import tempfile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--client', type=Path, required=True)
parser.add_argument('--responses', type=Path, required=True)
args = parser.parse_args()
types = {
    'capabilities': 'SessionWorkspaceServiceCapabilities',
    'filesList': 'SessionWorkspaceFileListing',
    'filesSearch': 'SessionWorkspaceFileListing',
    'filesRead': 'SessionWorkspaceFileContent',
    'turnChanges': 'SessionTurnChanges',
    'gitRepository': 'GitRepositoryStateResultV1',
    'gitStatus': 'GitStatusResultV1',
    'gitBranches': 'GitBranchesResultV1',
    'gitLog': 'SessionGitLogPage',
    'gitDiff': 'GitDiffResultV1',
    'gitChangesNavigator': 'GitChangesNavigatorResultV1',
    'gitCommit': 'GitCommitResultV1',
    'gitPush': 'GitPushResultV1',
    'gitChangeBranch': 'GitBranchResultV1',
}
values = json.loads(args.responses.read_text())
assert set(types) <= {x['method'].removeprefix('tekesWorkspace.') for x in values}, 'Missing service response coverage'
sources = [
    'TekesLocalGit/Sources/TekesLocalGit/GitIntegrationContractV1.swift',
    'Tekes/SessionEndpoint/Contract/SessionWorkspaceServices.swift',
    'Tekes/SessionEndpoint/Contract/SessionGitService.swift',
]
# Compile DTOs together. Only the module import is removed; no DTO is rewritten.
# The empty parent protocol stands in for the unrelated session transport surface.
source = 'import Foundation\npublic protocol SessionEndpoint {}\n'
for relative in sources:
    source += (args.client/relative).read_text().replace('import TekesLocalGit\n', '') + '\n'
source += '''
let rows = try JSONSerialization.jsonObject(with: Data(contentsOf: URL(fileURLWithPath: CommandLine.arguments[1]))) as! [[String: Any]]
let decoder = JSONDecoder()
for row in rows {
    let data = try JSONSerialization.data(withJSONObject: row["value"]!)
    switch row["method"] as! String {
'''
for method, name in types.items():
    source += f'    case "tekesWorkspace.{method}": _ = try decoder.decode({name}.self, from: data)\n'
source += '''    default: fatalError("Unknown service method")
    }
}
print("Decoded \\(rows.count) actual carrier responses with Client DTOs")
'''
with tempfile.TemporaryDirectory(prefix='kernel-client-dto-') as directory:
    root = Path(directory)
    (root/'main.swift').write_text(source)
    subprocess.run(['swiftc', str(root/'main.swift'), '-o', str(root/'verify')], check=True)
    subprocess.run([str(root/'verify'), str(args.responses.resolve())], check=True)

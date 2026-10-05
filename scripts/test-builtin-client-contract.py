"""Decode actual Kernel frames with the current sibling Tekes Swift contract."""
import os
from pathlib import Path
import subprocess
import tempfile

repo = Path(__file__).resolve().parent.parent
client = Path(os.environ.get('TEKES_CLIENT_ROOT', str(repo.parent / 'Tekes')))
# These optional service protocols depend on app runtime and Git modules; none
# participates in the five native SessionEndpoint synchronization streams.
excluded = {'SessionGitService.swift', 'WorkspaceServiceEndpoint.swift',
            'SessionProjectedStream.swift'}
sources = sorted(str(path) for path in (client / 'Tekes/SessionEndpoint/Contract').glob('*.swift')
                 if path.name not in excluded)
if not sources:
    raise SystemExit('Current Tekes contract sources are unavailable')
with tempfile.TemporaryDirectory(prefix='tekes-client-contract-') as directory:
    directory = Path(directory)
    runtime_stub = directory / 'RuntimeError.swift'
    runtime_stub.write_text('enum SessionEndpointRuntimeError: Error { case workspaceServiceUnsupported }\n')
    sources.append(str(runtime_stub))
    verifier = directory / 'verify'
    subprocess.run(['swiftc', *sources, str(repo / 'tools/client-contract-check/main.swift'),
                    '-o', str(verifier)], check=True)
    native = directory / 'native'
    subprocess.run(['swiftc', '-swift-version', '6', '-parse-as-library', *sources,
                    *sorted(str(p) for p in (client / 'Tekes/SessionEndpoint/Native').glob('NativeSessionEndpoint*.swift')),
                    str(repo / 'tools/native-client-check/main.swift'), '-o', str(native)], check=True)
    frames = directory / 'frames.json' 
    environment = dict(os.environ, TEKES_BUILTIN_CLIENT_FRAMES=str(frames), TEKES_NATIVE_CLIENT_EXEC=str(native))
    subprocess.run(['python3', str(repo / 'scripts/test-builtin-launch.py')],
                   env=environment, check=True)
    subprocess.run([str(verifier), str(frames)], check=True)

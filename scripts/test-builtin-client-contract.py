"""Decode actual Kernel frames with the current TekesClientKit Swift contract."""
import os
from pathlib import Path
import subprocess
import tempfile

repo = Path(__file__).resolve().parent.parent
kit = Path(os.environ.get('TEKES_CLIENT_KIT_ROOT', str(repo.parent / 'TekesClientKit')))
contract_dir = kit / 'Sources/Contract/SessionEndpoint'
session_dir = kit / 'Sources/Session/SessionEndpoint'
native_dir = session_dir / 'Native'
# The Git service protocols depend on the TekesLocalGit package; neither
# participates in the five native SessionEndpoint synchronization streams.
excluded = {'SessionGitService.swift', 'WorkspaceServiceEndpoint.swift'}
sources = sorted(str(path) for path in contract_dir.glob('*.swift') if path.name not in excluded)
# NativeSessionEndpoint lives in the TekesClientSession target; besides the
# contract it needs only SessionHostActions, whose defaults suit a headless check.
native_sources = sorted(str(path) for path in native_dir.glob('NativeSessionEndpoint*.swift'))
host_actions = session_dir / 'SessionHostActions.swift'
for location, found in ((contract_dir, sources), (native_dir, native_sources),
                        (host_actions, [host_actions] if host_actions.is_file() else [])):
    if not found:
        raise SystemExit(f'TekesClientKit sources are unavailable: {location} has no Swift sources. '
                         'Check out TekesClientKit next to this repository or set TEKES_CLIENT_KIT_ROOT.')
native_sources.append(str(host_actions))
# The sources are compiled as one module. The module and package names match
# TekesClientKit, so `import TekesClientContract` in the native sources is a
# no-op and `package` declarations stay visible.
module = ['-module-name', 'TekesClientContract', '-package-name', 'TekesClientKit']
with tempfile.TemporaryDirectory(prefix='tekes-client-contract-') as directory:
    directory = Path(directory)
    verifier = directory / 'verify'
    subprocess.run(['swiftc', *module, *sources, str(repo / 'tools/client-contract-check/main.swift'),
                    '-o', str(verifier)], check=True)
    native = directory / 'native'
    subprocess.run(['swiftc', '-swift-version', '6', '-parse-as-library', *module, *sources,
                    *native_sources, str(repo / 'tools/native-client-check/main.swift'),
                    '-o', str(native)], check=True)
    frames = directory / 'frames.json'
    environment = dict(os.environ, TEKES_BUILTIN_CLIENT_FRAMES=str(frames), TEKES_NATIVE_CLIENT_EXEC=str(native))
    subprocess.run(['python3', str(repo / 'scripts/test-builtin-launch.py')],
                   env=environment, check=True)
    subprocess.run([str(verifier), str(frames)], check=True)

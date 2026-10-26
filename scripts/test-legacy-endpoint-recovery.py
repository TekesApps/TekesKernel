"""Real built-in process regression for legacy endpoint caches; no provider calls.

Run with a fixed --binary-dir and a new --output evidence directory. An optional
--expect-quarantine records the old-program negative control without claiming it
passed recovery. This script never reads the user's data root.
"""
import argparse
import base64
import hashlib
import json
from pathlib import Path
import secrets
import select
import socket
import struct
import subprocess
from urllib.parse import urlparse

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--binary-dir', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--expect-quarantine', action='store_true')
args = parser.parse_args()
repo = Path(__file__).resolve().parent.parent
case = args.output.resolve()
case.mkdir(mode=0o700)
root = case / 'state'
home = case / 'home'
home.mkdir(mode=0o700)
workspace = case / 'workspace'
workspace.mkdir(mode=0o700)
session = '018f0000-0000-7000-8000-000000000091'
folder = root / 'threads' / session
root.mkdir(mode=0o700)
(root / 'threads').mkdir(mode=0o700)
folder.mkdir(mode=0o700)
origin = dict(principal='legacy-regression', client='fixture', target=session,
              op='create', key='create')
events = [dict(config={'digest': 'fixture'}, format=1, kind='genesis', min_reader=1,
               min_writer=1, origin_key='create', origin_tuple=origin, resume='never',
               seq=1, thread=session, ts='2026-10-26T00:00:00.000Z', v=1, workspace='fixture'),
          dict(content=[{'type': 'text', 'text': 'LEGACY_RECOVERY_HISTORY'}], kind='input',
               origin_key='input', origin_tuple=dict(origin, op='submit', key='input'),
               seq=2, ts='2026-10-26T00:00:01.000Z', v=1)]
ledger = folder / 'main.jsonl'
original = ''.join(json.dumps(e, sort_keys=True, separators=(',', ':')) + '\n'
                   for e in events).encode()
ledger.write_bytes(original)
for name in ['endpoint-v2.jsonl', 'endpoint-requests-v2.jsonl']:
    (folder / name).write_bytes(b'')
providers = json.loads((repo / 'fixtures/config/providers.canonical.json').read_text())
launch = case / 'launch.json'
launch.write_text(json.dumps(dict(format=1, bootstrap_version=1, root=str(root),
    worker=str(args.binary_dir / 'tekes-worker'), listen='127.0.0.1:0',
    providers=providers, credential_bindings={'openai-main': 'TEKES_FIXTURE_KEY'})))
launch.chmod(0o600)
token = secrets.token_bytes(32)
material = json.dumps({'endpointToken': token.hex(),
                       'credentials': {'openai-main': 'synthetic-no-model-call'}}).encode()
bootstrap = b'TEKES-KERNEL-BOOTSTRAP/1\n' + struct.pack('!I', len(material)) + material
bearer = base64.urlsafe_b64encode(token).decode().rstrip('=')
env = {'HOME': str(home), 'PATH': '/usr/bin:/bin', 'LANG': 'en_US.UTF-8'}
identity = json.loads(subprocess.check_output([str(args.binary_dir / 'tekes-supervisor'), '--describe-build']))


def journal(ready):
    url = urlparse(ready['url'])
    with socket.create_connection((url.hostname, url.port), timeout=15) as sock:
        key = base64.b64encode(secrets.token_bytes(16)).decode()
        sock.sendall((f'GET /api/remote.mux HTTP/1.1\r\nHost: {url.netloc}\r\nUpgrade: websocket\r\n'
                      f'Connection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: {key}\r\n'
                      f'Authorization: Bearer {bearer}\r\n\r\n').encode())
        with sock.makefile('rb') as stream:
            assert b' 101 ' in stream.readline()
            while stream.readline() != b'\r\n':
                pass

            def receive():
                header = stream.read(2)
                assert len(header) == 2, 'mux closed before journal result'
                assert header[0] & 15 == 1, 'expected mux JSON text frame'
                length = header[1] & 127
                if length == 126:
                    length = struct.unpack('!H', stream.read(2))[0]
                elif length == 127:
                    length = struct.unpack('!Q', stream.read(8))[0]
                return json.loads(stream.read(length))

            assert receive()['type'] == 'ready'
            payload = json.dumps({'type': 'open', 'streamId': 'legacy-history', 'target': {
                'kind': 'session-journal', 'address': {'sessionId': session}, 'maxMessages': 10}}).encode()
            mask = secrets.token_bytes(4)
            header = bytes([129, 128 | len(payload)]) if len(payload) < 126 else bytes([129, 254]) + struct.pack('!H', len(payload))
            sock.sendall(header + mask + bytes(b ^ mask[i % 4] for i, b in enumerate(payload)))
            return receive()


phases = []
for phase in range(1 if args.expect_quarantine else 2):
    with (case / f'kernel-{phase}.private.log').open('wb') as log:
        process = subprocess.Popen([str(args.binary_dir / 'tekes-supervisor'), '--built-in', str(launch)],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=log, env=env)
        try:
            process.stdin.write(bootstrap)
            process.stdin.flush()
            assert select.select([process.stdout], [], [], 30)[0], 'readiness timeout'
            line = process.stdout.readline(65536)
            assert line, 'no readiness frame'
            ready = json.loads(line)
            frame = journal(ready)
            (case / f'public-frame-{phase}.json').write_text(json.dumps(frame, indent=2) + '\n')
            if args.expect_quarantine:
                assert 'quarantined' in json.dumps(frame), 'negative control did not reproduce quarantine'
                phases.append({'quarantineReproduced': True})
            else:
                snapshot = frame['frame']['snapshot']
                assert snapshot['address']['sessionId'] == session
                assert 'LEGACY_RECOVERY_HISTORY' in json.dumps(snapshot['entries'])
                assert ledger.read_bytes() == original, 'semantic history changed during recovery'
                phases.append({'publicHistoryReadable': True, 'entries': len(snapshot['entries']),
                               'semanticBytesUnchanged': True})
        finally:
            process.stdin.close()
            process.wait(timeout=20)
        assert process.returncode == 0
assert ledger.read_bytes() == original
result = {'build': identity, 'scope': 'isolated real public mux history and process restart; synthetic fixture; no model calls',
          'expectedOldFailure': args.expect_quarantine, 'phases': phases,
          'semanticSha256': hashlib.sha256(original).hexdigest(), 'modelCalls': 0,
          'programSha256': hashlib.sha256((args.binary_dir / 'tekes-supervisor').read_bytes()).hexdigest()}
(case / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result))

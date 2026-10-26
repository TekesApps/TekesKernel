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
import urllib.request
import urllib.error
from urllib.parse import urlparse

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--binary-dir', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--expect-quarantine', action='store_true')
parser.add_argument('--historical-read-only', action='store_true', help='exercise pre-catalog writer history and strict mutation rejection')
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
               seq=2, ts='2026-10-26T00:00:01.000Z', v=1),
          dict(kind='turn_open', seq=3, trigger={'inputs': [2]}, turn=1,
               ts='2026-10-26T00:00:02.000Z', v=1),
          dict(kind='settle', outcome='completed', seq=4, turn=1,
               ts='2026-10-26T00:00:03.000Z', v=1)]
if args.historical_read_only:
    events = [json.loads(line) for line in (repo / 'fixtures/invalid/_valid-baseline.jsonl').read_text().splitlines()]
    for event in events:
        if event['kind'] == 'genesis':
            event['thread'] = session
            event['origin_tuple']['target'] = session
        elif event['kind'] == 'input':
            event['content'] = [{'type': 'text', 'text': 'LEGACY_RECOVERY_HISTORY'}]
            event['origin_tuple']['target'] = session
        elif event['kind'] == 'run_start':
            event['binary'] = 'dev-cfa7a6be31c9-123456789abc'
        elif event['kind'] == 'epoch':
            del event['tools']
            event['tools_digest'] = 'sha256-' + 'a' * 64
        elif event['kind'] == 'attempt':
            event.pop('request', None)
        elif event['kind'] == 'output':
            event.pop('usage', None)
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
            def send(value):
                payload = json.dumps(value).encode()
                mask = secrets.token_bytes(4)
                header = bytes([129, 128 | len(payload)]) if len(payload) < 126 else bytes([129, 254]) + struct.pack('!H', len(payload))
                sock.sendall(header + mask + bytes(b ^ mask[i % 4] for i, b in enumerate(payload)))

            send({'type': 'open', 'streamId': 'legacy-history', 'target': {
                'kind': 'session-journal', 'address': {'sessionId': session},
                'maxMessages': 1 if args.historical_read_only else 10}})
            frame = receive()
            checks = {}
            if args.historical_read_only and not args.expect_quarantine:
                snapshot = frame['frame']['snapshot']
                access = snapshot['projections']['values']['historyAccess']
                assert access['mode'] == 'read-only'
                assert access['sourceDigest'] == 'sha256-' + hashlib.sha256(original).hexdigest()
                assert snapshot['hasMoreBefore']
                before = snapshot['entries'][0]['event']['seq']
                send({'type': 'journal-page', 'requestId': 'older-history',
                    'address': {'sessionId': session}, 'throughSequence': snapshot['throughSequence'],
                    'beforeSequence': before, 'maxMessages': 500})
                page_frame = receive()
                (case / f'public-page-{phase}.json').write_text(json.dumps(page_frame, indent=2) + '\n')
                assert page_frame['type'] == 'journal-page-result'
                assert 'LEGACY_RECOVERY_HISTORY' in json.dumps(page_frame['page']['entries'])
                checks = {'readOnlyMarker': True, 'publicPagination': True}
            return frame, checks


def reject_mutations(ready):
    results = []
    for method, payload in [('session.rename', {'sessionId': session, 'title': 'must-not-write'}),
                            ('session.cancel', {'sessionId': session}),
                            ('session.prompt', {'sessionId': session, 'mode': 'queue', 'content': [{'type': 'text', 'text': 'must-not-run'}]})]:
        rpc = secrets.token_hex(8)
        body = json.dumps({'type': 'client-request', 'rpcId': rpc, 'method': method, 'payload': payload}).encode()
        request = urllib.request.Request(ready['url'] + '/api/' + method, data=body,
            headers={'Authorization': 'Bearer ' + bearer, 'Content-Type': 'application/json'})
        try:
            with urllib.request.urlopen(request, timeout=10) as response: reply = json.load(response)
        except urllib.error.HTTPError as error:
            reply = json.load(error)
        assert reply['result']['ok'] is False, method + ' unexpectedly accepted'
        code = reply['result']['error']['code']
        assert code == 'internal', (method, code)
        assert reply['result']['error']['details']['reason'] == 'legacy-history-read-only', method
        results.append({'method': method, 'code': code})
    return results



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
            before_open_rejections = reject_mutations(ready) if args.historical_read_only and not args.expect_quarantine else None
            frame, checks = journal(ready)
            if before_open_rejections is not None:
                checks['beforeOpenMutationsRejected'] = before_open_rejections
            (case / f'public-frame-{phase}.json').write_text(json.dumps(frame, indent=2) + '\n')
            if args.expect_quarantine:
                assert 'quarantined' in json.dumps(frame), 'negative control did not reproduce quarantine'
                phases.append({'quarantineReproduced': True})
            else:
                snapshot = frame['frame']['snapshot']
                assert snapshot['sessionId'] == session
                if not args.historical_read_only:
                    assert 'LEGACY_RECOVERY_HISTORY' in json.dumps(snapshot['entries'])
                else:
                    checks['mutationsRejected'] = reject_mutations(ready)
                assert ledger.read_bytes() == original, 'semantic history changed during recovery'
                phases.append({'publicHistoryReadable': True, 'entries': len(snapshot['entries']),
                               'semanticBytesUnchanged': True, **checks})
        finally:
            process.stdin.close()
            process.wait(timeout=20)
        assert process.returncode == 0
assert ledger.read_bytes() == original
result = {'build': identity, 'scope': 'isolated real public mux history and process restart; synthetic fixture; no model calls',
          'expectedOldFailure': args.expect_quarantine, 'historicalReadOnly': args.historical_read_only, 'phases': phases,
          'semanticSha256': hashlib.sha256(original).hexdigest(), 'modelCalls': 0,
          'driverSha256': hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
          'programSha256': hashlib.sha256((args.binary_dir / 'tekes-supervisor').read_bytes()).hexdigest()}
(case / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result))

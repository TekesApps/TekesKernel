"""External v3 client for approvals and final delivery in disposable task workflows."""
import base64
import hashlib
import json
import os
from pathlib import Path
import socket
import struct
import sys
import time

root = Path(sys.argv[1])
deadline = time.monotonic() + 360
while not (root/'client-endpoint.json').exists():
    if time.monotonic() > deadline: raise TimeoutError('endpoint not published')
    time.sleep(.05)
endpoint = json.loads((root/'client-endpoint.json').read_text())
host, port = endpoint['address'].rsplit(':',1)
connection = socket.create_connection((host,int(port)),timeout=180)
key = base64.b64encode(os.urandom(16)).decode()
token = base64.urlsafe_b64encode(bytes([42])*32).decode().rstrip('=')
connection.sendall((f'GET /api/remote.mux HTTP/1.1\r\nHost: {host}:{port}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: {key}\r\nAuthorization: Bearer {token}\r\n\r\n').encode())
stream = connection.makefile('rb')
status = stream.readline().decode().strip()
assert ' 101 ' in status, status
headers = {}
while True:
    line = stream.readline()
    if line == b'\r\n': break
    name,value = line.decode().split(':',1); headers[name.lower()] = value.strip()
assert headers['sec-websocket-accept'] == base64.b64encode(hashlib.sha1((key+'258EAFA5-E914-47DA-95CA-C5AB0DC85B11').encode()).digest()).decode()
def send(payload, opcode=1):
    mask=os.urandom(4); length=len(payload)
    header=bytes([128|opcode,128|length]) if length<126 else bytes([128|opcode,254])+struct.pack('!H',length)
    connection.sendall(header+mask+bytes(byte^mask[i%4] for i,byte in enumerate(payload)))
def receive():
    header=stream.read(2)
    if len(header)!=2: raise EOFError('WebSocket closed before completion')
    assert header[0]&128, 'fragmented frame not supported by this fixture'
    opcode=header[0]&15; length=header[1]&127
    assert not header[1]&128, 'server must not mask frames'
    if length==126:length=struct.unpack('!H',stream.read(2))[0]
    elif length==127:length=struct.unpack('!Q',stream.read(8))[0]
    payload=stream.read(length)
    assert len(payload)==length
    if opcode==9:send(payload,10);return receive()
    if opcode==8:
        assert len(payload)>=2
        send(payload,8)
        return {'type':'websocket-close','code':struct.unpack('!H',payload[:2])[0],'reason':payload[2:].decode()}
    assert opcode==1, (opcode,payload)
    return json.loads(payload)
ready=receive(); assert ready['type']=='ready',ready
send(json.dumps({'type':'open','streamId':'task','target':{'kind':'session-journal','address':{'sessionId':endpoint['session']},'maxMessages':100}}).encode())
send(json.dumps({'type':'open','streamId':'approvals','target':{'kind':'actionables'}}).encode())
requests={}; responses={}; baseline=set(); final=None
workspace=(root.parent/'workspace').resolve()
with (root/'client-frames.jsonl').open('w') as log:
    while final is None or set(requests)!=set(responses):
        frame=receive();log.write(json.dumps(frame)+'\n');log.flush()
        assert frame.get('type') not in ('error','websocket-close'), frame
        inner=frame.get('frame',{})
        if inner.get('type') in ('journal-snapshot','actionable-baseline'):
            baseline.add(inner['type'])
            if len(baseline)==2: (root/'client-ready').write_text('subscribed')
        candidates=inner.get('items',[]) if inner.get('type')=='actionable-baseline' else ([inner['actionable']] if inner.get('type')=='actionable-upsert' else [])
        for item in candidates:
            if item['id'] in requests: continue
            assert item['sessionId']==endpoint['session'] and item['kind']=='approval',item
            payload=item['payload']; assert payload['toolName'] in ('task','apply_patch'),item
            # Read-only fixture boundary audit; all responses still travel over
            # the public WebSocket and no ledger is authored by this client.
            # Requests are derived from holds: find the ledger line whose
            # approval_request at the item's revision carries this rpc id.
            source_line=None
            for candidate in (root/'threads'/endpoint['session']).glob('*.jsonl'):
                if candidate.name.startswith('endpoint'): continue
                rows=[json.loads(line) for line in candidate.read_text().splitlines()]
                if any(row['kind']=='approval_request' and row['seq']==item['revision'] for row in rows):
                    name=candidate.name
                    preimage=(f"{endpoint['session']}\0approval/requested\0{item['revision']}".encode() if name=='main.jsonl'
                              else b''.join(part.encode()+b'\0' for part in ('child-request',endpoint['session'],name,'approval/requested',str(item['revision']))))
                    if 'request-'+hashlib.sha256(preimage).hexdigest()==item['id']: source_line=name; facts=rows
            assert source_line is not None,item
            request={'source_line':source_line}
            call=next(row for row in facts if row['kind']=='tool_call' and row['call']==payload['callId'])
            assert call['name']==payload['toolName']
            if call['name']=='apply_patch':
                path=Path(call['args']['path']);path=path if path.is_absolute() else workspace/path
                assert path.resolve().is_relative_to(workspace),path
            requests[item['id']]={'actionable':item,'source_line':request.get('source_line','main.jsonl')}
            send(json.dumps({'type':'actionable-respond','requestId':'answer-'+item['id'],'actionableId':item['id'],'expectedRevision':item['revision'],'outcome':{'approvalId':payload['approvalId'],'outcome':'allowed-once'}}).encode())
        if frame.get('type')=='actionable-response-result':
            assert frame['actionableId'] in requests,frame
            responses[frame['actionableId']]=frame
        if inner.get('type')=='journal-event' and inner['event']['type']=='turn/end':final=inner['event']
assert requests and any(r['source_line']!='main.jsonl' for r in requests.values()), 'no child approval exercised'
assert final['data']['reason']=='completed',final
(root/'client-task-receipt.json').write_text(json.dumps({'status':'passed','requests':requests,'responses':responses,'final':final},indent=2))
connection.close()

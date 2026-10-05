"""External v3 WebSocket observer for the disposable exhaustion workflow."""
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
final_step = int(sys.argv[2]) if len(sys.argv)>2 else 1
public_approval = len(sys.argv)>4 and sys.argv[4]=='1'
approval_requests = {}
approval_results = []
deadline = time.monotonic() + 120
while not (root/'client-endpoint.json').exists():
    if time.monotonic() > deadline: raise TimeoutError('endpoint not published')
    time.sleep(.05)
endpoint = json.loads((root/'client-endpoint.json').read_text())
host, port = endpoint['address'].rsplit(':',1)
connection = socket.create_connection((host,int(port)),timeout=30)
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
send(json.dumps({'type':'open','streamId':'exhaustion','target':{'kind':'session-journal','address':{'sessionId':endpoint['session']},'maxMessages':100}}).encode())
send(json.dumps({'type':'open','streamId':'inventory','target':{'kind':'session-inventory'}}).encode())
if public_approval:
    send(json.dumps({'type':'open','streamId':'approvals','target':{'kind':'actionables'}}).encode())
# Exercise concurrent production mutations while the receive loop remains active.
from concurrent.futures import ThreadPoolExecutor
import http.client
pool = ThreadPoolExecutor(max_workers=3)
def create_session(index):
    rpc_id = f'concurrent-create-{index}'
    request = {'type':'client-request','rpcId':rpc_id,'method':'session.create','payload':{'workspaceId':'ws'}}
    channel = http.client.HTTPConnection(host,int(port),timeout=30)
    try:
        channel.request('POST','/api/session.create',json.dumps(request),{'Authorization':'Bearer '+token,'Content-Type':'application/json'})
        response = channel.getresponse()
        body = json.loads(response.read())
        assert response.status==200 and body['rpcId']==rpc_id and body['result']['ok'], body
        return body
    finally:
        channel.close()
creates = []
frames=[]; ends=[]; texts=[]
runtime_futures=[]; runtime_ends=[]; runtime_ids=set(); runtime_messages={}
def prompt_created(sid):
    channel=http.client.HTTPConnection(host,int(port),timeout=30)
    rpc={"type":"client-request","rpcId":"prompt-"+sid,"method":"session.prompt","payload":{"sessionId":sid,"mode":"queue","content":[{"type":"text","text":"CONCURRENT_RUNTIME_INPUT:"+sid}]}}
    try:
        channel.request("POST","/api/session.prompt",json.dumps(rpc),{"Authorization":"Bearer "+token,"Content-Type":"application/json"})
        response=channel.getresponse(); body=json.loads(response.read())
        assert response.status==200 and body["result"]["ok"], body
        return body
    finally: channel.close()
with (root/'client-frames.jsonl').open('w') as log:
    while len(ends)<2 or len(runtime_ends)<3:
        frame=receive(); log.write(json.dumps(frame)+'\n'); log.flush(); frames.append(frame)
        inner=frame.get('frame',{})
        if public_approval:
            if frame.get('type')=='error': raise AssertionError(frame)
            candidates = inner.get('items',[]) if inner.get('type')=='actionable-baseline' else ([inner['actionable']] if inner.get('type')=='actionable-upsert' else [])
            for item in candidates:
                assert item['sessionId']==endpoint['session'] and item['kind']=='approval', item
                assert item['payload']['toolName']=='apply_patch' and item['payload']['callId']=='write-1', item
                if item['id'] not in approval_requests:
                    approval_requests[item['id']] = item
                    send(json.dumps({'type':'actionable-respond','requestId':'public-approval-'+item['id'],'actionableId':item['id'],'expectedRevision':item['revision'],'outcome':{'approvalId':item['payload']['approvalId'],'outcome':'allowed-once'}}).encode())
            if frame.get('type')=='actionable-response-result':
                assert frame['actionableId'] in approval_requests, frame
                approval_results.append(frame)
                (root/'client-approval-receipt.json').write_text(json.dumps({'requests':approval_requests,'responses':approval_results},indent=2))
        if creates and all(f.done() for f in creates) and not runtime_ids:
            runtime_ids={f.result()['result']['value']['sessionId'] for f in creates}
            for sid in runtime_ids:
                send(json.dumps({'type':'open','streamId':sid,'target':{'kind':'session-journal','address':{'sessionId':sid},'maxMessages':100}}).encode())
        if frame.get('streamId') in runtime_ids:
            if inner.get('type')=='journal-snapshot':
                runtime_futures.append(pool.submit(prompt_created,frame['streamId']))
            if inner.get('type')=='journal-event' and inner['event'].get('type')=='assistant/message':
                runtime_messages.setdefault(frame['streamId'],[]).append(inner['event'])
            if inner.get('type')=='journal-event' and inner['event'].get('type')=='turn/end':
                runtime_ends.append({'session':frame['streamId'],'event':inner['event']})
            continue

        if inner.get('type')=='inventory-baseline':
            assert not creates, 'duplicate baseline'
            creates = [pool.submit(create_session,index) for index in range(3)]
        if inner.get('type')=='journal-snapshot': (root/'client-ready').write_text('subscribed')
        if inner.get('type') in ['journal-event','journal-transient']:
            event = inner['event']
            if event.get('type')=='assistant/chunk' and event['data'].get('turn')==1 and event['data'].get('step')==final_step and event['data']['chunk'].get('text')=='KERNEL_VALIDATOR_OK':
                assert not ends, 'streamed chunk arrived after first turn completion'
                assert 'usagePreview' not in event['data'], event
                (root/'client-chunk-observed.json').write_text(json.dumps({'status':'passed','event':event,'before_turn_end':True}))
        if inner.get('type')=='journal-event':
            event=inner['event']
            if event.get('type')=='turn/end':ends.append(event)
            if event.get('type')=='assistant/message':texts.append(event)
if public_approval:
    assert len(approval_requests)==len(approval_results)==1, (approval_requests,approval_results)
assert (root/'client-chunk-observed.json').exists()
assert [event['data']['validationOutcome'] for event in ends]==['inconclusive','pass'],ends
if final_step in [2,4]:
    ordinary = [event for event in texts if 'STILL_WORKING_NOT_FINAL' in json.dumps(event)]
    assert len(ordinary)==1, ordinary
    assert ordinary[0]['data'].get('sessionFinal') is not True
    assert ordinary[0]['data']['turn']==ends[0]['data']['turn']
    assert ordinary[0]['seq'] < ends[0]['seq']

for end, expected in zip(ends, ['KERNEL_VALIDATOR_OK','SECOND_AFTER_EXHAUSTION']):
    promoted=end['data']['promotedMessageID']
    message=next(event for event in texts if event['data']['message']['id']==promoted)
    assert message['data']['message']['content'] == [{'type':'text','text':expected}]
    assert message['data']['sessionFinal'] is True
    assert message['seq']<end['seq']
if len(sys.argv)>3 and sys.argv[3]=='1':
    import http.client
    rpc = {'type':'client-request','rpcId':'cancel-after-final','method':'session.cancel','payload':{'sessionId':endpoint['session']}}
    receipts=[]
    for _ in range(2):
        rpc_connection = http.client.HTTPConnection(host,int(port),timeout=30)
        rpc_connection.request('POST','/api/session.cancel',json.dumps(rpc),{'Authorization':'Bearer '+token,'Content-Type':'application/json'})
        response=rpc_connection.getresponse()
        body=json.loads(response.read())
        assert response.status==200, body
        assert body['rpcId']==rpc['rpcId'] and body['result']=={'ok':True,'value':{'accepted':True}}, body
        receipts.append(body)
        rpc_connection.close()
    assert receipts[0]==receipts[1]
    (root/'client-cancel-receipt.json').write_text(json.dumps({'status':'passed','responses':receipts},indent=2))
created = [future.result(timeout=30) for future in creates]
runtime_receipts=[future.result(timeout=30) for future in runtime_futures]
assert len(runtime_receipts)==3 and len({row['session'] for row in runtime_ends})==3
for row in runtime_ends:
    sid=row['session']; end=row['event']
    message=next(event for event in runtime_messages[sid] if event['data']['message']['id']==end['data']['promotedMessageID'])
    assert message['data']['sessionFinal'] is True and message['seq']<end['seq']
    assert message['data']['message']['content']==[{'type':'text','text':'CONCURRENT_RUNTIME_ANSWER:'+sid}]

(root/'client-runtime-receipt.json').write_text(json.dumps({'status':'passed','responses':runtime_receipts,'ends':runtime_ends},indent=2))
pool.shutdown()
created_ids = {response['result']['value']['sessionId'] for response in created}
assert len(created_ids)==3
notifications = [frame['frame']['session'] for frame in frames if frame.get('frame',{}).get('type')=='inventory-upsert' and frame['frame']['session']['sessionId'] in created_ids]
assert {item['sessionId'] for item in notifications}==created_ids, notifications
assert len({item["sessionId"] for item in notifications})==3, notifications
assert len({item['cwd'] for item in notifications})==1

(root/'client-create-receipt.json').write_text(json.dumps({'status':'passed','responses':created,'inventory_notifications':notifications},indent=2))
(root/'client-receipt.json').write_text(json.dumps({'status':'passed','ends':ends,'frames':len(frames)},indent=2))
drain_frames=[]
while True:
    frame=receive(); drain_frames.append(frame)
    if frame.get('type')=='websocket-close':
        assert frame['code']==1013 and frame['reason']=='server-draining', frame
        break
assert any(frame.get('type')=='error' and frame.get('error',{}).get('code')=='server-draining' for frame in drain_frames), drain_frames
(root/'client-drain-receipt.json').write_text(json.dumps({'status':'passed','frames':drain_frames},indent=2))
connection.close()

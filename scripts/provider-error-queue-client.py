"""External v3 client for permanent provider error and queued recovery."""
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
realtime_mode = '--realtime' in sys.argv[2:]
question_mode = '--question' in sys.argv[2:]
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
send(json.dumps({'type':'open','streamId':'workflow','target':{'kind':'session-journal','address':{'sessionId':endpoint['session']},'maxMessages':100}}).encode())
if question_mode:
    send(json.dumps({'type':'open','streamId':'questions','target':{'kind':'actionables'}}).encode())
import http.client
from concurrent.futures import ThreadPoolExecutor

def submit_inputs():
    receipts=[]
    for index,text in enumerate(["input that hits a permanent 400","queued input after this turn's permanent failure"]):
        channel=http.client.HTTPConnection(host,int(port),timeout=30)
        request={'type':'client-request','rpcId':f'queue-input-{index}','method':'session.prompt','payload':{'sessionId':endpoint['session'],'mode':'queue','content':[{'type':'text','text':text}]}}
        try:
            channel.request('POST','/api/session.prompt',json.dumps(request),{'Authorization':'Bearer '+token,'Content-Type':'application/json'})
            response=channel.getresponse();body=json.loads(response.read())
            assert response.status==200 and body['result']['ok'],body
            receipts.append(body)
        finally:channel.close()
    return receipts
pool=ThreadPoolExecutor(max_workers=1)
submitted=None;events=[];ends=[];questions={};responses={}
with (root/'client-frames.jsonl').open('w') as log:
    while len(ends)<2 or set(questions)!=set(responses):
        frame=receive();log.write(json.dumps(frame)+'\n');log.flush()
        assert frame.get('type') not in ('error','websocket-close'),frame
        inner=frame.get('frame',{})
        candidates=inner.get('items',[]) if inner.get('type')=='actionable-baseline' else ([inner['actionable']] if inner.get('type')=='actionable-upsert' else [])
        for item in candidates:
            if item['id'] in questions: continue
            assert question_mode and item['sessionId']==endpoint['session'],item
            assert item['payload']['type']=='question/requested',item
            assert item['payload']['questions']==[{'question':'Which option?','options':['A','B']}],item
            questions[item['id']]=item
            send(json.dumps({'type':'actionable-respond','requestId':'answer-'+item['id'],'actionableId':item['id'],'expectedRevision':item['revision'],'outcome':{'answer':{'answers':[{'question':'Which option?','answer':'A'}]}}}).encode())
        if frame.get('type')=='actionable-response-result':
            assert frame['actionableId'] in questions,frame
            assert frame['requestId']=='answer-'+frame['actionableId'] and frame['revision']==questions[frame['actionableId']]['revision'],frame
            responses[frame['actionableId']]=frame
        if inner.get('type')=='journal-snapshot':
            assert submitted is None
            submitted=pool.submit(submit_inputs)
        if realtime_mode and inner.get('type')=='journal-transient':
            event=inner['event']
            if event['type']=='assistant/chunk' and event['data']['chunk'].get('text')=='recovered after provider failure':
                assert len(ends)==1, 'chunk must precede the recovered turn end'
                (root/'realtime-chunk-observed.json').write_text(json.dumps(frame))
        if inner.get('type')=='journal-event':
            events.append(inner['event'])
            if inner['event']['type']=='turn/end':ends.append(inner['event'])
receipts=submitted.result(timeout=30);pool.shutdown()
assert len(questions)==len(responses)==int(question_mode),(questions,responses)
assert [e['data']['reason'] for e in ends]==['error','completed'],ends
errors=[e for e in events if e['type']=='response/error']
assert len(errors)==1 and errors[0]['data']['turn']==1,errors
assert errors[0]['data']['classification']=='provider_terminal',errors
assert errors[0]['data']['recoverable'] is False and errors[0]['data']['message'],errors
starts=[e for e in events if e['type']=='turn/start']
assert [e['data']['turn'] for e in starts]==[1,2],starts
assert errors[0]['seq']<ends[0]['seq']<starts[1]['seq']<ends[1]['seq']
answer=next(e for e in events if e['type']=='assistant/message' and e['data']['message']['id']==ends[1]['data']['promotedMessageID'])
assert answer['data']['message']['content']==[{'type':'text','text':'recovered after provider failure'}]
assert answer['data']['sessionFinal'] is True
(root/'client-error-queue-receipt.json').write_text(json.dumps({'status':'passed','receipts':receipts,'starts':starts,'errors':errors,'ends':ends,'answer':answer,'questions':questions,'question_responses':responses},indent=2))
connection.close()

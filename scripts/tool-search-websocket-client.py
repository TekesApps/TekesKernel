"""External v3 client for two-turn deferred tool discovery and execution."""
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
import atexit
def record_failure():
    if root.exists() and not (root/'client-tool-search-receipt.json').exists():
        (root/'client-tool-search-failed').write_text('See client.log for the failed assertion')
atexit.register(record_failure)
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
import http.client
from concurrent.futures import ThreadPoolExecutor
prompts = ["Find the hidden function that emits a UAT marker using tool_search, call it with\nFIRST_HOT_OK, then finish with exactly FIRST_HOT_OK.", "Call the currently available mcp__local__uat_marker function with SECOND_HOT_OK, then finish with exactly SECOND_HOT_OK."]
# Some routes' content classifiers refuse the original "hidden function" wording
# outright; TEKES_TOOL_SEARCH_PROMPT_STYLE=plain keeps the same two-turn
# search-then-call contract with neutral phrasing. The default is unchanged.
if os.environ.get('TEKES_TOOL_SEARCH_PROMPT_STYLE') == 'plain':
    prompts = ["Use tool_search to look up the deferred function that records a UAT marker, then call that function with the marker FIRST_HOT_OK, then reply with exactly FIRST_HOT_OK.", "Use tool_search again to look up the deferred function that records a UAT marker, call it with the marker SECOND_HOT_OK, then reply with exactly SECOND_HOT_OK."]
finals = ['FIRST_HOT_OK', 'SECOND_HOT_OK']
# TEKES_TOOL_SEARCH_SCENARIO=shipping: same contract with a shipping-ETA helper.
if os.environ.get('TEKES_TOOL_SEARCH_SCENARIO') == 'shipping':
    prompts = ["Use tool_search to find the deferred function that returns a shipping ETA for an order, call it for order LIVE-42, then reply with exactly the eta value it returns and nothing else.", "Use tool_search again to find the deferred shipping ETA function, call it for order LIVE-43, then reply with exactly the eta value it returns and nothing else."]
    finals = ['ETA-3D', 'ETA-5D']
def submit(index):
    channel=http.client.HTTPConnection(host,int(port),timeout=30)
    request={'type':'client-request','rpcId':f'hot-input-{index}','method':'session.prompt','payload':{'sessionId':endpoint['session'],'mode':'queue','content':[{'type':'text','text':prompts[index]}]}}
    try:
        channel.request('POST','/api/session.prompt',json.dumps(request),{'Authorization':'Bearer '+token,'Content-Type':'application/json'})
        response=channel.getresponse();body=json.loads(response.read())
        assert response.status==200 and body['result']['ok'],body
        return body
    finally:channel.close()
pool=ThreadPoolExecutor(max_workers=1);submitted=[];events=[];ends=[]
with (root/'client-frames.jsonl').open('w') as log:
    while len(ends)<2:
        frame=receive();log.write(json.dumps(frame)+'\n');log.flush()
        assert frame.get('type') not in ('error','websocket-close'),frame
        inner=frame.get('frame',{})
        if inner.get('type')=='journal-snapshot':
            assert not submitted
            submitted.append(pool.submit(submit,0))
        if inner.get('type')=='journal-event':
            event=inner['event'];events.append(event)
            assert event['type']!='response/error',event
            if event['type']=='turn/end':
                assert event['data']['reason']=='completed',event
                ends.append(event)
                answer=next(e for e in events if e['type']=='assistant/message' and e['data']['message']['id']==event['data']['promotedMessageID'])
                marker=finals[len(ends)-1]
                assert answer['data']['message']['content']==[{'type':'text','text':marker}],answer
                if len(ends)==1:submitted.append(pool.submit(submit,1))
receipts=[f.result(timeout=30) for f in submitted];pool.shutdown()
(root/'client-tool-search-receipt.json').write_text(json.dumps({'status':'passed','receipts':receipts,'ends':ends},indent=2))
connection.close()

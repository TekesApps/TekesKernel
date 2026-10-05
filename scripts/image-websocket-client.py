"""External v3 client for real image attachment intake and vision completion."""
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
    if root.exists() and not (root/'client-image-receipt.json').exists():
        (root/'client-image-failed').write_text('See client.log for the failed assertion')
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
IMAGE_BASE64 = "iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAIAAAAlC+aJAAAAS0lEQVR42u3PQQkAAAgAsetfWiP4FgYrsKZeS0BAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEDgsqnc8OJg6Ln3AAAAAElFTkSuQmCC"
prompts = ["An image is attached to this message. Reply with exactly one lowercase English word: the dominant color of the image."]
def submit(index):
    channel=http.client.HTTPConnection(host,int(port),timeout=30)
    request={'type':'client-request','rpcId':f'hot-input-{index}','method':'session.prompt','payload':{'sessionId':endpoint['session'],'mode':'queue','content':[{'type':'text','text':prompts[index]},{'type':'image','mediaType':'image/png','data':IMAGE_BASE64}]}}
    try:
        channel.request('POST','/api/session.prompt',json.dumps(request),{'Authorization':'Bearer '+token,'Content-Type':'application/json'})
        response=channel.getresponse();body=json.loads(response.read())
        assert response.status==200 and body['result']['ok'],body
        return body
    finally:channel.close()
pool=ThreadPoolExecutor(max_workers=1);submitted=[];events=[];ends=[]
with (root/'client-frames.jsonl').open('w') as log:
    while len(ends)<1:
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
                text=''.join(part.get('text','') for part in answer['data']['message']['content'])
                assert text.strip().lower()=='red',answer
receipts=[f.result(timeout=30) for f in submitted];pool.shutdown()
(root/'client-image-receipt.json').write_text(json.dumps({'status':'passed','receipts':receipts,'ends':ends},indent=2))
connection.close()

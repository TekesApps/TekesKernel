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
def report_failure(kind,error,traceback):
    if not (root/'client-flow-failure.json').exists():(root/'client-flow-failure.json').write_text(json.dumps({'error':str(error)}))
    sys.__excepthook__(kind,error,traceback)
sys.excepthook=report_failure

realtime_mode = '--realtime' in sys.argv[2:]
question_mode = '--question' in sys.argv[2:]
deadline = time.monotonic() + 360
while not (root/'client-endpoint.json').exists():
    if time.monotonic() > deadline: raise TimeoutError('endpoint not published')
    time.sleep(.05)
endpoint = json.loads((root/'client-endpoint.json').read_text())
host, port = endpoint['address'].rsplit(':',1)
connection = socket.create_connection((host,int(port)),timeout=int(json.loads(Path(sys.argv[2]).read_text()).get('receive_timeout_seconds',180)))
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
case=json.loads(Path(sys.argv[2]).read_text())
ready=receive();assert ready['type']=='ready'
send(json.dumps({'type':'open','streamId':'workflow','target':{'kind':'session-journal','address':{'sessionId':endpoint['session']},'maxMessages':100}}).encode())
send(json.dumps({'type':'open','streamId':'approvals','target':{'kind':'actionables'}}).encode())
import http.client
import urllib.parse
def rpc_probe(method, payload, key):
    channel=http.client.HTTPConnection(host,int(port),timeout=60)
    # Extension methods carry a slash (commands/run); the carrier's unary path is
    # one segment, so the method name is percent-encoded and decoded by the router.
    channel.request('POST','/api/'+urllib.parse.quote(method,safe=''),json.dumps({'type':'client-request','rpcId':key,'method':method,'payload':payload}),{'Authorization':'Bearer '+token,'Content-Type':'application/json'})
    response=channel.getresponse();body=json.loads(response.read());channel.close()
    assert response.status==200 and body['result']['ok'],(method,body)
    return body['result']['value']
def run_route_probes(model_id):
    # Legacy endpoint-v1 spawned-CLI contract, ported to the v2 carrier: the
    # public routes are callable on the spawned host before any turn runs.
    # v3 carrier registry (session-endpoint): session.models replaces the
    # legacy modelProfiles read, session.selectModel the profile save, and
    # the journal snapshot received before any turn is the empty running tail.
    models=rpc_probe('session.models',{'sessionId':endpoint['session']},'probe-models')
    assert model_id and model_id in json.dumps(models),models
    selected=rpc_probe('session.selectModel',{'sessionId':endpoint['session'],'provider':case['provider_id'],'model':model_id},'probe-select')
    return {'models_include_configured':True,'select_model':selected,'snapshot_before_turn':snapshot_summary}
from concurrent.futures import ThreadPoolExecutor
pool=ThreadPoolExecutor(max_workers=1)
def submit_checked(function, *arguments):
    future=pool.submit(function,*arguments)
    def check(done):
        error=done.exception()
        if error is not None:
            if not (root/'client-flow-failure.json').exists():(root/'client-flow-failure.json').write_text(json.dumps({'error':str(error),'phase':'prompt-admission'}))
    future.add_done_callback(check)
    return future

def submit_text(text,rpc_id='flow-input'):
    channel=http.client.HTTPConnection(host,int(port),timeout=30)
    payload={'type':'client-request','rpcId':rpc_id,'method':'session.prompt','payload':{'sessionId':endpoint['session'],'mode':'queue','content':[{'type':'text','text':text}]}}
    channel.request('POST','/api/session.prompt',json.dumps(payload),{'Authorization':'Bearer '+token,'Content-Type':'application/json'})
    response=channel.getresponse();body=json.loads(response.read());channel.close()
    assert response.status==200 and body['result']['ok'],body
    return body
def submit():
    channel=http.client.HTTPConnection(host,int(port),timeout=30)
    payload={'type':'client-request','rpcId':'flow-input','method':'session.prompt','payload':{'sessionId':endpoint['session'],'mode':'queue','content':[{'type':'text','text':case['prompt']}]}}
    channel.request('POST','/api/session.prompt',json.dumps(payload),{'Authorization':'Bearer '+token,'Content-Type':'application/json'})
    response=channel.getresponse();body=json.loads(response.read());channel.close()
    assert response.status==200 and body['result']['ok'],body
    return body
route_probes=None;snapshot_summary=None
if case['id'] in ('cache-corpus','agent-eval'):
    def rpc(method, payload, key):
        channel=http.client.HTTPConnection(host,int(port),timeout=60)
        channel.request('POST','/api/'+method,json.dumps({'type':'client-request','rpcId':key,'method':method,'payload':payload}),{'Authorization':'Bearer '+token,'Content-Type':'application/json'})
        response=channel.getresponse();body=json.loads(response.read());channel.close()
        assert response.status==200 and body['result']['ok'],body
        return body
    def submit_turn(index):
        return rpc('session.prompt',{'sessionId':endpoint['session'],'mode':'queue','content':[{'type':'text','text':case['prompts'][index]}]},'corpus-'+str(index))
    events=[];approvals=set();turn_index=0;submitted=None;completed=[]
    with (root/'client-frames.jsonl').open('w') as log:
        while True:
            frame=receive();log.write(json.dumps(frame)+'\n');log.flush()
            assert frame.get('type') not in ('error','websocket-close'),frame
            inner=frame.get('frame',{})
            if inner.get('type') in ('journal-snapshot','snapshot') and submitted is None:
                submitted=submit_checked(submit_turn,turn_index)
            candidates=inner.get('items',[]) if inner.get('type')=='actionable-baseline' else ([inner['actionable']] if inner.get('type')=='actionable-upsert' else [])
            for item in candidates:
                if item['id'] in approvals:continue
                approvals.add(item['id'])
                send(json.dumps({'type':'actionable-respond','requestId':'approve-'+item['id'],'actionableId':item['id'],'expectedRevision':item['revision'],'outcome':{'approvalId':item['payload']['approvalId'],'outcome':'allowed-once'}}).encode())
            if inner.get('type') not in ('journal-event','event'):continue
            event=inner['event'] if inner['type']=='journal-event' else inner['entry']['event']
            events.append(event)
            assert event['type']!='response/error' or event['data'].get('recoverable') is True,event
            if event['type']=='turn/end':
                assert event['data']['reason']=='completed',event
                submitted.result(timeout=60)
                completed.append(event);turn_index+=1
                if turn_index==len(case['prompts']):break
                submitted=submit_checked(submit_turn,turn_index)
    seqs=[event['seq'] for event in events];assert seqs==sorted(set(seqs))
    assert len([event for event in events if event['type']=='turn/start'])==len(case['prompts'])
    for event in completed:
        assert any(row['type']=='assistant/message' and row['data']['message']['id']==event['data']['promotedMessageID'] for row in events),event
    for method in (['usage.summary','usage.cacheAttribution'] if case['id']=='cache-corpus' else ['usage.summary']):
        report=rpc(method,{'sessionId':endpoint['session']},'corpus-'+method)
        (root/(method+'.json')).write_text(json.dumps(report,indent=2))
        if method=='usage.cacheAttribution':assert report['result']['value']['entries'], 'no attributable cache rows'
    (root/'client-flow-receipt.json').write_text(json.dumps({'status':'passed','case':case['id'],'route_probes':route_probes,'scenario':case['scenario'],'completed_turns':len(completed),'events':events},indent=2))
    pool.shutdown();connection.close();sys.exit(0)

submitted=None;queued=None;compact_run=None;events=[];approvals=set()
with (root/'client-frames.jsonl').open('w') as log:
    while True:
        frame=receive();log.write(json.dumps(frame)+'\n');log.flush()
        assert frame.get('type') not in ('error','websocket-close'),frame
        inner=frame.get('frame',{})
        if inner.get('type') in ('journal-snapshot','snapshot'):
            snapshot_events=inner.get('events',inner.get('snapshot',{}).get('entries',[]))
            snapshot_summary={'events':len(snapshot_events),'assistant_messages':sum(1 for e in snapshot_events if e.get('type')=='assistant/message')}
            assert snapshot_summary['assistant_messages']==0,snapshot_summary
            route_probes=run_route_probes(case.get('model_id','')) if case.get('route_probes',True) else None
            if case.get('goal'):
                goal=rpc_probe('goals.edit',{'sessionId':endpoint['session'],'ref':{'id':'new','revision':0},'objective':'Complete the two-turn public flow. Finish the first turn with ROUND_ONE while remaining active. In the next turn mark the goal complete with set_goal_state and finish with '+case['token']+'.'},'flow-goal-create')
                assert goal['phase']=='active',goal
            submitted=submit_checked(submit)
        candidates=inner.get('items',[]) if inner.get('type')=='actionable-baseline' else ([inner['actionable']] if inner.get('type') in ('actionable-upsert','upsert') else [])
        for item in candidates:
            if item['id'] in approvals:continue
            approvals.add(item['id'])
            send(json.dumps({'type':'actionable-respond','requestId':'approve-'+item['id'],'actionableId':item['id'],'expectedRevision':item['revision'],'outcome':{'approvalId':item['payload']['approvalId'],'outcome':'allowed-once'}}).encode())
        if inner.get('type') in ('journal-event','event'):
            event=inner['event'] if inner['type']=='journal-event' else inner['entry']['event'];events.append(event)
            # A durable provider-admission wait surfaces its recoverable error
            # publicly (the legacy ordinary response error); only a terminal
            # error fails the flow.
            assert event['type']!='response/error' or (case.get('wholesale') and event['data'].get('recoverable') is True),event
            if case.get('compaction') and event['type']=='turn/end':
                # Compaction: the seeded turns settle one by one, the
                # named compact command compacts the settled history (supervisor
                # authored: no worker is alive), then the recall turn runs on the
                # compaction generation.
                ends=[e for e in events if e['type']=='turn/end']
                assert event['data']['reason']=='completed',event
                if queued is not None:queued.result(timeout=30)
                seeds=case['seed_prompts']
                if len(ends)<len(seeds):
                    queued=submit_checked(submit_text,seeds[len(ends)],'flow-seed-%d'%len(ends))
                elif len(ends)==len(seeds):
                    compact_run=rpc_probe('commands/run',{'session_id':endpoint['session'],'name':'compact','arguments':'','key':'flow-compact-1'},'flow-compact')
                    assert compact_run.get('accepted') is True and compact_run.get('command')=='compact',compact_run
                    queued=submit_checked(submit_text,case['post_prompt'],'flow-recall')
                else:
                    break
                continue
            if case.get('followup_prompt') and event['type']=='turn/end' and queued is None and not case.get('skill_compact'):
                # A second ordinary turn submitted only after the first settled.
                queued=submit_checked(submit_text,case['followup_prompt'],'flow-followup')
            if case.get('skill_compact') and event['type']=='turn/end' and compact_run is None:
                # Legacy: after the PRE-COMPACT turn settles, the named `compact`
                # command is the manual compaction request; the POST-COMPACT turn
                # then runs on the compaction generation with a fresh skill chain.
                compact_run=rpc_probe('commands/run',{'session_id':endpoint['session'],'name':'compact','arguments':'','key':'flow-compact-1'},'flow-compact')
                assert compact_run.get('accepted') is True and compact_run.get('command')=='compact',compact_run
                compact_again=rpc_probe('commands/run',{'session_id':endpoint['session'],'name':'compact','arguments':'','key':'flow-compact-1'},'flow-compact-retry')
                assert compact_again.get('seq')==compact_run.get('seq') and compact_again.get('deduplicated') is True,compact_again
                queued=submit_checked(submit_text,case['post_prompt'],'flow-post')
            if case.get('queued_prompt') and queued is None and event['type'] in ('tool/call','assistant/chunk','step/start'):
                # Legacy queued-root-input gate: a second ordinary input submitted
                # while the first turn is still running (its tool call is in flight)
                # must wait for that turn to settle, validator included.
                queued=submit_checked(submit_text,case['queued_prompt'],'flow-queued')
            if event['type']=='turn/end':
                ends=[e for e in events if e['type']=='turn/end']
                if case.get('goal') and len(ends)<2:continue
                if not (case.get('queued_prompt') or case.get('skill_compact') or case.get('followup_prompt')) or len(ends)==2:break
submitted.result(timeout=30)
if case.get('goal'):
    goal=rpc_probe('goals.get',{'sessionId':endpoint['session']},'flow-goal-check')['goal']
    assert goal['phase']=='complete',goal
    assert len([e for e in events if e['type']=='turn/end'])==2,events
    assert len([e for e in events if e['type']=='user/message'])==1,events
usage_reports=None
if case.get('usage_rpcs'):
    # Legacy cache-attribution assertion: the public usage routes answer for
    # the settled session and attribution has rows.
    usage_reports={method:rpc_probe(method,{'sessionId':endpoint['session']},'usage-'+method) for method in ('usage.summary','usage.cacheAttribution')}
skill_compact=None
if case.get('skill_compact'):
    assert compact_run is not None,'the compact command never ran'
    queued.result(timeout=30)
    starts=[e for e in events if e['type']=='turn/start'];ends=[e for e in events if e['type']=='turn/end']
    assert len(starts)==2 and len(ends)==2 and all(e['data']['reason']=='completed' for e in ends),(starts,ends)
    def final_text(end):
        message=next(e for e in events if e['type']=='assistant/message' and e['data']['message']['id']==end['data']['promotedMessageID'])
        return ''.join(block.get('text','') for block in message['data']['message']['content']).strip()
    finals=[final_text(end) for end in ends]
    assert finals==[case['pre_token'],case['token']],finals
    def calls(turn):
        return [(e['data']['name'],e['data']['callId']) for e in events if e['type']=='tool/call' and e['data'].get('turn')==turn]
    pre_calls,post_calls=calls(1),calls(2)
    for chain in (pre_calls,post_calls):
        names=[name for name,_ in chain]
        assert 'skill_explorer' in names and 'skill' in names,chain
    assert not ({cid for _,cid in pre_calls} & {cid for _,cid in post_calls}),'post-compact turn reused pre-compact call ids'
    skill_compact={'compact':compact_run,'finals':finals,'pre_calls':pre_calls,'post_calls':post_calls,'first_turn_end_seq':ends[0]['seq'],'second_turn_start_seq':starts[1]['seq']}
compaction=None
if case.get('compaction'):
    assert compact_run is not None,'the compact command never ran'
    queued.result(timeout=30)
    ends=[e for e in events if e['type']=='turn/end']
    assert len(ends)==len(case['seed_prompts'])+1 and all(e['data']['reason']=='completed' for e in ends),ends
    recall=next(e for e in events if e['type']=='assistant/message' and e['data']['message']['id']==ends[-1]['data']['promotedMessageID'])
    answer=''.join(block.get('text','') for block in recall['data']['message']['content'])
    recalled=[code for code in case['codes'] if code in answer]
    last_seed_end=ends[len(case['seed_prompts'])-1]['seq']
    compaction={'compact':compact_run,'codes':case['codes'],'recalled':recalled,'recall_answer':answer,'last_seed_end_seq':last_seed_end,'recall_start_seq':next(e for e in events if e['type']=='turn/start' and e['seq']>last_seed_end)['seq']}
if case.get('followup_prompt') and not case.get('skill_compact'):
    queued.result(timeout=30)
    ends=[e for e in events if e['type']=='turn/end']
    assert len(ends)==2 and all(e['data']['reason']=='completed' for e in ends),ends
if case.get('queued_prompt'):
    assert queued is not None,'the queued prompt was never submitted (no in-flight frame observed)'
    queued.result(timeout=30)
pool.shutdown()
assert events[-1]['data']['reason']=='completed',events[-1]
seqs=[e['seq'] for e in events];assert seqs==sorted(set(seqs)),seqs
start=next(i for i,e in enumerate(events) if e['type']=='turn/start')
queue_release=None
if case.get('queued_prompt'):
    starts=[e for e in events if e['type']=='turn/start'];ends=[e for e in events if e['type']=='turn/end']
    assert len(starts)==2 and len(ends)==2,(len(starts),len(ends))
    assert starts[1]['seq']>ends[0]['seq'],'queued input started before the running turn settled'
    assert all(e['data']['reason']=='completed' for e in ends),ends
    users=[e for e in events if e['type']=='user/message']
    assert len(users)==2 and users[1]['seq']>ends[0]['seq'],'queued user message surfaced before the first turn ended'
    second=next(e for e in events if e['type']=='assistant/message' and e['data']['message']['id']==ends[1]['data']['promotedMessageID'])
    assert case['token2'] in json.dumps(second['data']['message']['content']),second
    queue_release={'first_turn_end_seq':ends[0]['seq'],'second_turn_start_seq':starts[1]['seq'],'second_user_message_seq':users[1]['seq'],'second_answer_seq':second['seq']}
    events=[e for e in events if e['seq']<=ends[0]['seq']]
answer=next(i for i,e in enumerate(events) if e['type']=='assistant/message' and e['data']['message']['id']==events[-1]['data']['promotedMessageID'])
if case.get('free_final'):
    # SWE-bench drive: the answer text is free; the prediction is the workspace diff.
    assert events[answer]['data']['message']['content'],events[answer]
elif case.get('memory'):
    # The legacy memory smoke never pinned the answer text; the token is the
    # expected single lowercase word and is matched case-insensitively.
    assert case['token'] in json.dumps(events[answer]['data']['message']['content']).lower(),events[answer]
else:
    assert events[answer]['data']['message']['content']==[{'type':'text','text':case['token']}]
user=next(i for i,e in enumerate(events) if e['type']=='user/message')
assert any(block.get('type')=='text' and block.get('text')==case['prompt'] for block in events[user]['data']['content'])
assert start<user<answer<len(events)-1
if case['call']:
    name=case['call']['name']
    calls=[(i,e) for i,e in enumerate(events) if e['type']=='tool/call' and e['data']['name']==name]
    assert calls,(name,events)
    successes=[]
    for index,call in calls:
        for result_index,result in enumerate(events):
            if result['type']=='tool/result' and result['data']['message']['source']['callId']==call['data']['callId'] and result['data']['error'] is False:
                successes.append((index,result_index,result))
    assert successes, ('no successful tool execution',calls,events)
    index,result_index,result=successes[-1]
    assert start<index<result_index<answer
    if case['id']=='shell':
        payload=json.loads(result['data']['message']['content'][0]['text'])
        assert payload['status']=='completed' and payload['is_error'] is False,payload
        assert payload['steps'] and all(step['exit_code']==0 for step in payload['steps']),payload
        assert any(step['stdout']['encoding']=='utf8' and Path(step['stdout']['data'].strip()).resolve()==(root.parent/'workspace').resolve() for step in payload['steps']),payload
    if not case.get('live') and case['id'] in ('read','glob','grep'):
        marker='FLOW_SEED_MARKER' if case['id']=='read' else 'seed.md'
        assert marker in json.dumps(result),result
(root/'client-flow-receipt.json').write_text(json.dumps({'status':'passed','case':case['id'],'route_probes':route_probes,'queue_release':queue_release,'skill_compact':skill_compact,'compaction':compaction,'usage':usage_reports,'events':events,'approvals':list(approvals)},indent=2))
connection.close()

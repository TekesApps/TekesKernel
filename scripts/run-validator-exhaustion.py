#!/usr/bin/env python3
"""Exercise validator exhaustion and queued input with real processes and a local scripted provider."""
import argparse
import json
import hashlib
import shutil
import os
from pathlib import Path
import re
import subprocess

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output',type=Path,required=True)
parser.add_argument('--reported-usage',action='store_true')
parser.add_argument('--public-approval',action='store_true')
parser.add_argument('--commentary-before-final',action='store_true')
parser.add_argument('--cancel-after-final',action='store_true')
parser.add_argument('--skill-before-write',action='store_true')
parser.add_argument('--stale-skill-next-turn',action='store_true')
parser.add_argument('--compact-next-turn',action='store_true')
parser.add_argument('--reload-skill-next-turn',action='store_true')
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
args.output.mkdir(parents=True,exist_ok=False)
script_snapshot = args.output.resolve()/'scripts'
script_snapshot.mkdir()
for name in ['run-validator-exhaustion.py','exhaustion-websocket-client.py']:
    shutil.copy2(root/'scripts'/name,script_snapshot/name)
(args.output/'scripts.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in script_snapshot.iterdir()},indent=2))

from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import threading
import time
key = 'disposable-scripted-provider-key'
secrets = {}
requests = []
overlap_barrier = threading.Barrier(3)
overlap_lock = threading.Lock()
overlap_events = []
class Handler(BaseHTTPRequestHandler):
    def log_message(self, *args): pass
    def do_POST(self):
        request = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
        if 'CONCURRENT_RUNTIME_INPUT' in json.dumps(request):
            matches = set(re.findall(r'CONCURRENT_RUNTIME_INPUT:([0-9a-f-]{36})',json.dumps(request)))
            assert len(matches)==1, 'missing or ambiguous concurrent session marker'
            concurrent_id = matches.pop()
            with overlap_lock:
                request_index = len(overlap_events) + 1
                overlap_events.append({'request':request_index,'arrived':time.monotonic_ns()})
            overlap_barrier.wait(timeout=20)
            with overlap_lock:
                overlap_events[request_index-1]['released'] = time.monotonic_ns()
                (args.output/'concurrent-overlap.json').write_text(json.dumps(overlap_events,indent=2))

            response = {'id':'concurrent-response-'+concurrent_id,'status':'completed','output':[{'type':'message','role':'assistant','phase':'final_answer','status':'completed','content':[{'type':'output_text','text':'CONCURRENT_RUNTIME_ANSWER:'+concurrent_id}]}]}
            body = ('data: '+json.dumps({'type':'response.completed','response':response})+'\n\n').encode()
            self.send_response(200); self.send_header('Content-Type','text/event-stream'); self.send_header('Content-Length',str(len(body))); self.end_headers(); self.wfile.write(body)
            return
        requests.append(request)
        n = len(requests)
        response_number = n
        (args.output/f'request-{n}.json').write_text(json.dumps(request,indent=2))
        skill_request = n if args.skill_before_write and n <= 2 else None
        if args.skill_before_write: n -= 2
        if args.commentary_before_final and n >= 2:
            n = 0 if n == 2 else n - 1
        if args.compact_next_turn and n == 7:
            (args.output/'overflow-request-index.json').write_text(json.dumps({'request':response_number}))
            body = json.dumps({'error':{'code':'context_length_exceeded','type':'invalid_request_error','message':'fixture requires compaction'}}).encode()
            self.send_response(400); self.send_header('Content-Type','application/json'); self.send_header('Content-Length',str(len(body))); self.end_headers(); self.wfile.write(body)
            return
        if args.compact_next_turn and n >= 8: n -= 1
        stale_skill = args.stale_skill_next_turn and n == 7
        if args.stale_skill_next_turn and n >= 8: n -= 1
        reload_skill = n - 6 if args.reload_skill_next_turn and not stale_skill and n in [7,8] else None
        if args.reload_skill_next_turn and n >= 9: n -= 2
        if reload_skill:
            name = 'skill_explorer' if reload_skill == 1 else 'skill'
            arguments = {'query':'Live proof','limit':5} if reload_skill == 1 else {'skill':'live-proof.md'}
            output = [{'type':'function_call','status':'completed','call_id':f'reload-{reload_skill}','name':name,'arguments':json.dumps(arguments)}]
        elif stale_skill:
            output = [{'type':'function_call','status':'completed','call_id':'stale-skill','name':'skill','arguments':json.dumps({'skill':'live-proof.md'})}]
        elif skill_request:
            name = 'skill_explorer' if skill_request == 1 else 'skill'
            arguments = {'query':'Live proof','limit':5} if skill_request == 1 else {'skill':'live-proof.md'}
            output = [{'type':'function_call','status':'completed','call_id':f'skill-{skill_request}','name':name,'arguments':json.dumps(arguments)}]
        elif n == 0:
            output = [{'type':'message','role':'assistant','phase':'commentary','status':'completed','content':[{'type':'output_text','text':'STILL_WORKING_NOT_FINAL'}]}]
        elif n == 1:
            output = [{'type':'function_call','status':'completed','call_id':'write-1','name':'apply_patch','arguments':json.dumps({'path':str(args.output.resolve()/'workspace/proof.txt'),'operation':'create_file','diff':'+KERNEL_VALIDATOR_OK','summary':'validator exhaustion fixture','expected_artifact_version':0})}]
        elif n == 2:
            output = [{'type':'message','role':'assistant','phase':'final_answer','status':'completed','content':[{'type':'output_text','text':'KERNEL_VALIDATOR_OK'}]}]
        elif n <= 6:
            time.sleep(.15)
            output = [{'type':'message','role':'assistant','phase':'final_answer','status':'completed','content':[{'type':'output_text','text':f'validator refusal {n}'}]}]
        elif n == 7:
            output = [{'type':'message','role':'assistant','phase':'final_answer','status':'completed','content':[{'type':'output_text','text':'SECOND_AFTER_EXHAUSTION'}]}]
        else:
            coverage = [{'id':str(args.output.resolve()/'workspace/proof.txt'),'dedup_key':'sha256-'+hashlib.sha256(b'KERNEL_VALIDATOR_OK').hexdigest()}]
            output = [{'type':'function_call','status':'completed','call_id':f'verify-{n}','name':'verify','arguments':json.dumps({'covered_set':coverage,'verdict':'pass','failures':[]})}]
        frames = []
        if output[0]['type'] == 'function_call':
            frames += [{'type':'response.output_item.added','output_index':0,'item':dict(output[0],arguments='')},
                       {'type':'response.function_call_arguments.done','output_index':0,'arguments':output[0]['arguments']},
                       {'type':'response.output_item.done','output_index':0,'item':output[0]}]
        response = {'id':f'resp-{response_number}','status':'completed','output':output}
        if n != 2: response['usage'] = {'input_tokens':10,'output_tokens':10}
        elif args.reported_usage: response['usage'] = {'input_tokens':10,'output_tokens':3}
        body = ('data: '+json.dumps({'type':'response.completed','response':response})+'\n\n').encode()
        body = ''.join('data: '+json.dumps(frame)+'\n\n' for frame in frames).encode() + body
        prefix = b''
        if n == 2:
            prefix = ('data: '+json.dumps({'type':'response.output_text.delta','delta':'KERNEL_VALIDATOR_OK'})+'\n\n').encode()
        self.send_response(200); self.send_header('Content-Type','text/event-stream'); self.send_header('Content-Length',str(len(prefix)+len(body))); self.end_headers()
        if prefix:
            self.wfile.write(prefix); self.wfile.flush()
            deadline = time.monotonic()+15
            # The terminal is withheld until the independent client has seen the streamed
            # text chunk: live chunks must reach a subscriber before the turn can end.
            while not (args.output/'runtime/client-chunk-observed.json').exists():
                if time.monotonic()>deadline: raise TimeoutError('external client did not receive the streamed chunk')
                time.sleep(.02)
        self.wfile.write(body); self.wfile.flush()

server = ThreadingHTTPServer(('127.0.0.1',0),Handler)
threading.Thread(target=server.serve_forever,daemon=True).start()
provider = {'id':'fixture','adapter':'responses','dialect':'openai_responses_v1','endpoint_owner':'openai','gateway_translation':'direct','evidence_revision':'openai-2026-08-01','endpoint':f'http://127.0.0.1:{server.server_port}/v1','credential_key':'fixture-key','models':[{'id':'gpt-5','profile':'openai_responses_v1:gpt-5','enabled':True,'context_window_tokens':32000,'compact_trigger_tokens':24000}]}
config = args.output.resolve()/'provider.json'
config.write_text(json.dumps(provider,indent=2))
subprocess.run(['cargo','build','-p','tekes-worker','-p','workspace-service','-p','tools','--bins','--locked'],cwd=root,check=True)
frozen = args.output.resolve()/'bin'
frozen.mkdir()
for name in ['tekes-worker','tekes-helper','tekes-workspace-service']:
    shutil.copy2(root/'target/debug'/name, frozen/name)
build = subprocess.run(['cargo','test','-p','tekes-supervisor','--lib','--no-run','--locked','--message-format=json'],cwd=root,check=True,stdout=subprocess.PIPE,text=True)
artifacts = [json.loads(line) for line in build.stdout.splitlines() if line.strip()]
executables = {entry['executable'] for entry in artifacts if entry.get('reason')=='compiler-artifact' and entry.get('executable') and entry.get('target',{}).get('name')=='tekes_supervisor'}
if len(executables)!=1: raise SystemExit('Expected exactly one supervisor test binary')
shutil.copy2(next(iter(executables)),frozen/'tekes-supervisor-live-test')
(args.output/'binaries.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in frozen.iterdir()},indent=2))
env = dict(os.environ,TEKES_KERNEL_LIVE_PROVIDER=str(config),TEKES_KERNEL_LIVE_KEY=key,TEKES_TEST_REAL_WORKER=str(frozen/'tekes-worker'),TEKES_PROCESS_ARTIFACT=str(args.output.resolve()/'runtime'))
env['TEKES_PUBLIC_APPROVAL'] = str(int(args.public_approval))
test_name = 'real_validator_exhaustion_releases_queued_input'
client_log = (args.output/'client.log').open('w')
client = subprocess.Popen([__import__('sys').executable,str(script_snapshot/'exhaustion-websocket-client.py'),str(args.output.resolve()/'runtime'),str((2 if args.commentary_before_final else 1)+(2 if args.skill_before_write else 0)),str(int(args.cancel_after_final)),str(int(args.public_approval))],stdout=client_log,stderr=subprocess.STDOUT)
run = subprocess.run([str(frozen/'tekes-supervisor-live-test'),'process_host::tests::'+test_name,'--exact','--ignored','--nocapture'],cwd=root,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
if run.returncode != 0: client.terminate()
client_code = client.wait(timeout=35)
client_log.close()
server.shutdown()
log = run.stdout.decode(errors='replace')
for secret in [key,*secrets.values()]:
    if len(secret)>8: log=log.replace(secret,'[REDACTED]')
(args.output/'test.log').write_text(log)
if run.returncode == 0 and client_code == 0:
    runtime = args.output/'runtime'
    overlap = json.loads((args.output/'concurrent-overlap.json').read_text())
    assert len(overlap)==3 and max(e['arrived'] for e in overlap) <= min(e['released'] for e in overlap)
    runtime_results = json.loads((runtime/'client-runtime-receipt.json').read_text())
    for result in runtime_results['ends']:
        ledger = runtime/'threads'/result['session']/'main.jsonl'
        facts = [json.loads(line) for line in ledger.read_text().splitlines()]
        outputs = [e for e in facts if e['kind']=='output']
        settles = [e for e in facts if e['kind']=='settle']
        assert len(outputs)==1 and outputs[0]['final_answer'] is True and outputs[0]['content']==[{'type':'text','text':'CONCURRENT_RUNTIME_ANSWER:'+result['session']}]
        assert len(settles)==1 and settles[0]['outcome']=='completed'
    creates = json.loads((runtime/'client-create-receipt.json').read_text())
    genesis = []
    for response in creates['responses']:
        created_id = response['result']['value']['sessionId']
        record = json.loads((runtime/'threads'/created_id/'main.jsonl').read_text().splitlines()[0])
        assert record['workspace']=='ws' and record['origin_key']==response['rpcId'] and record['thread']==created_id
        genesis.append(record)
    assert all(Path(item['cwd']).resolve()==(args.output/'workspace').resolve() for item in creates['inventory_notifications'])
    (runtime/'create-membership-audit.json').write_text(json.dumps({'status':'passed','genesis':genesis},indent=2))
    observed = json.loads((runtime/'client-chunk-observed.json').read_text())['event']
    session_id = json.loads((runtime/'client-endpoint.json').read_text())['session']
    ledger_path = runtime/'threads'/session_id/'main.jsonl'
    events = [json.loads(line) for line in ledger_path.read_text().splitlines()]
    if args.skill_before_write:
        before, offered, loaded = [json.loads((args.output/f'request-{n}.json').read_text()) for n in [1,2,3]]
        assert 'SKILL_LIVE_BODY_OK' not in json.dumps(before)
        offer_result = next(item for item in offered['input'] if item.get('call_id')=='skill-1')
        load_result = next(item for item in loaded['input'] if item.get('call_id')=='skill-2')
        assert offer_result['type']=='function_call_output' and 'live-proof.md' in offer_result['output']
        assert 'SKILL_LIVE_BODY_OK' not in json.dumps(offered)
        assert load_result['type']=='function_call_output' and 'SKILL_LIVE_BODY_OK' in load_result['output']
        (runtime/'skill-provider-projection.json').write_text(json.dumps({'status':'passed','offer_request':2,'load_request':3,'offer':offer_result,'load':load_result},indent=2))
        skill_calls = [e for e in events if e['kind']=='tool_call' and e['name'] in ['skill_explorer','skill'] and e['turn']==1]
        assert [e['name'] for e in skill_calls]==['skill_explorer','skill']
        for call in skill_calls:
            result = next(e for e in events if e['kind']=='tool_result' and e['call']==call['call'])
            assert result['outcome']=='ok', result
            if call['name']=='skill': assert 'SKILL_LIVE_BODY_OK' in json.dumps(result), result
    if args.compact_next_turn:
        overflow_index = json.loads((args.output/'overflow-request-index.json').read_text())['request']
        rebuilt = json.loads((args.output/f'request-{overflow_index+1}.json').read_text())
        assert 'previous_response_id' not in rebuilt
        assert '[compacted history]' in json.dumps(rebuilt)
        assert any('重试' in json.dumps(item,ensure_ascii=False) and item.get('role')=='user' for item in rebuilt['input'])
        (runtime/'compact-provider-request.json').write_text(json.dumps({'status':'passed','request_index':overflow_index+1,'request':rebuilt},indent=2,ensure_ascii=False))
        compacts = [e for e in events if e['kind']=='compact']
        assert len(compacts)==1, compacts
        epochs = [e for e in events if e['kind']=='epoch' and e.get('reason')=='compaction']
        assert len(epochs)==1 and epochs[0]['seq']>compacts[0]['seq']
        (runtime/'compact-audit.json').write_text(json.dumps({'status':'passed','compact':compacts[0],'epoch':epochs[0]},indent=2))
    if args.reload_skill_next_turn:
        results = [next(e for e in events if e['kind']=='tool_result' and e['call']==call) for call in ['reload-1','reload-2']]
        assert all(e['turn']==2 and e['outcome']=='ok' for e in results), results
        assert 'SKILL_LIVE_BODY_OK' in json.dumps(results[1])
        compact = next(e for e in events if e['kind']=='compact')
        assert compact['seq'] < results[0]['seq'] < results[1]['seq']
        projected = [request for request in requests if any(item.get('call_id')=='reload-2' and 'SKILL_LIVE_BODY_OK' in item.get('output','') for item in request.get('input',[]) if isinstance(item,dict))]
        assert len(projected)==1
        (runtime/'skill-reload-audit.json').write_text(json.dumps({'status':'passed','results':results,'provider_request':projected[0]},indent=2))
    if args.stale_skill_next_turn:
        stale = next(e for e in events if e['kind']=='tool_result' and e['call']=='stale-skill')
        assert stale['turn']==2 and stale['outcome']=='error' and 'not causally offered' in json.dumps(stale), stale
        (runtime/'stale-skill-audit.json').write_text(json.dumps({'status':'passed','rejection':stale},indent=2))
    if args.commentary_before_final:
        ordinary = [e for e in events if e['kind']=='output' and 'STILL_WORKING_NOT_FINAL' in json.dumps(e)]
        assert len(ordinary)==1 and ordinary[0]['turn']==1, ordinary
        assert ordinary[0]['final_answer'] is False
        first_settle = next(e for e in events if e['kind']=='settle')
        final = next(e for e in events if e['kind']=='output' and e['turn']==1 and e['final_answer'])
        validation = next(e for e in events if e['kind']=='spawn')
        assert ordinary[0]['seq'] < final['seq'] < validation['seq'] < first_settle['seq']
        assert first_settle['validation']['promoted_output_seq'] == final['seq']
        assert len([e for e in events if e['kind']=='turn_open' and e['turn']==1])==1
        (runtime/'commentary-loop-audit.json').write_text(json.dumps({'status':'passed','ordinary_output_seq':ordinary[0]['seq'],'final_output_seq':final['seq'],'validator_spawn_seq':validation['seq'],'settle_seq':first_settle['seq'],'turn':1},indent=2))
    if args.cancel_after_final:
        cancellations = [e for e in events if e.get('origin_tuple',{}).get('op')=='session.cancel']
        assert len(cancellations)==1, cancellations
        settles = [e for e in events if e['kind']=='settle']
        assert len(settles)==2 and cancellations[0]['seq'] > settles[-1]['seq']
        (runtime/'cancel-after-final-audit.json').write_text(json.dumps({'status':'passed','cancellation':cancellations[0],'settles':settles},indent=2))
    # Usage has one owner: the durable usage event written from the provider terminal. A chunk
    # carries no token claim, so the client saw the streamed text and nothing else.
    assert 'usagePreview' not in observed['data'], observed
    final_output = next(e for e in events if e['kind']=='output' and e['turn']==1 and e['final_answer'])
    attempt, usage = final_output['attempt'], final_output['usage']
    assert usage['availability']==('reported' if args.reported_usage else 'unavailable'), usage
    if args.reported_usage:
        assert usage['output_tokens'] == '3', usage
    journal = [json.loads(line)['event'] for line in ledger_path.with_name('endpoint.jsonl').read_text().splitlines()]
    replayed = next(event for event in journal if event.get('data')==observed['data'])
    (runtime/'usage-replay-audit.json').write_text(json.dumps({'status':'passed','attempt':attempt,'kernel_usage':usage,'replayed_chunk':replayed},indent=2))
passed = client_code == 0 and run.returncode == 0 and '1 passed' in log and (args.output/'runtime/receipt.json').exists()
passed = passed and not any(marker in log for marker in (
    'process-host-publish-frame-failed:', 'process-host-publish-appended-failed:',
    'process-host-publish-child-actionables-failed:', 'process-host-reconcile-failed:',
))
print('passed' if passed else 'failed')
raise SystemExit(0 if passed else 1)

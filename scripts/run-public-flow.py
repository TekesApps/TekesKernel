#!/usr/bin/env python3
"""Exercise a preserved flow case through an actual worker and public v3 client."""
import argparse, hashlib, json, os, re, shutil, signal, subprocess, sys, threading, time
from pathlib import Path
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--case',choices=['text','write','shell','read','glob','grep'],default='text')
parser.add_argument('--output',type=Path,required=True)
parser.add_argument('--live',action='store_true')
parser.add_argument('--goal',action='store_true',help='Scripted text case: exercise durable goal continuation through the public supervisor')
parser.add_argument('--wholesale',action='store_true',help='Scripted write case: two 402 wholesale-limit responses after the write result, the worker is killed during the durable admission wait, and the original turn must finish')
parser.add_argument('--queue-during-turn',action='store_true',help='Submit a second ordinary prompt while the first turn is running; it must wait for settlement (validator included) and complete as turn 2')
parser.add_argument('--live-provider',help='Configured provider id for --live (default: the single Cloudflare Responses provider)')
parser.add_argument('--live-secret-name',help='Credential name in the keys file for --live-provider (default: the provider endpoint owner)')
parser.add_argument('--skill-compact',action='store_true',help='Live text case: legacy skill projection PRE-COMPACT turn, the named compact command (manual compaction), then the POST-COMPACT turn on the compaction generation')
parser.add_argument('--swebench-instance',type=Path,help='Live text case driven by one SWE-bench instance JSON (legacy SWEBenchDriver instruction); the workspace is pre-provisioned by scripts/run-swebench.py')
parser.add_argument('--validation-repair',action='store_true',help='Live text case: legacy validation-repair prompt (deliberate wrong draft, validator feedback, same-session repair) plus the public usage.summary/usage.cacheAttribution assertions')
parser.add_argument('--compaction',action='store_true',help='Live text case: four seeded turns each carrying a private code, the named compact command (supervisor-authored manual compaction with the model summary request), then one recall turn on the compaction generation')
parser.add_argument('--codes',help='Comma-separated private codes for --compaction (default: four fresh codes)')
parser.add_argument('--no-route-probes',action='store_true',help='Skip the pre-turn session.models/session.selectModel probes so the turn resolves the catalog default')
parser.add_argument('--parallel-tools',action='store_true')
parser.add_argument('--tool-ready-gate',action='store_true',help='Scripted provider waits for public tool completion before response terminal')
parser.add_argument('--approval-batch-gate',action='store_true',help='Scripted write case: approve the batch head before terminal, then approve two editing siblings')
parser.add_argument('--agent-eval',choices=['memory-recall-codename','subagent-summarize-notes'])
parser.add_argument('--toolchain-root',type=Path,action='append',default=[],help='Explicit read-only toolchain installation; adds its bin to tool PATH')
parser.add_argument('--corpus',choices=['hello','deepseek-responses-smoke','weighted-ttl','quota-ledger','long-session-cache','large-prefix-cache','batch-ledger-bugfix'])
parser.add_argument('--seed-agents-md',type=Path,help='Corpus runs: seed the workspace with this file as AGENTS.md so the instruction snapshot carries it (the Kernel-owned instruction source)')
parser.add_argument('--config', type=Path, default=os.environ.get('TEKES_PROVIDERS_CONFIG'), required='TEKES_PROVIDERS_CONFIG' not in os.environ, help='providers.json (default: $TEKES_PROVIDERS_CONFIG)')
parser.add_argument('--keys', type=Path, default=os.environ.get('TEKES_LIVE_KEYS_FILE'), help='Key file for --live (default: $TEKES_LIVE_KEYS_FILE); TEKES_KERNEL_LIVE_KEY overrides it')
args=parser.parse_args()
if args.tool_ready_gate and args.live:parser.error('--tool-ready-gate is a scripted socket test')
if args.approval_batch_gate and (args.live or args.case!='write' or args.tool_ready_gate):parser.error('--approval-batch-gate needs the scripted write case without --tool-ready-gate')
if args.wholesale and (args.live or args.case!='write'):parser.error('--wholesale is the scripted write case')
if args.skill_compact and (not args.live or args.case!='text'):parser.error('--skill-compact is the live text case')
if args.validation_repair and (not args.live or args.case!='text'):parser.error('--validation-repair is the live text case')
if args.compaction and (not args.live or args.case!='text'):parser.error('--compaction is the live text case')
if args.swebench_instance and (not args.live or args.case!='text'):parser.error('--swebench-instance is the live text case')
if args.goal and (args.live or args.case!='text' or args.corpus):parser.error('--goal requires the scripted text case')
root=Path(__file__).resolve().parents[1]
# A SWE-bench drive pre-provisions <output>/workspace; every other run starts from a new directory.
args.output.mkdir(parents=True,exist_ok=bool(args.swebench_instance))
case=next(c for c in json.loads((root/'fixtures/live/flow/cases.json').read_text()) if c['id']==args.case)
case['live']=args.live
case['expected_file']={'path':'docs/flow-write.md','content':'FLOW_OK\n'} if args.case=='write' else None
if args.live:
    live_token='APP_SERVER_LIVE_FLOW_'+args.case.upper()+'_KERNEL'
    case['prompt']=case['prompt'].replace(case['token'],live_token);case['token']=live_token
    if args.case=='write':
        case['prompt']='Use the apply_patch tool to create docs/live-smoke.md.\nThe file content must be exactly:\nLIVE_OK\n\nAfter the file is written, provide the final answer exactly:\n'+live_token
        case['expected_file']={'path':'docs/live-smoke.md','content':'LIVE_OK'}
if args.queue_during_turn:
    if args.case not in ('read','write','shell','glob','grep'):parser.error('--queue-during-turn needs a tool-bearing case')
    case['token2']='QUEUED_TURN_'+args.case.upper()+'_KERNEL'
    case['queued_prompt']='Reply with exactly the text '+case['token2']+' and nothing else.'
    case['timeout_seconds']=max(case.get('timeout_seconds') or 0,420)
if args.compaction:
    import secrets as _secrets
    codes=args.codes.split(',') if args.codes else ['CODE-%s-%s'%(word,_secrets.token_hex(2).upper()) for word in ('AMBER','COBALT','SAFFRON','VIRIDIAN')]
    assert len(codes)==4,codes
    case['codes']=codes
    case['seed_prompts']=['Private code %d is %s. Remember it exactly; it will be asked for later. Reply with exactly: OK'%(i+1,code) for i,code in enumerate(codes)]
    case['prompt']=case['seed_prompts'][0]
    case['post_prompt']='Without using any tool, list every private code I gave you earlier in this conversation, exactly as written, comma-separated, and nothing else.'
    case['prompts']=case['seed_prompts']+[case['post_prompt']]
    case['token']=None;case['free_final']=True;case['compaction']=True
    case['seed_files']={'.agent/commands/compact.md':'---\ndescription: Force an effective compaction\n---\ncompact\n'}
    case['allowed_tools']=['read','glob','grep'];case['timeout_seconds']=900;case['max_wall_seconds']=300;case['receive_timeout_seconds']=600
if args.validation_repair:
    case['prompt']='The final deliverable must be proof.txt containing exactly KERNEL_FILE_OK. This is a validation repair test: before any validation.feedback exists, deliberately create the first draft containing exactly WRONG and answer exactly KERNEL_DRAFT. Do not repair it early; the validator must reject this draft against the final deliverable requirement. When validation.feedback arrives, repair proof.txt in this same session using apply_patch, read it to verify, and answer exactly KERNEL_FILE_OK.'
    case['token']='KERNEL_FILE_OK';case['validation_repair']=True;case['usage_rpcs']=True
    case['allowed_tools']=['read','apply_patch'];case['timeout_seconds']=900;case['max_wall_seconds']=600;case['receive_timeout_seconds']=600
if args.swebench_instance:
    # Legacy SWEBenchDriver.instruction(for:), verbatim except the tool names,
    # which are the Kernel catalog's (apply_patch is the write tool).
    instance=json.loads(args.swebench_instance.read_text())
    case['prompt']=('You are working inside a checked-out copy of the '+instance['repo']+' repository. '
        'Resolve the following issue by editing the repository\'s own source code. Use the available tools '
        '(read, grep, glob, shell, apply_patch) to investigate and make the change. Do NOT edit or add test files '
        '— the fix must make the project\'s existing tests pass. When the change is complete, stop.\n\nIssue:\n'+instance['problem_statement'])
    case['token']=None;case['free_final']=True;case['swebench']={'instance_id':instance['instance_id'],'repo':instance['repo'],'base_commit':instance['base_commit']}
    case['allowed_tools']=['read','grep','glob','shell','apply_patch'];case['timeout_seconds']=int(os.environ.get('TEKES_SWEBENCH_TIMEOUT_SECONDS','1500'));case['max_wall_seconds']=int(os.environ.get('TEKES_SWEBENCH_TURN_WALL_SECONDS','1200'))
    case['seed_files']={};case['receive_timeout_seconds']=900
if args.skill_compact:
    # Legacy workflow_live_skill_projection_resets_and_reloads_after_effective_compact,
    # verbatim skill, command and prompts; Kernel skills/commands live under .agent/.
    case['seed_files']={
        '.agent/skills/uat-format/SKILL.md':'---\nname: uat-format\ndescription: Formats the final skill projection UAT acceptance marker.\n---\nIf the current request says PRE-COMPACT, reply with exactly `SKILL_PRE_COMPACT_OK`.\nIf the current request says POST-COMPACT, reply with exactly `SKILL_POST_COMPACT_OK`.\n',
        '.agent/commands/compact.md':'---\ndescription: Force an effective Compact v4 checkpoint\n---\ncompact\n'}
    case['prompt']='This is PRE-COMPACT. First call skill_explorer for the UAT marker formatter, then call\nskill for uat-format, then follow the loaded SKILL.md. Do not answer before both calls\nfinish.'
    case['post_prompt']='This is POST-COMPACT. Rediscover the UAT marker formatter with skill_explorer, call skill\nfor uat-format again, then follow the newly loaded SKILL.md. Do not answer before both\ncalls finish.'
    case['prompts']=[case['prompt'],case['post_prompt']]
    case['pre_token']='SKILL_PRE_COMPACT_OK';case['token']='SKILL_POST_COMPACT_OK';case['skill_compact']=True;case['timeout_seconds']=600
    case['allowed_tools']=['read','apply_patch','shell','glob','grep','skill_explorer','skill'];case['max_wall_seconds']=300
if args.corpus:
    if not args.live:raise SystemExit('--corpus requires --live')
    from cache_corpus import prompts, seed_files
    # The corpus measures prompt-cache behaviour over a whole agent session, so
    # it sets no turn wall budget at all: `max_wall_seconds` is a product policy
    # and absence means no whole-turn cap, which is how an installed Kernel runs.
    # A budget that expires mid-session measures the budget, and its own request
    # cancellation surfaces as a transport error that reads like a defect. The
    # harness keeps a process-level bound so a live run cannot hang forever, and
    # a turn budget can still be set explicitly when a case wants to exercise it.
    case={'id':'cache-corpus','scenario':args.corpus,'live':True,'prompts':prompts(args.corpus),'seed_files':seed_files(args.corpus),
          'timeout_seconds':int(os.environ.get('TEKES_CACHE_CORPUS_TIMEOUT_SECONDS','7200'))}
    if args.corpus in ('long-session-cache','large-prefix-cache'):
        case['allowed_tools']=[]
    else:
        case['allowed_tools']=['write','edit','apply_patch','read','shell','glob','grep']
    # Child validation can be active without emitting a frame on the root
    # journal subscription. Bound that silence by the existing process limit,
    # rather than terminating a healthy corpus at the generic 180-second read.
    case['receive_timeout_seconds']=case['timeout_seconds']
    if args.seed_agents_md:
        case['seed_files']['AGENTS.md']=args.seed_agents_md.read_text()
        case['agents_md_sha256']=hashlib.sha256(case['seed_files']['AGENTS.md'].encode()).hexdigest()
    if os.environ.get('TEKES_CACHE_CORPUS_TURN_WALL_SECONDS'):
        case['max_wall_seconds']=int(os.environ['TEKES_CACHE_CORPUS_TURN_WALL_SECONDS'])
if args.agent_eval:
    if not args.live or args.corpus:raise SystemExit('--agent-eval requires --live and excludes --corpus')
    from agent_eval import TASKS
    case=dict(TASKS[args.agent_eval],id='agent-eval',scenario=args.agent_eval,live=True,timeout_seconds=600,max_wall_seconds=240)
    case['allowed_tools']=['task','subagent','read','apply_patch','shell','glob','grep']
if args.parallel_tools:
    if not args.live or args.corpus or args.agent_eval:parser.error('--parallel-tools requires --live and excludes corpus/agent-eval')
    case={'id':'parallel-tools','live':True,'token':'PARALLEL_TOOLS_DONE','call':None,'seed_files':{},'expected_file':None,'allowed_tools':['mcp__local__record_left','mcp__local__record_right'],'prompt':'Call mcp__local__record_left with value "A" and mcp__local__record_right with value "B" together in ONE response, in parallel. Do not wait for the first result before making the second call. After both tools succeed, reply exactly PARALLEL_TOOLS_DONE.','timeout_seconds':300,'max_wall_seconds':180}
if args.goal:
    case['goal']=True
    case['prompt']='First finish a turn with exactly ROUND_ONE. On the next automatic goal turn, call set_goal_state with state complete, then finish with exactly '+case['token']+'.'
    case['allowed_tools']=['set_goal_state']
    case['timeout_seconds']=300
case['toolchain_roots']=[str(path.resolve(strict=True)) for path in args.toolchain_root]
case_path=args.output.resolve()/'case.json';case_path.write_text(json.dumps(case,indent=2))
requests=[]
credential_hits=[]
completed_calls=set()
wholesale_hits=[]
killed_workers=[]
queued_turn_requests=[]
def kill_worker_during_wait():
    # The frozen worker binary path is unique to this output directory, so the
    # match cannot reach another run's worker.
    needle=str(args.output.resolve()/'bin/tekes-worker')
    deadline=time.monotonic()+5
    while time.monotonic()<deadline:
        time.sleep(0.4)
        listing=subprocess.run(['ps','-axo','pid=,command='],stdout=subprocess.PIPE,text=True).stdout
        pids=[int(line.split()[0]) for line in listing.splitlines() if needle in line]
        if pids:
            for pid in pids:
                try:os.kill(pid,signal.SIGKILL);killed_workers.append(pid)
                except ProcessLookupError:pass
            return

class Handler(BaseHTTPRequestHandler):
    def log_message(self,*args):pass
    def do_POST(self):
        # The credential reaches the provider only through the secret store
        # binding named by the config (never argv/env of the worker).
        assert self.headers.get('Authorization')=='Bearer '+secret, 'provider request lacks the bound credential'
        credential_hits.append(1)
        request=json.loads(self.rfile.read(int(self.headers['Content-Length'])))
        requests.append(request);n=len(requests)
        (args.output/f'request-{n}.json').write_text(json.dumps(request,indent=2))
        names=[t.get('name') for t in request.get('tools',[])]
        results=[i.get('call_id') for i in request.get('input',[]) if i.get('type')=='function_call_output']
        if args.wholesale and 'flow-tool' in results and 'verify' not in names and len(wholesale_hits)<2:
            # Cloudflare AI Gateway wholesale admission limit on the request that
            # carries the write result. The worker must wait durably and retry the
            # same open turn; the kill during the first wait proves the retry
            # survives the process.
            wholesale_hits.append(n)
            body=json.dumps({'error':{'code':'invalid_prompt','message':'Wholesale rate limit exceeded for this gateway. Please reduce request rate or use BYOK.'}}).encode()
            self.send_response(402);self.send_header('Content-Type','application/json');self.send_header('Retry-After','3');self.send_header('Content-Length',str(len(body)));self.end_headers();self.wfile.write(body)
            if len(wholesale_hits)==1:threading.Thread(target=kill_worker_during_wait,daemon=True).start()
            return
        if 'verify' not in names: completed_calls.update(results)
        if case.get('queued_prompt') and case['queued_prompt'] in json.dumps(request.get('input',[])) and 'verify' not in names:
            # Turn 2 (the queued input) is served after turn 1 settled; the
            # scripted provider must never see it inside turn 1's requests.
            assert any('flow-tool' in json.dumps(r) for r in requests[:-1]),'queued prompt reached the provider before the first turn ran its tool'
            queued_turn_requests.append(n)
            output=[{'type':'message','role':'assistant','phase':'final_answer','status':'completed','content':[{'type':'output_text','text':case['token2']}]}]
            frames=[{'type':'response.completed','response':{'id':f'flow-{n}','status':'completed','output':output,'usage':{'input_tokens':10,'output_tokens':10}}}]
            body=''.join('data: '+json.dumps(f)+'\n\n' for f in frames).encode()
            self.send_response(200);self.send_header('Content-Type','text/event-stream');self.send_header('Content-Length',str(len(body)));self.end_headers();self.wfile.write(body);return
        if 'verify' in names:
            path=args.output.resolve()/'workspace/docs/flow-write.md'
            assert path.read_bytes()==b'FLOW_OK\n'
            name='verify';arguments={'covered_set':[{'id':str(path),'dedup_key':'sha256-'+hashlib.sha256(path.read_bytes()).hexdigest()}],'verdict':'pass','failures':[]}
        elif case.get('goal') and n==1:
            name=None
        elif case.get('goal') and 'flow-goal-state' not in results:
            name='set_goal_state';arguments={'state':'complete','progress':None,'reason':None,'user_action':None}
        elif case['call'] and 'flow-tool' not in completed_calls:
            name=case['call']['name'];arguments=case['call']['arguments']
        else:name=None
        if name:
            call_id='flow-verify' if name=='verify' else ('flow-goal-state' if case.get('goal') else 'flow-tool')
            output=[{'type':'function_call','status':'completed','call_id':call_id,'name':name,'arguments':json.dumps(arguments)}]
            frames=[{'type':'response.output_item.added','output_index':0,'item':dict(output[0],arguments='')},{'type':'response.function_call_arguments.done','output_index':0,'arguments':output[0]['arguments']},{'type':'response.output_item.done','output_index':0,'item':output[0]}]
            if args.approval_batch_gate and name=='apply_patch':
                for index,(before,after) in enumerate([('FLOW_OK','FLOW_INTERMEDIATE'),('FLOW_INTERMEDIATE','FLOW_OK')],1):
                    sibling=dict(arguments,operation='update_file',diff=f'-{before}\n+{after}\n',expected_artifact_version=index)
                    item={'type':'function_call','status':'completed','call_id':f'flow-sibling-{index}','name':name,'arguments':json.dumps(sibling)}
                    output.append(item)
                    frames.extend([{'type':'response.output_item.added','output_index':index,'item':dict(item,arguments='')},{'type':'response.function_call_arguments.done','output_index':index,'arguments':item['arguments']},{'type':'response.output_item.done','output_index':index,'item':item}])
        else:
            final_text='ROUND_ONE' if case.get('goal') and n==1 else case['token']
            output=[{'type':'message','role':'assistant','phase':'final_answer','status':'completed','content':[{'type':'output_text','text':final_text}]}];frames=[]
        frames.append({'type':'response.completed','response':{'id':f'flow-{n}','status':'completed','output':output,'usage':{'input_tokens':10,'output_tokens':10}}})
        body=''.join('data: '+json.dumps(f)+'\n\n' for f in frames).encode()
        self.send_response(200);self.send_header('Content-Type','text/event-stream');self.send_header('Content-Length',str(len(body)));self.end_headers()
        if args.approval_batch_gate and name=='apply_patch':
            prefix=''.join('data: '+json.dumps(f)+'\n\n' for f in frames[:3]).encode()
            self.wfile.write(prefix);self.wfile.flush()
            ledger=args.output/'runtime/threads/018f0000-0000-7000-8000-000000000129/main.jsonl'
            deadline=time.monotonic()+15
            while time.monotonic()<deadline:
                if ledger.exists():
                    rows=[]
                    for line in ledger.read_text().splitlines():
                        try:rows.append(json.loads(line))
                        except json.JSONDecodeError:pass
                    if any(r.get('kind')=='approval_response' and r.get('call')=='flow-tool' for r in rows):break
                time.sleep(.01)
            else:raise AssertionError('batch head approval did not arrive before terminal')
            self.wfile.write(body[len(prefix):])
        elif args.tool_ready_gate and name:
            prefix=''.join('data: '+json.dumps(f)+'\n\n' for f in frames[:-1]).encode()
            self.wfile.write(prefix);self.wfile.flush()
            deadline=time.monotonic()+15
            received=False
            while time.monotonic()<deadline:
                path=args.output/'runtime/client-frames.jsonl'
                if path.exists():
                    for line in path.read_text().splitlines():
                        try:record=json.loads(line)
                        except json.JSONDecodeError:continue
                        chunk=record.get('frame',{}).get('event',{}).get('data',{}).get('chunk',{})
                        if chunk.get('id')==call_id and chunk.get('argumentsComplete') is True:
                            received=True;break
                if received:break
                time.sleep(.01)
            assert received,'public client did not observe tool completion before terminal'
            (args.output/f'tool-ready-{call_id}.json').write_text(json.dumps({'call_id':call_id,'before_terminal':True}))
            self.wfile.write(body[len(prefix):])
        else:self.wfile.write(body)
server=ThreadingHTTPServer(('127.0.0.1',0),Handler)
threading.Thread(target=server.serve_forever,daemon=True).start()
provider={'id':'fixture','adapter':'responses','dialect':'openai_responses_v1','endpoint_owner':'openai','gateway_translation':'direct','evidence_revision':'openai-2026-08-01','endpoint':f'http://127.0.0.1:{server.server_port}/v1','credential_key':'fixture-key','models':[{'id':'gpt-5','profile':'openai_responses_v1:gpt-5','enabled':True,'context_window_tokens':32000,'compact_trigger_tokens':24000}]}
secret='disposable-fixture-key'
if args.live:
    configured=json.loads(args.config.read_text())['providers']
    if args.live_provider:
        providers=[p for p in configured if p['id']==args.live_provider]
        if len(providers)!=1:raise SystemExit(f'provider {args.live_provider} is not configured')
    else:
        providers=[p for p in configured if p['dialect']=='openai_responses_v1' and p['endpoint_owner']=='cloudflare']
        if len(providers)!=1:raise SystemExit('Expected exactly one configured Cloudflare Responses provider')
    provider=providers[0]
    secrets=dict(re.findall(r'^\s*"([\w]+)"\s*:\s*"([^"\r\n]+)"',args.keys.read_text(),re.M)) if args.keys and args.keys.exists() else {}
    secret_name=args.live_secret_name or ('cloudflare' if provider['endpoint_owner']=='cloudflare' else provider['endpoint_owner'])
    secret=os.environ.get('TEKES_KERNEL_LIVE_KEY') or secrets.get(secret_name)
    if not secret:raise SystemExit(f'credential {secret_name} unavailable in the keys file')
config=args.output.resolve()/'provider.json';config.write_text(json.dumps(provider,indent=2))
case['model_id']=provider['models'][0]['id'];case['provider_id']=provider['id'];case['route_probes']=not args.no_route_probes;case['wholesale']=args.wholesale;case_path.write_text(json.dumps(case,indent=2))
subprocess.run(['cargo','build','-p','tekes-worker','-p','workspace-service','-p','tools','-p','mcp','--bins','--locked'],cwd=root,check=True)
frozen=args.output.resolve()/'bin';frozen.mkdir()
for name in ['tekes-worker','tekes-helper','tekes-workspace-service','mcp-fixture-server']:shutil.copy2(root/'target/debug'/name,frozen/name)
build=subprocess.run(['cargo','test','-p','tekes-supervisor','--lib','--no-run','--locked','--message-format=json'],cwd=root,check=True,stdout=subprocess.PIPE,text=True)
artifacts=[json.loads(line) for line in build.stdout.splitlines() if line.strip()]
executables={entry['executable'] for entry in artifacts if entry.get('reason')=='compiler-artifact' and entry.get('executable') and entry.get('target',{}).get('name')=='tekes_supervisor'}
assert len(executables)==1
shutil.copy2(next(iter(executables)),frozen/'tekes-supervisor-live-test')
(args.output/'binaries.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in frozen.iterdir()},indent=2))

client_script=args.output.resolve()/'flow-websocket-client.py';shutil.copy2(root/'scripts'/client_script.name,client_script)
shutil.copy2(__file__,args.output/'runner.py')
env=dict(os.environ,TEKES_KERNEL_LIVE_PROVIDER=str(config),TEKES_KERNEL_LIVE_KEY=secret,TEKES_TEST_REAL_WORKER=str(frozen/'tekes-worker'),TEKES_TEST_MCP_FIXTURE_SERVER=str(frozen/'mcp-fixture-server'),TEKES_PROCESS_ARTIFACT=str(args.output.resolve()/'runtime'),TEKES_FLOW_CASE=str(case_path))
if args.skill_compact:
    wire=args.output.resolve()/'wire';wire.mkdir();env['TEKES_KERNEL_LIVE_ARTIFACT']=str(wire)
with (args.output/'client.log').open('w') as log:
    client=subprocess.Popen([sys.executable,str(client_script),str(args.output.resolve()/'runtime'),str(case_path)],stdout=log,stderr=subprocess.STDOUT)
    run=subprocess.run([str(frozen/'tekes-supervisor-live-test'),'process_host::tests::real_public_flow_case','--exact','--ignored','--nocapture'],cwd=root,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    if run.returncode:client.terminate()
    client_code=client.wait(timeout=30)
server.shutdown()
log=run.stdout.decode(errors='replace').replace(secret,'[REDACTED]');(args.output/'test.log').write_text(log)
frames=args.output/'runtime'/'client-frames.jsonl'
assert not frames.exists() or secret not in frames.read_text(), 'the provider credential leaked into public client frames'
passed=run.returncode==client_code==0 and '1 passed' in log and (args.output/'runtime/receipt.json').exists()
dispatched=0
for ledger in (args.output/'runtime/threads').rglob('*.jsonl'):
    for line in ledger.read_text().splitlines():
        try:event=json.loads(line)
        except json.JSONDecodeError:continue
        dispatched+=event.get('kind')=='attempt_dispatched'
matrix={'status':'passed' if passed else 'failed','case':case['id'],'scenario':case.get('scenario'),'scripted_provider_requests':len(requests),'dispatched_attempts':dispatched,'test_exit':run.returncode,'client_exit':client_code}
if args.approval_batch_gate:
    ledger=args.output/'runtime/threads/018f0000-0000-7000-8000-000000000129/main.jsonl'
    rows=[json.loads(line) for line in ledger.read_text().splitlines()]
    calls={call:{kind:[r for r in rows if r.get('kind')==kind and r.get('call')==call]
        for kind in ('approval_response','tool_execution_started','tool_result')}
        for call in ('flow-tool','flow-sibling-1','flow-sibling-2')}
    terminal=next(r['seq'] for r in rows if r['kind']=='output')
    ordered=all(all(len(records[k])==1 for k in records)
        and records['approval_response'][0]['seq']<records['tool_execution_started'][0]['seq']<records['tool_result'][0]['seq']
        and records['tool_result'][0]['outcome']=='ok' for records in calls.values())
    inline=bool(calls['flow-tool']['approval_response']) and calls['flow-tool']['approval_response'][0]['seq']<terminal
    siblings_after=all(records['approval_response'] and terminal<records['approval_response'][0]['seq']
        for call,records in calls.items() if call!='flow-tool')
    completed=[r.get('outcome') for r in rows if r['kind']=='settle']==['completed']
    gate={'ordered_once':ordered,'head_answer_before_terminal':inline,'sibling_answers_after_terminal':siblings_after,'completed':completed,
        'records':{call:{kind:[r['seq'] for r in records] for kind,records in groups.items()} for call,groups in calls.items()}}
    (args.output/'approval-batch-proof.json').write_text(json.dumps(gate,indent=2))
    passed=passed and ordered and inline and siblings_after and completed
    matrix['approval_batch']=gate;matrix['status']='passed' if passed else 'failed'
if args.queue_during_turn:
    receipt=json.loads((args.output/'runtime/client-flow-receipt.json').read_text()) if (args.output/'runtime/client-flow-receipt.json').exists() else {}
    matrix['queue_release']=receipt.get('queue_release');matrix['queued_turn_requests']=queued_turn_requests
    passed=passed and bool(receipt.get('queue_release'))
    matrix['status']='passed' if passed else 'failed'
if args.compaction:
    gate_path=args.output/'runtime/compaction-summary.json'
    gate=json.loads(gate_path.read_text()) if gate_path.exists() else None
    receipt=json.loads((args.output/'runtime/client-flow-receipt.json').read_text()) if (args.output/'runtime/client-flow-receipt.json').exists() else {}
    matrix['compaction']={'gate':gate,'client':receipt.get('compaction')}
    passed=passed and bool(gate) and bool((gate.get('summary_request') or {}).get('accepted')) and bool(receipt.get('compaction'))
    matrix['status']='passed' if passed else 'failed'
if args.validation_repair:
    receipt=json.loads((args.output/'runtime/client-flow-receipt.json').read_text()) if (args.output/'runtime/client-flow-receipt.json').exists() else {}
    rows=[]
    for ledger in (args.output/'runtime/threads').rglob('main.jsonl'):
        rows=[json.loads(line) for line in ledger.read_text().splitlines() if line.strip()]
    feedback=sum(1 for r in rows if r.get('kind')=='state' and r.get('subkind')=='validation.feedback')
    spawns=sum(1 for r in rows if r.get('kind')=='spawn')
    writes=sum(1 for r in rows if r.get('kind')=='tool_call' and r.get('name')=='apply_patch')
    settles=[r.get('outcome') for r in rows if r.get('kind')=='settle']
    proof=args.output/'workspace/proof.txt'
    usage=receipt.get('usage') or {}
    attribution=(usage.get('usage.cacheAttribution') or {}).get('entries') or []
    summary=usage.get('usage.summary') or {}
    matrix['validation_repair']={'feedback_rounds':feedback,'validator_spawns':spawns,'apply_patch_calls':writes,'settles':settles,'proof_exact':proof.exists() and proof.read_text().strip()=='KERNEL_FILE_OK','attribution_entries':len(attribution),'attribution_sample':attribution[:2],'summary':summary}
    v=matrix['validation_repair']
    passed=passed and 1<=v['feedback_rounds']<=2 and v['validator_spawns']==v['feedback_rounds']+1 and v['apply_patch_calls']>=2 and v['settles']==['completed'] and v['proof_exact'] and v['attribution_entries']>=1
    matrix['status']='passed' if passed else 'failed'
if args.skill_compact:
    gate_path=args.output/'runtime/compact-gate.json'
    gate=json.loads(gate_path.read_text()) if gate_path.exists() else None
    receipt=json.loads((args.output/'runtime/client-flow-receipt.json').read_text()) if (args.output/'runtime/client-flow-receipt.json').exists() else {}
    wire_check=None
    if gate:
        request_path=args.output/'wire'/(gate['first_post_compact_attempt']+'.request.json')
        if request_path.exists():
            body=request_path.read_text();request=json.loads(body)
            # Legacy asserted the first post-compact frame referenced none of the
            # pre-compact skill records. Kernel form: no request item (tool call or
            # tool output) carries a pre-compact call id; the compaction summary
            # is a user-role text that quotes the covered events, so the ids may
            # appear there and nowhere else.
            items=request.get('input',[]) if isinstance(request.get('input'),list) else request.get('messages',[])
            structural=[json.dumps(item) for item in items if isinstance(item,dict) and item.get('type') in ('function_call','function_call_output') or (isinstance(item,dict) and (item.get('call_id') or item.get('tool_call_id') or item.get('tool_calls')))]
            summary=[item for item in items if isinstance(item,dict) and 'compacted history' in json.dumps(item)]
            wire_check={'request':str(request_path),'mentions_post_compact':'POST-COMPACT' in body,'pre_compact_call_items_absent':not any(cid in text for cid in gate['pre_compact_call_ids'] for text in structural),'compacted_history_item_present':bool(summary),'pre_compact_call_ids_only_in_summary':all((cid in json.dumps(summary)) for cid in gate['pre_compact_call_ids']) and not any(cid in json.dumps([i for i in items if i not in summary]) for cid in gate['pre_compact_call_ids'])}
    matrix['skill_compact']={'gate':gate,'client':receipt.get('skill_compact'),'wire':wire_check}
    passed=passed and bool(gate) and bool(receipt.get('skill_compact')) and bool(wire_check) and wire_check['mentions_post_compact'] and wire_check['pre_compact_call_items_absent'] and wire_check['compacted_history_item_present']
    matrix['status']='passed' if passed else 'failed'
if args.wholesale:
    rows=[]
    for ledger in (args.output/'runtime/threads').rglob('main.jsonl'):
        rows=[json.loads(line) for line in ledger.read_text().splitlines() if line.strip()]
    admissions=[r for r in rows if r.get('kind')=='state' and r.get('subkind')=='provider_admission']
    limited=[r for r in rows if r.get('kind')=='error' and r.get('classification')=='rate_limit' and r.get('recoverable') is True]
    runs=[r for r in rows if r.get('kind')=='run_start']
    settles=[r for r in rows if r.get('kind')=='settle']
    written=(args.output/'workspace'/case['expected_file']['path'])
    matrix['wholesale']={'wholesale_responses':len(wholesale_hits),'killed_workers':killed_workers,'rate_limit_errors':len(limited),'admission_waits':[{'seq':r['seq'],'next_attempt_at':r['payload']['next_attempt_at'],'wait_ms':r['payload']['wait_ms'],'declared_retry_after':r['payload']['declared_retry_after']} for r in admissions],'worker_runs':len(runs),'settle':[s.get('outcome') for s in settles],'written_bytes_exact':written.exists() and written.read_text()==case['expected_file']['content']}
    ok=matrix['wholesale']
    passed=passed and ok['wholesale_responses']==2 and len(ok['killed_workers'])>=1 and ok['rate_limit_errors']==2 and len(ok['admission_waits'])==2 and ok['worker_runs']>=2 and ok['settle']==['completed'] and ok['written_bytes_exact'] and len(requests)==5
    matrix['status']='passed' if passed else 'failed'
(args.output/'matrix.json').write_text(json.dumps(matrix,indent=2))
if args.parallel_tools:
    subprocess.run([sys.executable,str(root/'scripts/audit-parallel-public.py'),str(args.output.resolve())],cwd=root,check=True)
if args.agent_eval:
    subprocess.run([sys.executable,str(root/'scripts/audit-agent-eval.py'),str(args.output.resolve())],cwd=root,check=True)
print(('harness passed; see agent-eval-verdict.json' if args.agent_eval else 'passed') if passed else 'failed');raise SystemExit(0 if passed else 1)

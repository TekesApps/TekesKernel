#!/usr/bin/env python3
"""Run permanent-400 queue recovery through real workers and an external v3 client."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output',type=Path,required=True)
parser.add_argument('--question',action='store_true',help='Answer a real workflow hold through the public client before final recovery')
parser.add_argument('--realtime', action='store_true', help='Gate provider completion on public WebSocket chunk acknowledgement')
args=parser.parse_args()
root=Path(__file__).resolve().parents[1]
args.output.mkdir(parents=True,exist_ok=False)
requests=[]
class Handler(BaseHTTPRequestHandler):
    def log_message(self,*args): pass
    def do_POST(self):
        request=json.loads(self.rfile.read(int(self.headers['Content-Length'])))
        requests.append(request);n=len(requests)
        (args.output/f'request-{n}.json').write_text(json.dumps(request,indent=2))
        if n==1:
            deadline=time.monotonic()+30
            while not (args.output/'runtime/queued-input-observed').exists():
                if time.monotonic()>deadline: raise TimeoutError('second input did not queue before permanent 400')
                time.sleep(.02)
            body=b'{"error":{"message":"model not available"}}'
            self.send_response(400);self.send_header('Content-Type','application/json')
        else:
            assert n in ((2,3,4) if args.question else (2,3)), 'unexpected provider retry/extra request'
            if n==2:
                assert "queued input after this turn's permanent failure" in json.dumps(request)
                arguments={'thought':'recovered after provider failure'}
                output=[{'type':'function_call','status':'completed','call_id':'recovery-think','name':'think','arguments':json.dumps(arguments)}]
            elif args.question and n==3:
                assert any(item.get('type')=='function_call_output' and item.get('call_id')=='recovery-think' for item in request['input'])
                output=[{'type':'function_call','status':'completed','call_id':'recovery-question','name':'ask_user_questions','arguments':json.dumps({'question':'Which option?','options':['A','B']})}]
            else:
                call_id='recovery-question' if args.question else 'recovery-think'
                result=next(item for item in request['input'] if item.get('type')=='function_call_output' and item.get('call_id')==call_id)
                if args.question:
                    content=json.loads(result['output'])
                    assert len(content)==1 and content[0]['type']=='text',content
                    assert json.loads(content[0]['text'])=={'answers':[{'question':'Which option?','answer':'A'}]},content
                output=[{'type':'message','role':'assistant','phase':'final_answer','status':'completed','content':[{'type':'output_text','text':'recovered after provider failure'}]}]
            response={'id':f'recovered-{n}','status':'completed','output':output,'usage':{'input_tokens':10,'output_tokens':10}}
            frames=[]
            if output[0]['type']=='function_call':
                frames += [{'type':'response.output_item.added','output_index':0,'item':dict(output[0],arguments='')},
                           {'type':'response.function_call_arguments.done','output_index':0,'arguments':output[0]['arguments']},
                           {'type':'response.output_item.done','output_index':0,'item':output[0]}]
            if args.realtime and output[0]['type']=='message':
                prefix=''.join('data: '+json.dumps(frame)+'\n\n' for frame in [
                    {'type':'response.output_item.added','output_index':0,'item':dict(output[0],content=[])},
                    {'type':'response.output_text.delta','output_index':0,'content_index':0,'delta':'recovered after provider failure'}]).encode()
                terminal=('data: '+json.dumps({'type':'response.completed','response':response})+'\n\n').encode()
                self.send_response(200);self.send_header('Content-Type','text/event-stream')
                self.send_header('Content-Length',str(len(prefix)+len(terminal)));self.end_headers()
                self.wfile.write(prefix);self.wfile.flush()
                deadline=time.monotonic()+30
                marker=args.output/'runtime/realtime-chunk-observed.json'
                while not marker.exists():
                    if time.monotonic()>deadline: raise TimeoutError('public sink buffered content until completion')
                    time.sleep(.01)
                (args.output/'terminal-release.json').write_text(json.dumps({'ack':json.loads(marker.read_text()),'request':n}))
                self.wfile.write(terminal);self.wfile.flush()
                return
            frames.append({'type':'response.completed','response':response})
            body=''.join('data: '+json.dumps(frame)+'\n\n' for frame in frames).encode()
            self.send_response(200);self.send_header('Content-Type','text/event-stream')
        self.send_header('Content-Length',str(len(body)));self.end_headers();self.wfile.write(body)

server=ThreadingHTTPServer(('127.0.0.1',0),Handler)
threading.Thread(target=server.serve_forever,daemon=True).start()
provider={'id':'fixture','adapter':'responses','dialect':'openai_responses_v1','endpoint_owner':'openai','gateway_translation':'direct','evidence_revision':'openai-2026-08-01','endpoint':f'http://127.0.0.1:{server.server_port}/v1','credential_key':'fixture-key','models':[{'id':'gpt-5','profile':'openai_responses_v1:gpt-5','enabled':True,'context_window_tokens':32000,'compact_trigger_tokens':24000}]}
config=args.output.resolve()/'provider.json';config.write_text(json.dumps(provider,indent=2))
subprocess.run(['cargo','build','-p','tekes-worker','-p','workspace-service','-p','tools','--bins','--locked'],cwd=root,check=True)
frozen=args.output.resolve()/'bin';frozen.mkdir()
for name in ['tekes-worker','tekes-helper','tekes-workspace-service']:shutil.copy2(root/'target/debug'/name,frozen/name)
build=subprocess.run(['cargo','test','-p','tekes-supervisor','--lib','--no-run','--locked','--message-format=json'],cwd=root,check=True,stdout=subprocess.PIPE,text=True)
artifacts=[json.loads(line) for line in build.stdout.splitlines() if line.strip()]
executables={entry['executable'] for entry in artifacts if entry.get('reason')=='compiler-artifact' and entry.get('executable') and entry.get('target',{}).get('name')=='tekes_supervisor'}
assert len(executables)==1
shutil.copy2(next(iter(executables)),frozen/'tekes-supervisor-live-test')
(args.output/'binaries.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in frozen.iterdir()},indent=2))
client_script=args.output.resolve()/'provider-error-queue-client.py';shutil.copy2(root/'scripts'/client_script.name,client_script)
shutil.copy2(__file__,args.output/'runner.py')
env=dict(os.environ,TEKES_KERNEL_LIVE_PROVIDER=str(config),TEKES_KERNEL_LIVE_KEY='disposable-fixture-key',TEKES_TEST_REAL_WORKER=str(frozen/'tekes-worker'),TEKES_PROCESS_ARTIFACT=str(args.output.resolve()/'runtime'),TEKES_PUBLIC_QUESTION='1' if args.question else '0')
with (args.output/'client.log').open('w') as log:
    client=subprocess.Popen([sys.executable,str(client_script),str(args.output.resolve()/'runtime')]+(['--question'] if args.question else [])+(['--realtime'] if args.realtime else []),stdout=log,stderr=subprocess.STDOUT)
    run=subprocess.run([str(frozen/'tekes-supervisor-live-test'),'process_host::tests::real_provider_400_releases_queue_over_public_transport','--exact','--ignored','--nocapture'],cwd=root,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    if run.returncode:client.terminate()
    client_code=client.wait(timeout=30)
server.shutdown()
log=run.stdout.decode(errors='replace');(args.output/'test.log').write_text(log)
passed=run.returncode==client_code==0 and '1 passed' in log and len(requests)==(4 if args.question else 3) and (args.output/'runtime/receipt.json').exists()
passed=passed and (not args.realtime or (args.output/'terminal-release.json').exists())
passed=passed and not any(marker in log for marker in ('process-host-publish-frame-failed:','process-host-publish-appended-failed:','process-host-publish-child-actionables-failed:','process-host-reconcile-failed:'))
(args.output/'matrix.json').write_text(json.dumps({'status':'passed' if passed else 'failed','provider_requests':len(requests),'test_exit':run.returncode,'client_exit':client_code},indent=2))
print('passed' if passed else 'failed')
raise SystemExit(0 if passed else 1)

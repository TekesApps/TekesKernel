#!/usr/bin/env python3
"""Run a real worker process gate with an explicit or configured provider."""
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
parser.add_argument('--scenario',choices=['drama','image','tool-search','mcp-task','queue','validator','repair','task','tasks-markdown','tasks-python','update-code-no-hint','update-files-hint','update-files-no-hint'],default='queue')
providers = parser.add_mutually_exclusive_group()
providers.add_argument('--provider',default=None,help='exact configured provider id')
providers.add_argument('--provider-config',type=Path,help='isolated provider JSON; never modifies installed configuration')
parser.add_argument('--secret-name',help='credential field in the existing live keys file; alternatively set TEKES_KERNEL_LIVE_KEY')
parser.add_argument('--public-approval',action='store_true')
args = parser.parse_args()
if args.public_approval and args.scenario not in ('drama','task','tasks-markdown','tasks-python'):
    parser.error('--public-approval requires an ordinary delegated task scenario')
root = Path(__file__).resolve().parents[1]
args.output.mkdir(parents=True,exist_ok=False)
provider = json.loads(args.provider_config.read_text()) if args.provider_config else next(p for p in json.loads(Path(os.environ['TEKES_PROVIDERS_CONFIG']).read_text())['providers'] if (p['id']==args.provider if args.provider else p['dialect']=='deepseek_responses_v1'))
secrets = dict(re.findall(r'^\s*"([\w]+)"\s*:\s*"([^"\r\n]+)"',Path(os.environ['TEKES_LIVE_KEYS_FILE']).read_text(),re.M))
secret_name = args.secret_name or ('cloudflare' if provider['endpoint_owner']=='cloudflare' else {'deepseek_responses_v1':'deepSeekKey','google_generation_v1':'googleKey'}.get(provider['dialect']))
key = os.environ.get('TEKES_KERNEL_LIVE_KEY') or secrets[secret_name]
config = args.output.resolve()/'provider.json'
config.write_text(json.dumps(provider,indent=2))
subprocess.run(['cargo','build','-p','tekes-worker','-p','workspace-service','-p','tools','-p','mcp','--bins','--locked'],cwd=root,check=True)
frozen = args.output.resolve()/'bin'
frozen.mkdir()
for name in ['tekes-worker','tekes-helper','tekes-workspace-service','mcp-fixture-server']:
    shutil.copy2(root/'target/debug'/name, frozen/name)
build = subprocess.run(['cargo','test','-p','tekes-supervisor','--lib','--no-run','--locked','--message-format=json'],cwd=root,check=True,stdout=subprocess.PIPE,text=True)
artifacts = [json.loads(line) for line in build.stdout.splitlines() if line.strip()]
executables = {entry['executable'] for entry in artifacts if entry.get('reason')=='compiler-artifact' and entry.get('executable') and entry.get('target',{}).get('name')=='tekes_supervisor'}
if len(executables)!=1: raise SystemExit('Expected exactly one supervisor test binary')
shutil.copy2(next(iter(executables)),frozen/'tekes-supervisor-live-test')
(args.output/'binaries.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in frozen.iterdir()},indent=2))
env = dict(os.environ,TEKES_KERNEL_LIVE_PROVIDER=str(config),TEKES_KERNEL_LIVE_KEY=key,TEKES_TEST_REAL_WORKER=str(frozen/'tekes-worker'),TEKES_TEST_MCP_FIXTURE_SERVER=str(frozen/'mcp-fixture-server'),TEKES_PROCESS_ARTIFACT=str(args.output.resolve()/'runtime'))
test_name = 'real_worker_repeated_context_releases_queued_body' if args.scenario=='queue' else ('real_validator_process_promotes_frozen_candidate' if args.scenario=='validator' else 'real_validator_feedback_repairs_same_worker')
if args.scenario in ('drama','task','tasks-markdown','tasks-python','update-code-no-hint','update-files-hint','update-files-no-hint'): test_name='real_legacy_simple_task'
if args.scenario=='image': test_name='real_public_image_attachment'
if args.scenario=='tool-search': test_name='real_public_deferred_tool_search'
if args.scenario=='mcp-task': test_name='real_public_mcp_task_continuation'
env['TEKES_LEGACY_TASK_SCENARIO'] = args.scenario
env['TEKES_PUBLIC_APPROVAL'] = str(int(args.public_approval))
client = None
if args.public_approval or args.scenario in ('tool-search','image'):
    client_name='tool-search-websocket-client.py' if args.scenario=='tool-search' else 'task-approval-websocket-client.py'
    if args.scenario=='image': client_name='image-websocket-client.py'
    client_script=args.output.resolve()/client_name
    shutil.copy2(root/'scripts'/client_name,client_script)
    client_log=(args.output/'client.log').open('w')
    client=subprocess.Popen([__import__('sys').executable,str(client_script),str(args.output.resolve()/'runtime')],stdout=client_log,stderr=subprocess.STDOUT)
run = subprocess.run([str(frozen/'tekes-supervisor-live-test'),'process_host::tests::'+test_name,'--exact','--ignored','--nocapture'],cwd=root,env=env,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
client_code=0
if client:
    if run.returncode: client.terminate()
    client_code=client.wait(timeout=30)
    client_log.close()
log = run.stdout.decode(errors='replace')
for secret in [key,*secrets.values()]:
    if len(secret)>8: log=log.replace(secret,'[REDACTED]')
(args.output/'test.log').write_text(log)
passed = client_code == 0 and run.returncode == 0 and '1 passed' in log and (args.output/'runtime/receipt.json').exists()
passed = passed and not any(marker in log for marker in ('process-host-publish-frame-failed:', 'process-host-publish-appended-failed:', 'process-host-reconcile-failed:', 'process-host-publish-child-actionables-failed:'))
if passed and args.scenario in ('drama', 'task', 'tasks-markdown','tasks-python','update-code-no-hint', 'update-files-hint', 'update-files-no-hint'):
    audit_run = subprocess.run(['python3', str(root/'scripts/audit-live-task-validation.py'), str(args.output.resolve())], cwd=root, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    (args.output/'validation-audit.log').write_bytes(audit_run.stdout)
    passed = audit_run.returncode == 0
if passed and args.scenario in ('tasks-python','update-code-no-hint'):
    functional = subprocess.run(['python3', str(root/'scripts/audit-live-python-task.py'), str(args.output.resolve()), *(['--country','Japan','--capital','Tokyo'] if args.scenario == 'update-code-no-hint' else [])], cwd=root)
    passed = functional.returncode == 0
if passed and args.public_approval:
    public_audit = subprocess.run(['python3', str(root/'scripts/audit-public-task-approvals.py'), str(args.output.resolve())], cwd=root)
    passed = public_audit.returncode == 0
if passed and args.scenario=='drama':
    audit = subprocess.run(['python3',str(root/'scripts/audit-live-drama.py'),str(args.output.resolve())],cwd=root)
    passed = audit.returncode == 0
print('passed' if passed else 'failed')
raise SystemExit(0 if passed else 1)

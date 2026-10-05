#!/usr/bin/env python3
"""Exercise actual Kernel HTTP authentication, dispatcher and workspace children.
No Client checkout is changed. All writes use disposable repositories.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile
import time
import urllib.request
import urllib.error
import uuid

parser = argparse.ArgumentParser()
parser.add_argument('--server', type=Path, required=True)
parser.add_argument('--helper', type=Path, required=True)
parser.add_argument('--report', type=Path, required=True)
parser.add_argument('--swift-wse-probe', type=Path, help='Same compiled Client WSE probe also used against DSH')
args = parser.parse_args()
results = []
responses = []

def git(root, *command):
    return subprocess.run(['git', *command], cwd=root, check=True, capture_output=True, text=True).stdout.strip()

with tempfile.TemporaryDirectory(prefix='kernel-workspace-http-') as directory:
    temporary = Path(directory)
    ready = temporary/'ready'
    env = {**os.environ, 'TEKES_WORKSPACE_SERVICE_BIN': str(args.helper.resolve())}
    with (temporary/'server.log').open('wb') as log:
        server = subprocess.Popen([str(args.server.resolve()), str(ready)], env=env, stdout=log, stderr=log)
        try:
            deadline = time.monotonic()+30
            while not ready.exists():
                if server.poll() is not None:
                    raise RuntimeError('Kernel harness exited before readiness; '+(temporary/'server.log').read_text()[-3000:])
                if time.monotonic()>deadline:
                    raise TimeoutError('Kernel harness did not become ready')
                time.sleep(.05)
            base, token, project, storage = ready.read_text().splitlines()
            project, storage = Path(project).resolve(), Path(storage).resolve()
            def rpc(method, payload, *, rpc_id=None, authenticated=True):
                body = json.dumps({'type':'client-request','rpcId':rpc_id or str(uuid.uuid4()),'method':method,'payload':payload}).encode()
                headers = {'Content-Type':'application/json'}
                if authenticated: headers['Authorization']='Bearer '+token
                req = urllib.request.Request(base+'/api/'+method, data=body, headers=headers, method='POST')
                try:
                    with urllib.request.urlopen(req, timeout=40) as response:
                        return response.status, json.load(response)
                except urllib.error.HTTPError as response:
                    return response.code, json.load(response)
            def success(method, payload, **options):
                status, response=rpc(method,payload,**options)
                assert status==200 and response['result']['ok'], (method,status,response)
                results.append({'method':method,'outcome':'success'})
                if method.startswith('tekesWorkspace.'):
                    responses.append({'method':method,'value':response['result']['value']})
                return response['result']['value']
            def workspace(method, **payload):
                return success('tekesWorkspace.'+method,{'workspaceId':workspace_id,**payload})
            code, _=rpc('tekesWorkspace.capabilities',{},authenticated=False)
            assert code==401,code
            results.append({'method':'unauthenticated','outcome':'rejected'})
            capabilities=success('tekesWorkspace.capabilities',{})
            assert capabilities['mutations'] is False
            created=success('workspace.create',{'path':str(project)})
            workspace_id=created['workspace']['workspaceId']
            (project/'selected.txt').write_text('alpha\nbeta\n')
            (project/'unrelated.txt').write_text('original\n')
            git(project,'init','-b','main')
            git(project,'config','user.name','Kernel Acceptance')
            git(project,'config','user.email','kernel-test@example.invalid')
            git(project,'add','.')
            git(project,'commit','-m','initial')
            assert any(e['name']=='selected.txt' for e in workspace('filesList')['entries'])
            assert workspace('filesRead',path='selected.txt')['content']=='YWxwaGEKYmV0YQo='
            assert len(workspace('filesSearch',query='selected')['entries'])==1
            assert workspace('gitRepository')['isRepository']
            assert workspace('gitStatus')['branch']=='main'
            assert workspace('gitBranches')['defaultBranch']=='main'
            assert workspace('gitLog')['commits'][0]['subject']=='initial'
            assert len(workspace('gitChangesNavigator',includeClean=True)['repositories'])==1
            (project/'selected.txt').write_text('alpha\ngamma\ndelta\n')
            assert '+gamma' in workspace('gitDiff',paths=['selected.txt'])['unifiedDiff']
            mutation={'workspaceId':workspace_id,'operation':'create','name':'feature'}
            code,response=rpc('tekesWorkspace.gitChangeBranch',mutation)
            assert code==200 and not response['result']['ok'] and response['result']['error']['code']=='permission-denied',response
            results.append({'method':'default Git write','outcome':'rejected'})
            policy=storage/'workspace-service/git-authority.json'
            policy.parent.mkdir(parents=True,exist_ok=True)
            policy.write_text(json.dumps({'allowedOperations':['commit','branch.create','branch.switch','push'],'allowedRepositoryRoots':[str(project)],'allowFileWrites':True}))
            (project/'unrelated.txt').write_text('keep staged\n')
            git(project,'add','unrelated.txt')
            payload={'workspaceId':workspace_id,'message':'selected only','paths':['selected.txt']}
            first=success('tekesWorkspace.gitCommit',payload,rpc_id='exact-commit')
            replay=success('tekesWorkspace.gitCommit',payload,rpc_id='exact-commit')
            assert first==replay
            assert git(project,'rev-list','--count','HEAD')=='2'
            assert git(project,'show','HEAD:unrelated.txt')=='original'
            assert git(project,'show',':unrelated.txt')=='keep staged'
            code, conflict=rpc('tekesWorkspace.gitCommit',{**payload,'message':'different'},rpc_id='exact-commit')
            assert code==200 and not conflict['result']['ok'],conflict
            results.append({'method':'RPC id conflict','outcome':'rejected'})
            success('tekesWorkspace.gitChangeBranch',mutation)
            remote=temporary/'remote.git';remote.mkdir();git(remote,'init','--bare')
            git(project,'remote','add','origin',str(remote))
            pushed=workspace('gitPush',remote='origin')
            assert git(remote,'rev-parse','refs/heads/feature')==first['commit']==pushed['head']
            workspace('gitChangeBranch',operation='switch',name='main')
            session_id=str(uuid.uuid4())
            session=success('session.create',{'workspaceId':workspace_id,'sessionId':session_id})
            assert session['sessionId']==session_id
            # Fixture fact through the internal child protocol; separate Worker tests
            # establish that real tool executions author the same record shape.
            authority=temporary/'capture.json'
            authority.write_text(json.dumps({'stateRoot':str(storage/'workspace-service'),'allowedOperations':[],'allowedRepositoryRoots':[]}))
            event={'workspaceId':workspace_id,'sessionId':session_id,'turnId':'1','edit':{'eventId':'fixture-edit','path':str(project/'selected.txt'),'before':'alpha\nbeta\n','after':'alpha\ngamma\ndelta\n'}}
            output=subprocess.run([str(args.helper.resolve()),'--root',str(project),'--authority-file',str(authority)],input=json.dumps({'method':'recordTurnEdit','request':event}),text=True,capture_output=True,check=True)
            captured=json.loads(output.stdout)
            assert captured.get('result',{}).get('additions')==2,captured
            snapshot=workspace('turnChanges',sessionId=session_id,turnId='1')
            assert snapshot['additions']==2 and snapshot['deletions']==1
            code,rejected=rpc('tekesWorkspace.turnChanges',{'workspaceId':workspace_id,'sessionId':str(uuid.uuid4()),'turnId':'1'})
            assert code==200 and not rejected['result']['ok'],rejected
            results.append({'method':'foreign session','outcome':'rejected'})
            if args.swift_wse_probe:
                (project/'wse-proof.txt').write_text('shared WSE\n')
                fixture=temporary/'swift-wse.json'
                fixture.write_text(json.dumps({'baseURL':base,'headers':{'Authorization':'Bearer '+token},
                    'workspaceId':workspace_id,'sessionId':session_id,'turnId':'1'}))
                fixture.chmod(0o600)
                subprocess.run([str(args.swift_wse_probe.resolve()),str(fixture)],check=True)
                results.append({'method':'shared production Swift WSE','outcome':'passed'})
        finally:
            server.terminate()
            try: server.wait(timeout=10)
            except subprocess.TimeoutExpired:
                server.kill();server.wait(timeout=5)
args.report.parent.mkdir(parents=True,exist_ok=True)
args.report.with_suffix('.responses.json').write_text(json.dumps(responses,indent=2)+'\n')
args.report.write_text(json.dumps({'scope':'Kernel production HTTP carrier + dispatcher + workspace helper','passed':True,'checks':results,'limits':['fixture turn fact; real Worker pipeline tested separately','no signed installation or Client UI acceptance']},indent=2)+'\n')
print(f'Passed {len(results)} authenticated-carrier checks; report: {args.report}')

import os,json,tempfile,subprocess,select,secrets,urllib.request,base64,socket,struct,sys
from pathlib import Path
from urllib.parse import urlparse
repo=Path(__file__).resolve().parent.parent
with tempfile.TemporaryDirectory(prefix='tekes-builtin-') as temporary:
    base=Path(temporary).resolve()
    root=str(base/'state')
    workspace_path=base/'project';workspace_path.mkdir()
    launch=base/'launch.json'
    providers=json.loads((repo/'fixtures/config/providers.canonical.json').read_text())
    # The provider is always the local mock: the native client check sends a real prompt, and a
    # worker stuck on the internet would keep the session busy for the later selectModel step.
    # TEKES_TEST_IMAGE_PROMPT only enables the extra image/command steps.
    import http.server,threading
    provider_requests=[]
    class MockProvider(http.server.BaseHTTPRequestHandler):
        def do_POST(self):
            provider_requests.append(json.loads(self.rfile.read(int(self.headers['Content-Length']))))
            event={'type':'response.completed','response':{'id':'fixture-response','status':'completed',
                'output':[{'type':'message','role':'assistant','phase':'final_answer','status':'completed',
                'content':[{'type':'output_text','text':'KERNEL_IMAGE_RESPONSE_OK'}]}],
                'usage':{'input_tokens':10,'output_tokens':4,'total_tokens':14}}}
            self.send_response(200);self.send_header('Content-Type','text/event-stream');self.end_headers()
            self.wfile.write(('data: '+json.dumps(event)+'\n\n').encode())
        def log_message(self,*args): pass
    mock_provider=http.server.ThreadingHTTPServer(('127.0.0.1',0),MockProvider)
    threading.Thread(target=mock_provider.serve_forever,daemon=True).start()
    providers['providers'][0]['endpoint']=f'http://127.0.0.1:{mock_provider.server_port}/v1'
    launch.write_text(json.dumps(dict(format=1,bootstrap_version=1,root=root,worker=str(repo/'target/debug/tekes-worker'),listen='127.0.0.1:0',providers=providers,credential_bindings={'openai-main':'TEKES_BUILTIN_TEST_KEY'})))
    token=secrets.token_bytes(32)
    env=os.environ.copy();env['HOME']=temporary
    env.pop('TEKES_KERNEL_ENDPOINT_TOKEN',None);env.pop('TEKES_BUILTIN_TEST_KEY',None)
    visible_marker='synthetic-bootstrap-control-'+secrets.token_hex(16)
    env['TEKES_BOOTSTRAP_TEST_VISIBLE_MARKER']=visible_marker
    material='synthetic-builtin-test-key'
    payload=json.dumps({'endpointToken':token.hex(),'credentials':{'openai-main':material}}).encode()
    bootstrap=b'TEKES-KERNEL-BOOTSTRAP/1\n'+struct.pack('!I',len(payload))+payload
    command=[str(repo/'target/debug/tekes-supervisor'),'--built-in',str(launch)]
    for legacy in ['TEKES_KERNEL_ENDPOINT_TOKEN','TEKES_BUILTIN_TEST_KEY']:
        legacy_env=env.copy();legacy_env[legacy]='synthetic-legacy-not-echoed'
        failed=subprocess.run(command,input=bootstrap,capture_output=True,env=legacy_env,timeout=10)
        assert failed.returncode!=0 and not failed.stdout,(legacy,failed.returncode)
        assert b'legacy-secret-environment-rejected' in failed.stderr
        assert b'synthetic-legacy-not-echoed' not in failed.stderr
    config=json.loads(launch.read_text());config['web_listen']='127.0.0.1:0'
    launch.write_text(json.dumps(config))
    failed=subprocess.run(command,input=bootstrap,capture_output=True,env=env,timeout=10)
    assert failed.returncode!=0 and b'unauthenticated-web-client-disabled' in failed.stderr
    assert not (Path(root)/'config').exists(),'failed launch produced config side effects'
    config.pop('web_listen');launch.write_text(json.dumps(config))
    for invalid in [b'',bootstrap[:30],b'TEKES-KERNEL-BOOTSTRAP/1\n'+struct.pack('!I',1024*1024+1)]:
        failed=subprocess.run(command,input=invalid,capture_output=True,env=env,timeout=10)
        assert failed.returncode!=0 and not failed.stdout
        assert b'private-bootstrap-rejected' in failed.stderr
        assert material.encode() not in failed.stderr
        assert not (Path(root)/'config').exists(),'invalid frame produced config side effects'
    print('PRIVATE_BOOTSTRAP_AND_LEGACY_WEB_REJECTIONS_OK')
    if os.environ.get('TEKES_FILE_TRANSFER_WORKER'):
        import hashlib
        attachment_session='018f0000-0000-7000-8000-000000000061'
        image=base64.b64decode('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=')
        digest=hashlib.sha256(image).hexdigest()
        folder=Path(root)/'threads'/attachment_session
        (folder/'assets').mkdir(parents=True)
        for directory in [folder.parent,folder,folder/'assets']: directory.chmod(0o700)
        (folder/'assets'/('sha256-'+digest)).write_bytes(image)
        origin={'client':'client','key':'create','op':'create','principal':'principal','target':attachment_session}
        events=[{'config':{'digest':'cfg-1'},'format':1,'kind':'genesis','min_reader':1,'min_writer':1,
            'origin_key':'create','origin_tuple':origin,'resume':'never','seq':1,'thread':attachment_session,
            'ts':'2026-08-26T09:00:00.000Z','v':1,'workspace':'workspace'},
            {'content':[{'type':'image','asset':'sha256-'+digest,'mime':'image/png'}],
             'kind':'input','origin_key':'prompt','origin_tuple':dict(origin,key='prompt',op='submit'),
             'seq':2,'ts':'2026-08-26T09:00:01.000Z','v':1}]
        (folder/'main.jsonl').write_text(''.join(json.dumps(e,sort_keys=True,separators=(',',':'))+'\n' for e in events))
    p=subprocess.Popen(command,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=env)
    p.stdin.write(bootstrap);p.stdin.flush()
    try:
        if not select.select([p.stdout],[],[],20)[0]: raise RuntimeError('readiness timeout')
        line=p.stdout.readline()
        if not line: raise RuntimeError(p.stderr.read())
        ready=json.loads(line);print('READY',ready)
        if sys.platform=='darwin':
            # CTL_KERN/KERN_PROCARGS2 reads the initial environment, even
            # under the current macOS tool sandbox. Never print its contents.
            import ctypes
            libc=ctypes.CDLL(None,use_errno=True)
            mib=(ctypes.c_int*3)(1,49,p.pid)
            size=ctypes.c_size_t(1024*1024)
            initial=ctypes.create_string_buffer(size.value)
            try:
                assert libc.sysctl(mib,3,initial,ctypes.byref(size),None,0)==0,'initial process query failed'
                initial_bytes=initial.raw[:size.value]
                assert visible_marker.encode() in initial_bytes,'initial environment positive control missing'
                assert token.hex().encode() not in initial_bytes,'endpoint token exposed in initial process data'
                assert material.encode() not in initial_bytes,'provider key exposed in initial process data'
                del initial_bytes
            finally:
                ctypes.memset(initial,0,ctypes.sizeof(initial))
            print('INITIAL_PROCESS_SECRET_ABSENCE_OK')
        duplicate=subprocess.run(command,input=bootstrap,capture_output=True,env=env,timeout=10)
        assert duplicate.returncode!=0 and not duplicate.stdout,'duplicate instance accepted'
        print('DUPLICATE_INSTANCE_REJECTED')
        req=urllib.request.Request(ready['url']+'/health/ready',headers={'Authorization':'Bearer '+base64.urlsafe_b64encode(token).decode().rstrip('=')})
        try:
            with urllib.request.urlopen(req,timeout=5) as response: assert response.status in (200,204); print('HEALTH_READY',response.status)
        except urllib.error.HTTPError as error: raise AssertionError(f'health failed: {error.code}') from None
        assert 'webUrl' not in ready and ready['bootstrapVersion']==1,ready
        def call(method,payload):
            rpc=secrets.token_hex(8)
            body=json.dumps({'type':'client-request','rpcId':rpc,'method':method,'payload':payload}).encode()
            request=urllib.request.Request(ready['url']+'/api/'+method,data=body,headers={'Authorization':'Bearer '+base64.urlsafe_b64encode(token).decode().rstrip('='),'Content-Type':'application/json'})
            with urllib.request.urlopen(request,timeout=10) as response: result=json.load(response)
            assert result['rpcId']==rpc,result
            assert result['result']['ok'],result
            return result['result']['value']
        rpc=secrets.token_hex(8)
        body=json.dumps({'type':'client-request','rpcId':rpc,'method':'providers.connection.save','payload':{}}).encode()
        request=urllib.request.Request(ready['url']+'/api/providers.connection.save',data=body,
            headers={'Authorization':'Bearer '+base64.urlsafe_b64encode(token).decode().rstrip('='),'Content-Type':'application/json'})
        try:
            with urllib.request.urlopen(request,timeout=5) as response: rejected=json.load(response)
        except urllib.error.HTTPError as error:
            rejected=json.load(error)
        assert rejected.get('error',{}).get('code') == 'not-found',rejected
        print('PROVIDER_EDIT_REJECTED')
        draft_catalog=call('models.list',{})
        assert draft_catalog['groups'] and draft_catalog['routable'],draft_catalog
        print('DRAFT_MODELS_ROUTABLE')
        workspace=call('workspace.create',{'path':str(workspace_path)})['workspace']
        if os.environ.get('TEKES_FILE_TRANSFER_WORKER'):
            from urllib.parse import urlparse, unquote
            import hashlib
            content=bytes(range(256))*1000
            (workspace_path/'binary-preview.dat').write_bytes(content)
            cache=base/'preview-cache';cache.mkdir()
            transfer={'workspace':{'_0':{'baseURL':ready['url'],
                'headers':{'Authorization':'Bearer '+base64.urlsafe_b64encode(token).decode().rstrip('=')},
                'identity':{'workspace':{'workspaceID':workspace['workspaceId'],'path':'binary-preview.dat'}},
                'cacheDirectory':cache.as_uri()+'/', 'maximumBytes':1024*1024*1024,'timeout':15}}}
            result=subprocess.run([os.environ['TEKES_FILE_TRANSFER_WORKER']],input=json.dumps(transfer),capture_output=True,text=True,timeout=20)
            assert result.returncode==0, result.stdout
            receipt=json.loads(result.stdout)['receipt']
            assert receipt['size']==len(content),receipt
            assert receipt['revision']==hashlib.sha256(content).hexdigest(),receipt
            assert Path(unquote(urlparse(receipt['fileURL']).path)).read_bytes()==content
            print('SWIFT_FILE_TRANSFER_BINARY_ROUNDTRIP_OK')
            transfer['workspace']['_0']['identity']={'sessionAttachment':{'sessionID':attachment_session,'attachmentID':'sha256:'+digest}}
            result=subprocess.run([os.environ['TEKES_FILE_TRANSFER_WORKER']],input=json.dumps(transfer),capture_output=True,text=True,timeout=20)
            assert result.returncode==0,result.stdout
            receipt=json.loads(result.stdout)['receipt']
            assert receipt['attachment']['attachmentId']=='sha256:'+digest,receipt
            assert receipt['attachment']['width']==1 and receipt['attachment']['height']==1,receipt
            assert receipt['revision']==digest,receipt
            assert Path(unquote(urlparse(receipt['fileURL']).path)).read_bytes()==image
            print('SWIFT_ATTACHMENT_TRANSFER_ROUNDTRIP_OK')
            call('workspace.archiveSession',{'sessionId':attachment_session})
        session=call('session.create',{'workspaceId':workspace['workspaceId']})['sessionId']
        print('SESSION_CREATED',session)
        (workspace_path/'file-page.txt').write_text('first\nsecond\n')
        file_stat=call('session.files.stat',{'sessionId':session,'path':'file-page.txt'})
        text_page=call('session.files.read',{'sessionId':session,'path':'file-page.txt','offset':2,'limit':1})
        byte_page=call('session.files.readBytes',{'sessionId':session,'path':'file-page.txt','offset':6,'limit':6})
        assert file_stat['bytes']==13,file_stat
        assert text_page['text']=='second' and text_page['lines']==1 and text_page['eof'],text_page
        assert base64.b64decode(byte_page['data'])==b'second' and not byte_page['eof'],byte_page
        assert file_stat['version']==text_page['version']==byte_page['version']
        print('SESSION_FILE_PAGES_REAL_PROCESS_OK')
        if os.environ.get('TEKES_TEST_IMAGE_PROMPT'):
            import time,hashlib
            image_data='iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII='
            receipt=call('session.prompt',{'sessionId':session,'mode':'queue','content':[{'type':'image','mediaType':'image/png','data':image_data}]})
            assert receipt['accepted'],receipt
            image_id='sha256:'+hashlib.sha256(base64.b64decode(image_data)).hexdigest()
            attachment=call('session.attachment',{'sessionId':session,'attachmentId':image_id})
            assert attachment['data']==image_data,attachment
            deadline=time.monotonic()+15
            while not provider_requests and time.monotonic()<deadline: time.sleep(.1)
            assert provider_requests,'Worker did not reach local provider'
            assert 'data:image/png;base64,'+image_data in json.dumps(provider_requests[0]),'Provider request omitted image'
            print('IMAGE_PROMPT_TO_LOCAL_PROVIDER_OK')
            deadline=time.monotonic()+15
            while time.monotonic()<deadline:
                events=[json.loads(line) for line in (Path(root)/'threads'/session/'main.jsonl').read_text().splitlines()]
                if any(e.get('kind')=='output' and 'KERNEL_IMAGE_RESPONSE_OK' in json.dumps(e) for e in events): break
                time.sleep(.1)
            else: raise AssertionError('Provider answer was not committed to the semantic ledger')
            print('IMAGE_PROVIDER_ANSWER_COMMITTED_OK')
        project=workspace_path/'resource-project'
        skill=project/'.agents/skills/project-only/SKILL.md'
        skill.parent.mkdir(parents=True)
        skill.write_text('---\nname: project-only\ndescription: Project scoped skill\n---\nProject rules.\n')
        command_file=project/'.agents/commands/project-only.md'
        command_file.parent.mkdir(parents=True)
        command_file.write_text('---\ndescription: Project scoped command\n---\nReview $ARGUMENTS\n')
        resource_workspace=call('workspace.create',{'path':str(project)})['workspace']
        resource_session=call('session.create',{'workspaceId':resource_workspace['workspaceId']})['sessionId']
        assert call('skills/list',{'sessionId':resource_session})['skills']==[],'untrusted project skill enabled'
        trust=call('folders.trust.request',{'path':str(project),'expectedRevision':0})
        import pty,time
        child,terminal=pty.fork()
        if child==0:
            os.execve(command[0],[command[0],'--folder-trust','grant',root,trust['requestId'],'smoke-grant','1'],env)
        output=b'';confirmed=False;terminal_open=True;deadline=time.monotonic()+10
        try:
            while time.monotonic()<deadline:
                if terminal_open and select.select([terminal],[],[],.1)[0]:
                    try: chunk=os.read(terminal,8192)
                    except OSError: chunk=b''
                    if not chunk: terminal_open=False
                    output+=chunk
                    if b'to continue:' in output and not confirmed:
                        os.write(terminal,('approve '+json.dumps(trust['requestId'])+'\n').encode());confirmed=True
                if not terminal_open: time.sleep(.01)
                done,status=os.waitpid(child,os.WNOHANG)
                if done: child=0;assert os.waitstatus_to_exitcode(status)==0,output;break
            if child:
                done,status=os.waitpid(child,os.WNOHANG)
                assert done and os.waitstatus_to_exitcode(status)==0,output
                child=0
            assert confirmed,'CLI did not request explicit consent'
        finally:
            os.close(terminal)
            if child:
                try: os.kill(child,9)
                except ProcessLookupError: pass
                os.waitpid(child,0)
        assert call('folders.trust.status',{'path':str(project)})['folder']['trusted']
        print('HOST_FOLDER_TRUST_AND_DEFAULT_DENY_OK')
        skills=call('skills/list',{'sessionId':resource_session})['skills']
        assert skills == [{'name':'project-only','description':'Project scoped skill','modelInvocable':True}],skills
        commands=call('commands/list',{'sessionId':resource_session})['commands']
        assert [item['name'] for item in commands] == ['project-only'],commands
        assert call('skills/list',{'sessionId':session})['skills'] == []
        assert call('commands/list',{'sessionId':session})['commands'] == []
        print('SESSION_RESOURCES_ISOLATED')
        if os.environ.get('TEKES_TEST_IMAGE_PROMPT'):
            command_key=secrets.token_hex(16)
            command_receipt=call('commands/run',{'session_id':resource_session,'name':'project-only','arguments':'command-fixture','key':command_key})
            assert command_receipt['commandId']==command_key,command_receipt
            retry=call('commands/run',{'session_id':resource_session,'name':'project-only','arguments':'command-fixture','key':command_key})
            assert retry['commandId']==command_key,retry
            deadline=time.monotonic()+15
            while time.monotonic()<deadline:
                if any('Review command-fixture' in json.dumps(request) for request in provider_requests): break
                time.sleep(.1)
            else: raise AssertionError('Expanded project command did not reach provider')
            print('PROJECT_COMMAND_EXECUTION_TO_PROVIDER_OK')
            records=[json.loads(line) for line in (Path(root)/'threads'/resource_session/'main.jsonl').read_text().splitlines()]
            command_inputs=[e for e in records if e.get('kind')=='input' and e.get('origin_tuple',{}).get('key')==command_key]
            assert len(command_inputs)==1, 'Repeated command produced duplicate durable input'
            print('COMMAND_RETRY_SINGLE_INPUT_OK')
        initial_catalog=call('session.models',{'sessionId':session})
        assert initial_catalog['routable'],initial_catalog
        print('DEFAULT_MODEL_ROUTABLE')
        call('session.selectModel',{'sessionId':session,'provider':'openai','model':'gpt-5'})
        catalog=call('session.models',{'sessionId':session})
        assert catalog['routable'] and catalog['groups'],catalog;print('MODEL_CATALOG_ROUTABLE')
        # A secondary folder must never inherit the workspace's primary cwd
        # in either the grouped list or the actual native inventory stream.
        first=base/'group-first';first.mkdir()
        second=base/'group-second';second.mkdir()
        grouped_workspace=call('workspace.create',{'path':str(first)})['workspace']['workspaceId']
        call('workspace.addFolder',{'workspaceId':grouped_workspace,'path':str(second)})
        grouped_session=call('session.create',{'workspaceId':grouped_workspace,'cwd':str(second)})['sessionId']
        grouped_ledger=Path(root)/'threads'/grouped_session/'main.jsonl'
        original_ledger=grouped_ledger.read_bytes()
        assert json.loads(original_ledger.splitlines()[0])['cwd']==str(second)
        grouped=call('folders.list',{})
        assert next(item for item in grouped['items'] if item['path']==str(second))['sessionIds']==[grouped_session],grouped
        moved=base/'group-moved';second.rename(moved)
        assert call('folders.relocate',{'previousPath':str(second),'path':str(moved)})=={'relocated':True}
        grouped=call('folders.list',{})
        assert next(item for item in grouped['items'] if item['path']==str(moved))['sessionIds']==[grouped_session],grouped
        assert not any(item['path']==str(second) for item in grouped['items']),grouped
        assert grouped_ledger.read_bytes()==original_ledger
        assert not call('folders.trust.status',{'path':str(moved)})['folder']['trusted']
        print('FOLDER_GROUPING_RELOCATION_AND_LEDGER_PRESERVATION_OK')
        import socket,struct
        from urllib.parse import urlparse
        url=urlparse(ready['url']); sock=socket.create_connection((url.hostname,url.port),timeout=5)
        key=base64.b64encode(secrets.token_bytes(16)).decode()
        auth=base64.urlsafe_b64encode(token).decode().rstrip('=')
        sock.sendall((f'GET /api/remote.mux HTTP/1.1\r\nHost: {url.netloc}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: {key}\r\nAuthorization: Bearer {auth}\r\n\r\n').encode())
        stream=sock.makefile('rb');status=stream.readline();assert b' 101 ' in status,status
        while stream.readline()!=b'\r\n':pass
        def receive():
            header=stream.read(2);assert len(header)==2
            length=header[1]&127
            if length==126:length=struct.unpack('!H',stream.read(2))[0]
            elif length==127:length=struct.unpack('!Q',stream.read(8))[0]
            return json.loads(stream.read(length))
        frame=receive();assert frame['type']=='ready' and frame['host']['protocolVersion']==3,frame
        print('MUX_READY',frame['host']['product'])
        payload=json.dumps({'type':'open','streamId':'workspace-smoke','target':{'kind':'workspace'}}).encode()
        mask=secrets.token_bytes(4);sock.sendall(bytes([129,128|len(payload)])+mask+bytes(b^mask[i%4] for i,b in enumerate(payload)))
        frame=receive();assert frame['frame']['type']=='baseline' and isinstance(frame['frame']['generation'],str),frame;print('WORKSPACE_BASELINE_OK')
        captured=[{'kind':'workspace','frame':frame['frame']}]
        for kind,target in [('inventory',{'kind':'session-inventory'}),('control',{'kind':'session-control'}),('actionables',{'kind':'actionables'}),('journal',{'kind':'session-journal','address':{'sessionId':session},'maxMessages':7})]:
            payload=json.dumps({'type':'open','streamId':kind,'target':target}).encode()
            mask=secrets.token_bytes(4)
            header=bytes([129,128|len(payload)]) if len(payload)<126 else bytes([129,254])+struct.pack('!H',len(payload))
            sock.sendall(header+mask+bytes(b^mask[i%4] for i,b in enumerate(payload)))
            while True:
                value=receive()
                if value.get('streamId')==kind:break
                other=value.get('streamId')
                assert other in ('workspace-smoke','inventory','control','actionables','journal'),value
                captured.append({'kind':'workspace' if other=='workspace-smoke' else other,'frame':value['frame']})
            captured.append({'kind':kind,'frame':value['frame']})
            if kind=='inventory':
                row=next(row for row in value['frame']['snapshot']['items'] if row['sessionId']==grouped_session)
                assert row['cwd']==str(moved),row
                print('FOLDER_INVENTORY_RELOCATED_CWD_OK')
                current=moved
                for index,method in enumerate(('folders.relocate','workspace.relocate')):
                    destination=base/f'group-moved-{index}';current.rename(destination)
                    relocation={'previousPath':str(current),'path':str(destination)}
                    if method=='workspace.relocate':relocation['workspaceId']=grouped_workspace
                    result=call(method,relocation)
                    if method=='folders.relocate':assert result=={'relocated':True},result
                    else:assert result['workspace']['workspaceId']==grouped_workspace,result
                    sock.settimeout(10)
                    try:
                        while True:
                            delta=receive()
                            other=delta['streamId']
                            captured.append({'kind':'workspace' if other=='workspace-smoke' else other,'frame':delta['frame']})
                            row=delta['frame'].get('session',{})
                            if other=='inventory' and row.get('sessionId')==grouped_session and row['cwd']==str(destination):
                                break
                    finally:
                        sock.settimeout(None)
                    assert grouped_ledger.read_bytes()==original_ledger
                    current=destination
                print('FOLDER_RELOCATION_LIVE_INVENTORY_UPSERT_OK')
            if kind=='journal':
                assert value['frame']['snapshot']['windowLimit']==7,value
                # N1: the context projections newer than the frozen cut follow the snapshot as
                # projection frames at their true clock sequence (the snapshot label is clamped
                # to the cut, which a client holding the newer control baseline does not take).
                snapshot=value['frame']['snapshot'];cut=snapshot['throughSequence'];label=snapshot['projections']['asOfSeq']
                sock.settimeout(1.0);replayed=[]
                try:
                    while True:
                        more=receive()
                        if more.get('streamId')!='journal':continue
                        captured.append({'kind':'journal','frame':more['frame']});replayed.append(more['frame'])
                except (socket.timeout,TimeoutError):pass
                sock.settimeout(None)
                assert [f['type'] for f in replayed]==['projection']*2,replayed
                assert [f['frame']['key'] for f in replayed]==['contextDetails','contextUsage'],replayed
                for f in replayed:
                    assert f['frame']['seq']>cut and f['frame']['seq']>label,(f,cut,label)
                    assert f['frame']['value']==snapshot['projections']['values'][f['frame']['key']],f
                print('JOURNAL_CONTEXT_PROJECTION_REPLAY_OK',cut,label,[f['frame']['seq'] for f in replayed])
            if kind=='journal' and os.environ.get('TEKES_TEST_IMAGE_PROMPT'):
                assert 'KERNEL_IMAGE_RESPONSE_OK' in json.dumps(value['frame']), 'Committed answer missing from journal baseline'
                print('IMAGE_ANSWER_CLIENT_BASELINE_OK')
        if os.environ.get('TEKES_BUILTIN_CLIENT_FRAMES'):
            Path(os.environ['TEKES_BUILTIN_CLIENT_FRAMES']).write_text(json.dumps(captured))
        print('ALL_STREAM_BASELINES_OK')
        stream.close();sock.close()
        if os.environ.get('TEKES_NATIVE_CLIENT_EXEC'):
            subprocess.run([os.environ['TEKES_NATIVE_CLIENT_EXEC'],ready['url'],str(workspace_path)],input=token.hex().encode(),env=env,check=True,timeout=20)
        p.stdin.close();p.wait(timeout=15)
        print('EXIT',p.returncode)
        if p.returncode: raise RuntimeError(p.stderr.read())
    finally:
        if p.poll() is None: p.kill();p.wait()
        mock_provider.shutdown();mock_provider.server_close()

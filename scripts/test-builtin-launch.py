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
    tool_state={"issued":False,"mcp":False}
    class MockProvider(http.server.BaseHTTPRequestHandler):
        def do_POST(self):
            provider_requests.append(json.loads(self.rfile.read(int(self.headers['Content-Length']))))
            event={'type':'response.completed','response':{'id':'fixture-response','status':'completed',
                'output':[{'type':'message','role':'assistant','phase':'final_answer','status':'completed',
                'content':[{'type':'output_text','text':'KERNEL_IMAGE_RESPONSE_OK'}]}],
                'usage':{'input_tokens':10,'output_tokens':4,'total_tokens':14}}}
            if 'Read file-page.txt' in json.dumps(provider_requests[-1]) and not tool_state['issued']:
                tool_state['issued']=True
                event['response']['output']=[{'type':'function_call','id':'fc-read','call_id':'call-read',
                    'name':'read','arguments':json.dumps({'path':'file-page.txt','offset':1,'limit':10})}]
            if 'Call host MCP echo' in json.dumps(provider_requests[-1]) and not tool_state['mcp']:
                tool_state['mcp']=True
                event['response']['output']=[{'type':'function_call','id':'fc-mcp','call_id':'call-mcp',
                    'name':'mcp__fixture__echo','arguments':json.dumps({'value':'HOST_STDIO_MCP_OK'})}]
            self.send_response(200);self.send_header('Content-Type','text/event-stream');self.end_headers()
            if event['response']['output'][0]['type']=='function_call':
                item=event['response']['output'][0]
                added={'type':'response.output_item.added','output_index':0,'item':dict(item,arguments='')}
                done={'type':'response.function_call_arguments.done','output_index':0,'arguments':item['arguments']}
                for frame in (added,done):self.wfile.write(('data: '+json.dumps(frame)+'\n\n').encode())
            self.wfile.write(('data: '+json.dumps(event)+'\n\n').encode())
        def log_message(self,*args): pass
    mock_provider=http.server.ThreadingHTTPServer(('127.0.0.1',0),MockProvider)
    threading.Thread(target=mock_provider.serve_forever,daemon=True).start()
    providers['providers'][0]['endpoint']=f'http://127.0.0.1:{mock_provider.server_port}/v1'
    launch.write_text(json.dumps(dict(format=1,bootstrap_version=1,root=root,worker=str(repo/'target/debug/tekes-worker'),listen='127.0.0.1:0',providers=providers,credential_bindings={'openai-main':'TEKES_BUILTIN_TEST_KEY'})))
    launch.chmod(0o600)
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
    if os.environ.get('TEKES_TEST_CLI_HOST'):
        server=base/'mcp.py'
        server.write_text('''import json,sys,os
with open(sys.argv[1],'a') as log:log.write(str(os.getpid())+'\\n')
for line in sys.stdin:
 r=json.loads(line)
 if 'id' not in r:continue
 method=r['method']
 if method=='initialize':result={'protocolVersion':'2025-11-25','capabilities':{'tools':{}},'serverInfo':{'name':'host-fixture','version':'1'}}
 elif method=='tools/list':result={'tools':[{'name':'echo','description':'Echo','inputSchema':{'type':'object','properties':{'value':{'type':'string'}},'required':['value'],'additionalProperties':False},'annotations':{'readOnlyHint':True}}]}
 elif method=='tools/call':result={'content':[{'type':'text','text':r['params']['arguments']['value']}],'isError':False}
 else:result={}
 print(json.dumps({'jsonrpc':'2.0','id':r['id'],'result':result}),flush=True)
''')
        config['local_mcp']={'fixture':{'command':[sys.executable,'-u',str(server),str(base/'mcp-pids')]}}
        launch.write_text(json.dumps(config))
        command=[sys.executable,str(repo/'host/kernel_cli.py'),'--launch',str(launch),'--supervisor',str(repo/'target/debug/tekes-supervisor')]
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
        import time
        (workspace_path/'file-page.txt').write_text('SESSION_EXECUTION_READ_OK\n')
        # Poison all retired configuration before create and again before restart.
        retired={'config/environments.json':b'corrupt environment','config/folders.json':b'corrupt catalog',
            'workspaces/retired/workspace.json':b'corrupt workspace'}
        for relative,content in retired.items():
            path=Path(root)/relative;path.parent.mkdir(parents=True,exist_ok=True);path.write_bytes(content)
        execution={'cwd':str(workspace_path),'network':False,'allowed_tools':['read']}
        if os.environ.get('TEKES_TEST_CLI_HOST'):
            execution['mcp']=[{'name':'fixture','transport':{'type':'stdio','channel':'fixture'}}]
            execution['allowed_tools'].append('mcp__fixture__echo')
            execution['allowed_tools'].sort()
        session=call('session.create',{'execution':execution,'identityProfile':'general'})['sessionId']
        ledger=Path(root)/'threads'/session/'main.jsonl'
        genesis=json.loads(ledger.read_text().splitlines()[0])
        assert genesis['execution']==execution and genesis['min_reader']==5,genesis
        assert not any(key in genesis for key in ('environment','folders','session_scope'))
        page=call('session.files.read',{'sessionId':session,'path':'file-page.txt','offset':1,'limit':1})
        assert page['text']=='SESSION_EXECUTION_READ_OK',page
        receipt=call('session.prompt',{'sessionId':session,'mode':'queue','content':[{'type':'text','text':'Read file-page.txt'}]})
        assert receipt['accepted'],receipt
        def settled():
            events=[json.loads(line) for line in ledger.read_text().splitlines()]
            return any(event.get('kind')=='settle' and event.get('outcome')=='completed' for event in events)
        deadline=time.monotonic()+30
        while time.monotonic()<deadline and not settled():time.sleep(.1)
        assert settled(),ledger.read_text()
        assert len(provider_requests)>=2,provider_requests
        assert tool_state['issued'] and any('SESSION_EXECUTION_READ_OK' in json.dumps(item) for item in provider_requests),'actual read result did not reach provider'
        assert 'KERNEL_IMAGE_RESPONSE_OK' in ledger.read_text()
        print('SESSION_EXECUTION_REAL_WORKER_AND_SANDBOXED_READ_OK')
        if os.environ.get('TEKES_TEST_CLI_HOST'):
            call('session.prompt',{'sessionId':session,'mode':'queue','content':[{'type':'text','text':'Call host MCP echo'}]})
            deadline=time.monotonic()+30
            while time.monotonic()<deadline and sum(json.loads(line).get('kind')=='settle' for line in ledger.read_text().splitlines())<2:time.sleep(.1)
            assert tool_state['mcp'] and 'HOST_STDIO_MCP_OK' in ledger.read_text(),ledger.read_text()
            assert any('HOST_STDIO_MCP_OK' in json.dumps(item) and 'function_call_output' in json.dumps(item) for item in provider_requests),provider_requests
            print('STANDALONE_CLI_HOST_REAL_MCP_CALL_OK')
        original_genesis=ledger.read_text().splitlines()[0]
        # Open the actual mux, then reconnect after a full supervisor restart.
        def inventory():
            url=urlparse(ready['url']);sock=socket.create_connection((url.hostname,url.port),timeout=10)
            key=base64.b64encode(secrets.token_bytes(16)).decode()
            auth=base64.urlsafe_b64encode(token).decode().rstrip('=')
            sock.sendall((f'GET /api/remote.mux HTTP/1.1\r\nHost: {url.netloc}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: {key}\r\nAuthorization: Bearer {auth}\r\n\r\n').encode())
            stream=sock.makefile('rb')
            try:
                assert b' 101 ' in stream.readline()
                while stream.readline()!=b'\r\n':pass
                def receive():
                    header=stream.read(2);assert len(header)==2
                    length=header[1]&127
                    if length==126:length=struct.unpack('!H',stream.read(2))[0]
                    elif length==127:length=struct.unpack('!Q',stream.read(8))[0]
                    return json.loads(stream.read(length))
                frame=receive();assert frame['type']=='ready',frame
                assert 'workspace-sync' not in frame['host']['capabilities'],frame
                payload=json.dumps({'type':'open','streamId':'inventory','target':{'kind':'session-inventory'}}).encode()
                mask=secrets.token_bytes(4)
                header=bytes([129,128|len(payload)]) if len(payload)<126 else bytes([129,254])+struct.pack('!H',len(payload))
                sock.sendall(header+mask+bytes(b^mask[i%4] for i,b in enumerate(payload)))
                frame=receive()
                rows=frame['frame']['snapshot']['items']
                row=next(row for row in rows if row['sessionId']==session)
                assert row['cwd']==str(workspace_path),row
            finally:
                stream.close();sock.close()
        inventory()
        p.stdin.close();p.wait(timeout=15);assert p.returncode==0,p.stderr.read()
        def no_mcp_processes():
            if not os.environ.get('TEKES_TEST_CLI_HOST'):return
            for pid in (base/'mcp-pids').read_text().splitlines():
                try:os.kill(int(pid),0)
                except ProcessLookupError:continue
                raise AssertionError('host left MCP process alive: '+pid)
        no_mcp_processes()
        p=subprocess.Popen(command,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,env=env)
        p.stdin.write(bootstrap);p.stdin.flush()
        assert select.select([p.stdout],[],[],20)[0],'restart readiness timeout'
        line=p.stdout.readline();assert line,p.stderr.read()
        ready=json.loads(line)
        inventory()
        assert ledger.read_text().splitlines()[0]==original_genesis
        call('session.rename',{'sessionId':session,'title':'Execution restart verified'})
        assert 'Execution restart verified' in ledger.read_text()
        restart_prompt='Confirm execution after restart'
        if os.environ.get('TEKES_TEST_CLI_HOST'):
            tool_state['mcp']=False
            restart_prompt='Call host MCP echo after restart'
        call('session.prompt',{'sessionId':session,'mode':'queue','content':[{'type':'text','text':restart_prompt}]})
        def completed_turns():
            return sum(event.get('kind')=='settle' and event.get('outcome')=='completed'
                for event in (json.loads(line) for line in ledger.read_text().splitlines()))
        expected_turns=3 if os.environ.get('TEKES_TEST_CLI_HOST') else 2
        deadline=time.monotonic()+30
        while time.monotonic()<deadline and completed_turns()<expected_turns:time.sleep(.1)
        assert completed_turns()==expected_turns,ledger.read_text()
        for relative,content in retired.items():assert (Path(root)/relative).read_bytes()==content
        print('SESSION_EXECUTION_RESTART_RECONNECT_AND_RETIRED_STATE_ISOLATION_OK')
        p.stdin.close();p.wait(timeout=15);assert p.returncode==0,p.stderr.read()
        no_mcp_processes()
        if os.environ.get('TEKES_TEST_CLI_HOST'):
            assert tool_state['mcp'] and len((base/'mcp-pids').read_text().splitlines())>=2
            print('CLI_MCP_RESTART_AND_ALL_HOST_CHILDREN_REAPED_OK')
        print('EXIT',p.returncode)
    finally:
        if p.poll() is None:p.kill();p.wait()
        mock_provider.shutdown();mock_provider.server_close()

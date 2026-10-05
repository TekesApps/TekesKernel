#!/usr/bin/env python3
"""Public-tunnel live gates against the official TypeScript MCP SDK.

Starts scripts/mcp-official-sdk/server.mjs (official @modelcontextprotocol/sdk
1.30.0 servers: tasks lifecycle and forced-SSE conformance), exposes it through an
ngrok HTTPS tunnel (the user's own ngrok agent configuration; the auth token never
leaves ngrok's files), proves the public URL answers from the internet, then runs
the Kernel gates in crates/mcp/tests/live_official_sdk_tunnel.rs with
TEKES_LIVE_MCP_TASKS_URL / TEKES_LIVE_MCP_CONFORMANCE_URL pointing at the tunnel.

Usage: run-live-mcp-official-sdk.py --output <new directory> [--loopback]

--loopback skips ngrok and runs the same gates over http://127.0.0.1 (rehearsal;
recorded as loopback, never as the public gate).
"""
import argparse, json, os, shutil, signal, subprocess, sys, time, urllib.request
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--loopback', action='store_true')
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
fixture = root / 'scripts/mcp-official-sdk'
args.output.mkdir(parents=True, exist_ok=False)
children = []

def stop_all():
    for child in children:
        if child.poll() is None:
            child.send_signal(signal.SIGTERM)
    for child in children:
        try: child.wait(timeout=10)
        except subprocess.TimeoutExpired: child.kill()

try:
    if not (fixture / 'node_modules/@modelcontextprotocol/sdk/package.json').exists():
        subprocess.run(['npm', 'install', '--no-audit', '--no-fund'], cwd=fixture, check=True)
    sdk_version = json.loads((fixture / 'node_modules/@modelcontextprotocol/sdk/package.json').read_text())['version']
    server_log = (args.output / 'server.log').open('w')
    server = subprocess.Popen(['node', 'server.mjs'], cwd=fixture, env=dict(os.environ, PORT='0'), stdout=subprocess.PIPE, stderr=server_log, text=True)
    children.append(server)
    port = json.loads(server.stdout.readline())['listening']
    local = f'http://127.0.0.1:{port}'
    assert json.loads(urllib.request.urlopen(local + '/healthz', timeout=10).read())['ok']

    public = None
    if not args.loopback:
        ngrok_log = (args.output / 'ngrok.log').open('w')
        # The free ngrok agent refuses to run behind an HTTP proxy (ERR_NGROK_9009);
        # its control connection goes out directly, without the shell's proxy variables.
        ngrok_env = {k: v for k, v in os.environ.items() if k.lower() not in ('http_proxy', 'https_proxy', 'all_proxy', 'no_proxy')}
        ngrok = subprocess.Popen(['ngrok', 'http', str(port), '--log', 'stdout', '--log-format', 'json'], stdout=ngrok_log, stderr=subprocess.STDOUT, env=ngrok_env)
        children.append(ngrok)
        deadline = time.monotonic() + 40
        while public is None and time.monotonic() < deadline:
            time.sleep(1)
            try:
                tunnels = json.loads(urllib.request.urlopen('http://127.0.0.1:4040/api/tunnels', timeout=5).read())['tunnels']
                public = next((t['public_url'] for t in tunnels if t['public_url'].startswith('https://')), None)
            except Exception:
                continue
        if public is None:
            raise SystemExit('ngrok did not publish an https tunnel within 40 s; see ngrok.log')
        # The public URL must answer through the internet, not only the agent.
        health = json.loads(urllib.request.urlopen(public + '/healthz', timeout=30).read())
        assert health['ok'], health
    base = public or local
    env = dict(os.environ,
               TEKES_LIVE_MCP_TASKS_URL=base + '/tasks/mcp',
               TEKES_LIVE_MCP_CONFORMANCE_URL=base + '/conformance/mcp',
               TEKES_LIVE_MCP_RECEIPT_DIR=str(args.output.resolve() / 'receipts'))
    if args.loopback:
        env['TEKES_LIVE_MCP_ALLOW_HTTP'] = '1'
    run = subprocess.run(['cargo', 'test', '-p', 'mcp', '--test', 'live_official_sdk_tunnel', '--locked', '--', '--ignored', '--nocapture'],
                         cwd=root, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    log = run.stdout.decode(errors='replace')
    (args.output / 'test.log').write_text(log)
    receipts = {p.stem: json.loads(p.read_text()) for p in sorted((args.output / 'receipts').glob('*.json'))} if (args.output / 'receipts').exists() else {}
    passed = run.returncode == 0 and '2 passed' in log and {'tasks-lifecycle', 'conformance-sse'} <= set(receipts)
    matrix = {'status': 'passed' if passed else 'failed', 'transport': 'loopback' if args.loopback else 'ngrok-public-https',
              'public_url_host': (public or local).split('//', 1)[1].split('/')[0], 'sdk_version': sdk_version,
              'protocol': '2025-11-25 (Kernel legacy mode; released SDKs implement no 2026-07-28 modern handshake)',
              'gates': {name: value.get('status') for name, value in receipts.items()}, 'test_exit': run.returncode}
    (args.output / 'matrix.json').write_text(json.dumps(matrix, indent=2))
    print(json.dumps(matrix, indent=2))
    sys.exit(0 if passed else 1)
finally:
    stop_all()

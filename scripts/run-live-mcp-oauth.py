#!/usr/bin/env python3
"""Legacy rpcOAuthPersistsRefreshTokenAndReconnectsAfterRestart, Kernel form.

Interactive once: dynamic client registration and an authorization-code + PKCE
grant against the MCP server's OAuth authorization server (Cloudflare by
default; the system browser opens once for consent, the loopback redirect
receives the code). The refresh grant is written to a 0600 file that only the
Kernel gate reads; tokens are never printed.

Then the Kernel gate (crates/supervisor/tests/live_oauth.rs) runs: the Kernel
mints its own OAuth record (secret-store §OAuth secret mutation), a fresh
supervisor MCP runtime exchanges the refresh token at the real token endpoint,
the rotated refresh token is written back (generation advances), a second
fresh runtime uses it, remote revocation then makes a third runtime fail with
`token endpoint returned HTTP 400`.

Usage: run-live-mcp-oauth.py --output <new directory> [--server-url URL] [--no-browser]
"""
import argparse, base64, hashlib, http.server, json, os, secrets, subprocess, sys, threading, urllib.parse, urllib.request, webbrowser
from pathlib import Path

parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--server-url', default='https://mcp.cloudflare.com/mcp')
parser.add_argument('--no-browser', action='store_true', help='Print the authorization URL instead of opening the system browser')
parser.add_argument('--grant', type=Path, help='Reuse an existing 0600 grant file (skips registration and consent)')
parser.add_argument('--timeout', type=int, default=300, help='Seconds to wait for the browser consent')
args = parser.parse_args()
root = Path(__file__).resolve().parents[1]
args.output.mkdir(parents=True, exist_ok=False)
private = args.output / 'private'
private.mkdir(mode=0o700)
origin = '{0.scheme}://{0.netloc}'.format(urllib.parse.urlsplit(args.server_url))

def get_json(url, data=None, headers=None):
    # Cloudflare's edge answers 403 to the bare urllib agent; identify the harness.
    merged = {'User-Agent': 'TekesKernel-live-oauth-gate/1', 'Accept': 'application/json'}
    merged.update(headers or {})
    request = urllib.request.Request(url, data=data, headers=merged, method='POST' if data is not None else 'GET')
    with urllib.request.urlopen(request, timeout=30) as response:
        return json.loads(response.read())

if args.grant:
    grant_path = args.grant
else:
    # RFC 8414 metadata (RFC 9728 protected-resource metadata names the server when the origin differs).
    try:
        metadata = get_json(origin + '/.well-known/oauth-authorization-server')
    except Exception:
        resource = get_json(origin + '/.well-known/oauth-protected-resource')
        metadata = get_json(resource['authorization_servers'][0].rstrip('/') + '/.well-known/oauth-authorization-server')
    assert 'refresh_token' in metadata.get('grant_types_supported', ['authorization_code', 'refresh_token']), metadata
    (args.output / 'authorization-server-metadata.json').write_text(json.dumps(metadata, indent=2))

    # Loopback redirect first so the registration carries the exact redirect URI.
    received = {}
    ready = threading.Event()
    class Callback(http.server.BaseHTTPRequestHandler):
        def log_message(self, *a): pass
        def do_GET(self):
            query = urllib.parse.parse_qs(urllib.parse.urlsplit(self.path).query)
            received.update({k: v[0] for k, v in query.items()})
            self.send_response(200); self.send_header('Content-Type', 'text/plain; charset=utf-8'); self.end_headers()
            self.wfile.write(b'TekesKernel OAuth gate: consent received, you can close this tab.')
            ready.set()
    server = http.server.HTTPServer(('127.0.0.1', 0), Callback)
    redirect_uri = 'http://127.0.0.1:%d/callback' % server.server_address[1]
    threading.Thread(target=server.serve_forever, daemon=True).start()

    # RFC 7591 dynamic client registration (public client, PKCE).
    registration = get_json(metadata['registration_endpoint'], data=json.dumps({
        'client_name': 'TekesKernel live OAuth gate', 'redirect_uris': [redirect_uri],
        'grant_types': ['authorization_code', 'refresh_token'], 'response_types': ['code'],
        'token_endpoint_auth_method': 'none'}).encode(), headers={'Content-Type': 'application/json'})
    client_id = registration['client_id']
    client_secret = registration.get('client_secret')
    (args.output / 'client-registration.json').write_text(json.dumps({k: v for k, v in registration.items() if k not in ('client_secret', 'registration_access_token')}, indent=2))

    verifier = base64.urlsafe_b64encode(secrets.token_bytes(48)).rstrip(b'=').decode()
    challenge = base64.urlsafe_b64encode(hashlib.sha256(verifier.encode()).digest()).rstrip(b'=').decode()
    state = secrets.token_urlsafe(24)
    query = {'response_type': 'code', 'client_id': client_id, 'redirect_uri': redirect_uri, 'code_challenge': challenge,
             'code_challenge_method': 'S256', 'state': state, 'resource': args.server_url}
    if metadata.get('scopes_supported'): query['scope'] = ' '.join(metadata['scopes_supported'])
    authorization_url = metadata['authorization_endpoint'] + '?' + urllib.parse.urlencode(query)
    if args.no_browser:
        print('Open this URL in a browser and complete the consent:\n' + authorization_url)
    else:
        print('Opening the system browser for the OAuth consent (once)...')
        webbrowser.open(authorization_url)
    if not ready.wait(args.timeout):
        raise SystemExit('no consent received within %d seconds' % args.timeout)
    server.shutdown()
    assert received.get('state') == state, 'state mismatch on the redirect'
    if 'error' in received: raise SystemExit('authorization refused: %s %s' % (received['error'], received.get('error_description', '')))
    form = {'grant_type': 'authorization_code', 'code': received['code'], 'redirect_uri': redirect_uri, 'client_id': client_id,
            'code_verifier': verifier, 'resource': args.server_url}
    if client_secret: form['client_secret'] = client_secret
    tokens = get_json(metadata['token_endpoint'], data=urllib.parse.urlencode(form).encode(), headers={'Content-Type': 'application/x-www-form-urlencoded'})
    assert tokens.get('refresh_token'), 'the token endpoint issued no refresh token: ' + ','.join(sorted(tokens))
    grant_path = private / 'oauth-grant.json'
    fd = os.open(grant_path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(fd, 'w') as handle:
        json.dump({'token_endpoint': metadata['token_endpoint'], 'revocation_endpoint': metadata.get('revocation_endpoint'),
                   'client_id': client_id, 'client_secret': client_secret, 'refresh_token': tokens['refresh_token'],
                   'resource': args.server_url, 'scope': tokens.get('scope')}, handle)
    print('grant stored (0600, not printed); access token expires_in=%s' % tokens.get('expires_in'))

runtime = args.output / 'runtime'
runtime.mkdir()
env = dict(os.environ, TEKES_LIVE_MCP_OAUTH_GRANT=str(grant_path), TEKES_LIVE_MCP_OAUTH_URL=args.server_url, TEKES_PROCESS_ARTIFACT=str(runtime))
log = args.output / 'gate.log'
with log.open('w') as handle:
    run = subprocess.run(['cargo', 'test', '-p', 'tekes-supervisor', '--test', 'live_oauth', '--', '--ignored', '--nocapture', '--test-threads=1'],
                         cwd=root, env=env, stdout=handle, stderr=subprocess.STDOUT)
receipt_path = runtime / 'oauth-gate.json'
receipt = json.loads(receipt_path.read_text()) if receipt_path.exists() else None
matrix = {'status': 'passed' if run.returncode == 0 and receipt and receipt.get('status') == 'passed' else 'failed',
          'server_url': args.server_url, 'test_exit': run.returncode, 'gate': receipt, 'log': str(log)}
(args.output / 'matrix.json').write_text(json.dumps(matrix, indent=2))
print(json.dumps({'status': matrix['status'], 'gate': receipt and {k: receipt[k] for k in receipt if k != 'steps'}}, indent=1))
sys.exit(0 if matrix['status'] == 'passed' else 1)

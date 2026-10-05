#!/usr/bin/env python3
"""Forward provider requests to an upstream base URL and record each body.

Bodies are saved as request-<n>.json beside a small index; the Authorization
header is never written. Streaming responses are relayed unchanged; their
terminal usage frame is recorded as response-<n>.json when parseable.
"""

import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import ssl
import threading
import urllib.request

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--upstream', required=True, help='e.g. https://api.deepseek.com')
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--port', type=int, default=0)
parser.add_argument('--ready-file', type=Path, help='written with the bound port once listening')
args = parser.parse_args()
args.output.mkdir(parents=True, exist_ok=True)
upstream = args.upstream.rstrip('/')
lock = threading.Lock()
counter = [0]
HOP = {'connection', 'keep-alive', 'transfer-encoding', 'host', 'content-length', 'accept-encoding'}


class Handler(BaseHTTPRequestHandler):
    protocol_version = 'HTTP/1.1'

    def log_message(self, *_):
        pass

    def do_POST(self):
        length = int(self.headers.get('Content-Length') or 0)
        body = self.rfile.read(length)
        with lock:
            counter[0] += 1
            n = counter[0]
        try:
            parsed = json.loads(body)
            (args.output / f'request-{n}.json').write_text(json.dumps(parsed, ensure_ascii=False, indent=1) + '\n')
        except json.JSONDecodeError:
            (args.output / f'request-{n}.bin').write_bytes(body)
        headers = {k: v for k, v in self.headers.items() if k.lower() not in HOP}
        headers['Accept-Encoding'] = 'identity'
        request = urllib.request.Request(upstream + self.path, data=body, headers=headers, method='POST')
        context = ssl.create_default_context()
        try:
            response = urllib.request.urlopen(request, context=context, timeout=600)
        except urllib.error.HTTPError as error:
            response = error
        status = response.getcode()
        self.send_response(status)
        for key, value in response.headers.items():
            if key.lower() in HOP or key.lower() == 'content-length':
                continue
            self.send_header(key, value)
        chunked = True
        self.send_header('Transfer-Encoding', 'chunked')
        self.end_headers()
        collected = bytearray()
        while True:
            chunk = response.read(4096)
            if not chunk:
                break
            collected.extend(chunk)
            self.wfile.write(f'{len(chunk):x}\r\n'.encode() + chunk + b'\r\n')
            self.wfile.flush()
        self.wfile.write(b'0\r\n\r\n')
        self.wfile.flush()
        record = {'status': status, 'path': self.path, 'bytes': len(collected)}
        text = collected.decode('utf-8', 'replace')
        usage = None
        for line in text.splitlines():
            if line.startswith('data: '):
                try:
                    frame = json.loads(line[6:])
                except json.JSONDecodeError:
                    continue
                if frame.get('type') == 'response.completed':
                    usage = frame.get('response', {}).get('usage')
        if usage is None and text.strip().startswith('{'):
            try:
                usage = json.loads(text).get('usage')
            except json.JSONDecodeError:
                pass
        record['usage'] = usage
        (args.output / f'response-{n}.json').write_text(json.dumps(record, indent=1) + '\n')


server = ThreadingHTTPServer(('127.0.0.1', args.port), Handler)
if args.ready_file:
    args.ready_file.write_text(str(server.server_port))
print(f'listening on http://127.0.0.1:{server.server_port} -> {upstream}', flush=True)
server.serve_forever()

#!/usr/bin/env python3
"""Standalone Kernel host. No Tekes/App dependency.

Run: python3 host/kernel_cli.py --launch HOST.json --supervisor BIN
HOST.json is the Kernel launch value plus optional local_mcp mapping:
  {"name": {"command":["/absolute/server", "arg"], "cwd":"/project",
            "environment": {"NAME":"value"}}}
The caller sends the private bootstrap frame on stdin and keeps the pipe open
for the desired host lifetime. stdout forwards Kernel readiness unchanged.
The host owns every MCP child; Kernel sees only a private stdio channel.
"""
import argparse
import contextlib
from collections import deque
import json
import os
from pathlib import Path
import signal
import select
import socket
import stat
import subprocess
import tempfile
import threading
import weakref


class LocalMCPHost:
    def __init__(self, servers):
        self._temporary = tempfile.TemporaryDirectory(prefix="tekes-mcp-", dir="/tmp")
        self.root = Path(self._temporary.name).resolve()
        os.chmod(self.root, 0o700)
        self._lock = threading.Lock()
        self._closed = False
        self._listeners = []
        self._connections = set()
        self._children = set()
        self._stopped = weakref.WeakSet()
        self._threads = []
        self.channels = {}
        try:
            for index, (name, config) in enumerate(servers.items()):
                if not name or len(name) > 128 or any(not (c.isascii() and (c.isalnum() or c in "_-")) for c in name):
                    raise ValueError("invalid MCP channel name")
                command = config.get("command")
                if not isinstance(command, list) or not command or any(not isinstance(arg, str) or "\0" in arg for arg in command) or not Path(command[0]).is_absolute():
                    raise ValueError("MCP command must be an explicit absolute executable and arguments")
                if set(config) - {"command", "cwd", "environment"}:
                    raise ValueError("unknown local MCP option")
                environment = config.get("environment", {})
                if not isinstance(environment, dict) or any(not isinstance(k, str) or not isinstance(v, str) or "=" in k or "\0" in k + v for k, v in environment.items()):
                    raise ValueError("invalid MCP environment")
                cwd = config.get("cwd")
                if cwd is not None and (not isinstance(cwd, str) or not Path(cwd).is_absolute()):
                    raise ValueError("MCP cwd must be absolute")
                config = {"command": list(command), "cwd": cwd, "environment": dict(environment)}
                listener = socket.socket(socket.AF_UNIX)
                path = self.root / f"channel-{index}.sock"
                listener.bind(str(path))
                os.chmod(path, 0o600)
                listener.listen()
                listener.settimeout(0.2)
                self.channels[name] = str(path)
                self._listeners.append(listener)
                self._start(self._accept, listener, config)
        except BaseException:
            self.close()
            raise

    def _start(self, target, *args):
        with self._lock:
            if self._closed:
                return False
            self._threads = [thread for thread in self._threads if thread.is_alive()]
            thread = threading.Thread(target=target, args=args, daemon=True)
            self._threads.append(thread)
            thread.start()
            return True

    def _accept(self, listener, config):
        while not self._closed:
            try:
                connection, _ = listener.accept()
            except socket.timeout:
                continue
            except OSError:
                return
            with self._lock:
                if self._closed:
                    connection.close()
                    return
                self._connections.add(connection)
            if not self._start(self._serve, connection, config):
                connection.close()
                with self._lock:
                    self._connections.discard(connection)

    def _stop(self, child):
        with self._lock:
            if child in self._stopped:
                return
            self._stopped.add(child)
        # A group may outlive its leader, so always signal the host-owned pgid.
        try:
            os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        except PermissionError:
            # A failed signal is only harmless when the leader is reaped and
            # the process group is demonstrably gone. Keep EPERM fail-closed.
            child.wait(timeout=5)
            try:
                os.killpg(child.pid, 0)
            except ProcessLookupError:
                return
            raise
        child.wait(timeout=5)

    def _serve(self, connection, config):
        child = None
        try:
            with self._lock:
                if self._closed:
                    return
                child = subprocess.Popen(config["command"], cwd=config.get("cwd"),
                    env=config.get("environment", {}), stdin=subprocess.PIPE,
                    stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                    start_new_session=True, close_fds=True, bufsize=0)
                self._children.add(child)
            def output():
                try:
                    while data := child.stdout.read(65536):
                        connection.sendall(data)
                except (OSError, ValueError):
                    pass
                finally:
                    with contextlib.suppress(OSError):
                        connection.shutdown(socket.SHUT_RDWR)
            output_thread = threading.Thread(target=output, daemon=True)
            output_thread.start()
            # Preserve backpressure without losing the peer's HUP while the
            # child's stdin pipe is full. Blocking write() here would prevent
            # observing Kernel EOF and leave an uncooperative child alive.
            os.set_blocking(child.stdin.fileno(), False)
            pending = deque()
            queued_bytes = 0
            offset = 0
            while True:
                events = select.poll()
                # Always drain input so EOF stays observable on platforms that
                # do not report HUP until unread socket bytes are consumed.
                events.register(connection, select.POLLIN)
                if pending:
                    events.register(child.stdin, select.POLLOUT)
                for descriptor, flags in events.poll(200):
                    if descriptor == connection.fileno():
                        if flags & (select.POLLHUP | select.POLLERR | select.POLLNVAL):
                            return
                        if flags & select.POLLIN:
                            data = connection.recv(65536)
                            if not data:
                                return
                            queued_bytes += len(data)
                            # Kernel frames are bounded to 4 MiB. Keep a
                            # bounded relay backlog even for a hostile peer.
                            if queued_bytes > 8 * 1024 * 1024:
                                return
                            pending.append(data)
                    elif descriptor == child.stdin.fileno():
                        if flags & (select.POLLHUP | select.POLLERR | select.POLLNVAL):
                            return
                        try:
                            written = os.write(child.stdin.fileno(), pending[0][offset:])
                        except BlockingIOError:
                            continue
                        offset += written
                        queued_bytes -= written
                        if offset == len(pending[0]):
                            pending.popleft()
                            offset = 0
        except (OSError, ValueError):
            pass
        finally:
            if child is not None:
                child.stdin.close()
                self._stop(child)
                child.stdout.close()
                output_thread.join(timeout=2)
            connection.close()
            with self._lock:
                self._connections.discard(connection)
                if child is not None:
                    self._children.discard(child)

    def close(self):
        with self._lock:
            self._closed = True
            listeners = list(self._listeners)
            connections = list(self._connections)
            children = list(self._children)
            threads = list(self._threads)
        for listener in listeners:
            listener.close()
        for connection in connections:
            with contextlib.suppress(OSError):
                connection.shutdown(socket.SHUT_RDWR)
        for child in children:
            self._stop(child)
        for thread in threads:
            thread.join(timeout=2)
        self._temporary.cleanup()

    def __enter__(self):
        return self

    def __exit__(self, *args):
        self.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--launch", required=True, type=Path)
    parser.add_argument("--supervisor", required=True, type=Path)
    args = parser.parse_args()
    def terminate(_signal, _frame):
        raise KeyboardInterrupt
    signal.signal(signal.SIGTERM, terminate)
    with args.launch.open("rb") as source:
        metadata = os.fstat(source.fileno())
        if metadata.st_uid != os.geteuid() or stat.S_IMODE(metadata.st_mode) & 0o077:
            parser.error("host launch configuration must be private to its owner")
        launch = json.load(source)
    servers = launch.pop("local_mcp", {})
    with LocalMCPHost(servers) as host:
        if "stdio_channels" in launch:
            parser.error("stdio_channels is supplied by this host")
        launch["stdio_channels"] = host.channels
        path = host.root / "kernel-launch.json"
        descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        with os.fdopen(descriptor, "w") as output:
            json.dump(launch, output)
        # The private bootstrap/lifetime pipe is inherited as stdin. No
        # credentials are parsed, copied into the environment, or logged here.
        kernel = subprocess.Popen([str(args.supervisor.resolve()), "--built-in", str(path)],
                                  close_fds=True)
        try:
            return kernel.wait()
        except KeyboardInterrupt:
            kernel.terminate()
            return kernel.wait(timeout=10)
        finally:
            if kernel.poll() is None:
                kernel.kill()
                kernel.wait(timeout=10)


if __name__ == "__main__":
    raise SystemExit(main())

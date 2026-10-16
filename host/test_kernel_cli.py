import contextlib
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import time
import unittest
from kernel_cli import LocalMCPHost


def gone(pid):
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return True
    return False


def eventually(predicate, timeout=4):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if predicate():
            return
        time.sleep(.02)
    raise AssertionError("host did not finish process cleanup")


class LocalMCPHostTests(unittest.TestCase):
    def connect(self, host, name="fixture"):
        connection = socket.socket(socket.AF_UNIX)
        connection.settimeout(3)
        connection.connect(host.channels[name])
        self.addCleanup(connection.close)
        return connection

    def test_protocol_stream_and_reconnect_use_distinct_host_processes(self):
        code = "import json,sys,os\nfor line in sys.stdin: print(json.dumps(dict(pid=os.getpid(),value=json.loads(line))),flush=True)"
        with LocalMCPHost({"fixture": {"command": [sys.executable, "-u", "-c", code]}}) as host:
            first = self.connect(host)
            first.sendall(b'{"value":"first"}\n')
            data = json.loads(first.makefile("rb").readline())
            self.assertEqual(data["value"], {"value": "first"})
            first.close()
            eventually(lambda: gone(data["pid"]))
            second = self.connect(host)
            second.sendall(b'{"value":"second"}\n')
            other = json.loads(second.makefile("rb").readline())
            self.assertNotEqual(data["pid"], other["pid"])
            self.assertEqual(other["value"]["value"], "second")
        eventually(lambda: gone(other["pid"]))

    def test_host_reaps_an_uncooperative_server_on_kernel_stream_eof(self):
        code = "import os,time; print(os.getpid(),flush=True); time.sleep(3600)"
        with LocalMCPHost({"fixture": {"command": [sys.executable, "-u", "-c", code]}}) as host:
            connection = self.connect(host)
            with connection.makefile("rb") as stream:
                pid = int(stream.readline())
            connection.close()
            eventually(lambda: gone(pid))

    def test_backpressured_stdin_does_not_hide_kernel_disconnect(self):
        code = "import os,time; print(os.getpid(),flush=True); time.sleep(3600)"
        with LocalMCPHost({"fixture": {"command": [sys.executable, "-u", "-c", code]}}) as host:
            connection = self.connect(host)
            with connection.makefile("rb") as stream:
                pid = int(stream.readline())
            connection.settimeout(.2)
            with contextlib.suppress(socket.timeout):
                connection.sendall(b"x" * (1024 * 1024))
            connection.shutdown(socket.SHUT_RDWR)
            connection.close()
            eventually(lambda: gone(pid), timeout=2)

    def test_host_shutdown_reaps_live_server_and_removes_channels(self):
        code = "import os,time; print(os.getpid(),flush=True); time.sleep(3600)"
        host = LocalMCPHost({"fixture": {"command": [sys.executable, "-u", "-c", code]}})
        self.addCleanup(host.close)
        connection = self.connect(host)
        with connection.makefile("rb") as stream:
            pid = int(stream.readline())
        root = host.root
        host.close()
        eventually(lambda: gone(pid))
        self.assertFalse(root.exists())

    def test_explicit_environment_and_frozen_command_exclude_host_secrets(self):
        code = "import os,json; print(json.dumps(dict(visible=os.getenv('VISIBLE'),leaked='HOST_ONLY_SECRET' in os.environ)),flush=True)"
        original = os.environ.get("HOST_ONLY_SECRET")
        os.environ["HOST_ONLY_SECRET"] = "never-in-mcp"
        try:
            config = {"fixture": {"command": [sys.executable, "-u", "-c", code], "environment": {"VISIBLE": "yes"}}}
            with LocalMCPHost(config) as host:
                config["fixture"]["command"][:] = ["/does-not-exist"]
                connection = self.connect(host)
                with connection.makefile("rb") as stream:
                    value = json.loads(stream.readline())
                self.assertEqual(value, {"visible": "yes", "leaked": False})
        finally:
            if original is None:
                os.environ.pop("HOST_ONLY_SECRET", None)
            else:
                os.environ["HOST_ONLY_SECRET"] = original

    def test_invalid_command_is_rejected_before_any_process_or_channel(self):
        for config in [{"command": ["relative"]}, {"command": [sys.executable], "trusted": True}, {"command": [sys.executable], "environment": {"BAD=KEY": "value"}}]:
            with self.assertRaises(ValueError):
                LocalMCPHost({"fixture": config})

    def test_failed_spawn_closes_channel_without_breaking_host_lifetime(self):
        with LocalMCPHost({"fixture": {"command": ["/nonexistent-mcp-server"]}}) as host:
            connection = self.connect(host)
            self.assertEqual(connection.recv(1), b"")
            self.assertTrue(Path(host.channels["fixture"]).exists())

    def test_cli_rejects_nonprivate_config_before_starting_kernel(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "host.json"
            path.write_text("{}")
            path.chmod(0o644)
            result = subprocess.run([sys.executable, str(Path(__file__).with_name("kernel_cli.py")), "--launch", str(path), "--supervisor", "/not-started"], input=b"private-input", capture_output=True, timeout=5)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn(b"private to its owner", result.stderr)
            self.assertNotIn(b"private-input", result.stderr)
            self.assertEqual(result.stdout, b"")


if __name__ == "__main__":
    unittest.main()

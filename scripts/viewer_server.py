#!/usr/bin/env python3

"""Serve the sekai viewer and expose a local round-trip simulation endpoint."""

from __future__ import annotations

import argparse
import json
import subprocess
import tempfile
from http import HTTPStatus
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent


class ViewerHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, directory: str | None = None, **kwargs):
        super().__init__(*args, directory=str(REPO_ROOT), **kwargs)

    def do_POST(self) -> None:  # noqa: N802
        if self.path != "/api/simulate":
            self.send_error(HTTPStatus.NOT_FOUND, "Unknown endpoint")
            return

        try:
            length = int(self.headers.get("Content-Length", "0"))
        except ValueError:
            self.send_error(HTTPStatus.BAD_REQUEST, "Invalid Content-Length")
            return

        raw_body = self.rfile.read(length)
        try:
            payload = json.loads(raw_body.decode("utf-8"))
        except json.JSONDecodeError as error:
            self.send_json(
                HTTPStatus.BAD_REQUEST,
                {
                    "source": "draft.sk",
                    "status": "error",
                    "error": f"invalid JSON payload: {error}",
                    "snapshots": [],
                },
            )
            return

        source = payload.get("source", "")
        name = payload.get("name", "draft.sk")
        if not isinstance(source, str) or not source.strip():
            self.send_json(
                HTTPStatus.BAD_REQUEST,
                {
                    "source": name,
                    "status": "error",
                    "error": "draft source must be a non-empty string",
                    "snapshots": [],
                },
            )
            return

        report = run_simulation(source, name)
        self.send_json(HTTPStatus.OK, report)

    def send_json(self, status: HTTPStatus, payload: dict[str, object]) -> None:
        encoded = json.dumps(payload, ensure_ascii=True, indent=2).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(encoded)))
        self.end_headers()
        self.wfile.write(encoded)


def run_simulation(source: str, name: str) -> dict[str, object]:
    with tempfile.NamedTemporaryFile(
        mode="w",
        encoding="utf-8",
        suffix=".sk",
        prefix="viewer-draft-",
        delete=False,
        dir=REPO_ROOT,
    ) as handle:
        temp_path = Path(handle.name)
        handle.write(source)

    try:
        completed = subprocess.run(
            [
                "cargo",
                "run",
                "--quiet",
                "-p",
                "sekai-cli",
                "--",
                "simulate-report",
                str(temp_path),
            ],
            cwd=REPO_ROOT,
            capture_output=True,
            text=True,
            check=False,
        )

        if completed.returncode != 0:
            return {
                "source": name,
                "status": "error",
                "error": completed.stderr.strip() or "sekai runtime failed",
                "snapshots": [],
            }

        try:
            report = json.loads(completed.stdout)
        except json.JSONDecodeError as error:
            return {
                "source": name,
                "status": "error",
                "error": f"runtime returned invalid JSON: {error}",
                "snapshots": [],
            }

        report["source"] = name
        return report
    finally:
        temp_path.unlink(missing_ok=True)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Serve the sekai viewer with round-trip execution.")
    parser.add_argument("--host", default="127.0.0.1", help="host interface to bind")
    parser.add_argument("--port", default=8000, type=int, help="port to bind")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    server = ThreadingHTTPServer((args.host, args.port), ViewerHandler)
    print(f"sekai viewer server running at http://{args.host}:{args.port}/viewer/")
    print("Press Ctrl+C to stop.")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()


if __name__ == "__main__":
    main()

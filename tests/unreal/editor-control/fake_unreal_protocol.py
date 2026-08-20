# Copyright:
#   - Copyright © 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Fake unreal protocol test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Fake unreal protocol test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Fake unreal protocol test module."""

from __future__ import annotations

from http.server import BaseHTTPRequestHandler
import json
from time import sleep
from typing import TYPE_CHECKING
from typing import cast

from fake_unreal_tools import tool_result
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import JsonValue
from mcp.domain.json_types import require_json_object

if TYPE_CHECKING:
    from fake_unreal_server import FakeUnrealHttpServer

_SESSION_ID = "0123456789abcdef0123456789abcdef"
_PROTOCOL_VERSION = "2025-11-25"
_HTTP_OK = 200
_HTTP_ACCEPTED = 202
_HTTP_FOUND = 302
_HTTP_BAD_REQUEST = 400
_HTTP_FORBIDDEN = 403
_HTTP_INTERNAL_SERVER_ERROR = 500


class FakeUnrealRequestHandler(BaseHTTPRequestHandler):
    def do_POST(self) -> None:
        """Handle one native MCP JSON-RPC request or notification."""
        if not self._origin_header_is_valid():
            self._write_empty(_HTTP_FORBIDDEN)
            return
        payload = self._read_json_payload()
        self._test_server().requests.append(payload)
        method = payload.get("method")
        if not isinstance(method, str):
            self._write_rpc_error(payload.get("id"), "missing method")
        elif method == "initialize":
            self._write_initialize(payload)
        elif not self._session_headers_are_valid():
            self._write_rpc_error(payload.get("id"), "invalid session")
        elif method == "notifications/initialized":
            status = (
                _HTTP_OK
                if self._test_server().behavior.reject_initialized_notification
                else _HTTP_ACCEPTED
            )
            self._write_empty(status)
        elif method == "notifications/cancelled":
            self._write_cancelled(payload)
        elif method == "ping":
            self._write_ping(payload)
        elif method == "tools/list":
            self._write_tools_list(payload)
        elif method == "tools/call":
            self._write_tool_call(payload)
        else:
            self._write_rpc_error(payload.get("id"), "unknown method")

    def do_DELETE(self) -> None:
        """Handle one native MCP session deletion."""
        if not self._origin_header_is_valid():
            self._write_empty(_HTTP_FORBIDDEN)
            return
        if self.headers.get("Mcp-Session-Id") != _SESSION_ID:
            self._write_empty(_HTTP_BAD_REQUEST)
            return
        server = self._test_server()
        if server.behavior.reject_session_delete:
            self._write_empty(_HTTP_BAD_REQUEST)
            return
        server.session_closed = True
        self._write_empty(_HTTP_ACCEPTED)

    def log_message(
        self,
        format: str,  # noqa: A002 -- Matches the base override signature.
        *args: object,
    ) -> None:
        """Suppress deterministic test-server access logs."""
        del self, format, args

    def _test_server(self) -> FakeUnrealHttpServer:
        """Return the synthetic server with its narrowed fixture type."""
        return cast("FakeUnrealHttpServer", self.server)

    def _read_json_payload(self) -> JsonObject:
        raw_length = self.headers.get("Content-Length", "0")
        length = int(raw_length)
        body = self.rfile.read(length)
        parsed = cast("object", json.loads(body))
        return require_json_object(parsed, context="fake request")

    def _origin_header_is_valid(self) -> bool:
        host, port = cast(
            "tuple[str, int]",
            self._test_server().server_address,
        )
        return self.headers.get("Origin") == f"http://{host}:{port}"

    def _session_headers_are_valid(self) -> bool:
        return (
            self.headers.get("Mcp-Session-Id") == _SESSION_ID
            and self.headers.get("Mcp-Protocol-Version") == _PROTOCOL_VERSION
        )

    def _write_ping(self, payload: JsonObject) -> None:
        behavior = self._test_server().behavior
        if behavior.plain_error_ping:
            self._write_text(_HTTP_INTERNAL_SERVER_ERROR, "native failure")
            return
        status = _HTTP_FOUND if behavior.redirect_ping else _HTTP_OK
        self._write_rpc_result(payload.get("id"), {}, status=status)

    def _write_cancelled(self, payload: JsonObject) -> None:
        params = require_json_object(
            payload.get("params", {}),
            context="notifications/cancelled params",
        )
        request_id = params.get("requestId")
        server = self._test_server()
        sleep(server.behavior.cancellation_delay_seconds)
        server.cancelled_request_ids.append(request_id)
        server.cancel_event.set()
        self._write_empty(_HTTP_ACCEPTED)

    def _write_initialize(self, payload: JsonObject) -> None:
        result: JsonObject = {
            "protocolVersion": _PROTOCOL_VERSION,
            "capabilities": {
                "tools": {"listChanged": True},
                "resources": {},
            },
            "serverInfo": {
                "name": "",
                "title": "",
                "version": "",
            },
        }
        if self._test_server().behavior.malformed_initialize_result:
            result["capabilities"] = {}
        self._write_json(
            _HTTP_OK,
            {"jsonrpc": "2.0", "id": payload.get("id"), "result": result},
            session_id=_SESSION_ID,
        )

    def _write_tools_list(self, payload: JsonObject) -> None:
        params = require_json_object(
            payload.get("params", {}),
            context="tools/list params",
        )
        cursor = params.get("cursor")
        server = self._test_server()
        if cursor is None:
            result: JsonObject = {
                "tools": [
                    {"name": "list_toolsets", "inputSchema": {}},
                    {"name": "describe_toolset", "inputSchema": {}},
                ],
                "nextCursor": "repeat"
                if server.behavior.repeat_cursor
                else "page-2",
            }
        else:
            result = {"tools": [{"name": "call_tool", "inputSchema": {}}]}
            if server.behavior.repeat_cursor:
                result["nextCursor"] = "repeat"
            elif server.behavior.advance_cursor:
                result["nextCursor"] = f"{cursor}-next"
        self._write_rpc_result(payload.get("id"), result)

    def _write_tool_call(self, payload: JsonObject) -> None:
        params = require_json_object(
            payload.get("params", {}),
            context="tools/call params",
        )
        tool_name = params.get("name")
        arguments = require_json_object(
            params.get("arguments", {}),
            context="tools/call arguments",
        )
        server = self._test_server()
        if server.behavior.delay_tool_calls and tool_name == "call_tool":
            _ = server.cancel_event.wait(timeout=2.0)
            if server.cancel_event.is_set():
                return
        text, structured = tool_result(
            tool_name,
            arguments,
            empty_toolsets=server.behavior.empty_toolsets,
            plan_execution=server.behavior.plan_execution,
            assets=server.assets,
            dirty_assets=server.dirty_assets,
            media_payloads=server.media_payloads,
        )
        result: JsonObject = {
            "content": [{"type": "text", "text": text}],
            "isError": False,
        }
        if structured is not None:
            result["structuredContent"] = structured
        progress: JsonObject = {
            "jsonrpc": "2.0",
            "method": "notifications/progress",
            "params": {"progress": 0.5},
        }
        final: JsonObject = {
            "jsonrpc": "2.0",
            "id": payload.get("id"),
            "result": result,
        }
        body = (
            f"data: {json.dumps(progress, separators=(",", ":"))}\n\n"
            f"data: {json.dumps(final, separators=(",", ":"))}\n"
        ).encode()
        self.send_response(_HTTP_OK)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        _ = self.wfile.write(body)

    def _write_rpc_result(
        self,
        request_id: JsonValue,
        result: JsonObject,
        *,
        status: int = _HTTP_OK,
    ) -> None:
        self._write_json(
            status,
            {"jsonrpc": "2.0", "id": request_id, "result": result},
        )

    def _write_rpc_error(
        self,
        request_id: JsonValue,
        message: str,
    ) -> None:
        self._write_json(
            _HTTP_BAD_REQUEST,
            {
                "jsonrpc": "2.0",
                "id": request_id,
                "error": {"code": -32600, "message": message},
            },
        )

    def _write_json(
        self,
        status: int,
        payload: JsonObject,
        *,
        session_id: str | None = None,
    ) -> None:
        body = json.dumps(payload, separators=(",", ":")).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        if session_id is not None:
            self.send_header("Mcp-Session-Id", session_id)
        self.end_headers()
        _ = self.wfile.write(body)

    def _write_text(self, status: int, text: str) -> None:
        body = text.encode()
        self.send_response(status)
        self.send_header("Content-Type", "text/plain")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        _ = self.wfile.write(body)

    def _write_empty(self, status: int) -> None:
        self.send_response(status)
        self.send_header("Content-Length", "0")
        self.end_headers()

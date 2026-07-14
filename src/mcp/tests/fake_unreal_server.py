# File:
#   - fake_unreal_server.py
# Path:
#   - src/mcp/tests/fake_unreal_server.py
#
# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE
# Path-Rule:
#   - All paths in this header are repository-root relative.
#
# Boundary-Contract:
# - Owns:
#   - Lifecycle and observable state for a synthetic Unreal MCP server.
# - Must-Not:
#   - Parse protocol messages or implement HTTP response behavior.
# - Allows:
#   - Ephemeral loopback startup, shutdown, and test-state snapshots.
# - Split-When:
#   - Server state and context-manager lifecycle evolve independently.
# - Merge-When:
#   - The protocol handler no longer needs an independent HTTP boundary.
# - Summary:
#   - Provides the context-managed black-box Unreal MCP fixture.
# - Description:
#   - Keeps test server lifetime separate from request handling semantics.
# - Usage:
#   - Imported by black-box translator tests.
# - Defaults:
#   - Binds an ephemeral loopback port and closes after each test.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: synthetic server lifecycle fixture
#   - reason: context management and observable server state form one contract
#   - split: separate state storage if it gains behavior beyond snapshots
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on any lifecycle responsibility or line-count growth
#
"""Context-managed synthetic Unreal MCP server for black-box tests."""

from __future__ import annotations

from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from threading import Event, Thread
from typing import TYPE_CHECKING, NamedTuple, Self, cast

from tests.fake_unreal_protocol import FakeUnrealRequestHandler

if TYPE_CHECKING:
    from types import TracebackType

    from mcp.src.domain.json_types import JsonObject


class FakeUnrealBehavior(NamedTuple):
    """Deterministic protocol switches for one synthetic server."""

    repeat_cursor: bool = False
    delay_tool_calls: bool = False
    cancellation_delay_seconds: float = 0.0
    empty_toolsets: bool = False


class FakeUnrealServer:
    """Context-managed synthetic native Unreal MCP endpoint."""

    def __init__(
        self,
        *,
        repeat_cursor: bool = False,
        delay_tool_calls: bool = False,
        cancellation_delay_seconds: float = 0.0,
        empty_toolsets: bool = False,
    ) -> None:
        """Create a stopped server with deterministic behavior."""
        behavior = FakeUnrealBehavior(
            repeat_cursor=repeat_cursor,
            delay_tool_calls=delay_tool_calls,
            cancellation_delay_seconds=cancellation_delay_seconds,
            empty_toolsets=empty_toolsets,
        )
        self._server = FakeUnrealHttpServer(
            ("127.0.0.1", 0),
            FakeUnrealRequestHandler,
            behavior,
        )
        self._thread = Thread(
            target=self._server.serve_forever,
            name="fake-unreal-mcp",
            daemon=True,
        )

    def __enter__(self) -> Self:
        """Start the loopback server.

        Returns:
            This running server fixture.
        """
        self._thread.start()
        return self

    def __exit__(
        self,
        exception_type: type[BaseException] | None,
        exception: BaseException | None,
        traceback: TracebackType | None,
    ) -> None:
        """Stop the loopback server and release its socket."""
        del exception_type, exception, traceback
        self._server.shutdown()
        self._server.server_close()
        self._thread.join()

    @property
    def endpoint(self) -> str:
        """The ephemeral MCP endpoint.

        Returns:
            A loopback HTTP URL ending in `/mcp`.
        """
        host, port = cast("tuple[str, int]", self._server.server_address)
        return f"http://{host}:{port}/mcp"

    @property
    def requests(self) -> tuple[JsonObject, ...]:
        """Every received JSON-RPC payload.

        Returns:
            An immutable request snapshot in arrival order.
        """
        return tuple(self._server.requests)

    @property
    def cancelled_request_ids(self) -> tuple[object, ...]:
        """Request IDs received through cancellation notifications."""
        return tuple(self._server.cancelled_request_ids)

    @property
    def session_closed(self) -> bool:
        """Whether the client deleted its session.

        Returns:
            `True` after one valid MCP DELETE request.
        """
        return self._server.session_closed


class FakeUnrealHttpServer(ThreadingHTTPServer):
    daemon_threads = True

    requests: list[JsonObject]
    session_closed: bool
    cancelled_request_ids: list[object]
    cancel_event: Event
    behavior: FakeUnrealBehavior

    def __init__(
        self,
        server_address: tuple[str, int],
        handler_type: type[BaseHTTPRequestHandler],
        behavior: FakeUnrealBehavior,
    ) -> None:
        super().__init__(server_address, handler_type)
        self.requests = []
        self.session_closed = False
        self.cancelled_request_ids = []
        self.cancel_event = Event()
        self.behavior = behavior

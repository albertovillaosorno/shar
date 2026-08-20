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
#   - Fake unreal server test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Fake unreal server test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Fake unreal server test module."""

from __future__ import annotations

from http.server import BaseHTTPRequestHandler
from http.server import ThreadingHTTPServer
from threading import Event
from threading import Thread
from typing import NamedTuple
from typing import Self
from typing import TYPE_CHECKING
from typing import cast

from fake_unreal_protocol import FakeUnrealRequestHandler

if TYPE_CHECKING:
    from types import TracebackType

    from mcp.domain.json_types import JsonObject


class FakeUnrealBehavior(NamedTuple):
    """Deterministic protocol switches for one synthetic server."""

    repeat_cursor: bool = False
    advance_cursor: bool = False
    delay_tool_calls: bool = False
    cancellation_delay_seconds: float = 0.0
    empty_toolsets: bool = False
    reject_initialized_notification: bool = False
    reject_session_delete: bool = False
    malformed_initialize_result: bool = False
    redirect_ping: bool = False
    plain_error_ping: bool = False
    plan_execution: bool = False


class FakeUnrealServer:
    """Context-managed synthetic native Unreal MCP endpoint."""

    def __init__(
        self,
        *,
        repeat_cursor: bool = False,
        advance_cursor: bool = False,
        delay_tool_calls: bool = False,
        cancellation_delay_seconds: float = 0.0,
        empty_toolsets: bool = False,
        plan_execution: bool = False,
    ) -> None:
        """Create a stopped server with deterministic behavior."""
        behavior = FakeUnrealBehavior(
            repeat_cursor=repeat_cursor,
            advance_cursor=advance_cursor,
            delay_tool_calls=delay_tool_calls,
            cancellation_delay_seconds=cancellation_delay_seconds,
            empty_toolsets=empty_toolsets,
            plan_execution=plan_execution,
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

    @classmethod
    def with_initialization_failure(
        cls,
        *,
        reject_session_delete: bool = False,
    ) -> Self:
        """Create a server that rejects post-initialize negotiation.

        Returns:
            A stopped server configured for initialization failure.

        """
        server = cls()
        server._server.behavior = FakeUnrealBehavior(
            reject_initialized_notification=True,
            reject_session_delete=reject_session_delete,
        )
        return server

    @classmethod
    def with_malformed_initialize_result(cls) -> Self:
        """Create a server that returns malformed initialize metadata.

        Returns:
            A stopped server configured with an invalid initialize result.

        """
        server = cls()
        server._server.behavior = FakeUnrealBehavior(
            malformed_initialize_result=True,
        )
        return server

    @classmethod
    def with_redirected_ping(cls) -> Self:
        """Create a server whose ping returns one redirect status.

        Returns:
            A stopped server configured with a redirected ping response.

        """
        server = cls()
        server._server.behavior = FakeUnrealBehavior(redirect_ping=True)
        return server

    @classmethod
    def with_plain_error_ping(cls) -> Self:
        """Create a server whose ping returns a plain-text HTTP error.

        Returns:
            A stopped server configured with a non-JSON error response.

        """
        server = cls()
        server._server.behavior = FakeUnrealBehavior(plain_error_ping=True)
        return server

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
    def assets(self) -> dict[str, str]:
        """A snapshot of synthetic native package classes."""
        return dict(self._server.assets)

    @property
    def dirty_assets(self) -> frozenset[str]:
        """A snapshot of synthetic dirty package paths."""
        return frozenset(self._server.dirty_assets)

    @property
    def media_payloads(self) -> dict[str, str]:
        """A snapshot of external movie payloads keyed by object path."""
        return dict(self._server.media_payloads)

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
    assets: dict[str, str]
    dirty_assets: set[str]
    media_payloads: dict[str, str]

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
        self.assets = {}
        self.dirty_assets = set()
        self.media_payloads = {}

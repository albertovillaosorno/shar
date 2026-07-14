# File:
#   - http_exchange.py
# Path:
#   - src/mcp/src/adapters/driven/http_exchange.py
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
#   - Low-level loopback HTTP and SSE exchange mechanics.
# - Must-Not:
#   - Interpret toolsets, catalog policy, or terminal commands.
# - Allows:
#   - Bounded JSON-RPC POST, session DELETE, and SSE decoding.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Owns reliable native MCP wire exchanges.
# - Description:
#   - Separates HTTP framing from MCP application operations.
# - Usage:
#   - Composed by the Streamable HTTP transport adapter.
# - Defaults:
#   - Creates one bounded loopback connection per exchange.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: Streamable HTTP wire exchange adapter
#   - reason: POST, DELETE, and SSE decoding share one connection lifecycle
#   - split: extract SSE parsing if retry or resumption support is added
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Low-level loopback HTTP and SSE exchanges for native Unreal MCP."""

from __future__ import annotations

from http.client import HTTPConnection, HTTPException
from typing import TYPE_CHECKING, NamedTuple

from mcp.src.adapters.driven.http_payload import (
    DEFAULT_MAX_RESPONSE_BYTES,
    read_bounded_body,
    read_http_payload,
    validate_max_response_bytes,
)
from mcp.src.adapters.driven.http_request import (
    DEFAULT_MAX_REQUEST_BYTES,
    encode_json_request,
    validate_max_request_bytes,
)
from mcp.src.adapters.driven.http_status import (
    raise_http_status_error,
)
from mcp.src.adapters.driven.http_timeout import resolve_timeout_seconds
from mcp.src.domain.errors import fail_timeout, fail_transport

if TYPE_CHECKING:
    from mcp.src.domain.endpoint import McpEndpoint
    from mcp.src.domain.json_types import JsonObject
    from mcp.src.domain.session import McpSession

_HTTP_ERROR_MINIMUM = 400
_CONTENT_TYPE_EVENT_STREAM = "text/event-stream"


class HttpExchange(NamedTuple):
    """One completed native MCP HTTP exchange."""

    status: int
    session_id: str | None
    payload: JsonObject | None


class HttpExchangeClient:
    """Bounded loopback HTTP client with SSE result decoding."""

    def __init__(
        self,
        endpoint: McpEndpoint,
        *,
        timeout_seconds: float,
        max_request_bytes: int = DEFAULT_MAX_REQUEST_BYTES,
        max_response_bytes: int = DEFAULT_MAX_RESPONSE_BYTES,
    ) -> None:
        """Create one exchange client.

        Args:
            endpoint: Validated loopback MCP endpoint.
            timeout_seconds: Positive socket-operation timeout.
            max_request_bytes: Positive per-request byte ceiling.
            max_response_bytes: Positive per-response byte ceiling.
        """
        self._endpoint = endpoint
        self._timeout_seconds = resolve_timeout_seconds(timeout_seconds, None)
        self._max_request_bytes = validate_max_request_bytes(max_request_bytes)
        self._max_response_bytes = validate_max_response_bytes(
            max_response_bytes
        )

    def post(
        self,
        *,
        payload: JsonObject,
        request_id: int | None,
        session: McpSession | None,
        timeout_seconds: float | None = None,
    ) -> HttpExchange:
        """Complete one JSON-RPC POST exchange.

        Args:
            payload: Strict JSON-RPC request or notification object.
            request_id: Expected response id, or `None` for a notification.
            session: Initialized session for post-initialize operations.
            timeout_seconds: Optional timeout override for this exchange.

        Returns:
            Status, optional session header, and optional JSON payload.
        """
        exchange_timeout = resolve_timeout_seconds(
            self._timeout_seconds,
            timeout_seconds,
        )
        body = encode_json_request(
            payload,
            max_request_bytes=self._max_request_bytes,
        )
        headers = {
            "Accept": "application/json, text/event-stream",
            "Content-Type": "application/json",
            "Origin": self._endpoint.origin,
        }
        if session is not None:
            headers.update(_session_headers(session))
        connection = self._connection(exchange_timeout)
        try:
            connection.request(
                "POST",
                self._endpoint.path,
                body=body,
                headers=headers,
            )
            response = connection.getresponse()
            response_payload = read_http_payload(
                response,
                request_id,
                max_response_bytes=self._max_response_bytes,
            )
            if response.status >= _HTTP_ERROR_MINIMUM:
                raise_http_status_error(response.status, response_payload)
        except TimeoutError as error:
            request_label = (
                "notification"
                if request_id is None
                else f"request {request_id}"
            )
            message = " ".join(
                (
                    f"MCP {request_label} to {self._endpoint.url} timed out",
                    f"after {exchange_timeout:g} seconds",
                )
            )
            fail_timeout(message, cause=error)
        except (OSError, HTTPException) as error:
            fail_transport(
                f"MCP request to {self._endpoint.url} failed: {error}",
                cause=error,
            )
        else:
            return HttpExchange(
                status=response.status,
                session_id=response.getheader("Mcp-Session-Id"),
                payload=response_payload,
            )
        finally:
            connection.close()

    def delete(
        self,
        *,
        session_id: str,
        protocol_version: str,
    ) -> int:
        """Delete one native MCP session identity.

        Args:
            session_id: Validated session header identity.
            protocol_version: Protocol version sent during initialization.

        Returns:
            Native server HTTP status.
        """
        connection = self._connection()
        try:
            connection.request(
                "DELETE",
                self._endpoint.path,
                headers={
                    "Origin": self._endpoint.origin,
                    "Mcp-Protocol-Version": protocol_version,
                    "Mcp-Session-Id": session_id,
                },
            )
            response = connection.getresponse()
            _ = read_bounded_body(
                response,
                max_response_bytes=self._max_response_bytes,
            )
        except TimeoutError as error:
            fail_timeout(
                "timed out while closing the MCP session",
                cause=error,
            )
        except (OSError, HTTPException) as error:
            fail_transport(
                f"failed to close MCP session: {error}",
                cause=error,
            )
        else:
            return response.status
        finally:
            connection.close()

    def _connection(
        self,
        timeout_seconds: float | None = None,
    ) -> HTTPConnection:
        timeout = resolve_timeout_seconds(
            self._timeout_seconds,
            timeout_seconds,
        )
        return HTTPConnection(
            self._endpoint.authority,
            self._endpoint.port,
            timeout=timeout,
        )


def _session_headers(session: McpSession) -> dict[str, str]:
    return {
        "Mcp-Protocol-Version": session.protocol_version,
        "Mcp-Session-Id": session.session_id,
    }

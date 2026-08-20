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
#   - Http exchange outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Http exchange outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Http exchange outbound adapter."""

from __future__ import annotations

from http.client import HTTPConnection
from http.client import HTTPException
from http.client import HTTPResponse
from typing import NamedTuple
from typing import TYPE_CHECKING

from mcp.adapter_outbound.http_payload import DEFAULT_MAX_RESPONSE_BYTES
from mcp.adapter_outbound.http_payload import read_bounded_body
from mcp.adapter_outbound.http_payload import read_http_error_payload
from mcp.adapter_outbound.http_payload import read_http_payload
from mcp.adapter_outbound.http_payload import validate_max_response_bytes
from mcp.adapter_outbound.http_request import DEFAULT_MAX_REQUEST_BYTES
from mcp.adapter_outbound.http_request import encode_json_request
from mcp.adapter_outbound.http_request import validate_max_request_bytes
from mcp.adapter_outbound.http_status import is_http_success
from mcp.adapter_outbound.http_status import require_http_success
from mcp.adapter_outbound.http_timeout import resolve_timeout_seconds
from mcp.domain.errors import fail_timeout
from mcp.domain.errors import fail_transport

if TYPE_CHECKING:
    from mcp.domain.endpoint import McpEndpoint
    from mcp.domain.json_types import JsonObject
    from mcp.domain.session import McpSession

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
        response_limit = validate_max_response_bytes(max_response_bytes)
        self._max_response_bytes = response_limit

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
            response_payload = _read_exchange_payload(
                response,
                request_id,
                max_response_bytes=self._max_response_bytes,
            )
            require_http_success(
                response.status,
                response_payload,
                request_id=request_id,
            )
        except TimeoutError as error:
            if request_id is None:
                request_label = "notification"
            else:
                request_label = f"request {request_id}"
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


def _read_exchange_payload(
    response: HTTPResponse,
    request_id: int | None,
    *,
    max_response_bytes: int,
) -> JsonObject | None:
    if is_http_success(response.status):
        return read_http_payload(
            response,
            request_id,
            max_response_bytes=max_response_bytes,
        )
    return read_http_error_payload(
        response,
        max_response_bytes=max_response_bytes,
    )


def _session_headers(session: McpSession) -> dict[str, str]:
    return {
        "Mcp-Protocol-Version": session.protocol_version,
        "Mcp-Session-Id": session.session_id,
    }

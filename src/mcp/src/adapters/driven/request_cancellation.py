# File:
#   - request_cancellation.py
# Path:
#   - src/mcp/src/adapters/driven/request_cancellation.py
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
#   - Timeout-triggered MCP cancellation notification handling.
# - Must-Not:
#   - Allocate request IDs, invoke tools, or manage session lifecycle.
# - Allows:
#   - Bounded cancellation POSTs and timeout error enrichment.
# - Split-When:
#   - Cancellation acknowledgement gains independent protocol states.
# - Merge-When:
#   - Another adapter module owns the same cancellation invariant.
# - Summary:
#   - Cancels timed-out native Unreal MCP requests.
# - Description:
#   - Preserves the original timeout while reporting cancellation failure.
# - Usage:
#   - Called by the serialized transport after a tool-call timeout.
# - Defaults:
#   - Uses a five-second cancellation grace window.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - false
#
"""Timeout-triggered native Unreal MCP request cancellation."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.src.adapters.driven.json_rpc_request import build_json_rpc_request
from mcp.src.domain.errors import UnrealMcpError, fail_protocol, fail_timeout

if TYPE_CHECKING:
    from mcp.src.adapters.driven.http_exchange import HttpExchangeClient
    from mcp.src.domain.errors import RequestTimeoutError
    from mcp.src.domain.session import McpSession

_HTTP_ACCEPTED = 202
_CANCELLATION_TIMEOUT_SECONDS = 5.0


def cancel_timed_out_request(
    exchange: HttpExchangeClient,
    session: McpSession,
    request_id: int,
    timeout_error: RequestTimeoutError,
) -> None:
    """Cancel one timed-out request without hiding its timeout.

    Args:
        exchange: Active low-level HTTP exchange client.
        session: Active initialized MCP session.
        request_id: Timed-out JSON-RPC request identity.
        timeout_error: Original timeout failure to preserve.
    """
    try:
        cancellation = exchange.post(
            payload=build_json_rpc_request(
                method="notifications/cancelled",
                params={"requestId": request_id},
                request_id=None,
            ),
            request_id=None,
            session=session,
            timeout_seconds=_CANCELLATION_TIMEOUT_SECONDS,
        )
        if cancellation.status != _HTTP_ACCEPTED:
            fail_protocol("cancelled notification did not return HTTP 202")
    except UnrealMcpError as cancellation_error:
        cancellation_message = " ".join(
            (
                f"cancellation of request {request_id} failed:",
                str(cancellation_error),
            )
        )
        message = "; ".join((str(timeout_error), cancellation_message))
        fail_timeout(message, cause=timeout_error)

# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Request cancellation outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Request cancellation outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Request cancellation outbound adapter."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.adapter_outbound.json_rpc_request import build_json_rpc_request
from mcp.domain.errors import UnrealMcpError
from mcp.domain.errors import fail_protocol
from mcp.domain.errors import fail_timeout

if TYPE_CHECKING:
    from mcp.adapter_outbound.http_exchange import HttpExchangeClient
    from mcp.domain.errors import RequestTimeoutError
    from mcp.domain.session import McpSession

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

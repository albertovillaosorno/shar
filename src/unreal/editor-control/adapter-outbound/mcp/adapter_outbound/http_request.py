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
#   - Http request outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Http request outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Http request outbound adapter."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

from mcp.domain.errors import ProtocolError
from mcp.domain.errors import fail_configuration
from mcp.domain.errors import fail_transport
from mcp.domain.json_types import require_json_object

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject

DEFAULT_MAX_REQUEST_BYTES = 64 * 1024 * 1024


def validate_max_request_bytes(value: int) -> int:
    """Return one positive request byte ceiling.

    Returns:
        The validated request byte ceiling.

    """
    if value <= 0:
        fail_configuration("max_request_bytes must be positive")
    return value


def encode_json_request(
    payload: JsonObject,
    *,
    max_request_bytes: int,
) -> bytes:
    """Encode one strict JSON request within its byte ceiling.

    Args:
        payload: Strict JSON request or notification object.
        max_request_bytes: Maximum serialized UTF-8 body size.

    Returns:
        Deterministic compact UTF-8 JSON bytes.

    """
    limit = validate_max_request_bytes(max_request_bytes)
    try:
        normalized = require_json_object(payload, context="MCP request")
        serialized = json.dumps(
            normalized,
            ensure_ascii=False,
            allow_nan=False,
            separators=(",", ":"),
        )
        body = serialized.encode("utf-8")
    except ProtocolError as error:
        fail_transport(
            f"MCP request contains non-finite or invalid JSON: {error}",
            cause=error,
        )
    except (RecursionError, TypeError, UnicodeEncodeError, ValueError) as error:
        fail_transport(
            "MCP request contains a non-finite value or cycle",
            cause=error,
        )
    if len(body) > limit:
        fail_transport(f"MCP request exceeded {limit} bytes")
    return body

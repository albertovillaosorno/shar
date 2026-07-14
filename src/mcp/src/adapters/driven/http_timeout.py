# File:
#   - http_timeout.py
# Path:
#   - src/mcp/src/adapters/driven/http_timeout.py
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
#   - Positive timeout resolution for bounded HTTP exchanges.
# - Must-Not:
#   - Open sockets, interpret MCP payloads, or manage sessions.
# - Allows:
#   - Selecting a per-exchange override over one validated default.
# - Split-When:
#   - Timeout classes gain independent retry or deadline semantics.
# - Merge-When:
#   - Another adapter module owns the same timeout-selection invariant.
# - Summary:
#   - Resolves bounded HTTP timeout values.
# - Description:
#   - Rejects non-positive defaults and overrides before socket creation.
# - Usage:
#   - Called by the loopback HTTP exchange adapter.
# - Defaults:
#   - Uses the configured client timeout when no override is supplied.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - false
#
"""Timeout selection for bounded native MCP HTTP exchanges."""

from __future__ import annotations

import math

from mcp.src.domain.errors import fail_configuration


def resolve_timeout_seconds(
    default_seconds: float,
    override_seconds: float | None,
) -> float:
    """Return one positive exchange timeout.

    Returns:
        The override when present, otherwise the configured default.
    """
    resolved = default_seconds if override_seconds is None else override_seconds
    if not math.isfinite(resolved) or resolved <= 0:
        fail_configuration("exchange timeout must be finite and positive")
    return resolved

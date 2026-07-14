# File:
#   - session.py
# Path:
#   - src/mcp/src/domain/session.py
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
#   - Negotiated MCP session identity and protocol version.
# - Must-Not:
#   - Open connections or mutate session transport state.
# - Allows:
#   - Immutable session values returned by one transport.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Carries the minimum state required after initialization.
# - Description:
#   - Separates protocol state from the HTTP implementation.
# - Usage:
#   - Passed from application use cases to the transport port.
# - Defaults:
#   - No session exists before successful initialization.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Negotiated native Unreal MCP session values."""

from typing import NamedTuple


class McpSession(NamedTuple):
    """One initialized native Unreal MCP session."""

    session_id: str
    protocol_version: str
    server_name: str
    server_version: str

# File:
#   - tool_identity.py
# Path:
#   - src/mcp/src/domain/tool_identity.py
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
#   - Canonical native Unreal tool identities and invocation leaf names.
# - Must-Not:
#   - Import transport, command-line, filesystem, or Unreal implementation APIs.
# - Allows:
#   - Normalizing qualified and leaf tool names against one toolset identity.
# - Split-When:
#   - Toolset and tool identity grammars gain independent native contracts.
# - Merge-When:
#   - Another domain module owns the same identity invariant.
# - Summary:
#   - Keeps discovery identities and invocation names interoperable.
# - Description:
#   - Accepts copyable qualified names while emitting native call leaf names.
# - Usage:
#   - Used by schema parsing and Toolset Registry invocation orchestration.
# - Defaults:
#   - Qualified tools must belong to the explicitly selected toolset.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Native Unreal MCP tool identity normalization."""

from __future__ import annotations

from mcp.src.domain.errors import fail_protocol

_MAX_IDENTITY_BYTES = 4_096


def canonical_tool_identity(toolset_name: str, tool_name: str) -> str:
    """Return one fully qualified tool identity.

    Args:
        toolset_name: Explicit native Toolset Registry identity.
        tool_name: Native leaf name or matching fully qualified identity.

    Returns:
        The canonical `<toolset>.<leaf>` tool identity.
    """
    toolset = _validated_identity(toolset_name, context="toolset name")
    tool = _validated_identity(tool_name, context="tool name")
    prefix = f"{toolset}."
    if tool.startswith(prefix):
        leaf = tool[len(prefix) :]
    elif "." in tool:
        fail_protocol(
            f"qualified tool `{tool}` does not belong to toolset `{toolset}`"
        )
    else:
        leaf = tool
    if not leaf or "." in leaf:
        fail_protocol(f"tool `{tool}` must resolve to one leaf name")
    return f"{toolset}.{leaf}"


def native_tool_leaf(toolset_name: str, tool_name: str) -> str:
    """Return the leaf name accepted by the native `call_tool` meta-tool.

    Args:
        toolset_name: Explicit native Toolset Registry identity, or empty.
        tool_name: Native leaf name or matching fully qualified identity.

    Returns:
        The native invocation leaf, or the unchanged name for global lookup.
    """
    tool = _validated_identity(tool_name, context="tool name")
    toolset = toolset_name.strip()
    if not toolset:
        return tool
    qualified = canonical_tool_identity(toolset, tool)
    return qualified[len(toolset) + 1 :]


def _validated_identity(value: str, *, context: str) -> str:
    normalized = value.strip()
    if not normalized:
        fail_protocol(f"{context} must not be empty")
    if not normalized.isprintable():
        fail_protocol(f"{context} must contain printable characters only")
    if len(normalized.encode()) > _MAX_IDENTITY_BYTES:
        fail_protocol(f"{context} exceeded its byte limit")
    if any(character.isspace() for character in normalized):
        fail_protocol(f"{context} must not contain whitespace")
    if normalized.startswith(".") or normalized.endswith("."):
        fail_protocol(f"{context} must not start or end with a period")
    if ".." in normalized:
        fail_protocol(f"{context} must not contain an empty segment")
    return normalized

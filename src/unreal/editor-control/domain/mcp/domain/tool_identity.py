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
#   - Tool identity domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Tool identity domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Tool identity domain module."""

from __future__ import annotations

from mcp.domain.errors import fail_protocol

_MAX_IDENTITY_BYTES = 4_096


def validated_toolset_identity(value: str) -> str:
    """Return one canonical native Toolset Registry identity.

    Args:
        value: Candidate toolset identity.

    Returns:
        The normalized bounded identity.

    """
    return _validated_identity(value, context="toolset name")


def canonical_tool_identity(toolset_name: str, tool_name: str) -> str:
    """Return one fully qualified tool identity.

    Args:
        toolset_name: Explicit native Toolset Registry identity.
        tool_name: Native leaf name or matching fully qualified identity.

    Returns:
        The canonical `<toolset>.<leaf>` tool identity.

    """
    toolset = validated_toolset_identity(toolset_name)
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

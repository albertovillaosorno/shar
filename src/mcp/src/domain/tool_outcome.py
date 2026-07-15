# File:
#   - tool_outcome.py
# Path:
#   - src/mcp/src/domain/tool_outcome.py
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
#   - Native MCP tool-result normalization and success semantics.
# - Must-Not:
#   - Parse toolset schemas, open transports, or render terminal output.
# - Allows:
#   - Text projection, structured JSON preservation, and content validation.
# - Split-When:
#   - Individual MCP content block kinds require independent validation.
# - Merge-When:
#   - Another domain module owns the same result-normalization invariants.
# - Summary:
#   - Normalizes native Unreal MCP tool outcomes.
# - Description:
#   - Preserves raw and structured results while validating content blocks.
# - Usage:
#   - Called by the Streamable HTTP adapter after JSON-RPC validation.
# - Defaults:
#   - Missing or malformed content fails closed.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - false
#
"""Native Unreal MCP tool-result domain values."""

from __future__ import annotations

from typing import NamedTuple

from mcp.src.domain.errors import fail_protocol, fail_tool_call
from mcp.src.domain.json_types import JsonObject, JsonValue, require_json_object

_MAX_CONTENT_BLOCKS = 100_000


class ToolCallOutcome(NamedTuple):
    """Normalized native tool outcome."""

    raw: JsonObject
    text: str
    structured_content: JsonValue | None
    is_error: bool

    def require_success(self) -> ToolCallOutcome:
        """Return this outcome or raise its native tool error.

        Returns:
            This outcome when the native call succeeded.
        """
        if self.is_error:
            message = (
                _escape_diagnostic_controls(self.text)
                if self.text
                else "Unreal tool call failed"
            )
            fail_tool_call(message)
        return self


def _escape_diagnostic_controls(value: str) -> str:
    """Render nonprintable tool text without changing printable evidence.

    Returns:
        Text with nonprintable characters represented by visible escapes.
    """
    return "".join(
        character
        if character.isprintable()
        else character.encode("unicode_escape").decode("ascii")
        for character in value
    )


def parse_tool_outcome(value: object) -> ToolCallOutcome:
    """Normalize one MCP `tools/call` outcome object.

    Args:
        value: Untrusted JSON-RPC result value.

    Returns:
        A validated native tool outcome.
    """
    outcome = require_json_object(value, context="tools/call outcome")
    is_error = outcome.get("isError", False)
    if not isinstance(is_error, bool):
        fail_protocol("tools/call outcome: isError must be boolean")
    return ToolCallOutcome(
        raw=outcome,
        text=_extract_text(outcome),
        structured_content=outcome.get("structuredContent"),
        is_error=is_error,
    )


def _extract_text(outcome: JsonObject) -> str:
    raw_content = outcome.get("content")
    if not isinstance(raw_content, list):
        fail_protocol("tools/call outcome: content must be an array")
    if len(raw_content) > _MAX_CONTENT_BLOCKS:
        fail_protocol("tools/call outcome exceeded its content block limit")
    parts: list[str] = []
    for index, raw_item in enumerate(raw_content):
        item = require_json_object(
            raw_item,
            context=f"tools/call outcome.content[{index}]",
        )
        content_type = item.get("type")
        if not isinstance(content_type, str) or not content_type:
            message = " ".join(
                (
                    f"tools/call outcome.content[{index}]:",
                    "type must be non-empty text",
                )
            )
            fail_protocol(message)
        if content_type != "text":
            continue
        text = item.get("text")
        if not isinstance(text, str):
            fail_protocol("tools/call outcome: text content must contain text")
        parts.append(text)
    return "\n".join(parts)

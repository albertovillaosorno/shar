# File:
#   - test_tool_outcome.py
# Path:
#   - src/mcp/tests/test_tool_outcome.py
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
#   - Regression evidence for native MCP tool-result normalization.
# - Must-Not:
#   - Open sockets, invoke Unreal, or parse toolset schemas.
# - Allows:
#   - Text, structured JSON, non-text content, and error fixtures.
# - Split-When:
#   - Individual content block kinds require dedicated fixture families.
# - Merge-When:
#   - Another test module owns the same tool-result contract.
# - Summary:
#   - Guards native Unreal MCP result normalization.
# - Description:
#   - Proves malformed result blocks fail before application use.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Requires a content array on every tool result.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: native MCP tool-result regression tests
#   - reason: text, structured, non-text, and error cases share one boundary
#   - split: split by content block kind when another family is implemented
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Regression tests for native Unreal MCP tool outcomes."""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest
from mcp.src.domain.errors import ProtocolError, ToolCallError
from mcp.src.domain.tool_outcome import parse_tool_outcome

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject

_MALFORMED_CASES: tuple[tuple[JsonObject, str], ...] = (
    ({"isError": False}, "content must be an array"),
    ({"content": [{}]}, "type must be non-empty text"),
    ({"content": [{"type": "text"}]}, "must contain text"),
)


def test_tool_outcome_preserves_text_and_structured_content() -> None:
    """Text fallback and structured JSON remain independently available."""
    outcome = parse_tool_outcome(
        {
            "content": [{"type": "text", "text": "created"}],
            "isError": False,
        }
    )
    assert outcome.text == "created"
    assert outcome.structured_content is None
    assert outcome.require_success() is outcome

    structured = parse_tool_outcome(
        {
            "content": [
                {
                    "type": "text",
                    "text": '{"asset":"/Game/Test"}',
                }
            ],
            "structuredContent": {
                "asset": "/Game/Test",
                "values": [1, 2, 3],
            },
            "isError": False,
        }
    )
    assert structured.text == '{"asset":"/Game/Test"}'
    assert structured.structured_content == {
        "asset": "/Game/Test",
        "values": [1, 2, 3],
    }

    structured_array = parse_tool_outcome(
        {
            "content": [],
            "structuredContent": ["first", "second"],
        }
    )
    assert not structured_array.text
    assert structured_array.structured_content == ["first", "second"]


def test_tool_outcome_native_error_fails_on_demand() -> None:
    """Native `isError` remains data until success is required."""
    failed = parse_tool_outcome(
        {
            "content": [{"type": "text", "text": "blocked"}],
            "isError": True,
        }
    )

    with pytest.raises(ToolCallError, match="blocked"):
        _ = failed.require_success()


def test_tool_outcome_native_error_escapes_diagnostic_controls() -> None:
    """Native error text cannot inject terminal control characters."""
    failed = parse_tool_outcome(
        {
            "content": [
                {
                    "type": "text",
                    "text": "blocked\n\x1b[2J",
                }
            ],
            "isError": True,
        }
    )

    assert failed.text == "blocked\n\x1b[2J"
    with pytest.raises(ToolCallError) as captured:
        _ = failed.require_success()
    assert str(captured.value) == r"blocked\n\x1b[2J"


@pytest.mark.parametrize(
    ("outcome", "message"),
    _MALFORMED_CASES,
)
def test_tool_outcome_rejects_malformed_content_blocks(
    outcome: JsonObject,
    message: str,
) -> None:
    """Malformed MCP content blocks fail at the domain boundary."""
    with pytest.raises(ProtocolError, match=message):
        _ = parse_tool_outcome(outcome)


def test_tool_outcome_rejects_excessive_content_blocks(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """One response cannot force unbounded content-block traversal."""
    monkeypatch.setattr(
        "mcp.src.domain.tool_outcome._MAX_CONTENT_BLOCKS",
        2,
        raising=False,
    )
    outcome: JsonObject = {
        "content": [
            {"type": "text", "text": "first"},
            {"type": "image", "data": "AA=="},
            {"type": "text", "text": "third"},
        ]
    }

    with pytest.raises(ProtocolError, match="content block limit"):
        _ = parse_tool_outcome(outcome)


def test_tool_outcome_rejects_excessive_projected_text_bytes(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Text projection cannot allocate another unbounded response copy."""
    monkeypatch.setattr(
        "mcp.src.domain.tool_outcome._MAX_PROJECTED_TEXT_BYTES",
        4,
        raising=False,
    )
    outcome: JsonObject = {
        "content": [
            {"type": "text", "text": "ab"},
            {"type": "text", "text": "cd"},
        ]
    }

    with pytest.raises(ProtocolError, match="text byte limit"):
        _ = parse_tool_outcome(outcome)


def test_tool_outcome_preserves_valid_non_text_content() -> None:
    """Non-text blocks remain raw without inventing a text projection."""
    outcome = parse_tool_outcome(
        {
            "content": [
                {
                    "type": "image",
                    "data": "AA==",
                    "mimeType": "image/png",
                }
            ]
        }
    )

    assert not outcome.text
    assert outcome.raw["content"] == [
        {
            "type": "image",
            "data": "AA==",
            "mimeType": "image/png",
        }
    ]

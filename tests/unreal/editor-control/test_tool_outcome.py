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
#   - Test tool outcome test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test tool outcome test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test tool outcome test module."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.domain.errors import ProtocolError
from mcp.domain.errors import ToolCallError
from mcp.domain.tool_outcome import parse_tool_outcome
import pytest

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject

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
        "mcp.domain.tool_outcome._MAX_CONTENT_BLOCKS",
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
        "mcp.domain.tool_outcome._MAX_PROJECTED_TEXT_BYTES",
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

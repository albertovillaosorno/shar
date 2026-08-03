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
#   - Test tool identity test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test tool identity test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test tool identity test module."""

from __future__ import annotations

from mcp.domain.errors import ProtocolError
from mcp.domain.tool_identity import canonical_tool_identity
from mcp.domain.tool_identity import native_tool_leaf
import pytest

_TOOLSET = "EditorToolset.EditorAppToolset"
_FULL_TOOL = f"{_TOOLSET}.GetOpenAssets"


def test_tool_identity_accepts_leaf_or_matching_qualified_name() -> None:
    assert canonical_tool_identity(_TOOLSET, "GetOpenAssets") == _FULL_TOOL
    assert canonical_tool_identity(_TOOLSET, _FULL_TOOL) == _FULL_TOOL
    assert native_tool_leaf(_TOOLSET, "GetOpenAssets") == "GetOpenAssets"
    assert native_tool_leaf(_TOOLSET, _FULL_TOOL) == "GetOpenAssets"


def test_tool_identity_trims_boundary_whitespace() -> None:
    assert (
        canonical_tool_identity(
            f"  {_TOOLSET}  ",
            f"  {_FULL_TOOL}  ",
        )
        == _FULL_TOOL
    )


def test_tool_identity_rejects_control_characters() -> None:
    """Native identity keys cannot contain hidden non-printable characters."""
    for name in ("Get\x00OpenAssets", "Get\x07OpenAssets"):
        with pytest.raises(ProtocolError, match="printable"):
            _ = canonical_tool_identity(_TOOLSET, name)


@pytest.mark.parametrize(
    ("toolset_name", "tool_name"),
    [
        ("abcde", "tool"),
        ("Toolset.Name", "abcde"),
    ],
)
def test_tool_identity_rejects_excessive_utf8_bytes(
    toolset_name: str,
    tool_name: str,
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Native identities cannot become oversized routes or path segments."""
    monkeypatch.setattr(
        "mcp.domain.tool_identity._MAX_IDENTITY_BYTES",
        4,
        raising=False,
    )

    with pytest.raises(ProtocolError, match="byte limit"):
        _ = canonical_tool_identity(toolset_name, tool_name)


def test_tool_identity_rejects_mismatched_or_malformed_names() -> None:
    with pytest.raises(ProtocolError, match="does not belong"):
        _ = native_tool_leaf(
            _TOOLSET,
            "OtherToolset.OtherToolset.GetOpenAssets",
        )
    with pytest.raises(ProtocolError, match="one leaf name"):
        _ = canonical_tool_identity(_TOOLSET, f"{_TOOLSET}.Nested.Tool")
    with pytest.raises(ProtocolError, match="whitespace"):
        _ = canonical_tool_identity(_TOOLSET, "Get Open Assets")

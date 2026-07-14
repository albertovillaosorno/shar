# File:
#   - test_tool_identity.py
# Path:
#   - src/mcp/tests/test_tool_identity.py
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
#   - Regression tests for native tool identity normalization.
# - Must-Not:
#   - Open sockets, invoke Unreal, or depend on adapter behavior.
# - Allows:
#   - Pure qualified-name, leaf-name, and mismatch fixtures.
# - Split-When:
#   - Toolset and tool identity grammars require separate fixtures.
# - Merge-When:
#   - Another pure domain test module owns these identity invariants.
# - Summary:
#   - Guards copyable discovery identities and native invocation leaf names.
# - Description:
#   - Proves qualified skills remain executable through the terminal client.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses synthetic public-safe Unreal Toolset Registry identities.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Regression tests for native Unreal MCP tool identities."""

from __future__ import annotations

import pytest
from mcp.src.domain.errors import ProtocolError
from mcp.src.domain.tool_identity import (
    canonical_tool_identity,
    native_tool_leaf,
)

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

# File:
#   - test_domain.py
# Path:
#   - src/mcp/tests/test_domain.py
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
#   - Regression tests for pure MCP translator domain contracts.
# - Must-Not:
#   - Open sockets, start Unreal, or inspect proprietary source.
# - Allows:
#   - Endpoint, catalog, schema, and outcome validation tests.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Guards fail-closed translator domain behavior.
# - Description:
#   - Exercises domain invariants without adapter dependencies.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - All fixtures are synthetic and public-safe.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: pure domain regression tests
#   - reason: endpoint, catalog, and outcome tests require no adapter fixtures
#   - split: split by domain aggregate when any fixture family grows
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
from __future__ import annotations

import json
from typing import TYPE_CHECKING

import pytest
from mcp.src.domain.catalog import (
    parse_toolset_catalog,
    parse_toolset_definition,
)
from mcp.src.domain.endpoint import McpEndpoint
from mcp.src.domain.errors import EndpointValidationError, ProtocolError

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject


def test_endpoint_accepts_only_explicit_loopback_http() -> None:
    endpoint = McpEndpoint.parse("http://127.0.0.1:8123/mcp")
    assert endpoint.url == "http://127.0.0.1:8123/mcp"

    with pytest.raises(EndpointValidationError):
        _ = McpEndpoint.parse("https://127.0.0.1:8123/mcp")
    with pytest.raises(EndpointValidationError):
        _ = McpEndpoint.parse("http://example.com:8123/mcp")
    with pytest.raises(EndpointValidationError):
        _ = McpEndpoint.parse("http://127.0.0.1/")


def test_endpoint_rejects_dns_alias_for_loopback_boundary() -> None:
    """A mutable hostname must not stand in for a literal loopback address."""
    with pytest.raises(EndpointValidationError):
        _ = McpEndpoint.parse("http://localhost:8123/mcp")

    endpoint = McpEndpoint.parse("http://[::1]:8123/mcp")
    assert endpoint.url == "http://[::1]:8123/mcp"


def test_toolset_catalog_preserves_multiline_descriptions() -> None:
    """Qualified headers delimit toolsets and preserve nested bullets."""
    catalog_text = """- EditorToolset.EditorToolset: Editor operations

Provides:
- create_asset: synthetic asset creation
- UMGToolSet.UMGToolSet: UI operations
"""
    summaries = parse_toolset_catalog(catalog_text)

    assert tuple(item.name for item in summaries) == (
        "EditorToolset.EditorToolset",
        "UMGToolSet.UMGToolSet",
    )
    assert (
        summaries[0].description
        == """Editor operations

Provides:
- create_asset: synthetic asset creation"""
    )


def test_toolset_catalog_rejects_duplicate_or_orphan_lines() -> None:
    """Duplicate registry identities and orphan text fail closed."""
    duplicate = """- EditorToolset.EditorToolset: First
- EditorToolset.EditorToolset: Second
"""
    with pytest.raises(ProtocolError, match="duplicate"):
        _ = parse_toolset_catalog(duplicate)
    with pytest.raises(ProtocolError, match="expected qualified header"):
        _ = parse_toolset_catalog("Provides:\n")


@pytest.mark.parametrize(
    "second_name",
    ["create_asset", "EditorToolset.create_asset"],
)
def test_toolset_schema_rejects_duplicate_canonical_tool_identity(
    second_name: str,
) -> None:
    schema: JsonObject = {
        "tools": [
            {"name": "create_asset", "inputSchema": {}},
            {"name": second_name, "inputSchema": {}},
        ]
    }

    with pytest.raises(ProtocolError, match="duplicate tool identity"):
        _ = parse_toolset_definition("EditorToolset", json.dumps(schema))


def test_toolset_schema_is_lossless() -> None:
    schema = {
        "description": "Editor operations",
        "tools": [
            {
                "name": "create_asset",
                "description": "Create one asset.",
                "inputSchema": {"type": "object"},
                "outputSchema": {"type": "object"},
            }
        ],
    }
    definition = parse_toolset_definition(
        "EditorToolset",
        json.dumps(schema),
    )
    assert definition.name == "EditorToolset"
    assert definition.tools[0].name == "EditorToolset.create_asset"
    assert definition.raw_schema == schema

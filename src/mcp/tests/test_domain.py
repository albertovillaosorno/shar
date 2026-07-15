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
import sys
from typing import TYPE_CHECKING, cast

import pytest
from mcp.src.domain.catalog import (
    parse_toolset_catalog,
    parse_toolset_definition,
)
from mcp.src.domain.endpoint import McpEndpoint
from mcp.src.domain.errors import EndpointValidationError, ProtocolError
from mcp.src.domain.json_types import normalize_json

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject


def test_endpoint_accepts_only_explicit_loopback_http() -> None:
    endpoint = McpEndpoint.parse("http://127.0.0.1:8123/mcp")
    assert endpoint.url == "http://127.0.0.1:8123/mcp"
    encoded = McpEndpoint.parse("http://127.0.0.1:8123/mcp%20v2")
    assert encoded.path == "/mcp%20v2"

    with pytest.raises(EndpointValidationError):
        _ = McpEndpoint.parse("https://127.0.0.1:8123/mcp")
    with pytest.raises(EndpointValidationError):
        _ = McpEndpoint.parse("http://example.com:8123/mcp")
    with pytest.raises(EndpointValidationError):
        _ = McpEndpoint.parse("http://127.0.0.1/")


def test_endpoint_rejects_explicit_zero_port() -> None:
    """An invalid explicit port must not be replaced by the default port."""
    with pytest.raises(
        EndpointValidationError,
        match="port must be between 1 and 65535",
    ):
        _ = McpEndpoint.parse("http://127.0.0.1:0/mcp")

    endpoint = McpEndpoint.parse("http://127.0.0.1/mcp")
    assert endpoint.port == 8000


def test_endpoint_rejects_noncanonical_text_and_empty_port() -> None:
    """Endpoint parsing must not normalize malformed operator input."""
    for value in (
        " http://127.0.0.1:8000/mcp",
        "http://127.0.0.1:8000/mcp\n",
        "\x00http://127.0.0.1:8000/mcp",
        "\x01http://127.0.0.1:8000/mcp",
        "http://127.0.0.1:8000/\x00mcp",
        "http://127.0.0.1:8000/☃",
        "http://127.0.0.1:8000/mcp%",
        "http://127.0.0.1:8000/mcp%2",
        "http://127.0.0.1:8000/mcp%GG",
        "http://127.0.0.1:8000/mcp?",
        "http://127.0.0.1:8000/mcp#",
        "http://127.0.0.1:/mcp",
        "http://[::1]:/mcp",
    ):
        with pytest.raises(EndpointValidationError):
            _ = McpEndpoint.parse(value)


def test_endpoint_rejects_dns_alias_for_loopback_boundary() -> None:
    """A mutable hostname must not stand in for a literal loopback address."""
    with pytest.raises(EndpointValidationError):
        _ = McpEndpoint.parse("http://localhost:8123/mcp")

    endpoint = McpEndpoint.parse("http://[::1]:8123/mcp")
    assert endpoint.url == "http://[::1]:8123/mcp"


def test_endpoint_wraps_malformed_ipv6_url_error() -> None:
    """Malformed IPv6 syntax must remain a typed endpoint failure."""
    with pytest.raises(EndpointValidationError, match="URL is malformed"):
        _ = McpEndpoint.parse("http://[::1:8123/mcp")


def test_json_normalizer_rejects_non_finite_numbers() -> None:
    """Recursive JSON validation rejects non-standard numeric constants."""
    for value in (float("nan"), float("inf"), float("-inf")):
        with pytest.raises(
            ProtocolError,
            match=r"payload\.value: JSON number must be finite",
        ):
            _ = normalize_json({"value": value}, context="payload")


def test_json_key_context_escapes_control_text() -> None:
    """Nested validation contexts remain reversible and single-line."""
    with pytest.raises(ProtocolError) as caught:
        _ = normalize_json(
            {"bad\ninjected": float("nan")},
            context="payload",
        )

    assert str(caught.value) == (
        r"payload.bad\ninjected: JSON number must be finite"
    )
    assert "\n" not in str(caught.value)


def test_json_normalizer_handles_surrogate_pairs() -> None:
    """Valid pairs become scalars while lone surrogates fail closed."""
    pair = cast("str", json.loads('"\\ud83d\\ude00"'))
    normalized = normalize_json({"value": pair}, context="payload")
    assert isinstance(normalized, dict)
    normalized_value = normalized["value"]
    assert isinstance(normalized_value, str)
    assert normalized_value.encode() == bytes.fromhex("f09f9880")

    for escaped in ('"\\ud800"', '"\\udc00"'):
        decoded = cast("str", json.loads(escaped))
        with pytest.raises(ProtocolError, match="unpaired Unicode surrogate"):
            _ = normalize_json(
                {"value": decoded},
                context="payload",
            )


def test_json_normalizer_rejects_excessive_nesting() -> None:
    """Deep recursive values fail as protocol data rather than Python errors."""
    value: object = 0
    for _ in range(sys.getrecursionlimit() + 100):
        value = [value]

    with pytest.raises(ProtocolError, match="JSON nesting is too deep"):
        _ = normalize_json(value, context="payload")


def test_toolset_definition_rejects_duplicate_json_keys() -> None:
    """Live schemas cannot silently replace an earlier object member."""
    with pytest.raises(ProtocolError, match="duplicate JSON key: tools"):
        _ = parse_toolset_definition(
            "EditorToolset",
            '{"tools":[],"tools":[]}',
        )


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


def test_toolset_catalog_rejects_excessive_registry_entries(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """One bounded response cannot amplify into unbounded schema requests."""
    monkeypatch.setattr(
        "mcp.src.domain.catalog._MAX_TOOLSET_SUMMARIES",
        2,
        raising=False,
    )
    catalog_text = """- FirstToolset.FirstToolset:
- SecondToolset.SecondToolset:
- ThirdToolset.ThirdToolset:
"""

    with pytest.raises(ProtocolError, match="toolset limit"):
        _ = parse_toolset_catalog(catalog_text)


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

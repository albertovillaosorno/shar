# File:
#   - test_http_request.py
# Path:
#   - src/mcp/tests/test_http_request.py
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
#   - Regression evidence for bounded MCP JSON request serialization.
# - Must-Not:
#   - Open sockets, invoke Unreal, or inspect response behavior.
# - Allows:
#   - Compact JSON, UTF-8 byte, and limit-validation fixtures.
# - Split-When:
#   - Streaming or compressed request bodies are introduced.
# - Merge-When:
#   - Another test module owns the same request byte-limit contract.
# - Summary:
#   - Guards bounded native MCP request encoding.
# - Description:
#   - Proves request ceilings operate on serialized UTF-8 bytes.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses deliberately small byte ceilings for deterministic tests.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
#
# Large file:
#   - false
#
"""Regression tests for bounded native MCP HTTP request encoding."""

from __future__ import annotations

from typing import TYPE_CHECKING

import pytest
from mcp.src.adapters.driven.http_request import (
    encode_json_request,
    validate_max_request_bytes,
)
from mcp.src.domain.errors import ConfigurationError, TransportError

if TYPE_CHECKING:
    from mcp.src.domain.json_types import JsonObject


def test_request_encoding_is_compact_and_deterministic() -> None:
    """Strict JSON requests use compact UTF-8 serialization."""
    payload: JsonObject = {
        "jsonrpc": "2.0",
        "id": 1,
        "params": {"label": "SHAR"},
    }
    expected = b'{"jsonrpc":"2.0","id":1,"params":{"label":"SHAR"}}'

    assert (
        encode_json_request(
            payload,
            max_request_bytes=len(expected),
        )
        == expected
    )


def test_request_limit_counts_utf8_bytes_not_characters() -> None:
    """A multibyte value is measured after UTF-8 serialization."""
    payload: JsonObject = {"value": "é"}
    encoded = b'{"value":"\xc3\xa9"}'

    assert (
        encode_json_request(
            payload,
            max_request_bytes=len(encoded),
        )
        == encoded
    )
    with pytest.raises(TransportError, match="exceeded"):
        _ = encode_json_request(
            payload,
            max_request_bytes=len(encoded) - 1,
        )


def test_request_limit_must_be_positive() -> None:
    """Invalid request ceilings fail before serialization."""
    with pytest.raises(ConfigurationError, match="must be positive"):
        _ = validate_max_request_bytes(0)

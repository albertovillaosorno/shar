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
#   - Test http request test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test http request test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test http request test module."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING
from typing import cast

from mcp.adapter_outbound.http_request import encode_json_request
from mcp.adapter_outbound.http_request import validate_max_request_bytes
from mcp.domain.errors import ConfigurationError
from mcp.domain.errors import TransportError
import pytest

if TYPE_CHECKING:
    from mcp.domain.json_types import JsonObject


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


def test_request_encoding_rejects_non_finite_numbers() -> None:
    """JSON-RPC requests cannot contain NaN or infinity literals."""
    for value in (float("nan"), float("inf"), float("-inf")):
        payload: JsonObject = {"value": value}
        with pytest.raises(TransportError, match="non-finite"):
            _ = encode_json_request(
                payload,
                max_request_bytes=128,
            )


def test_request_encoding_normalizes_or_rejects_surrogates() -> None:
    """Valid pairs encode as scalars and lone surrogates fail as transport."""
    pair = cast("str", json.loads('"\\ud83d\\ude00"'))
    encoded = encode_json_request(
        {"value": pair},
        max_request_bytes=128,
    )
    assert encoded == bytes.fromhex("7b2276616c7565223a22f09f9880227d")

    lone = cast("str", json.loads('"\\ud800"'))
    with pytest.raises(TransportError, match="unpaired Unicode surrogate"):
        _ = encode_json_request(
            {"value": lone},
            max_request_bytes=128,
        )


def test_request_limit_must_be_positive() -> None:
    """Invalid request ceilings fail before serialization."""
    with pytest.raises(ConfigurationError, match="must be positive"):
        _ = validate_max_request_bytes(0)

# File:
#   - json_types.py
# Path:
#   - src/mcp/src/domain/json_types.py
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
#   - Recursive JSON value validation and normalization.
# - Must-Not:
#   - Interpret MCP, Unreal, HTTP, or command semantics.
# - Allows:
#   - Pure JSON aliases and fail-closed validation.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Normalizes untrusted values into strict JSON types.
# - Description:
#   - Prevents untyped or non-JSON values crossing boundaries.
# - Usage:
#   - Called after parsing and before transport serialization.
# - Defaults:
#   - Unsupported values fail with a precise context path.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: strict JSON boundary
#   - reason: aliases and recursive normalization form one JSON-only contract
#   - split: extract traversal if serialization behavior is introduced
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Strict JSON values used across translator boundaries."""

from __future__ import annotations

import math
from typing import cast

from mcp.src.domain.errors import fail_protocol

type JsonScalar = str | int | float | bool | None
type JsonValue = JsonScalar | list[JsonValue] | dict[str, JsonValue]
type JsonObject = dict[str, JsonValue]

_MAX_CONTAINER_ITEMS = 100_000


def _escape_diagnostic_text(value: str) -> str:
    """Return reversible ASCII text for one diagnostic fragment."""
    return value.encode("unicode_escape").decode("ascii")


class DuplicateJsonKeyError(ValueError):
    """Raised when one JSON object repeats a member name."""

    def __init__(self, key: str) -> None:
        """Create one duplicate-member failure."""
        escaped_key = _escape_diagnostic_text(key)
        super().__init__(f"duplicate JSON key: {escaped_key}")


def reject_duplicate_json_object(
    pairs: list[tuple[str, object]],
) -> dict[str, object]:
    """Build one object while rejecting repeated member names.

    Args:
        pairs: Object members in source order.

    Returns:
        One mapping containing every unique source member.

    Raises:
        DuplicateJsonKeyError: When one member name appears more than once.
    """
    _require_container_item_limit(len(pairs), context="JSON object")
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise DuplicateJsonKeyError(key)
        result[key] = value
    return result


def _normalize_json_text(value: str, *, context: str) -> str:
    try:
        encoded = value.encode("utf-16-le", errors="surrogatepass")
        return encoded.decode("utf-16-le")
    except UnicodeDecodeError as error:
        fail_protocol(f"{context}: unpaired Unicode surrogate", cause=error)


def _require_container_item_limit(size: int, *, context: str) -> None:
    if size > _MAX_CONTAINER_ITEMS:
        fail_protocol(f"{context}: JSON container item limit exceeded")


def normalize_json(value: object, *, context: str) -> JsonValue:
    """Return one deeply validated JSON value.

    Args:
        value: Untrusted value returned by a JSON parser or caller.
        context: Human-readable source used in failure messages.

    Returns:
        A JSON-only value without untyped objects.
    """
    try:
        return _normalize_json_value(value, context=context)
    except RecursionError as error:
        fail_protocol(f"{context}: JSON nesting is too deep", cause=error)


def _normalize_json_value(value: object, *, context: str) -> JsonValue:
    """Normalize one JSON value during a guarded recursive traversal.

    Returns:
        A JSON-only value without untyped objects.
    """
    if isinstance(value, str):
        return _normalize_json_text(value, context=context)
    if isinstance(value, float):
        if not math.isfinite(value):
            fail_protocol(f"{context}: JSON number must be finite")
        return value
    if value is None or isinstance(value, bool | int):
        return value
    if isinstance(value, dict):
        raw_mapping = cast("dict[object, object]", value)
        _require_container_item_limit(len(raw_mapping), context=context)
        result: JsonObject = {}
        for raw_key, raw_value in raw_mapping.items():
            if not isinstance(raw_key, str):
                fail_protocol(f"{context}: JSON key is not text")
            key = _normalize_json_text(
                raw_key,
                context=f"{context}: JSON key",
            )
            if key in result:
                fail_protocol(f"{context}: duplicate normalized JSON key")
            diagnostic_key = _escape_diagnostic_text(key)
            result[key] = _normalize_json_value(
                raw_value,
                context=f"{context}.{diagnostic_key}",
            )
        return result
    if isinstance(value, list):
        raw_items = cast("list[object]", value)
        _require_container_item_limit(len(raw_items), context=context)
        return [
            _normalize_json_value(item, context=f"{context}[{index}]")
            for index, item in enumerate(raw_items)
        ]
    return fail_protocol(
        f"{context}: unsupported JSON type {type(value).__name__}"
    )


def require_json_object(value: object, *, context: str) -> JsonObject:
    """Return one validated JSON object.

    Args:
        value: Untrusted value to validate.
        context: Human-readable source used in failure messages.

    Returns:
        A strict JSON object.

    """
    normalized = normalize_json(value, context=context)
    if not isinstance(normalized, dict):
        fail_protocol(f"{context}: expected a JSON object")
    return normalized

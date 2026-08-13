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
#   - Argument schema domain tests.
# - Must-Not:
#   - Open MCP sessions or mutate Unreal Editor.
# - Allows:
#   - Exercise supported schema assertions and fail-closed behavior.
# - Split-When:
#   - Split when schema families gain independent fixture lifecycles.
# - Merge-When:
#   - Merge when argument validation has no independent tests.
# - Summary:
#   - Native tool argument schema tests.
# - Description:
#   - Proves invalid native call payloads fail before transport invocation.
# - Usage:
#   - Run through the repository Python validator.
# - Defaults:
#   - Diagnostics never include rejected argument values.
#

"""Tests for live native tool argument schema validation."""

from __future__ import annotations

from mcp.domain.argument_schema import validate_tool_arguments
from mcp.domain.errors import ProtocolError
from mcp.domain.json_types import JsonObject
import pytest

_CONTEXT = "tool EditorToolset.create_asset arguments"


def _schema() -> JsonObject:
    return {
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "minLength": 2,
                "maxLength": 12,
                "pattern": "^[A-Z][A-Za-z0-9_]+$",
            },
            "count": {
                "type": "integer",
                "minimum": 1,
                "maximum": 8,
                "multipleOf": 1,
            },
            "mode": {"enum": ["create", "replace"]},
            "tags": {
                "type": "array",
                "items": {"type": "string", "minLength": 1},
                "minItems": 1,
                "maxItems": 3,
                "uniqueItems": True,
            },
            "options": {
                "type": "object",
                "properties": {
                    "enabled": {"type": "boolean"},
                    "weight": {
                        "type": ["number", "null"],
                        "exclusiveMinimum": 0,
                        "exclusiveMaximum": 1,
                    },
                },
                "required": ["enabled"],
                "additionalProperties": False,
            },
        },
        "required": ["name", "count", "mode", "tags", "options"],
        "additionalProperties": False,
    }


def _arguments() -> JsonObject:
    return {
        "name": "Asset_1",
        "count": 2,
        "mode": "create",
        "tags": ["world", "prop"],
        "options": {"enabled": True, "weight": 0.5},
    }


def test_accepts_supported_nested_schema_constraints() -> None:
    validate_tool_arguments(_schema(), _arguments(), context=_CONTEXT)


@pytest.mark.parametrize(
    ("mutate", "message"),
    [
        (lambda value: value.pop("name"), "required property is missing"),
        (lambda value: value.__setitem__("name", "bad"), "match pattern"),
        (lambda value: value.update(count=True), "declared type"),
        (lambda value: value.__setitem__("count", 9), "above maximum"),
        (lambda value: value.__setitem__("mode", "delete"), "declared enum"),
        (
            lambda value: value.__setitem__("tags", ["same", "same"]),
            "not unique",
        ),
        (
            lambda value: value.__setitem__(
                "options", {"enabled": True, "unknown": 1}
            ),
            "undeclared property",
        ),
        (lambda value: value.__setitem__("unknown", 1), "undeclared property"),
    ],
)
def test_rejects_invalid_arguments_without_echoing_values(
    mutate: object,
    message: str,
) -> None:
    arguments = _arguments()
    mutation = mutate
    assert callable(mutation)
    mutation(arguments)
    with pytest.raises(ProtocolError, match=message) as captured:
        validate_tool_arguments(_schema(), arguments, context=_CONTEXT)
    diagnostic = str(captured.value)
    assert "delete" not in diagnostic
    assert "same" not in diagnostic
    assert "unknown" not in diagnostic


def test_integer_type_accepts_mathematically_integral_json_numbers() -> None:
    schema: JsonObject = {
        "type": "object",
        "properties": {"count": {"type": "integer"}},
        "required": ["count"],
        "additionalProperties": False,
    }
    validate_tool_arguments(schema, {"count": 1.0}, context=_CONTEXT)
    with pytest.raises(ProtocolError, match="declared type"):
        validate_tool_arguments(schema, {"count": 1.5}, context=_CONTEXT)


def test_composition_keywords_require_exact_branch_semantics() -> None:
    schema: JsonObject = {
        "type": "object",
        "properties": {
            "value": {
                "oneOf": [
                    {"type": "string", "pattern": "^asset:"},
                    {"type": "integer", "minimum": 1},
                ]
            }
        },
        "required": ["value"],
        "additionalProperties": False,
    }
    validate_tool_arguments(schema, {"value": "asset:car"}, context=_CONTEXT)
    validate_tool_arguments(schema, {"value": 2}, context=_CONTEXT)
    with pytest.raises(ProtocolError, match="exactly one branch"):
        validate_tool_arguments(schema, {"value": False}, context=_CONTEXT)


def test_rejects_unsupported_or_malformed_live_schemas() -> None:
    schemas: tuple[JsonObject, ...] = (
        {"type": "object", "$ref": "#/$defs/Input"},
        {"type": "object", "required": ["missing"], "properties": {}},
        {"type": "object", "properties": {}, "additionalProperties": "no"},
        {"type": "object", "properties": {}, "minProperties": -1},
        {"type": ["object", "object"]},
        {"type": "mystery"},
        {"enum": [1, 1.0]},
        {"pattern": "["},
    )
    for schema in schemas:
        with pytest.raises(ProtocolError):
            validate_tool_arguments(schema, {}, context=_CONTEXT)


def test_annotation_keywords_do_not_change_argument_semantics() -> None:
    schema: JsonObject = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "Input",
        "description": "One native input.",
        "type": "object",
        "properties": {"name": {"type": "string", "default": "Asset"}},
        "additionalProperties": False,
    }
    validate_tool_arguments(schema, {}, context=_CONTEXT)

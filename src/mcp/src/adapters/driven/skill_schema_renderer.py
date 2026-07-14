# File:
#   - skill_schema_renderer.py
# Path:
#   - src/mcp/src/adapters/driven/skill_schema_renderer.py
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
#   - Human-readable input, output, and example schema summaries.
# - Must-Not:
#   - Render complete skills, infer side effects, access files, or call tools.
# - Allows:
#   - Describing fields, constraints, defaults, and placeholder values.
# - Split-When:
#   - Input and output schema rendering require independent depth policies.
# - Merge-When:
#   - Another adapter owns the same generated schema guidance contract.
# - Summary:
#   - Converts live native tool schemas into actionable skill sections.
# - Description:
#   - Keeps every field explanation tied to exposed MCP interface metadata.
# - Usage:
#   - Consumed by the per-tool skill renderer.
# - Defaults:
#   - Summarizes fields and requires live `describe` for exact nesting.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated Unreal MCP schema guidance
#   - reason: field summaries, constraints, and examples share one contract
#   - split: extract example generation if nested schema support expands
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess when native schema vocabulary changes
#
"""Render live Unreal MCP JSON schemas as actionable skill guidance."""

from __future__ import annotations

import json
import textwrap
from html import escape
from typing import TYPE_CHECKING, cast

from mcp.src.domain.errors import fail_protocol
from mcp.src.domain.json_types import JsonObject, JsonValue, require_json_object

if TYPE_CHECKING:
    from mcp.src.domain.catalog import ToolDefinition

_MAX_INLINE_DEFAULT_LENGTH = 72


def render_inputs(
    tool: ToolDefinition,
    argument_docs: dict[str, str],
) -> list[str]:
    """Render top-level input fields with requiredness and guidance.

    Returns:
        Markdown lines describing every exposed input field.
    """
    properties = _properties(tool.input_schema, f"{tool.name}.inputSchema")
    required = _required_names(tool.input_schema, f"{tool.name}.inputSchema")
    if not properties:
        return ["This tool accepts no input fields."]
    lines: list[str] = []
    for name in sorted(properties):
        _ = _inline_schema_text(name, "field name")
        schema = require_json_object(
            properties[name],
            context=f"{tool.name}.inputSchema.properties.{name}",
        )
        lines.extend(
            _field_lines(
                name,
                schema,
                required=name in required,
                fallback_description=argument_docs.get(name, ""),
            )
        )
    return lines


def render_output(tool: ToolDefinition, returns_text: str) -> list[str]:
    """Render structured output fields and native return guidance.

    Returns:
        Markdown lines describing the declared structured output.
    """
    schema = tool.output_schema
    if schema is None:
        return [
            "The live interface does not declare a structured output schema.",
            *(_wrapped_note(returns_text) if returns_text else []),
        ]
    properties = _properties(schema, f"{tool.name}.outputSchema")
    if not properties:
        lines = ["The tool declares output without named top-level fields."]
        lines.extend(_wrapped_note(returns_text))
        return lines
    required = _required_names(schema, f"{tool.name}.outputSchema")
    lines: list[str] = []
    if returns_text:
        lines.extend(_wrapped_note(returns_text))
        lines.append("")
    for name in sorted(properties):
        _ = _inline_schema_text(name, "field name")
        field_schema = require_json_object(
            properties[name],
            context=f"{tool.name}.outputSchema.properties.{name}",
        )
        lines.extend(
            _field_lines(
                name,
                field_schema,
                required=name in required,
                fallback_description="",
            )
        )
    return lines


def example_arguments(tool: ToolDefinition) -> JsonObject:
    """Return placeholder JSON for every required input field."""
    properties = _properties(tool.input_schema, f"{tool.name}.inputSchema")
    required = _required_names(tool.input_schema, f"{tool.name}.inputSchema")
    result: JsonObject = {}
    for name in sorted(required):
        value = properties.get(name)
        if value is None:
            fail_protocol(f"{tool.name}: required field lacks a schema: {name}")
        schema = require_json_object(
            value,
            context=f"{tool.name}.inputSchema.properties.{name}",
        )
        result[name] = _example_value(schema)
    return result


def render_example_json(arguments: JsonObject) -> str:
    """Return stable pretty JSON for a generated invocation example."""
    return json.dumps(arguments, ensure_ascii=True, indent=2, sort_keys=True)


def _field_lines(
    name: str,
    schema: JsonObject,
    *,
    required: bool,
    fallback_description: str,
) -> list[str]:
    description = _description(schema) or fallback_description
    lines = [
        f"### `{name}`",
        "",
        f"- Required: **{'yes' if required else 'no'}**",
        f"- Type: `{_schema_type(schema)}`",
    ]
    default = schema.get("default")
    if default is not None:
        lines.extend(_default_lines(default))
    enum = schema.get("enum")
    if isinstance(enum, list) and enum:
        lines.extend(["- Allowed values:", ""])
        lines.extend(f"  - `{_compact_json(value)}`" for value in enum)
    pattern = schema.get("pattern")
    if isinstance(pattern, str) and pattern:
        pattern = _inline_schema_text(pattern, "pattern")
        lines.append(f"- Pattern: `{pattern}`")
    lines.extend(["- Purpose:", ""])
    lines.extend(_wrapped_note(description or _missing_description(name)))
    lines.append("")
    return lines


def _inline_schema_text(value: str, label: str) -> str:
    if any(not character.isprintable() for character in value):
        fail_protocol(f"schema {label} contains controls")
    return value


def _properties(schema: JsonObject, context: str) -> JsonObject:
    value = schema.get("properties", {})
    return require_json_object(value, context=f"{context}.properties")


def _required_names(schema: JsonObject, context: str) -> frozenset[str]:
    value = schema.get("required", [])
    if not isinstance(value, list):
        fail_protocol(f"{context}.required must be an array")
    names: set[str] = set()
    for item in value:
        if not isinstance(item, str):
            fail_protocol(f"{context}.required must contain strings")
        names.add(item)
    return frozenset(names)


def _description(schema: JsonObject) -> str:
    for key in ("description", "title"):
        value = schema.get(key)
        if isinstance(value, str) and value.strip():
            if any(
                not character.isprintable() and not character.isspace()
                for character in value
            ):
                fail_protocol("schema prose contains controls")
            return " ".join(value.split())
    return ""


def _schema_type(schema: JsonObject) -> str:
    raw_type = schema.get("type")
    result = "unspecified"
    if isinstance(raw_type, str):
        raw_type = _inline_schema_text(raw_type, "type")
        result = raw_type
        if raw_type == "array":
            result = "array"
            items = schema.get("items")
            if isinstance(items, dict):
                item_schema = cast("JsonObject", items)
                result = f"array<{_schema_type(item_schema)}>"
    elif isinstance(raw_type, list):
        names = [
            _inline_schema_text(item, "type")
            for item in raw_type
            if isinstance(item, str)
        ]
        if names:
            result = " | ".join(names)
    elif "properties" in schema:
        result = "object"
    elif "enum" in schema:
        result = "enum"
    return result


def _example_value(schema: JsonObject) -> JsonValue:
    enum = schema.get("enum")
    default = schema.get("default")
    result: JsonValue = "<value>"
    if isinstance(enum, list) and enum:
        result = cast("JsonValue", enum[0])
    elif default is not None:
        result = cast("JsonValue", default)
    else:
        schema_type = _schema_type(schema)
        if schema_type == "boolean":
            result = False
        elif schema_type == "integer":
            result = 0
        elif schema_type == "number":
            result = 0.0
        elif schema_type.startswith("array"):
            result = []
        elif schema_type == "object":
            result = {}
    return result


def _wrapped_note(text: str) -> list[str]:
    normalized = " ".join(text.split()).replace(" - ", "; ")
    escaped = escape(normalized, quote=False)
    return textwrap.wrap(escaped, width=79) if escaped else []


def _missing_description(name: str) -> str:
    return f"`{name}` has no prose; confirm its meaning with `describe`."


def _default_lines(value: object) -> list[str]:
    compact = _compact_json(value)
    if len(compact) <= _MAX_INLINE_DEFAULT_LENGTH:
        return [f"- Default: `{compact}`"]
    return [
        "- Default: declared by the live schema; inspect it with `describe`."
    ]


def _compact_json(value: object) -> str:
    return json.dumps(value, ensure_ascii=True, separators=(",", ":"))

# File:
#   - catalog.py
# Path:
#   - src/mcp/src/domain/catalog.py
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
#   - Native toolset catalog, schema, and tool outcome values.
# - Must-Not:
#   - Perform HTTP, CLI parsing, file IO, or editor mutation.
# - Allows:
#   - Pure parsing and validation of native catalog evidence.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Models the native Unreal tool catalog deterministically.
# - Description:
#   - Parses discovery output without adapter dependencies.
# - Usage:
#   - Consumed by discovery and invocation application use cases.
# - Defaults:
#   - Malformed, duplicate, or error outcomes fail closed.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: native tool catalog domain
#   - reason: toolset schemas and outcomes share catalog validation invariants
#   - split: extract outcome parsing if additional MCP content kinds are added
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Native Unreal toolset and tool catalog values."""

from __future__ import annotations

import json
import re
from typing import NamedTuple, cast

from mcp.src.domain.errors import fail_protocol
from mcp.src.domain.json_types import (
    DuplicateJsonKeyError,
    JsonObject,
    JsonValue,
    reject_duplicate_json_object,
    require_json_object,
)
from mcp.src.domain.tool_identity import (
    canonical_tool_identity,
    validated_toolset_identity,
)

_TOOLSET_NAME_PATTERN = r"[A-Za-z0-9_]+(?:\.[A-Za-z0-9_]+)+"
_TOOLSET_DESCRIPTION_PATTERN = r"(?: (?P<description>.*))?$"
_TOOLSET_HEADER_PREFIX = f"^- (?P<name>{_TOOLSET_NAME_PATTERN}):"
_TOOLSET_HEADER = re.compile(
    f"{_TOOLSET_HEADER_PREFIX}{_TOOLSET_DESCRIPTION_PATTERN}",
)
_MAX_TOOLSET_SUMMARIES = 10_000
_MAX_TOOLS_PER_TOOLSET = 10_000
_MAX_DESCRIPTION_BYTES = 64 * 1_024


class ToolsetSummary(NamedTuple):
    """One discoverable native Unreal toolset."""

    name: str
    description: str


class ToolDefinition(NamedTuple):
    """One tool exposed by a native Unreal toolset."""

    name: str
    description: str
    input_schema: JsonObject
    output_schema: JsonObject | None


class ToolsetDefinition(NamedTuple):
    """Complete schema for one native Unreal toolset."""

    name: str
    description: str
    tools: tuple[ToolDefinition, ...]
    raw_schema: JsonObject


def parse_toolset_catalog(text: str) -> tuple[ToolsetSummary, ...]:
    """Parse the native qualified-header toolset catalog.

    Args:
        text: Text returned by the native `list_toolsets` meta-tool.

    Returns:
        Toolsets in the same deterministic order returned by Unreal.

    """
    summaries: list[ToolsetSummary] = []
    seen: set[str] = set()
    current_name: str | None = None
    description_lines: list[str] = []
    for number, raw_line in enumerate(text.splitlines(), 1):
        line = raw_line.rstrip()
        header = _TOOLSET_HEADER.fullmatch(line)
        if header is not None:
            if current_name is not None:
                summaries.append(
                    _toolset_summary(
                        current_name,
                        description_lines,
                    ),
                )
            matched_name = header.group("name")
            if matched_name is None:
                fail_protocol(
                    f"toolset catalog line {number}: missing toolset name",
                )
            current_name = validated_toolset_identity(
                cast(
                    "str",
                    matched_name,
                ),
            )
            if current_name in seen:
                fail_protocol(
                    f"toolset catalog line {number}: duplicate {current_name}"
                )
            if len(seen) >= _MAX_TOOLSET_SUMMARIES:
                fail_protocol("toolset catalog exceeded its toolset limit")
            seen.add(current_name)
            first_description = header.group("description")
            description_lines = _initial_description_lines(first_description)
            continue
        if current_name is None:
            if line.strip():
                fail_protocol(
                    f"toolset catalog line {number}: expected qualified header"
                )
            continue
        description_lines.append(line)
    if current_name is not None:
        summaries.append(_toolset_summary(current_name, description_lines))
    return tuple(summaries)


def _initial_description_lines(value: str | None) -> list[str]:
    return [] if value is None else [value]


def _toolset_summary(
    name: str,
    description_lines: list[str],
) -> ToolsetSummary:
    description = _validated_description(
        "\n".join(description_lines).strip(),
        context=f"toolset {name}",
    )
    return ToolsetSummary(name=name, description=description)


def _validated_description(value: object, *, context: str) -> str:
    if not isinstance(value, str):
        fail_protocol(f"{context}: description must be text")
    try:
        size = len(value.encode())
    except UnicodeEncodeError as error:
        fail_protocol(
            f"{context}: description contains invalid Unicode",
            cause=error,
        )
    if size > _MAX_DESCRIPTION_BYTES:
        fail_protocol(f"{context}: description byte limit exceeded")
    return value


def parse_toolset_definition(
    toolset_name: str,
    schema_text: str,
) -> ToolsetDefinition:
    """Parse one Toolset Registry JSON schema document.

    Args:
        toolset_name: Native Toolset Registry identity.
        schema_text: JSON text returned by `describe_toolset`.

    Returns:
        A complete validated toolset definition.

    """
    toolset = validated_toolset_identity(toolset_name)
    try:
        parsed = cast(
            "object",
            json.loads(
                schema_text,
                object_pairs_hook=reject_duplicate_json_object,
            ),
        )
    except DuplicateJsonKeyError as error:
        fail_protocol(str(error), cause=error)
    except json.JSONDecodeError as error:
        fail_protocol(
            f"toolset {toolset}: schema is not valid JSON",
            cause=error,
        )
    schema = require_json_object(parsed, context=f"toolset {toolset}")
    raw_tools = schema.get("tools")
    if not isinstance(raw_tools, list):
        fail_protocol(f"toolset {toolset}: missing tools array")
    if len(raw_tools) > _MAX_TOOLS_PER_TOOLSET:
        fail_protocol("toolset schema exceeded its tool limit")
    tools = tuple(
        _parse_tool(
            toolset,
            item,
            index,
        )
        for index, item in enumerate(raw_tools)
    )
    names = [tool.name for tool in tools]
    if len(set(names)) != len(names):
        fail_protocol(f"toolset {toolset}: duplicate tool identity")
    description = _validated_description(
        schema.get("description", ""),
        context=f"toolset {toolset}",
    )
    return ToolsetDefinition(
        name=toolset,
        description=description,
        tools=tools,
        raw_schema=schema,
    )


def _parse_tool(
    toolset_name: str,
    value: JsonValue,
    index: int,
) -> ToolDefinition:
    context = f"toolset {toolset_name}.tools[{index}]"
    tool = require_json_object(value, context=context)
    raw_name = tool.get("name")
    if not isinstance(raw_name, str) or not raw_name:
        fail_protocol(f"{context}: name must be non-empty text")
    name = canonical_tool_identity(toolset_name, raw_name)
    description = _validated_description(
        tool.get("description", ""),
        context=context,
    )
    input_schema = require_json_object(
        tool.get("inputSchema", {}),
        context=f"{context}.inputSchema",
    )
    output_value = tool.get("outputSchema")
    output_schema = (
        None
        if output_value is None
        else require_json_object(
            output_value,
            context=f"{context}.outputSchema",
        )
    )
    return ToolDefinition(
        name=name,
        description=description,
        input_schema=input_schema,
        output_schema=output_schema,
    )

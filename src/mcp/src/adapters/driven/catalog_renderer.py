# File:
#   - catalog_renderer.py
# Path:
#   - src/mcp/src/adapters/driven/catalog_renderer.py
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
#   - Deterministic JSON and Markdown catalog rendering.
# - Must-Not:
#   - Call Unreal, open transports, or mutate repository files.
# - Allows:
#   - Stable presentation of validated domain catalog values.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Renders discovered tools for humans and automation.
# - Description:
#   - Keeps presentation formatting outside the application core.
# - Usage:
#   - Called by the driving CLI after complete catalog discovery.
# - Defaults:
#   - Sorts schemas and preserves native toolset ordering.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: deterministic catalog renderers
#   - reason: JSON and Markdown rendering share one ordering contract
#   - split: extract Markdown rendering if another output format is added
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
"""Deterministic renderers for discovered native Unreal tool catalogs."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

from mcp.src.adapters.driven.skill_technical_text import validated_live_prose

if TYPE_CHECKING:
    from mcp.src.domain.catalog import ToolsetDefinition
    from mcp.src.domain.json_types import JsonObject, JsonValue


def render_json(value: JsonValue) -> str:
    """Render one JSON value with stable keys and a final newline.

    Returns:
        Deterministic pretty-printed JSON ending in one newline.
    """
    return (
        json.dumps(
            value,
            ensure_ascii=False,
            indent=2,
            sort_keys=True,
        )
        + "\n"
    )


def render_catalog_json(
    toolsets: tuple[ToolsetDefinition, ...],
) -> str:
    """Render the complete tool catalog as stable JSON.

    Returns:
        Deterministic catalog JSON ending in one newline.
    """
    payload: JsonObject = {
        "toolsets": [toolset.raw_schema for toolset in toolsets]
    }
    return render_json(payload)


def render_catalog_markdown(
    toolsets: tuple[ToolsetDefinition, ...],
) -> str:
    """Render the complete tool catalog as reviewable Markdown.

    Returns:
        Deterministic Markdown ending in one newline.
    """
    lines = [
        "# Unreal native MCP tool catalog",
        "",
        "Generated from live `list_toolsets` and `describe_toolset` calls.",
        "",
    ]
    for toolset in toolsets:
        lines.extend([f"## `{toolset.name}`", ""])
        if toolset.description:
            lines.extend([validated_live_prose(toolset.description), ""])
        lines.extend(
            [
                f"Discovered tools: **{len(toolset.tools)}**",
                "",
            ]
        )
        for tool in toolset.tools:
            lines.extend([f"### `{tool.name}`", ""])
            if tool.description:
                lines.extend([validated_live_prose(tool.description), ""])
            lines.extend(
                [
                    "Input schema:",
                    "",
                    "```json",
                    render_json(tool.input_schema).rstrip(),
                    "```",
                    "",
                ]
            )
            if tool.output_schema is not None:
                lines.extend(
                    [
                        "Output schema:",
                        "",
                        "```json",
                        render_json(tool.output_schema).rstrip(),
                        "```",
                        "",
                    ]
                )
    return "\n".join(lines).rstrip() + "\n"

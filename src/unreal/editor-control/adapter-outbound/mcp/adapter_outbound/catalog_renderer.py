# Copyright:
#   - Copyright © 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Catalog renderer outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Catalog renderer outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Catalog renderer outbound adapter."""

from __future__ import annotations

import json
from typing import TYPE_CHECKING

from mcp.adapter_outbound.skill_technical_text import validated_live_prose

if TYPE_CHECKING:
    from mcp.domain.catalog import ToolsetDefinition
    from mcp.domain.json_types import JsonObject
    from mcp.domain.json_types import JsonValue


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

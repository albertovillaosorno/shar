# File:
#   - skill_capability_renderer.py
# Path:
#   - src/mcp/src/adapters/driven/skill_capability_renderer.py
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
#   - Complete generated Markdown for one native Unreal MCP tool skill.
# - Must-Not:
#   - Generate paths, parse raw schemas, access files, or invoke tools.
# - Allows:
#   - Combining description, schema guidance, invocation, and checks.
# - Split-When:
#   - Invocation and verification guidance require independent policies.
# - Merge-When:
#   - Another adapter owns the same complete per-tool skill document.
# - Summary:
#   - Renders one actionable skill for one native Unreal MCP tool.
# - Description:
#   - Produces purpose, inputs, outputs, example use, posture, and checks.
# - Usage:
#   - Called once per live tool by the skill Markdown orchestration adapter.
# - Defaults:
#   - Requires a live `describe` refresh before state-changing use.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated native Unreal MCP per-tool skill rendering
#   - reason: purpose, invocation, posture, and checks form one contract
#   - split: extract verification policy if native annotations become available
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after generated skill content or workflow changes
#
"""Render one actionable Markdown skill per native Unreal MCP tool."""

from __future__ import annotations

import re
from pathlib import PurePosixPath
from typing import TYPE_CHECKING

from mcp.src.adapters.driven.skill_description import (
    markdown_paragraphs,
    operational_posture,
    parse_description,
)
from mcp.src.adapters.driven.skill_manual_fields import render_manual_section
from mcp.src.adapters.driven.skill_markdown_policy import (
    render_unbreakable_line,
)
from mcp.src.adapters.driven.skill_schema_renderer import (
    example_arguments,
    render_example_json,
    render_inputs,
    render_output,
)
from mcp.src.domain.skill_documents import SkillDocument

if TYPE_CHECKING:
    from mcp.src.domain.catalog import ToolDefinition, ToolsetDefinition
    from mcp.src.domain.skill_categories import SkillCategory
    from mcp.src.domain.skill_revision import SkillRevision

_WORDS = re.compile(r"[A-Z]+(?=[A-Z][a-z]|\d|$)|[A-Z]?[a-z]+|[A-Z]+|\d+")
_GENERATED_NOTICE = (
    "Generated from live MCP metadata; no engine source is copied."
)
_WHEN_TO_USE_LINES = (
    "Use this skill when the requested outcome matches its purpose.",
    "Choose it only when it is the most specific available action.",
    "Do not substitute it for a narrower read or mutation capability.",
)
_EXAMPLE_NOTICE = "Replace placeholders with validated project values."
_STATE_CHANGE_LINE = (
    "1. Capture pre-state and define an independent postcondition check."
)


def render_tool_skill(
    category: SkillCategory,
    toolset: ToolsetDefinition,
    tool: ToolDefinition,
    revision: SkillRevision,
    relative_path: str,
) -> SkillDocument:
    """Render one complete actionable tool skill document.

    Returns:
        A complete generated Markdown document for this native tool.
    """
    sections = parse_description(tool.description)
    posture = operational_posture(tool.name)
    arguments = example_arguments(tool)
    index_link = _index_backlink(relative_path)
    lines = [
        f"# {_display_name(tool.name)}",
        "",
        *render_unbreakable_line(
            f"[Return to the central Unreal MCP index]({index_link})."
        ),
        "",
        _GENERATED_NOTICE,
        "",
        f"- Domain: {category.title}",
        f"- Operational posture: **{posture.label}**",
        *render_unbreakable_line(
            f"- Interface digest: `{revision.interface_digest}`"
        ),
        "",
        "## Native identities",
        "",
        "Tool:",
        "",
        "```text",
        tool.name,
        "```",
        "",
        "Toolset:",
        "",
        "```text",
        toolset.name,
        "```",
        "",
        "## What this tool does",
        "",
    ]
    lines.extend(markdown_paragraphs(sections.purpose))
    lines.extend(
        [
            "",
            "## When to use it",
            "",
            *_WHEN_TO_USE_LINES,
            "",
            "## Technical execution posture",
            "",
            *markdown_paragraphs(posture.guidance),
        ]
    )
    if sections.notes:
        lines.extend(["", "Additional native guidance:", ""])
        lines.extend(markdown_paragraphs(sections.notes))
    lines.extend(["", *render_manual_section(revision.token)])
    lines.extend(
        [
            "",
            "## Before invocation",
            "",
            "1. Run `shar-unreal-mcp doctor` and require `ready: true`.",
            "1. Select this skill from the central index, not from memory.",
            "1. Refresh the live schema:",
            "",
            "```text",
            f"shar-unreal-mcp describe {toolset.name}",
            "```",
            "",
            "1. Confirm every required input against the current schema.",
        ]
    )
    if posture.requires_state_verification:
        lines.append(_STATE_CHANGE_LINE)
    lines.extend(["", "## Inputs", ""])
    lines.extend(render_inputs(tool, sections.arguments))
    lines.extend(
        [
            "",
            "## Invocation example",
            "",
            _EXAMPLE_NOTICE,
            "",
            "```text",
            "shar-unreal-mcp call \\",
            f"  {toolset.name} \\",
            f"  {tool.name} \\",
            "  --arguments '",
        ]
    )
    lines.extend(render_example_json(arguments).splitlines())
    lines.extend(["'", "```", "", "## Expected output", ""])
    lines.extend(render_output(tool, sections.returns))
    lines.extend(["", "## Verification", ""])
    lines.extend(
        _verification_lines(
            requires_state_verification=posture.requires_state_verification
        )
    )
    lines.extend(["", "## Common failure modes", ""])
    lines.extend(_failure_lines(sections.raises))
    content = "\n".join(lines).rstrip()
    content = re.sub(r"\n{3,}", "\n\n", content)
    return SkillDocument(
        relative_path=relative_path,
        content=f"{content}\n",
    )


def _display_name(tool_name: str) -> str:
    leaf = tool_name.rsplit(".", 1)[-1]
    words = [match.group(0).casefold() for match in _WORDS.finditer(leaf)]
    if not words:
        return leaf
    return " ".join(words).capitalize()


def _index_backlink(relative_path: str) -> str:
    parent_depth = len(PurePosixPath(relative_path).parent.parts)
    return f"{'../' * parent_depth}index.md"


def _verification_lines(*, requires_state_verification: bool) -> list[str]:
    lines = [
        "- Check the returned `isError` state and structured output.",
        "- Compare returned identities and counts with the requested scope.",
        "- Treat transport success as insufficient evidence by itself.",
    ]
    if requires_state_verification:
        lines.extend(
            [
                "- Verify changed state through a separate read or inspection.",
                "- Use another capability to confirm the postcondition.",
                "- Inspect editor logs when state is not directly observable.",
            ]
        )
    else:
        lines.extend(
            [
                "- Confirm the response belongs to the open editor project.",
                "- Reject evidence derived from stale discovery state.",
            ]
        )
    return lines


def _failure_lines(raises_text: str) -> list[str]:
    lines = [
        "- The skill may be stale; run `describe` and regenerate the catalog.",
        "- A required editor object or asset may not be loaded.",
        "- Placeholder values are not valid project identities.",
        "- Native validation may reject semantically invalid JSON values.",
    ]
    if raises_text:
        lines.extend(["", "Native failure guidance:", ""])
        lines.extend(markdown_paragraphs(raises_text))
    return lines

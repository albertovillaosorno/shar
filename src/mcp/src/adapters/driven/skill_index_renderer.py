# File:
#   - skill_index_renderer.py
# Path:
#   - src/mcp/src/adapters/driven/skill_index_renderer.py
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
#   - The single central Unreal MCP skill index Markdown document.
# - Must-Not:
#   - Render skill-page details, open files, or invoke Unreal.
# - Allows:
#   - Listing every toolset and capability under a docs-style taxonomy.
# - Split-When:
#   - The central index gains another independently versioned output format.
# - Merge-When:
#   - Another adapter owns the same generated central index.
# - Summary:
#   - Renders the mandatory central Unreal MCP skill index.
# - Description:
#   - Mirrors docs indexes with semantic headings and direct skill links.
# - Usage:
#   - Called by the generated skill orchestration adapter.
# - Defaults:
#   - Includes capabilities, workflow routing, and regeneration guidance.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: central generated Unreal MCP skill index
#   - reason: all capability routing must remain discoverable in one index
#   - split: extract introductory prose if another index format is added
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after taxonomy, workflow, or generation changes
#
"""Central docs-style index for native Unreal MCP skills."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.src.adapters.driven.skill_document_layout import tool_skill_path
from mcp.src.adapters.driven.skill_markdown_policy import (
    render_unbreakable_line,
)
from mcp.src.domain.errors import fail_protocol
from mcp.src.domain.skill_categories import CATEGORIES, SkillCategory

if TYPE_CHECKING:
    from mcp.src.domain.catalog import ToolsetDefinition
    from mcp.src.domain.skill_revision import SkillRevision

_GENERATED_NOTICE = (
    "Generated from the live MCP interface; edit only protected fields."
)
_WORKFLOWS = (
    ("Editor readiness", "workflows/editor-readiness.md"),
    ("Capability selection", "workflows/capability-selection.md"),
    ("Schema and arguments", "workflows/schema-and-arguments.md"),
    ("Read-only operations", "workflows/read-only-operations.md"),
    ("Safe mutations", "workflows/safe-mutations.md"),
    (
        "Long-running and batch operations",
        "workflows/long-running-and-batch-operations.md",
    ),
    ("Programmatic tool scripts", "workflows/programmatic-tool-scripts.md"),
    ("Verification and recovery", "workflows/verification-and-recovery.md"),
    (
        "Manual guidance maintenance",
        "workflows/manual-guidance-maintenance.md",
    ),
    ("Regeneration and taxonomy", "workflows/regeneration-and-taxonomy.md"),
)

_MANUAL_CURRENT_PREFIX = "- Manual guidance current: **"
_MANUAL_REVIEW_PREFIX = "- Manual guidance review required: **"


def render_root_index(
    grouped: dict[str, tuple[ToolsetDefinition, ...]],
    revision: SkillRevision,
) -> str:
    """Render the only Unreal MCP skill index.

    Returns:
        Complete central index Markdown containing every capability.
    """
    toolset_count = sum(len(items) for items in grouped.values())
    tool_count = sum(
        len(toolset.tools) for items in grouped.values() for toolset in items
    )
    lines = [
        "# Unreal MCP skill index",
        "",
        "> Read this index first for every Unreal MCP task.",
        "",
        _GENERATED_NOTICE,
        "",
        "This catalog follows the `docs/` navigation model.",
        "It uses one central index and name-derived taxonomy folders.",
        "Shared sibling prefixes become folders; unique suffixes become files.",
        "Every link opens one focused per-tool skill.",
        "",
        f"- Unreal MCP version: `{revision.unreal_mcp_version}`",
        *render_unbreakable_line(
            f"- Interface digest: `{revision.interface_digest}`"
        ),
        *render_unbreakable_line(
            f"- Manual review revision: `{revision.token}`"
        ),
        f"- Toolsets: **{toolset_count}**",
        f"- Capabilities: **{tool_count}**",
        f"{_MANUAL_CURRENT_PREFIX}0**",
        f"{_MANUAL_REVIEW_PREFIX}{tool_count}**",
        "- Protocol: `2025-11-25`",
        "",
        "## Usage",
        "",
        "1. Read this index before selecting a capability.",
        "1. Open the workflow skill for the operation stage.",
        "1. Open the linked capability skill.",
        "1. Fill protected fields only when project evidence exists.",
        "1. Run `describe` against the live editor before every mutation.",
        "1. Verify editor state independently after every mutation.",
        "",
        "Regeneration preserves text inside manual-field markers.",
        "Everything outside those markers is refreshed from live MCP metadata.",
        "The protected review revision is never advanced automatically.",
        "A version or interface change marks preserved guidance for review.",
        "The live schema is authoritative when generated files drift.",
        "Regenerate after Unreal Engine or Toolset plugin changes:",
        "",
        "```text",
        "shar-unreal-mcp skills",
        "```",
        "",
        "## Workflow skills",
        "",
    ]
    for title, path in _WORKFLOWS:
        lines.extend(render_unbreakable_line(f"- [{title}]({path})"))
    lines.extend(["", "## Capability taxonomy", ""])
    for category in CATEGORIES:
        lines.extend(_category_lines(category, grouped[category.slug]))
    return "\n".join(lines).rstrip() + "\n"


def _category_lines(
    category: SkillCategory,
    toolsets: tuple[ToolsetDefinition, ...],
) -> list[str]:
    capability_count = sum(len(toolset.tools) for toolset in toolsets)
    lines = [
        f"### {category.title}",
        "",
        category.purpose,
        "",
        f"{len(toolsets)} toolsets; {capability_count} capabilities.",
        "",
    ]
    for toolset in toolsets:
        lines.extend(
            [
                f"#### `{toolset.name}`",
                "",
                f"Capabilities: **{len(toolset.tools)}**",
                "",
            ]
        )
        for tool in sorted(toolset.tools, key=lambda item: item.name):
            link = f"- [`{tool.name}`]({tool_skill_path(toolset, tool)})"
            lines.extend(render_unbreakable_line(link))
        lines.append("")
    return lines


def replace_manual_review_summary(
    content: str,
    *,
    current_count: int,
    review_required_count: int,
) -> str:
    """Replace generated manual-review counts in the persisted root index.

    Returns:
        Complete index content with exact persisted review counts.
    """
    updated = _replace_count_line(
        content,
        prefix=_MANUAL_CURRENT_PREFIX,
        count=current_count,
    )
    return _replace_count_line(
        updated,
        prefix=_MANUAL_REVIEW_PREFIX,
        count=review_required_count,
    )


def _replace_count_line(content: str, *, prefix: str, count: int) -> str:
    if count < 0:
        fail_protocol("manual review summary count must not be negative")
    lines = content.splitlines()
    matches = [
        index for index, line in enumerate(lines) if line.startswith(prefix)
    ]
    if len(matches) != 1:
        fail_protocol("generated index has invalid manual review summary")
    lines[matches[0]] = f"{prefix}{count}**"
    return "\n".join(lines) + "\n"

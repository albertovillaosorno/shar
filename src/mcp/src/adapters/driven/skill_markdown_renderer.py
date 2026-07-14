# File:
#   - skill_markdown_renderer.py
# Path:
#   - src/mcp/src/adapters/driven/skill_markdown_renderer.py
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
#   - Orchestration of generated Unreal skill Markdown documents.
# - Must-Not:
#   - Format indexes, format tool details, open files, or invoke Unreal tools.
# - Allows:
#   - Validating taxonomy coverage and routing catalog slices to renderers.
# - Split-When:
#   - Catalog validation and document orchestration evolve independently.
# - Merge-When:
#   - Another adapter owns the complete generated document composition.
# - Summary:
#   - Composes the canonical Unreal MCP skill document tree.
# - Description:
#   - Delegates index and toolset formatting to responsibility-focused modules.
# - Usage:
#   - Injected into the Unreal skill export application use case.
# - Defaults:
#   - Rejects missing, duplicate, or unowned live toolsets.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated Unreal skill document orchestration
#   - reason: taxonomy validation and document routing form one adapter contract
#   - split: extract catalog validation if another renderer consumes it
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after taxonomy or renderer contract changes
#
"""Orchestration for deterministic native Unreal MCP skill Markdown."""

from __future__ import annotations

from collections import defaultdict
from typing import TYPE_CHECKING

from mcp.src.adapters.driven.skill_capability_renderer import render_tool_skill
from mcp.src.adapters.driven.skill_document_layout import (
    tool_skill_path,
    validate_unique_tool_paths,
)
from mcp.src.adapters.driven.skill_index_renderer import render_root_index
from mcp.src.domain.errors import fail_protocol
from mcp.src.domain.skill_documents import SkillDocument, interface_digest
from mcp.src.domain.skill_revision import build_skill_revision
from mcp.src.domain.skill_taxonomy import (
    CATEGORIES,
    category_for_toolset,
    known_toolset_names,
)

if TYPE_CHECKING:
    from mcp.src.domain.catalog import ToolsetDefinition


class MarkdownSkillRenderer:
    """Render a complete live catalog into SRP Markdown skill documents."""

    def __init__(self, unreal_mcp_version: str) -> None:
        """Create one renderer for an installed Unreal MCP version."""
        self._unreal_mcp_version = unreal_mcp_version

    def render(
        self,
        catalog: tuple[ToolsetDefinition, ...],
    ) -> tuple[SkillDocument, ...]:
        """Return the complete deterministic skill document set."""
        _check_catalog(catalog)
        validate_unique_tool_paths(catalog)
        digest = interface_digest(catalog)
        revision = build_skill_revision(self._unreal_mcp_version, digest)
        grouped = _group_catalog(catalog)
        documents = [
            SkillDocument(
                "index.md",
                render_root_index(grouped, revision),
            )
        ]
        for category in CATEGORIES:
            for toolset in grouped[category.slug]:
                documents.extend(
                    render_tool_skill(
                        category,
                        toolset,
                        tool,
                        revision,
                        tool_skill_path(toolset, tool),
                    )
                    for tool in sorted(
                        toolset.tools,
                        key=lambda item: item.name,
                    )
                )
        return tuple(documents)


def _check_catalog(catalog: tuple[ToolsetDefinition, ...]) -> None:
    names = [toolset.name for toolset in catalog]
    if len(names) != len(set(names)):
        fail_protocol(
            "live skill catalog contains duplicate toolset identities"
        )
    actual = frozenset(names)
    expected = known_toolset_names()
    missing = sorted(expected.difference(actual))
    unknown = sorted(actual.difference(expected))
    if missing:
        fail_protocol(
            f"live skill catalog is missing toolsets: {', '.join(missing)}"
        )
    if unknown:
        fail_protocol(
            f"live skill catalog has unowned toolsets: {', '.join(unknown)}"
        )


def _group_catalog(
    catalog: tuple[ToolsetDefinition, ...],
) -> dict[str, tuple[ToolsetDefinition, ...]]:
    grouped: dict[str, list[ToolsetDefinition]] = defaultdict(list)
    for toolset in catalog:
        grouped[category_for_toolset(toolset.name).slug].append(toolset)
    return {
        category.slug: tuple(
            sorted(grouped[category.slug], key=lambda item: item.name)
        )
        for category in CATEGORIES
    }

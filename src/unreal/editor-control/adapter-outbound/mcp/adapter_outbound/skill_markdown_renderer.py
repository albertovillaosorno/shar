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
#   - Skill markdown renderer outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill markdown renderer outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill markdown renderer outbound adapter."""

from __future__ import annotations

from collections import defaultdict
from typing import TYPE_CHECKING

from mcp.adapter_outbound.skill_capability_renderer import render_tool_skill
from mcp.adapter_outbound.skill_document_layout import tool_skill_path
from mcp.adapter_outbound.skill_document_layout import (
    validate_unique_tool_paths,
)
from mcp.adapter_outbound.skill_index_renderer import render_root_index
from mcp.domain.errors import fail_protocol
from mcp.domain.skill_documents import SkillDocument
from mcp.domain.skill_documents import interface_digest
from mcp.domain.skill_revision import build_skill_revision
from mcp.domain.skill_taxonomy import CATEGORIES
from mcp.domain.skill_taxonomy import category_for_toolset
from mcp.domain.skill_taxonomy import known_toolset_names

if TYPE_CHECKING:
    from mcp.domain.catalog import ToolsetDefinition


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

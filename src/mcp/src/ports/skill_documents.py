# File:
#   - skill_documents.py
# Path:
#   - src/mcp/src/ports/skill_documents.py
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
#   - Application-facing contracts for rendering and storing skill documents.
# - Must-Not:
#   - Implement Markdown, filesystem behavior, MCP transport, or CLI parsing.
# - Allows:
#   - Decoupling live catalog discovery from documentation adapters.
# - Split-When:
#   - Rendering and persistence need independently versioned ports.
# - Merge-When:
#   - Another port module owns the same generated-document contracts.
# - Summary:
#   - Defines generated Unreal skill document ports.
# - Description:
#   - Keeps the export use case independent from output technology.
# - Usage:
#   - Implemented by Markdown and filesystem driven adapters.
# - Defaults:
#   - One renderer and one replace-style store participate per export.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Ports for generated Unreal MCP skill documents."""

from __future__ import annotations

from typing import TYPE_CHECKING, Protocol

if TYPE_CHECKING:
    from mcp.src.domain.catalog import ToolsetDefinition
    from mcp.src.domain.skill_documents import SkillDocument


class SkillDocumentRenderer(Protocol):
    """Render a complete catalog into deterministic skill documents."""

    def render(
        self,
        catalog: tuple[ToolsetDefinition, ...],
    ) -> tuple[SkillDocument, ...]:
        """Return every generated document for one live catalog."""
        ...


class SkillDocumentStore(Protocol):
    """Replace the generated skill document surface."""

    @property
    def display_path(self) -> str:
        """The operator-facing output path."""
        ...

    def replace(self, documents: tuple[SkillDocument, ...]) -> None:
        """Persist a complete generated document set."""
        ...

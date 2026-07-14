# File:
#   - skill_export.py
# Path:
#   - src/mcp/src/application/skill_export.py
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
#   - Orchestration of live catalog discovery, rendering, and persistence.
# - Must-Not:
#   - Implement Markdown, filesystem operations, HTTP, or terminal grammar.
# - Allows:
#   - Producing one deterministic export report after successful persistence.
# - Split-When:
#   - Validation and persistence become independent application use cases.
# - Merge-When:
#   - The translator service becomes the explicit owner of generated skills.
# - Summary:
#   - Exports the live Unreal MCP catalog as repository skills.
# - Description:
#   - Coordinates ports without leaking adapter details inward.
# - Usage:
#   - Composed by the terminal driving adapter for the `skills` command.
# - Defaults:
#   - Fails before persistence when taxonomy or rendering is incomplete.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Application use case for exporting native Unreal MCP skills."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.src.domain.skill_documents import (
    SkillExportReport,
    interface_digest,
)
from mcp.src.domain.skill_taxonomy import CATEGORIES

if TYPE_CHECKING:
    from mcp.src.application.service import UnrealMcpTranslator
    from mcp.src.ports.skill_documents import (
        SkillDocumentRenderer,
        SkillDocumentStore,
    )


class UnrealSkillExporter:
    """Export one live catalog through injected renderer and store ports."""

    def __init__(
        self,
        translator: UnrealMcpTranslator,
        renderer: SkillDocumentRenderer,
        store: SkillDocumentStore,
    ) -> None:
        """Create one skill export use case."""
        self._translator = translator
        self._renderer = renderer
        self._store = store

    def export(self) -> SkillExportReport:
        """Discover, render, persist, and summarize one live catalog.

        Returns:
            Counts, digest, and output path for the completed export.
        """
        catalog = self._translator.discover_catalog()
        documents = self._renderer.render(catalog)
        self._store.replace(documents)
        return SkillExportReport(
            category_count=len(CATEGORIES),
            document_count=len(documents),
            interface_digest=interface_digest(catalog),
            output_path=self._store.display_path,
            tool_count=sum(len(toolset.tools) for toolset in catalog),
            toolset_count=len(catalog),
        )

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
#   - Skill export application service.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill export application service.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill export application service."""

from __future__ import annotations

from typing import TYPE_CHECKING

from mcp.domain.skill_documents import SkillExportReport
from mcp.domain.skill_documents import interface_digest
from mcp.domain.skill_taxonomy import CATEGORIES

if TYPE_CHECKING:
    from mcp.application.service import UnrealMcpTranslator
    from mcp.port_outbound.skill_documents import SkillDocumentRenderer
    from mcp.port_outbound.skill_documents import SkillDocumentStore


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

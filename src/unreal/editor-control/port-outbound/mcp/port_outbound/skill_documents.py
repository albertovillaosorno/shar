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
#   - Skill documents outbound port.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill documents outbound port.
# - Description:
#   - Implements the declared outbound port responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill documents outbound port."""

from __future__ import annotations

from typing import Protocol
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from mcp.domain.catalog import ToolsetDefinition
    from mcp.domain.skill_documents import SkillDocument


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

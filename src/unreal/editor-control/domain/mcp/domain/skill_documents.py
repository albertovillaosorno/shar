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
#   - Skill documents domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill documents domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill documents domain module."""

from __future__ import annotations

import hashlib
import json
from typing import NamedTuple
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from mcp.domain.catalog import ToolsetDefinition


class SkillDocument(NamedTuple):
    """One complete generated Markdown document."""

    relative_path: str
    content: str


class SkillExportReport(NamedTuple):
    """Deterministic summary of one generated skill export."""

    category_count: int
    document_count: int
    interface_digest: str
    output_path: str
    tool_count: int
    toolset_count: int


def interface_digest(catalog: tuple[ToolsetDefinition, ...]) -> str:
    """Return a stable digest of exact live toolset schemas."""
    payload = [
        {
            "name": toolset.name,
            "schema": toolset.raw_schema,
        }
        for toolset in sorted(catalog, key=lambda item: item.name)
    ]
    serialized = json.dumps(
        payload,
        ensure_ascii=True,
        separators=(",", ":"),
        sort_keys=True,
    ).encode("utf-8")
    return hashlib.sha256(serialized).hexdigest()

# File:
#   - skill_documents.py
# Path:
#   - src/mcp/src/domain/skill_documents.py
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
#   - Immutable generated-skill document and export report values.
# - Must-Not:
#   - Render Markdown, open files, connect to Unreal, or parse CLI input.
# - Allows:
#   - Carrying validated relative paths and complete text between layers.
# - Split-When:
#   - Export reporting and document identity evolve independently.
# - Merge-When:
#   - Another domain module owns the same immutable skill values.
# - Summary:
#   - Defines generated Unreal skill document values.
# - Description:
#   - Keeps application and adapters independent from filesystem details.
# - Usage:
#   - Returned by renderers and persisted through skill document ports.
# - Defaults:
#   - Documents are UTF-8 Markdown with repository-relative paths.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Immutable values for generated Unreal MCP skill documents."""

from __future__ import annotations

import hashlib
import json
from typing import TYPE_CHECKING, NamedTuple

if TYPE_CHECKING:
    from mcp.src.domain.catalog import ToolsetDefinition


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

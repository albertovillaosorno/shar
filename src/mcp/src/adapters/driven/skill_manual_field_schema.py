# File:
#   - skill_manual_field_schema.py
# Path:
#   - src/mcp/src/adapters/driven/skill_manual_field_schema.py
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
#   - Stable manual-field identities, order, placeholders, and marker text.
# - Must-Not:
#   - Parse documents, merge content, render status, or access files.
# - Allows:
#   - Defining the legacy five-field schema for lossless migration.
# - Split-When:
#   - Marker syntax and field definitions need independent versioning.
# - Merge-When:
#   - Another module owns the same protected field schema.
# - Summary:
#   - Defines the generated Unreal skill manual-field schema.
# - Description:
#   - The review revision is the sixth and final protected field.
# - Usage:
#   - Imported by renderers, parsers, stores, and regression tests.
# - Defaults:
#   - New review fields start as `[REVIEW_REQUIRED]`.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Stable protected field schema for generated Unreal MCP skills."""

from __future__ import annotations

from typing import NamedTuple

from mcp.src.adapters.driven.skill_manual_review import (
    MANUAL_REVIEW_FIELD_KEY,
    MANUAL_REVIEW_PLACEHOLDER,
)


class ManualFieldDefinition(NamedTuple):
    """One stable protected field in every generated tool skill."""

    key: str
    title: str
    placeholder: str


MANUAL_FIELDS = (
    ManualFieldDefinition(
        key="project-use-cases",
        title="SHAR-specific use cases",
        placeholder="[TODO]",
    ),
    ManualFieldDefinition(
        key="project-prerequisites",
        title="Project prerequisites",
        placeholder="[TODO]",
    ),
    ManualFieldDefinition(
        key="validated-arguments",
        title="Validated argument example",
        placeholder="[FILL_ME]",
    ),
    ManualFieldDefinition(
        key="project-verification",
        title="Project verification notes",
        placeholder="[TODO]",
    ),
    ManualFieldDefinition(
        key="known-caveats",
        title="Known project caveats",
        placeholder="[TODO]",
    ),
    ManualFieldDefinition(
        key=MANUAL_REVIEW_FIELD_KEY,
        title="Manual guidance reviewed revision",
        placeholder=MANUAL_REVIEW_PLACEHOLDER,
    ),
)
LEGACY_MANUAL_FIELDS = MANUAL_FIELDS[:-1]
MANUAL_KEYS = frozenset(field.key for field in MANUAL_FIELDS)
LEGACY_MANUAL_KEYS = frozenset(field.key for field in LEGACY_MANUAL_FIELDS)


def begin_marker(key: str) -> str:
    """Return one protected field's opening marker."""
    return f"<!-- BEGIN MANUAL FIELD: {key} -->"


def end_marker(key: str) -> str:
    """Return one protected field's closing marker."""
    return f"<!-- END MANUAL FIELD: {key} -->"


def expected_events(
    fields: tuple[ManualFieldDefinition, ...],
) -> tuple[tuple[str, str], ...]:
    """Return the required ordered marker events for one schema revision."""
    return tuple(
        event
        for field in fields
        for event in (("BEGIN", field.key), ("END", field.key))
    )

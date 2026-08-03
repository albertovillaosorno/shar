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
#   - Skill manual field schema outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill manual field schema outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill manual field schema outbound adapter."""

from __future__ import annotations

from typing import NamedTuple

from mcp.adapter_outbound.skill_manual_review import MANUAL_REVIEW_FIELD_KEY
from mcp.adapter_outbound.skill_manual_review import MANUAL_REVIEW_PLACEHOLDER


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

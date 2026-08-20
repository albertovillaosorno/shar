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
#   - Test skill manual review test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test skill manual review test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test skill manual review test module."""

from __future__ import annotations

from manual_skill_fixture import CURRENT_REVISION
from manual_skill_fixture import document
from manual_skill_fixture import legacy_document
from manual_skill_fixture import merge_and_extract
from manual_skill_fixture import replace_field
from mcp.adapter_outbound.skill_manual_fields import merge_manual_fields
from mcp.adapter_outbound.skill_manual_review import MANUAL_REVIEW_FIELD_KEY
from mcp.adapter_outbound.skill_manual_review import MANUAL_REVIEW_PLACEHOLDER


def test_merge_upgrades_pre_marker_file_with_placeholders() -> None:
    """The first regeneration of pre-marker files creates safe defaults."""
    generated = document("new generated purpose")

    merged = merge_manual_fields(
        generated,
        "# Legacy generated skill\n\nNo protected markers existed yet.\n",
        context="capabilities/legacy.md",
    )

    assert merged == generated
    assert merged.count("[TODO]") == 4
    assert merged.count("[FILL_ME]") == 1
    assert merged.count(MANUAL_REVIEW_PLACEHOLDER) == 1
    assert "- Manual guidance status: **Review required**" in merged


def test_merge_migrates_legacy_five_field_skill_without_data_loss() -> None:
    """Legacy guidance gains the review field and remains review-required."""
    existing = legacy_document("old generated purpose")
    existing = replace_field(
        existing,
        "project-use-cases",
        "Preserve this exact project guidance.",
    )

    merged, values = merge_and_extract(
        existing,
        context="capabilities/legacy-five-fields.md",
    )

    assert values["project-use-cases"] == (
        "Preserve this exact project guidance."
    )
    assert values[MANUAL_REVIEW_FIELD_KEY] == MANUAL_REVIEW_PLACEHOLDER
    assert "- Manual guidance status: **Review required**" in merged


def test_matching_review_revision_marks_manual_guidance_current() -> None:
    """Only an exact protected revision token marks guidance current."""
    existing = replace_field(
        document("old generated purpose"),
        MANUAL_REVIEW_FIELD_KEY,
        CURRENT_REVISION,
    )

    merged = merge_manual_fields(
        document("new generated purpose"),
        existing,
        context="capabilities/current.md",
    )

    assert "- Manual guidance status: **Current**" in merged
    assert "- Manual guidance status: **Review required**" not in merged

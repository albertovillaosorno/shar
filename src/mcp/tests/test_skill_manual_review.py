# File:
#   - test_skill_manual_review.py
# Path:
#   - src/mcp/tests/test_skill_manual_review.py
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
#   - Regression tests for manual-review revision migration and status.
# - Must-Not:
#   - Test malformed marker parsing, filesystem replacement, or live Unreal.
# - Allows:
#   - Current, pre-marker, and legacy five-field generated skill fixtures.
# - Split-When:
#   - Review status gains another independent lifecycle state.
# - Merge-When:
#   - Manual-field parser tests become the sole review-status owner.
# - Summary:
#   - Guards fail-safe version-aware manual guidance review state.
# - Description:
#   - Proves legacy data survives and only exact revision matches are current.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Legacy and new guidance require review until explicitly revalidated.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Manual-review revision migration and status regression tests."""

from __future__ import annotations

from mcp.src.adapters.driven.skill_manual_fields import (
    merge_manual_fields,
)
from mcp.src.adapters.driven.skill_manual_review import (
    MANUAL_REVIEW_FIELD_KEY,
    MANUAL_REVIEW_PLACEHOLDER,
)

from tests.manual_skill_fixture import (
    CURRENT_REVISION,
    document,
    legacy_document,
    merge_and_extract,
    replace_field,
)


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

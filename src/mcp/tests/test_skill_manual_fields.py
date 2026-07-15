# File:
#   - test_skill_manual_fields.py
# Path:
#   - src/mcp/tests/test_skill_manual_fields.py
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
#   - Regression tests for protected manual fields in generated Unreal skills.
# - Must-Not:
#   - Access repository skills, connect to Unreal, or test filesystem cleanup.
# - Allows:
#   - Pure generated templates and deliberately malformed marker fixtures.
# - Split-When:
#   - Marker parsing and merge evolution require independent test fixtures.
# - Merge-When:
#   - Another test module owns the same manual-field preservation contract.
# - Summary:
#   - Guards default placeholders and lossless human-content preservation.
# - Description:
#   - Proves malformed markers fail before generated replacement.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Uses the complete current protected field schema.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated Unreal skill manual-field regression tests
#   - reason: defaults, merge, and fail-closed parsing share one contract
#   - split: separate parser failures if marker schema becomes versioned
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess before changing the protected field schema
#
"""Regression tests for protected manual fields in generated tool skills."""

from __future__ import annotations

import pytest
from mcp.src.adapters.driven.skill_manual_field_schema import MANUAL_FIELDS
from mcp.src.adapters.driven.skill_manual_fields import (
    merge_manual_fields,
    render_manual_section,
)
from mcp.src.adapters.driven.skill_manual_review import (
    MANUAL_REVIEW_FIELD_KEY,
    MANUAL_REVIEW_PLACEHOLDER,
)
from mcp.src.domain.errors import ProtocolError

from tests.manual_skill_fixture import (
    CURRENT_REVISION,
    begin_marker,
    document,
    end_marker,
    field_block,
    merge_and_extract,
    replace_field,
)


def test_manual_section_starts_with_explicit_placeholders() -> None:
    """New tools expose every stable field without invented human guidance."""
    content = "\n".join(render_manual_section(CURRENT_REVISION))

    assert content.count("[TODO]") == 4
    assert content.count("[FILL_ME]") == 1
    assert content.count(MANUAL_REVIEW_PLACEHOLDER) == 1
    assert f"- Current revision: `{CURRENT_REVISION}`" in content
    assert "- Manual guidance status: **Review required**" in content
    for field in MANUAL_FIELDS:
        assert content.count(begin_marker(field.key)) == 1
        assert content.count(end_marker(field.key)) == 1
        assert f"### {field.title}" in content


def test_merge_preserves_human_fields_during_refresh() -> None:
    """Regeneration replaces machine text while retaining every human value."""
    old_generated = document("old generated purpose")
    existing = old_generated
    existing = replace_field(
        existing,
        "project-use-cases",
        "Use this after map import.\nKeep the target world open.",
    )
    existing = replace_field(
        existing,
        "project-prerequisites",
        "- The editor is idle.\n- The asset exists.",
    )
    existing = replace_field(
        existing,
        "validated-arguments",
        '```json\n{"asset": "/Game/Test"}\n```',
    )
    existing = replace_field(
        existing,
        "project-verification",
        "Re-read the created asset and inspect the editor log.",
    )
    existing = replace_field(
        existing,
        "known-caveats",
        "Do not call while compilation is active.",
    )

    merged, values = merge_and_extract(
        existing,
        context="capabilities/example.md",
    )

    assert "new generated purpose" in merged
    assert "old generated purpose" not in merged
    assert values["project-use-cases"] == (
        "Use this after map import.\nKeep the target world open."
    )
    assert values["project-prerequisites"] == (
        "- The editor is idle.\n- The asset exists."
    )
    assert values["validated-arguments"] == (
        '```json\n{"asset": "/Game/Test"}\n```'
    )
    assert values["project-verification"] == (
        "Re-read the created asset and inspect the editor log."
    )
    assert values["known-caveats"] == (
        "Do not call while compilation is active."
    )
    assert values[MANUAL_REVIEW_FIELD_KEY] == MANUAL_REVIEW_PLACEHOLDER
    assert "- Manual guidance status: **Review required**" in merged


def test_merge_rejects_incomplete_manual_field_set() -> None:
    """Deleting one protected field cannot silently reset human content."""
    existing = document("old generated purpose").replace(
        end_marker("known-caveats"),
        "",
    )

    with pytest.raises(ProtocolError, match="markers are out of order"):
        _ = merge_manual_fields(
            document("new generated purpose"),
            existing,
            context="capabilities/incomplete.md",
        )


def test_merge_rejects_malformed_manual_field_marker() -> None:
    """Malformed marker syntax cannot be discarded during regeneration."""
    existing = document("old generated purpose").replace(
        begin_marker("project-use-cases"),
        "<!-- BEGIN MANUAL FIELD: project_use_cases -->",
    )

    with pytest.raises(ProtocolError, match="malformed manual field marker"):
        _ = merge_manual_fields(
            document("new generated purpose"),
            existing,
            context="capabilities/malformed.md",
        )


def test_malformed_marker_error_does_not_reflect_controls() -> None:
    """Malformed marker text cannot inject terminal control characters."""
    existing = document("old generated purpose").replace(
        begin_marker("project-use-cases"),
        "<!-- BEGIN MANUAL FIELD: project-use-cases\x07 -->",
    )

    with pytest.raises(ProtocolError) as caught:
        _ = merge_manual_fields(
            document("new generated purpose"),
            existing,
            context="capabilities/malformed.md",
        )

    assert str(caught.value) == (
        "capabilities/malformed.md: existing skill: "
        "malformed manual field marker"
    )


def test_merge_rejects_reordered_manual_field_pairs() -> None:
    """Protected field pairs retain one stable document order."""
    existing = document("old generated purpose")
    use_cases = field_block("project-use-cases", "[TODO]")
    prerequisites = field_block("project-prerequisites", "[TODO]")
    existing = existing.replace(use_cases, "__MANUAL_FIELD_SWAP__")
    existing = existing.replace(prerequisites, use_cases)
    existing = existing.replace("__MANUAL_FIELD_SWAP__", prerequisites)

    with pytest.raises(ProtocolError, match="markers are out of order"):
        _ = merge_manual_fields(
            document("new generated purpose"),
            existing,
            context="capabilities/reordered.md",
        )


def test_merge_rejects_unknown_manual_field_marker() -> None:
    """Unknown protected data stops regeneration instead of being dropped."""
    existing = document("old generated purpose")
    existing += (
        "\n<!-- BEGIN MANUAL FIELD: future-field -->\n"
        "Important human text.\n"
        "<!-- END MANUAL FIELD: future-field -->\n"
    )

    with pytest.raises(ProtocolError, match="unknown manual fields"):
        _ = merge_manual_fields(
            document("new generated purpose"),
            existing,
            context="capabilities/unknown.md",
        )

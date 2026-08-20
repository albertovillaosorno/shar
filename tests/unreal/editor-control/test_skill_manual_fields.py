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
#   - Test skill manual fields test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test skill manual fields test module.
# - Description:
#   - Implements the declared test module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test skill manual fields test module."""

from __future__ import annotations

from manual_skill_fixture import CURRENT_REVISION
from manual_skill_fixture import begin_marker
from manual_skill_fixture import document
from manual_skill_fixture import end_marker
from manual_skill_fixture import field_block
from manual_skill_fixture import merge_and_extract
from manual_skill_fixture import replace_field
from mcp.adapter_outbound.skill_manual_field_schema import MANUAL_FIELDS
from mcp.adapter_outbound.skill_manual_fields import merge_manual_fields
from mcp.adapter_outbound.skill_manual_fields import render_manual_section
from mcp.adapter_outbound.skill_manual_review import MANUAL_REVIEW_FIELD_KEY
from mcp.adapter_outbound.skill_manual_review import MANUAL_REVIEW_PLACEHOLDER
from mcp.domain.errors import ProtocolError
import pytest


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

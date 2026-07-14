# File:
#   - manual_skill_fixture.py
# Path:
#   - src/mcp/tests/manual_skill_fixture.py
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
#   - Shared synthetic generated-skill fixtures for manual-field tests.
# - Must-Not:
#   - Access repository skills, connect to Unreal, or perform assertions.
# - Allows:
#   - Building current and legacy marker layouts with deterministic content.
# - Split-When:
#   - Another fixture family requires different generated document structure.
# - Merge-When:
#   - Another test helper owns the same synthetic manual-skill contract.
# - Summary:
#   - Provides reusable manual-field document fixtures.
# - Description:
#   - Keeps parser and review-status test modules responsibility-focused.
# - Usage:
#   - Imported only by MCP manual-field regression tests.
# - Defaults:
#   - Uses one deterministic synthetic translator/interface revision.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Synthetic generated-skill fixtures for manual-field tests."""

from __future__ import annotations

from mcp.src.adapters.driven.skill_manual_fields import (
    extract_manual_fields,
    merge_manual_fields,
    render_manual_section,
)
from mcp.src.adapters.driven.skill_manual_review import (
    MANUAL_REVIEW_FIELD_KEY,
    MANUAL_REVIEW_PLACEHOLDER,
)

CURRENT_REVISION = "1.0.0/" + ("a" * 64)


def document(generated_purpose: str) -> str:
    """Build one current six-field generated skill fixture.

    Returns:
        Complete synthetic generated skill content.
    """
    lines = [
        "# Example tool",
        "",
        generated_purpose,
        "",
        *render_manual_section(CURRENT_REVISION),
        "",
        "## Generated suffix",
        "",
        "Current live schema output.",
    ]
    return "\n".join(lines) + "\n"


def legacy_document(generated_purpose: str) -> str:
    """Build the prior five-field layout without review metadata.

    Returns:
        Complete legacy generated skill content.
    """
    content = document(generated_purpose)
    review_block = field_block(
        MANUAL_REVIEW_FIELD_KEY,
        MANUAL_REVIEW_PLACEHOLDER,
    )
    review_section = (
        f"\n### Manual guidance reviewed revision\n\n{review_block}"
        f"\n- Current revision: `{CURRENT_REVISION}`"
        "\n- Manual guidance status: **Review required**"
    )
    return content.replace(review_section, "", 1)


def merge_and_extract(
    existing: str,
    *,
    context: str,
) -> tuple[str, dict[str, str]]:
    """Merge one existing fixture and extract its protected values.

    Returns:
        Refreshed content and its complete protected field mapping.
    """
    merged = merge_manual_fields(
        document("new generated purpose"),
        existing,
        context=context,
    )
    values = extract_manual_fields(
        merged,
        context=f"{context}: merged",
        require_complete=True,
    )
    return merged, values


def replace_field(content: str, key: str, value: str) -> str:
    """Replace one protected field value while preserving its markers.

    Returns:
        Content with the requested protected value replaced.
    """
    begin = begin_marker(key)
    end = end_marker(key)
    start = content.index(begin) + len(begin)
    finish = content.index(end)
    return f"{content[:start]}\n{value}\n{content[finish:]}"


def field_block(key: str, value: str) -> str:
    """Render one complete protected marker block.

    Returns:
        Opening marker, value, and closing marker.
    """
    newline = chr(10)
    return f"{begin_marker(key)}{newline}{value}{newline}{end_marker(key)}"


def begin_marker(key: str) -> str:
    """Return one opening manual-field marker."""
    return f"<!-- BEGIN MANUAL FIELD: {key} -->"


def end_marker(key: str) -> str:
    """Return one closing manual-field marker."""
    return f"<!-- END MANUAL FIELD: {key} -->"

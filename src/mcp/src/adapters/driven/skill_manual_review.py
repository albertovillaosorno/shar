# File:
#   - skill_manual_review.py
# Path:
#   - src/mcp/src/adapters/driven/skill_manual_review.py
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
#   - Generated manual-guidance revision and review-status Markdown.
# - Must-Not:
#   - Parse protected field markers, access files, or invoke Unreal tools.
# - Allows:
#   - Rendering, refreshing, and validating one deterministic review status.
# - Split-When:
#   - Review state gains another output format or lifecycle state.
# - Merge-When:
#   - Protected marker parsing owns generated review metadata too.
# - Summary:
#   - Marks preserved manual guidance current or review-required.
# - Description:
#   - A review is current only when its protected token exactly matches.
# - Usage:
#   - Called during skill rendering, merge, and index summary finalization.
# - Defaults:
#   - Missing and legacy review tokens require review.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Version-aware manual review status for generated Unreal MCP skills."""

from __future__ import annotations

import re
from typing import NamedTuple

from mcp.src.domain.errors import fail_protocol

MANUAL_REVIEW_FIELD_KEY = "manual-review-revision"
MANUAL_REVIEW_PLACEHOLDER = "[REVIEW_REQUIRED]"
_CURRENT_REVISION_PATTERN = re.compile(
    r"^- Current revision: `(?P<revision>[^`\r\n]+)`$",
    re.MULTILINE,
)
_STATUS_PATTERN = re.compile(
    r"^- Manual guidance status: \*\*(?P<status>Current|Review required)\*\*$",
    re.MULTILINE,
)


class ManualReviewState(NamedTuple):
    """One generated skill's current and reviewed revision state."""

    current_revision: str
    reviewed_revision: str
    is_current: bool


def render_manual_review_lines(
    current_revision: str,
    reviewed_revision: str,
) -> list[str]:
    """Render visible revision and status lines for one tool skill.

    Returns:
        Generated Markdown lines for revision and review status.
    """
    state = _state(current_revision, reviewed_revision)
    return [
        f"- Current revision: `{state.current_revision}`",
        (
            "- Manual guidance status: "
            f"**{_status_label(is_current=state.is_current)}**"
        ),
    ]


def refresh_manual_review_status(
    content: str,
    reviewed_revision: str,
    *,
    context: str,
) -> str:
    """Refresh generated status after protected review content is restored.

    Returns:
        Complete content with the derived review status refreshed.
    """
    current_revision = _extract_current_revision(content, context=context)
    replacement = (
        f"- Manual guidance status: "
        f"**{_status_label(is_current=reviewed_revision == current_revision)}**"
    )
    refreshed, count = _STATUS_PATTERN.subn(replacement, content)
    if count != 1:
        fail_protocol(
            f"{context}: expected one generated manual guidance status"
        )
    return refreshed


def manual_review_state(
    content: str,
    reviewed_revision: str,
    *,
    context: str,
) -> ManualReviewState:
    """Validate one persisted skill's review state.

    Returns:
        Parsed current and reviewed revision state.
    """
    current_revision = _extract_current_revision(content, context=context)
    status_matches = tuple(_STATUS_PATTERN.finditer(content))
    if len(status_matches) != 1:
        fail_protocol(
            f"{context}: expected one generated manual guidance status"
        )
    status = status_matches[0].group("status")
    state = _state(current_revision, reviewed_revision)
    if status != _status_label(is_current=state.is_current):
        fail_protocol(f"{context}: manual guidance status is inconsistent")
    return state


def _extract_current_revision(content: str, *, context: str) -> str:
    matches = tuple(_CURRENT_REVISION_PATTERN.finditer(content))
    if len(matches) != 1:
        fail_protocol(f"{context}: expected one current revision")
    revision = matches[0].group("revision")
    if revision is None or not revision:
        fail_protocol(f"{context}: current revision is empty")
    return revision


def _state(current_revision: str, reviewed_revision: str) -> ManualReviewState:
    return ManualReviewState(
        current_revision=current_revision,
        reviewed_revision=reviewed_revision,
        is_current=reviewed_revision == current_revision,
    )


def _status_label(*, is_current: bool) -> str:
    return "Current" if is_current else "Review required"

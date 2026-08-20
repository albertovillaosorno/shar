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
#   - Skill manual review outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill manual review outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill manual review outbound adapter."""

from __future__ import annotations

import re
from typing import NamedTuple

from mcp.adapter_outbound.skill_markdown_policy import render_unbreakable_line
from mcp.domain.errors import fail_protocol

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
        *render_unbreakable_line(
            f"- Current revision: `{state.current_revision}`"
        ),
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

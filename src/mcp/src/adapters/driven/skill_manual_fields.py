# File:
#   - skill_manual_fields.py
# Path:
#   - src/mcp/src/adapters/driven/skill_manual_fields.py
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
#   - Stable human-owned field markers and generated-skill merge behavior.
# - Must-Not:
#   - Access files, call Unreal, classify tools, or render live MCP metadata.
# - Allows:
#   - Rendering empty manual fields and preserving their exact authored content.
# - Split-When:
#   - Marker parsing and field-schema evolution require separate versioning.
# - Merge-When:
#   - Another adapter owns the same protected Markdown field contract.
# - Summary:
#   - Preserves manual guidance inside otherwise generated tool skills.
# - Description:
#   - Fails closed on malformed, duplicate, missing, or unknown field markers.
# - Usage:
#   - Renderers emit the template; the filesystem store merges existing values.
# - Defaults:
#   - New skills start with five content placeholders and one review token.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated skill manual-field preservation contract
#   - reason: field schema, marker parsing, and merge safety form one invariant
#   - split: extract schema migration if protected fields ever change
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess before adding, renaming, or removing a manual field
#
"""Protected human-authored fields for generated Unreal MCP tool skills."""

from __future__ import annotations

import re

from mcp.src.adapters.driven.skill_manual_field_schema import (
    LEGACY_MANUAL_FIELDS,
    LEGACY_MANUAL_KEYS,
    MANUAL_FIELDS,
    MANUAL_KEYS,
    begin_marker,
    end_marker,
    expected_events,
)
from mcp.src.adapters.driven.skill_manual_review import (
    MANUAL_REVIEW_FIELD_KEY,
    refresh_manual_review_status,
    render_manual_review_lines,
)
from mcp.src.domain.errors import fail_protocol

_MANUAL_HEADING = "## Human-authored guidance"
_MANUAL_NOTICE_LINES = (
    "Edit only between matching manual-field markers.",
    "Regeneration preserves those contents and refreshes everything else.",
    "A revision mismatch marks preserved guidance for human review.",
)
_MARKER_PATTERN = re.compile(
    r"""
    ^<!--[ ](?P<kind>BEGIN|END)[ ]MANUAL[ ]FIELD:[ ]
    (?P<key>[a-z0-9]+(?:-[a-z0-9]+)*)[ ]-->$
    """,
    re.MULTILINE | re.VERBOSE,
)
_MARKER_SENTINEL = "MANUAL FIELD:"


def render_manual_section(
    current_revision: str,
    values: dict[str, str] | None = None,
) -> list[str]:
    """Render every protected field with supplied or default content.

    Returns:
        Markdown lines for the complete human-authored guidance section.
    """
    resolved = values or {}
    unknown = frozenset(resolved) - MANUAL_KEYS
    if unknown:
        unknown_text = ", ".join(sorted(unknown))
        fail_protocol(
            f"manual skill values contain unknown fields: {unknown_text}"
        )
    lines = [_MANUAL_HEADING, "", *_MANUAL_NOTICE_LINES]
    for field in MANUAL_FIELDS:
        value = resolved.get(field.key, field.placeholder)
        lines.extend(
            [
                "",
                f"### {field.title}",
                "",
                begin_marker(field.key),
                *(_content_lines(value)),
                end_marker(field.key),
            ]
        )
        if field.key == MANUAL_REVIEW_FIELD_KEY:
            lines.extend(
                [
                    "",
                    *render_manual_review_lines(
                        current_revision,
                        value,
                    ),
                ]
            )
    return lines


def merge_manual_fields(
    generated_content: str,
    existing_content: str | None,
    *,
    context: str,
) -> str:
    """Merge protected values from an existing skill into a fresh template.

    Returns:
        Fresh generated content with exact existing manual values restored.
    """
    generated_values = extract_manual_fields(
        generated_content,
        context=f"{context}: generated template",
        require_complete=True,
    )
    existing_values = (
        extract_manual_fields(
            existing_content,
            context=f"{context}: existing skill",
            require_complete=False,
        )
        if existing_content is not None
        else {}
    )
    merged = (
        _inject_manual_fields(
            generated_content,
            existing_values,
            context=context,
        )
        if existing_values
        else generated_content
    )
    merged_values = extract_manual_fields(
        merged,
        context=f"{context}: merged skill",
        require_complete=True,
    )
    reviewed_revision = merged_values.get(
        MANUAL_REVIEW_FIELD_KEY,
        generated_values[MANUAL_REVIEW_FIELD_KEY],
    )
    return refresh_manual_review_status(
        merged,
        reviewed_revision,
        context=context,
    )


def extract_manual_fields(
    content: str,
    *,
    context: str,
    require_complete: bool,
) -> dict[str, str]:
    """Extract and validate protected field values from one Markdown document.

    Returns:
        Exact field contents keyed by stable field identity.
    """
    _check_marker_lines(content, context=context)
    markers = tuple(_MARKER_PATTERN.finditer(content))
    if not markers:
        if require_complete:
            fail_protocol(f"{context}: manual field markers are missing")
        return {}
    marker_events = tuple(
        _marker_event(marker, context=context) for marker in markers
    )
    marker_keys = frozenset(key for _, key in marker_events)
    unknown = marker_keys - MANUAL_KEYS
    if unknown:
        unknown_text = ", ".join(sorted(unknown))
        fail_protocol(f"{context}: unknown manual fields: {unknown_text}")
    current_events = expected_events(MANUAL_FIELDS)
    legacy_events = expected_events(LEGACY_MANUAL_FIELDS)
    if marker_events == current_events:
        fields = MANUAL_FIELDS
    elif not require_complete and marker_events == legacy_events:
        fields = LEGACY_MANUAL_FIELDS
    else:
        if require_complete and marker_keys != MANUAL_KEYS:
            missing_text = ", ".join(sorted(MANUAL_KEYS - marker_keys))
            fail_protocol(f"{context}: missing manual fields: {missing_text}")
        allowed_keys = (MANUAL_KEYS, LEGACY_MANUAL_KEYS)
        if not require_complete and marker_keys not in allowed_keys:
            missing_text = ", ".join(sorted(MANUAL_KEYS - marker_keys))
            fail_protocol(
                f"{context}: incomplete manual field set: {missing_text}"
            )
        fail_protocol(f"{context}: manual field markers are out of order")
    return {
        field.key: _extract_field(content, field.key, context=context)
        for field in fields
    }


def _extract_field(content: str, key: str, *, context: str) -> str:
    begin = begin_marker(key)
    end = end_marker(key)
    if content.count(begin) != 1 or content.count(end) != 1:
        fail_protocol(
            f"{context}: manual field `{key}` must have one marker pair"
        )
    begin_index = content.index(begin)
    end_index = content.index(end)
    if begin_index >= end_index:
        fail_protocol(
            f"{context}: manual field `{key}` markers are out of order"
        )
    content_start = begin_index + len(begin)
    if content[content_start : content_start + 1] != "\n":
        fail_protocol(
            f"{context}: manual field `{key}` must start on a new line"
        )
    value = content[content_start + 1 : end_index]
    if not value.endswith("\n"):
        fail_protocol(f"{context}: manual field `{key}` must end on a new line")
    return value[:-1]


def _inject_manual_fields(
    content: str,
    values: dict[str, str],
    *,
    context: str,
) -> str:
    merged = content
    for field in MANUAL_FIELDS:
        value = values.get(field.key, field.placeholder)
        begin = begin_marker(field.key)
        end = end_marker(field.key)
        start = merged.index(begin) + len(begin)
        finish = merged.index(end)
        replacement = f"\n{value}\n"
        merged = f"{merged[:start]}{replacement}{merged[finish:]}"
    _ = extract_manual_fields(
        merged,
        context=f"{context}: merged skill",
        require_complete=True,
    )
    return merged


def _check_marker_lines(content: str, *, context: str) -> None:
    for line in content.splitlines():
        if _MARKER_SENTINEL not in line:
            continue
        if _MARKER_PATTERN.fullmatch(line) is None:
            fail_protocol(f"{context}: malformed manual field marker")


def _marker_event(
    match: re.Match[str],
    *,
    context: str,
) -> tuple[str, str]:
    kind = match.group("kind")
    key = match.group("key")
    if kind is None or key is None:
        fail_protocol(f"{context}: manual field marker is incomplete")
    return kind, key


def _content_lines(value: str) -> list[str]:
    return value.splitlines() or [""]

# File:
#   - skill_markdown_policy.py
# Path:
#   - src/mcp/src/adapters/driven/skill_markdown_policy.py
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
#   - Narrow Markdown lint guards for generated unbreakable lines.
# - Must-Not:
#   - Wrap prose, render documents, access files, or invoke external tools.
# - Allows:
#   - Guarding stable hashes, revision tokens, and Markdown destinations.
# - Split-When:
#   - Generated Markdown gains another independently configured lint policy.
# - Merge-When:
#   - Another adapter owns the same generated-line lint contract.
# - Summary:
#   - Preserves strict line-length validation for unbreakable generated values.
# - Description:
#   - Emits one exact next-line marker only when a stable line exceeds the
#     canonical 80-column limit.
# - Usage:
#   - Called by generated skill renderers for values that cannot be wrapped.
# - Defaults:
#   - Lines at or below the canonical limit are emitted without a marker.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Narrow line-length policy for generated Unreal MCP Markdown."""

from __future__ import annotations

_MARKDOWN_LINE_LIMIT = 80
_MARKDOWNLINT_MD013_NEXT_LINE = "<!-- markdownlint-disable-next-line MD013 -->"


def render_unbreakable_line(line: str) -> tuple[str, ...]:
    """Render one stable line with an exact line-length guard when required.

    The caller must use this only for a value whose bytes cannot be wrapped
    without changing a machine-readable token or Markdown destination.

    Returns:
        The line alone, or one exact MD013 marker followed by the line.
    """
    if len(line) <= _MARKDOWN_LINE_LIMIT:
        return (line,)
    return (_MARKDOWNLINT_MD013_NEXT_LINE, line)

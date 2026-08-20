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
#   - Skill markdown policy outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill markdown policy outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill markdown policy outbound adapter."""

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

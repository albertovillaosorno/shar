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
#   - Skill native identity outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill native identity outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill native identity outbound adapter."""

from __future__ import annotations

import re

from mcp.domain.errors import fail_protocol

_TOOL_PATTERN = re.compile(
    r"^Tool:\n\n```text\n(?P<identity>[^\n]+)\n```$",
    re.MULTILINE,
)


def extract_native_tool_identity(content: str, *, context: str) -> str:
    """Return the unique native tool identity in one generated skill.

    Returns:
        The complete registry-qualified native tool identity.

    """
    matches = tuple(_TOOL_PATTERN.finditer(content))
    if len(matches) != 1:
        fail_protocol(
            f"{context}: generated skill must contain one native Tool block"
        )
    identity = matches[0].group("identity").strip()
    if not identity:
        fail_protocol(f"{context}: generated skill Tool identity is empty")
    return identity

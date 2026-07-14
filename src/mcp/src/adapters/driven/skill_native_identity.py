# File:
#   - skill_native_identity.py
# Path:
#   - src/mcp/src/adapters/driven/skill_native_identity.py
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
#   - Parsing the stable native tool identity from one generated skill.
# - Must-Not:
#   - Read files, render Markdown, merge manual fields, or invoke Unreal.
# - Allows:
#   - Identity-based migration when generated taxonomy paths change.
# - Split-When:
#   - Toolset and tool identities become independently migrated surfaces.
# - Merge-When:
#   - Another adapter owns the same generated identity marker contract.
# - Summary:
#   - Extracts one native tool identity from generated Markdown.
# - Description:
#   - Uses the generated Native identities Tool block as stable ownership.
# - Usage:
#   - Called before filesystem mutation by the generated skill store.
# - Defaults:
#   - Requires exactly one non-empty Tool identity block.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Native tool identity parsing for generated Unreal MCP skills."""

from __future__ import annotations

import re

from mcp.src.domain.errors import fail_protocol

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

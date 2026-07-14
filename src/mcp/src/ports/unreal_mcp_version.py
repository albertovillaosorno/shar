# File:
#   - unreal_mcp_version.py
# Path:
#   - src/mcp/src/ports/unreal_mcp_version.py
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
#   - Application-facing discovery contract for the Unreal MCP plugin version.
# - Must-Not:
#   - Parse descriptors, inspect installations, or define version values.
# - Allows:
#   - Decoupling skill generation from installed Unreal filesystem layout.
# - Split-When:
#   - More plugin metadata requires an independent port.
# - Merge-When:
#   - Another port owns the same Unreal MCP installation metadata.
# - Summary:
#   - Defines the single Unreal MCP version discovery port.
# - Description:
#   - The implementation reads the active engine plugin descriptor.
# - Usage:
#   - Composed by the CLI only for generated skill export.
# - Defaults:
#   - No version fallback is permitted.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - false
#
"""Port for resolving the installed Unreal MCP plugin version."""

from __future__ import annotations

from typing import Protocol


class UnrealMcpVersionProvider(Protocol):
    """Resolve one normalized installed Unreal MCP plugin version."""

    def read_version(self) -> str:
        """Return the canonical normalized plugin version."""
        ...

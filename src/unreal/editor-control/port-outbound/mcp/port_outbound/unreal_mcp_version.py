# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Unreal mcp version outbound port.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Unreal mcp version outbound port.
# - Description:
#   - Implements the declared outbound port responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Unreal mcp version outbound port."""

from __future__ import annotations

from typing import Protocol


class UnrealMcpVersionProvider(Protocol):
    """Resolve one normalized installed Unreal MCP plugin version."""

    def read_version(self) -> str:
        """Return the canonical normalized plugin version."""
        ...

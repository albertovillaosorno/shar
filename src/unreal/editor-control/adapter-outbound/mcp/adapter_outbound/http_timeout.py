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
#   - Http timeout outbound adapter.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Http timeout outbound adapter.
# - Description:
#   - Implements the declared responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Http timeout outbound adapter."""

from __future__ import annotations

import math

from mcp.domain.errors import fail_configuration


def resolve_timeout_seconds(
    default_seconds: float,
    override_seconds: float | None,
) -> float:
    """Return one positive exchange timeout.

    Returns:
        The override when present, otherwise the configured default.

    """
    resolved = default_seconds if override_seconds is None else override_seconds
    if not math.isfinite(resolved) or resolved <= 0:
        fail_configuration("exchange timeout must be finite and positive")
    return resolved

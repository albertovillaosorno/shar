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
#   - Renderer protocol consumed by deterministic platform icon packaging.
# - Must-Not:
#   - Select renderer implementations or perform filesystem or process effects.
# - Allows:
#   - Declare the exact SVG-to-PNG rendering interface.
# - Split-When:
#   - Another rendering capability requires an independent protocol.
# - Merge-When:
#   - Another port owns the identical SVG rasterization contract.
# - Summary:
#   - SVG renderer port for icon packaging.
# - Description:
#   - Defines the minimal square-PNG rendering dependency for platform export.
# - Usage:
#   - Implemented by local render adapters and consumed by PlatformExporter.
# - Defaults:
#   - Implementations return exact PNG bytes or fail explicitly.
#

"""Renderer port consumed by deterministic platform icon packaging."""

from __future__ import annotations

from pathlib import Path
from typing import Protocol


class SvgRenderer(Protocol):
    """Rasterize one SVG page to an exact square PNG size."""

    def render_png(self, svg: Path, size: int) -> bytes:
        """Return a PNG rendering of ``svg`` at ``size`` square pixels."""
        ...

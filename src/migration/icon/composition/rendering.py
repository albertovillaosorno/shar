# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT

"""Renderer port consumed by deterministic platform icon packaging."""

from __future__ import annotations

from pathlib import Path
from typing import Protocol


class SvgRenderer(Protocol):
    """Rasterize one SVG page to an exact square PNG size."""

    def render_png(self, svg: Path, size: int) -> bytes:
        """Return a PNG rendering of ``svg`` at ``size`` square pixels."""
        ...

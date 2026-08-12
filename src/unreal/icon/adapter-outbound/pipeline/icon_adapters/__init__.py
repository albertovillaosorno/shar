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
#   - Concrete adapters for the icon-tool ports.
# - Must-Not:
#   - Cross declared architecture boundaries or persist undeclared dependencies.
# - Allows:
#   - Inputs: values admitted by this module interface.
#   - Outputs: deterministic values or effects declared by that interface.
#   - Side effects: only those explicitly owned by the implementation.
# - Split-When:
#   - Split when another responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Concrete adapters for the icon-tool ports.
# - Description:
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Concrete adapters for the icon-tool ports."""

from .platform_export import PlatformExporter
from .source_bound import SourceBoundAuthorer, SourceBoundReconstructor
from .svg_renderer import AutoSvgRenderer

__all__ = [
    "AutoSvgRenderer",
    "PlatformExporter",
    "SourceBoundReconstructor",
    "SourceBoundAuthorer",
]

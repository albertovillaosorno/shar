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
#   - Stable cross-platform icon format contract.
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
#   - Stable cross-platform icon format contract.
# - Description:
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Stable cross-platform icon format contract."""

from .profiles import (
    ANDROID_DENSITY_SCALE,
    ANDROID_LEGACY,
    IOS_APPICON_SIZE,
    LINUX_SIZES,
    MAC_ICONSET,
    WINDOWS_SIZES,
)

__all__ = [
    "ANDROID_DENSITY_SCALE",
    "ANDROID_LEGACY",
    "IOS_APPICON_SIZE",
    "LINUX_SIZES",
    "MAC_ICONSET",
    "WINDOWS_SIZES",
]

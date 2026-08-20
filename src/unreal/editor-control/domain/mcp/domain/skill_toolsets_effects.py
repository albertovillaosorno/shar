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
#   - Skill toolsets effects domain module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Skill toolsets effects domain module.
# - Description:
#   - Implements the declared domain module responsibility for editor control.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Skill toolsets effects domain module."""

from __future__ import annotations

EFFECTS_TOOLSETS = (
    (
        "DataflowAgent",
        "DataflowAgentToolset",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_Assets",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_Blueprint",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_Component",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_Info",
    ),
    (
        "NiagaraToolsets",
        "NiagaraToolset_System",
    ),
    (
        "PCGToolset",
        "PCGSpatialToolset",
    ),
    (
        "PCGToolset",
        "PCGToolset",
    ),
    (
        "PhysicsToolsets",
        "PhysicsAssetToolset",
    ),
)

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
#   - Ports for authoring and executing the source-bound transform.
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
#   - Ports for authoring and executing the source-bound transform.
# - Description:
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Ports for authoring and executing the source-bound transform."""

from __future__ import annotations

from pathlib import Path
from typing import Protocol


class TransformAuthorer(Protocol):
    """Generate a distributable transform from local source + local oracle."""

    def author(
        self,
        source_files: tuple[Path, ...],
        oracle_root: Path,
        output_algorithm: Path,
    ) -> None: ...


class MasterReconstructor(Protocol):
    """Recover ignored masters from admitted original source evidence."""

    def reconstruct(
        self,
        source_files: tuple[Path, ...],
        algorithm: Path,
        output_root: Path,
    ) -> None: ...

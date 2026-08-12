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
#   - Application services for authoring and source-bound reconstruction.
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
#   - Application services for authoring and source-bound reconstruction.
# - Description:
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Application services for authoring and source-bound reconstruction."""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import shutil

from icon_domain import IconLayout
from icon_ports import MasterReconstructor, TransformAuthorer


@dataclass(slots=True)
class IconApplication:
    """Use cases independent from CLI and concrete I/O implementations."""

    layout: IconLayout
    authorer: TransformAuthorer
    reconstructor: MasterReconstructor

    def author(self) -> None:
        source_files = self._require_game_icons()
        self._require_nonempty_directory(self.layout.assets, "algorithm/assets")
        self.authorer.author(
            source_files,
            self.layout.assets,
            self.layout.transform,
        )

    def reconstruct(self) -> None:
        source_files = self._require_game_icons()
        transform_missing = not self.layout.transform.is_file()
        transform_empty = (
            not transform_missing
            and self.layout.transform.stat().st_size == 0
        )
        if transform_missing or transform_empty:
            raise RuntimeError(
                "composition/algorithm/main.rs is missing or empty; "
                "run `python main.py author` while local authored SVGs exist"
            )
        if self.layout.reconstructed_assets.exists():
            shutil.rmtree(self.layout.reconstructed_assets)
        self.reconstructor.reconstruct(
            source_files,
            self.layout.transform,
            self.layout.reconstructed_assets,
        )

    def _require_game_icons(self) -> tuple[Path, ...]:
        source_files = self.layout.game_icons
        if not source_files:
            raise RuntimeError(
                "no local source evidence matched game/*.ico under "
                f"{self.layout.game}"
            )
        return source_files

    @staticmethod
    def _require_nonempty_directory(path: Path, label: str) -> None:
        contains_file = path.is_dir() and any(
            item.is_file() for item in path.rglob("*")
        )
        if not contains_file:
            raise RuntimeError(
                f"{label} is missing or contains no files: {path}"
            )

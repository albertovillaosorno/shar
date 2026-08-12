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
#   - Repository-relative layout for the icon pipeline.
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
#   - Repository-relative layout for the icon pipeline.
# - Description:
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Repository-relative layout for the icon pipeline."""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass(frozen=True, slots=True)
class IconLayout:
    """Resolve every path from the checked-in ``src/unreal/icon`` boundary."""

    root: Path

    @property
    def repository(self) -> Path:
        """Return the repository root without hard-coding a workstation path."""
        return self.root.parents[2]

    @property
    def game(self) -> Path:
        """Original locally extracted game directory used as evidence."""
        return self.repository / "game"

    @property
    def game_icons(self) -> tuple[Path, ...]:
        """Return deterministic local ``game/*.ico`` source evidence."""
        if not self.game.is_dir():
            return ()
        return tuple(
            sorted(
                self.game.glob("*.ico"),
                key=lambda path: path.name.lower(),
            )
        )

    @property
    def algorithm(self) -> Path:
        return self.root / "composition" / "algorithm"

    @property
    def assets(self) -> Path:
        """Ignored authoring oracle used only to regenerate ``main.rs``."""
        return self.algorithm / "assets"

    @property
    def output(self) -> Path:
        """Ignored reconstruction and platform-export directory."""
        return self.algorithm / "out"

    @property
    def reconstructed_assets(self) -> Path:
        return self.output / "assets"

    @property
    def export(self) -> Path:
        return self.output / "export"

    @property
    def transform(self) -> Path:
        return self.algorithm / "main.rs"

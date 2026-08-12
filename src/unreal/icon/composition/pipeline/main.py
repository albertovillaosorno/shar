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
#   - Public entry point for the SHAR cross-platform icon pipeline.
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
#   - Public entry point for the SHAR cross-platform icon pipeline.
# - Description:
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Public entry point for the SHAR cross-platform icon pipeline."""

from __future__ import annotations

from pathlib import Path
import sys

_ROOT = Path(__file__).resolve().parents[2]
_PART_ROOTS = (
    _ROOT / "domain" / "pipeline",
    _ROOT / "contract" / "pipeline",
    _ROOT / "port-outbound" / "pipeline",
    _ROOT / "application" / "pipeline",
    _ROOT / "adapter-outbound" / "pipeline",
    _ROOT / "composition" / "pipeline",
)
for _part_root in reversed(_PART_ROOTS):
    sys.path.insert(0, str(_part_root))

from icon_composition.cli import main  # noqa: E402


if __name__ == "__main__":
    raise SystemExit(main(_ROOT))

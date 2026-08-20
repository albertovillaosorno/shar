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
#   - Explicit local CLI for reconstructed SVG platform export.
# - Must-Not:
#   - Read lawful source files directly or write outside the requested output.
# - Allows:
#   - Parse explicit local paths and invoke the icon export application flow.
# - Split-When:
#   - Another icon export command gains an independent invocation contract.
# - Merge-When:
#   - Another adapter owns the identical platform-export CLI behavior.
# - Summary:
#   - Reconstructed SVG platform-export CLI.
# - Description:
#   - Parses export arguments and reports the selected local renderer backend.
# - Usage:
#   - Invoked explicitly after source-bound icon reconstruction.
# - Defaults:
#   - Function-local assets and output paths are used when omitted.
#

"""Explicit local CLI for SVG-to-platform icon export."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys

from platform_export import PlatformExporter
from svg_renderer import AutoSvgRenderer

_FUNCTION_ROOT = Path(__file__).resolve().parents[1]


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Export local authored SVGs to native platform icon assets."
    )
    parser.add_argument(
        "--assets",
        type=Path,
        default=_FUNCTION_ROOT / "assets",
        help="local SVG asset directory (default: function-local assets)",
    )
    parser.add_argument(
        "--out",
        type=Path,
        default=_FUNCTION_ROOT / "out",
        help="generated output directory (default: function-local out)",
    )
    parser.add_argument(
        "--icon-name",
        default="simpsons-hit-run",
        help="Linux application icon basename",
    )
    return parser


def main() -> int:
    """Run one explicit icon export without touching the lawful source tree."""
    arguments = _parser().parse_args()
    with AutoSvgRenderer() as renderer:
        exporter = PlatformExporter(renderer, icon_name=arguments.icon_name)
        exporter.export_all(arguments.assets, arguments.out)
        sys.stdout.write(f"icon export complete via {renderer.backend}\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

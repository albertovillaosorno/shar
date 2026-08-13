# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT

"""Explicit local CLI for SVG-to-platform icon export."""

from __future__ import annotations

import argparse
from pathlib import Path

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
        print(f"icon export complete via {renderer.backend}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

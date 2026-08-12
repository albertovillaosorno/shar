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
#   - Command-line composition root for the icon tool.
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
#   - Command-line composition root for the icon tool.
# - Description:
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Command-line composition root for the icon tool."""

from __future__ import annotations

import argparse
from pathlib import Path
import sys

from icon_adapters import (
    AutoSvgRenderer,
    PlatformExporter,
    SourceBoundAuthorer,
    SourceBoundReconstructor,
)
from icon_application import IconApplication
from icon_domain import IconLayout


def _parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="main.py",
        description=(
            "Author a source-bound SVG transform and export Windows, macOS, "
            "Linux, Android, and iOS icons. All paths derive from main.py."
        ),
    )
    parser.add_argument(
        "command",
        nargs="?",
        default="build",
        choices=(
            "author",
            "reconstruct",
            "export",
            "export-local",
            "build",
            "all",
        ),
        help=(
            "build = reconstruct + export (default); "
            "all = author + reconstruct + export"
        ),
    )
    parser.add_argument(
        "--name",
        default="simpsons-hit-run",
        help="Linux icon basename (default: simpsons-hit-run)",
    )
    return parser


def _export(layout: IconLayout, source: Path, icon_name: str) -> None:
    with AutoSvgRenderer() as renderer:
        exporter = PlatformExporter(renderer, icon_name=icon_name)
        notes = exporter.export_all(source, layout.export)
        backend = renderer.backend
    print(f"renderer: {backend}")
    print(f"export:   {layout.export}")
    for note in notes:
        print(f"  - {note}")


def run(tool_root: Path, argv: list[str] | None = None) -> int:
    args = _parser().parse_args(argv)
    layout = IconLayout(tool_root.resolve())
    application = IconApplication(
        layout=layout,
        authorer=SourceBoundAuthorer(),
        reconstructor=SourceBoundReconstructor(),
    )

    try:
        if args.command == "author":
            application.author()
            print(f"authored: {layout.transform}")
            return 0

        if args.command == "reconstruct":
            application.reconstruct()
            print(f"reconstructed: {layout.reconstructed_assets}")
            return 0

        if args.command == "export":
            _export(layout, layout.reconstructed_assets, args.name)
            return 0

        if args.command == "export-local":
            _export(layout, layout.assets, args.name)
            return 0

        if args.command == "all":
            application.author()
            print(f"authored: {layout.transform}")

        # `build` and the tail of `all` both prove the committed source-bound
        # transform before packaging the reconstructed assets.
        application.reconstruct()
        print(f"reconstructed: {layout.reconstructed_assets}")
        _export(layout, layout.reconstructed_assets, args.name)
        return 0
    except (OSError, RuntimeError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1


def main(tool_root: Path) -> int:
    return run(tool_root)

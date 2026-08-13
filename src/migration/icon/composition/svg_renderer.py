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
#   - SVG rasterizer with disposable, non-project dependencies.
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
#   - SVG rasterizer with disposable, non-project dependencies.
# - Description:
#   - Implements the declared responsibility for the migration icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""SVG rasterizer with disposable, non-project dependencies."""

from __future__ import annotations

import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

_RESVG_CHILD = (
    "import pathlib, sys, resvg_py; "
    "data=resvg_py.svg_to_bytes(svg_path=sys.argv[1], "
    "width=int(sys.argv[3]), height=int(sys.argv[3])); "
    "pathlib.Path(sys.argv[2]).write_bytes(data)"
)


class AutoSvgRenderer:
    """Use Inkscape, else provision resvg_py into a disposable directory."""

    def __init__(self) -> None:
        self._inkscape = shutil.which("inkscape")
        self._uv = shutil.which("uv")
        self._temporary: tempfile.TemporaryDirectory[str] | None = None
        self._site: Path | None = None
        self._cache: dict[tuple[str, int, int], bytes] = {}
        if self._inkscape is None:
            self._bootstrap_resvg()

    @property
    def backend(self) -> str:
        if self._inkscape:
            return "inkscape"
        if self._uv:
            return "temporary-resvg-py-via-uv"
        return "temporary-resvg-py-via-pip"

    def __enter__(self) -> "AutoSvgRenderer":
        return self

    def __exit__(self, _exc_type, _exc, _traceback) -> None:
        self.close()

    def close(self) -> None:
        self._cache.clear()
        if self._temporary is not None:
            self._temporary.cleanup()
            self._temporary = None
            self._site = None

    def _bootstrap_resvg(self) -> None:
        self._temporary = tempfile.TemporaryDirectory(prefix="shar-icon-deps-")
        root = Path(self._temporary.name)
        site = root / "site"
        site.mkdir()
        environment = os.environ.copy()
        environment.update(
            {
                "PIP_DISABLE_PIP_VERSION_CHECK": "1",
                "PIP_NO_CACHE_DIR": "1",
                "PYTHONDONTWRITEBYTECODE": "1",
                "UV_CACHE_DIR": str(root / "uv-cache"),
            }
        )
        if self._uv:
            command = [
                self._uv,
                "pip",
                "install",
                "--python",
                sys.executable,
                "--target",
                str(site),
                "--no-cache",
                "resvg_py==0.3.3",
            ]
        else:
            command = [
                sys.executable,
                "-m",
                "pip",
                "install",
                "--disable-pip-version-check",
                "--no-input",
                "--no-cache-dir",
                "--target",
                str(site),
                "resvg_py==0.3.3",
            ]
        try:
            subprocess.run(
                command,
                check=True,
                env=environment,
                stdout=subprocess.DEVNULL,
            )
        except (OSError, subprocess.CalledProcessError) as error:
            self.close()
            raise RuntimeError(
                "SVG rendering needs Inkscape or disposable resvg_py "
                "provisioning"
            ) from error
        self._site = site

    def render_png(self, svg: Path, size: int) -> bytes:
        """Render exactly ``size`` square pixels directly from the SVG page."""
        if size <= 0:
            raise ValueError("icon raster size must be positive")
        if not svg.is_file():
            raise RuntimeError(f"SVG input does not exist: {svg}")
        stat = svg.stat()
        key = (str(svg.resolve()), stat.st_mtime_ns, size)
        cached = self._cache.get(key)
        if cached is not None:
            return cached

        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as handle:
            target = Path(handle.name)
        try:
            command, environment = self._render_command(svg, target, size)
            subprocess.run(
                command,
                check=True,
                env=environment,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
            data = target.read_bytes()
        except (OSError, subprocess.CalledProcessError) as error:
            raise RuntimeError(f"SVG rasterization failed for {svg}") from error
        finally:
            target.unlink(missing_ok=True)
        self._cache[key] = data
        return data

    def _render_command(
        self, svg: Path, target: Path, size: int
    ) -> tuple[list[str], dict[str, str] | None]:
        if self._inkscape:
            return (
                [
                    self._inkscape,
                    str(svg),
                    "--export-type=png",
                    "--export-area-page",
                    f"--export-filename={target}",
                    f"--export-width={size}",
                    f"--export-height={size}",
                ],
                None,
            )
        if self._site is None:
            raise RuntimeError("temporary resvg_py environment is unavailable")
        environment = os.environ.copy()
        existing = environment.get("PYTHONPATH")
        environment["PYTHONPATH"] = (
            str(self._site)
            if not existing
            else os.pathsep.join((str(self._site), existing))
        )
        environment["PYTHONDONTWRITEBYTECODE"] = "1"
        return (
            [
                sys.executable,
                "-c",
                _RESVG_CHILD,
                str(svg),
                str(target),
                str(size),
            ],
            environment,
        )

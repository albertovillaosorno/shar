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
#   - Synthetic platform-icon export helper regression tests.
# - Must-Not:
#   - Download renderers, read lawful game sources, or write repository outputs.
# - Allows:
#   - Temporary SVG fixtures and direct helper assertions.
# - Split-When:
#   - Renderer provisioning needs independent integration fixtures.
# - Merge-When:
#   - Another test module owns identical platform-icon helper evidence.
# - Summary:
#   - Platform-icon helper regression tests.
# - Description:
#   - Proves temporary SVG composition and explicit missing-input diagnostics.
# - Usage:
#   - Runs through the repository packaging pytest gate.
# - Defaults:
#   - Every filesystem fixture is isolated below a temporary directory.
#

"""Regression tests for deterministic platform-icon helper behavior."""

from __future__ import annotations

import importlib
from pathlib import Path
import sys
import tempfile
import unittest

_ROOT = Path(__file__).resolve().parents[2]
_COMPOSITION = _ROOT / "src" / "migration" / "icon" / "composition"
sys.path.insert(0, str(_COMPOSITION))
try:
    _PLATFORM_EXPORT = importlib.import_module("platform_export")
finally:
    sys.path.remove(str(_COMPOSITION))


class PlatformIconHelperTests(unittest.TestCase):
    """Exercise temporary SVG helpers without a rasterizer or network access."""

    def test_safe_zone_expands_view_box_and_removes_temporary_svg(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-icon-test-") as root_text:
            source = Path(root_text) / "source.svg"
            source.write_text(
                '<svg viewBox="0 0 100 100"></svg>',
                encoding="utf-8",
            )
            with _PLATFORM_EXPORT.PlatformExporter._svg_inside_safe_zone(
                source,
                0.5,
            ) as temporary:
                self.assertTrue(temporary.is_file())
                text = temporary.read_text(encoding="utf-8")
                self.assertIn('viewBox="-50 -50 200 200"', text)
            self.assertFalse(temporary.exists())

    def test_background_composition_is_temporary(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-icon-test-") as root_text:
            source = Path(root_text) / "source.svg"
            source.write_text("<svg></svg>", encoding="utf-8")
            with _PLATFORM_EXPORT.PlatformExporter._svg_over_background(
                source,
                "#000000",
            ) as temporary:
                text = temporary.read_text(encoding="utf-8")
                self.assertIn('fill="#000000"', text)
            self.assertFalse(temporary.exists())

    def test_missing_required_asset_names_the_logical_label(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-icon-test-") as root_text:
            missing = Path(root_text) / "missing.svg"
            with self.assertRaisesRegex(
                RuntimeError,
                "required icon asset is missing: logical.svg",
            ):
                _PLATFORM_EXPORT.PlatformExporter._required(
                    missing,
                    "logical.svg",
                )


if __name__ == "__main__":
    unittest.main()

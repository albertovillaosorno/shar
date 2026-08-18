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
#   - Canonical build-state filesystem-boundary regression tests.
# - Must-Not:
#   - Invoke Cargo, Unreal, package managers, or network services.
# - Allows:
#   - Temporary repository roots and mocked canonical build steps.
# - Split-When:
#   - One build-state producer gains an independent storage policy.
# - Merge-When:
#   - Canonical build-state ownership moves into one shared adapter.
# - Summary:
#   - Build-state path containment tests.
# - Description:
#   - Proves canonical evidence writers reject linked repository cache roots.
# - Usage:
#   - Run with Python's standard-library unittest runner.
# - Defaults:
#   - Explicit testing output overrides remain outside this policy surface.
#

"""Tests for repository-owned build evidence paths."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import sys
import tempfile
from types import ModuleType
import unittest
from unittest import mock

_ROOT = Path(__file__).resolve().parents[2]


def _load(name: str, relative: str) -> ModuleType:
    path = _ROOT / relative
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {relative}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


_ARCH = _load(
    "shar_build_arch_boundary_test",
    "tools/build/adapter-inbound/arch.py",
)
_CHECK = _load(
    "shar_build_check_boundary_test",
    "tools/build/adapter-inbound/check.py",
)
_DEPENDENCIES = _load(
    "shar_build_dependencies_boundary_test",
    "tools/build/adapter-inbound/dependencies.py",
)


class CanonicalBuildStateBoundaryTests(unittest.TestCase):
    """Keep canonical build evidence below real repository cache roots."""

    @staticmethod
    def _linked_cache_patch(cache_root: Path) -> mock._patch[object]:
        original = Path.is_symlink

        def report_cache_as_link(path: Path) -> bool:
            return path == cache_root or original(path)

        return mock.patch.object(Path, "is_symlink", report_cache_as_link)

    def test_arch_rejects_linked_canonical_cache_root(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-arch-state-") as raw:
            root = Path(raw)
            cache_root = root / ".cache"
            cache_root.mkdir()
            output = root / _ARCH._DATA_PATH
            with (
                mock.patch.object(_ARCH, "_root", return_value=root),
                self._linked_cache_patch(cache_root),
                mock.patch.object(
                    sys,
                    "argv",
                    ["arch.py", "--select", "linux-x64"],
                ),
                self.assertRaisesRegex(
                    SystemExit,
                    "repository cache root must be a real directory",
                ),
            ):
                _ARCH.main()
            self.assertFalse(output.exists())

    def test_check_rejects_linked_canonical_cache_root(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-check-state-") as raw:
            root = Path(raw)
            cache_root = root / ".cache"
            cache_root.mkdir()
            output = root / _CHECK._DATA_PATH
            output.parent.mkdir(parents=True)
            output.write_text("sentinel\n", encoding="utf-8")
            with (
                mock.patch.object(_CHECK, "_root", return_value=root),
                mock.patch.object(_CHECK, "_run", return_value={}),
                self._linked_cache_patch(cache_root),
                mock.patch.object(sys, "argv", ["check.py"]),
            ):
                self.assertEqual(_CHECK.main(), 1)
            self.assertEqual(output.read_text(encoding="utf-8"), "sentinel\n")

    def test_dependencies_reject_linked_canonical_cache_root(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-dependency-state-",
        ) as raw:
            root = Path(raw)
            cache_root = root / ".cache"
            cache_root.mkdir()
            output = root / _DEPENDENCIES._DATA_PATH
            output.parent.mkdir(parents=True)
            output.write_text("sentinel\n", encoding="utf-8")
            with (
                mock.patch.object(_DEPENDENCIES, "_root", return_value=root),
                mock.patch.object(_DEPENDENCIES, "_run", return_value={}),
                self._linked_cache_patch(cache_root),
                mock.patch.object(sys, "argv", ["dependencies.py"]),
            ):
                self.assertEqual(_DEPENDENCIES.main(), 1)
            self.assertEqual(output.read_text(encoding="utf-8"), "sentinel\n")

    def test_explicit_output_override_ignores_canonical_cache_identity(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-override-state-") as raw:
            root = Path(raw)
            cache_root = root / ".cache"
            cache_root.mkdir()
            override = root / "testing/output.json"
            with self._linked_cache_patch(cache_root):
                _ARCH._validate_canonical_output_root(root, override)
                _CHECK._validate_canonical_output_root(root, override)
                self.assertFalse(
                    _DEPENDENCIES._validate_canonical_output_root(
                        root,
                        override,
                    )
                )


if __name__ == "__main__":
    unittest.main()

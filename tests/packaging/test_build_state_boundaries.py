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


class AtomicBuildEvidenceWriteTests(unittest.TestCase):
    """Never truncate a pre-existing atomic staging identity."""

    @staticmethod
    def _collision(output: Path, pid: int) -> Path:
        output.parent.mkdir(parents=True, exist_ok=True)
        collision = output.with_name(f".{output.name}.{pid}.tmp")
        collision.write_text("sentinel\n", encoding="utf-8")
        return collision

    def test_arch_write_preserves_preexisting_staging_file(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-arch-atomic-") as raw:
            output = Path(raw) / "arch.json"
            collision = self._collision(output, 4242)
            with (
                mock.patch.object(_ARCH.os, "getpid", return_value=4242),
                self.assertRaises(FileExistsError),
            ):
                _ARCH._write_selection(
                    output,
                    [_ARCH._TARGETS_BY_ID["linux-x64"]],
                )
            self.assertEqual(
                collision.read_text(encoding="utf-8"),
                "sentinel\n",
            )
            self.assertFalse(output.exists())

    def test_check_write_preserves_preexisting_staging_file(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-check-atomic-") as raw:
            output = Path(raw) / "check.json"
            collision = self._collision(output, 4242)
            with (
                mock.patch.object(_CHECK.os, "getpid", return_value=4242),
                self.assertRaises(FileExistsError),
            ):
                _CHECK._write_json(output, {})
            self.assertEqual(
                collision.read_text(encoding="utf-8"),
                "sentinel\n",
            )
            self.assertFalse(output.exists())

    def test_dependency_write_preserves_preexisting_staging_file(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-dependency-atomic-",
        ) as raw:
            output = Path(raw) / "dependencies.json"
            collision = self._collision(output, 4242)
            with (
                mock.patch.object(
                    _DEPENDENCIES.os,
                    "getpid",
                    return_value=4242,
                ),
                self.assertRaises(FileExistsError),
            ):
                _DEPENDENCIES._write_json(output, {})
            self.assertEqual(
                collision.read_text(encoding="utf-8"),
                "sentinel\n",
            )
            self.assertFalse(output.exists())


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

    def test_arch_rejects_linked_canonical_evidence_file(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-arch-file-") as raw:
            root = Path(raw)
            output = root / _ARCH._DATA_PATH
            output.parent.mkdir(parents=True)
            output.write_text("sentinel\n", encoding="utf-8")
            original = Path.is_symlink

            def report_output_as_link(path: Path) -> bool:
                return path == output or original(path)

            with (
                mock.patch.object(Path, "is_symlink", report_output_as_link),
                self.assertRaisesRegex(
                    SystemExit,
                    "architecture evidence must be a real file",
                ),
            ):
                _ARCH._validate_canonical_output_root(root, output)

    def test_check_rejects_linked_canonical_evidence_file(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-check-file-") as raw:
            root = Path(raw)
            output = root / _CHECK._DATA_PATH
            output.parent.mkdir(parents=True)
            output.write_text("sentinel\n", encoding="utf-8")
            original = Path.is_symlink

            def report_output_as_link(path: Path) -> bool:
                return path == output or original(path)

            with (
                mock.patch.object(Path, "is_symlink", report_output_as_link),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "preflight evidence must be a real file",
                ),
            ):
                _CHECK._validate_canonical_output_root(root, output)

    @unittest.skipIf(sys.platform == "win32", "symlink setup is Unix-focused")
    def test_check_rejects_linked_dependency_evidence_input(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-dependency-input-link-"
        ) as raw:
            root = Path(raw)
            external = root / "external-dependencies.json"
            external.write_text(
                '{"schema":"shar.build.dependencies.v1"}\n',
                encoding="utf-8",
            )
            dependency = root / _CHECK._DEPENDENCIES_PATH
            dependency.parent.mkdir(parents=True)
            dependency.symlink_to(external)

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "dependency evidence must be a real file",
            ):
                _CHECK._dependency_evidence(root)

    @unittest.skipIf(sys.platform == "win32", "symlink setup is Unix-focused")
    def test_check_rejects_linked_dependency_evidence_parent(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-dependency-parent-link-"
        ) as raw:
            root = Path(raw)
            external = root / "external-data"
            external.mkdir()
            (external / "dependencies.json").write_text(
                '{"schema":"shar.build.dependencies.v1"}\n',
                encoding="utf-8",
            )
            build = root / ".cache/build"
            build.mkdir(parents=True)
            (build / "data").symlink_to(external, target_is_directory=True)

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "build data root must be a real directory",
            ):
                _CHECK._dependency_evidence(root)

    def test_check_rejects_junction_dependency_evidence_input(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-dependency-input-junction-"
        ) as raw:
            root = Path(raw)
            dependency = root / _CHECK._DEPENDENCIES_PATH
            dependency.parent.mkdir(parents=True)
            dependency.write_text(
                '{"schema":"shar.build.dependencies.v1"}\n',
                encoding="utf-8",
            )
            with (
                mock.patch.object(
                    _CHECK.os.path,
                    "isjunction",
                    side_effect=lambda path: Path(path) == dependency,
                ),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "dependency evidence must be a real file",
                ),
            ):
                _CHECK._dependency_evidence(root)

    def test_check_rejects_junction_dependency_evidence_parent(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-dependency-parent-junction-"
        ) as raw:
            root = Path(raw)
            dependency = root / _CHECK._DEPENDENCIES_PATH
            dependency.parent.mkdir(parents=True)
            dependency.write_text(
                '{"schema":"shar.build.dependencies.v1"}\n',
                encoding="utf-8",
            )
            parent = dependency.parent
            with (
                mock.patch.object(
                    _CHECK.os.path,
                    "isjunction",
                    side_effect=lambda path: Path(path) == parent,
                ),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "build data root must be a real directory",
                ),
            ):
                _CHECK._dependency_evidence(root)

    def test_dependencies_reject_linked_canonical_evidence_file(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-dependency-file-") as raw:
            root = Path(raw)
            output = root / _DEPENDENCIES._DATA_PATH
            output.parent.mkdir(parents=True)
            output.write_text("sentinel\n", encoding="utf-8")
            original = Path.is_symlink

            def report_output_as_link(path: Path) -> bool:
                return path == output or original(path)

            with (
                mock.patch.object(Path, "is_symlink", report_output_as_link),
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "dependency evidence must be a real file",
                ),
            ):
                _DEPENDENCIES._validate_canonical_output_root(root, output)

    def test_arch_and_check_output_overrides_ignore_canonical_cache_identity(
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

    def test_dependencies_override_still_rejects_linked_build_cache(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-dependency-override-state-",
        ) as raw:
            root = Path(raw)
            cache_root = root / ".cache"
            cache_root.mkdir()
            with (
                self._linked_cache_patch(cache_root),
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "repository cache root must be a real directory",
                ),
            ):
                _DEPENDENCIES._validate_dependency_storage_roots(root)

    def test_dependencies_reject_linked_dependency_root(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-dependency-root-state-",
        ) as raw:
            root = Path(raw)
            dependency_root = root / ".dependencies"
            dependency_root.mkdir()
            original = Path.is_symlink

            def report_dependency_as_link(path: Path) -> bool:
                return path == dependency_root or original(path)

            with (
                mock.patch.object(_DEPENDENCIES, "_root", return_value=root),
                mock.patch.object(_DEPENDENCIES, "_run", return_value={}),
                mock.patch.object(
                    Path,
                    "is_symlink",
                    report_dependency_as_link,
                ),
                mock.patch.object(
                    sys,
                    "argv",
                    ["dependencies.py", "--output", "testing/output.json"],
                ),
            ):
                self.assertEqual(_DEPENDENCIES.main(), 1)
            self.assertFalse((root / "testing/output.json").exists())


if __name__ == "__main__":
    unittest.main()

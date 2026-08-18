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
#   - Canonical build-runner project-state migration tests.
# - Must-Not:
#   - Invoke Unreal or mutate the repository project.
# - Allows:
#   - Temporary project roots, directory links, and injected failures.
# - Split-When:
#   - Build-runner tests gain an independent platform integration lifecycle.
# - Merge-When:
#   - Project-state migration moves to a dedicated adapter.
# - Summary:
#   - Build runner project-state migration tests.
# - Description:
#   - Proves generated Unreal project roots live physically below build cache.
# - Usage:
#   - Run directly with Python's standard-library unittest runner.
# - Defaults:
#   - Tests use isolated temporary directories only.
#

"""Tests for canonical build-runner project-state migration."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest import mock

_RUN_PATH = (
    Path(__file__).resolve().parents[2]
    / "tools"
    / "build"
    / "adapter-inbound"
    / "run.py"
)
_SPEC = importlib.util.spec_from_file_location("shar_build_run", _RUN_PATH)
if _SPEC is None or _SPEC.loader is None:
    raise RuntimeError("cannot load build runner for tests")
_RUN = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(_RUN)


class ProjectStateMigrationTests(unittest.TestCase):
    """Exercise build-state adoption without running Unreal."""

    def _fixture(self) -> tuple[tempfile.TemporaryDirectory[str], Path, Path]:
        temporary = tempfile.TemporaryDirectory(prefix="shar-build-state-")
        root = Path(temporary.name)
        project_dir = root / "project"
        project_dir.mkdir()
        project = project_dir / "shar.uproject"
        project.write_text("{}\n", encoding="utf-8")
        return temporary, root, project

    def _unlink_project_state(self, project: Path) -> None:
        for name in _RUN._PROJECT_STATE_NAMES:
            path = project.parent / name
            if _RUN._is_directory_link(path):
                _RUN._remove_directory_link(path)

    def test_adopts_all_legacy_project_state_roots(self) -> None:
        temporary, root, project = self._fixture()
        try:
            for name in _RUN._PROJECT_STATE_NAMES:
                source = project.parent / name
                source.mkdir()
                (source / "sentinel.txt").write_text(name, encoding="utf-8")

            state_root = _RUN._prepare_project_state(root, project)

            self.assertEqual(state_root, root / _RUN._PROJECT_STATE_ROOT)
            for name in _RUN._PROJECT_STATE_NAMES:
                link = project.parent / name
                canonical = state_root / name
                self.assertTrue(_RUN._is_directory_link(link))
                self.assertEqual(link.resolve(), canonical.resolve())
                self.assertEqual(
                    (canonical / "sentinel.txt").read_text(encoding="utf-8"),
                    name,
                )
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()

    def test_reuses_existing_canonical_links(self) -> None:
        temporary, root, project = self._fixture()
        try:
            first = _RUN._prepare_project_state(root, project)
            second = _RUN._prepare_project_state(root, project)
            self.assertEqual(first, second)
            for name in _RUN._PROJECT_STATE_NAMES:
                self.assertTrue(_RUN._is_directory_link(project.parent / name))
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()

    def test_rejects_legacy_and_canonical_state_conflict(self) -> None:
        temporary, root, project = self._fixture()
        try:
            legacy = project.parent / "Saved"
            legacy.mkdir()
            (legacy / "legacy.txt").write_text("legacy", encoding="utf-8")
            canonical = root / _RUN._PROJECT_STATE_ROOT / "Saved"
            canonical.mkdir(parents=True)
            (canonical / "canonical.txt").write_text(
                "canonical",
                encoding="utf-8",
            )

            with self.assertRaisesRegex(_RUN.RunFailure, "both exist"):
                _RUN._prepare_project_state(root, project)

            self.assertTrue((legacy / "legacy.txt").is_file())
            self.assertTrue((canonical / "canonical.txt").is_file())
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()

    def test_rolls_back_when_a_later_link_creation_fails(self) -> None:
        temporary, root, project = self._fixture()
        try:
            for name in _RUN._PROJECT_STATE_NAMES:
                source = project.parent / name
                source.mkdir()
                (source / "sentinel.txt").write_text(name, encoding="utf-8")
            original = _RUN._create_directory_link

            def fail_on_intermediate(link: Path, target: Path) -> None:
                if link.name == "Intermediate":
                    raise _RUN.RunFailure("injected link failure")
                original(link, target)

            with (
                mock.patch.object(
                    _RUN,
                    "_create_directory_link",
                    side_effect=fail_on_intermediate,
                ),
                self.assertRaisesRegex(_RUN.RunFailure, "injected"),
            ):
                _RUN._prepare_project_state(root, project)

            for name in _RUN._PROJECT_STATE_NAMES:
                source = project.parent / name
                self.assertTrue(source.is_dir())
                self.assertFalse(_RUN._is_directory_link(source))
                self.assertEqual(
                    (source / "sentinel.txt").read_text(encoding="utf-8"),
                    name,
                )
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()


class PublicationArtifactTests(unittest.TestCase):
    """Keep runtime publication separate from cached build diagnostics."""

    def test_windows_publication_caches_manifests_and_pdbs(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-publication-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            binary = candidate / "shar/Binaries/Win64/shar.exe"
            symbols = candidate / "shar/Binaries/Win64/shar.pdb"
            binary.parent.mkdir(parents=True)
            binary.write_bytes(b"runtime")
            symbols.write_bytes(b"symbols")
            (candidate / "Manifest_UFSFiles_Win64.txt").write_text(
                "manifest\n",
                encoding="utf-8",
            )
            stale = work / "symbols/stale.pdb"
            stale.parent.mkdir(parents=True)
            stale.write_bytes(b"stale")

            target = _RUN._TARGETS_BY_ID["windows-x64"]
            _RUN._cache_nonruntime_artifacts(candidate, work, target)

            self.assertEqual(binary.read_bytes(), b"runtime")
            self.assertFalse(symbols.exists())
            self.assertFalse(
                (candidate / "Manifest_UFSFiles_Win64.txt").exists()
            )
            self.assertEqual(
                (work / "symbols/shar/Binaries/Win64/shar.pdb").read_bytes(),
                b"symbols",
            )
            self.assertEqual(
                (
                    work / "publication-metadata/Manifest_UFSFiles_Win64.txt"
                ).read_text(encoding="utf-8"),
                "manifest\n",
            )
            self.assertFalse(stale.exists())


if __name__ == "__main__":
    unittest.main()

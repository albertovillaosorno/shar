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
#   - Build validator source-evidence regression tests.
# - Must-Not:
#   - Invoke Cargo, mutate canonical build evidence, or contact the network.
# - Allows:
#   - Temporary source closures and validator byte fixtures.
# - Split-When:
#   - Bootstrap evidence gains another independently versioned build artifact.
# - Merge-When:
#   - Build bootstrap and preflight no longer share validator evidence.
# - Summary:
#   - Build validator source-evidence tests.
# - Description:
#   - Proves a byte-intact validator is rejected after source input drift.
# - Usage:
#   - Run with Python's standard-library unittest runner.
# - Defaults:
#   - Every fixture is isolated below a temporary directory.
#

"""Tests for source-bound build validator evidence."""

from __future__ import annotations

import hashlib
import importlib.util
from pathlib import Path
import tempfile
from types import ModuleType
import unittest

_ROOT = Path(__file__).resolve().parents[2]


def _load(name: str, relative: str) -> ModuleType:
    path = _ROOT / relative
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {relative}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


_DEPENDENCIES = _load(
    "shar_build_dependencies_test",
    "tools/build/dependencies.py",
)
_CHECK = _load("shar_build_check_test", "tools/build/check.py")


class ValidatorSourceEvidenceTests(unittest.TestCase):
    """Exercise validator source fingerprints with no toolchain execution."""

    def _fixture(self) -> tuple[tempfile.TemporaryDirectory[str], Path]:
        temporary = tempfile.TemporaryDirectory(
            prefix="shar-validator-evidence-"
        )
        root = Path(temporary.name)
        for relative in _DEPENDENCIES._VALIDATOR_SOURCE_INPUTS:
            path = root / relative
            if path.suffix:
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(
                    f"fixture:{relative.as_posix()}\n",
                    encoding="utf-8",
                )
            else:
                path.mkdir(parents=True, exist_ok=True)
                (path / "fixture.rs").write_text(
                    f"// {relative.as_posix()}\n",
                    encoding="utf-8",
                )
        return temporary, root

    def test_bootstrap_and_preflight_compute_identical_source_digest(
        self,
    ) -> None:
        temporary, root = self._fixture()
        try:
            self.assertEqual(
                _DEPENDENCIES._validator_source_sha256(root),
                _CHECK._validator_source_sha256(root),
            )
        finally:
            temporary.cleanup()

    def test_source_drift_rejects_byte_intact_validator(self) -> None:
        temporary, root = self._fixture()
        try:
            validator = root / ".dependencies/build/bin/validate-game.exe"
            validator.parent.mkdir(parents=True)
            validator.write_bytes(b"validator")
            binary_hash = hashlib.sha256(validator.read_bytes()).hexdigest()
            evidence = {
                "validator": {
                    "path": str(validator),
                    "sha256": binary_hash,
                    "source_sha256": _CHECK._validator_source_sha256(root),
                }
            }
            self.assertEqual(
                _CHECK._dependency_validator(root, evidence),
                validator.resolve(),
            )
            source = root / "src/migration/manifest/fixture.rs"
            source.write_text("// changed\n", encoding="utf-8")

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "source inputs no longer match",
            ):
                _CHECK._dependency_validator(root, evidence)
        finally:
            temporary.cleanup()

    def test_legacy_validator_evidence_fails_closed(self) -> None:
        temporary, root = self._fixture()
        try:
            validator = root / ".dependencies/build/bin/validate-game.exe"
            validator.parent.mkdir(parents=True)
            validator.write_bytes(b"validator")
            evidence = {
                "validator": {
                    "path": str(validator),
                    "sha256": hashlib.sha256(b"validator").hexdigest(),
                }
            }
            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "evidence is incomplete",
            ):
                _CHECK._dependency_validator(root, evidence)
        finally:
            temporary.cleanup()


if __name__ == "__main__":
    unittest.main()


class SourceSelectionTests(unittest.TestCase):
    """Exercise read-only source-root selection without build toolchains."""

    def test_directory_and_simpsons_exe_resolve_to_same_source_root(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-source-select-") as value:
            root = Path(value)
            repository = root / "repository"
            source = root / "installed-game"
            repository.mkdir()
            source.mkdir()
            executable = source / "Simpsons.exe"
            executable.write_bytes(b"fixture")

            self.assertEqual(
                _CHECK._check_game(repository, source),
                source.resolve(),
            )
            self.assertEqual(
                _CHECK._check_game(repository, executable),
                source.resolve(),
            )

    def test_non_game_file_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-source-file-") as value:
            root = Path(value)
            repository = root / "repository"
            repository.mkdir()
            other = root / "README.rtf"
            other.write_bytes(b"fixture")

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "selected source file must be Simpsons.exe",
            ):
                _CHECK._check_game(repository, other)

    def test_missing_source_diagnostic_does_not_echo_private_path(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-source-missing-"
        ) as value:
            root = Path(value)
            repository = root / "repository"
            repository.mkdir()
            missing = root / "private-user-installation"

            with self.assertRaises(_CHECK.CheckFailure) as raised:
                _CHECK._check_game(repository, missing)
            message = str(raised.exception)
            self.assertNotIn(str(missing), message)
            self.assertIn("selected source path does not exist", message)

    def test_validator_command_keeps_manifest_separate_from_source(
        self,
    ) -> None:
        validator = Path("validate-game.exe")
        source = Path("external-source")
        manifest = Path("repository/game/manifest/game.jsonl")

        command = _CHECK._validator_command(validator, source, manifest)

        self.assertEqual(
            command,
            [str(validator), str(source), str(manifest)],
        )

    def test_saved_source_root_is_required_for_revalidation(self) -> None:
        expected = Path("C:/lawful/source")
        self.assertEqual(
            _CHECK._saved_game_root({"game": {"path": str(expected)}}),
            expected,
        )
        with self.assertRaisesRegex(
            _CHECK.CheckFailure,
            "no source game root",
        ):
            _CHECK._saved_game_root({"game": {}})

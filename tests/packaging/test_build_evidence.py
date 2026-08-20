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
import os
from pathlib import Path
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


_DEPENDENCIES = _load(
    "shar_build_dependencies_test",
    "tools/build/adapter-inbound/dependencies.py",
)
_CHECK = _load("shar_build_check_test", "tools/build/adapter-inbound/check.py")


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

    def _deep_fixture(self) -> tuple[tempfile.TemporaryDirectory[str], Path]:
        temporary = tempfile.TemporaryDirectory(
            prefix="shar-deep-validator-evidence-"
        )
        root = Path(temporary.name)
        for relative in _DEPENDENCIES._DEEP_VALIDATOR_SOURCE_INPUTS:
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

    def test_deep_bootstrap_and_preflight_compute_identical_source_digest(
        self,
    ) -> None:
        temporary, root = self._deep_fixture()
        try:
            self.assertEqual(
                _DEPENDENCIES._deep_validator_source_sha256(root),
                _CHECK._deep_validator_source_sha256(root),
            )
        finally:
            temporary.cleanup()

    def test_deep_source_drift_rejects_byte_intact_validator(self) -> None:
        temporary, root = self._deep_fixture()
        try:
            validator = (
                root / ".dependencies/build/bin/validate-source-deep"
            )
            validator.parent.mkdir(parents=True)
            validator.write_bytes(b"deep-validator")
            evidence = {
                "deep_source_validator": {
                    "path": str(validator),
                    "sha256": hashlib.sha256(b"deep-validator").hexdigest(),
                    "source_sha256": (
                        _CHECK._deep_validator_source_sha256(root)
                    ),
                }
            }
            self.assertEqual(
                _CHECK._dependency_deep_source_validator(root, evidence),
                validator.resolve(),
            )
            source = root / "src/migration/source-audit/fixture.rs"
            source.write_text("// changed\n", encoding="utf-8")

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "deep source validator source inputs no longer match",
            ):
                _CHECK._dependency_deep_source_validator(root, evidence)
        finally:
            temporary.cleanup()

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


class EngineSelectionTests(unittest.TestCase):
    """Exercise portable default Unreal Engine candidate selection."""

    def test_engine_rejects_directory_at_editor_executable_path(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-engine-editor-") as raw:
            engine = Path(raw) / "UE_5.8"
            version = engine / "Engine/Build/Build.version"
            version.parent.mkdir(parents=True)
            version.write_text(
                '{"MajorVersion":5,"MinorVersion":8,"PatchVersion":1}\n',
                encoding="utf-8",
            )
            editor = engine / "fake-editor"
            editor.mkdir()

            with (
                mock.patch.object(_CHECK, "_editor_path", return_value=editor),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "Unreal editor executable is missing",
                ),
            ):
                _CHECK._check_engine(engine)

    @unittest.skipIf(os.name == "nt", "POSIX execute bits are not portable")
    def test_engine_rejects_non_executable_editor_file(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-engine-editor-") as raw:
            engine = Path(raw) / "UE_5.8"
            version = engine / "Engine/Build/Build.version"
            version.parent.mkdir(parents=True)
            version.write_text(
                '{"MajorVersion":5,"MinorVersion":8,"PatchVersion":1}\n',
                encoding="utf-8",
            )
            editor = engine / "fake-editor"
            editor.write_bytes(b"editor")
            editor.chmod(0o644)

            with (
                mock.patch.object(_CHECK, "_editor_path", return_value=editor),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "Unreal editor is not executable",
                ),
            ):
                _CHECK._check_engine(engine)

    def test_macos_default_engine_path_preserves_launcher_location(
        self,
    ) -> None:
        with (
            mock.patch.object(_CHECK.os, "name", "posix"),
            mock.patch.object(_CHECK.sys, "platform", "darwin"),
            mock.patch.dict(_CHECK.os.environ, {}, clear=True),
        ):
            candidates = _CHECK._engine_candidates(None)

        expected = (
            Path("/") / "Users" / "Shared" / "Epic Games" / "UE_5.8"
        )
        self.assertEqual(candidates, [expected])


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

            with mock.patch.object(
                _CHECK.os.path,
                "isjunction",
                side_effect=lambda path: Path(path) == source,
            ):
                for selection in (source, executable):
                    with (
                        self.subTest(selection=selection),
                        self.assertRaisesRegex(
                            _CHECK.CheckFailure,
                            "real source directory",
                        ),
                    ):
                        _CHECK._check_game(repository, selection)

    def test_non_game_file_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-source-file-") as value:
            root = Path(value)
            repository = root / "repository"
            repository.mkdir()
            other = root / "README.rtf"
            other.write_bytes(b"fixture")
            wrong_case = root / "simpsons.exe"
            wrong_case.write_bytes(b"fixture")

            for selected in (other, wrong_case):
                with self.subTest(selected=selected), self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "selected source file must be Simpsons.exe",
                ):
                    _CHECK._check_game(repository, selected)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_redirected_wrong_source_file_name_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-source-redirect-"
        ) as value:
            root = Path(value)
            repository = root / "repository"
            source = root / "installed-game"
            repository.mkdir()
            source.mkdir()
            executable = source / "Simpsons.exe"
            executable.write_bytes(b"fixture")
            redirect = source / "README.rtf"
            redirect.symlink_to(executable.name)

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "selected source file must be Simpsons.exe",
            ):
                _CHECK._check_game(repository, redirect)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_nested_executable_redirect_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-source-nested-link-"
        ) as value:
            root = Path(value)
            repository = root / "repository"
            source = root / "installed-game"
            repository.mkdir()
            source.mkdir()
            executable = source / "Simpsons.exe"
            executable.write_bytes(b"fixture")
            nested = source / "copy"
            nested.mkdir()
            (nested / "Simpsons.exe").symlink_to(executable)

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "selected source contains another nested Simpsons.exe",
            ):
                _CHECK._check_game(repository, source)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_direct_source_redirects_are_rejected(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-source-direct-link-"
        ) as value:
            root = Path(value)
            repository = root / "repository"
            source = root / "installed-game"
            outside = root / "outside"
            repository.mkdir()
            source.mkdir()
            outside.mkdir()
            target = outside / "Simpsons.exe"
            target.write_bytes(b"fixture")
            redirect = source / "Simpsons.exe"
            redirect.symlink_to(target)

            for selection in (source, redirect):
                with self.subTest(selection=selection), self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "real Simpsons.exe",
                ):
                    _CHECK._check_game(repository, selection)

            real_source = root / "real-source"
            real_source.mkdir()
            (real_source / "Simpsons.exe").write_bytes(b"fixture")
            selected = root / "selected-game"
            selected.symlink_to(real_source, target_is_directory=True)
            default = repository / "game"
            default.symlink_to(real_source, target_is_directory=True)
            for selection in (selected, None):
                with self.subTest(selection=selection), self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "real source directory",
                ):
                    _CHECK._check_game(repository, selection)

            parent_target = root / "parent-target"
            nested_source = parent_target / "installed-game"
            nested_source.mkdir(parents=True)
            nested_executable = nested_source / "Simpsons.exe"
            nested_executable.write_bytes(b"fixture")
            parent_redirect = root / "parent-redirect"
            parent_redirect.symlink_to(parent_target, target_is_directory=True)
            redirected_source = parent_redirect / nested_source.name
            redirected_executable = redirected_source / nested_executable.name
            for selection in (redirected_source, redirected_executable):
                with self.subTest(selection=selection), self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "real source directory",
                ):
                    _CHECK._check_game(repository, selection)

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

    def test_redaction_covers_escaped_windows_source_path(self) -> None:
        source = Path(r"C:\private\user\installed-game")
        diagnostic = (
            r"scan C:\private\user\installed-game: access denied"
        )
        escaped = diagnostic.replace("\\", "\\\\")

        redacted = _CHECK._redact_selected_source(escaped, source)

        self.assertNotIn("private", redacted)
        self.assertIn("<selected-source>", redacted)

    def test_manifest_failure_redacts_selected_source_path(self) -> None:
        source = Path("/private/user/installed-game")
        validator = Path("validate-game")
        manifest = Path("repository/game/manifest/game.jsonl")
        result = _CHECK.subprocess.CompletedProcess(
            args=[],
            returncode=1,
            stdout="",
            stderr=(
                "game manifest FAILED: 1 of 2 records below minimum in "
                f"{source}\n  <root> .rcf: have 0, need at least 9\n"
            ),
        )

        with (
            mock.patch.object(_CHECK.subprocess, "run", return_value=result),
            self.assertRaises(_CHECK.CheckFailure) as raised,
        ):
            _CHECK._check_manifest(validator, source, manifest)

        message = str(raised.exception)
        self.assertNotIn(str(source), message)
        self.assertIn("<selected-source>", message)
        self.assertIn("have 0, need at least 9", message)

    def test_manifest_success_redacts_selected_source_path(self) -> None:
        source = Path("/private/user/installed-game")
        result = _CHECK.subprocess.CompletedProcess(
            args=[],
            returncode=0,
            stdout=f"game manifest ok: all 2 minimums met in {source}\n",
            stderr="",
        )
        with mock.patch.object(
            _CHECK.subprocess,
            "run",
            return_value=result,
        ):
            message = _CHECK._check_manifest(
                Path("validate-game"),
                source,
                Path("game.jsonl"),
            )

        self.assertNotIn(str(source), message)
        self.assertIn("<selected-source>", message)

    def test_deep_failure_redacts_selected_source_path(self) -> None:
        source = Path("/private/user/installed-game")
        result = _CHECK.subprocess.CompletedProcess(
            args=[],
            returncode=1,
            stdout="",
            stderr=f"deep audit failed while reading {source}\n",
        )

        with (
            mock.patch.object(_CHECK.subprocess, "run", return_value=result),
            self.assertRaises(_CHECK.CheckFailure) as raised,
        ):
            _CHECK._check_deep_source(Path("validate-source-deep"), source)

        message = str(raised.exception)
        self.assertNotIn(str(source), message)
        self.assertIn("<selected-source>", message)

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

    def test_deep_validator_command_uses_only_source_root(self) -> None:
        validator = Path("validate-source-deep")
        source = Path("external-source")

        command = _CHECK._deep_validator_command(validator, source)

        self.assertEqual(command, [str(validator), str(source)])

    def test_preflight_records_manifest_then_deep_validation(self) -> None:
        root = Path("/synthetic/repository")
        game = Path("/synthetic/game")
        project = root / "src/unreal/project/shar.uproject"
        dependency_path = root / ".cache/build/data/dependencies.json"
        manifest_validator = root / ".dependencies/build/bin/validate-game"
        deep_validator = (
            root / ".dependencies/build/bin/validate-source-deep"
        )
        engine = _CHECK.EngineEvidence(Path("/synthetic/engine"), "5.8.1")
        args = _CHECK.argparse.Namespace(
            engine_root=None,
            game=game,
            manifest_validator=None,
            deep_source_validator=None,
        )
        calls: list[str] = []

        def manifest_gate(*_args: object) -> str:
            calls.append("manifest")
            return "manifest-ok"

        def deep_gate(*_args: object) -> str:
            calls.append("deep")
            return "deep-source\tfiles=0\tp3d=0\trcf=0\trsd=0\trmv=0"

        with (
            mock.patch.object(_CHECK, "_root", return_value=root),
            mock.patch.object(_CHECK, "_check_python", return_value={}),
            mock.patch.object(_CHECK, "_check_game", return_value=game),
            mock.patch.object(Path, "is_file", return_value=True),
            mock.patch.object(_CHECK, "_check_project", return_value=project),
            mock.patch.object(
                _CHECK,
                "_dependency_evidence",
                return_value=(dependency_path, {}),
            ),
            mock.patch.object(
                _CHECK,
                "_resolve_validator",
                return_value=manifest_validator,
            ),
            mock.patch.object(
                _CHECK,
                "_resolve_deep_source_validator",
                return_value=deep_validator,
            ),
            mock.patch.object(
                _CHECK, "_check_manifest", side_effect=manifest_gate
            ),
            mock.patch.object(
                _CHECK,
                "_check_deep_source",
                side_effect=deep_gate,
            ),
            mock.patch.object(_CHECK, "_check_engine", return_value=engine),
            mock.patch.object(_CHECK, "_host_evidence", return_value={}),
            mock.patch.object(_CHECK, "_sha256", return_value="a" * 64),
        ):
            evidence = _CHECK._run(args)

        self.assertEqual(calls, ["manifest", "deep"])
        self.assertEqual(evidence["game"]["validation"], "manifest-ok")
        self.assertEqual(
            evidence["game"]["deep_validation"],
            "deep-source\tfiles=0\tp3d=0\trcf=0\trsd=0\trmv=0",
        )
        self.assertEqual(
            evidence["deep_source_validator"],
            str(deep_validator.resolve()),
        )

    def test_revalidation_rejects_deep_validator_override(self) -> None:
        args = _CHECK.argparse.Namespace(
            engine_root=None,
            game=None,
            manifest_validator=None,
            deep_source_validator=Path("validate-source-deep"),
        )

        with self.assertRaisesRegex(
            _CHECK.CheckFailure,
            "cannot be combined with preflight overrides",
        ):
            _CHECK._reject_revalidate_overrides(args)

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

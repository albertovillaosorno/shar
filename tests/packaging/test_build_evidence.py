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

from collections.abc import Callable
from collections.abc import Iterator
from contextlib import AbstractContextManager
import hashlib
import importlib.util
import io
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

    def test_source_closure_scan_failures_are_not_suppressed(self) -> None:
        temporary, root = self._fixture()
        try:
            blocked = root / "src/migration/manifest"

            def assert_scan_failure(module: ModuleType) -> None:
                real_scan = module._scan_source_directory

                def strict_scan(
                    path: Path,
                ) -> AbstractContextManager[Iterator[os.DirEntry[str]]]:
                    if path == blocked:
                        raise PermissionError("source closure scan blocked")
                    return real_scan(path)

                with (
                    mock.patch.object(
                        module,
                        "_scan_source_directory",
                        strict_scan,
                    ),
                    self.assertRaises(PermissionError),
                ):
                    module._validator_source_sha256(root)

            for module in (_DEPENDENCIES, _CHECK):
                with self.subTest(module=module.__name__):
                    assert_scan_failure(module)
        finally:
            temporary.cleanup()

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_source_closure_rejects_redirected_entries(self) -> None:
        temporary, root = self._fixture()
        try:
            manifest_root = root / "src/migration/manifest"
            redirected = manifest_root / "redirected.rs"
            target = root / "redirect-target.rs"
            target.write_text("// redirected\n", encoding="utf-8")
            redirected.symlink_to(target)

            for module in (_DEPENDENCIES, _CHECK):
                with (
                    self.subTest(module=module.__name__),
                    self.assertRaisesRegex(
                        OSError,
                        "source closure contains a redirected entry",
                    ),
                ):
                    module._validator_source_sha256(root)
        finally:
            temporary.cleanup()

    def test_source_closure_rejects_junction_entries(self) -> None:
        temporary, root = self._fixture()
        try:
            redirected = root / "src/migration/manifest/redirected"
            redirected.mkdir()

            for module in (_DEPENDENCIES, _CHECK):
                with (
                    self.subTest(module=module.__name__),
                    mock.patch.object(
                        module.os.path,
                        "isjunction",
                        side_effect=lambda path: Path(path) == redirected,
                    ),
                    self.assertRaisesRegex(
                        OSError,
                        "source closure contains a redirected entry",
                    ),
                ):
                    module._validator_source_sha256(root)
        finally:
            temporary.cleanup()

    def _combined_fixture(
        self,
    ) -> tuple[tempfile.TemporaryDirectory[str], Path]:
        temporary = tempfile.TemporaryDirectory(
            prefix="shar-combined-validator-evidence-"
        )
        root = Path(temporary.name)
        inputs = dict.fromkeys(
            (
                *_DEPENDENCIES._VALIDATOR_SOURCE_INPUTS,
                *_DEPENDENCIES._DEEP_VALIDATOR_SOURCE_INPUTS,
            )
        )
        for relative in inputs:
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

    def test_validator_evidence_rejects_source_drift_during_build(self) -> None:
        temporary, root = self._combined_fixture()
        try:
            manifest_source = root / "src/migration/manifest/fixture.rs"
            outputs = {
                "validate-game": root / "validate-game",
                "validate-source-deep": root / "validate-source-deep",
            }
            for output in outputs.values():
                output.write_bytes(b"validator")

            def build(
                _root: Path,
                _context: object,
                *,
                package: str,
                binary: str,
            ) -> Path:
                del package
                if binary == "validate-game":
                    manifest_source.write_text(
                        "// changed during build\n",
                        encoding="utf-8",
                    )
                return outputs[binary]

            context = _DEPENDENCIES.CargoBuildContext(
                Path("cargo"),
                Path("rustc"),
                None,
                {},
            )
            with (
                mock.patch.object(
                    _DEPENDENCIES,
                    "_build_cargo_binary",
                    side_effect=build,
                ),
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "source inputs changed during build",
                ),
            ):
                _DEPENDENCIES._validator_evidence(
                    root,
                    context,
                    publish_validator=False,
                )
        finally:
            temporary.cleanup()

    def test_validator_evidence_rejects_source_drift_during_publication(
        self,
    ) -> None:
        temporary, root = self._combined_fixture()
        try:
            manifest_source = root / "src/migration/manifest/fixture.rs"
            release = root / _DEPENDENCIES._CARGO_TARGET / "release"
            release.mkdir(parents=True)
            outputs = {
                "validate-game": release / "validate-game",
                "validate-source-deep": release / "validate-source-deep",
            }
            for output in outputs.values():
                output.write_bytes(b"validator")

            def build(
                _root: Path,
                _context: object,
                *,
                package: str,
                binary: str,
            ) -> Path:
                del package
                return outputs[binary]

            publication_count = 0

            def publish(_root: Path, built: Path) -> Path:
                nonlocal publication_count
                publication_count += 1
                if publication_count == 1:
                    manifest_source.write_text(
                        "// changed during publication\n",
                        encoding="utf-8",
                    )
                return built.resolve()

            context = _DEPENDENCIES.CargoBuildContext(
                Path("cargo"),
                Path("rustc"),
                None,
                {},
            )
            with (
                mock.patch.object(
                    _DEPENDENCIES,
                    "_build_cargo_binary",
                    side_effect=build,
                ),
                mock.patch.object(
                    _DEPENDENCIES,
                    "_publish_validator",
                    side_effect=publish,
                ),
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "source inputs changed during publication",
                ),
            ):
                _DEPENDENCIES._validator_evidence(
                    root,
                    context,
                    publish_validator=True,
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

    def test_validator_publication_allows_cargo_owned_hard_link(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-validator-build-cargo-link-"
        ) as raw:
            root = Path(raw)
            release = root / _DEPENDENCIES._CARGO_TARGET / "release"
            deps = release / "deps"
            deps.mkdir(parents=True)
            built = release / "validate-game"
            built.write_bytes(b"validator")
            (deps / "validate_game-fixture").hardlink_to(built)

            published = _DEPENDENCIES._publish_validator(root, built)

            self.assertEqual(published.read_bytes(), b"validator")

    def test_validator_publication_rejects_external_hard_link(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-validator-build-external-link-"
        ) as raw:
            root = Path(raw)
            release = root / _DEPENDENCIES._CARGO_TARGET / "release"
            deps = release / "deps"
            deps.mkdir(parents=True)
            built = release / "validate-game"
            built.write_bytes(b"validator")
            (deps / "validate_game-fixture").hardlink_to(built)
            (root / "external-validator").hardlink_to(built)

            with self.assertRaisesRegex(
                _DEPENDENCIES.BootstrapFailure,
                "external hard-link alias",
            ):
                _DEPENDENCIES._publish_validator(root, built)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_validator_publication_rejects_redirected_destination(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-validator-link-") as raw:
            root = Path(raw)
            built = (
                root / _DEPENDENCIES._CARGO_TARGET / "release/validate-game"
            )
            built.parent.mkdir(parents=True)
            built.write_bytes(b"validator")
            destination = root / _DEPENDENCIES._BIN_ROOT / built.name
            destination.parent.mkdir(parents=True)
            external = root / "external-validator"
            external.write_bytes(b"validator")
            destination.symlink_to(external)

            with self.assertRaisesRegex(
                _DEPENDENCIES.BootstrapFailure,
                "validator destination must be a real file",
            ):
                _DEPENDENCIES._publish_validator(root, built)

    def test_preflight_rejects_hard_linked_dependency_validator(self) -> None:
        temporary, root = self._fixture()
        try:
            validator = root / ".dependencies/build/bin/validate-game"
            validator.parent.mkdir(parents=True)
            validator.write_bytes(b"validator")
            alias = root / "validator-alias"
            alias.hardlink_to(validator)
            evidence = {
                "validator": {
                    "path": str(validator),
                    "sha256": hashlib.sha256(b"validator").hexdigest(),
                    "source_sha256": _CHECK._validator_source_sha256(root),
                }
            }
            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "dependency validator must be a real file",
            ):
                _CHECK._dependency_validator(root, evidence)
        finally:
            temporary.cleanup()

    def test_preflight_rejects_redirected_validator_storage(self) -> None:
        temporary, root = self._fixture()
        try:
            validator = root / ".dependencies/build/bin/validate-game"
            validator.parent.mkdir(parents=True)
            validator.write_bytes(b"validator")
            evidence = {
                "validator": {
                    "path": str(validator),
                    "sha256": hashlib.sha256(b"validator").hexdigest(),
                    "source_sha256": _CHECK._validator_source_sha256(root),
                }
            }
            build_root = root / ".dependencies/build"
            with (
                mock.patch.object(
                    _CHECK.os.path,
                    "isjunction",
                    side_effect=lambda path: Path(path) == build_root,
                ),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "validator storage must use real directories",
                ),
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


class DependencyValidatorHashBoundaryTests(unittest.TestCase):
    """Bind preflight validator hashes to repository-owned file identities."""

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_preflight_rejects_validator_replacement_during_hash(self) -> None:
        cases = (
            (
                "validator",
                "validate-game",
                _CHECK._dependency_validator,
            ),
            (
                "deep_source_validator",
                "validate-source-deep",
                _CHECK._dependency_deep_source_validator,
            ),
        )
        for field, name, resolve in cases:
            with (
                self.subTest(field=field),
                tempfile.TemporaryDirectory(
                    prefix="shar-preflight-validator-hash-race-"
                ) as raw,
            ):
                root = Path(raw)
                validator = root / ".dependencies/build/bin" / name
                validator.parent.mkdir(parents=True)
                validator.write_bytes(b"validator")
                external = root / "external-validator"
                external.write_bytes(b"validator")
                displaced = root / "displaced-validator"
                evidence = {
                    field: {
                        "path": str(validator),
                        "sha256": hashlib.sha256(b"validator").hexdigest(),
                        "source_sha256": "source",
                    }
                }
                real_identity = _CHECK._real_evidence_identity
                replaced = False

                def replace_after_identity(
                    path: Path,
                    label: str,
                    *,
                    identity_reader: Callable[
                        [Path, str], tuple[int, ...]
                    ] = real_identity,
                    validator_path: Path = validator,
                    displaced_path: Path = displaced,
                    external_path: Path = external,
                ) -> tuple[int, ...]:
                    nonlocal replaced
                    identity = identity_reader(path, label)
                    if path == validator_path and not replaced:
                        validator_path.replace(displaced_path)
                        validator_path.symlink_to(external_path)
                        replaced = True
                    return identity

                with (
                    mock.patch.object(
                        _CHECK,
                        "_real_evidence_identity",
                        side_effect=replace_after_identity,
                    ),
                    mock.patch.object(
                        _CHECK,
                        "_validator_source_sha256",
                        return_value="source",
                    ),
                    mock.patch.object(
                        _CHECK,
                        "_deep_validator_source_sha256",
                        return_value="source",
                    ),
                    self.assertRaisesRegex(
                        _CHECK.CheckFailure,
                        "changed while hashing",
                    ),
                ):
                    resolve(root, evidence)

                self.assertTrue(validator.is_symlink())
                self.assertEqual(external.read_bytes(), b"validator")

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_preflight_rejects_validator_replacement_during_source_hash(
        self,
    ) -> None:
        cases = (
            (
                "validator",
                "validate-game",
                _CHECK._dependency_validator,
                "_validator_source_sha256",
            ),
            (
                "deep_source_validator",
                "validate-source-deep",
                _CHECK._dependency_deep_source_validator,
                "_deep_validator_source_sha256",
            ),
        )
        for field, name, resolve, source_hash_name in cases:
            with (
                self.subTest(field=field),
                tempfile.TemporaryDirectory(
                    prefix="shar-preflight-validator-source-race-"
                ) as raw,
            ):
                root = Path(raw)
                validator = root / ".dependencies/build/bin" / name
                validator.parent.mkdir(parents=True)
                validator.write_bytes(b"validator")
                external = root / "external-validator"
                external.write_bytes(b"validator")
                displaced = root / "displaced-validator"
                evidence = {
                    field: {
                        "path": str(validator),
                        "sha256": hashlib.sha256(b"validator").hexdigest(),
                        "source_sha256": "source",
                    }
                }

                def replace_during_source_hash(
                    _root: Path,
                    *,
                    validator_path: Path = validator,
                    displaced_path: Path = displaced,
                    external_path: Path = external,
                ) -> str:
                    validator_path.replace(displaced_path)
                    validator_path.symlink_to(external_path)
                    return "source"

                with (
                    mock.patch.object(
                        _CHECK,
                        source_hash_name,
                        side_effect=replace_during_source_hash,
                    ),
                    self.assertRaisesRegex(
                        _CHECK.CheckFailure,
                        "must be a real file",
                    ),
                ):
                    resolve(root, evidence)

                self.assertTrue(validator.is_symlink())
                self.assertEqual(external.read_bytes(), b"validator")


class VisualStudioBootstrapBoundaryTests(unittest.TestCase):
    """Preserve pre-existing Visual Studio bootstrap staging identities."""

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_environment_batch_preserves_redirected_staging_file(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-vs-environment-staging-link-"
        ) as raw:
            root = Path(raw)
            program_files = root / "Program Files (x86)"
            vswhere = (
                program_files
                / "Microsoft Visual Studio/Installer/vswhere.exe"
            )
            vswhere.parent.mkdir(parents=True)
            vswhere.write_bytes(b"vswhere")
            installation = root / "VS"
            script = installation / "VC/Auxiliary/Build/vcvars64.bat"
            script.parent.mkdir(parents=True)
            script.write_text("@echo off\n", encoding="utf-8")
            command_root = root / _DEPENDENCIES._BOOTSTRAP_CACHE
            command_root.mkdir(parents=True)
            external = root / "external-command"
            external.write_text("outside\n", encoding="utf-8")
            batch = command_root / f"vsenv-{os.getpid()}.cmd"
            batch.symlink_to(external)

            with (
                mock.patch.dict(
                    _DEPENDENCIES.os.environ,
                    {
                        "ProgramFiles(x86)": str(program_files),
                        "SystemRoot": str(root / "Windows"),
                        "COMSPEC": "cmd.exe",
                    },
                    clear=True,
                ),
                mock.patch.object(
                    _DEPENDENCIES.subprocess,
                    "run",
                    return_value=mock.Mock(
                        stdout=f"{installation}\n"
                    ),
                ) as run,
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "Visual Studio environment staging file already exists",
                ),
            ):
                _DEPENDENCIES._visual_studio_environment(
                    root, "x86_64-pc-windows-msvc"
                )

            run.assert_called_once()
            self.assertTrue(batch.is_symlink())
            self.assertEqual(external.read_text(encoding="utf-8"), "outside\n")


class RustupBootstrapBoundaryTests(unittest.TestCase):
    """Keep pinned rustup installer authority inside repository cache files."""

    _TARGET = "x86_64-unknown-linux-gnu"
    _PAYLOAD = b"pinned-rustup-fixture"

    class _Response(io.BytesIO):
        def __enter__(self) -> RustupBootstrapBoundaryTests._Response:
            return self

        def __exit__(self, *_args: object) -> None:
            self.close()

    def _expected(self) -> str:
        return hashlib.sha256(self._PAYLOAD).hexdigest()

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_cached_rustup_rejects_redirected_installer(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-rustup-cache-link-"
        ) as raw:
            root = Path(raw)
            installer = _DEPENDENCIES._rustup_installer_path(
                root, self._TARGET
            )
            installer.parent.mkdir(parents=True)
            external = root / "external-installer"
            external.write_bytes(self._PAYLOAD)
            installer.symlink_to(external)
            with (
                mock.patch.dict(
                    _DEPENDENCIES._RUSTUP_SHA256,
                    {self._TARGET: self._expected()},
                    clear=True,
                ),
                mock.patch.object(
                    _DEPENDENCIES.urllib.request, "urlopen"
                ) as urlopen,
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "rustup installer must be a real file",
                ),
            ):
                _DEPENDENCIES._download_rustup(root, self._TARGET)

            urlopen.assert_not_called()
            self.assertEqual(external.read_bytes(), self._PAYLOAD)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_cached_rustup_rejects_replacement_during_hash(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-rustup-cache-race-"
        ) as raw:
            root = Path(raw)
            installer = root / "installer"
            installer.write_bytes(self._PAYLOAD)
            external = root / "external-installer"
            external.write_bytes(self._PAYLOAD)
            displaced = root / "displaced-installer"
            real_metadata = _DEPENDENCIES._rustup_installer_metadata
            replaced = False

            def replace_after_metadata(
                path: Path, label: str
            ) -> os.stat_result:
                nonlocal replaced
                metadata = real_metadata(path, label)
                if path == installer and not replaced:
                    installer.replace(displaced)
                    installer.symlink_to(external)
                    replaced = True
                return metadata

            with (
                mock.patch.object(
                    _DEPENDENCIES,
                    "_rustup_installer_metadata",
                    side_effect=replace_after_metadata,
                ),
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "rustup installer changed while hashing",
                ),
            ):
                _DEPENDENCIES._cached_rustup_matches(
                    installer, self._expected()
                )

            self.assertTrue(installer.is_symlink())
            self.assertEqual(external.read_bytes(), self._PAYLOAD)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_rustup_download_preserves_redirected_staging_file(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-rustup-staging-link-"
        ) as raw:
            root = Path(raw)
            installer = _DEPENDENCIES._rustup_installer_path(
                root, self._TARGET
            )
            installer.parent.mkdir(parents=True)
            external = root / "external-stage"
            external.write_bytes(b"outside")
            candidate = installer.with_name(
                f".{installer.name}.{os.getpid()}.tmp"
            )
            candidate.symlink_to(external)
            with (
                mock.patch.dict(
                    _DEPENDENCIES._RUSTUP_SHA256,
                    {self._TARGET: self._expected()},
                    clear=True,
                ),
                mock.patch.object(
                    _DEPENDENCIES.urllib.request, "urlopen"
                ) as urlopen,
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "rustup staging file already exists",
                ),
            ):
                _DEPENDENCIES._download_rustup(root, self._TARGET)

            urlopen.assert_not_called()
            self.assertTrue(candidate.is_symlink())
            self.assertEqual(external.read_bytes(), b"outside")
            self.assertFalse(installer.exists())

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_cached_toolchain_rejects_redirected_cargo(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-rust-tool-link-"
        ) as raw:
            root = Path(raw)
            required = "1.97.1"
            cargo, rustc = _DEPENDENCIES._repo_rust_paths(
                root, required, self._TARGET
            )
            cargo.parent.mkdir(parents=True)
            external = root / "external-cargo"
            external.write_bytes(b"external")
            cargo.symlink_to(external)
            rustc.write_bytes(b"rustc")
            with (
                mock.patch.object(
                    _DEPENDENCIES,
                    "_host_rust_target",
                    return_value=self._TARGET,
                ),
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "repo-local cargo must be a real file",
                ),
            ):
                _DEPENDENCIES._install_repo_rust(root, required)

    def test_cached_toolchain_rejects_hard_linked_rustc(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-rust-tool-hard-link-"
        ) as raw:
            root = Path(raw)
            required = "1.97.1"
            cargo, rustc = _DEPENDENCIES._repo_rust_paths(
                root, required, self._TARGET
            )
            cargo.parent.mkdir(parents=True)
            cargo.write_bytes(b"cargo")
            rustc.write_bytes(b"rustc")
            (root / "external-rustc-alias").hardlink_to(rustc)
            with (
                mock.patch.object(
                    _DEPENDENCIES,
                    "_host_rust_target",
                    return_value=self._TARGET,
                ),
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "repo-local rustc must be a real single-link file",
                ),
            ):
                _DEPENDENCIES._install_repo_rust(root, required)

    def test_rustup_download_hashes_owned_response_stream(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-rustup-download-"
        ) as raw:
            root = Path(raw)
            with (
                mock.patch.dict(
                    _DEPENDENCIES._RUSTUP_SHA256,
                    {self._TARGET: self._expected()},
                    clear=True,
                ),
                mock.patch.object(
                    _DEPENDENCIES.urllib.request,
                    "urlopen",
                    return_value=self._Response(self._PAYLOAD),
                ),
            ):
                installer = _DEPENDENCIES._download_rustup(
                    root, self._TARGET
                )

            self.assertEqual(installer.read_bytes(), self._PAYLOAD)
            self.assertFalse(installer.is_symlink())
            self.assertEqual(installer.stat().st_nlink, 1)


class ValidatorPublicationStagingTests(unittest.TestCase):
    """Keep validator staging exclusive and repository-local."""

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_validator_publication_preserves_redirected_staging_file(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-validator-staging-link-"
        ) as raw:
            root = Path(raw)
            release = root / _DEPENDENCIES._CARGO_TARGET / "release"
            deps = release / "deps"
            deps.mkdir(parents=True)
            built = release / "validate-game"
            built.write_bytes(b"validator")
            (deps / "validate_game-fixture").hardlink_to(built)
            destination = root / _DEPENDENCIES._BIN_ROOT / built.name
            destination.parent.mkdir(parents=True)
            external = root / "external-validator"
            external.write_bytes(b"outside")
            candidate = destination.with_name(
                f".{destination.name}.{os.getpid()}.tmp"
            )
            candidate.symlink_to(external)

            with self.assertRaisesRegex(
                _DEPENDENCIES.BootstrapFailure,
                "validator staging file already exists",
            ):
                _DEPENDENCIES._publish_validator(root, built)

            self.assertTrue(candidate.is_symlink())
            self.assertEqual(external.read_bytes(), b"outside")
            self.assertFalse(destination.exists())

    def test_validator_publication_preserves_existing_staging_file(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-validator-staging-file-"
        ) as raw:
            root = Path(raw)
            release = root / _DEPENDENCIES._CARGO_TARGET / "release"
            release.mkdir(parents=True)
            built = release / "validate-game"
            built.write_bytes(b"validator")
            destination = root / _DEPENDENCIES._BIN_ROOT / built.name
            destination.parent.mkdir(parents=True)
            candidate = destination.with_name(
                f".{destination.name}.{os.getpid()}.tmp"
            )
            candidate.write_bytes(b"preserve")

            with self.assertRaisesRegex(
                _DEPENDENCIES.BootstrapFailure,
                "validator staging file already exists",
            ):
                _DEPENDENCIES._publish_validator(root, built)

            self.assertEqual(candidate.read_bytes(), b"preserve")
            self.assertFalse(destination.exists())

    def test_validator_publication_rejects_build_drift_during_copy(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-validator-copy-drift-"
        ) as raw:
            root = Path(raw)
            release = root / _DEPENDENCIES._CARGO_TARGET / "release"
            release.mkdir(parents=True)
            built = release / "validate-game"
            built.write_bytes(b"validator-before")
            destination = root / _DEPENDENCIES._BIN_ROOT / built.name
            destination.parent.mkdir(parents=True)
            candidate = destination.with_name(
                f".{destination.name}.{os.getpid()}.tmp"
            )
            copyfileobj = _DEPENDENCIES.shutil.copyfileobj

            def drift_then_copy(*args: object, **kwargs: object) -> None:
                built.write_bytes(b"validator-after")
                copyfileobj(*args, **kwargs)

            with (
                mock.patch.object(
                    _DEPENDENCIES.shutil,
                    "copyfileobj",
                    side_effect=drift_then_copy,
                ),
                self.assertRaisesRegex(
                    _DEPENDENCIES.BootstrapFailure,
                    "changed while publishing",
                ),
            ):
                _DEPENDENCIES._publish_validator(root, built)

            self.assertFalse(candidate.exists())
            self.assertFalse(destination.exists())


class ValidatorSourceAliasTests(unittest.TestCase):
    """Reject physical aliases from validator source fingerprints."""

    def test_source_closure_rejects_hard_linked_files(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-validator-source-hard-link-"
        ) as raw:
            root = Path(raw)
            for relative in _DEPENDENCIES._VALIDATOR_SOURCE_INPUTS:
                path = root / relative
                if path.suffix:
                    path.parent.mkdir(parents=True, exist_ok=True)
                    path.write_text("fixture\n", encoding="utf-8")
                else:
                    path.mkdir(parents=True, exist_ok=True)
                    (path / "fixture.rs").write_text(
                        "fixture\n", encoding="utf-8"
                    )
            source = root / "Cargo.toml"
            alias = root / "external-source-alias"
            alias.hardlink_to(source)

            for module in (_DEPENDENCIES, _CHECK):
                with (
                    self.subTest(module=module.__name__),
                    self.assertRaisesRegex(
                        OSError,
                        "source closure contains a hard-linked file",
                    ),
                ):
                    module._validator_source_sha256(root)


class EngineSelectionTests(unittest.TestCase):
    """Exercise portable default Unreal Engine candidate selection."""

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_engine_rejects_build_version_replacement_during_read(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-engine-version-race-"
        ) as raw:
            root = Path(raw)
            engine = root / "UE_5.8"
            version = engine / "Engine/Build/Build.version"
            version.parent.mkdir(parents=True)
            local_payload = (
                b'{"MajorVersion":5,"MinorVersion":7,"PatchVersion":0}\n'
            )
            external_payload = (
                b'{"MajorVersion":5,"MinorVersion":8,"PatchVersion":1}\n'
            )
            version.write_bytes(local_payload)
            external = root / "external-Build.version"
            external.write_bytes(external_payload)
            displaced = root / "displaced-Build.version"
            real_identity = _CHECK._real_evidence_identity
            replaced = False

            def replace_after_identity(
                path: Path, label: str
            ) -> tuple[int, ...]:
                nonlocal replaced
                identity = real_identity(path, label)
                if path == version and not replaced:
                    version.replace(displaced)
                    version.symlink_to(external)
                    replaced = True
                return identity

            with (
                mock.patch.object(
                    _CHECK,
                    "_real_evidence_identity",
                    side_effect=replace_after_identity,
                ),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "Unreal Build.version changed while reading",
                ),
            ):
                _CHECK._engine_version(engine)

            self.assertTrue(version.is_symlink())
            self.assertEqual(displaced.read_bytes(), local_payload)
            self.assertEqual(external.read_bytes(), external_payload)

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


class CanonicalProjectBoundaryTests(unittest.TestCase):
    """Keep canonical Unreal project authority inside the repository."""

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_preflight_rejects_linked_project_descriptor(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-project-descriptor-link-"
        ) as raw:
            root = Path(raw)
            project = root / _CHECK._PROJECT_PATH
            project.parent.mkdir(parents=True)
            external = root / "external.uproject"
            external.write_text(
                '{"EngineAssociation":"5.8"}\n',
                encoding="utf-8",
            )
            project.symlink_to(external)

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "Unreal project descriptor must be a real file",
            ):
                _CHECK._check_project(root)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_preflight_rejects_linked_project_parent(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-project-parent-link-"
        ) as raw:
            root = Path(raw)
            outside = root / "outside-project"
            outside.mkdir()
            (outside / "shar.uproject").write_text(
                '{"EngineAssociation":"5.8"}\n',
                encoding="utf-8",
            )
            composition = root / "src/unreal/project/composition"
            composition.mkdir(parents=True)
            (composition / "uproject").symlink_to(
                outside, target_is_directory=True
            )

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "Unreal project root must be a real directory",
            ):
                _CHECK._check_project(root)

    def test_preflight_rejects_junction_project_parent(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-project-parent-junction-"
        ) as raw:
            root = Path(raw)
            project_root = root / "src/unreal/project/composition/uproject"
            project_root.mkdir(parents=True)
            project = project_root / "shar.uproject"
            project.write_text(
                '{"EngineAssociation":"5.8"}\n',
                encoding="utf-8",
            )
            with (
                mock.patch.object(
                    _CHECK.os.path,
                    "isjunction",
                    side_effect=lambda path: Path(path) == project_root,
                ),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "Unreal project root must be a real directory",
                ),
            ):
                _CHECK._check_project(root)


class PreflightEvidenceSnapshotTests(unittest.TestCase):
    """Bind preflight to one stable dependency-evidence snapshot."""

    def test_preflight_rejects_dependency_evidence_drift(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-preflight-dependency-drift-"
        ) as raw:
            root = Path(raw)
            dependency_path = root / _CHECK._DEPENDENCIES_PATH
            dependency_path.parent.mkdir(parents=True)
            dependency_path.write_text(
                '{"schema":"shar.build.dependencies.v1"}\n',
                encoding="utf-8",
            )
            manifest = root / "game/manifest/game.jsonl"
            manifest.parent.mkdir(parents=True)
            manifest.write_text("fixture\n", encoding="utf-8")
            game = root / "game-source"
            project = root / "src/unreal/project/shar.uproject"
            validator = root / ".dependencies/build/bin/validate-game"
            deep_validator = (
                root / ".dependencies/build/bin/validate-source-deep"
            )
            engine = _CHECK.EngineEvidence(root / "engine", "5.8.1")
            args = _CHECK.argparse.Namespace(
                engine_root=None,
                game=game,
                manifest_validator=None,
                deep_source_validator=None,
            )

            def manifest_gate(*_args: object) -> str:
                dependency_path.write_text(
                    '{"schema":"shar.build.dependencies.v1","drift":true}\n',
                    encoding="utf-8",
                )
                return "manifest-ok"

            with (
                mock.patch.object(_CHECK, "_root", return_value=root),
                mock.patch.object(_CHECK, "_check_python", return_value={}),
                mock.patch.object(_CHECK, "_check_game", return_value=game),
                mock.patch.object(
                    _CHECK,
                    "_check_project",
                    return_value=_CHECK.ProjectEvidence(project, b"{}"),
                ),
                mock.patch.object(
                    _CHECK,
                    "_resolve_validator",
                    return_value=validator,
                ),
                mock.patch.object(
                    _CHECK,
                    "_resolve_deep_source_validator",
                    return_value=deep_validator,
                ),
                mock.patch.object(
                    _CHECK,
                    "_check_manifest",
                    side_effect=manifest_gate,
                ),
                mock.patch.object(
                    _CHECK,
                    "_check_deep_source",
                    return_value="deep-ok",
                ),
                mock.patch.object(_CHECK, "_check_engine", return_value=engine),
                mock.patch.object(_CHECK, "_host_evidence", return_value={}),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "dependency evidence changed during preflight",
                ),
            ):
                _CHECK._run(args)

    def test_preflight_rejects_manifest_drift(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-preflight-manifest-drift-"
        ) as raw:
            root = Path(raw)
            dependency_path = root / _CHECK._DEPENDENCIES_PATH
            dependency_path.parent.mkdir(parents=True)
            dependency_path.write_text(
                '{"schema":"shar.build.dependencies.v1"}\n',
                encoding="utf-8",
            )
            manifest = root / "game/manifest/game.jsonl"
            manifest.parent.mkdir(parents=True)
            manifest.write_text("validated\n", encoding="utf-8")
            game = root / "game-source"
            project = root / "src/unreal/project/shar.uproject"
            validator = root / ".dependencies/build/bin/validate-game"
            deep_validator = (
                root / ".dependencies/build/bin/validate-source-deep"
            )
            engine = _CHECK.EngineEvidence(root / "engine", "5.8.1")
            args = _CHECK.argparse.Namespace(
                engine_root=None,
                game=game,
                manifest_validator=None,
                deep_source_validator=None,
            )

            def manifest_gate(*_args: object) -> str:
                manifest.write_text("changed\n", encoding="utf-8")
                return "manifest-ok"

            with (
                mock.patch.object(_CHECK, "_root", return_value=root),
                mock.patch.object(_CHECK, "_check_python", return_value={}),
                mock.patch.object(_CHECK, "_check_game", return_value=game),
                mock.patch.object(
                    _CHECK,
                    "_check_project",
                    return_value=_CHECK.ProjectEvidence(project, b"{}"),
                ),
                mock.patch.object(
                    _CHECK,
                    "_resolve_validator",
                    return_value=validator,
                ),
                mock.patch.object(
                    _CHECK,
                    "_resolve_deep_source_validator",
                    return_value=deep_validator,
                ),
                mock.patch.object(
                    _CHECK,
                    "_check_manifest",
                    side_effect=manifest_gate,
                ),
                mock.patch.object(
                    _CHECK,
                    "_check_deep_source",
                    return_value="deep-ok",
                ),
                mock.patch.object(_CHECK, "_check_engine", return_value=engine),
                mock.patch.object(_CHECK, "_host_evidence", return_value={}),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "canonical game manifest changed during preflight",
                ),
            ):
                _CHECK._run(args)

    def test_preflight_rejects_project_descriptor_drift(self) -> None:
        root = Path("/synthetic/repository")
        game = Path("/synthetic/game")
        manifest = root / "game/manifest/game.jsonl"
        project = root / _CHECK._PROJECT_PATH
        dependency_path = root / _CHECK._DEPENDENCIES_PATH
        validator = root / ".dependencies/build/bin/validate-game"
        deep_validator = root / ".dependencies/build/bin/validate-source-deep"
        engine = _CHECK.EngineEvidence(Path("/synthetic/engine"), "5.8.1")
        args = _CHECK.argparse.Namespace(
            engine_root=None,
            game=game,
            manifest_validator=None,
            deep_source_validator=None,
        )

        def read_snapshot(path: Path, _label: str) -> bytes:
            if path == project:
                return b"changed-project"
            if path == dependency_path:
                return b"dependency"
            if path == manifest:
                return b"manifest"
            raise AssertionError(path)

        with (
            mock.patch.object(_CHECK, "_root", return_value=root),
            mock.patch.object(_CHECK, "_check_python", return_value={}),
            mock.patch.object(_CHECK, "_check_game", return_value=game),
            mock.patch.object(_CHECK, "_require_real_manifest_roots"),
            mock.patch.object(
                _CHECK,
                "_check_project",
                return_value=_CHECK.ProjectEvidence(project, b"project"),
            ),
            mock.patch.object(
                _CHECK,
                "_dependency_evidence",
                return_value=(dependency_path, b"dependency", {}),
            ),
            mock.patch.object(
                _CHECK, "_resolve_validator", return_value=validator
            ),
            mock.patch.object(
                _CHECK,
                "_resolve_deep_source_validator",
                return_value=deep_validator,
            ),
            mock.patch.object(_CHECK, "_check_manifest", return_value="ok"),
            mock.patch.object(_CHECK, "_check_deep_source", return_value="ok"),
            mock.patch.object(_CHECK, "_check_engine", return_value=engine),
            mock.patch.object(_CHECK, "_host_evidence", return_value={}),
            mock.patch.object(
                _CHECK,
                "_read_real_evidence_bytes",
                side_effect=read_snapshot,
            ),
            self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "Unreal project descriptor changed during preflight",
            ),
        ):
            _CHECK._run(args)


class CanonicalManifestBoundaryTests(unittest.TestCase):
    """Keep canonical manifest authority inside real repository directories."""

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_preflight_rejects_linked_manifest_parent(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-manifest-parent-link-"
        ) as raw:
            root = Path(raw)
            outside = root / "outside-manifest"
            outside.mkdir()
            (outside / "game.jsonl").write_text(
                "external\n", encoding="utf-8"
            )
            game = root / "game"
            game.mkdir()
            (game / "manifest").symlink_to(
                outside, target_is_directory=True
            )

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "canonical manifest root must be a real directory",
            ):
                _CHECK._require_real_manifest_roots(root)

    def test_preflight_rejects_junction_manifest_parent(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-manifest-parent-junction-"
        ) as raw:
            root = Path(raw)
            manifest_root = root / "game/manifest"
            manifest_root.mkdir(parents=True)
            with (
                mock.patch.object(
                    _CHECK.os.path,
                    "isjunction",
                    side_effect=lambda path: Path(path) == manifest_root,
                ),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "canonical manifest root must be a real directory",
                ),
            ):
                _CHECK._require_real_manifest_roots(root)


class CheckRevalidationSnapshotTests(unittest.TestCase):
    """Keep direct check revalidation bound to one saved snapshot."""

    def test_revalidation_rejects_hard_linked_saved_evidence(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-check-revalidation-hard-link-"
        ) as raw:
            root = Path(raw)
            external = root / "external-check.json"
            external.write_text("{}\n", encoding="utf-8")
            path = root / "check.json"
            path.hardlink_to(external)

            with self.assertRaisesRegex(
                _CHECK.CheckFailure,
                "saved check evidence must be a real file",
            ):
                _CHECK._revalidate(path)

    def test_revalidation_rejects_saved_evidence_drift(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-check-revalidation-drift-"
        ) as raw:
            path = Path(raw) / "check.json"
            saved = {
                "game": {"path": "/lawful/game"},
                "schema": _CHECK._SCHEMA,
                "unreal": {"root": "/engine"},
            }
            path.write_text(
                _CHECK.json.dumps(saved) + "\n",
                encoding="utf-8",
            )

            def recompute(_args: object) -> dict[str, object]:
                path.write_text(
                    _CHECK.json.dumps({**saved, "drift": True}) + "\n",
                    encoding="utf-8",
                )
                return saved

            with (
                mock.patch.object(_CHECK, "_run", side_effect=recompute),
                self.assertRaisesRegex(
                    _CHECK.CheckFailure,
                    "saved check evidence changed during revalidation",
                ),
            ):
                _CHECK._revalidate(path)


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

            with self.assertRaises(_CHECK.CheckFailure) as raised:
                _CHECK._check_game(repository, source)

            message = str(raised.exception)
            self.assertEqual(
                message,
                "selected source contains another nested Simpsons.exe",
            )
            self.assertNotIn("copy", message)

            nested_only = root / "nested-only"
            private_copy = nested_only / "private-copy"
            private_copy.mkdir(parents=True)
            (private_copy / "Simpsons.exe").write_bytes(b"fixture")

            with self.assertRaises(_CHECK.CheckFailure) as raised:
                _CHECK._check_game(repository, nested_only)

            message = str(raised.exception)
            self.assertEqual(
                message,
                "Simpsons.exe must be directly inside the selected source",
            )
            self.assertNotIn("private-copy", message)

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
            prefix="shar-source-unreadable-"
        ) as value:
            root = Path(value)
            repository = root / "repository"
            source = root / "installed-game"
            blocked = source / "private-copy"
            repository.mkdir()
            blocked.mkdir(parents=True)
            (source / "Simpsons.exe").write_bytes(b"fixture")
            real_scan = _CHECK._scan_directory

            def strict_scan(
                path: Path,
            ) -> AbstractContextManager[Iterator[os.DirEntry[str]]]:
                if path == blocked:
                    raise PermissionError(f"cannot scan {blocked}")
                return real_scan(path)

            with (
                mock.patch.object(_CHECK, "_scan_directory", strict_scan),
                self.assertRaises(_CHECK.CheckFailure) as raised,
            ):
                _CHECK._check_game(repository, source)

            message = str(raised.exception)
            self.assertNotIn(str(blocked), message)
            self.assertEqual(
                message,
                "selected source cannot be inspected safely",
            )

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

            source = root / "private-user-installation"
            source.mkdir()
            (source / "Simpsons.exe").write_bytes(b"fixture")
            native = OSError(f"cannot scan {source}")
            with (
                mock.patch.object(Path, "iterdir", side_effect=native),
                self.assertRaises(_CHECK.CheckFailure) as raised,
            ):
                _CHECK._check_game(repository, source)

            message = str(raised.exception)
            self.assertNotIn(str(source), message)
            self.assertEqual(
                message,
                "selected source cannot be inspected safely",
            )

    def test_redaction_covers_escaped_windows_source_path(self) -> None:
        source = Path(r"C:\private\user\installed-game")
        diagnostic = (
            r"scan C:\private\user\installed-game: access denied"
        )
        escaped = diagnostic.replace("\\", "\\\\")

        for value in (escaped, escaped.upper()):
            redacted = _CHECK._redact_selected_source(value, source)

            self.assertNotIn("private", redacted.casefold())
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

        def resolve_manifest(
            _root: Path,
            _explicit: Path | None,
            _dependencies: dict[str, object],
        ) -> Path:
            calls.append("resolve-manifest")
            return manifest_validator

        def resolve_deep(
            _root: Path,
            _explicit: Path | None,
            _dependencies: dict[str, object],
        ) -> Path:
            calls.append("resolve-deep")
            return deep_validator

        with (
            mock.patch.object(_CHECK, "_root", return_value=root),
            mock.patch.object(_CHECK, "_check_python", return_value={}),
            mock.patch.object(_CHECK, "_check_game", return_value=game),
            mock.patch.object(_CHECK, "_require_real_manifest_roots"),
            mock.patch.object(Path, "is_file", return_value=True),
            mock.patch.object(
                _CHECK,
                "_check_project",
                return_value=_CHECK.ProjectEvidence(project, b"{}"),
            ),
            mock.patch.object(
                _CHECK,
                "_dependency_evidence",
                return_value=(dependency_path, b"{}", {}),
            ),
            mock.patch.object(
                _CHECK,
                "_read_real_evidence_bytes",
                return_value=b"{}",
            ),
            mock.patch.object(
                _CHECK,
                "_resolve_validator",
                side_effect=resolve_manifest,
            ),
            mock.patch.object(
                _CHECK,
                "_resolve_deep_source_validator",
                side_effect=resolve_deep,
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

        self.assertEqual(
            calls,
            ["resolve-manifest", "manifest", "resolve-deep", "deep"],
        )
        self.assertEqual(evidence["game"]["validation"], "manifest-ok")
        self.assertEqual(
            evidence["game"]["deep_validation"],
            "deep-source\tfiles=0\tp3d=0\trcf=0\trsd=0\trmv=0",
        )
        self.assertEqual(
            evidence["deep_source_validator"],
            str(deep_validator.resolve()),
        )
        self.assertEqual(
            evidence["unreal"]["project_sha256"],
            hashlib.sha256(b"{}").hexdigest(),
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


if __name__ == "__main__":
    unittest.main()

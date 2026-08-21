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

from collections.abc import Callable
import hashlib
import importlib.util
import os
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


def _synthetic_elf(machine: int) -> bytes:
    """Return the minimum ELF header needed by runner target validation."""
    header = bytearray(20)
    header[:4] = b"\x7fELF"
    header[4] = 2
    header[5] = 1
    header[18:20] = machine.to_bytes(2, "little")
    return bytes(header)


def _synthetic_macho(cpu: int) -> bytes:
    """Return one minimal little-endian 64-bit Mach-O header."""
    return bytes.fromhex("cffaedfe") + cpu.to_bytes(4, "little")


def _synthetic_fat_macho(cpu_types: tuple[int, ...]) -> bytes:
    """Return one bounded big-endian universal Mach-O with tiny slices."""
    entry_size = 20
    table_end = 8 + (entry_size * len(cpu_types))
    slices = [_synthetic_macho(cpu) for cpu in cpu_types]
    offset = table_end
    entries: list[bytes] = []
    for cpu, payload in zip(cpu_types, slices, strict=True):
        entries.append(
            cpu.to_bytes(4, "big")
            + (b"\0" * 4)
            + offset.to_bytes(4, "big")
            + len(payload).to_bytes(4, "big")
            + (b"\0" * 4)
        )
        offset += len(payload)
    return (
        bytes.fromhex("cafebabe")
        + len(cpu_types).to_bytes(4, "big")
        + b"".join(entries)
        + b"".join(slices)
    )


def _write_android_apk(path: Path, machine: int = 0x00B7) -> None:
    """Write one synthetic APK with a native library entry."""
    with _RUN.zipfile.ZipFile(path, "w") as archive:
        archive.writestr(
            "lib/arm64-v8a/libUnreal.so",
            _synthetic_elf(machine),
        )


def _write_ios_ipa(
    path: Path,
    cpu: int = 0x0100000C,
    *,
    binary: bytes | None = None,
) -> None:
    """Write one synthetic IPA with a declared main executable."""
    with _RUN.zipfile.ZipFile(path, "w") as archive:
        archive.writestr(
            "Payload/SHAR.app/Info.plist",
            _RUN.plistlib.dumps({"CFBundleExecutable": "shar"}),
        )
        archive.writestr(
            "Payload/SHAR.app/shar",
            _synthetic_macho(cpu) if binary is None else binary,
        )


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

    def test_rejects_linked_repository_cache_root(self) -> None:
        temporary, root, project = self._fixture()
        try:
            cache_root = root / ".cache"
            cache_root.mkdir()
            original = _RUN._is_directory_link

            def report_cache_as_link(path: Path) -> bool:
                return path == cache_root or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_cache_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "repository cache root must be a real directory",
                ),
            ):
                _RUN._prepare_project_state(root, project)

            self.assertFalse((root / _RUN._PROJECT_STATE_ROOT).exists())
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()

    def test_rejects_linked_build_cache_root(self) -> None:
        temporary, root, project = self._fixture()
        try:
            build_root = root / ".cache/build"
            build_root.mkdir(parents=True)
            original = _RUN._is_directory_link

            def report_build_as_link(path: Path) -> bool:
                return path == build_root or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_build_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "build cache root must be a real directory",
                ),
            ):
                _RUN._prepare_project_state(root, project)

            self.assertFalse((root / _RUN._PROJECT_STATE_ROOT).exists())
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


class UatLauncherTests(unittest.TestCase):
    """Require the process launcher to be a real host-runnable file."""

    def test_rejects_linked_uat_launcher(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-launcher-") as raw:
            engine = Path(raw)
            launcher = engine / "Engine/Build/BatchFiles/RunUAT.sh"
            launcher.parent.mkdir(parents=True)
            launcher.write_text("#!/bin/sh\n", encoding="utf-8")
            launcher.chmod(0o755)
            original = Path.is_symlink

            def report_launcher_as_link(path: Path) -> bool:
                return path == launcher or original(path)

            with (
                mock.patch.object(
                    Path,
                    "is_symlink",
                    report_launcher_as_link,
                ),
                mock.patch.object(_RUN.os, "name", "posix"),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "launcher must be a real file",
                ),
            ):
                _RUN._uat_path(engine)

    @unittest.skipIf(_RUN.os.name == "nt", "POSIX launcher permission")
    def test_requires_executable_uat_launcher(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-launcher-") as raw:
            engine = Path(raw)
            launcher = engine / "Engine/Build/BatchFiles/RunUAT.sh"
            launcher.parent.mkdir(parents=True)
            launcher.write_text("#!/bin/sh\n", encoding="utf-8")
            launcher.chmod(0o644)

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "launcher is not executable",
            ):
                _RUN._uat_path(engine)

            launcher.chmod(0o755)
            self.assertEqual(_RUN._uat_path(engine), launcher)


class BuildWorkRootTests(unittest.TestCase):
    """Keep Turnkey and UAT work below real repository cache roots."""

    def test_rejects_linked_shared_run_root_before_sdk_verification(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-run-root-") as raw:
            root = Path(raw)
            build_root = root / ".cache/build"
            build_root.mkdir(parents=True)
            run_root = root / _RUN._WORK_ROOT
            run_root.mkdir()
            original = _RUN._is_directory_link

            def report_run_as_link(path: Path) -> bool:
                return path == run_root or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_run_as_link,
                ),
                mock.patch.object(_RUN, "_verify_sdk") as verify_sdk,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "build run root must be a real directory",
                ),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project/shar.uproject"),
                    _RUN._TARGETS_BY_ID["linux-x64"],
                    validate_only=True,
                )
            verify_sdk.assert_not_called()

    def test_rejects_linked_target_work_root_before_sdk_verification(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-target-work-") as raw:
            root = Path(raw)
            run_root = root / _RUN._WORK_ROOT
            run_root.mkdir(parents=True)
            target = _RUN._TARGETS_BY_ID["linux-x64"]
            work = run_root / target.identifier
            work.mkdir()
            original = _RUN._is_directory_link

            def report_work_as_link(path: Path) -> bool:
                return path == work or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_work_as_link,
                ),
                mock.patch.object(_RUN, "_verify_sdk") as verify_sdk,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "target work root must be a real directory",
                ),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project/shar.uproject"),
                    target,
                    validate_only=True,
                )
            verify_sdk.assert_not_called()


class BuildScratchPathTests(unittest.TestCase):
    """Reject redirected UAT candidate and staging scratch identities."""

    def _assert_linked_scratch_rejected(self, name: str, label: str) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-build-scratch-") as raw:
            root = Path(raw)
            work = root / _RUN._WORK_ROOT / "linux-x64"
            work.mkdir(parents=True)
            scratch = work / name
            scratch.mkdir()
            original = _RUN._is_directory_link
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def report_scratch_as_link(path: Path) -> bool:
                return path == scratch or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_scratch_as_link,
                ),
                mock.patch.object(_RUN, "_verify_sdk"),
                mock.patch.object(_RUN, "_run_uat") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    f"{label} must be a real directory",
                ),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    validate_only=False,
                )

            process.assert_not_called()
            self.assertTrue(scratch.is_dir())

    def test_rejects_linked_candidate_scratch_before_cleanup(self) -> None:
        self._assert_linked_scratch_rejected(
            "candidate",
            "candidate scratch root",
        )

    def test_rejects_linked_staging_scratch_before_cleanup(self) -> None:
        self._assert_linked_scratch_rejected(
            "stage",
            "staging scratch root",
        )


class UatWorkPathTests(unittest.TestCase):
    """Keep UAT logs and caches under real work-root identities."""

    def test_rejects_linked_log_before_subprocess(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-log-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            log = work / "build.log"
            log.write_text("sentinel\n", encoding="utf-8")
            original = Path.is_symlink

            def report_log_as_link(path: Path) -> bool:
                return path == log or original(path)

            with (
                mock.patch.object(Path, "is_symlink", report_log_as_link),
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT log must be a real file",
                ),
            ):
                _RUN._run_uat(root, Path("/uat"), ["probe"], log)

            self.assertEqual(log.read_text(encoding="utf-8"), "sentinel\n")
            process.assert_not_called()

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_rejects_log_replacement_before_truncate(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-log-race-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            log = work / "build.log"
            log.write_text("sentinel\n", encoding="utf-8")
            external = root / "external.log"
            external.write_text("outside\n", encoding="utf-8")
            displaced = root / "displaced.log"
            real_identity = _RUN._real_file_identity
            replaced = False

            def replace_after_identity(
                path: Path, label: str
            ) -> tuple[int, ...]:
                nonlocal replaced
                identity = real_identity(path, label)
                if path == log and not replaced:
                    log.replace(displaced)
                    log.symlink_to(external)
                    replaced = True
                return identity

            with (
                mock.patch.object(
                    _RUN,
                    "_real_file_identity",
                    side_effect=replace_after_identity,
                ),
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT log changed before opening",
                ),
            ):
                _RUN._run_uat(root, Path("/uat"), ["probe"], log)

            self.assertEqual(external.read_text(encoding="utf-8"), "outside\n")
            self.assertEqual(
                displaced.read_text(encoding="utf-8"), "sentinel\n"
            )
            process.assert_not_called()

    def test_rejects_hard_linked_log_before_subprocess(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-log-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            outside = root / "outside.log"
            outside.write_text("sentinel\n", encoding="utf-8")
            log = work / "build.log"
            log.hardlink_to(outside)

            with (
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT log must have one filesystem link",
                ),
            ):
                _RUN._run_uat(root, Path("/uat"), ["probe"], log)

            self.assertEqual(outside.read_text(encoding="utf-8"), "sentinel\n")
            process.assert_not_called()

    def test_rejects_linked_automation_saved_before_subprocess(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-saved-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            automation = work / "automation-saved"
            automation.mkdir()
            original = _RUN._is_directory_link

            def report_automation_as_link(path: Path) -> bool:
                return path == automation or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_automation_as_link,
                ),
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT saved root must be a real directory",
                ),
            ):
                _RUN._run_uat(
                    root,
                    Path("/uat"),
                    ["probe"],
                    work / "build.log",
                )

            process.assert_not_called()

    def test_rejects_linked_ddc_before_subprocess(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-ddc-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            ddc = work / "ddc"
            ddc.mkdir()
            original = _RUN._is_directory_link

            def report_ddc_as_link(path: Path) -> bool:
                return path == ddc or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_ddc_as_link,
                ),
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT DDC root must be a real directory",
                ),
            ):
                _RUN._run_uat(
                    root,
                    Path("/uat"),
                    ["probe"],
                    work / "build.log",
                )

            process.assert_not_called()


class TurnkeyReportTests(unittest.TestCase):
    """Require Turnkey SDK evidence to remain a real cache-owned file."""

    def test_rejects_preexisting_linked_sdk_report_before_uat(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            report.write_text("sentinel\n", encoding="utf-8")
            target = _RUN._TARGETS_BY_ID["linux-x64"]
            original = Path.is_symlink

            def report_as_link(path: Path) -> bool:
                return path == report or original(path)

            with (
                mock.patch.object(Path, "is_symlink", report_as_link),
                mock.patch.object(_RUN, "_run_uat") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report must be a real file",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

            self.assertEqual(report.read_text(encoding="utf-8"), "sentinel\n")
            process.assert_not_called()

    def test_rejects_preexisting_hard_linked_sdk_report_before_uat(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            outside = root / "outside.txt"
            outside.write_text("sentinel\n", encoding="utf-8")
            report = work / "turnkey.txt"
            report.hardlink_to(outside)
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            with (
                mock.patch.object(_RUN, "_run_uat") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report must have one filesystem link",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

            self.assertEqual(outside.read_text(encoding="utf-8"), "sentinel\n")
            self.assertTrue(report.exists())
            process.assert_not_called()

    def test_rejects_hard_linked_sdk_report_after_uat(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            outside = root / "outside.txt"
            outside.write_text(
                "Linux: (Status=Valid, MinAllowed=0)\n",
                encoding="utf-8",
            )
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def write_hard_linked_report(
                *_args: object,
                **_kwargs: object,
            ) -> None:
                report.hardlink_to(outside)

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_hard_linked_report,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report must have one filesystem link",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_rejects_sdk_report_replacement_during_read(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-turnkey-report-race-"
        ) as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            external = root / "external.txt"
            displaced = root / "displaced.txt"
            target = _RUN._TARGETS_BY_ID["windows-x64"]
            external.write_text(
                "Win64: (Status=Valid, MinAllowed=0)\n",
                encoding="utf-8",
            )
            real_identity = _RUN._real_file_identity
            replaced = False

            def write_invalid_report(
                *_args: object,
                **_kwargs: object,
            ) -> None:
                report.write_text(
                    "Win64: (Status=Invalid,)\n",
                    encoding="utf-8",
                )

            def replace_after_identity(
                path: Path, label: str
            ) -> tuple[int, ...]:
                nonlocal replaced
                identity = real_identity(path, label)
                if path == report and not replaced:
                    report.replace(displaced)
                    report.symlink_to(external)
                    replaced = True
                return identity

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_invalid_report,
                ),
                mock.patch.object(
                    _RUN,
                    "_real_file_identity",
                    side_effect=replace_after_identity,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report changed while reading",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

            self.assertTrue(report.is_symlink())
            self.assertEqual(
                displaced.read_text(encoding="utf-8"),
                "Win64: (Status=Invalid,)\n",
            )
            self.assertEqual(
                external.read_text(encoding="utf-8"),
                "Win64: (Status=Valid, MinAllowed=0)\n",
            )

    def test_accepts_exact_platform_sdk_row(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def write_valid_report(*_args: object, **_kwargs: object) -> None:
                report.write_text(
                    "Turnkey report\nLinux: (Status=Valid, MinAllowed=0)\n",
                    encoding="utf-8",
                )

            with mock.patch.object(
                _RUN,
                "_run_uat",
                side_effect=write_valid_report,
            ):
                actual = _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

            self.assertEqual(actual, report)

    def test_rejects_prefixed_platform_sdk_row(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def write_wrong_platform(*_args: object, **_kwargs: object) -> None:
                report.write_text(
                    "FakeLinux: (Status=Valid, MinAllowed=0)\n",
                    encoding="utf-8",
                )

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_wrong_platform,
                ),
                self.assertRaisesRegex(_RUN.RunFailure, "SDK is invalid"),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

    def test_rejects_non_utf8_sdk_report(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def write_non_utf8_report(
                *_args: object,
                **_kwargs: object,
            ) -> None:
                report.write_bytes(bytes([255, 254, 253]))

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_non_utf8_report,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "cannot read Turnkey SDK report",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

    def test_rejects_linked_sdk_report_after_uat(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            target = _RUN._TARGETS_BY_ID["linux-x64"]
            original = Path.is_symlink

            def write_linked_report(*_args: object, **_kwargs: object) -> None:
                report.write_text(
                    "Linux: (Status=Valid, MinAllowed=0)\n",
                    encoding="utf-8",
                )

            def report_as_link(path: Path) -> bool:
                return path == report or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_linked_report,
                ),
                mock.patch.object(Path, "is_symlink", report_as_link),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report must be a real file",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )


class PublicationTransactionTests(unittest.TestCase):
    """Keep malformed existing publications unchanged on rejection."""

    def test_rejects_existing_file_before_publication_mutation(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-publish-file-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            candidate.mkdir()
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            destination = root / "dist/linux-x64"
            destination.parent.mkdir()
            destination.write_text("old", encoding="utf-8")
            backup = destination.with_name(".linux-x64.previous")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "published target must be a real directory",
            ):
                _RUN._publish(candidate, destination)

            self.assertEqual(destination.read_text(encoding="utf-8"), "old")
            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse(backup.exists())

    def test_rejects_linked_dist_root_before_candidate_moves(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-publish-link-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            candidate.mkdir()
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            dist = root / "dist"
            dist.mkdir()
            destination = dist / "linux-x64"
            original = _RUN._is_directory_link

            def report_dist_as_link(path: Path) -> bool:
                return path == dist or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_dist_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "publication root must be a real directory",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse(destination.exists())

    def test_rejects_broken_link_backup_before_candidate_moves(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-backup-link-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            destination = root / "dist/linux-x64"
            candidate.mkdir()
            destination.parent.mkdir()
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            backup = destination.with_name(".linux-x64.previous")
            backup.mkdir()
            original_exists = Path.exists
            original_link = _RUN._is_directory_link

            def report_backup_missing(path: Path) -> bool:
                if path == backup:
                    return False
                return original_exists(path)

            def report_backup_as_link(path: Path) -> bool:
                return path == backup or original_link(path)

            with (
                mock.patch.object(Path, "exists", report_backup_missing),
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_backup_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "publication backup must be a real directory",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse(destination.exists())
            self.assertTrue(backup.is_dir())

    def test_cleanup_failure_rolls_back_publication_swap(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-publish-rollback-",
        ) as raw:
            root = Path(raw)
            candidate = root / "candidate"
            destination = root / "dist/linux-x64"
            candidate.mkdir()
            destination.mkdir(parents=True)
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            (destination / "old.txt").write_text("old", encoding="utf-8")
            backup = destination.with_name(".linux-x64.previous")
            original = _RUN.shutil.rmtree

            def fail_backup_cleanup(
                path: Path,
                *args: object,
                **kwargs: object,
            ) -> None:
                if Path(path) == backup:
                    raise OSError("injected backup cleanup failure")
                original(path, *args, **kwargs)

            with (
                mock.patch.object(
                    _RUN.shutil,
                    "rmtree",
                    side_effect=fail_backup_cleanup,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "publication cleanup failed",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((destination / "old.txt").is_file())
            self.assertFalse((destination / "new.txt").exists())
            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse(backup.exists())

    def test_rejects_hard_linked_candidate_file_before_publication(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-candidate-hard-link-",
        ) as raw:
            root = Path(raw)
            outside = root / "outside.bin"
            outside.write_bytes(b"runtime")
            candidate = root / "candidate"
            candidate.mkdir()
            linked = candidate / "runtime.bin"
            linked.hardlink_to(outside)
            destination = root / "dist/linux-x64"

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "candidate package contains a hard-linked file",
            ):
                _RUN._publish(candidate, destination)

            self.assertEqual(outside.read_bytes(), b"runtime")
            self.assertTrue(linked.is_file())
            self.assertFalse(destination.exists())

    def test_rejects_linked_candidate_before_publication(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-link-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            candidate.mkdir()
            (candidate / "runtime.bin").write_bytes(b"runtime")
            destination = root / "dist/linux-x64"
            original = _RUN._is_directory_link

            def report_candidate_as_link(path: Path) -> bool:
                return path == candidate or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_candidate_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "candidate package must be a real directory",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((candidate / "runtime.bin").is_file())
            self.assertFalse(destination.exists())


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


class ArtifactCacheEntryTests(unittest.TestCase):
    """Require cached packaging diagnostics to be regular files."""

    def test_rejects_manifest_named_directory(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-entry-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            candidate.mkdir()
            (candidate / "Manifest_UFSFiles_Linux.txt").mkdir()

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "packaging manifest must be a real file",
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

    def test_rejects_malformed_manifest_before_moving_valid_one(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-entry-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            candidate.mkdir()
            valid = candidate / "Manifest_A.txt"
            invalid = candidate / "Manifest_B.txt"
            valid.write_text("valid\n", encoding="utf-8")
            invalid.mkdir()
            cached = work / "publication-metadata/Manifest_Previous.txt"
            cached.parent.mkdir(parents=True)
            cached.write_text("previous\n", encoding="utf-8")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "packaging manifest must be a real file",
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

            self.assertTrue(valid.is_file())
            self.assertTrue(invalid.is_dir())
            self.assertEqual(
                cached.read_text(encoding="utf-8"),
                "previous\n",
            )

    def test_rejects_pdb_named_directory(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-entry-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            symbol = candidate / "shar/Binaries/Win64/shar.pdb"
            symbol.mkdir(parents=True)

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "debug symbol must be a real file",
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_malformed_pdb_before_moving_valid_one(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-entry-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            valid = candidate / "shar/Binaries/Win64/a.pdb"
            invalid = candidate / "shar/Binaries/Win64/b.pdb"
            valid.parent.mkdir(parents=True)
            valid.write_bytes(b"symbols")
            invalid.mkdir()
            cached = work / "symbols/shar/Binaries/Win64/previous.pdb"
            cached.parent.mkdir(parents=True)
            cached.write_bytes(b"previous")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "debug symbol must be a real file",
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            self.assertTrue(valid.is_file())
            self.assertTrue(invalid.is_dir())
            self.assertEqual(cached.read_bytes(), b"previous")


class ArtifactCachePathTests(unittest.TestCase):
    """Reject redirected metadata and symbol cache roots before moving files."""

    def _assert_linked_cache_rejected(
        self,
        target_id: str,
        cache_name: str,
        source_relative: str,
        label: str,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-cache-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            candidate.mkdir()
            work.mkdir()
            source = candidate / source_relative
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_bytes(b"artifact")
            cache = work / cache_name
            cache.mkdir()
            original = _RUN._is_directory_link

            def report_cache_as_link(path: Path) -> bool:
                return path == cache or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_cache_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    f"{label} must be a real directory",
                ),
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID[target_id],
                )

            self.assertTrue(source.is_file())
            self.assertTrue(cache.is_dir())

    def test_rejects_linked_publication_metadata_cache(self) -> None:
        self._assert_linked_cache_rejected(
            "linux-x64",
            "publication-metadata",
            "Manifest_UFSFiles_Linux.txt",
            "publication metadata cache",
        )

    def test_rejects_linked_symbol_cache(self) -> None:
        self._assert_linked_cache_rejected(
            "windows-x64",
            "symbols",
            "shar/Binaries/Win64/shar.pdb",
            "symbol cache",
        )


class CandidateTreeTests(unittest.TestCase):
    """Require packaged candidates to remain self-contained real trees."""

    def test_rejects_hard_linked_file(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-tree-") as raw:
            candidate = Path(raw) / "candidate"
            candidate.mkdir()
            source = candidate / "runtime.bin"
            source.write_bytes(b"fixture")
            alias = candidate / "runtime-copy.bin"
            alias.hardlink_to(source)

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "candidate package contains a hard-linked file",
            ):
                _RUN._validate_candidate_tree(candidate)

    def test_rejects_nested_link_without_following_it(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-tree-") as raw:
            candidate = Path(raw) / "candidate"
            nested = candidate / "nested"
            nested.mkdir(parents=True)
            linked = nested / "runtime.bin"
            linked.write_bytes(b"fixture")
            original = _RUN._is_directory_link

            def report_runtime_as_link(path: Path) -> bool:
                return path == linked or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_runtime_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "candidate package contains a linked entry",
                ),
            ):
                _RUN._validate_candidate_tree(candidate)

    def test_build_validates_candidate_tree_before_artifact_caching(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-order-") as raw:
            root = Path(raw)
            target = _RUN._TARGETS_BY_ID["linux-x64"]
            with (
                mock.patch.object(_RUN, "_verify_sdk"),
                mock.patch.object(_RUN, "_run_uat"),
                mock.patch.object(
                    _RUN,
                    "_validate_candidate_tree",
                    side_effect=_RUN.RunFailure("candidate tree drift"),
                ),
                mock.patch.object(
                    _RUN,
                    "_cache_nonruntime_artifacts",
                ) as cache_artifacts,
                self.assertRaisesRegex(_RUN.RunFailure, "candidate tree drift"),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project/shar.uproject"),
                    target,
                    validate_only=False,
                )
            cache_artifacts.assert_not_called()


class CandidateArtifactTests(unittest.TestCase):
    """Require each candidate to contain its declared runnable artifact."""

    def test_android_candidate_requires_apk(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-android-candidate-",
        ) as raw:
            candidate = Path(raw)
            apk = candidate / "shar.apk"
            apk.write_bytes(b"not a package")

            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )

            _write_android_apk(apk, machine=0x003E)
            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )

            _write_android_apk(apk)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["android-arm64"],
            )

    def test_ios_candidate_requires_ipa(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-ios-candidate-") as raw:
            candidate = Path(raw)
            ipa = candidate / "shar.ipa"
            ipa.write_bytes(b"not a package")

            with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["ios-arm64"],
                )

            _write_ios_ipa(ipa, cpu=0x01000007)
            with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["ios-arm64"],
                )

            _write_ios_ipa(ipa)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["ios-arm64"],
            )

            _write_ios_ipa(
                ipa,
                binary=_synthetic_fat_macho((
                    0x01000007,
                    _RUN._MACHO_ARM64_CPU,
                )),
            )
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["ios-arm64"],
            )

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_mobile_candidate_rejects_transient_external_archive(self) -> None:
        cases = (
            (
                "android-arm64",
                "shar.apk",
                _write_android_apk,
                "_is_android_apk",
                "Android APK",
            ),
            (
                "ios-arm64",
                "shar.ipa",
                _write_ios_ipa,
                "_is_ios_ipa",
                "iOS IPA",
            ),
        )
        for target_id, name, write_valid, validator_name, label in cases:
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(
                    prefix="shar-mobile-runtime-race-"
                ) as raw,
            ):
                root = Path(raw)
                candidate = root / "candidate"
                candidate.mkdir()
                package = candidate / name
                package.write_bytes(b"invalid local package")
                external = root / f"external-{name}"
                write_valid(external)
                displaced = root / f"displaced-{name}"
                real_validator = getattr(_RUN, validator_name)

                def transient_external(
                    path: Path,
                    *,
                    local_path: Path = package,
                    external_path: Path = external,
                    displaced_path: Path = displaced,
                    validate: Callable[[Path], bool] = real_validator,
                ) -> bool:
                    local_path.replace(displaced_path)
                    local_path.symlink_to(external_path)
                    try:
                        return validate(path)
                    finally:
                        local_path.unlink()
                        displaced_path.replace(local_path)

                with (
                    mock.patch.object(
                        _RUN,
                        validator_name,
                        side_effect=transient_external,
                    ),
                    self.assertRaisesRegex(_RUN.RunFailure, label),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

                self.assertEqual(package.read_bytes(), b"invalid local package")
                self.assertTrue(external.is_file())

    def test_linux_candidate_requires_shar_executable(self) -> None:
        for target_id, platform in (
            ("linux-arm64", "LinuxArm64"),
            ("linux-x64", "Linux"),
        ):
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(
                    prefix="shar-linux-candidate-",
                ) as raw,
            ):
                candidate = Path(raw)
                (candidate / "README.txt").write_text(
                    "not a runtime\n",
                    encoding="utf-8",
                )
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

                binary = (
                    candidate
                    / "shar"
                    / "Binaries"
                    / platform
                    / f"shar-{platform}-Shipping"
                )
                binary.parent.mkdir(parents=True)
                binary.write_bytes(b"not an ELF")
                if _RUN.os.name != "nt":
                    binary.chmod(0o755)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )
                header = bytearray(20)
                header[:4] = b"\x7fELF"
                header[4] = 2
                header[5] = 1
                machine = 0x00B7 if target_id == "linux-arm64" else 0x003E
                header[18:20] = machine.to_bytes(2, "little")
                binary.write_bytes(header)
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID[target_id],
                )

                wrong_machine = 0x003E if machine == 0x00B7 else 0x00B7
                header[18:20] = wrong_machine.to_bytes(2, "little")
                binary.write_bytes(header)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_rejects_runtime_replacement_before_signature_read(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-native-runtime-race-"
        ) as raw:
            root = Path(raw)
            candidate = root / "candidate"
            runtime = (
                candidate
                / "shar/Binaries/Linux/shar-Linux-Shipping"
            )
            runtime.parent.mkdir(parents=True)
            runtime.write_bytes(b"not an ELF runtime")
            runtime.chmod(0o755)
            external = root / "external-runtime"
            external.write_bytes(_synthetic_elf(0x003E))
            external.chmod(0o755)
            displaced = root / "displaced-runtime"
            real_identity = _RUN._real_file_identity
            replaced = False

            def replace_after_identity(
                path: Path, label: str
            ) -> tuple[int, ...]:
                nonlocal replaced
                identity = real_identity(path, label)
                if path == runtime and not replaced:
                    runtime.replace(displaced)
                    runtime.symlink_to(external)
                    replaced = True
                return identity

            with (
                mock.patch.object(
                    _RUN,
                    "_real_file_identity",
                    side_effect=replace_after_identity,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ),
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

            self.assertTrue(runtime.is_symlink())
            self.assertEqual(displaced.read_bytes(), b"not an ELF runtime")
            self.assertEqual(external.read_bytes(), _synthetic_elf(0x003E))

    def test_macos_candidate_requires_shar_app_runtime(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-candidate-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(b"")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

            executable.write_bytes(b"not Mach-O")
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )
            macho = bytearray(bytes.fromhex("cffaedfe"))
            macho.extend(_RUN._MACHO_ARM64_CPU.to_bytes(4, "little"))
            executable.write_bytes(macho)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["macos-arm64"],
            )

            macho[4:8] = (0x01000007).to_bytes(4, "little")
            executable.write_bytes(macho)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

            fat_entry = (
                _RUN._MACHO_ARM64_CPU.to_bytes(4, "big") + (b"\0" * 16)
            )
            truncated_fat = (
                bytes.fromhex("cafebabe")
                + (2).to_bytes(4, "big")
                + fat_entry
            )
            zero_slice_fat = (
                bytes.fromhex("cafebabe")
                + (1).to_bytes(4, "big")
                + fat_entry
            )
            in_bounds_non_macho = (
                bytes.fromhex("cafebabe")
                + (1).to_bytes(4, "big")
                + _RUN._MACHO_ARM64_CPU.to_bytes(4, "big")
                + (b"\0" * 4)
                + (28).to_bytes(4, "big")
                + (8).to_bytes(4, "big")
                + (b"\0" * 4)
                + b"invalid!"
            )
            for malformed in (
                truncated_fat,
                zero_slice_fat,
                in_bounds_non_macho,
            ):
                with (
                    self.subTest(malformed_size=len(malformed)),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "macOS SHAR app bundle",
                    ),
                ):
                    executable.write_bytes(malformed)
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["macos-arm64"],
                    )

            executable.write_bytes(_synthetic_fat_macho((
                0x01000007,
                _RUN._MACHO_ARM64_CPU,
            )))
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["macos-arm64"],
            )

    def test_windows_candidate_requires_shar_executable(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-windows-candidate-",
        ) as raw:
            candidate = Path(raw)
            (candidate / "CrashReportClient.exe").write_bytes(b"engine helper")
            (candidate / "shareware.exe").write_bytes(b"lookalike")
            (candidate / "shar-helper.exe").write_bytes(b"prefixed helper")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            executable = candidate / "shar-Win64-Shipping.exe"
            executable.write_bytes(b"not PE")
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )
            payload = bytearray(0x84)
            payload[:2] = b"MZ"
            payload[0x3C:0x40] = (0x80).to_bytes(4, "little")
            payload[0x80:0x84] = b"PE\0\0"
            payload.extend((0x8664).to_bytes(2, "little"))
            executable.write_bytes(payload)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["windows-x64"],
            )

            payload[0x84:0x86] = (0xAA64).to_bytes(2, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_candidate_rejects_empty_declared_artifact(self) -> None:
        cases = (
            ("android-arm64", "shar.apk", "valid ARM64 Android APK"),
            ("ios-arm64", "shar.ipa", "valid ARM64 iOS IPA"),
            (
                "windows-x64",
                "shar.exe",
                "non-empty Windows SHAR executable",
            ),
        )
        for target_id, filename, label in cases:
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(
                    prefix="shar-empty-mobile-candidate-",
                ) as raw,
            ):
                candidate = Path(raw)
                (candidate / filename).write_bytes(b"")
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    f"no {label}",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

    def test_mobile_artifact_may_be_nested_in_uat_archive(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-mobile-candidate-",
        ) as raw:
            candidate = Path(raw)
            nested = candidate / "package"
            nested.mkdir()
            _write_android_apk(nested / "shar.apk")

            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["android-arm64"],
            )

    def test_build_rejects_wrong_mobile_artifact_before_publication(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-mobile-build-") as raw:
            root = Path(raw)
            target = _RUN._TARGETS_BY_ID["android-arm64"]

            def write_wrong_archive(
                _root: Path,
                _uat: Path,
                arguments: list[str],
                _log: Path,
            ) -> None:
                archive = next(
                    value
                    for value in arguments
                    if value.startswith("-ArchiveDirectory=")
                )
                candidate = Path(archive.split("=", 1)[1])
                (candidate / "not-an-apk.txt").write_text(
                    "wrong artifact\n",
                    encoding="utf-8",
                )

            with (
                mock.patch.object(_RUN, "_verify_sdk"),
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_wrong_archive,
                ),
                mock.patch.object(
                    _RUN,
                    "_cache_nonruntime_artifacts",
                ) as cache_artifacts,
                self.assertRaisesRegex(_RUN.RunFailure, "Android APK"),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project/shar.uproject"),
                    target,
                    validate_only=False,
                )

            cache_artifacts.assert_not_called()
            self.assertFalse((root / "dist/android-arm64").exists())


class ArchitectureRevalidationTests(unittest.TestCase):
    """Require direct runner use to consume stable revalidated evidence."""

    def test_invokes_arch_revalidation_for_exact_saved_path(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-arch-revalidate-") as raw:
            root = Path(raw)
            arch_path = root / ".cache/build/data/arch.json"
            arch_path.parent.mkdir(parents=True)
            arch_path.write_bytes(b"stable architecture evidence")
            result = mock.Mock(returncode=0)

            with mock.patch.object(
                _RUN.subprocess,
                "run",
                return_value=result,
            ) as run:
                snapshot = _RUN._revalidate_arch(root, arch_path)

            self.assertEqual(snapshot, b"stable architecture evidence")
            run.assert_called_once_with(
                [
                    _RUN.sys.executable,
                    str(root / "tools/build/adapter-inbound/arch.py"),
                    "--revalidate",
                    "--output",
                    str(arch_path),
                    "--expected-sha256",
                    hashlib.sha256(
                        b"stable architecture evidence"
                    ).hexdigest(),
                ],
                cwd=root,
                check=False,
            )

    def test_rejects_failed_architecture_revalidation(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-arch-revalidate-") as raw:
            root = Path(raw)
            arch_path = root / ".cache/build/data/arch.json"
            arch_path.parent.mkdir(parents=True)
            arch_path.write_bytes(b"saved architecture evidence")
            result = mock.Mock(returncode=7)

            with (
                mock.patch.object(
                    _RUN.subprocess,
                    "run",
                    return_value=result,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "architecture decision did not revalidate",
                ),
            ):
                _RUN._revalidate_arch(root, arch_path)

    def test_rejects_architecture_drift_during_revalidation(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-arch-revalidate-") as raw:
            root = Path(raw)
            arch_path = root / ".cache/build/data/arch.json"
            arch_path.parent.mkdir(parents=True)
            arch_path.write_bytes(b"before")

            def replace_evidence(
                *_args: object,
                **_kwargs: object,
            ) -> mock.Mock:
                arch_path.write_bytes(b"after")
                return mock.Mock(returncode=0)

            with (
                mock.patch.object(
                    _RUN.subprocess,
                    "run",
                    side_effect=replace_evidence,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "architecture decision changed during revalidation",
                ),
            ):
                _RUN._revalidate_arch(root, arch_path)

    def test_rejects_preflight_drift_during_revalidation(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-check-revalidate-",
        ) as raw:
            root = Path(raw)
            check_path = root / ".cache/build/data/check.json"
            check_path.parent.mkdir(parents=True)
            check_path.write_bytes(b"before")

            def replace_evidence(
                *_args: object,
                **_kwargs: object,
            ) -> mock.Mock:
                check_path.write_bytes(b"after")
                return mock.Mock(returncode=0)

            with (
                mock.patch.object(
                    _RUN.subprocess,
                    "run",
                    side_effect=replace_evidence,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "build preflight changed during revalidation",
                ),
            ):
                _RUN._revalidate_check(root, check_path)

    def test_project_evidence_rejects_descriptor_drift(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-run-project-drift-"
        ) as raw:
            root = Path(raw)
            project = root / _RUN._PROJECT_PATH
            project.parent.mkdir(parents=True)
            project.write_text(
                '{"EngineAssociation":"5.8"}\n',
                encoding="utf-8",
            )
            unreal = {
                "project": str(project),
                "project_sha256": "0" * 64,
            }

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "project descriptor no longer matches preflight",
            ):
                _RUN._project_from_evidence(root, unreal)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_project_evidence_rejects_replacement_during_read(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-run-project-race-"
        ) as raw:
            root = Path(raw)
            project = root / _RUN._PROJECT_PATH
            project.parent.mkdir(parents=True)
            payload = b'{"EngineAssociation":"5.8"}\n'
            project.write_bytes(payload)
            external = root / "external.uproject"
            external.write_bytes(payload)
            displaced = root / "displaced.uproject"
            unreal = {
                "project": str(project),
                "project_sha256": hashlib.sha256(payload).hexdigest(),
            }
            real_identity = _RUN._real_file_identity
            replaced = False

            def replace_after_identity(
                path: Path, label: str
            ) -> tuple[int, ...]:
                nonlocal replaced
                identity = real_identity(path, label)
                if path == project and not replaced:
                    project.replace(displaced)
                    project.symlink_to(external)
                    replaced = True
                return identity

            with (
                mock.patch.object(
                    _RUN,
                    "_real_file_identity",
                    side_effect=replace_after_identity,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Unreal project descriptor changed while reading",
                ),
            ):
                _RUN._project_from_evidence(root, unreal)

            self.assertTrue(project.is_symlink())
            self.assertEqual(displaced.read_bytes(), payload)
            self.assertEqual(external.read_bytes(), payload)

    def test_project_evidence_accepts_current_canonical_descriptor(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-run-project-current-"
        ) as raw:
            root = Path(raw)
            project = root / _RUN._PROJECT_PATH
            project.parent.mkdir(parents=True)
            payload = b'{"EngineAssociation":"5.8"}\n'
            project.write_bytes(payload)
            unreal = {
                "project": str(project),
                "project_sha256": hashlib.sha256(payload).hexdigest(),
            }

            self.assertEqual(
                _RUN._project_from_evidence(root, unreal),
                project,
            )

    def test_main_consumes_revalidated_snapshots(self) -> None:
        root = Path("/repo")
        arch_snapshot = b"validated arch"
        check_snapshot = b"validated check"
        unreal = {
            "project": "/repo/project/shar.uproject",
            "project_sha256": "a" * 64,
            "root": "/engine",
            "version": "5.8.1",
        }
        with (
            mock.patch.object(_RUN, "_root", return_value=root),
            mock.patch.object(
                _RUN,
                "_revalidate_arch",
                return_value=arch_snapshot,
            ) as revalidate_arch,
            mock.patch.object(
                _RUN,
                "_selected_targets",
                return_value=[],
            ) as selected_targets,
            mock.patch.object(
                _RUN,
                "_revalidate_check",
                return_value=check_snapshot,
            ) as revalidate_check,
            mock.patch.object(
                _RUN,
                "_check_evidence",
                return_value={"unreal": unreal},
            ) as check_evidence,
            mock.patch.object(
                _RUN,
                "_require_unreal_evidence",
                return_value=unreal,
            ),
            mock.patch.object(
                _RUN,
                "_project_from_evidence",
                return_value=Path("/repo/project/shar.uproject"),
            ),
            mock.patch.object(_RUN, "_prepare_project_state"),
            mock.patch.object(_RUN, "_uat_path", return_value=Path("/uat")),
            mock.patch.object(_RUN.sys, "argv", ["run.py", "--validate-only"]),
        ):
            self.assertEqual(_RUN.main(), 0)

        arch_path = root / _RUN._ARCH_PATH
        check_path = root / _RUN._CHECK_PATH
        revalidate_arch.assert_called_once_with(root, arch_path)
        selected_targets.assert_called_once_with(arch_snapshot)
        revalidate_check.assert_called_once_with(root, check_path)
        check_evidence.assert_called_once_with(check_snapshot)


if __name__ == "__main__":
    unittest.main()

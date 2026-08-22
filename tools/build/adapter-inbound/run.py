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
#   - Revalidation, SDK gating, native packaging, and final build publication.
# - Must-Not:
#   - Auto-install SDKs, edit source decisions, or publish partial packages.
# - Allows:
#   - Inputs: saved check/architecture evidence and Unreal AutomationTool.
#   - Outputs: target build logs, reports, and transactional dist directories.
#   - Side effects: Unreal build work under .cache and atomic dist replacement.
# - Split-When:
#   - Split when one platform needs an independent packaging lifecycle.
# - Merge-When:
#   - Merge when orchestration owns identical target execution behavior.
# - Summary:
#   - Builds selected SHAR targets through native Unreal tooling.
# - Description:
#   - Revalidates every saved decision before Turnkey and BuildCookRun execute.
# - Usage:
#   - Run tools/build/adapter-inbound/run.py after dependencies.py, check.py,
#     and arch.py.
# - Defaults:
#   - Builds every selected target in Shipping configuration.
#

"""Build selected SHAR targets and publish only complete native packages."""

from __future__ import annotations

import argparse
from collections.abc import Callable
from collections.abc import Sequence
import hashlib
import json
import os
from pathlib import Path
import plistlib
import shutil
import subprocess
import sys
from typing import NamedTuple
from typing import TextIO
import zipfile

_ARCH_SCHEMA = "shar.build.arch.v1"
_CHECK_SCHEMA = "shar.build.check.v1"
_ARCH_PATH = Path(".cache/build/data/arch.json")
_CHECK_PATH = Path(".cache/build/data/check.json")
_PROJECT_PATH = Path("src/unreal/project/composition/uproject/shar.uproject")
_WORK_ROOT = Path(".cache/build/run")
_PROJECT_STATE_ROOT = Path(".cache/build/project-state")
_PROJECT_STATE_NAMES = ("Binaries", "DerivedDataCache", "Intermediate", "Saved")
_DIST_ROOT = Path("dist")


class RunFailure(RuntimeError):
    """One actionable build-runner failure."""


class Target(NamedTuple):
    """One admitted architecture-to-Unreal packaging projection."""

    identifier: str
    system: str
    architecture: str
    artifact: str
    unreal_platform: str
    unreal_architecture: str


class _CandidateTree(NamedTuple):
    """One strict candidate traversal split by regular files/directories."""

    files: tuple[Path, ...]
    directories: tuple[Path, ...]


class _CapturedArtifact(NamedTuple):
    """One stable diagnostic source snapshot bound to its file identity."""

    source: Path
    payload: bytes
    identity: tuple[int, ...]


_TARGETS = (
    Target("android-arm64", "android", "arm64", "apk", "Android", "arm64"),
    Target("ios-arm64", "ios", "arm64", "ipa", "IOS", "arm64"),
    Target("linux-arm64", "linux", "arm64", "native", "LinuxArm64", "arm64"),
    Target("linux-x64", "linux", "amd64", "native", "Linux", "x64"),
    Target("macos-arm64", "macos", "arm64", "native", "Mac", "arm64"),
    Target("windows-arm64", "windows", "arm64", "native", "Win64", "arm64"),
    Target("windows-x64", "windows", "amd64", "native", "Win64", "x64"),
)
_TARGETS_BY_ID = {target.identifier: target for target in _TARGETS}


def _root() -> Path:
    """Return the repository root from this tracked script location."""
    return Path(__file__).resolve().parents[3]


def _unique_json_object(
    pairs: list[tuple[str, object]],
) -> dict[str, object]:
    """Reject duplicate keys at every JSON object depth."""
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise RunFailure(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _object_from_bytes(data: bytes, label: str) -> dict[str, object]:
    """Decode one required UTF-8 JSON object from a stable byte snapshot."""
    try:
        value = json.loads(
            data.decode("utf-8"),
            object_pairs_hook=_unique_json_object,
        )
    except (UnicodeError, json.JSONDecodeError) as error:
        raise RunFailure(f"cannot decode {label}") from error
    if not isinstance(value, dict):
        raise RunFailure(f"{label} must contain a JSON object")
    return value


def _require_keys(
    value: dict[str, object],
    required: set[str],
    label: str,
) -> None:
    """Require an exact JSON object key set."""
    actual = set(value)
    if actual != required:
        missing = sorted(required - actual)
        extra = sorted(actual - required)
        raise RunFailure(
            f"{label} keys are invalid; missing={missing}, extra={extra}"
        )


def _target_from_json(value: object) -> Target:
    """Validate one saved architecture target against canonical policy."""
    if not isinstance(value, dict):
        raise RunFailure("architecture target must be a JSON object")
    _require_keys(
        value,
        {"architecture", "artifact", "id", "system"},
        "architecture target",
    )
    identifier = value.get("id")
    if not isinstance(identifier, str) or identifier not in _TARGETS_BY_ID:
        raise RunFailure(f"unsupported architecture target: {identifier!r}")
    target = _TARGETS_BY_ID[identifier]
    expected = {
        "architecture": target.architecture,
        "artifact": target.artifact,
        "id": target.identifier,
        "system": target.system,
    }
    if value != expected:
        message = f"architecture target drifted from policy: {identifier}"
        raise RunFailure(message)
    return target


def _selected_targets(snapshot: bytes) -> list[Target]:
    """Validate a captured architecture decision and return its targets."""
    value = _object_from_bytes(snapshot, "architecture evidence")
    _require_keys(value, {"host", "schema", "targets"}, "architecture evidence")
    if value.get("schema") != _ARCH_SCHEMA:
        raise RunFailure(f"architecture schema must be {_ARCH_SCHEMA}")
    raw_targets = value.get("targets")
    if not isinstance(raw_targets, list) or not raw_targets:
        message = "architecture evidence must select at least one target"
        raise RunFailure(message)
    targets = [_target_from_json(raw) for raw in raw_targets]
    identifiers = [target.identifier for target in targets]
    if len(set(identifiers)) != len(identifiers):
        raise RunFailure("architecture evidence contains duplicate targets")
    canonical = [
        target.identifier
        for target in _TARGETS
        if target.identifier in set(identifiers)
    ]
    if identifiers != canonical:
        raise RunFailure("architecture targets are not in canonical order")
    return targets


def _check_evidence(snapshot: bytes) -> dict[str, object]:
    """Load a captured preflight snapshot after child revalidation."""
    value = _object_from_bytes(snapshot, "check evidence")
    if value.get("schema") != _CHECK_SCHEMA:
        raise RunFailure(f"check schema must be {_CHECK_SCHEMA}")
    unreal = value.get("unreal")
    if not isinstance(unreal, dict):
        raise RunFailure("check evidence has no unreal object")
    for key in ("project", "project_sha256", "root", "version"):
        if not isinstance(unreal.get(key), str) or not unreal.get(key):
            raise RunFailure(f"check evidence has invalid unreal.{key}")
    project_sha256 = str(unreal["project_sha256"])
    if (
        len(project_sha256) != 64
        or project_sha256 != project_sha256.casefold()
        or any(
            character not in "0123456789abcdef"
            for character in project_sha256
        )
    ):
        raise RunFailure("check evidence has invalid unreal.project_sha256")
    if unreal.get("version") != "5.8.1":
        raise RunFailure("check evidence must target Unreal Engine 5.8.1")
    return value


def _project_from_evidence(
    root: Path,
    unreal: dict[str, object],
) -> Path:
    """Require the current canonical project to match saved preflight bytes."""
    expected_path = root / _PROJECT_PATH
    saved_path = Path(str(unreal["project"]))
    if saved_path != expected_path:
        raise RunFailure("check evidence project path is not canonical")
    for path, label in (
        (root / "src", "source root"),
        (root / "src/unreal", "Unreal source root"),
        (root / "src/unreal/project", "Unreal project source root"),
        (
            root / "src/unreal/project/composition",
            "Unreal project composition root",
        ),
        (expected_path.parent, "Unreal project root"),
    ):
        _require_real_directory(path, label)
    snapshot = _read_real_bytes(
        expected_path,
        "Unreal project descriptor",
    )
    actual = hashlib.sha256(snapshot).hexdigest()
    if actual != unreal["project_sha256"]:
        raise RunFailure(
            "Unreal project descriptor no longer matches preflight"
        )
    return expected_path


def _revalidate_snapshot(
    root: Path,
    path: Path,
    command: list[str],
    label: str,
) -> bytes:
    """Return the exact saved bytes validated by one child process."""
    before = _read_real_bytes(path, f"saved {label}")
    expected_sha256 = hashlib.sha256(before).hexdigest()
    result = subprocess.run(
        [*command, "--expected-sha256", expected_sha256],
        cwd=root,
        check=False,
    )
    if result.returncode:
        raise RunFailure(f"saved {label} did not revalidate")
    after = _read_real_bytes(path, f"saved {label}")
    if after != before:
        raise RunFailure(f"saved {label} changed during revalidation")
    return after


def _revalidate_arch(root: Path, arch_path: Path) -> bytes:
    """Invoke canonical arch.py and return its stable validated snapshot."""
    command = [
        sys.executable,
        str(root / "tools" / "build" / "adapter-inbound" / "arch.py"),
        "--revalidate",
        "--output",
        str(arch_path),
    ]
    return _revalidate_snapshot(
        root,
        arch_path,
        command,
        "architecture decision",
    )


def _revalidate_check(root: Path, check_path: Path) -> bytes:
    """Invoke check.py and return its stable validated snapshot."""
    command = [
        sys.executable,
        str(root / "tools" / "build" / "adapter-inbound" / "check.py"),
        "--revalidate",
        "--output",
        str(check_path),
    ]
    return _revalidate_snapshot(root, check_path, command, "build preflight")


class _ProjectStateAction(NamedTuple):
    """One reversible project-state migration mutation."""

    link: Path
    canonical: Path
    source_was_directory: bool
    canonical_was_present: bool


def _is_directory_link(path: Path) -> bool:
    """Return whether path is a symbolic directory link or Windows junction."""
    return path.is_symlink() or os.path.isjunction(path)


def _path_present(path: Path) -> bool:
    """Return whether a filesystem identity exists, including broken links."""
    return os.path.lexists(path)


def _require_real_directory(path: Path, label: str) -> None:
    """Require one existing directory that is not a link or junction."""
    if not path.is_dir() or _is_directory_link(path):
        raise RunFailure(f"{label} must be a real directory: {path}")


def _require_real_file(path: Path, label: str) -> None:
    """Require one unshared regular file that is not a symbolic link."""
    if not path.is_file() or path.is_symlink():
        raise RunFailure(f"{label} must be a real file: {path}")
    if path.stat(follow_symlinks=False).st_nlink != 1:
        raise RunFailure(f"{label} must have one filesystem link: {path}")


def _file_identity(metadata: os.stat_result) -> tuple[int, ...]:
    """Project one regular file to stable local identity fields."""
    return (
        metadata.st_dev,
        metadata.st_ino,
        metadata.st_mode,
        metadata.st_mtime_ns,
        metadata.st_ctime_ns,
        metadata.st_size,
        metadata.st_nlink,
    )


def _real_file_identity(path: Path, label: str) -> tuple[int, ...]:
    """Return the stable identity of one required real single-link file."""
    _require_real_file(path, label)
    return _file_identity(path.stat(follow_symlinks=False))


def _require_open_uat_log_identity(
    log: Path,
    handle: TextIO,
    expected: tuple[int, ...] | None,
) -> None:
    """Require an opened UAT log to match its repository path identity."""
    opened = _file_identity(os.fstat(handle.fileno()))
    if expected is not None and opened != expected:
        raise RunFailure(f"UAT log changed before opening: {log}")
    if _real_file_identity(log, "UAT log") != opened:
        raise RunFailure(f"UAT log changed before opening: {log}")


def _open_uat_log(log: Path) -> TextIO:
    """Open one UAT log without truncating a substituted filesystem identity."""
    existed = _path_present(log)
    expected = _real_file_identity(log, "UAT log") if existed else None
    mode = "r+" if existed else "x+"
    try:
        handle = log.open(mode, encoding="utf-8", newline="\n")
    except FileExistsError as error:
        raise RunFailure(f"UAT log appeared before creation: {log}") from error
    try:
        _require_open_uat_log_identity(log, handle, expected)
    except BaseException:
        handle.close()
        raise
    handle.seek(0)
    handle.truncate(0)
    return handle


def _capture_real_bytes(
    path: Path,
    label: str,
) -> tuple[bytes, tuple[int, ...]]:
    """Capture bytes and identity from one stable real single-link file."""
    expected = _real_file_identity(path, label)
    try:
        with path.open("rb") as handle:
            opened = _file_identity(os.fstat(handle.fileno()))
            if opened != expected:
                raise RunFailure(f"{label} changed while reading: {path}")
            payload = handle.read()
            finished = _file_identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise RunFailure(f"cannot read {label}: {path}") from error
    if finished != expected or len(payload) != expected[5]:
        raise RunFailure(f"{label} changed while reading: {path}")
    try:
        current = _real_file_identity(path, label)
    except RunFailure as error:
        raise RunFailure(f"{label} changed while reading: {path}") from error
    if current != expected:
        raise RunFailure(f"{label} changed while reading: {path}")
    return payload, expected


def _read_real_bytes(path: Path, label: str) -> bytes:
    """Read bytes from one stable real single-link file identity."""
    return _capture_real_bytes(path, label)[0]


def _read_real_text(path: Path, label: str) -> str:
    """Decode UTF-8 from one stable real single-link file identity."""
    try:
        return _read_real_bytes(path, label).decode("utf-8")
    except UnicodeError as error:
        raise RunFailure(f"cannot read {label}: {path}") from error


def _ensure_real_directory(path: Path, label: str) -> None:
    """Create one directory or require an existing real directory."""
    if _path_present(path):
        _require_real_directory(path, label)
        return
    path.mkdir()


def _remove_real_directory_if_present(path: Path, label: str) -> None:
    """Remove one stale repository directory without following links."""
    if not _path_present(path):
        return
    _require_real_directory(path, label)
    shutil.rmtree(path)


def _reset_real_directory(path: Path, label: str) -> None:
    """Replace one repository scratch directory without following links."""
    if _path_present(path):
        _require_real_directory(path, label)
        shutil.rmtree(path)
    path.mkdir()


def _ensure_build_cache_root(root: Path) -> Path:
    """Create or validate canonical repository build-cache ancestors."""
    cache_root = root / ".cache"
    _ensure_real_directory(cache_root, "repository cache root")
    build_root = cache_root / "build"
    _ensure_real_directory(build_root, "build cache root")
    return build_root


def _preflight_project_state(project_dir: Path, state_root: Path) -> None:
    """Reject conflicting or malformed project build-state identities."""
    for name in _PROJECT_STATE_NAMES:
        link = project_dir / name
        canonical = state_root / name
        link_present = _path_present(link)
        canonical_present = _path_present(canonical)
        if canonical_present:
            _require_real_directory(canonical, f"canonical project {name}")
        if _is_directory_link(link):
            if not canonical_present:
                raise RunFailure(
                    f"project {name} link has no canonical cache directory"
                )
            if link.resolve() != canonical.resolve():
                raise RunFailure(
                    f"project {name} link does not target canonical cache"
                )
            continue
        if link_present:
            _require_real_directory(link, f"legacy project {name}")
            if canonical_present:
                raise RunFailure(
                    f"legacy and canonical project {name} both exist"
                )


def _create_directory_link(link: Path, target: Path) -> None:
    """Create the host-native directory indirection used by Unreal."""
    if os.name == "nt":
        result = subprocess.run(
            ["cmd.exe", "/d", "/c", "mklink", "/J", str(link), str(target)],
            check=False,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        if result.returncode:
            raise RunFailure(f"cannot create project-state junction: {link}")
        return
    try:
        link.symlink_to(target, target_is_directory=True)
    except OSError as error:
        raise RunFailure(f"cannot create project-state link: {link}") from error


def _remove_directory_link(path: Path) -> None:
    """Remove one link or junction without deleting its target."""
    if os.path.isjunction(path):
        Path(path).rmdir()
    else:
        path.unlink()


def _adopt_project_state_path(
    project_dir: Path,
    state_root: Path,
    name: str,
) -> _ProjectStateAction | None:
    """Move or attach one project build-state root to canonical cache."""
    link = project_dir / name
    canonical = state_root / name
    if _is_directory_link(link):
        return None
    source_was_directory = _path_present(link)
    canonical_was_present = _path_present(canonical)
    if source_was_directory:
        Path(link).replace(canonical)
    elif not canonical_was_present:
        canonical.mkdir()
    try:
        _create_directory_link(link, canonical.resolve())
    except OSError, RunFailure:
        if source_was_directory:
            Path(canonical).replace(link)
        elif not canonical_was_present and canonical.exists():
            canonical.rmdir()
        raise
    return _ProjectStateAction(
        link,
        canonical,
        source_was_directory,
        canonical_was_present,
    )


def _rollback_project_state(actions: list[_ProjectStateAction]) -> None:
    """Restore project-state identities after a partial migration failure."""
    failures: list[str] = []
    for action in reversed(actions):
        try:
            if _is_directory_link(action.link):
                _remove_directory_link(action.link)
            if action.source_was_directory:
                Path(action.canonical).replace(action.link)
            elif not action.canonical_was_present:
                action.canonical.rmdir()
        except OSError as error:
            failures.append(f"{action.link.name}:{error.__class__.__name__}")
    if failures:
        raise RunFailure(
            "project-state migration rollback failed: " + ", ".join(failures)
        )


def _prepare_project_state(root: Path, project: Path) -> Path:
    """Keep Unreal project-generated state physically below repository cache."""
    project_dir = project.parent
    _ensure_build_cache_root(root)
    state_root = root / _PROJECT_STATE_ROOT
    _ensure_real_directory(state_root, "project-state cache root")
    _preflight_project_state(project_dir, state_root)
    actions: list[_ProjectStateAction] = []
    try:
        for name in _PROJECT_STATE_NAMES:
            action = _adopt_project_state_path(project_dir, state_root, name)
            if action is not None:
                actions.append(action)
    except (OSError, RunFailure) as error:
        try:
            _rollback_project_state(actions)
        except RunFailure as rollback:
            raise RunFailure(f"{error}; {rollback}") from error
        if isinstance(error, RunFailure):
            raise
        error_name = error.__class__.__name__
        raise RunFailure(
            f"cannot migrate Unreal project build state: {error_name}"
        ) from error
    return state_root


def _require_real_descendant_parents(
    root: Path,
    path: Path,
    label: str,
) -> None:
    """Require a real root and every internal parent of one trusted file."""
    _require_real_directory(root, "Unreal Engine root")
    try:
        relative = path.relative_to(root)
    except ValueError as error:
        message = f"{label} must remain inside the Unreal Engine root"
        raise RunFailure(message) from error
    current = root
    for component in relative.parts[:-1]:
        current /= component
        _require_real_directory(current, f"{label} parent")


def _uat_path(engine_root: Path) -> Path:
    """Resolve one real host-native RunUAT launcher."""
    batch = engine_root / "Engine" / "Build" / "BatchFiles"
    path = batch / "RunUAT.bat" if os.name == "nt" else batch / "RunUAT.sh"
    _require_real_descendant_parents(
        engine_root,
        path,
        "Unreal AutomationTool launcher",
    )
    _require_real_file(path, "Unreal AutomationTool launcher")
    if os.name != "nt" and not os.access(path, os.X_OK):
        raise RunFailure(
            f"Unreal AutomationTool launcher is not executable: {path}"
        )
    return path


def _uat_command(uat: Path, arguments: list[str]) -> list[str]:
    """Build the direct native RunUAT process argument vector."""
    return [str(uat), *arguments]


def _run_uat(
    root: Path,
    uat: Path,
    arguments: list[str],
    log: Path,
) -> None:
    """Run one bounded UAT command and persist its complete output."""
    work = log.parent
    _ensure_real_directory(work, "UAT work root")
    command = _uat_command(uat, arguments)
    automation_saved = work / "automation-saved"
    _ensure_real_directory(automation_saved, "UAT saved root")
    automation_logs = automation_saved / "logs"
    _ensure_real_directory(automation_logs, "UAT log root")
    ddc = work / "ddc"
    _ensure_real_directory(ddc, "UAT DDC root")
    environment = os.environ.copy()
    environment["uebp_EngineSavedFolder"] = str(automation_saved)
    environment["uebp_FinalLogFolder"] = str(automation_logs)
    environment["uebp_LogFolder"] = str(automation_logs)
    environment["UE-LocalDataCachePath"] = str(ddc)
    with _open_uat_log(log) as handle:
        result = subprocess.run(
            command,
            cwd=root,
            env=environment,
            check=False,
            stdout=handle,
            stderr=subprocess.STDOUT,
            text=True,
        )
    if result.returncode:
        raise RunFailure(
            f"Unreal AutomationTool failed with {result.returncode}; see {log}"
        )


def _verify_sdk(
    root: Path,
    uat: Path,
    project: Path,
    target: Target,
    work: Path,
) -> Path:
    """Require Turnkey to report a valid SDK without installing anything."""
    report = work / "turnkey.txt"
    log = work / "turnkey.log"
    if _path_present(report):
        _require_real_file(report, "Turnkey SDK report")
        report.unlink()
    arguments = [
        "Turnkey",
        "-Command=VerifySdk",
        f"-Platform={target.unreal_platform}",
        "-Unattended",
        f"-ReportFilename={report}",
        f"-Project={project}",
    ]
    _run_uat(root, uat, arguments, log)
    if not _path_present(report):
        raise RunFailure(f"Turnkey did not produce an SDK report: {report}")
    text = _read_real_text(report, "Turnkey SDK report")
    expected = f"{target.unreal_platform}: (Status=Valid,"
    if not any(line.strip().startswith(expected) for line in text.splitlines()):
        raise RunFailure(
            f"Turnkey SDK is invalid for {target.identifier}; see {report}"
        )
    return report


def _build_arguments(
    project: Path,
    target: Target,
    candidate: Path,
    staging: Path,
) -> list[str]:
    """Return the reviewed BuildCookRun argument vector for one target."""
    return [
        "BuildCookRun",
        f"-Project={project}",
        "-NoP4",
        "-UTF8Output",
        "-Build",
        "-Cook",
        "-Stage",
        "-Package",
        "-Archive",
        f"-ArchiveDirectory={candidate}",
        f"-StagingDirectory={staging}",
        "-ClientConfig=Shipping",
        f"-TargetPlatform={target.unreal_platform}",
        f"-SpecifiedArchitecture={target.unreal_architecture}",
        "-Pak",
        "-IoStore",
    ]


def _validate_candidate_tree(candidate: Path) -> _CandidateTree:
    """Return one strict, link-free inventory of the candidate tree."""
    _require_real_directory(candidate, "candidate package")
    files: list[Path] = []
    directories: list[Path] = []
    pending = [candidate]
    while pending:
        directory = pending.pop()
        try:
            entries = sorted(directory.iterdir(), key=lambda path: path.name)
        except OSError as error:
            raise RunFailure(
                f"candidate package could not be scanned: {directory}"
            ) from error
        for item in entries:
            try:
                linked = _is_directory_link(item)
                is_directory = item.is_dir()
                is_file = item.is_file()
                metadata = item.stat(follow_symlinks=False) if is_file else None
            except OSError as error:
                raise RunFailure(
                    f"candidate package entry could not be inspected: {item}"
                ) from error
            if linked:
                raise RunFailure(
                    f"candidate package contains a linked entry: {item}"
                )
            if is_directory:
                directories.append(item)
                pending.append(item)
                continue
            if is_file:
                if metadata is not None and metadata.st_nlink > 1:
                    raise RunFailure(
                        f"candidate package contains a hard-linked file: {item}"
                    )
                files.append(item)
                continue
            raise RunFailure(
                f"candidate package contains a special entry: {item}"
            )
    return _CandidateTree(tuple(files), tuple(directories))


def _is_shar_runtime_name(name: str) -> bool:
    """Return whether one native filename identifies the SHAR runtime."""
    normalized = name.casefold()
    return normalized == "shar" or (
        normalized.startswith("shar-")
        and normalized.endswith("-shipping")
    )


_ELF_MACHINES = {"amd64": 0x003E, "arm64": 0x00B7}
_PE_MACHINES = {"amd64": 0x8664, "arm64": 0xAA64}
_MACHO_ARM64_CPU = 0x0100000C
_MACHO_THIN_ENDIAN = {
    bytes.fromhex("cffaedfe"): "little",
    bytes.fromhex("feedfacf"): "big",
}
_MACHO_FAT = {
    bytes.fromhex("cafebabe"): ("big", 20),
    bytes.fromhex("cafebabf"): ("big", 32),
    bytes.fromhex("bebafeca"): ("little", 20),
    bytes.fromhex("bfbafeca"): ("little", 32),
}


def _elf_program_layout(
    header: bytes,
    byte_order: str,
    expected: int,
    file_size: int,
) -> tuple[int, int, int] | None:
    """Return one bounded ELF64 program-table layout for a runnable image."""
    image_type = int.from_bytes(header[16:18], byte_order)
    machine = int.from_bytes(header[18:20], byte_order)
    version = int.from_bytes(header[20:24], byte_order)
    if image_type not in {2, 3} or machine != expected or version != 1:
        return None
    program_offset = int.from_bytes(header[32:40], byte_order)
    header_size = int.from_bytes(header[52:54], byte_order)
    program_size = int.from_bytes(header[54:56], byte_order)
    program_count = int.from_bytes(header[56:58], byte_order)
    if header_size != 64 or program_size != 56 or program_count == 0:
        return None
    if program_offset < header_size or program_offset > file_size:
        return None
    if program_size * program_count > file_size - program_offset:
        return None
    return program_offset, program_size, program_count


def _matches_elf(
    stream: object,
    prefix: bytes,
    architecture: str,
    file_size: int,
) -> bool:
    """Return whether one loadable ELF64 image declares the selected CPU."""
    header = prefix + stream.read(60)
    if len(header) != 64 or header[4] != 2 or header[6] != 1:
        return False
    byte_order = {1: "little", 2: "big"}.get(header[5])
    expected = _ELF_MACHINES.get(architecture)
    if byte_order is None or expected is None:
        return False
    layout = _elf_program_layout(header, byte_order, expected, file_size)
    if layout is None:
        return False
    program_offset, program_size, program_count = layout
    try:
        stream.seek(program_offset)
    except (OSError, ValueError):
        return False
    loadable = False
    for _ in range(program_count):
        program = stream.read(program_size)
        if len(program) != program_size:
            return False
        loadable = loadable or int.from_bytes(program[:4], byte_order) == 1
    return loadable


def _matches_pe(stream: object, prefix: bytes, architecture: str) -> bool:
    """Return whether one PE32+ image declares the selected architecture."""
    if prefix[:2] != b"MZ":
        return False
    stream.seek(0x3C)
    offset_bytes = stream.read(4)
    if len(offset_bytes) != 4:
        return False
    stream.seek(int.from_bytes(offset_bytes, "little"))
    signature = stream.read(4)
    coff = stream.read(20)
    if signature != b"PE\0\0" or len(coff) != 20:
        return False
    expected = _PE_MACHINES.get(architecture)
    machine = int.from_bytes(coff[:2], "little")
    optional_size = int.from_bytes(coff[16:18], "little")
    if expected is None or machine != expected or optional_size < 2:
        return False
    optional_magic = stream.read(2)
    return optional_magic == bytes.fromhex("0b02")


def _matches_thin_macho(stream: object, byte_order: str) -> bool:
    """Return whether one thin ARM64 Mach-O header is executable."""
    header = stream.read(12)
    if len(header) != 12:
        return False
    cpu = int.from_bytes(header[:4], byte_order)
    file_type = int.from_bytes(header[8:12], byte_order)
    return cpu == _MACHO_ARM64_CPU and file_type == 2


def _fat_macho_slice_bounds(
    payload: bytes,
    byte_order: str,
    entry_size: int,
    table_end: int,
    file_size: int,
) -> tuple[int, int] | None:
    """Return one structurally bounded fat Mach-O slice range."""
    offset_width = 8 if entry_size == 32 else 4
    offset_start = 4
    size_start = offset_start + offset_width
    offset = int.from_bytes(
        payload[offset_start:size_start],
        byte_order,
    )
    size = int.from_bytes(
        payload[size_start : size_start + offset_width],
        byte_order,
    )
    if (
        size == 0
        or offset < table_end
        or offset > file_size
        or size > file_size - offset
    ):
        return None
    return offset, size


def _fat_macho_arm64_slice_is_native(stream: object, offset: int) -> bool:
    """Return whether one fat ARM64 slice begins with a matching thin header."""
    try:
        stream.seek(offset)
    except (OSError, ValueError):
        return False
    prefix = stream.read(4)
    thin_order = _MACHO_THIN_ENDIAN.get(prefix)
    return thin_order is not None and _matches_thin_macho(stream, thin_order)


def _fat_macho_contains_arm64(
    stream: object,
    byte_order: str,
    entry_size: int,
    file_size: int,
) -> bool:
    """Return whether one bounded universal Mach-O contains an ARM64 slice."""
    count_bytes = stream.read(4)
    if len(count_bytes) != 4:
        return False
    count = int.from_bytes(count_bytes, byte_order)
    if count == 0 or count > 64:
        return False
    table_end = 8 + (count * entry_size)
    arm64_offsets: list[int] = []
    for _ in range(count):
        cpu = stream.read(4)
        rest = stream.read(entry_size - 4)
        if len(cpu) != 4 or len(rest) != entry_size - 4:
            return False
        bounds = _fat_macho_slice_bounds(
            rest,
            byte_order,
            entry_size,
            table_end,
            file_size,
        )
        if bounds is None:
            return False
        offset, size = bounds
        if int.from_bytes(cpu, byte_order) == _MACHO_ARM64_CPU:
            if size < 16:
                return False
            arm64_offsets.append(offset)
    return bool(arm64_offsets) and all(
        _fat_macho_arm64_slice_is_native(stream, offset)
        for offset in arm64_offsets
    )


def _matches_macho(
    stream: object,
    prefix: bytes,
    architecture: str,
    file_size: int,
) -> bool:
    """Return whether one Mach-O header contains the selected architecture."""
    if architecture != "arm64":
        return False
    thin_order = _MACHO_THIN_ENDIAN.get(prefix)
    if thin_order is not None:
        return _matches_thin_macho(stream, thin_order)
    fat = _MACHO_FAT.get(prefix)
    if fat is None:
        return False
    byte_order, entry_size = fat
    return _fat_macho_contains_arm64(
        stream,
        byte_order,
        entry_size,
        file_size,
    )


def _matches_native_binary_stream(
    stream: object,
    system: str,
    architecture: str,
    file_size: int,
) -> bool:
    """Return whether one opened stream matches its declared native target."""
    prefix = stream.read(4)
    if system == "linux" and prefix == b"\x7fELF":
        return _matches_elf(stream, prefix, architecture, file_size)
    if system == "macos":
        return _matches_macho(
            stream,
            prefix,
            architecture,
            file_size,
        )
    if system == "windows":
        return _matches_pe(stream, prefix, architecture)
    return False


def _has_native_binary_signature(
    path: Path,
    system: str,
    architecture: str,
) -> bool:
    """Validate one native binary through a stable local file identity."""
    label = "candidate runtime"
    try:
        expected = _real_file_identity(path, label)
        if expected[5] == 0:
            return False
        permission_ok = (
            system == "windows"
            or os.name == "nt"
            or bool(expected[2] & 0o111)
        )
        if not permission_ok:
            return False
        with path.open("rb") as stream:
            opened = _file_identity(os.fstat(stream.fileno()))
            if opened != expected:
                return False
            matches = _matches_native_binary_stream(
                stream,
                system,
                architecture,
                expected[5],
            )
            finished = _file_identity(os.fstat(stream.fileno()))
        if finished != expected:
            return False
        return matches and _real_file_identity(path, label) == expected
    except (OSError, RunFailure):
        return False


def _zip_inventory(
    archive: zipfile.ZipFile,
) -> dict[str, zipfile.ZipInfo] | None:
    """Return a duplicate-free ZIP member inventory."""
    infos = archive.infolist()
    names = [info.filename for info in infos]
    if len(names) != len(set(names)):
        return None
    return {info.filename: info for info in infos}


def _zip_member_matches_elf(
    archive: zipfile.ZipFile,
    info: zipfile.ZipInfo,
    architecture: str,
) -> bool:
    """Return whether one ZIP member is an ELF for the selected CPU."""
    if info.is_dir():
        return False
    with archive.open(info) as stream:
        prefix = stream.read(4)
        return prefix == b"\x7fELF" and _matches_elf(
            stream,
            prefix,
            architecture,
            info.file_size,
        )


def _is_android_arm64_library_path(name: str) -> bool:
    """Return whether one APK member uses Android's ARM64 library shape."""
    parts = name.split("/")
    if len(parts) != 3 or parts[:2] != ["lib", "arm64-v8a"]:
        return False
    library = parts[2]
    return (
        library.startswith("lib")
        and library.endswith(".so")
        and len(library) > len("lib.so")
    )


def _android_archive_contains_arm64(archive: zipfile.ZipFile) -> bool:
    """Return whether one opened APK contains an ARM64 native library."""
    inventory = _zip_inventory(archive)
    if inventory is None:
        return False
    return any(
        _is_android_arm64_library_path(name)
        and _zip_member_matches_elf(archive, info, "arm64")
        for name, info in inventory.items()
    )


def _stable_zip_matches(
    path: Path,
    label: str,
    validator: Callable[[zipfile.ZipFile], bool],
) -> bool:
    """Validate one ZIP through a stable local single-link file identity."""
    try:
        expected = _real_file_identity(path, label)
        if expected[5] == 0:
            return False
        with path.open("rb") as handle:
            opened = _file_identity(os.fstat(handle.fileno()))
            if opened != expected:
                return False
            with zipfile.ZipFile(handle) as archive:
                matches = validator(archive)
            finished = _file_identity(os.fstat(handle.fileno()))
        if finished != expected:
            return False
        return matches and _real_file_identity(path, label) == expected
    except (
        OSError,
        RunFailure,
        RuntimeError,
        ValueError,
        plistlib.InvalidFileException,
        zipfile.BadZipFile,
    ):
        return False


def _is_android_apk(path: Path) -> bool:
    """Return whether one stable APK contains an ARM64 native library."""
    return _stable_zip_matches(
        path,
        "Android APK",
        _android_archive_contains_arm64,
    )


def _ios_main_binary(
    archive: zipfile.ZipFile,
    inventory: dict[str, zipfile.ZipInfo],
) -> zipfile.ZipInfo | None:
    """Return the main iOS application binary declared by one IPA."""
    property_lists = [
        info
        for name, info in inventory.items()
        if len(name.split("/")) == 3
        and name.startswith("Payload/")
        and name.endswith(".app/Info.plist")
        and not info.is_dir()
    ]
    if len(property_lists) != 1:
        return None
    plist_info = property_lists[0]
    document = plistlib.loads(archive.read(plist_info))
    if not isinstance(document, dict):
        return None
    executable = document.get("CFBundleExecutable")
    if (
        not isinstance(executable, str)
        or executable in {"", ".", ".."}
        or "/" in executable
        or "\\" in executable
    ):
        return None
    app_root = plist_info.filename.rsplit("/", 1)[0]
    binary_info = inventory.get(f"{app_root}/{executable}")
    if binary_info is None or binary_info.is_dir():
        return None
    return binary_info


def _ios_archive_contains_arm64(archive: zipfile.ZipFile) -> bool:
    """Return whether one opened IPA main executable contains ARM64."""
    inventory = _zip_inventory(archive)
    if inventory is None:
        return False
    binary_info = _ios_main_binary(archive, inventory)
    if binary_info is None:
        return False
    with archive.open(binary_info) as stream:
        prefix = stream.read(4)
        return _matches_macho(
            stream,
            prefix,
            "arm64",
            binary_info.file_size,
        )


def _is_ios_ipa(path: Path) -> bool:
    """Return whether one stable IPA main executable contains ARM64."""
    return _stable_zip_matches(
        path,
        "iOS IPA",
        _ios_archive_contains_arm64,
    )


def _is_native_executable(
    path: Path,
    system: str,
    architecture: str,
) -> bool:
    """Return whether one stable runtime is runnable and target-native."""
    return _has_native_binary_signature(path, system, architecture)


def _has_linux_runtime(files: Sequence[Path], target: Target) -> bool:
    """Return whether a Linux archive contains a non-empty SHAR binary."""
    return any(
        _is_native_executable(item, "linux", target.architecture)
        and _is_shar_runtime_name(item.name)
        for item in files
    )


def _has_macos_runtime(
    candidate: Path,
    files: Sequence[Path],
    target: Target,
) -> bool:
    """Return whether a macOS archive contains a runnable SHAR app bundle."""
    for item in files:
        parts = item.relative_to(candidate).parts
        if len(parts) < 4:
            continue
        bundle = parts[-4]
        if (
            not bundle.casefold().endswith(".app")
            or parts[-3:-1] != ("Contents", "MacOS")
        ):
            continue
        if (
            _is_native_executable(item, "macos", target.architecture)
            and _is_shar_runtime_name(item.name)
        ):
            return True
    return False


def _validate_candidate_artifact(
    candidate: Path,
    target: Target,
    files: Sequence[Path] | None = None,
) -> None:
    """Require UAT archives to contain their declared runnable artifact."""
    candidate_files = (
        _validate_candidate_tree(candidate).files
        if files is None
        else tuple(files)
    )
    if target.system == "linux":
        if _has_linux_runtime(candidate_files, target):
            return
        message = (
            "candidate package has no non-empty Linux SHAR executable: "
            f"{candidate}"
        )
        raise RunFailure(message)
    if target.system == "macos":
        if _has_macos_runtime(candidate, candidate_files, target):
            return
        message = (
            "candidate package has no runnable macOS SHAR app bundle: "
            f"{candidate}"
        )
        raise RunFailure(message)

    if target.system == "windows":
        if any(
            item.suffix.casefold() == ".exe"
            and _is_native_executable(item, "windows", target.architecture)
            and _is_shar_runtime_name(item.stem)
            for item in candidate_files
        ):
            return
        message = (
            "candidate package has no non-empty Windows SHAR executable: "
            f"{candidate}"
        )
        raise RunFailure(message)

    mobile_validator = {
        "apk": (".apk", "ARM64 Android APK", _is_android_apk),
        "ipa": (".ipa", "ARM64 iOS IPA", _is_ios_ipa),
    }.get(target.artifact)
    if mobile_validator is None:
        return
    suffix, label, validator = mobile_validator
    if any(
        item.suffix.casefold() == suffix and validator(item)
        for item in candidate_files
    ):
        return
    raise RunFailure(f"candidate package has no valid {label}: {candidate}")


def _require_cache_root_if_present(path: Path, label: str) -> None:
    """Preflight one existing diagnostic cache without mutating it."""
    if _path_present(path):
        _require_real_directory(path, label)


def _remove_captured_artifact(
    source: Path,
    label: str,
    expected: tuple[int, ...],
) -> None:
    """Remove only the cache-owned candidate identity that was captured."""
    try:
        current = _real_file_identity(source, label)
    except RunFailure as error:
        raise RunFailure(f"{label} changed before caching: {source}") from error
    if current != expected:
        raise RunFailure(f"{label} changed before caching: {source}")
    try:
        source.unlink()
    except OSError as error:
        raise RunFailure(f"cannot remove cached {label}: {source}") from error
    if _path_present(source):
        raise RunFailure(f"{label} reappeared while caching: {source}")


def _captured_artifact(source: Path, label: str) -> _CapturedArtifact:
    """Capture one diagnostic source before cache mutation begins."""
    payload, identity = _capture_real_bytes(source, label)
    return _CapturedArtifact(source, payload, identity)


def _write_cached_artifact(
    destination: Path,
    payload: bytes,
    label: str,
) -> None:
    """Persist one captured diagnostic artifact through an exclusive file."""
    try:
        with destination.open("xb") as handle:
            handle.write(payload)
            handle.flush()
            opened = _file_identity(os.fstat(handle.fileno()))
    except FileExistsError as error:
        message = f"{label} cache path already exists: {destination}"
        raise RunFailure(message) from error
    except OSError as error:
        raise RunFailure(f"cannot cache {label}: {destination}") from error
    current = _real_file_identity(destination, label)
    if opened[5] != len(payload) or current != opened:
        raise RunFailure(f"{label} cache changed while writing: {destination}")


def _ensure_cached_parent(root: Path, relative: Path, label: str) -> Path:
    """Create or require each repository-owned cache parent in order."""
    current = root
    for component in relative.parts:
        current /= component
        _ensure_real_directory(current, label)
    return current


def _cache_nonruntime_artifacts(
    candidate: Path,
    work: Path,
    target: Target,
    tree: _CandidateTree | None = None,
) -> None:
    """Keep packaging metadata and debug symbols out of final dist output."""
    _ensure_real_directory(work, "target work root")
    metadata = work / "publication-metadata"
    symbols = work / "symbols"
    inventory = _validate_candidate_tree(candidate) if tree is None else tree
    entries = (*inventory.files, *inventory.directories)
    manifests = sorted(
        item
        for item in entries
        if item.parent == candidate and item.match("Manifest_*.txt")
    )
    debug_files: list[Path] = []
    if target.system == "windows":
        debug_files = sorted(
            item for item in entries if item.match("*.pdb")
        )

    _require_cache_root_if_present(metadata, "publication metadata cache")
    _require_cache_root_if_present(symbols, "symbol cache")
    manifest_snapshots = [
        _captured_artifact(source, "packaging manifest")
        for source in manifests
    ]
    debug_snapshots = [
        _captured_artifact(source, "debug symbol")
        for source in debug_files
    ]
    for artifact in manifest_snapshots:
        _remove_captured_artifact(
            artifact.source,
            "packaging manifest",
            artifact.identity,
        )
    for artifact in debug_snapshots:
        _remove_captured_artifact(
            artifact.source,
            "debug symbol",
            artifact.identity,
        )

    _remove_real_directory_if_present(
        metadata,
        "publication metadata cache",
    )
    _remove_real_directory_if_present(symbols, "symbol cache")

    if manifest_snapshots:
        metadata.mkdir()
        for artifact in manifest_snapshots:
            _write_cached_artifact(
                metadata / artifact.source.name,
                artifact.payload,
                "packaging manifest",
            )
    if debug_snapshots:
        symbols.mkdir()
        for artifact in debug_snapshots:
            relative = artifact.source.relative_to(candidate)
            destination = symbols / relative
            _ensure_cached_parent(
                symbols,
                relative.parent,
                "symbol cache parent",
            )
            _write_cached_artifact(
                destination,
                artifact.payload,
                "debug symbol",
            )


def _rollback_publication_swap(
    candidate: Path,
    destination: Path,
    backup: Path,
    error: OSError,
) -> None:
    """Restore the previous publication after post-swap cleanup fails."""
    failures: list[str] = []
    for source, target, label in (
        (destination, candidate, "candidate"),
        (backup, destination, "previous"),
    ):
        try:
            Path(source).replace(target)
        except OSError as rollback:
            failures.append(f"{label}:{rollback.__class__.__name__}")
    if failures:
        detail = ", ".join(failures)
        raise RunFailure(
            "publication cleanup failed and rollback failed: " + detail
        ) from error
    raise RunFailure(
        "publication cleanup failed; previous target restored"
    ) from error


def _publish(candidate: Path, destination: Path) -> None:
    """Replace one published target without exposing a partial candidate."""
    tree = _validate_candidate_tree(candidate)
    if not tree.files:
        raise RunFailure(f"candidate package is empty: {candidate}")
    if _path_present(destination):
        _require_real_directory(destination, "published target")
    publication_root = destination.parent
    if _path_present(publication_root):
        _require_real_directory(publication_root, "publication root")
    else:
        publication_root.mkdir(parents=True)
    backup = destination.with_name(f".{destination.name}.previous")
    if _path_present(backup):
        _require_real_directory(backup, "publication backup")
        shutil.rmtree(backup)
    had_previous = destination.exists()
    if had_previous:
        Path(destination).replace(backup)
    try:
        Path(candidate).replace(destination)
    except OSError:
        if had_previous and backup.exists() and not destination.exists():
            Path(backup).replace(destination)
        raise
    if _path_present(backup):
        try:
            shutil.rmtree(backup)
        except OSError as error:
            _rollback_publication_swap(
                candidate,
                destination,
                backup,
                error,
            )


def _build_target(
    root: Path,
    uat: Path,
    project: Path,
    target: Target,
    *,
    validate_only: bool,
) -> None:
    """Verify and optionally package one selected target transactionally."""
    build_root = _ensure_build_cache_root(root)
    run_root = build_root / "run"
    _ensure_real_directory(run_root, "build run root")
    work = run_root / target.identifier
    _ensure_real_directory(work, "target work root")
    _verify_sdk(root, uat, project, target, work)
    if validate_only:
        print(f"run: {target.identifier}: SDK valid")
        return

    candidate = work / "candidate"
    staging = work / "stage"
    _reset_real_directory(candidate, "candidate scratch root")
    _reset_real_directory(staging, "staging scratch root")
    log = work / "build.log"
    arguments = _build_arguments(project, target, candidate, staging)
    _run_uat(root, uat, arguments, log)
    candidate_tree = _validate_candidate_tree(candidate)
    _validate_candidate_artifact(candidate, target, candidate_tree.files)
    _cache_nonruntime_artifacts(candidate, work, target, candidate_tree)
    destination = root / _DIST_ROOT / target.identifier
    _publish(candidate, destination)
    print(f"run: {target.identifier}: published {destination}")


def _parser() -> argparse.ArgumentParser:
    """Build the supported packaging command-line surface."""
    parser = argparse.ArgumentParser(
        description="Build selected SHAR targets with Unreal AutomationTool.",
    )
    parser.add_argument(
        "--arch-file",
        type=Path,
        help="override arch.json path for deterministic testing",
    )
    parser.add_argument(
        "--check-file",
        type=Path,
        help="override check.json path for deterministic testing",
    )
    parser.add_argument(
        "--validate-only",
        action="store_true",
        help="revalidate decisions and Turnkey SDKs without packaging",
    )
    return parser


def _require_unreal_evidence(check: dict[str, object]) -> dict[str, object]:
    """Return the saved Unreal evidence object or fail closed."""
    unreal = check.get("unreal")
    if not isinstance(unreal, dict):
        raise RunFailure("check evidence has no unreal object")
    return unreal


def main() -> int:
    """Revalidate saved decisions and build every selected target."""
    args = _parser().parse_args()
    root = _root()
    arch_path = args.arch_file or (root / _ARCH_PATH)
    check_path = args.check_file or (root / _CHECK_PATH)
    if not arch_path.is_absolute():
        arch_path = root / arch_path
    if not check_path.is_absolute():
        check_path = root / check_path
    try:
        arch_snapshot = _revalidate_arch(root, arch_path)
        targets = _selected_targets(arch_snapshot)
        check_snapshot = _revalidate_check(root, check_path)
        check = _check_evidence(check_snapshot)
        unreal = _require_unreal_evidence(check)
        engine_root = Path(str(unreal["root"])).resolve()
        project = _project_from_evidence(root, unreal)
        _prepare_project_state(root, project)
        uat = _uat_path(engine_root)
        for target in targets:
            _build_target(
                root,
                uat,
                project,
                target,
                validate_only=args.validate_only,
            )
    except (RunFailure, OSError) as error:
        print(f"run: {error}", file=sys.stderr)
        return 1
    if args.validate_only:
        print(f"run: validated {len(targets)} selected target(s)")
    else:
        print(f"run: published {len(targets)} selected target(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

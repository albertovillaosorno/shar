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

# CSpell:ignore APPL BNDL FMWK PHDR RVA dylinker linkedit symtab
# CSpell:ignore DYSYMTAB dysymtab NBLCK msvcrt
# CSpell:ignore phdr creationflags killpg taskkill axo ppid
# CSpell:ignore pthread sigmask SETMASK

from __future__ import annotations

import argparse
from collections.abc import Callable
from collections.abc import Sequence
from contextlib import suppress
import hashlib
from itertools import pairwise
import json
import os
from pathlib import Path
import plistlib
import signal
import time

if os.name == "nt":
    import msvcrt
else:
    import fcntl
import shutil
import stat
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
_RUN_LOCK_PATH = Path(".cache/build/run.lock")
_PROJECT_STATE_ROOT = Path(".cache/build/project-state")
_PROJECT_STATE_NAMES = ("Binaries", "DerivedDataCache", "Intermediate", "Saved")
_DIST_ROOT = Path("dist")
_CHILD_STOP_TIMEOUT_SECONDS = 5
_MANAGED_CHILD_ENV = "SHAR_BUILD_RUNNER_CHILD"


class RunFailure(RuntimeError):
    """One actionable build-runner failure."""


class _RunSignal(BaseException):
    """One runner termination signal converted into controlled cleanup."""

    def __init__(self, signum: int) -> None:
        self.signum = signum
        super().__init__(signum)


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


class _CandidateTreeSnapshot(NamedTuple):
    """One candidate tree bound to stable local filesystem identities."""

    root: tuple[int, int, int]
    files: tuple[tuple[Path, tuple[int, ...]], ...]
    directories: tuple[tuple[Path, tuple[int, int, int]], ...]


class _CapturedArtifact(NamedTuple):
    """One stable diagnostic source snapshot bound to its file identity."""

    source: Path
    payload: bytes
    identity: tuple[int, ...]


class _MachOSegment(NamedTuple):
    """One bounded 64-bit Mach-O segment mapping."""

    name: bytes
    virtual_address: int
    virtual_size: int
    file_offset: int
    file_size: int
    initial_protection: int


class _MachOAuxiliary(NamedTuple):
    """One parsed non-segment Mach-O command."""

    kind: str
    value: int = 0
    size: int = 0
    ranges: tuple[tuple[int, int], ...] = ()
    symbol_groups: tuple[tuple[int, int], ...] = ()


class _PeSectionPolicy(NamedTuple):
    """PE section alignment and image-boundary admission fields."""

    section_alignment: int
    file_alignment: int
    image_size: int
    header_size: int


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
    environment, token = _managed_child_environment()
    process = subprocess.Popen(
        [*command, "--expected-sha256", expected_sha256],
        cwd=root,
        env=environment,
        **_child_process_options(),
    )
    returncode = _wait_managed_child(process, token)
    if returncode:
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


def _real_directory_identity(path: Path, label: str) -> tuple[int, int, int]:
    """Return stable device/inode/mode identity for one real directory."""
    _require_real_directory(path, label)
    try:
        metadata = path.stat(follow_symlinks=False)
    except OSError as error:
        raise RunFailure(f"cannot inspect {label}: {path}") from error
    return metadata.st_dev, metadata.st_ino, metadata.st_mode


def _require_directory_identity(
    path: Path,
    label: str,
    expected: tuple[int, int, int],
) -> None:
    """Require one directory path to retain a captured local identity."""
    try:
        current = _real_directory_identity(path, label)
    except RunFailure as error:
        message = f"{label} changed before publication: {path}"
        raise RunFailure(message) from error
    if current != expected:
        raise RunFailure(f"{label} changed before publication: {path}")


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


def _lock_run_handle(handle: TextIO) -> None:
    """Acquire one non-blocking host-native advisory build-runner lock."""
    try:
        if os.name == "nt":
            handle.seek(0)
            msvcrt.locking(handle.fileno(), msvcrt.LK_NBLCK, 1)
        else:
            fcntl.flock(handle.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
    except OSError as error:
        message = "another build runner is already active"
        raise RunFailure(message) from error


def _require_run_lock_identity(
    path: Path,
    handle: TextIO,
    expected: tuple[int, ...] | None,
) -> None:
    """Require one opened runner-lock descriptor to match its path."""
    opened = _file_identity(os.fstat(handle.fileno()))
    if expected is not None and opened != expected:
        raise RunFailure(f"build runner lock changed before opening: {path}")
    current = _real_file_identity(path, "build runner lock")
    if current != opened:
        raise RunFailure(f"build runner lock changed before opening: {path}")


def _acquire_run_lock(root: Path) -> TextIO:
    """Open and lock the repository-local runner identity."""
    _ensure_build_cache_root(root)
    path = root / _RUN_LOCK_PATH
    existed = _path_present(path)
    expected = (
        _real_file_identity(path, "build runner lock")
        if existed
        else None
    )
    try:
        handle = path.open("a+", encoding="utf-8", newline="\n")
    except OSError as error:
        raise RunFailure(f"cannot open build runner lock: {path}") from error
    try:
        _require_run_lock_identity(path, handle, expected)
        handle.seek(0, os.SEEK_END)
        if handle.tell() == 0:
            handle.write("\0")
            handle.flush()
        _lock_run_handle(handle)
    except BaseException:
        handle.close()
        raise
    return handle


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


def _detach_project_state_link(
    link: Path,
    canonical: Path,
    name: str,
) -> None:
    """Detach one canonical project-state link without touching its target."""
    _require_real_directory(canonical, f"canonical project {name}")
    if not _is_directory_link(link):
        raise RunFailure(f"project {name} link changed before detach")
    if link.resolve() != canonical.resolve():
        raise RunFailure(f"project {name} link changed before detach")
    _remove_directory_link(link)


def _detach_project_state(root: Path, project: Path) -> None:
    """Detach runner-created project links while retaining cache contents."""
    project_dir = project.parent
    state_root = root / _PROJECT_STATE_ROOT
    _require_real_directory(state_root, "project-state cache root")
    failures: list[str] = []
    for name in _PROJECT_STATE_NAMES:
        try:
            _detach_project_state_link(
                project_dir / name,
                state_root / name,
                name,
            )
        except (OSError, RunFailure) as error:
            failures.append(f"{name}:{error}")
    if failures:
        raise RunFailure(
            "project-state detach failed: " + "; ".join(failures)
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


def _child_process_options() -> dict[str, object]:
    """Return host-native process isolation for one runner-owned child."""
    if os.name == "nt":
        return {
            "creationflags": getattr(
                subprocess,
                "CREATE_NEW_PROCESS_GROUP",
                0x00000200,
            )
        }
    return {"start_new_session": True}


def _managed_child_environment(
    environment: dict[str, str] | None = None,
) -> tuple[dict[str, str], str]:
    """Tag one child environment so escaped Linux descendants remain owned."""
    values = os.environ.copy() if environment is None else environment.copy()
    token = f"{os.getpid()}-{time.monotonic_ns()}"
    values[_MANAGED_CHILD_ENV] = token
    return values, token


def _posix_process_table() -> list[tuple[int, int, str]]:
    """Return one POSIX process snapshot for child descendant cleanup."""
    result = subprocess.run(
        ["ps", "-axo", "pid=,ppid=,stat="],
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode:
        raise RunFailure("cannot enumerate child descendants")
    rows: list[tuple[int, int, str]] = []
    for line in result.stdout.splitlines():
        fields = line.split(None, 2)
        if len(fields) != 3:
            continue
        if not fields[0].isdigit() or not fields[1].isdigit():
            continue
        rows.append((int(fields[0]), int(fields[1]), fields[2]))
    return rows


def _posix_tree_pids(root_pid: int) -> list[int]:
    """Return live child descendants, including processes in new sessions."""
    table = _posix_process_table()
    children: dict[int, list[int]] = {}
    live: set[int] = set()
    for pid, parent, state in table:
        if state.startswith("Z"):
            continue
        live.add(pid)
        children.setdefault(parent, []).append(pid)
    if root_pid not in live:
        return []
    found = [root_pid]
    pending = [root_pid]
    while pending:
        parent = pending.pop()
        for child in children.get(parent, []):
            found.append(child)
            pending.append(child)
    return found


def _wait_for_posix_pids(pids: set[int], timeout: float) -> set[int]:
    """Return child PIDs still live after one bounded wait."""
    deadline = time.monotonic() + timeout
    remaining = set(pids)
    while remaining and time.monotonic() < deadline:
        live = {pid for pid, _parent, state in _posix_process_table()
                if not state.startswith("Z")}
        remaining.intersection_update(live)
        if remaining:
            time.sleep(0.05)
    return remaining


def _signal_posix_pids(pids: set[int], signum: int) -> None:
    """Signal one snapshotted child tree without following replacement PIDs."""
    for pid in sorted(pids):
        with suppress(ProcessLookupError):
            os.kill(pid, signum)


def _linux_tagged_child_pids(token: str) -> set[int]:
    """Return Linux processes that still carry one managed-child token."""
    if sys.platform != "linux":
        return set()
    marker = f"{_MANAGED_CHILD_ENV}={token}".encode()
    found: set[int] = set()
    for process_root in Path("/proc").iterdir():
        if not process_root.name.isdigit():
            continue
        try:
            environment = (process_root / "environ").read_bytes().split(b"\0")
        except OSError:
            environment = []
        if marker in environment:
            found.add(int(process_root.name))
    return found


def _kill_linux_tagged_children(token: str) -> None:
    """Force-stop tagged Linux descendants that escaped ancestry cleanup."""
    if sys.platform != "linux":
        return
    deadline = time.monotonic() + _CHILD_STOP_TIMEOUT_SECONDS
    while True:
        pids = _linux_tagged_child_pids(token)
        if not pids:
            return
        _signal_posix_pids(pids, signal.SIGKILL)
        if time.monotonic() >= deadline:
            break
        time.sleep(0.05)
    remaining = _linux_tagged_child_pids(token)
    if remaining:
        raise RunFailure(
            "interrupted tagged child descendants did not terminate"
        )


def _terminate_child_tree(process: subprocess.Popen[str], token: str) -> None:
    """Stop one interrupted runner-owned process tree before cleanup."""
    if os.name == "nt":
        subprocess.run(
            ["taskkill", "/pid", str(process.pid), "/t", "/f"],
            check=False,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        process.wait()
        return
    pids = set(_posix_tree_pids(process.pid))
    _signal_posix_pids(pids, signal.SIGTERM)
    remaining = _wait_for_posix_pids(pids, _CHILD_STOP_TIMEOUT_SECONDS)
    if remaining:
        _signal_posix_pids(remaining, signal.SIGKILL)
        remaining = _wait_for_posix_pids(remaining, _CHILD_STOP_TIMEOUT_SECONDS)
    process.wait()
    if remaining:
        raise RunFailure("interrupted child descendants did not terminate")
    _kill_linux_tagged_children(token)


def _wait_managed_child(process: subprocess.Popen[str], token: str) -> int:
    """Wait for one runner-owned child and reap its tree on interruption."""
    try:
        return process.wait()
    except (KeyboardInterrupt, OSError, _RunSignal):
        _terminate_child_tree(process, token)
        raise


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
    environment, token = _managed_child_environment()
    environment["uebp_EngineSavedFolder"] = str(automation_saved)
    environment["uebp_FinalLogFolder"] = str(automation_logs)
    environment["uebp_LogFolder"] = str(automation_logs)
    environment["UE-LocalDataCachePath"] = str(ddc)
    with _open_uat_log(log) as handle:
        process = subprocess.Popen(
            command,
            cwd=root,
            env=environment,
            stdout=handle,
            stderr=subprocess.STDOUT,
            text=True,
            **_child_process_options(),
        )
        returncode = _wait_managed_child(process, token)
    if returncode:
        raise RunFailure(
            f"Unreal AutomationTool failed with {returncode}; see {log}"
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
    arguments = [
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
    if target.system == "linux":
        arguments.extend(("-SkipBuildEditor", "-UbtArgs=-NoUBA"))
    return arguments


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


def _candidate_tree_snapshot(
    candidate: Path,
    tree: _CandidateTree,
) -> _CandidateTreeSnapshot:
    """Capture one validated candidate tree's stable local identities."""
    return _CandidateTreeSnapshot(
        root=_real_directory_identity(candidate, "candidate package"),
        files=tuple(
            (item, _real_file_identity(item, "candidate package file"))
            for item in tree.files
        ),
        directories=tuple(
            (
                item,
                _real_directory_identity(item, "candidate package directory"),
            )
            for item in tree.directories
        ),
    )


def _require_candidate_tree_snapshot(
    candidate: Path,
    snapshot: _CandidateTreeSnapshot,
) -> None:
    """Require the candidate path to retain one validated tree snapshot."""
    message = f"candidate package changed before publication: {candidate}"
    try:
        current = _validate_candidate_tree(candidate)
        root = _real_directory_identity(candidate, "candidate package")
        files = tuple(
            (item, _real_file_identity(item, "candidate package file"))
            for item in current.files
        )
        directories = tuple(
            (
                item,
                _real_directory_identity(item, "candidate package directory"),
            )
            for item in current.directories
        )
    except RunFailure as error:
        raise RunFailure(message) from error
    if _CandidateTreeSnapshot(root, files, directories) != snapshot:
        raise RunFailure(message)


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
_MACHO_PLATFORMS = {"macos": 1, "ios": 2}
_MACHO_CPU_SUBTYPE_MASK = 0xFF000000
_MACHO_DYLIB_LOAD_COMMANDS = {0xC, 0x80000018, 0x8000001F, 0x80000023}
_MACHO_DYLD_INFO_COMMANDS = {0x22, 0x80000022}
_MACHO_LINKEDIT_DATA_COMMANDS = {
    0x1D,
    0x1E,
    0x26,
    0x29,
    0x2B,
    0x80000033,
    0x80000034,
}
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


def _elf_program_file_range_is_bounded(
    program: bytes,
    byte_order: str,
    file_size: int,
) -> bool:
    """Return whether one non-null ELF segment file image is bounded."""
    offset = int.from_bytes(program[8:16], byte_order)
    file_bytes = int.from_bytes(program[32:40], byte_order)
    return file_bytes == 0 or (
        offset <= file_size and file_bytes <= file_size - offset
    )


def _elf_load_segment_state(
    program: bytes,
    byte_order: str,
    file_size: int,
    entrypoint: int,
) -> tuple[bool, bool, bool]:
    """Return bounds, execute permission, and entrypoint containment."""
    flags = int.from_bytes(program[4:8], byte_order)
    offset = int.from_bytes(program[8:16], byte_order)
    virtual_address = int.from_bytes(program[16:24], byte_order)
    file_bytes = int.from_bytes(program[32:40], byte_order)
    memory_bytes = int.from_bytes(program[40:48], byte_order)
    alignment = int.from_bytes(program[48:56], byte_order)
    aligned = alignment in {0, 1} or (
        alignment & (alignment - 1) == 0
        and virtual_address % alignment == offset % alignment
    )
    bounded = (
        aligned
        and file_bytes <= memory_bytes
        and memory_bytes <= ((1 << 64) - 1) - virtual_address
        and offset <= file_size
        and file_bytes <= file_size - offset
    )
    executable = bool(flags & 0x1) and file_bytes > 0
    contains_entrypoint = (
        executable
        and virtual_address
        <= entrypoint
        < virtual_address + file_bytes
    )
    return bounded, executable, contains_entrypoint


def _elf_program_header_segment_is_valid(
    program: bytes,
    byte_order: str,
    layout: tuple[int, int, int],
) -> bool:
    """Return whether PT_PHDR exactly describes the ELF program table."""
    table_size = layout[1] * layout[2]
    return (
        int.from_bytes(program[8:16], byte_order) == layout[0]
        and int.from_bytes(program[32:40], byte_order) == table_size
        and int.from_bytes(program[40:48], byte_order) == table_size
    )


def _elf_load_contains_program_table(
    program: bytes,
    byte_order: str,
    layout: tuple[int, int, int],
) -> bool:
    """Return whether one load segment maps the whole program-header table."""
    offset = int.from_bytes(program[8:16], byte_order)
    file_bytes = int.from_bytes(program[32:40], byte_order)
    table_end = layout[0] + (layout[1] * layout[2])
    return offset <= layout[0] and table_end <= offset + file_bytes


def _elf_dynamic_array_is_valid(
    stream: object,
    program: bytes,
    byte_order: str,
) -> bool:
    """Return whether one PT_DYNAMIC has whole ELF64 entries and DT_NULL."""
    offset = int.from_bytes(program[8:16], byte_order)
    file_bytes = int.from_bytes(program[32:40], byte_order)
    if file_bytes < 16 or file_bytes % 16:
        return False
    try:
        cursor = stream.tell()
        stream.seek(offset)
        payload = stream.read(file_bytes)
        stream.seek(cursor)
    except (OSError, ValueError):
        return False
    if len(payload) != file_bytes:
        return False
    return any(
        int.from_bytes(payload[start : start + 8], byte_order) == 0
        for start in range(0, file_bytes, 16)
    )


def _elf_tls_segment_is_valid(
    program: bytes,
    byte_order: str,
) -> bool:
    """Return whether one PT_TLS header has a coherent TLS template."""
    flags = int.from_bytes(program[4:8], byte_order)
    offset = int.from_bytes(program[8:16], byte_order)
    virtual_address = int.from_bytes(program[16:24], byte_order)
    file_bytes = int.from_bytes(program[32:40], byte_order)
    memory_bytes = int.from_bytes(program[40:48], byte_order)
    alignment = int.from_bytes(program[48:56], byte_order)
    aligned = alignment in {0, 1} or (
        alignment & (alignment - 1) == 0
        and virtual_address % alignment == offset % alignment
    )
    return (
        flags == 0x4
        and file_bytes <= memory_bytes
        and memory_bytes <= ((1 << 64) - 1) - virtual_address
        and aligned
    )


def _elf_interpreter_path_is_valid(
    stream: object,
    program: bytes,
    byte_order: str,
) -> bool:
    """Return whether one PT_INTERP names one bounded nonempty path."""
    offset = int.from_bytes(program[8:16], byte_order)
    file_bytes = int.from_bytes(program[32:40], byte_order)
    if file_bytes < 2:
        return False
    try:
        cursor = stream.tell()
        stream.seek(offset)
        path = stream.read(file_bytes)
        stream.seek(cursor)
    except (OSError, ValueError):
        return False
    return (
        len(path) == file_bytes
        and path[-1:] == b"\0"
        and bool(path[:-1])
        and b"\0" not in path[:-1]
    )


class _ElfProgramContext(NamedTuple):
    """Immutable ELF program-table validation context."""

    byte_order: str
    layout: tuple[int, int, int]
    file_size: int


def _elf_supplementary_program_state(
    stream: object,
    program: bytes,
    context: _ElfProgramContext,
    state: int,
    *,
    loadable: bool,
) -> int:
    """Validate one non-load program header and return updated state."""
    program_type = int.from_bytes(program[:4], context.byte_order)
    if program_type == 0:
        return state
    if program_type == 5 or not _elf_program_file_range_is_bounded(
        program,
        context.byte_order,
        context.file_size,
    ):
        return -1
    result = state
    if program_type == 2:
        if not _elf_dynamic_array_is_valid(
            stream,
            program,
            context.byte_order,
        ):
            result = -1
    elif program_type == 3:
        valid = (
            not loadable
            and state & 0x1 == 0
            and _elf_interpreter_path_is_valid(
                stream,
                program,
                context.byte_order,
            )
        )
        result = state | 0x1 if valid else -1
    elif program_type == 6:
        valid = (
            not loadable
            and state & 0x2 == 0
            and _elf_program_header_segment_is_valid(
                program,
                context.byte_order,
                context.layout,
            )
        )
        result = state | 0x2 if valid else -1
    elif program_type == 7 and not _elf_tls_segment_is_valid(
        program,
        context.byte_order,
    ):
        result = -1
    return result


def _elf_entrypoint_is_metadata(
    program: bytes,
    byte_order: str,
    entrypoint: int,
    virtual_address: int,
    layout: tuple[int, int, int],
) -> bool:
    """Return whether one mapped ELF entrypoint resolves to image metadata."""
    file_offset = int.from_bytes(program[8:16], byte_order)
    entry_file_offset = file_offset + (entrypoint - virtual_address)
    program_offset, program_size, program_count = layout
    program_end = program_offset + (program_size * program_count)
    return entry_file_offset < 64 or (
        program_offset <= entry_file_offset < program_end
    )


def _elf_load_programs_match(
    stream: object,
    byte_order: str,
    layout: tuple[int, int, int],
    file_size: int,
    entrypoint: int,
    *,
    require_entrypoint: bool,
) -> bool:
    """Validate ELF load segments and optional process entrypoint."""
    program_offset = layout[0]
    try:
        stream.seek(program_offset)
    except (OSError, ValueError):
        return False
    loadable = False
    executable = False
    entrypoint_in_executable = False
    special_segments = 0
    load_segments_valid = True
    previous_load_address: int | None = None
    for _ in range(layout[2]):
        program = stream.read(layout[1])
        if len(program) != layout[1]:
            return False
        program_type = int.from_bytes(program[:4], byte_order)
        if program_type != 1:
            special_segments = _elf_supplementary_program_state(
                stream,
                program,
                _ElfProgramContext(byte_order, layout, file_size),
                special_segments,
                loadable=loadable,
            )
            if special_segments < 0:
                return False
            continue
        if not _elf_program_file_range_is_bounded(
            program,
            byte_order,
            file_size,
        ):
            return False
        virtual_address = int.from_bytes(program[16:24], byte_order)
        if (
            previous_load_address is not None
            and virtual_address < previous_load_address
        ):
            load_segments_valid = False
        previous_load_address = virtual_address
        bounded, segment_executable, contains_entrypoint = (
            _elf_load_segment_state(
                program,
                byte_order,
                file_size,
                entrypoint,
            )
        )
        if contains_entrypoint and _elf_entrypoint_is_metadata(
            program,
            byte_order,
            entrypoint,
            virtual_address,
            layout,
        ):
            contains_entrypoint = False
        load_segments_valid = load_segments_valid and bounded
        if bounded and _elf_load_contains_program_table(
            program,
            byte_order,
            layout,
        ):
            special_segments |= 0x4
        loadable = True
        executable = executable or segment_executable
        entrypoint_in_executable = (
            entrypoint_in_executable or contains_entrypoint
        )
    entrypoint_ok = not require_entrypoint or (
        entrypoint != 0 and entrypoint_in_executable
    )
    return (
        loadable
        and executable
        and load_segments_valid
        and entrypoint_ok
        and (special_segments & 0x2 == 0 or bool(special_segments & 0x4))
    )


def _elf_ident_is_valid(header: bytes) -> bool:
    """Return whether one complete ELF64 identification block is canonical."""
    return (
        len(header) == 64
        and header[4] == 2
        and header[6] == 1
        and header[7] in {0, 3}
        and header[8] == 0
        and not any(header[9:16])
    )


def _matches_elf(
    stream: object,
    prefix: bytes,
    architecture: str,
    file_size: int,
    *,
    require_entrypoint: bool = False,
    require_shared_object: bool = False,
) -> bool:
    """Return whether one admitted ELF64 image declares the selected CPU."""
    header = prefix + stream.read(60)
    if not _elf_ident_is_valid(header):
        return False
    byte_order = {1: "little", 2: "big"}.get(header[5])
    expected = _ELF_MACHINES.get(architecture)
    if byte_order is None or expected is None or byte_order != "little":
        return False
    processor_flags = int.from_bytes(header[48:52], byte_order)
    if processor_flags != 0:
        return False
    image_type = int.from_bytes(header[16:18], byte_order)
    if require_shared_object and image_type != 3:
        return False
    layout = _elf_program_layout(header, byte_order, expected, file_size)
    if layout is None:
        return False
    entrypoint = int.from_bytes(header[24:32], byte_order)
    return _elf_load_programs_match(
        stream,
        byte_order,
        layout,
        file_size,
        entrypoint,
        require_entrypoint=require_entrypoint,
    )


def _pe_optional_layout(
    coff: bytes,
    expected: int,
    offset: int,
    file_size: int,
) -> tuple[int, int] | None:
    """Return bounded PE32+ optional and section-table dimensions."""
    machine = int.from_bytes(coff[:2], "little")
    section_count = int.from_bytes(coff[2:4], "little")
    symbol_table = int.from_bytes(coff[8:12], "little")
    symbol_count = int.from_bytes(coff[12:16], "little")
    optional_size = int.from_bytes(coff[16:18], "little")
    characteristics = int.from_bytes(coff[18:20], "little")
    if (
        machine != expected
        or section_count == 0
        or section_count > 96
        or symbol_table != 0
        or symbol_count != 0
    ):
        return None
    if (
        optional_size < 112
        or characteristics & 0x0002 == 0
        or characteristics & 0x0010
        or characteristics & 0x0100
        or characteristics & 0x2000
    ):
        return None
    remaining = file_size - offset
    required = 24 + optional_size + (40 * section_count)
    if required > remaining:
        return None
    return optional_size, section_count


def _pe_optional_fields_are_valid(optional: bytes) -> bool:
    """Return whether required PE32+ loader fields are coherent."""
    image_base = int.from_bytes(optional[24:32], "little")
    section_alignment = int.from_bytes(optional[32:36], "little")
    file_alignment = int.from_bytes(optional[36:40], "little")
    dll_characteristics = int.from_bytes(optional[70:72], "little")
    file_alignment_valid = (
        0x200 <= file_alignment <= 0x10000
        and file_alignment & (file_alignment - 1) == 0
    )
    low_alignment_valid = (
        section_alignment >= 0x1000 or section_alignment == file_alignment
    )
    return (
        image_base % 0x10000 == 0
        and dll_characteristics & 0x000F == 0
        and file_alignment_valid
        and section_alignment >= file_alignment
        and low_alignment_valid
    )


def _pe_data_directories_are_valid(
    optional: bytes,
    file_size: int,
) -> bool:
    """Return whether declared PE data-directory ranges are bounded."""
    image_size = int.from_bytes(optional[56:60], "little")
    directory_count = int.from_bytes(optional[108:112], "little")
    for index in range(directory_count):
        start = 112 + (8 * index)
        address = int.from_bytes(optional[start : start + 4], "little")
        size = int.from_bytes(optional[start + 4 : start + 8], "little")
        if index in {7, 15} and (address != 0 or size != 0):
            return False
        if index == 8 and size != 0:
            return False
        if size == 0:
            continue
        if address == 0:
            return False
        if index == 4 and (address % 8 != 0 or size % 8 != 0):
            return False
        limit = file_size if index == 4 else image_size
        if address > limit or size > limit - address:
            return False
    return True


def _pe_loader_fields_are_valid(
    optional: bytes,
    *,
    header_end: int,
    file_size: int,
) -> bool:
    """Return whether bounded PE32+ loader metadata is structurally valid."""
    section_alignment = int.from_bytes(optional[32:36], "little")
    file_alignment = int.from_bytes(optional[36:40], "little")
    win32_version = int.from_bytes(optional[52:56], "little")
    image_size = int.from_bytes(optional[56:60], "little")
    header_size = int.from_bytes(optional[60:64], "little")
    loader_flags = int.from_bytes(optional[104:108], "little")
    directory_count = int.from_bytes(optional[108:112], "little")
    directory_capacity = (len(optional) - 112) // 8
    expected_header_size = (
        (header_end + file_alignment - 1) // file_alignment * file_alignment
    )
    return (
        win32_version == 0
        and loader_flags == 0
        and image_size <= 0x80000000
        and image_size % section_alignment == 0
        and header_size == expected_header_size
        and header_size <= file_size
        and directory_count <= directory_capacity
        and _pe_data_directories_are_valid(optional, file_size)
    )


def _pe_raw_offset_matches_virtual_address(
    raw_size: int,
    raw_offset: int,
    virtual_address: int,
    section_alignment: int,
) -> bool:
    """Return whether low-alignment image bytes use their RVA offset."""
    return (
        raw_size == 0
        or section_alignment >= 0x1000
        or raw_offset == virtual_address
    )


def _pe_raw_section_is_valid(
    raw_size: int,
    raw_offset: int,
    file_size: int,
    *,
    file_alignment: int,
    header_size: int,
    previous_raw_end: int | None,
) -> bool:
    """Return whether one PE section's optional raw range is well formed."""
    if raw_size == 0:
        return True
    aligned = (
        raw_size % file_alignment == 0 and raw_offset % file_alignment == 0
    )
    bounded = raw_offset <= file_size and raw_size <= file_size - raw_offset
    after_headers = raw_offset >= header_size
    ordered = previous_raw_end is None or raw_offset >= previous_raw_end
    return aligned and bounded and after_headers and ordered


def _pe_section_image_metadata_is_valid(section: bytes) -> bool:
    """Return whether one image section omits object-only COFF metadata."""
    characteristics = int.from_bytes(section[36:40], "little")
    object_only_flags = 0x01F01A08
    return (
        b"$" not in section[:8]
        and int.from_bytes(section[24:28], "little") == 0
        and int.from_bytes(section[28:32], "little") == 0
        and int.from_bytes(section[32:34], "little") == 0
        and int.from_bytes(section[34:36], "little") == 0
        and characteristics & object_only_flags == 0
    )


def _pe_sections_contain_entrypoint(
    stream: object,
    section_count: int,
    file_size: int,
    entrypoint: int,
    policy: _PeSectionPolicy,
) -> bool:
    """Require an executable section containing the program entrypoint."""
    admitted = False
    expected_virtual_address: int | None = None
    previous_raw_end: int | None = None
    for _ in range(section_count):
        section = stream.read(40)
        if (
            len(section) != 40
            or not _pe_section_image_metadata_is_valid(section)
        ):
            return False
        virtual_size = int.from_bytes(section[8:12], "little")
        virtual_address = int.from_bytes(section[12:16], "little")
        if (
            virtual_address < policy.header_size
            or virtual_address % policy.section_alignment != 0
        ):
            return False
        if (
            expected_virtual_address is not None
            and virtual_address != expected_virtual_address
        ):
            return False
        raw_size = int.from_bytes(section[16:20], "little")
        raw_offset = int.from_bytes(section[20:24], "little")
        characteristics = int.from_bytes(section[36:40], "little")
        if not _pe_raw_offset_matches_virtual_address(
            raw_size,
            raw_offset,
            virtual_address,
            policy.section_alignment,
        ) or not _pe_raw_section_is_valid(
            raw_size,
            raw_offset,
            file_size,
            file_alignment=policy.file_alignment,
            header_size=policy.header_size,
            previous_raw_end=previous_raw_end,
        ):
            return False
        if raw_size:
            previous_raw_end = raw_offset + raw_size
        mapped_size = max(virtual_size, raw_size)
        if mapped_size > 0xFFFFFFFF - virtual_address:
            return False
        virtual_end = virtual_address + mapped_size
        expected_virtual_address = (
            (virtual_end + policy.section_alignment - 1)
            // policy.section_alignment
            * policy.section_alignment
        )
        mapped_file_size = min(virtual_size, raw_size)
        if (
            characteristics & 0x20000000
            and mapped_file_size
            and virtual_address
            <= entrypoint
            < virtual_address + mapped_file_size
        ):
            admitted = True
    return (
        entrypoint != 0
        and admitted
        and expected_virtual_address == policy.image_size
    )


def _matches_pe(
    stream: object,
    prefix: bytes,
    architecture: str,
    file_size: int,
) -> bool:
    """Return whether one bounded executable PE32+ image matches target."""
    if prefix[:2] != b"MZ":
        return False
    stream.seek(0x3C)
    offset_bytes = stream.read(4)
    offset = int.from_bytes(offset_bytes, "little")
    if len(offset_bytes) != 4 or offset < 64 or offset > file_size:
        return False
    stream.seek(offset)
    signature = stream.read(4)
    coff = stream.read(20)
    expected = _PE_MACHINES.get(architecture)
    if signature != b"PE\0\0" or len(coff) != 20 or expected is None:
        return False
    layout = _pe_optional_layout(coff, expected, offset, file_size)
    if layout is None:
        return False
    optional_size, section_count = layout
    optional = stream.read(optional_size)
    header_end = offset + 24 + optional_size + (40 * section_count)
    if (
        len(optional) != optional_size
        or optional[:2] != bytes.fromhex("0b02")
        or not _pe_optional_fields_are_valid(optional)
        or not _pe_loader_fields_are_valid(
            optional,
            header_end=header_end,
            file_size=file_size,
        )
    ):
        return False
    entrypoint = int.from_bytes(optional[16:20], "little")
    policy = _PeSectionPolicy(
        section_alignment=int.from_bytes(optional[32:36], "little"),
        file_alignment=int.from_bytes(optional[36:40], "little"),
        image_size=int.from_bytes(optional[56:60], "little"),
        header_size=int.from_bytes(optional[60:64], "little"),
    )
    return _pe_sections_contain_entrypoint(
        stream,
        section_count,
        file_size,
        entrypoint,
        policy,
    )


def _macho_thread_state_entrypoint(
    body: bytes,
    byte_order: str,
) -> int | None:
    """Return the valid ARM64 legacy thread program counter when present."""
    cursor = 0
    entrypoint: int | None = None
    while cursor < len(body):
        if len(body) - cursor < 8:
            return None
        flavor = int.from_bytes(body[cursor : cursor + 4], byte_order)
        count = int.from_bytes(body[cursor + 4 : cursor + 8], byte_order)
        cursor += 8
        state_size = count * 4
        if state_size > len(body) - cursor:
            return None
        state = body[cursor : cursor + state_size]
        if flavor == 6:
            if count != 68 or entrypoint is not None:
                return None
            pc = int.from_bytes(state[256:264], byte_order)
            if pc == 0:
                return None
            entrypoint = pc
        cursor += state_size
    return entrypoint


def _macho_section_relocations_are_bounded(
    section: bytes,
    byte_order: str,
    file_size: int,
) -> bool:
    """Return whether one section relocation table stays inside the file."""
    relocation_offset = int.from_bytes(section[56:60], byte_order)
    relocation_count = int.from_bytes(section[60:64], byte_order)
    return (
        relocation_offset <= file_size
        and relocation_count * 8 <= file_size - relocation_offset
    )


def _macho_sections_fit_segment(
    body: bytes,
    section_count: int,
    byte_order: str,
    virtual_range: tuple[int, int],
    file_range: tuple[int, int, int],
) -> bool:
    """Return whether sections remain inside their owning segment ranges."""
    virtual_address, virtual_size = virtual_range
    file_offset, mapped_file_size, file_size = file_range
    segment_virtual_end = virtual_address + virtual_size
    segment_file_end = file_offset + mapped_file_size
    for index in range(section_count):
        start = 64 + (80 * index)
        section = body[start : start + 80]
        if section[16:32] != body[:16]:
            return False
        section_address = int.from_bytes(section[32:40], byte_order)
        section_size = int.from_bytes(section[40:48], byte_order)
        section_offset = int.from_bytes(section[48:52], byte_order)
        alignment_power = int.from_bytes(section[52:56], byte_order)
        section_type = int.from_bytes(section[64:68], byte_order) & 0xFF
        zero_fill = section_type in {0x1, 0xC, 0x12}
        if (
            alignment_power >= 64
            or section_address % (1 << alignment_power) != 0
            or section_address < virtual_address
            or section_address > segment_virtual_end
            or section_size > segment_virtual_end - section_address
        ):
            return False
        if not _macho_section_relocations_are_bounded(
            section,
            byte_order,
            file_size,
        ):
            return False
        if zero_fill:
            if section_offset != 0:
                return False
        elif section_size and (
            section_offset == 0
            or section_offset < file_offset
            or section_offset > segment_file_end
            or section_size > segment_file_end - section_offset
        ):
            return False
    return True


def _macho_segment64(
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> _MachOSegment | None:
    """Return one bounded 64-bit segment mapping."""
    if command_size < 72 or len(body) < 64:
        return None
    section_count = int.from_bytes(body[56:60], byte_order)
    if command_size != 72 + (80 * section_count):
        return None
    virtual_address = int.from_bytes(body[16:24], byte_order)
    virtual_size = int.from_bytes(body[24:32], byte_order)
    file_offset = int.from_bytes(body[32:40], byte_order)
    mapped_file_size = int.from_bytes(body[40:48], byte_order)
    maximum_protection = int.from_bytes(body[48:52], byte_order)
    initial_protection = int.from_bytes(body[52:56], byte_order)
    invalid_protection_bits = (
        maximum_protection | initial_protection
    ) & ~0x7
    unreadable_mapping = initial_protection != 0 and not (
        initial_protection & 0x1
    )
    if (
        invalid_protection_bits
        or unreadable_mapping
        or initial_protection & ~maximum_protection
        or mapped_file_size > virtual_size
    ):
        return None
    if (
        file_offset > file_size
        or mapped_file_size > file_size - file_offset
    ):
        return None
    if (
        virtual_size > ((1 << 64) - 1) - virtual_address
        or not _macho_sections_fit_segment(
            body,
            section_count,
            byte_order,
            (virtual_address, virtual_size),
            (file_offset, mapped_file_size, file_size),
        )
    ):
        return None
    return _MachOSegment(
        name=body[:16],
        virtual_address=virtual_address,
        virtual_size=virtual_size,
        file_offset=file_offset,
        file_size=mapped_file_size,
        initial_protection=initial_protection,
    )


def _macho_entrypoint_is_executable(
    entrypoint: int,
    segments: list[_MachOSegment],
) -> bool:
    """Return whether an entrypoint is inside executable file-backed bytes."""
    return any(
        segment.initial_protection & 0x4
        and segment.virtual_address
        <= entrypoint
        < segment.virtual_address + segment.file_size
        for segment in segments
    )


def _macho_read_command(
    stream: object,
    byte_order: str,
    remaining: int,
) -> tuple[int, int, bytes] | None:
    """Read one bounded and aligned Mach-O load command."""
    if remaining < 8:
        return None
    header = stream.read(8)
    if len(header) != 8:
        return None
    command = int.from_bytes(header[:4], byte_order)
    command_size = int.from_bytes(header[4:8], byte_order)
    if (
        command_size < 8
        or command_size % 8 != 0
        or command_size > remaining
    ):
        return None
    body = stream.read(command_size - 8)
    if len(body) != command_size - 8:
        return None
    return command, command_size, body


def _macho_build_platform(
    body: bytes,
    command_size: int,
    byte_order: str,
) -> int | None:
    """Return one structurally valid LC_BUILD_VERSION platform id."""
    if command_size < 24 or len(body) < 16:
        return None
    tool_count = int.from_bytes(body[12:16], byte_order)
    if command_size != 24 + (8 * tool_count):
        return None
    return int.from_bytes(body[:4], byte_order)


def _macho_lc_string_is_valid(
    body: bytes,
    command_size: int,
    byte_order: str,
    minimum_offset: int,
) -> bool:
    """Return whether one lc_str points to a bounded terminated string."""
    if command_size < minimum_offset or len(body) != command_size - 8:
        return False
    string_offset = int.from_bytes(body[:4], byte_order)
    if string_offset < minimum_offset or string_offset >= command_size:
        return False
    return b"\0" in body[string_offset - 8 :]


def _macho_dylinker_is_valid(
    body: bytes,
    command_size: int,
    byte_order: str,
) -> bool:
    """Return whether LC_LOAD_DYLINKER carries one bounded nonempty path."""
    if command_size < 16 or len(body) != command_size - 8:
        return False
    name_offset = int.from_bytes(body[:4], byte_order)
    if name_offset < 12 or name_offset >= command_size:
        return False
    path = body[name_offset - 8 :]
    terminator = path.find(b"\0")
    return terminator > 0


def _macho_dyld_info_ranges(
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> tuple[tuple[int, int], ...] | None:
    """Return the five bounded ranges from one dyld_info_command."""
    if command_size != 48 or len(body) != 40:
        return None
    ranges = tuple(
        (
            int.from_bytes(body[index : index + 4], byte_order),
            int.from_bytes(body[index + 4 : index + 8], byte_order),
        )
        for index in range(0, 40, 8)
    )
    if any(
        offset > file_size or size > file_size - offset
        for offset, size in ranges
    ):
        return None
    return ranges


def _macho_dysymtab_symbol_groups(
    body: bytes,
    byte_order: str,
) -> tuple[tuple[int, int], ...]:
    """Return local, external, and undefined LC_DYSYMTAB symbol groups."""
    fields = tuple(
        int.from_bytes(body[index : index + 4], byte_order)
        for index in range(0, 24, 4)
    )
    return (
        (fields[0], fields[1]),
        (fields[2], fields[3]),
        (fields[4], fields[5]),
    )


def _macho_dysymtab_ranges(
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> tuple[tuple[int, int], ...] | None:
    """Return bounded file tables from one LC_DYSYMTAB command."""
    if command_size != 80 or len(body) != 72:
        return None
    fields = tuple(
        int.from_bytes(body[index : index + 4], byte_order)
        for index in range(0, 72, 4)
    )
    ranges = (
        (fields[6], fields[7] * 8),
        (fields[8], fields[9] * 56),
        (fields[10], fields[11] * 4),
        (fields[12], fields[13] * 4),
        (fields[14], fields[15] * 8),
        (fields[16], fields[17] * 8),
    )
    if any(
        offset > file_size or size > file_size - offset
        for offset, size in ranges
    ):
        return None
    return ranges


def _macho_symtab_ranges(
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> tuple[tuple[int, int], ...] | None:
    """Return bounded symbol/string ranges from one LC_SYMTAB."""
    if command_size != 24 or len(body) != 16:
        return None
    symbol_offset = int.from_bytes(body[:4], byte_order)
    symbol_count = int.from_bytes(body[4:8], byte_order)
    string_offset = int.from_bytes(body[8:12], byte_order)
    string_size = int.from_bytes(body[12:16], byte_order)
    symbol_size = symbol_count * 16
    ranges = (
        (symbol_offset, symbol_size),
        (string_offset, string_size),
    )
    if symbol_offset + symbol_size > string_offset:
        return None
    if any(
        offset > file_size or size > file_size - offset
        for offset, size in ranges
    ):
        return None
    return ranges


def _macho_linkedit_data_range(
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> tuple[int, int] | None:
    """Return one file-bounded link-edit data range."""
    if command_size != 16 or len(body) != 8:
        return None
    data_offset = int.from_bytes(body[:4], byte_order)
    data_size = int.from_bytes(body[4:8], byte_order)
    if data_offset > file_size or data_size > file_size - data_offset:
        return None
    return data_offset, data_size


def _macho_linkedit_data_is_valid(
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> bool:
    """Return whether one link-edit data command stays inside the file."""
    return (
        _macho_linkedit_data_range(
            body,
            command_size,
            byte_order,
            file_size,
        )
        is not None
    )


def _macho_encryption_is_valid(
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> bool:
    """Return whether one 64-bit encryption range stays inside the file."""
    if command_size != 24 or len(body) != 16:
        return False
    crypt_offset = int.from_bytes(body[:4], byte_order)
    crypt_size = int.from_bytes(body[4:8], byte_order)
    return crypt_offset <= file_size and crypt_size <= file_size - crypt_offset


def _macho_note_is_valid(
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> bool:
    """Return whether one LC_NOTE payload range stays inside the file."""
    if command_size != 40 or len(body) != 32:
        return False
    note_offset = int.from_bytes(body[16:24], byte_order)
    note_size = int.from_bytes(body[24:32], byte_order)
    return note_offset <= file_size and note_size <= file_size - note_offset


def _macho_fixed_auxiliary(
    command: int,
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> _MachOAuxiliary | None:
    """Parse fixed-layout non-segment commands used by admission."""
    result: _MachOAuxiliary | None = None
    if command == 0x1B and command_size == 24:
        result = _MachOAuxiliary("uuid")
    elif command == 0x2C and _macho_encryption_is_valid(
        body,
        command_size,
        byte_order,
        file_size,
    ):
        result = _MachOAuxiliary("encryption")
    elif command == 0x31 and _macho_note_is_valid(
        body,
        command_size,
        byte_order,
        file_size,
    ):
        result = _MachOAuxiliary("note")
    return result


def _macho_linkedit_auxiliary(
    command: int,
    body: bytes,
    command_size: int,
    byte_order: str,
    file_size: int,
) -> _MachOAuxiliary | None:
    """Parse one command whose payload belongs in __LINKEDIT."""
    if command in _MACHO_DYLD_INFO_COMMANDS:
        ranges = _macho_dyld_info_ranges(
            body,
            command_size,
            byte_order,
            file_size,
        )
        return (
            None
            if ranges is None
            else _MachOAuxiliary("dyld-info", ranges=ranges)
        )
    if command == 0xB:
        ranges = _macho_dysymtab_ranges(
            body,
            command_size,
            byte_order,
            file_size,
        )
        return (
            None
            if ranges is None
            else _MachOAuxiliary(
                "dysymtab",
                ranges=ranges,
                symbol_groups=_macho_dysymtab_symbol_groups(body, byte_order),
            )
        )
    if command == 0x2:
        ranges = _macho_symtab_ranges(
            body,
            command_size,
            byte_order,
            file_size,
        )
        return (
            None
            if ranges is None
            else _MachOAuxiliary(
                "symtab",
                size=int.from_bytes(body[4:8], byte_order),
                ranges=ranges,
            )
        )
    data_range = _macho_linkedit_data_range(
        body,
        command_size,
        byte_order,
        file_size,
    )
    kind = (
        "code-signature"
        if command == 0x1D
        else f"linkedit-{command:x}"
    )
    return (
        None
        if data_range is None
        else _MachOAuxiliary(kind, ranges=(data_range,))
    )


def _macho_platform_command(
    command: int,
    command_size: int,
    body: bytes,
    byte_order: str,
) -> _MachOAuxiliary | None:
    """Parse one modern ARM64 platform command or reject a legacy form."""
    if command != 0x32:
        return None
    platform = _macho_build_platform(body, command_size, byte_order)
    return None if platform is None else _MachOAuxiliary("platform", platform)


def _macho_auxiliary_command(
    command: int,
    command_size: int,
    body: bytes,
    byte_order: str,
    file_size: int,
) -> _MachOAuxiliary | None:
    """Parse one non-segment Mach-O command used by admission."""
    result: _MachOAuxiliary | None = _MachOAuxiliary("ignored")
    if command in {0xD, 0xF, 0x1B, 0x21, 0x2C, 0x2D, 0x2E, 0x31}:
        result = _macho_fixed_auxiliary(
            command,
            body,
            command_size,
            byte_order,
            file_size,
        )
    elif command in _MACHO_DYLIB_LOAD_COMMANDS:
        if not _macho_lc_string_is_valid(
            body,
            command_size,
            byte_order,
            24,
        ):
            result = None
    elif command == 0x8000001C:
        if not _macho_lc_string_is_valid(
            body,
            command_size,
            byte_order,
            12,
        ):
            result = None
    elif command in {0x2, 0xB} or command in (
        _MACHO_LINKEDIT_DATA_COMMANDS | _MACHO_DYLD_INFO_COMMANDS
    ):
        result = _macho_linkedit_auxiliary(
            command,
            body,
            command_size,
            byte_order,
            file_size,
        )
    elif command == 0xE:
        result = (
            _MachOAuxiliary("dylinker")
            if _macho_dylinker_is_valid(body, command_size, byte_order)
            else None
        )
    elif command in {0x24, 0x25, 0x2F, 0x30, 0x32}:
        result = _macho_platform_command(
            command,
            command_size,
            body,
            byte_order,
        )
    elif command == 0x80000028:
        result = (
            _MachOAuxiliary("main", int.from_bytes(body[:8], byte_order))
            if command_size == 24
            else None
        )
    elif command == 0x5:
        entrypoint = _macho_thread_state_entrypoint(body, byte_order)
        result = (
            None
            if entrypoint is None
            else _MachOAuxiliary("thread", entrypoint)
        )
    return result


def _macho_auxiliary_metadata_is_valid(
    auxiliaries: Sequence[_MachOAuxiliary],
) -> bool:
    """Return whether singleton and companion Mach-O metadata is valid."""
    singleton_kinds = [
        item.kind
        for item in auxiliaries
        if item.kind
        in {
            "uuid",
            "dyld-info",
            "encryption",
            "symtab",
            "dysymtab",
            "code-signature",
            "dylinker",
        }
        or item.kind.startswith("linkedit-")
    ]
    kinds = set(singleton_kinds)
    if len(singleton_kinds) != len(kinds):
        return False
    if "dysymtab" not in kinds:
        return True
    if "symtab" not in kinds:
        return False
    symtab = next(item for item in auxiliaries if item.kind == "symtab")
    dysymtab = next(item for item in auxiliaries if item.kind == "dysymtab")
    return all(
        start <= symtab.size
        and count <= symtab.size
        and start + count <= symtab.size
        for start, count in dysymtab.symbol_groups
    )


def _macho_command_evidence(
    stream: object,
    byte_order: str,
    command_count: int,
    command_bytes: int,
    file_size: int,
) -> tuple[
    list[_MachOSegment],
    list[tuple[str, int]],
    bool,
    list[int],
    list[tuple[int, int]],
] | None:
    """Collect bounded segment, entrypoint, linker, and platform evidence."""
    remaining = command_bytes
    segments: list[_MachOSegment] = []
    auxiliaries: list[_MachOAuxiliary] = []
    for _ in range(command_count):
        record = _macho_read_command(stream, byte_order, remaining)
        if record is None:
            return None
        command, command_size, command_body = record
        if command in {0x1, 0x19}:
            segment = (
                None
                if command == 0x1
                else _macho_segment64(
                    command_body,
                    command_size,
                    byte_order,
                    file_size,
                )
            )
            if segment is None:
                return None
            segments.append(segment)
        else:
            auxiliary = _macho_auxiliary_command(
                command,
                command_size,
                command_body,
                byte_order,
                file_size,
            )
            if auxiliary is None:
                return None
            auxiliaries.append(auxiliary)
        remaining -= command_size
    if remaining != 0 or not _macho_auxiliary_metadata_is_valid(auxiliaries):
        return None
    entrypoints = [
        (item.kind, item.value)
        for item in auxiliaries
        if item.kind in {"main", "thread"}
    ]
    platforms = [item.value for item in auxiliaries if item.kind == "platform"]
    linkedit_ranges = [
        data_range
        for item in auxiliaries
        for data_range in item.ranges
        if data_range[1]
    ]
    has_dynamic_linker = any(item.kind == "dylinker" for item in auxiliaries)
    return (
        segments,
        entrypoints,
        has_dynamic_linker,
        platforms,
        linkedit_ranges,
    )


def _macho_ranges_are_disjoint(
    ranges: Sequence[tuple[int, int]],
) -> bool:
    """Return whether positive-size Mach-O ranges are pairwise disjoint."""
    positive = sorted(
        (start, start + size) for start, size in ranges if size > 0
    )
    return all(
        previous_end <= current_start
        for (_, previous_end), (current_start, _) in pairwise(positive)
    )


def _macho_segments_are_disjoint(segments: Sequence[_MachOSegment]) -> bool:
    """Return whether Mach-O segment VM and file mappings do not overlap."""
    virtual_ranges = [
        (segment.virtual_address, segment.virtual_size) for segment in segments
    ]
    file_ranges = [
        (segment.file_offset, segment.file_size) for segment in segments
    ]
    virtual_disjoint = _macho_ranges_are_disjoint(virtual_ranges)
    file_disjoint = _macho_ranges_are_disjoint(file_ranges)
    return virtual_disjoint and file_disjoint


def _macho_segments_follow_layout_order(
    segments: Sequence[_MachOSegment],
) -> bool:
    """Return whether load-command order follows dyld VM/file layout."""
    for index, earlier in enumerate(segments):
        earlier_name = earlier.name.split(b"\0", 1)[0]
        for later in segments[index + 1 :]:
            later_name = later.name.split(b"\0", 1)[0]
            if b"__DWARF" in {earlier_name, later_name}:
                continue
            if earlier.virtual_address > later.virtual_address:
                return False
            if (
                earlier.file_offset
                and later.file_offset
                and earlier.file_offset > later.file_offset
            ):
                return False
    return True


def _macho_linkedit_is_last_file_segment(
    segments: Sequence[_MachOSegment],
) -> bool:
    """Return whether __LINKEDIT has the greatest segment file offset."""
    linkedit = [
        segment
        for segment in segments
        if segment.name.split(b"\0", 1)[0] == b"__LINKEDIT"
    ]
    return (
        len(linkedit) == 1
        and linkedit[0].file_offset
        == max(segment.file_offset for segment in segments)
    )


def _macho_linkedit_ranges_fit_segment(
    segments: Sequence[_MachOSegment],
    ranges: Sequence[tuple[int, int]],
) -> bool:
    """Return whether positive link-edit payloads stay in __LINKEDIT."""
    linkedit = [
        segment
        for segment in segments
        if segment.name.split(b"\0", 1)[0] == b"__LINKEDIT"
    ]
    if len(linkedit) != 1:
        return False
    start = linkedit[0].file_offset
    size = linkedit[0].file_size
    return _macho_ranges_are_disjoint(ranges) and all(
        start <= offset <= start + size
        and payload_size <= start + size - offset
        for offset, payload_size in ranges
    )


def _macho_entrypoint_matches_segments(
    segments: list[_MachOSegment],
    entrypoints: list[tuple[str, int]],
    command_bytes: int,
) -> bool:
    """Bind one Mach-O process entry command to executable segment memory."""
    if (
        len(entrypoints) != 1
        or not _macho_segments_are_disjoint(segments)
        or not _macho_segments_follow_layout_order(segments)
        or not _macho_linkedit_is_last_file_segment(segments)
    ):
        return False
    text_segments = [
        segment
        for segment in segments
        if segment.name.split(b"\0", 1)[0] == b"__TEXT"
    ]
    linkedit_segments = [
        segment
        for segment in segments
        if segment.name.split(b"\0", 1)[0] == b"__LINKEDIT"
    ]
    if len(text_segments) != 1 or len(linkedit_segments) != 1:
        return False
    text_segment = text_segments[0]
    linkedit_segment = linkedit_segments[0]
    if (
        text_segment.file_offset != 0
        or text_segment.initial_protection != 0x5
        or linkedit_segment.initial_protection != 0x1
        or 32 + command_bytes > text_segment.file_size
    ):
        return False
    kind, entrypoint = entrypoints[0]
    command_end = 32 + command_bytes
    if kind == "main":
        return bool(text_segment.initial_protection & 0x4) and (
            command_end <= entrypoint < text_segment.file_size
        )
    text_command_end = text_segment.virtual_address + command_end
    if text_segment.virtual_address <= entrypoint < text_command_end:
        return False
    return _macho_entrypoint_is_executable(entrypoint, segments)


def _macho_commands_have_entrypoint(
    stream: object,
    byte_order: str,
    command_count: int,
    command_bytes: int,
    file_size: int,
    *,
    expected_platform: int,
) -> bool:
    """Validate one Mach-O load-command table and executable entrypoint."""
    evidence = _macho_command_evidence(
        stream,
        byte_order,
        command_count,
        command_bytes,
        file_size,
    )
    if evidence is None:
        return False
    (
        segments,
        entrypoints,
        has_dynamic_linker,
        platforms,
        linkedit_ranges,
    ) = evidence
    return (
        has_dynamic_linker
        and platforms == [expected_platform]
        and _macho_linkedit_ranges_fit_segment(segments, linkedit_ranges)
        and _macho_entrypoint_matches_segments(
            segments,
            entrypoints,
            command_bytes,
        )
    )


def _matches_thin_macho(
    stream: object,
    byte_order: str,
    file_size: int,
    expected_platform: int,
    expected_cpu_subtype: int | None = None,
) -> bool:
    """Return whether one ARM64 Mach-O64 executable has an entrypoint."""
    header = stream.read(28)
    if len(header) != 28 or file_size < 32:
        return False
    cpu = int.from_bytes(header[:4], byte_order)
    cpu_subtype = int.from_bytes(header[4:8], byte_order)
    file_type = int.from_bytes(header[8:12], byte_order)
    command_count = int.from_bytes(header[12:16], byte_order)
    command_bytes = int.from_bytes(header[16:20], byte_order)
    reserved = int.from_bytes(header[24:28], byte_order)
    if (
        cpu != _MACHO_ARM64_CPU
        or file_type != 2
        or command_count == 0
        or reserved != 0
    ):
        return False
    if expected_cpu_subtype is not None and (
        cpu_subtype & ~_MACHO_CPU_SUBTYPE_MASK
        != expected_cpu_subtype & ~_MACHO_CPU_SUBTYPE_MASK
    ):
        return False
    if command_bytes == 0 or command_bytes > file_size - 32:
        return False
    return _macho_commands_have_entrypoint(
        stream,
        byte_order,
        command_count,
        command_bytes,
        file_size,
        expected_platform=expected_platform,
    )


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
    align_start = size_start + offset_width
    alignment_power = int.from_bytes(
        payload[align_start : align_start + 4],
        byte_order,
    )
    if (
        entry_size == 32
        and int.from_bytes(payload[24:28], byte_order) != 0
    ):
        return None
    if (
        size == 0
        or offset < table_end
        or offset > file_size
        or size > file_size - offset
    ):
        return None
    if (
        alignment_power >= offset_width * 8
        or offset % (1 << alignment_power) != 0
    ):
        return None
    return offset, size


def _fat_macho_arm64_slice_is_native(
    stream: object,
    offset: int,
    size: int,
    cpu_subtype: int,
    expected_platform: int,
) -> bool:
    """Return whether one fat ARM64 slice is a bounded executable image."""
    try:
        stream.seek(offset)
    except (OSError, ValueError):
        return False
    prefix = stream.read(4)
    thin_order = _MACHO_THIN_ENDIAN.get(prefix)
    return thin_order is not None and _matches_thin_macho(
        stream,
        thin_order,
        size,
        expected_platform,
        cpu_subtype,
    )


def _fat_macho_slices_are_disjoint(
    slices: Sequence[tuple[int, int]],
) -> bool:
    """Return whether declared universal Mach-O slice ranges are disjoint."""
    ordered = sorted((offset, offset + size) for offset, size in slices)
    return all(
        previous_end <= current_start
        for (_, previous_end), (current_start, _) in pairwise(ordered)
    )


def _fat_macho_contains_arm64(
    stream: object,
    byte_order: str,
    entry_size: int,
    file_size: int,
    expected_platform: int,
) -> bool:
    """Return whether one bounded universal Mach-O contains an ARM64 slice."""
    count_bytes = stream.read(4)
    if len(count_bytes) != 4:
        return False
    count = int.from_bytes(count_bytes, byte_order)
    if count == 0 or count > 64:
        return False
    table_end = 8 + (count * entry_size)
    slices: list[tuple[int, int]] = []
    arm64_slices: list[tuple[int, int, int]] = []
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
        slices.append((offset, size))
        cpu_type = int.from_bytes(cpu, byte_order)
        cpu_subtype = int.from_bytes(rest[:4], byte_order)
        if cpu_type == _MACHO_ARM64_CPU:
            if size < 32 or offset % 0x4000 != 0:
                return False
            arm64_slices.append((offset, size, cpu_subtype))
    return (
        _fat_macho_slices_are_disjoint(slices)
        and bool(arm64_slices)
        and all(
            _fat_macho_arm64_slice_is_native(
                stream,
                offset,
                size,
                cpu_subtype,
                expected_platform,
            )
            for offset, size, cpu_subtype in arm64_slices
        )
    )


def _matches_macho(
    stream: object,
    prefix: bytes,
    system: str,
    architecture: str,
    file_size: int,
) -> bool:
    """Return whether one Mach-O header matches target platform and CPU."""
    expected_platform = _MACHO_PLATFORMS.get(system)
    if architecture != "arm64" or expected_platform is None:
        return False
    thin_order = _MACHO_THIN_ENDIAN.get(prefix)
    if thin_order is not None:
        return _matches_thin_macho(
            stream,
            thin_order,
            file_size,
            expected_platform,
        )
    fat = _MACHO_FAT.get(prefix)
    if fat is None:
        return False
    byte_order, entry_size = fat
    return _fat_macho_contains_arm64(
        stream,
        byte_order,
        entry_size,
        file_size,
        expected_platform,
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
        return _matches_elf(
            stream,
            prefix,
            architecture,
            file_size,
            require_entrypoint=True,
        )
    if system == "macos":
        return _matches_macho(
            stream,
            prefix,
            system,
            architecture,
            file_size,
        )
    if system == "windows":
        return _matches_pe(stream, prefix, architecture, file_size)
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


def _zip_member_path_is_safe(info: zipfile.ZipInfo) -> bool:
    """Return whether one archive member is a strict relative slash path."""
    name = info.filename
    drive_qualified = (
        len(name) >= 2
        and name[0].isascii()
        and name[0].isalpha()
        and name[1] == ":"
    )
    if not name or name.startswith("/") or "\\" in name or drive_qualified:
        return False
    logical = name[:-1] if info.is_dir() else name
    parts = logical.split("/")
    return bool(logical) and all(
        part not in {"", ".", ".."} for part in parts
    )


def _zip_member_is_regular_file(info: zipfile.ZipInfo) -> bool:
    """Return whether a ZIP member is not a declared Unix special node."""
    if info.is_dir():
        return False
    if info.create_system != 3:
        return True
    file_type = stat.S_IFMT(info.external_attr >> 16)
    return file_type in {0, stat.S_IFREG}


def _zip_inventory(
    archive: zipfile.ZipFile,
) -> dict[str, zipfile.ZipInfo] | None:
    """Return one duplicate-free inventory with a realizable member tree."""
    infos = archive.infolist()
    names = [info.filename for info in infos]
    if len(names) != len(set(names)):
        return None
    if not all(_zip_member_path_is_safe(info) for info in infos):
        return None
    logical = [
        info.filename[:-1] if info.is_dir() else info.filename
        for info in infos
    ]
    if len(logical) != len(set(logical)):
        return None
    files = {
        name
        for name, info in zip(logical, infos, strict=True)
        if not info.is_dir()
    }
    for name in logical:
        parts = name.split("/")
        ancestors = (
            "/".join(parts[:index]) for index in range(1, len(parts))
        )
        if any(ancestor in files for ancestor in ancestors):
            return None
    return {info.filename: info for info in infos}


def _zip_member_matches_elf(
    archive: zipfile.ZipFile,
    info: zipfile.ZipInfo,
    architecture: str,
) -> bool:
    """Return whether one ZIP member is an ELF for the selected CPU."""
    if not _zip_member_is_regular_file(info):
        return False
    with archive.open(info) as stream:
        prefix = stream.read(4)
        return prefix == b"\x7fELF" and _matches_elf(
            stream,
            prefix,
            architecture,
            info.file_size,
            require_shared_object=True,
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


def _android_xml_root_layout(payload: bytes) -> tuple[int, int] | None:
    """Return the bounded Android resource binary-XML root header and size."""
    if len(payload) < 8:
        return None
    root_type = int.from_bytes(payload[:2], "little")
    header_size = int.from_bytes(payload[2:4], "little")
    root_size = int.from_bytes(payload[4:8], "little")
    if (
        root_type != 0x0003
        or header_size != 8
        or root_size > len(payload)
        or root_size % 4 != 0
    ):
        return None
    return header_size, root_size


def _android_xml_chunk_layout(
    payload: bytes,
    cursor: int,
    root_size: int,
) -> tuple[int, int, int] | None:
    """Return one bounded and aligned Android resource chunk."""
    if root_size - cursor < 8:
        return None
    chunk_type = int.from_bytes(payload[cursor : cursor + 2], "little")
    header_size = int.from_bytes(payload[cursor + 2 : cursor + 4], "little")
    chunk_size = int.from_bytes(payload[cursor + 4 : cursor + 8], "little")
    if (
        header_size < 8
        or header_size % 4 != 0
        or chunk_size < header_size
        or chunk_size % 4 != 0
        or chunk_size > root_size - cursor
    ):
        return None
    return chunk_type, header_size, chunk_size


def _android_start_element_is_valid(
    payload: bytes,
    cursor: int,
    header_size: int,
    chunk_size: int,
) -> bool:
    """Return whether one Android start-element attribute area is bounded."""
    if chunk_size < header_size + 20:
        return False
    extension = cursor + header_size
    attribute_start = int.from_bytes(
        payload[extension + 8 : extension + 10],
        "little",
    )
    attribute_size = int.from_bytes(
        payload[extension + 10 : extension + 12],
        "little",
    )
    attribute_count = int.from_bytes(
        payload[extension + 12 : extension + 14],
        "little",
    )
    return (
        attribute_start + (attribute_size * attribute_count)
        <= chunk_size - header_size
    )


def _android_string_pool_is_valid(
    payload: bytes,
    cursor: int,
    header_size: int,
    chunk_size: int,
) -> bool:
    """Return whether one Android XML string-pool layout is bounded."""
    if header_size < 28:
        return False
    string_count = int.from_bytes(payload[cursor + 8 : cursor + 12], "little")
    style_count = int.from_bytes(payload[cursor + 12 : cursor + 16], "little")
    flags = int.from_bytes(payload[cursor + 16 : cursor + 20], "little")
    strings_start = int.from_bytes(payload[cursor + 20 : cursor + 24], "little")
    styles_start = int.from_bytes(payload[cursor + 24 : cursor + 28], "little")
    index_bytes = (string_count + style_count) * 4
    if index_bytes > chunk_size - header_size:
        return False
    strings_valid = True
    if string_count:
        string_end = styles_start if style_count else chunk_size
        terminator_size = 1 if flags & 0x100 else 2
        range_valid = (
            strings_start < chunk_size - 2
            and string_end > strings_start
            and string_end - strings_start >= terminator_size
        )
        terminator = payload[
            cursor + string_end - terminator_size : cursor + string_end
        ]
        strings_valid = range_valid and not any(terminator)
    styles_valid = True
    if style_count:
        range_valid = (
            styles_start < chunk_size
            and chunk_size - styles_start >= 12
        )
        style_end = cursor + chunk_size
        styles_valid = (
            range_valid
            and payload[style_end - 12 : style_end] == b"\xff" * 12
        )
    return strings_valid and styles_valid


def _android_string_pool_length8(
    data: bytes,
    cursor: int,
    limit: int,
) -> tuple[int, int] | None:
    """Decode one bounded Android UTF-8 string length field."""
    if cursor >= limit:
        return None
    first = data[cursor]
    cursor += 1
    if first & 0x80 == 0:
        return first, cursor
    if cursor >= limit:
        return None
    return ((first & 0x7F) << 8) | data[cursor], cursor + 1


def _android_string_pool_length16(
    data: bytes,
    cursor: int,
    limit: int,
) -> tuple[int, int] | None:
    """Decode one bounded Android UTF-16 string length field."""
    if limit - cursor < 2:
        return None
    first = int.from_bytes(data[cursor : cursor + 2], "little")
    cursor += 2
    if first & 0x8000 == 0:
        return first, cursor
    if limit - cursor < 2:
        return None
    second = int.from_bytes(data[cursor : cursor + 2], "little")
    return ((first & 0x7FFF) << 16) | second, cursor + 2


def _android_string_pool_entry_bounds(
    payload: bytes,
    cursor: int,
    chunk_size: int,
    index: int,
) -> tuple[int, int, bool] | None:
    """Return one bounded Android string entry and its encoding."""
    string_count = int.from_bytes(payload[cursor + 8 : cursor + 12], "little")
    if index >= string_count:
        return None
    flags = int.from_bytes(payload[cursor + 16 : cursor + 20], "little")
    strings_start = int.from_bytes(payload[cursor + 20 : cursor + 24], "little")
    styles_start = int.from_bytes(payload[cursor + 24 : cursor + 28], "little")
    entry = cursor + 28 + (index * 4)
    offset = int.from_bytes(payload[entry : entry + 4], "little")
    start = cursor + strings_start + offset
    limit = cursor + (styles_start or chunk_size)
    if start >= limit:
        return None
    return start, limit, bool(flags & 0x100)


def _android_utf8_pool_text(
    payload: bytes,
    start: int,
    limit: int,
) -> str | None:
    """Decode one bounded Android UTF-8 string-pool entry."""
    utf16_length = _android_string_pool_length8(payload, start, limit)
    if utf16_length is None:
        return None
    _, data_cursor = utf16_length
    byte_length = _android_string_pool_length8(payload, data_cursor, limit)
    if byte_length is None:
        return None
    length, data_cursor = byte_length
    if length >= limit - data_cursor:
        return None
    raw = payload[data_cursor : data_cursor + length]
    if payload[data_cursor + length] != 0:
        return None
    try:
        return raw.decode("utf-8")
    except UnicodeDecodeError:
        return None


def _android_utf16_pool_text(
    payload: bytes,
    start: int,
    limit: int,
) -> str | None:
    """Decode one bounded Android UTF-16 string-pool entry."""
    length_field = _android_string_pool_length16(payload, start, limit)
    if length_field is None:
        return None
    length, data_cursor = length_field
    byte_length = length * 2
    if byte_length + 2 > limit - data_cursor:
        return None
    raw = payload[data_cursor : data_cursor + byte_length]
    terminator = payload[
        data_cursor + byte_length : data_cursor + byte_length + 2
    ]
    if terminator != b"\0\0":
        return None
    try:
        return raw.decode("utf-16-le")
    except UnicodeDecodeError:
        return None


def _android_string_pool_text(
    payload: bytes,
    cursor: int,
    chunk_size: int,
    index: int,
) -> str | None:
    """Decode one referenced Android string-pool entry when bounded."""
    bounds = _android_string_pool_entry_bounds(
        payload,
        cursor,
        chunk_size,
        index,
    )
    if bounds is None:
        return None
    start, limit, utf8 = bounds
    decoder = _android_utf8_pool_text if utf8 else _android_utf16_pool_text
    return decoder(payload, start, limit)


def _android_package_name_is_valid(value: str) -> bool:
    """Return whether one Android package name matches the lite parser."""
    if value == "android":
        return True
    if not value or len(value) > 223 or "." not in value:
        return False
    for part in value.split("."):
        if not part or not part[0].isascii() or not part[0].isalpha():
            return False
        if not all(
            character.isascii()
            and (character.isalnum() or character == "_")
            for character in part[1:]
        ):
            return False
    return True


def _android_manifest_raw_attribute(
    payload: bytes,
    cursor: int,
    header_size: int,
    string_pool: tuple[int, int],
    expected_name: str,
) -> tuple[bool, str | None]:
    """Return one bounded no-namespace raw manifest attribute."""
    extension = cursor + header_size
    attribute_start = int.from_bytes(
        payload[extension + 8 : extension + 10],
        "little",
    )
    attribute_size = int.from_bytes(
        payload[extension + 10 : extension + 12],
        "little",
    )
    attribute_count = int.from_bytes(
        payload[extension + 12 : extension + 14],
        "little",
    )
    if attribute_size < 20:
        return False, None
    pool_cursor, pool_size = string_pool
    for index in range(attribute_count):
        attribute = extension + attribute_start + (attribute_size * index)
        namespace = int.from_bytes(
            payload[attribute : attribute + 4],
            "little",
        )
        if namespace != 0xFFFFFFFF:
            continue
        name_index = int.from_bytes(
            payload[attribute + 4 : attribute + 8],
            "little",
        )
        name = _android_string_pool_text(
            payload,
            pool_cursor,
            pool_size,
            name_index,
        )
        if name != expected_name:
            continue
        raw_index = int.from_bytes(
            payload[attribute + 8 : attribute + 12],
            "little",
        )
        if raw_index == 0xFFFFFFFF:
            return True, None
        return True, _android_string_pool_text(
            payload,
            pool_cursor,
            pool_size,
            raw_index,
        )
    return False, None


def _android_root_element_is_valid(
    payload: bytes,
    cursor: int,
    header_size: int,
    string_pool: tuple[int, int],
) -> bool:
    """Return whether one first XML start element resolves to manifest."""
    extension = cursor + header_size
    name_index = int.from_bytes(
        payload[extension + 4 : extension + 8],
        "little",
    )
    pool_cursor, pool_size = string_pool
    if (
        _android_string_pool_text(
            payload,
            pool_cursor,
            pool_size,
            name_index,
        )
        != "manifest"
    ):
        return False
    package_found, package_name = _android_manifest_raw_attribute(
        payload,
        cursor,
        header_size,
        string_pool,
        "package",
    )
    if (
        not package_found
        or package_name is None
        or not _android_package_name_is_valid(package_name)
    ):
        return False
    split_found, split_name = _android_manifest_raw_attribute(
        payload,
        cursor,
        header_size,
        string_pool,
        "split",
    )
    return not split_found or (split_name is not None and not split_name)


def _android_xml_children_are_valid(
    payload: bytes,
    cursor: int,
    root_size: int,
) -> bool:
    """Return whether binary-XML children provide strings and a root tag."""
    string_pool: tuple[int, int] | None = None
    has_start_element = False
    while cursor < root_size:
        layout = _android_xml_chunk_layout(payload, cursor, root_size)
        if layout is None:
            return False
        chunk_type, header_size, chunk_size = layout
        if chunk_type == 0x0001:
            if not _android_string_pool_is_valid(
                payload,
                cursor,
                header_size,
                chunk_size,
            ):
                return False
            string_pool = (cursor, chunk_size)
        elif 0x0100 <= chunk_type <= 0x017F:
            if string_pool is None or header_size < 16:
                return False
            if chunk_type == 0x0102:
                if not _android_start_element_is_valid(
                    payload,
                    cursor,
                    header_size,
                    chunk_size,
                ):
                    return False
                root_matches = has_start_element or (
                    _android_root_element_is_valid(
                        payload,
                        cursor,
                        header_size,
                        string_pool,
                    )
                )
                if not root_matches:
                    return False
                has_start_element = True
        cursor += chunk_size
    return (
        cursor == root_size
        and string_pool is not None
        and has_start_element
    )


def _android_manifest_is_binary_xml(
    archive: zipfile.ZipFile,
    info: zipfile.ZipInfo,
) -> bool:
    """Return whether the APK manifest has bounded resource binary XML."""
    if not _zip_member_is_regular_file(info):
        return False
    payload = archive.read(info)
    layout = _android_xml_root_layout(payload)
    if layout is None:
        return False
    header_size, root_size = layout
    return _android_xml_children_are_valid(payload, header_size, root_size)


def _android_archive_contains_arm64(archive: zipfile.ZipFile) -> bool:
    """Return whether one opened APK contains an ARM64 native library."""
    inventory = _zip_inventory(archive)
    if inventory is None:
        return False
    manifest = inventory.get("AndroidManifest.xml")
    if manifest is None or not _android_manifest_is_binary_xml(
        archive,
        manifest,
    ):
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
                archive_is_intact = archive.testzip() is None
                matches = archive_is_intact and validator(archive)
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


def _ios_bundle_identifier_is_valid(value: object) -> bool:
    """Return whether one iOS bundle identifier uses Apple's character set."""
    return (
        isinstance(value, str)
        and bool(value)
        and all(
            character.isascii()
            and (character.isalnum() or character in {"-", "."})
            for character in value
        )
    )


def _ios_bundle_metadata_is_valid(document: dict[object, object]) -> bool:
    """Return whether iOS bundle identity metadata is internally consistent."""
    if not _ios_bundle_identifier_is_valid(document.get("CFBundleIdentifier")):
        return False
    package_type = document.get("CFBundlePackageType")
    return package_type is None or package_type == "APPL"


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
        and _zip_member_is_regular_file(info)
    ]
    if len(property_lists) != 1:
        return None
    plist_info = property_lists[0]
    document = plistlib.loads(archive.read(plist_info))
    if not isinstance(document, dict):
        return None
    if not _ios_bundle_metadata_is_valid(document):
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
    if binary_info is None or not _zip_member_is_regular_file(binary_info):
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
            "ios",
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
    mobile_packages = [
        item for item in candidate_files if item.suffix.casefold() == suffix
    ]
    if len(mobile_packages) == 1 and validator(mobile_packages[0]):
        return
    message = (
        f"candidate package has no valid {label}; expected exactly one: "
        f"{candidate}"
    )
    raise RunFailure(message)


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


def _validate_publication_candidate(
    candidate: Path,
    target: Target | None,
) -> _CandidateTreeSnapshot:
    """Revalidate and bind final candidate bytes before publication."""
    tree = _validate_candidate_tree(candidate)
    snapshot = _candidate_tree_snapshot(candidate, tree)
    if target is not None:
        _validate_candidate_artifact(candidate, target, tree.files)
    _require_candidate_tree_snapshot(candidate, snapshot)
    return snapshot


def _require_publication_destination_state(
    destination: Path,
    expected: tuple[int, int, int] | None,
) -> None:
    """Require the published target path to retain its pre-validation state."""
    if expected is None:
        if _path_present(destination):
            raise RunFailure(
                f"published target changed before publication: {destination}"
            )
        return
    _require_directory_identity(destination, "published target", expected)


def _publication_root_after_admission(
    publication_root: Path,
    expected: tuple[int, int, int] | None,
) -> tuple[int, int, int]:
    """Create a missing publication root or require its preflight identity."""
    if expected is None:
        if _path_present(publication_root):
            raise RunFailure(
                "publication root changed before publication: "
                f"{publication_root}"
            )
        publication_root.mkdir(parents=True)
        return _real_directory_identity(publication_root, "publication root")
    _require_directory_identity(
        publication_root,
        "publication root",
        expected,
    )
    return expected


def _remove_preflighted_publication_backup(
    backup: Path,
    expected: tuple[int, int, int] | None,
) -> None:
    """Remove only the stale publication backup identity seen at preflight."""
    if expected is None:
        if _path_present(backup):
            raise RunFailure(
                f"publication backup changed before publication: {backup}"
            )
        return
    _require_directory_identity(backup, "publication backup", expected)
    shutil.rmtree(backup)


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


def _publish(
    candidate: Path,
    destination: Path,
    target: Target | None = None,
) -> None:
    """Replace one published target without exposing a partial candidate."""
    tree = _validate_candidate_tree(candidate)
    if not tree.files:
        raise RunFailure(f"candidate package is empty: {candidate}")
    destination_identity = (
        _real_directory_identity(destination, "published target")
        if _path_present(destination)
        else None
    )
    publication_root = destination.parent
    publication_root_identity = (
        _real_directory_identity(publication_root, "publication root")
        if _path_present(publication_root)
        else None
    )
    backup = destination.with_name(f".{destination.name}.previous")
    backup_identity = (
        _real_directory_identity(backup, "publication backup")
        if _path_present(backup)
        else None
    )
    candidate_snapshot = _validate_publication_candidate(candidate, target)
    publication_identity = _publication_root_after_admission(
        publication_root,
        publication_root_identity,
    )
    _remove_preflighted_publication_backup(backup, backup_identity)
    _require_directory_identity(
        publication_root,
        "publication root",
        publication_identity,
    )
    _require_publication_destination_state(destination, destination_identity)
    _require_candidate_tree_snapshot(candidate, candidate_snapshot)
    had_previous = destination_identity is not None
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
    _publish(candidate, destination, target)
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


def _build_selected_targets(
    root: Path,
    engine_root: Path,
    project: Path,
    targets: list[Target],
    *,
    validate_only: bool,
) -> None:
    """Attach cached project state only for the selected build lifecycle."""
    _prepare_project_state(root, project)
    try:
        uat = _uat_path(engine_root)
        for target in targets:
            _build_target(
                root,
                uat,
                project,
                target,
                validate_only=validate_only,
            )
    except BaseException as error:
        try:
            _detach_project_state(root, project)
        except (OSError, RunFailure) as cleanup:
            raise RunFailure(f"{error}; {cleanup}") from error
        raise
    _detach_project_state(root, project)


def _raise_run_signal(signum: int, frame: object | None) -> None:
    """Convert one external termination signal into controlled cleanup."""
    del frame
    raise _RunSignal(signum)


def _install_run_signal_handlers(
) -> tuple[dict[int, object], set[signal.Signals] | None]:
    """Install and unblock signals ignored or masked by background shells."""
    previous: dict[int, object] = {}
    previous_mask: set[signal.Signals] | None = None
    watched = {signal.SIGINT, signal.SIGTERM}
    if os.name != "nt":
        previous_mask = signal.pthread_sigmask(signal.SIG_UNBLOCK, watched)
    for signum in watched:
        previous[signum] = signal.signal(signum, _raise_run_signal)
    return previous, previous_mask


def _restore_run_signal_handlers(
    previous: dict[int, object],
    previous_mask: set[signal.Signals] | None,
) -> None:
    """Restore caller signal dispositions and mask after main."""
    for signum, handler in previous.items():
        signal.signal(signum, handler)
    if previous_mask is not None:
        signal.pthread_sigmask(signal.SIG_SETMASK, previous_mask)


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
    lock: TextIO | None = None
    handlers, previous_mask = _install_run_signal_handlers()
    try:
        lock = _acquire_run_lock(root)
        arch_snapshot = _revalidate_arch(root, arch_path)
        targets = _selected_targets(arch_snapshot)
        check_snapshot = _revalidate_check(root, check_path)
        check = _check_evidence(check_snapshot)
        unreal = _require_unreal_evidence(check)
        engine_root = Path(str(unreal["root"])).resolve()
        project = _project_from_evidence(root, unreal)
        _build_selected_targets(
            root,
            engine_root,
            project,
            targets,
            validate_only=args.validate_only,
        )
    except _RunSignal as error:
        name = signal.Signals(error.signum).name
        print(f"run: interrupted by {name}", file=sys.stderr)
        return 128 + error.signum
    except (RunFailure, OSError) as error:
        print(f"run: {error}", file=sys.stderr)
        return 1
    finally:
        if lock is not None:
            lock.close()
        _restore_run_signal_handlers(handlers, previous_mask)
    if args.validate_only:
        print(f"run: validated {len(targets)} selected target(s)")
    else:
        print(f"run: published {len(targets)} selected target(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

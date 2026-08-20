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
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
from typing import NamedTuple

_ARCH_SCHEMA = "shar.build.arch.v1"
_CHECK_SCHEMA = "shar.build.check.v1"
_ARCH_PATH = Path(".cache/build/data/arch.json")
_CHECK_PATH = Path(".cache/build/data/check.json")
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


def _read_object(path: Path, label: str) -> dict[str, object]:
    """Read one required UTF-8 JSON object."""
    try:
        value = json.loads(
            path.read_text(encoding="utf-8"),
            object_pairs_hook=_unique_json_object,
        )
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise RunFailure(f"cannot read {label} {path}: {error}") from error
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


def _selected_targets(path: Path) -> list[Target]:
    """Validate the versioned architecture decision and return its targets."""
    value = _read_object(path, "architecture evidence")
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


def _check_evidence(path: Path) -> dict[str, object]:
    """Load saved preflight evidence after check.py has revalidated it."""
    value = _read_object(path, "check evidence")
    if value.get("schema") != _CHECK_SCHEMA:
        raise RunFailure(f"check schema must be {_CHECK_SCHEMA}")
    unreal = value.get("unreal")
    if not isinstance(unreal, dict):
        raise RunFailure("check evidence has no unreal object")
    for key in ("project", "root", "version"):
        if not isinstance(unreal.get(key), str) or not unreal.get(key):
            raise RunFailure(f"check evidence has invalid unreal.{key}")
    if unreal.get("version") != "5.8.1":
        raise RunFailure("check evidence must target Unreal Engine 5.8.1")
    return value


def _revalidate_arch(root: Path, arch_path: Path) -> None:
    """Invoke canonical arch.py revalidation before using saved targets."""
    command = [
        sys.executable,
        str(root / "tools" / "build" / "adapter-inbound" / "arch.py"),
        "--revalidate",
        "--output",
        str(arch_path),
    ]
    result = subprocess.run(command, cwd=root, check=False)
    if result.returncode:
        raise RunFailure("saved architecture decision did not revalidate")


def _revalidate_check(root: Path, check_path: Path) -> None:
    """Invoke the supported check.py revalidation before using saved paths."""
    command = [
        sys.executable,
        str(root / "tools" / "build" / "adapter-inbound" / "check.py"),
        "--revalidate",
        "--output",
        str(check_path),
    ]
    result = subprocess.run(command, cwd=root, check=False)
    if result.returncode:
        raise RunFailure("saved build preflight did not revalidate")


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


def _uat_path(engine_root: Path) -> Path:
    """Resolve the native RunUAT launcher for the current host."""
    batch = engine_root / "Engine" / "Build" / "BatchFiles"
    path = batch / "RunUAT.bat" if os.name == "nt" else batch / "RunUAT.sh"
    if not path.is_file():
        raise RunFailure(f"Unreal AutomationTool launcher is missing: {path}")
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
    if _path_present(log):
        _require_real_file(log, "UAT log")
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
    with log.open("w", encoding="utf-8", newline="\n") as handle:
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
    _require_real_file(report, "Turnkey SDK report")
    try:
        text = report.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        raise RunFailure(f"cannot read Turnkey SDK report: {report}") from error
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


def _has_payload(path: Path) -> bool:
    """Return whether a candidate archive contains at least one regular file."""
    return path.is_dir() and any(item.is_file() for item in path.rglob("*"))


def _validate_candidate_tree(candidate: Path) -> None:
    """Reject links and special entries without traversing outside candidate."""
    _require_real_directory(candidate, "candidate package")
    pending = [candidate]
    while pending:
        directory = pending.pop()
        for item in sorted(directory.iterdir(), key=lambda path: path.name):
            if _is_directory_link(item):
                raise RunFailure(
                    f"candidate package contains a linked entry: {item}"
                )
            if item.is_dir():
                pending.append(item)
                continue
            if item.is_file():
                if item.stat(follow_symlinks=False).st_nlink > 1:
                    raise RunFailure(
                        f"candidate package contains a hard-linked file: {item}"
                    )
                continue
            raise RunFailure(
                f"candidate package contains a special entry: {item}"
            )


def _is_shar_runtime_name(name: str) -> bool:
    """Return whether one native filename identifies the SHAR runtime."""
    normalized = name.casefold()
    return normalized == "shar" or (
        normalized.startswith("shar-")
        and normalized.endswith("-shipping")
    )


def _has_linux_runtime(candidate: Path) -> bool:
    """Return whether a Linux archive contains a non-empty SHAR binary."""
    return any(
        item.is_file()
        and item.stat().st_size > 0
        and _is_shar_runtime_name(item.name)
        for item in candidate.rglob("*")
    )


def _has_macos_runtime(candidate: Path) -> bool:
    """Return whether a macOS archive contains a runnable SHAR app bundle."""
    for bundle in candidate.rglob("*"):
        if not bundle.is_dir() or bundle.suffix.casefold() != ".app":
            continue
        executable_root = bundle / "Contents" / "MacOS"
        if not executable_root.is_dir():
            continue
        if any(
            item.is_file()
            and item.stat().st_size > 0
            and _is_shar_runtime_name(item.name)
            for item in executable_root.iterdir()
        ):
            return True
    return False


def _validate_candidate_artifact(candidate: Path, target: Target) -> None:
    """Require UAT archives to contain their declared runnable artifact."""
    if target.system == "linux":
        if _has_linux_runtime(candidate):
            return
        message = (
            "candidate package has no non-empty Linux SHAR executable: "
            f"{candidate}"
        )
        raise RunFailure(message)
    if target.system == "macos":
        if _has_macos_runtime(candidate):
            return
        message = (
            "candidate package has no runnable macOS SHAR app bundle: "
            f"{candidate}"
        )
        raise RunFailure(message)

    if target.system == "windows":
        if any(
            item.is_file()
            and item.suffix.casefold() == ".exe"
            and item.stat().st_size > 0
            and _is_shar_runtime_name(item.stem)
            for item in candidate.rglob("*")
        ):
            return
        message = (
            "candidate package has no non-empty Windows SHAR executable: "
            f"{candidate}"
        )
        raise RunFailure(message)

    expected = {
        "apk": (".apk", "Android APK"),
        "ipa": (".ipa", "iOS IPA"),
    }.get(target.artifact)
    if expected is None:
        return
    suffix, label = expected
    if any(
        item.is_file()
        and item.suffix.casefold() == suffix
        and item.stat().st_size > 0
        for item in candidate.rglob("*")
    ):
        return
    raise RunFailure(f"candidate package has no non-empty {label}: {candidate}")


def _cache_nonruntime_artifacts(
    candidate: Path,
    work: Path,
    target: Target,
) -> None:
    """Keep packaging metadata and debug symbols out of final dist output."""
    metadata = work / "publication-metadata"
    symbols = work / "symbols"
    _remove_real_directory_if_present(
        metadata,
        "publication metadata cache",
    )
    _remove_real_directory_if_present(symbols, "symbol cache")

    manifests = sorted(candidate.glob("Manifest_*.txt"))
    for source in manifests:
        if not source.is_file():
            raise RunFailure(
                f"packaging manifest must be a real file: {source}"
            )

    debug_files: list[Path] = []
    if target.system == "windows":
        debug_files = sorted(candidate.rglob("*.pdb"))
    for source in debug_files:
        if not source.is_file():
            raise RunFailure(f"debug symbol must be a real file: {source}")

    if manifests:
        metadata.mkdir(parents=True)
        for source in manifests:
            Path(source).replace(metadata / source.name)
    if debug_files:
        symbols.mkdir(parents=True)
        for source in debug_files:
            relative = source.relative_to(candidate)
            destination = symbols / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            Path(source).replace(destination)


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
    _validate_candidate_tree(candidate)
    if not _has_payload(candidate):
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
    _validate_candidate_tree(candidate)
    _cache_nonruntime_artifacts(candidate, work, target)
    _validate_candidate_artifact(candidate, target)
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
        _revalidate_arch(root, arch_path)
        targets = _selected_targets(arch_path)
        _revalidate_check(root, check_path)
        check = _check_evidence(check_path)
        unreal = _require_unreal_evidence(check)
        engine_root = Path(str(unreal["root"])).resolve()
        project = Path(str(unreal["project"])).resolve()
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

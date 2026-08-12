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
#   - Run tools/build/run.py after dependencies.py, check.py, and arch.py.
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
    return Path(__file__).resolve().parents[2]


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


def _revalidate_check(root: Path, check_path: Path) -> None:
    """Invoke the supported check.py revalidation before using saved paths."""
    command = [
        sys.executable,
        str(root / "tools" / "build" / "check.py"),
        "--revalidate",
        "--output",
        str(check_path),
    ]
    result = subprocess.run(command, cwd=root, check=False)
    if result.returncode:
        raise RunFailure("saved build preflight did not revalidate")


def _uat_path(engine_root: Path) -> Path:
    """Resolve the native RunUAT launcher for the current host."""
    batch = engine_root / "Engine" / "Build" / "BatchFiles"
    if os.name == "nt":
        path = batch / "RunUAT.bat"
    else:
        path = batch / "RunUAT.sh"
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
    log.parent.mkdir(parents=True, exist_ok=True)
    command = _uat_command(uat, arguments)
    automation_saved = log.parent / "automation-saved"
    automation_logs = automation_saved / "logs"
    automation_logs.mkdir(parents=True, exist_ok=True)
    environment = os.environ.copy()
    environment["uebp_EngineSavedFolder"] = str(automation_saved)
    environment["uebp_FinalLogFolder"] = str(automation_logs)
    environment["uebp_LogFolder"] = str(automation_logs)
    environment["UE-LocalDataCachePath"] = str(log.parent / "ddc")
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
    report.unlink(missing_ok=True)
    arguments = [
        "Turnkey",
        "-Command=VerifySdk",
        f"-Platform={target.unreal_platform}",
        "-Unattended",
        f"-ReportFilename={report}",
        f"-Project={project}",
    ]
    _run_uat(root, uat, arguments, log)
    if not report.is_file():
        raise RunFailure(f"Turnkey did not produce an SDK report: {report}")
    text = report.read_text(encoding="utf-8")
    expected = f"{target.unreal_platform}: (Status=Valid,"
    if expected not in text:
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


def _publish(candidate: Path, destination: Path) -> None:
    """Replace one published target without exposing a partial candidate."""
    if not _has_payload(candidate):
        raise RunFailure(f"candidate package is empty: {candidate}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    backup = destination.with_name(f".{destination.name}.previous")
    if backup.exists():
        shutil.rmtree(backup)
    had_previous = destination.exists()
    if had_previous:
        os.replace(destination, backup)
    try:
        os.replace(candidate, destination)
    except OSError:
        if had_previous and backup.exists() and not destination.exists():
            os.replace(backup, destination)
        raise
    if backup.exists():
        shutil.rmtree(backup)


def _build_target(
    root: Path,
    uat: Path,
    project: Path,
    target: Target,
    validate_only: bool,
) -> None:
    """Verify and optionally package one selected target transactionally."""
    work = root / _WORK_ROOT / target.identifier
    work.mkdir(parents=True, exist_ok=True)
    _verify_sdk(root, uat, project, target, work)
    if validate_only:
        print(f"run: {target.identifier}: SDK valid")
        return

    candidate = work / "candidate"
    staging = work / "stage"
    shutil.rmtree(candidate, ignore_errors=True)
    shutil.rmtree(staging, ignore_errors=True)
    candidate.mkdir(parents=True)
    staging.mkdir(parents=True)
    log = work / "build.log"
    arguments = _build_arguments(project, target, candidate, staging)
    _run_uat(root, uat, arguments, log)
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
        targets = _selected_targets(arch_path)
        _revalidate_check(root, check_path)
        check = _check_evidence(check_path)
        unreal = check["unreal"]
        if not isinstance(unreal, dict):
            raise RunFailure("check evidence has no unreal object")
        engine_root = Path(str(unreal["root"])).resolve()
        project = Path(str(unreal["project"])).resolve()
        uat = _uat_path(engine_root)
        for target in targets:
            _build_target(
                root,
                uat,
                project,
                target,
                args.validate_only,
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

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
#   - Read-only user build preflight and saved validation evidence.
# - Must-Not:
#   - Extract game data, compile validators, or mutate build inputs.
# - Allows:
#   - Inputs: game, Python, Unreal, host, and source-validator evidence.
#   - Outputs: versioned build preflight JSON and diagnostics.
#   - Side effects: atomically replaces only the selected check JSON.
# - Split-When:
#   - Split when one prerequisite family gains an independent lifecycle.
# - Merge-When:
#   - Merge when dependency bootstrap owns identical read-only checks.
# - Summary:
#   - Validates the supported local build prerequisites.
# - Description:
#   - Fails closed before saving paths used by later build stages.
# - Usage:
#   - Run tools/build/adapter-inbound/check.py after dependencies.py.
# - Defaults:
#   - Requires Python 3.14.6 and Unreal Engine 5.8.1 exactly.
#

"""Validate SHAR user build prerequisites and persist checked evidence."""

from __future__ import annotations

import argparse
from collections.abc import Iterator
from contextlib import AbstractContextManager
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import stat
import subprocess
import sys
from typing import NamedTuple

_SCHEMA = "shar.build.check.v1"
_PYTHON_VERSION = (3, 14, 6)
_UNREAL_VERSION = (5, 8, 1)
_UNREAL_ASSOCIATION = "5.8"
_DATA_PATH = Path(".cache/build/data/check.json")
_DEPENDENCIES_PATH = Path(".cache/build/data/dependencies.json")
_DEPENDENCIES_SCHEMA = "shar.build.dependencies.v1"
_VALIDATOR_SOURCE_INPUTS = (
    Path("Cargo.toml"),
    Path("Cargo.lock"),
    Path("src/migration/manifest"),
    Path("src/foundation/command-line"),
    Path("src/foundation/filesystem"),
)
_DEEP_VALIDATOR_SOURCE_INPUTS = (
    Path("Cargo.toml"),
    Path("Cargo.lock"),
    Path("src/migration/source-audit"),
    Path("src/formats/p3d"),
    Path("src/formats/rcf"),
    Path("src/formats/rmv"),
    Path("src/formats/rsd"),
    Path("src/foundation/command-line"),
    Path("src/foundation/filesystem"),
    Path("src/foundation/json-text"),
    Path("src/foundation/sha256"),
)
_PROJECT_PATH = Path("src/unreal/project/composition/uproject/shar.uproject")


class CheckFailure(RuntimeError):
    """One user-actionable preflight failure."""


class EngineEvidence(NamedTuple):
    """Validated Unreal installation evidence."""

    root: Path
    version: str


class ProjectEvidence(NamedTuple):
    """Validated canonical Unreal project descriptor evidence."""

    path: Path
    snapshot: bytes


def _root() -> Path:
    """Return the repository root from this script's tracked location."""
    return Path(__file__).resolve().parents[3]


def _normalized(path: Path) -> str:
    """Return one absolute normalized filesystem path for saved evidence."""
    return str(path.resolve())


def _sha256(path: Path) -> str:
    """Hash one evidence file without loading it all into memory."""
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        while chunk := handle.read(1024 * 1024):
            digest.update(chunk)
    return digest.hexdigest()


def _scan_source_directory(
    path: Path,
) -> AbstractContextManager[Iterator[os.DirEntry[str]]]:
    """Open one source-closure directory without suppressing scan failures."""
    return os.scandir(path)


def _source_tree_files(source: Path) -> list[Path]:
    """Collect repository source files with strict non-redirected traversal."""
    files: list[Path] = []
    pending = [source]
    while pending:
        directory = pending.pop()
        with _scan_source_directory(directory) as entries:
            for entry in entries:
                path = Path(entry.path)
                if entry.is_symlink() or os.path.isjunction(path):
                    raise OSError(
                        "validator source closure contains a redirected entry"
                    )
                if entry.is_dir(follow_symlinks=False):
                    pending.append(path)
                elif entry.is_file(follow_symlinks=False):
                    files.append(path)
                else:
                    raise OSError(
                        "validator source closure contains a special entry"
                    )
    return files


def _source_file_identity(path: Path) -> tuple[int, ...]:
    """Return one stable real source-file identity for hashing."""
    if path.is_symlink() or os.path.isjunction(path):
        raise OSError("validator source closure contains a redirected file")
    metadata = path.stat(follow_symlinks=False)
    if not stat.S_ISREG(metadata.st_mode):
        raise OSError("validator source closure contains a special file")
    if metadata.st_nlink != 1:
        raise OSError("validator source closure contains a hard-linked file")
    return (
        metadata.st_dev,
        metadata.st_ino,
        metadata.st_mtime_ns,
        metadata.st_ctime_ns,
        metadata.st_size,
        metadata.st_nlink,
    )


def _read_source_bytes(path: Path) -> bytes:
    """Read one source file while preserving its lexical filesystem identity."""
    expected = _source_file_identity(path)
    with path.open("rb") as handle:
        opened = _source_file_identity_from_stat(os.fstat(handle.fileno()))
        if opened != expected:
            raise OSError("validator source file changed while hashing")
        payload = handle.read()
        finished = _source_file_identity_from_stat(os.fstat(handle.fileno()))
    if (
        finished != expected
        or len(payload) != expected[4]
        or _source_file_identity(path) != expected
    ):
        raise OSError("validator source file changed while hashing")
    return payload


def _source_file_identity_from_stat(
    metadata: os.stat_result,
) -> tuple[int, ...]:
    """Project source metadata to the stable hashing identity fields."""
    return (
        metadata.st_dev,
        metadata.st_ino,
        metadata.st_mtime_ns,
        metadata.st_ctime_ns,
        metadata.st_size,
        metadata.st_nlink,
    )


def _source_inputs_sha256(root: Path, inputs: tuple[Path, ...]) -> str:
    """Hash one deterministic repository source closure."""
    digest = hashlib.sha256()
    files: list[Path] = []
    for relative in inputs:
        source = root / relative
        if source.is_symlink() or os.path.isjunction(source):
            raise OSError(
                "validator source closure contains a redirected input"
            )
        if source.is_file():
            files.append(source)
            continue
        if not source.is_dir():
            raise FileNotFoundError(source)
        files.extend(_source_tree_files(source))
    ordered = sorted(
        files,
        key=lambda path: path.relative_to(root).as_posix(),
    )
    for source in ordered:
        relative = source.relative_to(root).as_posix().encode("utf-8")
        payload = _read_source_bytes(source)
        digest.update(len(relative).to_bytes(8, "big"))
        digest.update(relative)
        digest.update(len(payload).to_bytes(8, "big"))
        digest.update(payload)
    return digest.hexdigest()


def _validator_source_sha256(root: Path) -> str:
    """Hash source inputs that can change validate-game."""
    return _source_inputs_sha256(root, _VALIDATOR_SOURCE_INPUTS)


def _deep_validator_source_sha256(root: Path) -> str:
    """Hash source inputs that can change validate-source-deep."""
    return _source_inputs_sha256(root, _DEEP_VALIDATOR_SOURCE_INPUTS)


def _check_python() -> dict[str, str]:
    """Require the exact supported CPython interpreter version."""
    actual = sys.version_info[:3]
    if actual != _PYTHON_VERSION:
        wanted = ".".join(str(value) for value in _PYTHON_VERSION)
        found = ".".join(str(value) for value in actual)
        raise CheckFailure(
            f"Python {wanted} is required; this command is using {found}"
        )
    return {
        "executable": _normalized(Path(sys.executable)),
        "version": platform.python_version(),
    }


def _real_source_directory(path: Path) -> Path:
    """Resolve one source directory without accepting redirects.

    Raises:
        CheckFailure: If the directory or a lexical parent redirects.

    """
    redirected = any(
        prefix.is_symlink() or os.path.isjunction(prefix)
        for prefix in (path, *path.parents)
    )
    if redirected:
        raise CheckFailure(
            "selected source directory must be a real source directory"
        )
    return path.resolve()


def _game_candidate(root: Path, selected: Path | None) -> Path:
    """Resolve a selected source directory or Simpsons.exe to its root."""
    if selected is None:
        return _real_source_directory(root / "game")
    candidate = selected.expanduser()
    if not candidate.is_absolute():
        candidate = Path.cwd() / candidate
    if candidate.is_file():
        if candidate.name != "Simpsons.exe":
            raise CheckFailure("selected source file must be Simpsons.exe")
        if candidate.is_symlink():
            raise CheckFailure(
                "selected source file must be a real Simpsons.exe"
            )
        return _real_source_directory(candidate.parent)
    if candidate.is_dir():
        return _real_source_directory(candidate)
    raise CheckFailure("selected source path does not exist")


def _scan_directory(
    path: Path,
) -> AbstractContextManager[Iterator[os.DirEntry[str]]]:
    """Open one directory scan so tests can inject metadata failures."""
    return os.scandir(path)


def _nested_game_executables(
    game: Path, direct: Path | None,
) -> tuple[Path, ...]:
    """Find nested canonical executables without suppressing scan failures."""
    nested: list[Path] = []
    pending = [game]
    while pending:
        directory = pending.pop()
        with _scan_directory(directory) as entries:
            for entry in entries:
                path = Path(entry.path)
                if path != direct and entry.name == "Simpsons.exe":
                    nested.append(path)
                if (
                    entry.is_dir(follow_symlinks=False)
                    and not entry.is_symlink()
                    and not os.path.isjunction(path)
                ):
                    pending.append(path)
    return tuple(nested)


def _inspect_game_root(game: Path) -> None:
    """Require one resolved source root to contain one direct executable."""
    executable = next(
        (path for path in game.iterdir() if path.name == "Simpsons.exe"),
        None,
    ) if game.is_dir() else None
    if executable is None or not executable.is_file():
        nested = _nested_game_executables(game, None) if game.is_dir() else ()
        if nested:
            raise CheckFailure(
                "Simpsons.exe must be directly inside the selected source"
            )
        raise CheckFailure(
            "selected source does not contain a direct Simpsons.exe"
        )
    if executable.is_symlink():
        raise CheckFailure("selected source must contain a real Simpsons.exe")

    nested = _nested_game_executables(game, executable)
    if nested:
        raise CheckFailure(
            "selected source contains another nested Simpsons.exe"
        )


def _check_game(root: Path, selected: Path | None) -> Path:
    """Require one flat source installation without modifying it."""
    try:
        game = _game_candidate(root, selected)
        _inspect_game_root(game)
    except OSError as error:
        raise CheckFailure(
            "selected source cannot be inspected safely"
        ) from error
    else:
        return game


def _evidence_identity(metadata: os.stat_result) -> tuple[int, ...]:
    """Return filesystem identity used to bind one evidence read."""
    return (
        metadata.st_dev,
        metadata.st_ino,
        metadata.st_mtime_ns,
        metadata.st_ctime_ns,
        metadata.st_size,
        metadata.st_nlink,
    )


def _real_evidence_identity(path: Path, label: str) -> tuple[int, ...]:
    """Return one non-redirected regular evidence-file identity."""
    if path.is_symlink() or os.path.isjunction(path):
        raise CheckFailure(f"{label} must be a real file")
    try:
        metadata = path.stat(follow_symlinks=False)
    except OSError as error:
        raise CheckFailure(f"cannot read {label} {path}: {error}") from error
    if not stat.S_ISREG(metadata.st_mode) or metadata.st_nlink != 1:
        raise CheckFailure(f"{label} must be a real file")
    return _evidence_identity(metadata)


def _read_evidence_descriptor(
    path: Path,
    label: str,
    expected: tuple[int, ...],
) -> bytes:
    """Read bytes from the descriptor matching one expected identity."""
    try:
        with path.open("rb") as handle:
            opened = _evidence_identity(os.fstat(handle.fileno()))
            if opened != expected:
                raise CheckFailure(f"{label} changed while reading")
            payload = handle.read()
            finished = _evidence_identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise CheckFailure(f"cannot read {label} {path}: {error}") from error
    if finished != expected or len(payload) != expected[4]:
        raise CheckFailure(f"{label} changed while reading")
    return payload


def _read_real_evidence_bytes(path: Path, label: str) -> bytes:
    """Read one stable non-redirected regular evidence file."""
    expected = _real_evidence_identity(path, label)
    payload = _read_evidence_descriptor(path, label, expected)
    if _real_evidence_identity(path, label) != expected:
        raise CheckFailure(f"{label} changed while reading")
    return payload


def _sha256_real_evidence(path: Path, label: str) -> str:
    """Hash one stable non-redirected regular evidence file."""
    expected = _real_evidence_identity(path, label)
    digest = hashlib.sha256()
    total = 0
    try:
        with path.open("rb") as handle:
            opened = _evidence_identity(os.fstat(handle.fileno()))
            if opened != expected:
                raise CheckFailure(f"{label} changed while hashing")
            while chunk := handle.read(1024 * 1024):
                total += len(chunk)
                digest.update(chunk)
            finished = _evidence_identity(os.fstat(handle.fileno()))
    except OSError as error:
        raise CheckFailure(f"cannot read {label} {path}: {error}") from error
    if finished != expected or total != expected[4]:
        raise CheckFailure(f"{label} changed while hashing")
    if _real_evidence_identity(path, label) != expected:
        raise CheckFailure(f"{label} changed while hashing")
    return digest.hexdigest()


def _require_unchanged_evidence(
    path: Path,
    label: str,
    snapshot: bytes,
) -> None:
    """Require one stable evidence path to retain its captured bytes."""
    if _read_real_evidence_bytes(path, label) != snapshot:
        raise CheckFailure(f"{label} changed during preflight")


def _json_object_from_bytes(
    payload: bytes,
    label: str,
    path: Path,
) -> dict[str, object]:
    """Decode one UTF-8 JSON object from an already-stable snapshot."""
    try:
        value = json.loads(
            payload.decode("utf-8"),
            object_pairs_hook=_unique_json_object,
        )
    except (UnicodeError, json.JSONDecodeError) as error:
        raise CheckFailure(f"cannot read {label} {path}: {error}") from error
    if not isinstance(value, dict):
        raise CheckFailure(f"{label} must contain a JSON object: {path}")
    return value


def _require_real_directories(roots: tuple[tuple[Path, str], ...]) -> None:
    """Require repository authority paths to remain real directories."""
    for path, label in roots:
        if (
            not path.is_dir()
            or path.is_symlink()
            or os.path.isjunction(path)
        ):
            raise CheckFailure(f"{label} must be a real directory: {path}")


def _require_real_build_data_roots(root: Path) -> None:
    """Require canonical build-data ancestors to remain real directories."""
    _require_real_directories(
        (
            (root / ".cache", "repository cache root"),
            (root / ".cache/build", "build cache root"),
            (root / ".cache/build/data", "build data root"),
        )
    )


def _require_real_manifest_roots(root: Path) -> None:
    """Keep the canonical manifest under real repository directories."""
    _require_real_directories(
        (
            (root / "game", "canonical game root"),
            (root / "game/manifest", "canonical manifest root"),
        )
    )


def _require_real_project_roots(root: Path) -> None:
    """Keep the canonical Unreal descriptor under real repository roots."""
    _require_real_directories(
        (
            (root / "src", "source root"),
            (root / "src/unreal", "Unreal source root"),
            (root / "src/unreal/project", "Unreal project source root"),
            (
                root / "src/unreal/project/composition",
                "Unreal project composition root",
            ),
            (
                root / "src/unreal/project/composition/uproject",
                "Unreal project root",
            ),
        )
    )


def _dependency_evidence(
    root: Path,
) -> tuple[Path, bytes, dict[str, object]]:
    """Read and validate one stable dependency bootstrap snapshot."""
    _require_real_build_data_roots(root)
    path = root / _DEPENDENCIES_PATH
    snapshot = _read_real_evidence_bytes(path, "dependency evidence")
    data = _json_object_from_bytes(snapshot, "dependency evidence", path)
    if data.get("schema") != _DEPENDENCIES_SCHEMA:
        raise CheckFailure(
            f"dependency evidence schema must be {_DEPENDENCIES_SCHEMA}"
        )
    return path, snapshot, data


def _dependency_validator_root(root: Path) -> Path:
    """Return the canonical root after rejecting validator redirects."""
    paths = (
        root / ".dependencies",
        root / ".dependencies" / "build",
        root / ".dependencies" / "build" / "bin",
    )
    for path in paths:
        if (
            not path.is_dir()
            or path.is_symlink()
            or os.path.isjunction(path)
        ):
            raise CheckFailure(
                "dependency validator storage must use real directories"
            )
    return paths[-1].resolve()


def _real_dependency_validator(
    root: Path, raw_path: str, label: str,
) -> Path:
    """Require one repository-owned single-link validator binary."""
    candidate = Path(raw_path)
    owned = _dependency_validator_root(root)
    if candidate.is_symlink() or os.path.isjunction(candidate):
        raise CheckFailure(f"{label} must be a real file")
    validator = candidate.resolve()
    if validator.parent != owned:
        raise CheckFailure(f"{label} must be under .dependencies/build/bin")
    if not validator.is_file():
        raise CheckFailure(f"{label} is missing: {validator}")
    if validator.stat(follow_symlinks=False).st_nlink != 1:
        raise CheckFailure(f"{label} must be a real file")
    return validator


def _require_dependency_validator_hash(
    validator: Path,
    label: str,
    expected_hash: str,
) -> None:
    """Require one dependency validator to retain its saved binary hash."""
    if _sha256_real_evidence(validator, label) != expected_hash:
        raise CheckFailure(f"{label} SHA-256 no longer matches evidence")


def _dependency_validator(
    root: Path,
    data: dict[str, object],
) -> Path:
    """Require the hashed repository-owned validator from bootstrap evidence."""
    value = data.get("validator")
    if not isinstance(value, dict):
        raise CheckFailure("dependency evidence has no validator object")
    raw_path = value.get("path")
    expected_hash = value.get("sha256")
    expected_source_hash = value.get("source_sha256")
    if (
        not isinstance(raw_path, str)
        or not isinstance(expected_hash, str)
        or not isinstance(expected_source_hash, str)
    ):
        raise CheckFailure("dependency validator evidence is incomplete")
    validator = _real_dependency_validator(
        root,
        raw_path,
        "dependency validator",
    )
    _require_dependency_validator_hash(
        validator, "dependency validator", expected_hash
    )
    actual_source_hash = _validator_source_sha256(root)
    if actual_source_hash != expected_source_hash:
        raise CheckFailure(
            "dependency validator source inputs no longer match evidence; "
            "rerun tools/build/adapter-inbound/dependencies.py"
        )
    _require_dependency_validator_hash(
        validator, "dependency validator", expected_hash
    )
    return validator


def _dependency_deep_source_validator(
    root: Path,
    data: dict[str, object],
) -> Path:
    """Require the hashed repository-owned deep source validator."""
    value = data.get("deep_source_validator")
    if not isinstance(value, dict):
        raise CheckFailure("dependency evidence has no deep source validator")
    raw_path = value.get("path")
    expected_hash = value.get("sha256")
    expected_source_hash = value.get("source_sha256")
    if (
        not isinstance(raw_path, str)
        or not isinstance(expected_hash, str)
        or not isinstance(expected_source_hash, str)
    ):
        raise CheckFailure("deep source validator evidence is incomplete")
    validator = _real_dependency_validator(
        root,
        raw_path,
        "deep source validator",
    )
    _require_dependency_validator_hash(
        validator, "deep source validator", expected_hash
    )
    if _deep_validator_source_sha256(root) != expected_source_hash:
        raise CheckFailure(
            "deep source validator source inputs no longer match evidence; "
            "rerun tools/build/adapter-inbound/dependencies.py"
        )
    _require_dependency_validator_hash(
        validator, "deep source validator", expected_hash
    )
    return validator


def _resolve_validator(
    root: Path,
    explicit: Path | None,
    dependencies: dict[str, object],
) -> Path:
    """Resolve the canonical validator or an explicit testing override."""
    if explicit is not None:
        candidate = explicit if explicit.is_absolute() else root / explicit
        if not candidate.is_file():
            raise CheckFailure(
                f"manifest validator does not exist: {candidate.resolve()}"
            )
        return candidate.resolve()
    return _dependency_validator(root, dependencies)


def _resolve_deep_source_validator(
    root: Path,
    explicit: Path | None,
    dependencies: dict[str, object],
) -> Path:
    """Resolve the canonical deep validator or a testing override."""
    if explicit is not None:
        candidate = explicit if explicit.is_absolute() else root / explicit
        if not candidate.is_file():
            raise CheckFailure(
                f"deep source validator does not exist: {candidate.resolve()}"
            )
        return candidate.resolve()
    return _dependency_deep_source_validator(root, dependencies)


def _validator_command(
    validator: Path,
    game: Path,
    manifest: Path,
) -> list[str]:
    """Build a portable command for an executable or Windows cmd wrapper."""
    if os.name == "nt" and validator.suffix.casefold() == ".cmd":
        return [
            os.environ.get("COMSPEC", "cmd.exe"),
            "/d",
            "/c",
            str(validator),
            str(game),
            str(manifest),
        ]
    return [str(validator), str(game), str(manifest)]


def _redact_selected_source(value: str, game: Path) -> str:
    """Redact common native renderings of the selected private source root."""
    raw = str(game)
    spellings = {raw, game.as_posix()}
    spellings.update(
        spelling.replace("\\", "\\\\")
        for spelling in tuple(spellings)
        if "\\" in spelling
    )
    redacted = value
    for spelling in sorted(spellings, key=len, reverse=True):
        if spelling:
            redacted = re.sub(
                re.escape(spelling),
                "<selected-source>",
                redacted,
                flags=re.IGNORECASE,
            )
    return redacted


def _check_manifest(validator: Path, game: Path, manifest: Path) -> str:
    """Run the canonical manifest validator without compiling or mutating."""
    try:
        result = subprocess.run(
            _validator_command(validator, game, manifest),
            check=False,
            capture_output=True,
            text=True,
            timeout=180,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        detail = _redact_selected_source(str(error), game)
        message = f"manifest validation could not run: {detail}"
        raise CheckFailure(message) from error
    if result.returncode:
        detail = result.stderr.strip() or result.stdout.strip()
        detail = _redact_selected_source(detail, game)
        raise CheckFailure(f"game manifest validation failed: {detail}")
    return _redact_selected_source(result.stdout.strip(), game)


def _deep_validator_command(validator: Path, game: Path) -> list[str]:
    """Build a portable deep-source command without a manifest argument."""
    if os.name == "nt" and validator.suffix.casefold() == ".cmd":
        return [
            os.environ.get("COMSPEC", "cmd.exe"),
            "/d",
            "/c",
            str(validator),
            str(game),
        ]
    return [str(validator), str(game)]


def _check_deep_source(validator: Path, game: Path) -> str:
    """Run deep structural source validation after the fast manifest gate."""
    try:
        result = subprocess.run(
            _deep_validator_command(validator, game),
            check=False,
            capture_output=True,
            text=True,
            timeout=300,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        detail = _redact_selected_source(str(error), game)
        message = f"deep source validation could not run: {detail}"
        raise CheckFailure(message) from error
    if result.returncode:
        detail = result.stderr.strip() or result.stdout.strip()
        detail = _redact_selected_source(detail, game)
        raise CheckFailure(f"deep source validation failed: {detail}")
    return _redact_selected_source(result.stdout.strip(), game)


def _unique_json_object(
    pairs: list[tuple[str, object]],
) -> dict[str, object]:
    """Reject duplicate keys at every JSON object depth."""
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise CheckFailure(f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _read_json_object(path: Path, label: str) -> dict[str, object]:
    """Read one required JSON object with a user-facing source label."""
    try:
        value = json.loads(
            path.read_text(encoding="utf-8"),
            object_pairs_hook=_unique_json_object,
        )
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise CheckFailure(f"cannot read {label} {path}: {error}") from error
    if not isinstance(value, dict):
        raise CheckFailure(f"{label} must contain a JSON object: {path}")
    return value


def _check_project(root: Path) -> ProjectEvidence:
    """Require the tracked Unreal project association used by the build."""
    _require_real_project_roots(root)
    project = root / _PROJECT_PATH
    snapshot = _read_real_evidence_bytes(project, "Unreal project descriptor")
    descriptor = _json_object_from_bytes(
        snapshot,
        "Unreal project descriptor",
        project,
    )
    association = descriptor.get("EngineAssociation")
    if association != _UNREAL_ASSOCIATION:
        raise CheckFailure(
            "Unreal project EngineAssociation must be "
            f"{_UNREAL_ASSOCIATION}, not {association!r}"
        )
    return ProjectEvidence(project, snapshot)


def _engine_candidates(explicit: Path | None) -> list[Path]:
    """Return explicit, environment, and common launcher engine roots."""
    if explicit is not None:
        return [explicit]
    candidates: list[Path] = []
    environment = os.environ.get("UNREAL_ENGINE_ROOT")
    if environment:
        candidates.append(Path(environment))
    if os.name == "nt":
        program_files = os.environ.get("PROGRAMFILES")
        if program_files:
            candidates.append(Path(program_files) / "Epic Games" / "UE_5.8")
    elif sys.platform == "darwin":
        candidates.append(
            Path("/") / "Users" / "Shared" / "Epic Games" / "UE_5.8"
        )
    return candidates


def _engine_version(root: Path) -> str:
    """Read and require the exact Unreal Build.version tuple."""
    build_version = root / "Engine" / "Build" / "Build.version"
    data = _read_json_object(build_version, "Unreal Build.version")
    keys = ("MajorVersion", "MinorVersion", "PatchVersion")
    raw = tuple(data.get(key) for key in keys)
    if any(type(value) is not int for value in raw):
        raise CheckFailure(
            f"Unreal Build.version has invalid version fields: {build_version}"
        )
    actual = tuple(int(value) for value in raw)
    if actual != _UNREAL_VERSION:
        wanted = ".".join(str(value) for value in _UNREAL_VERSION)
        found = ".".join(str(value) for value in actual)
        raise CheckFailure(f"Unreal Engine {wanted} is required; found {found}")
    return ".".join(str(value) for value in actual)


def _editor_path(engine: Path) -> Path | None:
    """Return the host-native Unreal editor executable when known."""
    if os.name == "nt":
        return engine / "Engine" / "Binaries" / "Win64" / "UnrealEditor.exe"
    if sys.platform == "darwin":
        return (
            engine
            / "Engine"
            / "Binaries"
            / "Mac"
            / "UnrealEditor.app"
            / "Contents"
            / "MacOS"
            / "UnrealEditor"
        )
    if sys.platform.startswith("linux"):
        return engine / "Engine" / "Binaries" / "Linux" / "UnrealEditor"
    return None


def _check_engine(explicit: Path | None) -> EngineEvidence:
    """Resolve an exact 5.8.1 engine installation with a host editor."""
    checked: list[str] = []
    for candidate in _engine_candidates(explicit):
        resolved = candidate.expanduser().resolve()
        checked.append(str(resolved))
        build_version = resolved / "Engine" / "Build" / "Build.version"
        if not build_version.is_file():
            continue
        version = _engine_version(resolved)
        editor = _editor_path(resolved)
        if editor is not None:
            if not editor.is_file():
                message = f"Unreal editor executable is missing: {editor}"
                raise CheckFailure(message)
            if os.name != "nt" and not os.access(editor, os.X_OK):
                raise CheckFailure(f"Unreal editor is not executable: {editor}")
        return EngineEvidence(resolved, version)
    searched = ", ".join(checked) if checked else "no default location"
    raise CheckFailure(
        "Unreal Engine 5.8.1 was not found; use --engine-root or set "
        f"UNREAL_ENGINE_ROOT (searched: {searched})"
    )


def _dependency_host_tools(
    dependencies: dict[str, object],
) -> dict[str, object]:
    """Revalidate host build tools recorded by dependency bootstrap."""
    if os.name != "nt":
        return {}
    external = dependencies.get("external_prerequisites")
    if not isinstance(external, dict):
        raise CheckFailure("dependency evidence has no external prerequisites")
    visual_studio = external.get("visual_studio")
    if not isinstance(visual_studio, dict):
        raise CheckFailure(
            "Visual Studio C++ Build Tools were not validated by "
            "dependencies.py"
        )
    result: dict[str, str] = {}
    for name in ("compiler", "linker"):
        raw = visual_studio.get(name)
        if not isinstance(raw, str) or not raw:
            raise CheckFailure(f"Visual Studio evidence has no {name} path")
        path = Path(raw).resolve()
        if not path.is_file():
            raise CheckFailure(f"Visual Studio {name} is missing: {path}")
        result[name] = _normalized(path)
    installation = visual_studio.get("installation")
    if not isinstance(installation, str) or not Path(installation).is_dir():
        raise CheckFailure("Visual Studio installation evidence is invalid")
    result["installation"] = _normalized(Path(installation))
    return {"visual_studio": result}


def _host_evidence(
    dependencies: dict[str, object],
) -> dict[str, object]:
    """Record current host identity and basic command prerequisites."""
    required = ["git"]
    if os.name == "nt":
        required.extend(("cmd.exe", "powershell.exe"))
    elif sys.platform.startswith("linux"):
        required.append("bash")
    elif sys.platform == "darwin":
        required.append("xcrun")

    commands: dict[str, str] = {}
    missing: list[str] = []
    for name in required:
        resolved = shutil.which(name)
        if resolved is None:
            missing.append(name)
        else:
            commands[name] = _normalized(Path(resolved))
    if missing:
        raise CheckFailure(
            "required host commands are missing: " + ", ".join(missing)
        )
    return {
        "architecture": platform.machine().casefold(),
        "build_tools": _dependency_host_tools(dependencies),
        "commands": commands,
        "system": platform.system().casefold(),
    }


def _validate_canonical_output_root(root: Path, output: Path) -> bool:
    """Reject malformed canonical ancestors and identify canonical output."""
    canonical = root / _DATA_PATH
    if output != canonical:
        return False
    roots = (
        (root / ".cache", "repository cache root"),
        (root / ".cache/build", "build cache root"),
        (root / ".cache/build/data", "build data root"),
    )
    for path, label in roots:
        if not os.path.lexists(path):
            continue
        if (
            not path.is_dir()
            or path.is_symlink()
            or os.path.isjunction(path)
        ):
            raise CheckFailure(f"{label} must be a real directory: {path}")
    if os.path.lexists(output) and (
        not output.is_file() or output.is_symlink()
    ):
        raise CheckFailure(
            f"preflight evidence must be a real file: {output}"
        )
    return True


def _write_json(path: Path, value: dict[str, object]) -> None:
    """Atomically replace saved preflight evidence."""
    path.parent.mkdir(parents=True, exist_ok=True)
    candidate = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    text = json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True)
    created = False
    try:
        with candidate.open(
            "x",
            encoding="utf-8",
            newline="\n",
        ) as handle:
            created = True
            handle.write(text + "\n")
        Path(candidate).replace(path)
    finally:
        if created:
            candidate.unlink(missing_ok=True)


def _run(args: argparse.Namespace) -> dict[str, object]:
    """Execute all preflight checks before producing saved evidence."""
    root = _root()
    python = _check_python()
    game = _check_game(root, args.game)
    _require_real_manifest_roots(root)
    manifest = root / "game" / "manifest" / "game.jsonl"
    manifest_snapshot = _read_real_evidence_bytes(
        manifest,
        "canonical game manifest",
    )
    project = _check_project(root)
    dependencies_path, dependencies_snapshot, dependencies = (
        _dependency_evidence(root)
    )
    validator = _resolve_validator(
        root,
        args.manifest_validator,
        dependencies,
    )
    manifest_result = _check_manifest(validator, game, manifest)
    deep_validator = _resolve_deep_source_validator(
        root,
        args.deep_source_validator,
        dependencies,
    )
    deep_result = _check_deep_source(deep_validator, game)
    engine = _check_engine(args.engine_root)
    host = _host_evidence(dependencies)
    _require_unchanged_evidence(
        dependencies_path,
        "dependency evidence",
        dependencies_snapshot,
    )
    _require_unchanged_evidence(
        manifest,
        "canonical game manifest",
        manifest_snapshot,
    )
    _require_unchanged_evidence(
        project.path,
        "Unreal project descriptor",
        project.snapshot,
    )
    return {
        "dependencies": {
            "path": _normalized(dependencies_path),
            "schema": _DEPENDENCIES_SCHEMA,
            "sha256": hashlib.sha256(dependencies_snapshot).hexdigest(),
        },
        "game": {
            "manifest": _normalized(manifest),
            "manifest_sha256": hashlib.sha256(manifest_snapshot).hexdigest(),
            "path": _normalized(game),
            "validation": manifest_result,
            "deep_validation": deep_result,
        },
        "host": host,
        "python": python,
        "schema": _SCHEMA,
        "unreal": {
            "project": _normalized(project.path),
            "project_sha256": hashlib.sha256(project.snapshot).hexdigest(),
            "root": _normalized(engine.root),
            "version": engine.version,
        },
        "validator": _normalized(validator),
        "deep_source_validator": _normalized(deep_validator),
    }


def _saved_game_root(saved: dict[str, object]) -> Path:
    """Read the source root needed to reproduce one saved preflight."""
    game = saved.get("game")
    if not isinstance(game, dict):
        raise CheckFailure("saved check evidence has no game object")
    raw_root = game.get("path")
    if not isinstance(raw_root, str) or not raw_root:
        raise CheckFailure("saved check evidence has no source game root")
    return Path(raw_root)


def _saved_engine_root(saved: dict[str, object]) -> Path:
    """Read the engine root needed to reproduce one saved preflight."""
    unreal = saved.get("unreal")
    if not isinstance(unreal, dict):
        raise CheckFailure("saved check evidence has no unreal object")
    raw_root = unreal.get("root")
    if not isinstance(raw_root, str) or not raw_root:
        raise CheckFailure("saved check evidence has no Unreal root")
    return Path(raw_root)


def _revalidate(path: Path) -> None:
    """Recompute preflight evidence and require exact saved equality."""
    snapshot = _read_real_evidence_bytes(path, "saved check evidence")
    saved = _json_object_from_bytes(snapshot, "saved check evidence", path)
    if saved.get("schema") != _SCHEMA:
        raise CheckFailure(f"saved check evidence schema must be {_SCHEMA}")
    engine_root = _saved_engine_root(saved)
    game_root = _saved_game_root(saved)
    arguments = argparse.Namespace(
        engine_root=engine_root,
        game=game_root,
        manifest_validator=None,
        deep_source_validator=None,
    )
    current = _run(arguments)
    if _read_real_evidence_bytes(path, "saved check evidence") != snapshot:
        raise CheckFailure("saved check evidence changed during revalidation")
    if saved != current:
        raise CheckFailure(
            "saved check evidence no longer matches validated state; "
            "rerun tools/build/adapter-inbound/check.py"
        )


def _parser() -> argparse.ArgumentParser:
    """Build the supported preflight command-line interface."""
    parser = argparse.ArgumentParser(
        description="Validate SHAR game and build prerequisites.",
    )
    parser.add_argument(
        "--game",
        type=Path,
        help="lawful source directory or Simpsons.exe path",
    )
    parser.add_argument(
        "--engine-root",
        type=Path,
        help="explicit Unreal Engine 5.8.1 installation root",
    )
    parser.add_argument(
        "--manifest-validator",
        type=Path,
        help="explicit already-built validate-game executable",
    )
    parser.add_argument(
        "--deep-source-validator",
        type=Path,
        help="explicit already-built validate-source-deep executable",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="override .cache/build/data/check.json for testing",
    )
    parser.add_argument(
        "--revalidate",
        action="store_true",
        help="revalidate the saved check JSON instead of replacing it",
    )
    return parser


def _reject_revalidate_overrides(args: argparse.Namespace) -> None:
    """Reject options that would replace saved preflight evidence."""
    has_override = (
        args.engine_root is not None
        or args.game is not None
        or args.manifest_validator is not None
        or args.deep_source_validator is not None
    )
    if has_override:
        raise CheckFailure(
            "--revalidate cannot be combined with preflight overrides"
        )


def main() -> int:
    """Fail closed or atomically save complete preflight evidence."""
    args = _parser().parse_args()
    root = _root()
    output = args.output or (root / _DATA_PATH)
    if not output.is_absolute():
        output = root / output
    cleanup_output_on_failure = False
    try:
        cleanup_output_on_failure = _validate_canonical_output_root(
            root, output
        )
        if args.revalidate:
            _reject_revalidate_overrides(args)
            _revalidate(output)
            print(f"check: revalidated saved evidence at {output.resolve()}")
            return 0
        evidence = _run(args)
        _write_json(output, evidence)
    except (CheckFailure, OSError) as error:
        if not args.revalidate and cleanup_output_on_failure:
            output.unlink(missing_ok=True)
        print(f"check: {error}", file=sys.stderr)
        return 1
    print(f"check: clean; saved evidence to {output.resolve()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

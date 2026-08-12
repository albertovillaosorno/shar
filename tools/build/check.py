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
#   - Read-only user build preflight and saved validation evidence.
# - Must-Not:
#   - Extract game data, compile validators, or mutate build inputs.
# - Allows:
#   - Inputs: game, Python, Unreal, host, and manifest-validator evidence.
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
#   - Run tools/build/check.py after dependencies.py.
# - Defaults:
#   - Requires Python 3.14.6 and Unreal Engine 5.8.1 exactly.
#

"""Validate SHAR user build prerequisites and persist checked evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path
from typing import NamedTuple

_SCHEMA = "shar.build.check.v1"
_PYTHON_VERSION = (3, 14, 6)
_UNREAL_VERSION = (5, 8, 1)
_UNREAL_ASSOCIATION = "5.8"
_DATA_PATH = Path(".cache/build/data/check.json")
_DEPENDENCIES_PATH = Path(".cache/build/data/dependencies.json")
_DEPENDENCIES_SCHEMA = "shar.build.dependencies.v1"
_PROJECT_PATH = Path(
    "src/unreal/project/composition/uproject/shar.uproject"
)


class CheckFailure(RuntimeError):
    """One user-actionable preflight failure."""


class EngineEvidence(NamedTuple):
    """Validated Unreal installation evidence."""

    root: Path
    version: str


def _root() -> Path:
    """Return the repository root from this script's tracked location."""
    return Path(__file__).resolve().parents[2]


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


def _check_game(root: Path) -> tuple[Path, Path]:
    """Require the supported flat game layout and canonical manifest file."""
    game = root / "game"
    executable = game / "Simpsons.exe"
    if not executable.is_file():
        nested = sorted(game.rglob("Simpsons.exe")) if game.is_dir() else []
        if nested:
            example = nested[0].relative_to(game)
            raise CheckFailure(
                "game/Simpsons.exe is required directly under game/; "
                f"found nested {example}"
            )
        raise CheckFailure(
            "game/Simpsons.exe is missing; copy the installed game contents "
            "directly into game/"
        )

    nested = [
        path
        for path in game.rglob("Simpsons.exe")
        if path.resolve() != executable.resolve()
    ]
    if nested:
        example = sorted(nested)[0].relative_to(game)
        raise CheckFailure(
            "nested game copies are unsupported; remove the extra "
            f"{example} layout"
        )

    manifest = game / "manifest" / "game.jsonl"
    if not manifest.is_file():
        raise CheckFailure("game/manifest/game.jsonl is missing")
    return game, manifest


def _dependency_evidence(root: Path) -> tuple[Path, dict[str, object]]:
    """Read and validate the dependency bootstrap evidence."""
    path = root / _DEPENDENCIES_PATH
    data = _read_json_object(path, "dependency evidence")
    if data.get("schema") != _DEPENDENCIES_SCHEMA:
        raise CheckFailure(
            f"dependency evidence schema must be {_DEPENDENCIES_SCHEMA}"
        )
    return path, data


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
    if not isinstance(raw_path, str) or not isinstance(expected_hash, str):
        raise CheckFailure("dependency validator evidence is incomplete")
    validator = Path(raw_path).resolve()
    owned = (root / ".dependencies" / "build" / "bin").resolve()
    if validator.parent != owned:
        raise CheckFailure(
            "dependency validator must be under .dependencies/build/bin"
        )
    if not validator.is_file():
        raise CheckFailure(f"dependency validator is missing: {validator}")
    actual_hash = _sha256(validator)
    if actual_hash != expected_hash:
        message = "dependency validator SHA-256 no longer matches evidence"
        raise CheckFailure(message)
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


def _validator_command(validator: Path, game: Path) -> list[str]:
    """Build a portable command for an executable or Windows cmd wrapper."""
    if os.name == "nt" and validator.suffix.casefold() == ".cmd":
        return [
            os.environ.get("COMSPEC", "cmd.exe"),
            "/d",
            "/c",
            str(validator),
            str(game),
        ]
    return [str(validator), str(game)]


def _check_manifest(validator: Path, game: Path) -> str:
    """Run the canonical manifest validator without compiling or mutating."""
    try:
        result = subprocess.run(
            _validator_command(validator, game),
            check=False,
            capture_output=True,
            text=True,
            timeout=180,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        message = f"manifest validation could not run: {error}"
        raise CheckFailure(message) from error
    if result.returncode:
        detail = result.stderr.strip() or result.stdout.strip()
        raise CheckFailure(f"game manifest validation failed: {detail}")
    return result.stdout.strip()


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


def _check_project(root: Path) -> Path:
    """Require the tracked Unreal project association used by the build."""
    project = root / _PROJECT_PATH
    descriptor = _read_json_object(project, "Unreal project descriptor")
    association = descriptor.get("EngineAssociation")
    if association != _UNREAL_ASSOCIATION:
        raise CheckFailure(
            "Unreal project EngineAssociation must be "
            f"{_UNREAL_ASSOCIATION}, not {association!r}"
        )
    return project


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
            candidates.append(
                Path(program_files) / "Epic Games" / "UE_5.8"
            )
    elif sys.platform == "darwin":
        candidates.append(Path("/Users/Shared/Epic Games/UE_5.8"))
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
        candidate = candidate.expanduser().resolve()
        checked.append(str(candidate))
        build_version = candidate / "Engine" / "Build" / "Build.version"
        if not build_version.is_file():
            continue
        version = _engine_version(candidate)
        editor = _editor_path(candidate)
        if editor is not None and not editor.exists():
            raise CheckFailure(f"Unreal editor executable is missing: {editor}")
        return EngineEvidence(candidate, version)
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


def _write_json(path: Path, value: dict[str, object]) -> None:
    """Atomically replace saved preflight evidence."""
    path.parent.mkdir(parents=True, exist_ok=True)
    candidate = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    text = json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True)
    try:
        candidate.write_text(text + "\n", encoding="utf-8", newline="\n")
        os.replace(candidate, path)
    finally:
        candidate.unlink(missing_ok=True)


def _run(args: argparse.Namespace) -> dict[str, object]:
    """Execute all preflight checks before producing saved evidence."""
    root = _root()
    python = _check_python()
    game, manifest = _check_game(root)
    project = _check_project(root)
    dependencies_path, dependencies = _dependency_evidence(root)
    validator = _resolve_validator(
        root,
        args.manifest_validator,
        dependencies,
    )
    manifest_result = _check_manifest(validator, game)
    engine = _check_engine(args.engine_root)
    host = _host_evidence(dependencies)
    return {
        "dependencies": {
            "path": _normalized(dependencies_path),
            "schema": _DEPENDENCIES_SCHEMA,
            "sha256": _sha256(dependencies_path),
        },
        "game": {
            "manifest": _normalized(manifest),
            "manifest_sha256": _sha256(manifest),
            "path": _normalized(game),
            "validation": manifest_result,
        },
        "host": host,
        "python": python,
        "schema": _SCHEMA,
        "unreal": {
            "project": _normalized(project),
            "root": _normalized(engine.root),
            "version": engine.version,
        },
        "validator": _normalized(validator),
    }


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
    saved = _read_json_object(path, "saved check evidence")
    if saved.get("schema") != _SCHEMA:
        raise CheckFailure(f"saved check evidence schema must be {_SCHEMA}")
    engine_root = _saved_engine_root(saved)
    arguments = argparse.Namespace(
        engine_root=engine_root,
        manifest_validator=None,
    )
    current = _run(arguments)
    if saved != current:
        raise CheckFailure(
            "saved check evidence no longer matches validated state; "
            "rerun tools/build/check.py"
        )


def _parser() -> argparse.ArgumentParser:
    """Build the supported preflight command-line interface."""
    parser = argparse.ArgumentParser(
        description="Validate SHAR game and build prerequisites.",
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


def main() -> int:
    """Fail closed or atomically save complete preflight evidence."""
    args = _parser().parse_args()
    root = _root()
    output = args.output or (root / _DATA_PATH)
    if not output.is_absolute():
        output = root / output
    try:
        if args.revalidate:
            has_override = (
                args.engine_root is not None
                or args.manifest_validator is not None
            )
            if has_override:
                raise CheckFailure(
                    "--revalidate cannot be combined with preflight overrides"
                )
            _revalidate(output)
            print(f"check: revalidated saved evidence at {output.resolve()}")
            return 0
        evidence = _run(args)
        _write_json(output, evidence)
    except (CheckFailure, OSError) as error:
        if not args.revalidate:
            output.unlink(missing_ok=True)
        print(f"check: {error}", file=sys.stderr)
        return 1
    print(f"check: clean; saved evidence to {output.resolve()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

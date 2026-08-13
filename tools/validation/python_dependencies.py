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
#   - Repository-local Python environment used by Jig's pytest and Ruff gates.
# - Must-Not:
#   - Install global packages, mutate the source-linked Jig checkout, or use
#     ambient Python package state as validation authority.
# - Allows:
#   - Inputs: exact CPython 3.14.6 from the source-linked Jig dependency.
#   - Outputs: .dependencies/python/3.14.6 and deterministic tool evidence.
#   - Side effects: venv creation and pinned pip installs below .dependencies.
# - Split-When:
#   - Split when another Python validation tool gains an independent lifecycle.
# - Merge-When:
#   - Merge when Jig directly owns project-specific Python environment creation.
# - Summary:
#   - Materializes SHAR's exact repository-local pytest and Ruff environment.

"""Prepare the repository-local Python tools required by canonical Jig gates."""

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

_PYTHON_VERSION = "3.14.6"
_PYTEST_VERSION = "9.1.1"
_RUFF_VERSION = "0.15.21"
_SCHEMA = "shar.validation-python.v1"
_REQUIREMENTS = (
    "colorama==0.4.6",
    "iniconfig==2.3.0",
    "packaging==26.3",
    "pluggy==1.6.0",
    "Pygments==2.20.0",
    f"pytest=={_PYTEST_VERSION}",
    f"ruff=={_RUFF_VERSION}",
)


class BootstrapError(RuntimeError):
    """One deterministic validation-environment bootstrap failure."""


def _root() -> Path:
    return Path(__file__).resolve().parents[2]


def _default_source_python(root: Path) -> Path:
    if os.name == "nt":
        return (
            root
            / ".dependencies/jig/source/.dependencies/python"
            / f"{_PYTHON_VERSION}-windows-x86_64/python.exe"
        )
    raise BootstrapError("pass --python on non-Windows validation hosts")


def _venv_paths(root: Path) -> tuple[Path, Path, Path]:
    environment = root / ".dependencies/python" / _PYTHON_VERSION
    if os.name == "nt":
        scripts = environment / "Scripts"
        return environment, scripts / "python.exe", scripts / "pytest.exe"
    scripts = environment / "bin"
    return environment, scripts / "python", scripts / "pytest"


def _ruff_path(environment: Path) -> Path:
    return environment / ("Scripts/ruff.exe" if os.name == "nt" else "bin/ruff")


def _run(command: list[str], *, cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    try:
        return subprocess.run(
            command,
            cwd=cwd,
            check=True,
            capture_output=True,
            text=True,
            timeout=600,
        )
    except (OSError, subprocess.SubprocessError) as error:
        rendered = " ".join(command)
        raise BootstrapError(f"command failed: {rendered}: {error}") from error


def _python_version(executable: Path) -> str:
    result = _run(
        [
            str(executable),
            "-c",
            "import platform; print(platform.python_version())",
        ]
    )
    return result.stdout.strip()


def _tool_version(executable: Path) -> str:
    return _run([str(executable), "--version"]).stdout.strip()


def _validate_source_python(executable: Path) -> None:
    if not executable.is_file():
        raise BootstrapError(f"source Python does not exist: {executable}")
    actual = _python_version(executable)
    if actual != _PYTHON_VERSION:
        raise BootstrapError(
            f"CPython {_PYTHON_VERSION} is required; source reports {actual or 'unknown'}"
        )


def _evidence(environment: Path, python: Path, pytest: Path, ruff: Path) -> dict[str, str]:
    if not python.is_file() or not pytest.is_file() or not ruff.is_file():
        raise BootstrapError("validation environment is incomplete")
    python_version = _python_version(python)
    pytest_version = _tool_version(pytest)
    ruff_version = _tool_version(ruff)
    if python_version != _PYTHON_VERSION:
        raise BootstrapError(f"validation Python drift: {python_version}")
    if _PYTEST_VERSION not in pytest_version:
        raise BootstrapError(f"pytest version drift: {pytest_version}")
    if _RUFF_VERSION not in ruff_version:
        raise BootstrapError(f"Ruff version drift: {ruff_version}")
    return {
        "environment": environment.as_posix(),
        "python": python_version,
        "pytest": pytest_version,
        "ruff": ruff_version,
        "schema": _SCHEMA,
    }


def _remove_real_directory(path: Path) -> None:
    if not path.exists():
        return
    if path.is_symlink() or not path.is_dir():
        raise BootstrapError(f"refusing to replace non-directory dependency path: {path}")
    shutil.rmtree(path)


def prepare(root: Path, source_python: Path, *, replace: bool) -> dict[str, str]:
    """Create or verify the exact local Python validation environment."""
    root = root.resolve(strict=True)
    source_python = source_python.resolve(strict=False)
    _validate_source_python(source_python)
    environment, python, pytest = _venv_paths(root)
    ruff = _ruff_path(environment)
    if environment.exists() and not replace:
        return _evidence(environment, python, pytest, ruff)

    parent = environment.parent
    parent.mkdir(parents=True, exist_ok=True)
    backup = parent / f".{environment.name}.validation-{os.getpid()}.backup"
    if backup.exists():
        raise BootstrapError("validation environment backup path already exists")

    had_previous = environment.exists()
    if had_previous:
        if environment.is_symlink() or not environment.is_dir():
            raise BootstrapError(
                f"refusing to replace non-directory dependency path: {environment}"
            )
        os.replace(environment, backup)

    try:
        _run([str(source_python), "-m", "venv", str(environment)])
        _run(
            [
                str(python),
                "-m",
                "pip",
                "install",
                "--disable-pip-version-check",
                "--no-input",
                "--only-binary=:all:",
                "--no-deps",
                *_REQUIREMENTS,
            ]
        )
        result = _evidence(environment, python, pytest, ruff)
    except BaseException:
        if environment.exists():
            _remove_real_directory(environment)
        if had_previous and backup.exists():
            os.replace(backup, environment)
        raise

    if backup.exists():
        _remove_real_directory(backup)
    return result


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        prog="shar-validation-python",
        description="Prepare exact local pytest/Ruff dependencies for Jig.",
        allow_abbrev=False,
    )
    parser.add_argument("--python", type=Path)
    parser.add_argument("--replace", action="store_true")
    args = parser.parse_args(argv)
    root = _root()
    source_python = args.python or _default_source_python(root)
    try:
        result = prepare(root, source_python, replace=args.replace)
    except (BootstrapError, OSError) as error:
        print(f"shar-validation-python: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

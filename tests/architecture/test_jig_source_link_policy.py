# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT

"""Tracked policy guards for SHAR's source-linked Jig development install."""

from __future__ import annotations

import tomllib
from pathlib import Path

_ROOT = Path(__file__).resolve().parents[2]
_CONFIG = _ROOT / ".jig" / "jig.toml"
_SOURCE_PREFIX = ".dependencies/jig/source/"
_SHARED_SOURCE_TOOLS = {
    "cspell",
    "git",
    "markdownlint",
    "rust_nightly_cargo",
    "rust_nightly_cargo_clippy",
    "rust_nightly_cargo_fmt",
    "rust_stable_cargo",
}


def test_shared_jig_tools_resolve_through_source_link() -> None:
    """Prevent tracked validation paths from bypassing Jig's source link."""
    with _CONFIG.open("rb") as stream:
        config = tomllib.load(stream)
    tools = config.get("tool")
    assert isinstance(tools, dict), ".jig/jig.toml must declare [tool] entries"

    offenders: list[str] = []
    for name in sorted(_SHARED_SOURCE_TOOLS):
        entry = tools.get(name)
        if not isinstance(entry, dict):
            offenders.append(f"{name}:missing")
            continue
        path = entry.get("path")
        if not isinstance(path, str) or not path.startswith(_SOURCE_PREFIX):
            offenders.append(f"{name}:{path!r}")

    assert not offenders, (
        "shared Jig validation tools must resolve through "
        f"{_SOURCE_PREFIX}: {offenders}"
    )


_PROJECT_PYTHON_TOOLS = {
    "pytest": ".dependencies/python/3.14.6/Scripts/pytest.exe",
    "ruff": ".dependencies/python/3.14.6/Scripts/ruff.exe",
}


def test_project_python_tools_use_materialized_environment() -> None:
    """Keep pytest/Ruff on direct repository-local executables, not stale shims."""
    with _CONFIG.open("rb") as stream:
        config = tomllib.load(stream)
    tools = config.get("tool")
    assert isinstance(tools, dict), ".jig/jig.toml must declare [tool] entries"
    mismatches: list[str] = []
    for name, expected in sorted(_PROJECT_PYTHON_TOOLS.items()):
        entry = tools.get(name)
        actual = entry.get("path") if isinstance(entry, dict) else None
        if actual != expected:
            mismatches.append(f"{name}:{actual!r}")
    bootstrap = _ROOT / "tools" / "validation" / "python_dependencies.py"
    assert bootstrap.is_file(), "tracked Python validation bootstrap is missing"
    assert not mismatches, f"project Python tool path drift: {mismatches}"


def test_pytest_gate_covers_all_python_test_roots() -> None:
    """Require every tracked top-level Python test root in canonical pytest."""
    with _CONFIG.open("rb") as stream:
        config = tomllib.load(stream)
    validation = config.get("validation")
    pytest_gate = validation.get("pytest") if isinstance(validation, dict) else None
    args = pytest_gate.get("args") if isinstance(pytest_gate, dict) else None
    assert isinstance(args, list), "validation.pytest.args must be configured"
    configured = {value for value in args if isinstance(value, str) and value.startswith("tests/")}
    required = {
        path.parent.as_posix()
        for path in (_ROOT / "tests").glob("*/test_*.py")
    }
    required.update(
        path.parent.as_posix()
        for path in (_ROOT / "tests" / "unreal").glob("*/test_*.py")
    )
    normalized = {
        str(Path(value).as_posix()) for value in configured
    }
    relative_required = {
        Path(value).relative_to(_ROOT).as_posix() if Path(value).is_absolute() else value
        for value in required
    }
    missing = sorted(relative_required - normalized)
    assert not missing, f"Python test roots missing from Jig pytest gate: {missing}"

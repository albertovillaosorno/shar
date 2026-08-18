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
#   - Host-portable repository-local native launchers used by Jig validation.
# - Must-Not:
#   - Install global packages, alter the Jig checkout, or write outside SHAR's
#     .dependencies/.cache roots.
# - Allows:
#   - Inputs: exact host Git/Node/npm plus repo-local Python/Rust
#     bootstrap state.
#   - Outputs: .dependencies/validation tools and exact version evidence.
#   - Side effects: pinned npm packages and Rust nightly below repository roots.
# - Split-When:
#   - Split when one native validation runtime gains an independent lifecycle.
# - Merge-When:
#   - Merge when the public dependency bootstrap owns identical
#     validation state.
# - Summary:
#   - Materializes portable Jig validation dependencies without global mutation.
# - Description:
#   - Owns host launchers plus Jig's isolated native Cargo runtime root.
# - Usage:
#   - Run before invoking Jig validation on a supported host.
# - Defaults:
#   - Uses pinned tool versions and repository-owned runtime directories.
#

"""Prepare host-portable repository-local tools required by Jig."""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import shlex
import shutil
import stat
import subprocess
import sys
from typing import NamedTuple

_GIT_VERSION = "2.55.0"
_NODE_VERSION = "24.18.0"
_NPM_VERSION = "11.16.0"
_PYTHON_VERSION = "3.14.6"
_PYTEST_VERSION = "9.1.1"
_RUFF_VERSION = "0.15.21"
_CSPELL_VERSION = "10.0.1"
_MARKDOWNLINT_VERSION = "0.23.0"
_STABLE_RUST = "1.97.1"
_NIGHTLY_RUST = "nightly-2026-07-14"
_NIGHTLY_CARGO_VERSION = "1.99.0-nightly"
_NIGHTLY_CLIPPY_VERSION = "0.1.99"
_NIGHTLY_RUSTFMT_VERSION = "1.9.0-nightly"
_SCHEMA = "shar.validation-native.v1"


class BootstrapError(RuntimeError):
    """One deterministic native-validation bootstrap failure."""


class HostTools(NamedTuple):
    """Exact host programs admitted only as bootstrap/runtime interpreters."""

    git: Path
    node: Path
    npm: Path


class Launcher(NamedTuple):
    """One generated host-native launcher declaration."""

    command: tuple[str, ...]
    environment: tuple[tuple[str, str], ...] = ()


def _root() -> Path:
    return Path(__file__).resolve().parents[2]


def _run(
    command: list[str],
    *,
    environment: dict[str, str] | None = None,
    timeout: int = 600,
) -> subprocess.CompletedProcess[str]:
    try:
        return subprocess.run(
            command,
            check=True,
            capture_output=True,
            text=True,
            env=environment,
            timeout=timeout,
        )
    except (OSError, subprocess.SubprocessError) as error:
        rendered = " ".join(command)
        raise BootstrapError(f"command failed: {rendered}: {error}") from error


def _program(name: str) -> Path:
    executable = shutil.which(name)
    if executable is None:
        raise BootstrapError(f"required host command is missing: {name}")
    return Path(executable).resolve()


def _require_git(executable: Path) -> None:
    output = _run([str(executable), "--version"]).stdout.strip()
    match = re.fullmatch(r"git version (\S+)", output)
    actual = match.group(1) if match is not None else ""
    windows_version = actual.startswith(f"{_GIT_VERSION}.windows.")
    if actual != _GIT_VERSION and not windows_version:
        raise BootstrapError(
            f"Git {_GIT_VERSION} is required; "
            f"host reports {actual or 'unknown'}"
        )


def _require_simple_version(
    executable: Path,
    expected: str,
    label: str,
    *,
    prefix: str = "",
) -> None:
    output = _run([str(executable), "--version"]).stdout.strip()
    actual = output.removeprefix(prefix)
    if actual != expected:
        raise BootstrapError(
            f"{label} {expected} is required; "
            f"host reports {actual or 'unknown'}"
        )


def _host_tools() -> HostTools:
    git = _program("git")
    node = _program("node")
    npm = _program("npm.cmd" if os.name == "nt" else "npm")
    _require_git(git)
    _require_simple_version(node, _NODE_VERSION, "Node.js", prefix="v")
    _require_simple_version(npm, _NPM_VERSION, "npm")
    return HostTools(git=git, node=node, npm=npm)


def _python_tools(root: Path) -> tuple[Path, Path]:
    environment = root / ".dependencies/python" / _PYTHON_VERSION
    scripts = environment / ("Scripts" if os.name == "nt" else "bin")
    pytest = scripts / ("pytest.exe" if os.name == "nt" else "pytest")
    ruff = scripts / ("ruff.exe" if os.name == "nt" else "ruff")
    if not pytest.is_file() or not ruff.is_file():
        raise BootstrapError(
            "Python validation tools are missing; run "
            "tools/validation/python_dependencies.py --replace"
        )
    return pytest, ruff


def _rust_tools(root: Path) -> tuple[Path, dict[str, str]]:
    cargo_home = root / ".dependencies/build/rustup-cargo"
    rustup_home = root / ".dependencies/build/rustup"
    rustup_name = "rustup.exe" if os.name == "nt" else "rustup"
    rustup = cargo_home / "bin" / rustup_name
    cargo = cargo_home / "bin" / ("cargo.exe" if os.name == "nt" else "cargo")
    if not rustup.is_file() or not cargo.is_file():
        raise BootstrapError(
            "repo-local Rust bootstrap is missing; "
            "run tools/build/dependencies.py"
        )
    environment = os.environ.copy()
    environment["RUSTUP_HOME"] = str(rustup_home)
    environment["CARGO_HOME"] = str(cargo_home)
    _run(
        [
            str(rustup),
            "toolchain",
            "install",
            _NIGHTLY_RUST,
            "--profile",
            "minimal",
            "--component",
            "clippy",
            "--component",
            "rustfmt",
        ],
        environment=environment,
        timeout=1200,
    )
    return cargo, {
        "RUSTUP_HOME": str(rustup_home),
        "CARGO_HOME": str(cargo_home),
    }


def _install_javascript(root: Path, tools: HostTools) -> tuple[Path, Path]:
    install = root / ".dependencies/validation/node"
    cache = root / ".cache/validation/npm"
    install.mkdir(parents=True, exist_ok=True)
    cache.mkdir(parents=True, exist_ok=True)
    environment = os.environ.copy()
    environment["npm_config_cache"] = str(cache)
    _run(
        [
            str(tools.npm),
            "install",
            "--prefix",
            str(install),
            "--no-save",
            "--no-package-lock",
            "--ignore-scripts",
            "--no-audit",
            "--no-fund",
            f"cspell@{_CSPELL_VERSION}",
            f"markdownlint-cli2@{_MARKDOWNLINT_VERSION}",
        ],
        environment=environment,
        timeout=1200,
    )
    cspell = install / "node_modules/cspell/bin.mjs"
    markdownlint = (
        install / "node_modules/markdownlint-cli2/markdownlint-cli2-bin.mjs"
    )
    if not cspell.is_file() or not markdownlint.is_file():
        raise BootstrapError("npm validation packages are incomplete")
    return cspell, markdownlint


def _launchers(
    root: Path,
    host: HostTools,
    *,
    cargo: Path,
    rust_environment: dict[str, str],
    cspell: Path,
    markdownlint: Path,
) -> dict[str, Launcher]:
    pytest, ruff = _python_tools(root)
    rust_env = tuple(sorted(rust_environment.items()))
    return {
        "cspell.cmd": Launcher(
            (str(host.node), "--no-global-search-paths", str(cspell))
        ),
        "git.cmd": Launcher((str(host.git),)),
        "markdownlint.cmd": Launcher(
            (str(host.node), "--no-global-search-paths", str(markdownlint))
        ),
        "node.cmd": Launcher((str(host.node),)),
        "pytest.cmd": Launcher((str(pytest),)),
        "ruff.cmd": Launcher((str(ruff),)),
        "rust-nightly-cargo.cmd": Launcher(
            (str(cargo), f"+{_NIGHTLY_RUST}"), rust_env
        ),
        "rust-nightly-clippy.cmd": Launcher(
            (str(cargo), f"+{_NIGHTLY_RUST}", "clippy"), rust_env
        ),
        "rust-nightly-fmt.cmd": Launcher(
            (str(cargo), f"+{_NIGHTLY_RUST}", "fmt"), rust_env
        ),
        "rust-stable-cargo.cmd": Launcher(
            (str(cargo), f"+{_STABLE_RUST}"), rust_env
        ),
    }


def _posix_launcher(launcher: Launcher) -> str:
    lines = ["#!/bin/sh", "set -eu"]
    for key, value in launcher.environment:
        lines.append(f"export {key}={shlex.quote(value)}")
    command = " ".join(shlex.quote(value) for value in launcher.command)
    lines.append(f'exec {command} "$@"')
    return "\n".join(lines) + "\n"


def _windows_launcher(launcher: Launcher) -> str:
    lines = ["@echo off", "setlocal"]
    for key, value in launcher.environment:
        lines.append(f'set "{key}={value}"')
    command = " ".join(f'"{value}"' for value in launcher.command)
    lines.extend((f"{command} %*", "exit /b %errorlevel%"))
    return "\r\n".join(lines) + "\r\n"


def _prepare_jig_cargo_home(root: Path) -> Path:
    """Create Jig's real native Cargo runtime root without duplicating tools."""
    cargo_home = root / ".dependencies/cargo-home"
    is_junction = getattr(os.path, "isjunction", None)
    redirected = is_junction is not None and is_junction(cargo_home)
    if cargo_home.is_symlink() or redirected:
        raise BootstrapError(
            f"Jig Cargo home may not be a redirect: {cargo_home}"
        )
    cargo_home.mkdir(parents=True, exist_ok=True)
    if not cargo_home.is_dir():
        raise BootstrapError(f"Jig Cargo home is not a directory: {cargo_home}")
    return cargo_home


def _write_launchers(
    root: Path,
    launchers: dict[str, Launcher],
) -> dict[str, Path]:
    directory = root / ".dependencies/validation/bin"
    directory.mkdir(parents=True, exist_ok=True)
    paths: dict[str, Path] = {}
    for name, launcher in sorted(launchers.items()):
        path = directory / name
        contents = (
            _windows_launcher(launcher)
            if os.name == "nt"
            else _posix_launcher(launcher)
        )
        path.write_text(contents, encoding="utf-8", newline="")
        if os.name != "nt":
            executable = stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH
            path.chmod(path.stat().st_mode | executable)
        paths[name] = path
    return paths


def _require_launcher_version(path: Path, expected: str, label: str) -> str:
    output = _run([str(path), "--version"], timeout=60).stdout.strip()
    tokens = output.split()
    admitted = any(
        token in {expected, f"v{expected}"}
        or token.startswith(f"{expected}.windows.")
        for token in tokens
    )
    if not admitted:
        raise BootstrapError(f"{label} version drift: {output or 'no output'}")
    return output


def prepare(root: Path) -> dict[str, object]:
    """Materialize and verify the complete host-portable Jig tool surface."""
    root = root.resolve(strict=True)
    _prepare_jig_cargo_home(root)
    host = _host_tools()
    cargo, rust_environment = _rust_tools(root)
    cspell, markdownlint = _install_javascript(root, host)
    launchers = _write_launchers(
        root,
        _launchers(
            root,
            host,
            cargo=cargo,
            rust_environment=rust_environment,
            cspell=cspell,
            markdownlint=markdownlint,
        ),
    )
    expected = {
        "cspell.cmd": (_CSPELL_VERSION, "CSpell"),
        "git.cmd": (_GIT_VERSION, "Git"),
        "markdownlint.cmd": (_MARKDOWNLINT_VERSION, "markdownlint-cli2"),
        "node.cmd": (_NODE_VERSION, "Node.js"),
        "pytest.cmd": (_PYTEST_VERSION, "pytest"),
        "ruff.cmd": (_RUFF_VERSION, "Ruff"),
        "rust-nightly-cargo.cmd": (_NIGHTLY_CARGO_VERSION, "nightly Cargo"),
        "rust-nightly-clippy.cmd": (_NIGHTLY_CLIPPY_VERSION, "nightly Clippy"),
        "rust-nightly-fmt.cmd": (_NIGHTLY_RUSTFMT_VERSION, "nightly rustfmt"),
        "rust-stable-cargo.cmd": (_STABLE_RUST, "stable Cargo"),
    }
    versions = {
        name: _require_launcher_version(launchers[name], version, label)
        for name, (version, label) in sorted(expected.items())
    }
    return {
        "root": (root / ".dependencies/validation").as_posix(),
        "schema": _SCHEMA,
        "tools": versions,
    }


def main() -> int:
    """Prepare portable native validation dependencies or fail closed."""
    try:
        result = prepare(_root())
    except (BootstrapError, OSError) as error:
        print(f"shar-validation-native: {error}", file=sys.stderr)
        return 1
    print(json.dumps(result, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

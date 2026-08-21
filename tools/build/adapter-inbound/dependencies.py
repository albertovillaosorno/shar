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
#   - Hermetic preparation of public build dependencies needed by SHAR.
# - Must-Not:
#   - Install global packages, vendor Unreal, or provision signing secrets.
# - Allows:
#   - Inputs: exact Python and Rust toolchain executables.
#   - Outputs: repository-owned validator binaries and dependency evidence.
#   - Side effects: Cargo downloads/builds only below repository-owned roots.
# - Split-When:
#   - Split when another public toolchain gains an independent lifecycle.
# - Merge-When:
#   - Merge when preflight owns identical dependency preparation.
# - Summary:
#   - Prepares deterministic public build dependencies without global mutation.
# - Description:
#   - Uses Cargo.lock and repository-owned Cargo home/target directories.
# - Usage:
#   - Run tools/build/adapter-inbound/dependencies.py before check.py.
# - Defaults:
#   - Requires CPython 3.14.6 and Rust/Cargo 1.97.1.
#

"""Prepare SHAR public build dependencies in repository-owned locations."""

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
import subprocess
import sys
import tomllib
from typing import NamedTuple
import urllib.error
import urllib.parse
import urllib.request

_SCHEMA = "shar.build.dependencies.v1"
_PYTHON_VERSION = (3, 14, 6)
_DATA_PATH = Path(".cache/build/data/dependencies.json")
_CARGO_HOME = Path(".cache/build/cargo-home")
_CARGO_TARGET = Path(".cache/build/cargo-target")
_BIN_ROOT = Path(".dependencies/build/bin")
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
_RUSTUP_VERSION = "1.29.0"
_RUSTUP_HOME = Path(".dependencies/build/rustup")
_RUSTUP_CARGO_HOME = Path(".dependencies/build/rustup-cargo")
_BOOTSTRAP_CACHE = Path(".cache/build/bootstrap")
_RUSTUP_SHA256 = {
    "aarch64-apple-darwin": (
        "aeb4105778ca1bd3c6b0e75768f581c656633cd51368fa61289b6a71696ac7e1"
    ),
    "aarch64-pc-windows-msvc": (
        "3af309e6c3062aa11df0e932954f69d13b734d8a431e593812f3ecd9ff9e6ef6"
    ),
    "aarch64-unknown-linux-gnu": (
        "9732d6c5e2a098d3521fca8145d826ae0aaa067ef2385ead08e6feac88fa5792"
    ),
    "x86_64-apple-darwin": (
        "33cf85df9142bc6d29cbc62fa5ca1d4c29622cddb55213a4c1a43c457fb9b2d7"
    ),
    "x86_64-pc-windows-msvc": (
        "86478e53f769379d7f0ebfa7c9aa97cb76ca92233f79aa2cc0dbee2efaac73c7"
    ),
    "x86_64-unknown-linux-gnu": (
        "4acc9acc76d5079515b46346a485974457b5a79893cfb01112423c89aeb5aa10"
    ),
}


class BootstrapFailure(RuntimeError):
    """One actionable dependency-bootstrap failure."""


class CargoBuildContext(NamedTuple):
    """Exact Rust tools and process environment for one Cargo build."""

    cargo: Path
    rustc: Path
    binutils: Path | None
    environment: dict[str, str]


def _root() -> Path:
    """Return the repository root from this script's tracked location."""
    return Path(__file__).resolve().parents[3]


def _sha256(path: Path) -> str:
    """Return a lowercase SHA-256 digest for one file."""
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
        payload = source.read_bytes()
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


def _require_python() -> dict[str, str]:
    """Require the exact supported CPython bootstrap version."""
    actual = sys.version_info[:3]
    if actual != _PYTHON_VERSION:
        wanted = ".".join(str(value) for value in _PYTHON_VERSION)
        found = ".".join(str(value) for value in actual)
        raise BootstrapFailure(
            f"Python {wanted} is required; this command is using {found}"
        )
    return {
        "executable": str(Path(sys.executable).resolve()),
        "version": platform.python_version(),
    }


def _required_rust_version(root: Path) -> str:
    """Read the workspace minimum Rust version from Cargo.toml."""
    with (root / "Cargo.toml").open("rb") as handle:
        cargo = tomllib.load(handle)
    try:
        value = cargo["workspace"]["package"]["rust-version"]
    except (KeyError, TypeError) as error:
        message = "Cargo.toml has no workspace rust-version"
        raise BootstrapFailure(message) from error
    if not isinstance(value, str) or not re.fullmatch(r"\d+\.\d+\.\d+", value):
        raise BootstrapFailure("Cargo.toml rust-version is invalid")
    return value


def _host_rust_target() -> str:
    """Return the pinned Rust host target for the current build machine."""
    system = platform.system().casefold()
    machine = platform.machine().casefold()
    architectures = {
        "aarch64": "aarch64",
        "amd64": "x86_64",
        "arm64": "aarch64",
        "x86_64": "x86_64",
    }
    architecture = architectures.get(machine)
    if architecture is None:
        message = f"unsupported Rust bootstrap architecture: {machine}"
        raise BootstrapFailure(message)
    if system == "windows":
        target = f"{architecture}-pc-windows-msvc"
    elif system == "linux":
        target = f"{architecture}-unknown-linux-gnu"
    elif system == "darwin":
        target = f"{architecture}-apple-darwin"
    else:
        raise BootstrapFailure(f"unsupported Rust bootstrap host: {system}")
    if target not in _RUSTUP_SHA256:
        raise BootstrapFailure(f"unsupported Rust bootstrap target: {target}")
    return target


def _rustup_installer_path(root: Path, target: str) -> Path:
    """Return the cache path for the pinned rustup installer."""
    suffix = ".exe" if "windows" in target else ""
    filename = f"rustup-init-{_RUSTUP_VERSION}-{target}{suffix}"
    return root / _BOOTSTRAP_CACHE / filename


def _download_rustup(root: Path, target: str) -> Path:
    """Download and verify the exact pinned rustup installer."""
    installer = _rustup_installer_path(root, target)
    expected = _RUSTUP_SHA256[target]
    if installer.is_file() and _sha256(installer) == expected:
        return installer
    installer.parent.mkdir(parents=True, exist_ok=True)
    suffix = ".exe" if "windows" in target else ""
    source_name = f"rustup-init{suffix}"
    url = (
        "https://static.rust-lang.org/rustup/archive/"
        f"{_RUSTUP_VERSION}/{target}/{source_name}"
    )
    candidate = installer.with_name(f".{installer.name}.{os.getpid()}.tmp")
    parsed = urllib.parse.urlsplit(url)
    if parsed.scheme != "https" or parsed.hostname != "static.rust-lang.org":
        raise BootstrapFailure(
            "rustup installer URL is not an approved HTTPS origin"
        )
    request = urllib.request.Request(  # noqa: S310 - origin checked above.
        url, method="GET"
    )
    try:
        with (
            urllib.request.urlopen(  # noqa: S310 - URL origin checked above.
                request, timeout=120
            ) as response,
            candidate.open("wb") as handle,
        ):
            shutil.copyfileobj(response, handle)
        actual = _sha256(candidate)
        if actual != expected:
            raise BootstrapFailure(
                "rustup installer checksum mismatch: "
                f"expected {expected}, found {actual}"
            )
        if os.name != "nt":
            candidate.chmod(candidate.stat().st_mode | 0o111)
        Path(candidate).replace(installer)
    except urllib.error.URLError as error:
        message = f"cannot download pinned rustup: {error}"
        raise BootstrapFailure(message) from error
    finally:
        candidate.unlink(missing_ok=True)
    return installer


def _repo_rust_paths(
    root: Path,
    required: str,
    target: str,
) -> tuple[Path, Path]:
    """Return direct Cargo and rustc paths for the repo-local toolchain."""
    toolchain = f"{required}-{target}"
    bin_root = root / _RUSTUP_HOME / "toolchains" / toolchain / "bin"
    suffix = ".exe" if "windows" in target else ""
    return bin_root / f"cargo{suffix}", bin_root / f"rustc{suffix}"


def _install_repo_rust(
    root: Path,
    required: str,
) -> tuple[Path, Path, dict[str, str]]:
    """Install the exact Rust toolchain under repository-owned directories."""
    target = _host_rust_target()
    cargo, rustc = _repo_rust_paths(root, required, target)
    if cargo.is_file() and rustc.is_file():
        installer = _rustup_installer_path(root, target)
        installer_hash = None
        installer_path = None
        if installer.is_file():
            actual = _sha256(installer)
            if actual == _RUSTUP_SHA256[target]:
                installer_hash = actual
                installer_path = str(installer.resolve())
        evidence = {
            "installer": installer_path,
            "installer_sha256": installer_hash,
            "target": target,
            "version": _RUSTUP_VERSION,
        }
        return cargo.resolve(), rustc.resolve(), evidence
    installer = _download_rustup(root, target)
    installer_sha256 = _sha256(installer)
    if not cargo.is_file() or not rustc.is_file():
        rustup_home = root / _RUSTUP_HOME
        cargo_home = root / _RUSTUP_CARGO_HOME
        rustup_home.mkdir(parents=True, exist_ok=True)
        cargo_home.mkdir(parents=True, exist_ok=True)
        environment = os.environ.copy()
        environment["CARGO_HOME"] = str(cargo_home)
        environment["RUSTUP_HOME"] = str(rustup_home)
        toolchain = f"{required}-{target}"
        command = [
            str(installer),
            "-y",
            "--no-modify-path",
            "--profile",
            "minimal",
            "--default-toolchain",
            toolchain,
        ]
        try:
            subprocess.run(command, env=environment, check=True)
        except (OSError, subprocess.CalledProcessError) as error:
            raise BootstrapFailure(
                f"rustup could not install Rust {toolchain}"
            ) from error
    if not cargo.is_file() or not rustc.is_file():
        raise BootstrapFailure("rustup did not publish the requested toolchain")
    evidence = {
        "installer": str(installer.resolve()),
        "installer_sha256": installer_sha256,
        "target": target,
        "version": _RUSTUP_VERSION,
    }
    return cargo.resolve(), rustc.resolve(), evidence


def _resolve_executable(
    root: Path,
    explicit: Path | None,
    environment_name: str,
    command_name: str,
) -> Path:
    """Resolve one explicit, environment, or PATH tool without installing it."""
    if explicit is not None:
        candidate = explicit if explicit.is_absolute() else root / explicit
        if candidate.is_file():
            return candidate.resolve()
        raise BootstrapFailure(f"{command_name} does not exist: {candidate}")
    environment = os.environ.get(environment_name)
    if environment:
        candidate = Path(environment)
        if candidate.is_file():
            return candidate.resolve()
    found = shutil.which(command_name)
    if found:
        return Path(found).resolve()
    raise BootstrapFailure(
        f"{command_name} is required; pass --{command_name} or set "
        f"{environment_name}"
    )


def _rustc_for_cargo(root: Path, cargo: Path, explicit: Path | None) -> Path:
    """Resolve rustc beside Cargo so an explicit toolchain stays paired."""
    if explicit is not None:
        return _resolve_executable(root, explicit, "RUSTC", "rustc")
    sibling = cargo.with_name("rustc.exe" if os.name == "nt" else "rustc")
    if sibling.is_file():
        return sibling.resolve()
    return _resolve_executable(root, None, "RUSTC", "rustc")


def _tool_version(tool: Path, name: str, required: str) -> str:
    """Require an exact semantic tool version prefix."""
    try:
        result = subprocess.run(
            [str(tool), "--version"],
            check=True,
            capture_output=True,
            text=True,
            timeout=30,
        )
    except (OSError, subprocess.SubprocessError) as error:
        raise BootstrapFailure(f"cannot run {name}: {error}") from error
    version = result.stdout.strip()
    expected = f"{name} {required} "
    if version != f"{name} {required}" and not version.startswith(expected):
        raise BootstrapFailure(
            f"{name} {required} is required; found {version or "unknown"}"
        )
    return version


def _rust_host(rustc: Path) -> str:
    """Return the exact rustc host triple used by the bootstrap."""
    try:
        result = subprocess.run(
            [str(rustc), "-vV"],
            check=True,
            capture_output=True,
            text=True,
            timeout=30,
        )
    except (OSError, subprocess.SubprocessError) as error:
        raise BootstrapFailure(f"cannot inspect rustc host: {error}") from error
    for line in result.stdout.splitlines():
        if line.startswith("host: "):
            return line.removeprefix("host: ").strip()
    raise BootstrapFailure("rustc -vV did not report a host triple")


def _resolve_binutils(
    root: Path,
    explicit: Path | None,
    host: str,
) -> Path | None:
    """Resolve GNU assembler tools only when the Rust host requires them."""
    if not host.endswith("windows-gnu"):
        return None
    if explicit is not None:
        directory = explicit if explicit.is_absolute() else root / explicit
        assembler = directory / "as.exe"
        if assembler.is_file():
            return directory.resolve()
        raise BootstrapFailure(f"GNU binutils has no as.exe: {directory}")
    assembler = shutil.which("as.exe") or shutil.which("as")
    if assembler:
        return Path(assembler).resolve().parent
    raise BootstrapFailure(
        "GNU Rust requires binutils; pass --binutils with a directory "
        "containing as.exe"
    )


def _visual_studio_environment(  # noqa: PLR0912, PLR0914
    root: Path,
    host: str,
) -> tuple[dict[str, str], dict[str, str] | None]:
    """Load one process-local MSVC environment when the Rust host needs it."""
    environment = os.environ.copy()
    if not host.endswith("pc-windows-msvc"):
        return environment, None
    program_files_x86 = (
        environment.get("ProgramFiles(x86)")
        or environment.get("PROGRAMFILES(X86)")
        or r"C:\Program Files (x86)"
    )
    vswhere = (
        Path(program_files_x86)
        / "Microsoft Visual Studio"
        / "Installer"
        / "vswhere.exe"
    )
    if not vswhere.is_file():
        raise BootstrapFailure(
            "Visual Studio Build Tools are required for the Windows Rust host"
        )
    query = [
        str(vswhere),
        "-latest",
        "-products",
        "*",
        "-property",
        "installationPath",
    ]
    try:
        result = subprocess.run(
            query,
            check=True,
            capture_output=True,
            text=True,
            timeout=30,
        )
    except (OSError, subprocess.SubprocessError) as error:
        message = f"cannot query Visual Studio: {error}"
        raise BootstrapFailure(message) from error
    installation = Path(result.stdout.strip())
    if not installation.is_dir():
        raise BootstrapFailure("Visual Studio C++ Build Tools were not found")
    script_name = (
        "vcvarsamd64_arm64.bat"
        if host.startswith("aarch64-")
        else "vcvars64.bat"
    )
    script = installation / "VC" / "Auxiliary" / "Build" / script_name
    if not script.is_file():
        raise BootstrapFailure(f"Visual Studio is missing {script_name}")
    comspec = environment.get("COMSPEC", "cmd.exe")
    command_root = root / _BOOTSTRAP_CACHE
    command_root.mkdir(parents=True, exist_ok=True)
    batch = command_root / f"vsenv-{os.getpid()}.cmd"
    newline = chr(13) + chr(10)
    batch_text = newline.join((
        "@echo off",
        f'call "{script}" >nul',
        "if errorlevel 1 exit /b %errorlevel%",
        "set",
        "",
    ))
    system_root = Path(environment.get("SystemRoot", r"C:\Windows"))
    bootstrap_environment = environment.copy()
    bootstrap_environment["PATH"] = os.pathsep.join((
        str(system_root / "System32"),
        str(system_root),
        str(system_root / "System32" / "Wbem"),
    ))
    try:
        batch.write_text(
            batch_text,
            encoding="utf-8",
            newline="",
        )
        result = subprocess.run(
            [comspec, "/d", "/c", str(batch)],
            env=bootstrap_environment,
            check=True,
            capture_output=True,
            text=True,
            timeout=60,
        )
    except (OSError, subprocess.SubprocessError) as error:
        raise BootstrapFailure(
            f"cannot initialize Visual Studio C++ tools: {error}"
        ) from error
    finally:
        batch.unlink(missing_ok=True)
    configured = bootstrap_environment.copy()
    for line in result.stdout.splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        if key:
            configured[key] = value
    tool_path = configured.get("PATH", "")
    compiler = shutil.which("cl.exe", path=tool_path)
    linker = shutil.which("link.exe", path=tool_path)
    if compiler is None or linker is None:
        message = "Visual Studio C++ compiler/linker are unavailable"
        raise BootstrapFailure(message)
    evidence = {
        "compiler": str(Path(compiler).resolve()),
        "installation": str(installation.resolve()),
        "linker": str(Path(linker).resolve()),
    }
    return configured, evidence


def _build_cargo_binary(
    root: Path,
    context: CargoBuildContext,
    *,
    package: str,
    binary: str,
) -> Path:
    """Build one canonical validator using locked Cargo inputs."""
    cargo_home = root / _CARGO_HOME
    target = root / _CARGO_TARGET
    cargo_home.mkdir(parents=True, exist_ok=True)
    target.mkdir(parents=True, exist_ok=True)
    environment = context.environment.copy()
    environment["CARGO_HOME"] = str(cargo_home)
    environment["CARGO_TARGET_DIR"] = str(target)
    environment["RUSTC"] = str(context.rustc)
    path_parts = [str(context.rustc.parent)]
    if context.binutils is not None:
        path_parts.append(str(context.binutils))
    previous_path = environment.get("PATH")
    if previous_path:
        path_parts.append(previous_path)
    environment["PATH"] = os.pathsep.join(path_parts)
    command = [
        str(context.cargo),
        "build",
        "--locked",
        "--release",
        "-p",
        package,
        "--bin",
        binary,
    ]
    try:
        subprocess.run(
            command,
            cwd=root,
            env=environment,
            check=True,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        raise BootstrapFailure(
            f"Cargo could not build {binary}; verify the exact Rust "
            "toolchain and host linker prerequisites"
        ) from error
    name = f"{binary}.exe" if os.name == "nt" else binary
    built = target / "release" / name
    if not built.is_file():
        raise BootstrapFailure(f"Cargo did not produce {built}")
    return built


def _publish_validator(root: Path, built: Path) -> Path:
    """Atomically publish one validator when its content changed."""
    destination = root / _BIN_ROOT / built.name
    destination.parent.mkdir(parents=True, exist_ok=True)
    if os.path.lexists(destination):
        is_real = (
            destination.is_file()
            and not destination.is_symlink()
            and not os.path.isjunction(destination)
            and destination.stat(follow_symlinks=False).st_nlink == 1
        )
        if not is_real:
            raise BootstrapFailure(
                f"validator destination must be a real file: {destination}"
            )
    if destination.is_file() and _sha256(destination) == _sha256(built):
        return destination.resolve()
    candidate = destination.with_name(f".{destination.name}.{os.getpid()}.tmp")
    try:
        shutil.copy2(built, candidate)
        Path(candidate).replace(destination)
    finally:
        candidate.unlink(missing_ok=True)
    return destination.resolve()


def _validator_source_hashes(root: Path) -> tuple[str, str]:
    """Return manifest and deep-validator source-closure fingerprints."""
    return (
        _validator_source_sha256(root),
        _deep_validator_source_sha256(root),
    )


def _require_validator_source_hashes(
    root: Path,
    expected: tuple[str, str],
    phase: str,
) -> None:
    """Require validator source closures to remain stable across one phase."""
    if _validator_source_hashes(root) != expected:
        raise BootstrapFailure(
            f"validator source inputs changed during {phase}; rerun bootstrap"
        )


def _validator_evidence(
    root: Path,
    context: CargoBuildContext,
    *,
    publish_validator: bool,
) -> tuple[dict[str, str], dict[str, str]]:
    """Build both source validators and return their publication evidence."""
    source_hashes = _validator_source_hashes(root)
    built = _build_cargo_binary(
        root,
        context,
        package="game_manifest",
        binary="validate-game",
    )
    deep_built = _build_cargo_binary(
        root,
        context,
        package="shar_source_audit",
        binary="validate-source-deep",
    )
    _require_validator_source_hashes(root, source_hashes, "build")
    validator = (
        _publish_validator(root, built)
        if publish_validator
        else built.resolve()
    )
    deep_validator = (
        _publish_validator(root, deep_built)
        if publish_validator
        else deep_built.resolve()
    )
    _require_validator_source_hashes(root, source_hashes, "publication")
    return (
        {
            "path": str(validator),
            "sha256": _sha256(validator),
            "source_sha256": source_hashes[0],
        },
        {
            "path": str(deep_validator),
            "sha256": _sha256(deep_validator),
            "source_sha256": source_hashes[1],
        },
    )


def _require_real_storage_roots(roots: tuple[tuple[Path, str], ...]) -> None:
    """Reject linked or malformed repository-owned storage roots."""
    for path, label in roots:
        if not os.path.lexists(path):
            continue
        is_real = (
            path.is_dir()
            and not path.is_symlink()
            and not os.path.isjunction(path)
        )
        if is_real:
            continue
        raise BootstrapFailure(f"{label} must be a real directory: {path}")


def _validate_dependency_storage_roots(root: Path) -> None:
    """Keep bootstrap caches and installed build tools inside the repo."""
    roots = (
        (root / ".cache", "repository cache root"),
        (root / ".cache/build", "build cache root"),
        (root / _BOOTSTRAP_CACHE, "bootstrap cache root"),
        (root / _CARGO_HOME, "Cargo cache root"),
        (root / _CARGO_TARGET, "Cargo target root"),
        (root / ".dependencies", "repository dependency root"),
        (root / ".dependencies/build", "build dependency root"),
        (root / _BIN_ROOT, "validator binary root"),
        (root / _RUSTUP_HOME, "rustup home"),
        (root / _RUSTUP_CARGO_HOME, "rustup Cargo home"),
    )
    _require_real_storage_roots(roots)


def _validate_canonical_output_root(root: Path, output: Path) -> bool:
    """Reject a linked canonical data root and identify canonical output."""
    canonical = root / _DATA_PATH
    if output != canonical:
        return False
    _require_real_storage_roots(
        ((root / ".cache/build/data", "build data root"),)
    )
    if os.path.lexists(output) and (
        not output.is_file() or output.is_symlink()
    ):
        raise BootstrapFailure(
            f"dependency evidence must be a real file: {output}"
        )
    return True


def _write_json(path: Path, value: dict[str, object]) -> None:
    """Atomically persist machine-readable dependency evidence."""
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


def _run(
    args: argparse.Namespace,
    *,
    publish_validator: bool,
) -> dict[str, object]:
    """Validate the bootstrap and prepare the canonical public validator."""
    root = _root()
    python = _require_python()
    required_rust = _required_rust_version(root)
    rustup: dict[str, str | None] | None = None
    if args.cargo is None:
        if args.rustc is not None or args.binutils is not None:
            raise BootstrapFailure(
                "--rustc and --binutils require an explicit --cargo"
            )
        cargo, rustc, rustup = _install_repo_rust(root, required_rust)
    else:
        cargo = _resolve_executable(root, args.cargo, "CARGO", "cargo")
        rustc = _rustc_for_cargo(root, cargo, args.rustc)
    cargo_version = _tool_version(cargo, "cargo", required_rust)
    rustc_version = _tool_version(rustc, "rustc", required_rust)
    rust_host = _rust_host(rustc)
    binutils = _resolve_binutils(root, args.binutils, rust_host)
    environment, msvc = _visual_studio_environment(root, rust_host)
    context = CargoBuildContext(cargo, rustc, binutils, environment)
    validator, deep_validator = _validator_evidence(
        root,
        context,
        publish_validator=publish_validator,
    )
    return {
        "cargo": {
            "executable": str(cargo),
            "home": str((root / _CARGO_HOME).resolve()),
            "target": str((root / _CARGO_TARGET).resolve()),
            "version": cargo_version,
        },
        "cargo_lock_sha256": _sha256(root / "Cargo.lock"),
        "external_prerequisites": {
            "platform_sdks": "validated by target build stages",
            "signing_material": "user-provided when a target requires it",
            "unreal_engine": (
                "5.8.1; validated by "
                "tools/build/adapter-inbound/check.py"
            ),
            "visual_studio": msvc,
        },
        "python": python,
        "rustup": rustup,
        "rustc": {
            "binutils": str(binutils) if binutils is not None else None,
            "executable": str(rustc),
            "host": rust_host,
            "version": rustc_version,
        },
        "schema": _SCHEMA,
        "validator": validator,
        "deep_source_validator": deep_validator,
    }


def _parser() -> argparse.ArgumentParser:
    """Build the deterministic dependency-bootstrap CLI."""
    parser = argparse.ArgumentParser(
        description="Prepare SHAR public build dependencies hermetically.",
    )
    parser.add_argument(
        "--cargo",
        type=Path,
        help="override the repo-local Rust bootstrap with an exact Cargo",
    )
    parser.add_argument(
        "--rustc",
        type=Path,
        help="exact rustc executable; defaults beside the selected Cargo",
    )
    parser.add_argument(
        "--binutils",
        type=Path,
        help="GNU binutils directory when the selected Rust host needs it",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="override dependency evidence path for testing",
    )
    return parser


def main() -> int:
    """Prepare dependencies or fail without publishing success evidence."""
    args = _parser().parse_args()
    root = _root()
    output = args.output or (root / _DATA_PATH)
    if not output.is_absolute():
        output = root / output
    output_safe_to_mutate = False
    try:
        _validate_dependency_storage_roots(root)
        publish_validator = _validate_canonical_output_root(root, output)
        output_safe_to_mutate = True
        evidence = _run(
            args,
            publish_validator=publish_validator,
        )
        _write_json(output, evidence)
    except (BootstrapFailure, OSError, tomllib.TOMLDecodeError) as error:
        if output_safe_to_mutate:
            output.unlink(missing_ok=True)
        print(f"dependencies: {error}", file=sys.stderr)
        return 1
    print(f"dependencies: clean; saved evidence to {output.resolve()}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

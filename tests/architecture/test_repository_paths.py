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
#   - Repository validation regression tests.
# - Must-Not:
#   - Publish private game inputs or mutate external repositories.
# - Allows:
#   - Repository-local policy and bootstrap inspection.
# - Split-When:
#   - One validation policy gains an independent lifecycle.
# - Merge-When:
#   - The guarded policies become one inseparable repository contract.
# - Summary:
#   - Guards repository validation policy.
# - Description:
#   - Exercises tracked configuration and repository-local validation behavior.
# - Usage:
#   - Run through the canonical Jig pytest gate or repository-local pytest.
# - Defaults:
#   - Reads the current repository and writes only test-managed temporary state.
#

"""Path-integrity guards for tracked architecture and documentation."""

from __future__ import annotations

import json
from pathlib import Path
import re
import tomllib

_ROOT = Path(__file__).resolve().parents[2]
_MIRROR = _ROOT / ".jig/graph/mirror"
_MARKDOWN_LINK = re.compile(r"\[[^\]]*\]\(([^)]+)\)")
_RETIRED_PATHS = (
    "mods",
    "src/languages",
    "src/mods/languages",
    "src/unreal/icon",
    "tools/language-mods",
    "tools/vwj",
)


def test_workspace_members_have_cargo_manifests() -> None:
    """Reject Cargo workspace members that no longer exist."""
    with (_ROOT / "Cargo.toml").open("rb") as stream:
        config = tomllib.load(stream)
    workspace = config.get("workspace")
    assert isinstance(workspace, dict), (
        "root Cargo.toml must declare [workspace]"
    )
    members = workspace.get("members")
    assert isinstance(members, list), "workspace.members must be a list"

    missing = [
        member
        for member in members
        if not isinstance(member, str)
        or not (_ROOT / member / "Cargo.toml").is_file()
    ]
    assert not missing, f"stale Cargo workspace members: {missing}"


def _identity_path(function_file: Path) -> str | None:
    """Read identity.path from the repository's narrow function.yml grammar."""
    in_identity = False
    for line in function_file.read_text(encoding="utf-8").splitlines():
        if line == "identity:":
            in_identity = True
            continue
        if in_identity and line and not line.startswith("  "):
            break
        if in_identity and line.startswith("  path: "):
            return line.split(": ", 1)[1].strip().strip("\"'")
    return None


def test_function_metadata_paths_match_directories() -> None:
    """Keep function identity paths synchronized with their real directories."""
    mismatches: list[str] = []
    for function_file in sorted((_ROOT / "src").glob("**/function.yml")):
        declared = _identity_path(function_file)
        actual = function_file.parent.relative_to(_ROOT).as_posix()
        if declared != actual:
            mismatches.append(
                f"{function_file.relative_to(_ROOT)}: "
                f"{declared!r} != {actual!r}"
            )
    assert not mismatches, f"stale function.yml paths: {mismatches}"


def _flat_yaml_values(path: Path) -> dict[str, str]:
    """Parse flat boundary metadata without external YAML dependencies."""
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if not line or line.startswith("#") or ": " not in line:
            continue
        key, value = line.split(": ", 1)
        values[key] = value.strip().strip("\"'")
    return values


def test_source_domains_have_boundary_metadata() -> None:
    """Require canonical README authority for every src/<domain> boundary."""
    mismatches: list[str] = []
    for domain in sorted(
        path for path in (_ROOT / "src").iterdir() if path.is_dir()
    ):
        relative = domain.relative_to(_ROOT).as_posix()
        readme = domain / "README.md"
        metadata = _MIRROR / relative / "README.md.yml"
        if not readme.is_file() or not metadata.is_file():
            mismatches.append(f"{relative}: missing README boundary metadata")
            continue
        values = _flat_yaml_values(metadata)
        expected = {
            "schema": "shar-boundary/v1",
            "path": f"{relative}/README.md",
            "boundary": relative,
            "authority": "README.md",
        }
        if values != expected:
            mismatches.append(f"{relative}: metadata={values!r}")
    assert not mismatches, (
        f"noncanonical source domain boundaries: {mismatches}"
    )


def _metadata_scalar(function_file: Path, section: str, key: str) -> str | None:
    """Read one scalar from a top-level mapping in function.yml."""
    active = False
    prefix = f"  {key}: "
    for line in function_file.read_text(encoding="utf-8").splitlines():
        if line == f"{section}:":
            active = True
            continue
        if active and line and not line.startswith("  "):
            break
        if active and line.startswith(prefix):
            return line.split(": ", 1)[1].strip().strip("\"'")
    return None


def _exact_ignored_directories() -> set[str]:
    """Return exact root-relative directory ignores, excluding pattern rules."""
    ignored: set[str] = set()
    ignore_text = (_ROOT / ".gitignore").read_text(encoding="utf-8")
    ignore_lines = ignore_text.splitlines()
    for raw_line in ignore_lines:
        line = raw_line.strip()
        if not line or line.startswith(("#", "!")):
            continue
        candidate = line.removeprefix("/").removesuffix("/")
        patterns = ("*", "?", "[")
        if not candidate or any(token in candidate for token in patterns):
            continue
        ignored.add(candidate)
    return ignored


def _declared_kinds(function_file: Path) -> list[str]:
    """Read the architecture.kinds sequence from function.yml."""
    lines = function_file.read_text(encoding="utf-8").splitlines()
    kinds: list[str] = []
    in_architecture = False
    in_kinds = False
    for line in lines:
        if line == "architecture:":
            in_architecture = True
            continue
        if in_architecture and line and not line.startswith("  "):
            break
        if in_architecture and line == "  kinds:":
            in_kinds = True
            continue
        if in_kinds and line.startswith("    - "):
            kinds.append(line[6:])
            continue
        if in_kinds and not line.startswith("    "):
            break
    return kinds


def test_source_tree_uses_domain_function_boundaries() -> None:
    """Require metadata for every src/<domain>/<function> directory."""
    missing: list[str] = []
    for domain in sorted(
        path for path in (_ROOT / "src").iterdir() if path.is_dir()
    ):
        missing.extend(
            function.relative_to(_ROOT).as_posix()
            for function in sorted(
                path for path in domain.iterdir() if path.is_dir()
            )
            if not (function / "function.yml").is_file()
        )
    assert not missing, f"source functions missing function.yml: {missing}"


def test_function_metadata_matches_canonical_route_and_kinds() -> None:
    """Keep function metadata synchronized with physical boundaries."""
    mismatches: list[str] = []
    for function_file in sorted((_ROOT / "src").glob("*/*/function.yml")):
        function_root = function_file.parent
        relative = function_root.relative_to(_ROOT).as_posix()
        domain = function_root.parent.name
        declared_path = _metadata_scalar(function_file, "identity", "path")
        declared_domain = _metadata_scalar(function_file, "identity", "domain")
        route = _metadata_scalar(function_file, "architecture", "route")
        declared_kinds = sorted(_declared_kinds(function_file))
        ignored_directories = _exact_ignored_directories()
        physical_kinds = sorted(
            path.name
            for path in function_root.iterdir()
            if path.is_dir()
            and not path.name.startswith(".")
            and path.relative_to(_ROOT).as_posix() not in ignored_directories
        )
        if declared_path != relative:
            mismatches.append(f"{relative}: identity.path={declared_path!r}")
        if declared_domain != domain:
            mismatches.append(
                f"{relative}: identity.domain={declared_domain!r}"
            )
        if route != "src/<domain>/<function>/<kind>/<part>":
            mismatches.append(f"{relative}: architecture.route={route!r}")
        if declared_kinds != physical_kinds:
            mismatches.append(
                f"{relative}: kinds={declared_kinds!r} "
                f"physical={physical_kinds!r}"
            )
    assert not mismatches, (
        f"noncanonical source function metadata: {mismatches}"
    )


def _documentation_files() -> list[Path]:
    files = [_ROOT / "README.md", _ROOT / "TODO.md"]
    files.extend((_ROOT / "docs").rglob("*.md"))
    files.extend((_ROOT / "docs").rglob("*.mdc"))
    return sorted(path for path in files if path.is_file())


def test_repository_relative_markdown_links_exist() -> None:
    """Reject relative documentation links whose repository targets vanished."""
    missing: list[str] = []
    for document in _documentation_files():
        text = document.read_text(encoding="utf-8")
        for match in _MARKDOWN_LINK.finditer(text):
            target = match.group(1).strip()
            if not target or target.startswith((
                "#",
                "http:",
                "https:",
                "mailto:",
                "<http:",
                "<https:",
            )):
                continue
            target = target.strip("<>").split("#", 1)[0]
            if not target:
                continue
            destination = (document.parent / target).resolve()
            if not destination.is_relative_to(_ROOT.resolve()):
                continue
            if not destination.exists():
                line = text.count("\n", 0, match.start()) + 1
                relative = document.relative_to(_ROOT).as_posix()
                missing.append(f"{relative}:{line}: {target}")
    assert not missing, f"stale repository Markdown links: {missing}"


def test_tool_python_parts_live_below_registered_kinds() -> None:
    """Reject Python tool parts placed directly at a function root."""
    misplaced = sorted(
        path.relative_to(_ROOT).as_posix()
        for function in (_ROOT / "tools").iterdir()
        if function.is_dir()
        for path in function.glob("*.py")
    )
    assert not misplaced, (
        f"tool Python parts bypass a registered kind: {misplaced}"
    )


def test_tool_functions_publish_canonical_boundary_metadata() -> None:
    """Require canonical boundary artifacts once a tool function has a kind."""
    taxonomy = json.loads(
        (_ROOT / ".jig" / "taxonomy.json").read_text(encoding="utf-8")
    )
    architecture = taxonomy.get("architecture")
    kinds = (
        architecture.get("kinds")
        if isinstance(architecture, dict)
        else None
    )
    assert isinstance(kinds, dict), "taxonomy must declare architecture kinds"
    admitted = set(kinds)
    missing: list[str] = []
    functions = sorted(
        path for path in (_ROOT / "tools").iterdir() if path.is_dir()
    )
    for function in functions:
        physical = {
            child.name
            for child in function.iterdir()
            if child.is_dir() and child.name in admitted
        }
        if not physical:
            continue
        required = (
            function / "function.yml",
            function / "README.md",
            function / f"{function.name}.jig",
        )
        missing.extend(
            path.relative_to(_ROOT).as_posix()
            for path in required
            if not path.is_file()
        )
        relative = function.relative_to(_ROOT)
        sidecar = _MIRROR / relative / "README.md.yml"
        if not sidecar.is_file():
            missing.append(sidecar.relative_to(_ROOT).as_posix())
    assert not missing, (
        f"tool function boundary artifacts missing: {missing}"
    )


def test_build_tool_source_is_explicitly_admitted_by_gitignore() -> None:
    """Keep repository-owned build adapters trackable despite build ignores."""
    lines = (_ROOT / ".gitignore").read_text(encoding="utf-8").splitlines()
    required = (
        "!tools/build/",
        "!tools/build/adapter-inbound/",
        "!tools/build/adapter-inbound/**",
    )
    missing = tuple(rule for rule in required if rule not in lines)
    assert not missing, (
        f"build adapter source ignore exceptions missing: {missing}"
    )


def test_retired_repository_paths_stay_absent() -> None:
    """Prevent retired compatibility and replacement paths from returning."""
    present = [path for path in _RETIRED_PATHS if (_ROOT / path).exists()]
    assert not present, f"retired repository paths returned: {present}"


def test_external_test_roots_disable_source_mirroring_explicitly() -> None:
    """Keep Jig aligned with function-owned external test roots."""
    with (_ROOT / ".jig/settings/repository.toml").open("rb") as stream:
        repository = tomllib.load(stream)
    layout = repository.get("layout")
    assert isinstance(layout, dict)
    assert layout.get("test_source_mirroring") is False

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
#   - Repository-policy evidence for the standalone LMLM compatibility tool.
# - Must-Not:
#   - Read private legacy packages or execute conversion behavior.
# - Allows:
#   - Inspect tracked workspace, function, layout, and ignore metadata.
# - Split-When:
#   - LMLM tool repository policy gains independently versioned surfaces.
# - Merge-When:
#   - Another architecture test owns the same compatibility-tool contract.
# - Summary:
#   - Guards the canonical LMLM tool boundary.
# - Description:
#   - Keeps compatibility code inside admitted kinds and root validation.
# - Usage:
#   - Run through the canonical repository pytest gate.
# - Defaults:
#   - Reads only tracked repository metadata.
#

"""Repository-policy tests for the LMLM compatibility tool."""

from __future__ import annotations

from pathlib import Path
import tomllib

_ROOT = Path(__file__).resolve().parents[2]
_TOOL = _ROOT / "tools/lmlm"


def test_lmlm_tool_is_root_workspace_component() -> None:
    """Keep the compatibility crate in root validation and Jig's graph."""
    root_cargo = tomllib.loads((_ROOT / "Cargo.toml").read_text())
    assert "tools/lmlm" in root_cargo["workspace"]["members"]
    tool_cargo = tomllib.loads((_TOOL / "Cargo.toml").read_text())
    assert "workspace" not in tool_cargo
    assert not (_TOOL / "Cargo.lock").exists()

    with (_ROOT / ".jig/jig.toml").open("rb") as stream:
        config = tomllib.load(stream)
    components = config["architecture"]["component"]
    assert components["shar_lmlm"]["depends_on"] == [
        "p3d",
        "schoenwald_cli",
        "schoenwald_filesystem",
        "shar_mod_package",
        "shar_sha256",
    ]


def test_lmlm_tool_uses_registered_function_layout() -> None:
    """Keep tracked implementation below canonical architectural kinds."""
    manifest = (_TOOL / "function.yml").read_text()
    assert "path: tools/lmlm" in manifest
    assert "domain: tools" in manifest
    assert "route: tools/<function>/<kind>/<part>" in manifest
    for kind in ("composition", "contract", "domain"):
        assert f"    - {kind}" in manifest
    assert "root: tests/tools/lmlm" in manifest
    assert not (_TOOL / "src").exists()
    assert not (_TOOL / "tests").exists()
    assert (_ROOT / "tests/tools/lmlm/converter.rs").is_file()


def test_lmlm_tool_publishes_canonical_surface_metadata() -> None:
    """Keep capability, boundary, and project surfaces explicit."""
    assert (_TOOL / "lmlm.jig").read_text() == "math,rust\n"
    assert (_TOOL / "README.md.yml").read_text() == (
        "schema: shar-boundary/v1\n"
        "path: tools/lmlm/README.md\n"
        "boundary: tools/lmlm\n"
        "authority: README.md\n"
    )
    assert (_TOOL / "Cargo.toml.yml").read_text() == (
        "schema: shar-surface/v1\n"
        "path: tools/lmlm/Cargo.toml\n"
        "function: lmlm\n"
        "kind: project\n"
        "authority: Cargo.toml\n"
    )


def test_lmlm_local_io_directories_are_fully_ignored() -> None:
    """Keep private imports and generated exports out of tracked kinds."""
    lines = (_ROOT / ".gitignore").read_text().splitlines()
    assert "/tools/lmlm/import/" in lines
    assert "/tools/lmlm/export/" in lines
    assert not (_TOOL / "import/README.md").exists()
    assert not (_TOOL / "export/README.md").exists()
    assert (_TOOL / "contract/import-workflow.md").is_file()
    assert (_TOOL / "contract/export-workflow.md").is_file()

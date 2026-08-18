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
#   - Repository-policy regression evidence for the deep source-audit function.
# - Must-Not:
#   - Parse proprietary payloads or execute the validator binary.
# - Allows:
#   - Inspect tracked Jig architecture and function-surface metadata.
# - Split-When:
#   - Source-audit repository policy gains independently versioned surfaces.
# - Merge-When:
#   - Another architecture test owns the same source-audit metadata contract.
# - Summary:
#   - Guards source-audit architecture admission and facade ownership.
# - Description:
#   - Keeps the deep validator declared in Jig's component and surface graphs.
# - Usage:
#   - Run through the canonical repository pytest gate.
# - Defaults:
#   - Reads only tracked repository metadata.
#

"""Repository-policy tests for the deep source-audit function."""

from __future__ import annotations

from pathlib import Path
import tomllib

_ROOT = Path(__file__).resolve().parents[2]


def test_source_audit_is_declared_architecture_component() -> None:
    """Keep the deep validator inside Jig's dependency graph."""
    with (_ROOT / ".jig" / "jig.toml").open("rb") as stream:
        config = tomllib.load(stream)
    architecture = config.get("architecture")
    assert isinstance(architecture, dict)
    components = architecture.get("component")
    assert isinstance(components, dict)
    source_audit = components.get("shar_source_audit")
    assert isinstance(source_audit, dict)
    assert source_audit.get("depends_on") == [
        "p3d",
        "rcf",
        "rmv",
        "rsd",
        "schoenwald_cli",
        "schoenwald_filesystem",
    ]


def test_source_audit_facade_has_canonical_surface_sidecar() -> None:
    """Keep the activated Rust facade explicit in Jig's surface graph."""
    expected = (
        "schema: shar-surface/v1\n"
        "path: src/migration/source-audit/composition/lib.rs\n"
        "function: source-audit\n"
        "kind: facade\n"
        "authority: composition/lib.rs\n"
    )
    sidecar = (
        _ROOT / "src/migration/source-audit/composition/lib.rs.yml"
    )
    assert sidecar.read_text(encoding="utf-8") == expected

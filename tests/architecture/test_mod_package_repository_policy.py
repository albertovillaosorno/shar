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
#   - Repository-policy evidence for the normalized mod-package function.
# - Must-Not:
#   - Parse package payloads or execute package activation behavior.
# - Allows:
#   - Inspect tracked architecture, boundary, and facade metadata.
# - Split-When:
#   - Mod-package repository policy gains independently versioned surfaces.
# - Merge-When:
#   - Another architecture test owns the same mod-package metadata contract.
# - Summary:
#   - Guards mod-package architecture and surface admission.
# - Description:
#   - Keeps the shared package contract declared in Jig's repository graph.
# - Usage:
#   - Run through the canonical repository pytest gate.
# - Defaults:
#   - Reads only tracked repository metadata.
#

"""Repository-policy tests for the normalized mod-package function."""

from __future__ import annotations

from pathlib import Path
import tomllib

_ROOT = Path(__file__).resolve().parents[2]


def test_mod_package_is_declared_architecture_component() -> None:
    """Keep the package contract inside Jig's dependency graph."""
    with (_ROOT / ".jig" / "jig.toml").open("rb") as stream:
        config = tomllib.load(stream)
    architecture = config.get("architecture")
    assert isinstance(architecture, dict)
    components = architecture.get("component")
    assert isinstance(components, dict)
    package = components.get("shar_mod_package")
    assert isinstance(package, dict)
    assert package.get("depends_on") == [
        "schoenwald_filesystem",
        "shar_sha256",
    ]


def test_mod_package_has_canonical_boundary_sidecar() -> None:
    """Keep the package README registered as boundary authority."""
    expected = (
        "schema: shar-boundary/v1\n"
        "path: src/modding/package/README.md\n"
        "boundary: src/modding/package\n"
        "authority: README.md\n"
    )
    sidecar = _ROOT / "src/modding/package/README.md.yml"
    assert sidecar.read_text(encoding="utf-8") == expected


def test_mod_package_facade_has_canonical_surface_sidecar() -> None:
    """Keep the activated Rust facade explicit in Jig's surface graph."""
    expected = (
        "schema: shar-surface/v1\n"
        "path: src/modding/package/composition/lib.rs\n"
        "function: package\n"
        "kind: facade\n"
        "authority: composition/lib.rs\n"
    )
    sidecar = _ROOT / "src/modding/package/composition/lib.rs.yml"
    assert sidecar.read_text(encoding="utf-8") == expected


def test_mod_package_domain_owns_no_external_effect_capabilities() -> None:
    """Keep serialization, hashing, and path policy outside pure domain data."""
    domain = (
        _ROOT / "src/modding/package/domain/mod.rs"
    ).read_text(encoding="utf-8")
    forbidden = (
        "serde",
        "schoenwald_filesystem",
        "shar_sha256",
        "unicode_normalization",
        "std::path",
    )
    for fragment in forbidden:
        assert fragment not in domain

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
#   - Repository-policy evidence for official-language composition.
# - Must-Not:
#   - Read proprietary localization payloads or publish language bundles.
# - Allows:
#   - Inspect tracked architecture, function, boundary, and facade metadata.
# - Split-When:
#   - Language repository policy gains independently versioned surfaces.
# - Merge-When:
#   - Another architecture test owns the same language metadata contract.
# - Summary:
#   - Guards official-language architecture and surface admission.
# - Description:
#   - Keeps language composition declared in Jig's repository graph.
# - Usage:
#   - Run through the canonical repository pytest gate.
# - Defaults:
#   - Reads only tracked repository metadata.
#

"""Repository-policy tests for official-language composition."""

from __future__ import annotations

from pathlib import Path
import tomllib

_ROOT = Path(__file__).resolve().parents[2]
_BOUNDARY = _ROOT / "src/localization/languages"
_SIDECARS = _ROOT / ".jig/graph/mirror/src/localization/languages"


def test_languages_is_declared_architecture_component() -> None:
    """Keep official languages inside Jig's dependency graph."""
    path = _ROOT / ".jig" / "settings" / "architecture.toml"
    with path.open("rb") as stream:
        architecture = tomllib.load(stream)
    components = architecture.get("component")
    assert isinstance(components, dict)
    languages = components.get("shar_languages")
    assert isinstance(languages, dict)
    assert languages.get("depends_on") == [
        "schoenwald_filesystem",
        "shar_mod_package",
        "shar_sha256",
    ]


def test_languages_has_canonical_function_metadata() -> None:
    """Keep the function capability declaration and boundary authority."""
    assert (_BOUNDARY / "languages.jig").read_text() == "math,rust\n"
    expected = (
        "schema: shar-boundary/v1\n"
        "path: src/localization/languages/README.md\n"
        "boundary: src/localization/languages\n"
        "authority: README.md\n"
    )
    assert (_SIDECARS / "README.md.yml").read_text() == expected


def test_languages_project_has_canonical_surface_sidecar() -> None:
    """Keep the activated Cargo project explicit in Jig's surface graph."""
    expected = (
        "schema: shar-surface/v1\n"
        "path: src/localization/languages/Cargo.toml\n"
        "function: languages\n"
        "kind: project\n"
        "authority: Cargo.toml\n"
    )
    sidecar = _SIDECARS / "Cargo.toml.yml"
    assert sidecar.read_text() == expected


def test_languages_facade_has_canonical_surface_sidecar() -> None:
    """Keep the activated Rust facade explicit in Jig's surface graph."""
    expected = (
        "schema: shar-surface/v1\n"
        "path: src/localization/languages/composition/lib.rs\n"
        "function: languages\n"
        "kind: facade\n"
        "authority: composition/lib.rs\n"
    )
    sidecar = _SIDECARS / "composition/lib.rs.yml"
    assert sidecar.read_text() == expected


def test_languages_tests_live_under_repository_test_root() -> None:
    """Keep executable language tests outside the production domain tree."""
    function = (_BOUNDARY / "function.yml").read_text()
    assert "root: tests/localization/languages" in function
    assert not (_BOUNDARY / "domain/tests.rs").exists()
    assert (_ROOT / "tests/localization/languages/contract.rs").is_file()


def test_languages_domain_owns_no_effect_capabilities() -> None:
    """Keep filesystem, serialization, and package effects in composition."""
    domain = (_BOUNDARY / "domain/mod.rs").read_text()
    forbidden = (
        "std::fs",
        "std::io",
        "std::path",
        "serde",
        "schoenwald_filesystem",
        "shar_mod_package",
        "shar_sha256",
    )
    for fragment in forbidden:
        assert fragment not in domain
    assert "pub fn export_language" not in domain
    assert (_BOUNDARY / "composition/export.rs").is_file()

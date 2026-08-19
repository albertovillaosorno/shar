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

"""Repository guards for the local algorithm workspace publication boundary."""

from __future__ import annotations

import json
from pathlib import Path
import tomllib

_ROOT = Path(__file__).resolve().parents[2]


def _algorithm_ignore_rules() -> tuple[str, ...]:
    """Return authored algorithm-workspace ignore rules in source order."""
    lines = (_ROOT / ".gitignore").read_text(encoding="utf-8").splitlines()
    prefixes = ("/algorithms", "!/algorithms")
    return tuple(line for line in lines if line.startswith(prefixes))


def test_algorithm_workspace_is_default_deny_with_public_exceptions() -> None:
    """Keep payloads private while admitting public metadata and plans."""
    assert _algorithm_ignore_rules() == (
        "/algorithms/**",
        "!/algorithms/**/",
        "!/algorithms/**/README.md",
        "!/algorithms/**/algorithm/*.txt",
    )


def test_algorithm_workspace_tracks_semantic_directory_anchors() -> None:
    """Require README anchors for semantic directories that may be empty."""
    required = {
        "algorithms/game/README.md",
        "algorithms/game/in/README.md",
        "algorithms/game/master/README.md",
        "algorithms/muckluck/README.md",
        "algorithms/muckluck/in/README.md",
        "algorithms/muckluck/master/README.md",
        "algorithms/muckluck/algorithm/README.md",
        "algorithms/out/README.md",
        "algorithms/out/mods/README.md",
    }
    for locale in ("french", "german", "spanish-latam", "spanish-spain"):
        required.update(
            {
                f"algorithms/lang/{locale}/README.md",
                f"algorithms/lang/{locale}/in/README.md",
                f"algorithms/lang/{locale}/master/README.md",
                f"algorithms/lang/{locale}/algorithm/README.md",
            }
        )

    missing = sorted(path for path in required if not (_ROOT / path).is_file())
    assert not missing, (
        f"algorithm workspace is missing README anchors: {missing}"
    )


def test_algorithm_workspace_has_one_taxonomy_role() -> None:
    """Keep the public workspace inside Jig's closed path taxonomy."""
    with (_ROOT / ".jig" / "jig.toml").open("rb") as stream:
        config = tomllib.load(stream)
    taxonomy = config.get("taxonomy")
    assert isinstance(taxonomy, dict)
    roles = taxonomy.get("role")
    assert isinstance(roles, dict)
    algorithms = roles.get("algorithms")
    assert algorithms == {"paths": ["algorithms/"]}


def test_public_algorithm_docs_use_family_plan_layout() -> None:
    """Reject the retired build/hash pair from current public authorities."""
    authorities = (
        "README.md",
        (
            "docs/technical/security/"
            "public-safe-reconstruction-gate.md"
        ),
        (
            "docs/todo/open/security/"
            "define-a-public-safe-reconstruction-algorithm-gate-with-a-"
            "bounded-source-similarity-window.mdc"
        ),
        (
            "docs/todo/open/packaging/"
            "build-the-lightweight-src-user-exporter-and-cross-platform-gui.mdc"
        ),
    )
    legacy = (
        "algorithms/<arch>/<os>/build.txt",
        "algorithms/<arch>/<os>/hash.txt",
        "{build.txt,hash.txt}",
        "`build.txt`",
        "`hash.txt`",
    )
    offenders: list[str] = []
    for relative in authorities:
        content = (_ROOT / relative).read_text(encoding="utf-8")
        if any(token in content for token in legacy):
            offenders.append(relative)
    assert not offenders, f"legacy algorithm layout remains in: {offenders}"


def test_public_algorithm_docs_name_source_bound_plan_schema() -> None:
    """Keep current docs tied to the implemented generic plan contract."""
    for relative in (
        "README.md",
        "docs/technical/security/public-safe-reconstruction-gate.md",
    ):
        content = (_ROOT / relative).read_text(encoding="utf-8")
        assert "shar.algorithm.v1" in content
        assert "algorithm/*.txt" in content


def test_algorithm_crate_is_declared_architecture_component() -> None:
    """Keep the generic algorithm crate inside Jig's dependency graph."""
    with (_ROOT / ".jig" / "jig.toml").open("rb") as stream:
        config = tomllib.load(stream)
    architecture = config.get("architecture")
    assert isinstance(architecture, dict)
    components = architecture.get("component")
    assert isinstance(components, dict)
    algorithm = components.get("shar_algorithm")
    assert isinstance(algorithm, dict)
    assert algorithm.get("depends_on") == [
        "schoenwald_cli",
        "schoenwald_filesystem",
        "shar_sha256",
    ]


def test_algorithm_surfaces_and_settings_follow_registered_layout() -> None:
    """Keep algorithm metadata and settings on declared surfaces."""
    expected = {
        "README.md.yml": (
            "schema: shar-boundary/v1\n"
            "path: src/foundation/algorithm/README.md\n"
            "boundary: src/foundation/algorithm\n"
            "authority: README.md\n"
        ),
        "composition/lib.rs.yml": (
            "schema: shar-surface/v1\n"
            "path: src/foundation/algorithm/composition/lib.rs\n"
            "function: algorithm\n"
            "kind: facade\n"
            "authority: composition/lib.rs\n"
        ),
    }
    boundary = _ROOT / "src" / "foundation" / "algorithm"
    for relative, content in expected.items():
        assert (boundary / relative).read_text() == content
    settings = boundary / "composition" / "adapter-inbound" / "settings.json"
    assert settings.is_file()
    assert not (boundary / "settings.json").exists()
    cli = (boundary / "composition" / "adapter-inbound" / "cli.rs").read_text()
    default_settings = (
        "src/foundation/algorithm/composition/adapter-inbound/settings.json"
    )
    assert default_settings in cli


def test_algorithm_domain_is_serialization_free() -> None:
    """Keep JSON wire ownership in composition rather than domain."""
    boundary = _ROOT / "src" / "foundation" / "algorithm"
    domain = (boundary / "domain" / "model.rs").read_text()
    assert "serde" not in domain
    document = boundary / "composition" / "document.rs"
    assert document.is_file()
    wire = document.read_text()
    assert "serde::{Deserialize, Serialize}" in wire
    assert "serde_json" in wire


_HEX = frozenset("0123456789abcdef")


def _is_lower_hex(value: object, length: int) -> bool:
    """Return whether one JSON value is exact lowercase hexadecimal."""
    return (
        isinstance(value, str)
        and len(value) == length
        and set(value) <= _HEX
    )


def _assert_source_bound_plan(text: str) -> None:
    """Require the publishable structural subset of shar.algorithm.v1."""
    document = json.loads(text)
    assert isinstance(document, dict)
    assert set(document) == {
        "schema",
        "settings_sha256",
        "source",
        "target_kind",
        "target",
    }
    assert document["schema"] == "shar.algorithm.v1"
    assert _is_lower_hex(document["settings_sha256"], 64)
    assert document["target_kind"] in {"file", "directory"}

    source = document["source"]
    assert isinstance(source, list)
    assert source
    for record in source:
        assert isinstance(record, dict)
        assert set(record) == {"input", "path", "bytes", "sha256"}
        assert isinstance(record["input"], int)
        assert record["input"] >= 0
        assert isinstance(record["path"], str)
        assert isinstance(record["bytes"], int)
        assert record["bytes"] >= 0
        assert _is_lower_hex(record["sha256"], 64)

    target = document["target"]
    assert isinstance(target, list)
    assert target
    for record in target:
        assert isinstance(record, dict)
        assert set(record) == {
            "path",
            "bytes",
            "sha256",
            "nonce",
            "ciphertext",
        }
        assert isinstance(record["path"], str)
        assert isinstance(record["bytes"], int)
        assert record["bytes"] >= 0
        assert _is_lower_hex(record["sha256"], 64)
        assert _is_lower_hex(record["nonce"], 24)
        ciphertext = record["ciphertext"]
        assert isinstance(ciphertext, str)
        assert ciphertext
        assert len(ciphertext) % 2 == 0
        assert set(ciphertext) <= _HEX


def test_substantive_algorithm_example_matches_source_bound_contract() -> None:
    """Exercise the publication guard against a real authored plan."""
    example = _ROOT / "src/migration/icon/icon_algorithm.txt"
    _assert_source_bound_plan(example.read_text(encoding="utf-8"))


def test_public_family_plan_publication_contract() -> None:
    """Reject arbitrary text from every publishable family plan path."""
    plans = sorted((_ROOT / "algorithms").glob("**/algorithm/*.txt"))
    assert plans
    for plan in plans:
        text = plan.read_text(encoding="utf-8")
        if not text.strip():
            continue
        _assert_source_bound_plan(text)

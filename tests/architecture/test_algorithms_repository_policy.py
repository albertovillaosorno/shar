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

import hashlib
import json
from pathlib import Path
import tomllib

import pytest

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
_U64_MAX = (1 << 64) - 1


def _json_object_without_duplicates(
    pairs: list[tuple[str, object]],
) -> dict[str, object]:
    """Build one JSON object while rejecting duplicate member names."""
    result: dict[str, object] = {}
    for key, value in pairs:
        assert key not in result, f"algorithm JSON repeats key: {key}"
        result[key] = value
    return result


def _is_nonnegative_integer(value: object) -> bool:
    """Match serde u64 admission rather than Python's bool-as-int rule."""
    return (
        isinstance(value, int)
        and not isinstance(value, bool)
        and 0 <= value <= _U64_MAX
    )


def _active_settings() -> dict[str, object]:
    """Load active generic algorithm settings with duplicate-key rejection."""
    path = (
        _ROOT
        / "src/foundation/algorithm/composition/adapter-inbound/settings.json"
    )
    document = json.loads(
        path.read_text(encoding="utf-8"),
        object_pairs_hook=_json_object_without_duplicates,
    )
    assert isinstance(document, dict)
    return document


def _active_settings_sha256() -> str:
    """Hash the active algorithm settings using the Rust compact JSON shape."""
    encoded = json.dumps(_active_settings(), separators=(",", ":")).encode()
    return hashlib.sha256(encoded).hexdigest()


def _is_lower_hex(value: object, length: int) -> bool:
    """Return whether one JSON value is exact lowercase hexadecimal."""
    return (
        isinstance(value, str)
        and len(value) == length
        and set(value) <= _HEX
    )


def _read_public_plan(path: Path) -> str:
    """Read one publishable plan only from a real repository file."""
    assert path.is_file(), f"public algorithm plan is not a file: {path}"
    assert not path.is_symlink(), f"public algorithm plan is linked: {path}"
    return path.read_text(encoding="utf-8")


def _assert_relative_record_path(
    path: str,
    *,
    allow_empty: bool,
) -> tuple[str, ...]:
    """Require one canonical forward-slash relative record path."""
    if not path:
        assert allow_empty
        return ()
    assert not path.startswith(("/", "\\"))
    assert "\\" not in path
    assert ":" not in path
    parts = tuple(path.split("/"))
    assert all(part not in {"", ".", ".."} for part in parts)
    return parts


def _assert_target_layout(document: dict[str, object]) -> None:
    """Mirror replay's file/directory target identity constraints."""
    target_kind = document["target_kind"]
    target = document["target"]
    assert isinstance(target, list)
    if target_kind == "file":
        assert len(target) == 1
        record = target[0]
        assert isinstance(record, dict)
        path = record.get("path")
        assert isinstance(path, str)
        assert not path
        return

    identities: list[tuple[str, ...]] = []
    for record in target:
        assert isinstance(record, dict)
        path = record.get("path")
        assert isinstance(path, str)
        parts = _assert_relative_record_path(path, allow_empty=False)
        identity = tuple(part.upper() for part in parts)
        assert not any(
            identity[: len(existing)] == existing
            or existing[: len(identity)] == identity
            for existing in identities
        )
        identities.append(identity)


def _assert_source_layout(source: list[object]) -> None:
    """Mirror the source collector's stable input grouping and identities."""
    identities: set[tuple[int, str]] = set()
    root_is_file: dict[int, bool] = {}
    previous_input = -1
    for record in source:
        assert isinstance(record, dict)
        input_index = record["input"]
        path = record["path"]
        assert isinstance(input_index, int)
        assert isinstance(path, str)
        assert input_index >= previous_input
        previous_input = input_index
        identity = (input_index, path)
        assert identity not in identities
        identities.add(identity)
        is_file = not path
        if input_index in root_is_file:
            assert root_is_file[input_index] == is_file
        else:
            root_is_file[input_index] = is_file


def _assert_plan_resource_limits(document: dict[str, object]) -> None:
    """Mirror replay's active integer, file-count, and byte-count limits."""
    settings = _active_settings()
    names = (
        "minimum_source_files",
        "maximum_source_files",
        "minimum_source_bytes",
        "maximum_source_bytes",
        "maximum_target_files",
        "maximum_target_bytes",
        "maximum_file_bytes",
    )
    limits: dict[str, int] = {}
    for name in names:
        value = settings[name]
        assert _is_nonnegative_integer(value)
        assert isinstance(value, int)
        limits[name] = value

    source = document["source"]
    target = document["target"]
    assert isinstance(source, list)
    assert isinstance(target, list)
    assert limits["minimum_source_files"] <= len(source)
    assert len(source) <= limits["maximum_source_files"]
    assert len(target) <= limits["maximum_target_files"]

    source_bytes = 0
    for record in source:
        assert isinstance(record, dict)
        byte_count = record["bytes"]
        assert isinstance(byte_count, int)
        assert byte_count <= limits["maximum_file_bytes"]
        source_bytes += byte_count
    assert limits["minimum_source_bytes"] <= source_bytes
    assert source_bytes <= limits["maximum_source_bytes"]

    target_bytes = 0
    for record in target:
        assert isinstance(record, dict)
        byte_count = record["bytes"]
        assert isinstance(byte_count, int)
        assert byte_count <= limits["maximum_file_bytes"]
        target_bytes += byte_count
    assert target_bytes <= limits["maximum_target_bytes"]


def _assert_source_bound_plan(text: str) -> None:
    """Require the publishable structural subset of shar.algorithm.v1."""
    document = json.loads(
        text,
        object_pairs_hook=_json_object_without_duplicates,
    )
    assert isinstance(document, dict)
    assert set(document) == {
        "schema",
        "settings_sha256",
        "source",
        "target_kind",
        "target",
    }
    assert document["schema"] == "shar.algorithm.v1"
    assert document["settings_sha256"] == _active_settings_sha256()
    assert document["target_kind"] in {"file", "directory"}

    source = document["source"]
    assert isinstance(source, list)
    assert source
    for record in source:
        assert isinstance(record, dict)
        assert set(record) == {"input", "path", "bytes", "sha256"}
        assert _is_nonnegative_integer(record["input"])
        assert isinstance(record["path"], str)
        _assert_relative_record_path(record["path"], allow_empty=True)
        assert _is_nonnegative_integer(record["bytes"])
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
        assert _is_nonnegative_integer(record["bytes"])
        assert _is_lower_hex(record["sha256"], 64)
        assert _is_lower_hex(record["nonce"], 24)
        ciphertext = record["ciphertext"]
        assert isinstance(ciphertext, str)
        assert len(ciphertext) == 2 * (record["bytes"] + 16)
        assert set(ciphertext) <= _HEX
    _assert_source_layout(source)
    _assert_plan_resource_limits(document)
    _assert_target_layout(document)


def _synthetic_plan(*, path: str = "asset.bin") -> dict[str, object]:
    """Return one minimal structurally valid directory plan fixture."""
    return {
        "schema": "shar.algorithm.v1",
        "settings_sha256": _active_settings_sha256(),
        "source": [
            {"input": 0, "path": "", "bytes": 1024, "sha256": "0" * 64}
        ],
        "target_kind": "directory",
        "target": [
            {
                "path": path,
                "bytes": 1,
                "sha256": "0" * 64,
                "nonce": "0" * 24,
                "ciphertext": "00" * 17,
            }
        ],
    }


def _synthetic_source_rows(
    *updates: dict[str, object],
) -> dict[str, object]:
    """Return a synthetic plan with explicit mutations for source rows."""
    plan = _synthetic_plan()
    source = plan["source"]
    assert isinstance(source, list)
    base = source[0]
    assert isinstance(base, dict)
    records: list[dict[str, object]] = []
    for change in updates:
        record = dict(base)
        record.update(change)
        records.append(record)
    plan["source"] = records
    return plan


def _synthetic_source_plan(**updates: object) -> dict[str, object]:
    """Return a synthetic plan with mutations on its only source record."""
    plan = _synthetic_plan()
    source = plan["source"]
    assert isinstance(source, list)
    record = source[0]
    assert isinstance(record, dict)
    record.update(updates)
    return plan


def test_public_plan_guard_matches_runtime_rejections() -> None:
    """Reject JSON/path/settings shapes that generic replay will reject."""
    invalid: list[str] = []
    wrong_settings = _synthetic_plan()
    wrong_settings["settings_sha256"] = "0" * 64
    invalid.append(json.dumps(wrong_settings))

    invalid.extend(
        (
            json.dumps(_synthetic_source_plan(bytes=True)),
            json.dumps(_synthetic_plan(path="../escape.bin")),
        )
    )

    short_ciphertext = _synthetic_plan()
    short_targets = short_ciphertext["target"]
    assert isinstance(short_targets, list)
    assert isinstance(short_targets[0], dict)
    short_targets[0]["ciphertext"] = "00"
    invalid.append(json.dumps(short_ciphertext))

    invalid.extend(
        (
            json.dumps(_synthetic_source_plan(bytes=1)),
            json.dumps(_synthetic_source_plan(input=1 << 64)),
            json.dumps(_synthetic_source_plan(path="../private.bin")),
        )
    )

    invalid.extend(
        (
            json.dumps(_synthetic_source_rows({}, {})),
            json.dumps(
                _synthetic_source_rows(
                    {"input": 1},
                    {"input": 0, "path": "asset.bin"},
                )
            ),
            json.dumps(
                _synthetic_source_rows({}, {"path": "asset.bin"})
            ),
        )
    )

    overlap = _synthetic_plan(path="Folder")
    target = overlap["target"]
    assert isinstance(target, list)
    child = dict(target[0])
    child["path"] = "folder/child.bin"
    target.append(child)
    invalid.append(json.dumps(overlap))

    duplicate_key = json.dumps(_synthetic_plan())
    duplicate_key = duplicate_key.replace(
        '{"schema": "shar.algorithm.v1",',
        '{"schema": "shar.algorithm.v1", "schema": "shar.algorithm.v1",',
        1,
    )
    invalid.append(duplicate_key)

    for text in invalid:
        with pytest.raises(AssertionError):
            _assert_source_bound_plan(text)


def test_substantive_algorithm_example_matches_source_bound_contract() -> None:
    """Exercise the publication guard against a real authored plan."""
    example = _ROOT / "src/migration/icon/icon_algorithm.txt"
    _assert_source_bound_plan(_read_public_plan(example))


def test_public_family_plan_publication_contract() -> None:
    """Reject arbitrary text from every publishable family plan path."""
    plans = sorted((_ROOT / "algorithms").glob("**/algorithm/*.txt"))
    assert plans
    for plan in plans:
        text = _read_public_plan(plan)
        if not text.strip():
            continue
        _assert_source_bound_plan(text)

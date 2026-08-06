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
#   - Canonical synthetic Unreal plan bundle fixtures.
# - Must-Not:
#   - Read generated staging or contact Unreal Editor.
# - Allows:
#   - Reproduce canonical plan hashing for deterministic tests.
# - Split-When:
#   - Split when fixture families gain independent lifecycles.
# - Merge-When:
#   - Merge when plan preflight tests own all fixture construction.
# - Summary:
#   - Synthetic Unreal plan bundle fixture.
# - Description:
#   - Builds tracked one-operation and empty-plan bundle evidence.
# - Usage:
#   - Imported by editor-control plan preflight tests.
# - Defaults:
#   - Targets Unreal Engine 5.8.1 with exact canonical JSON.
#

"""Canonical synthetic Unreal plan bundle fixture."""

from __future__ import annotations

import hashlib
import json
from collections import OrderedDict
from pathlib import Path
from mcp.domain.json_types import JsonValue

_CONTEXT = OrderedDict(
    (
        ("source_manifest_revision", "a" * 64),
        ("engine_contract_revision", "shar-unreal-porting-contract-v1"),
        ("target_engine_version", "5.8.1"),
        ("target_platform", "editor"),
    )
)
_PLAN_SPECS: tuple[tuple[str, str, tuple[str, ...]], ...] = (
    ("asset-import-plan", "asset-import-plan.json", ()),
    (
        "asset-construction-plan",
        "asset-construction-plan.json",
        ("asset-import-plan",),
    ),
    (
        "world-assembly-plan",
        "world-assembly-plan.json",
        ("asset-construction-plan", "asset-import-plan"),
    ),
    (
        "runtime-binding-plan",
        "runtime-binding-plan.json",
        (
            "asset-construction-plan",
            "asset-import-plan",
            "world-assembly-plan",
        ),
    ),
    (
        "validation-plan",
        "validation-plan.json",
        (
            "asset-construction-plan",
            "asset-import-plan",
            "runtime-binding-plan",
            "world-assembly-plan",
        ),
    ),
    ("package-plan", "package-plan.json", ("validation-plan",)),
)
_BASE_REQUIREMENTS = {
    "case-insensitive-destinations-unique",
    "generated-root-confined",
    "revision-matches-canonical-body",
    "schema-supported",
}
_FAMILY_REQUIREMENTS: dict[str, set[str]] = {
    "asset-import-plan": {
        "import-settings-match-profile",
        "saved-class-matches-plan",
        "source-bytes-match-revision",
    },
    "asset-construction-plan": {
        "native-factory-available",
        "normalized-json-schema-valid",
        "saved-class-matches-plan",
    },
    "world-assembly-plan": {
        "authored-transforms-preserved",
        "streaming-ownership-valid",
    },
    "runtime-binding-plan": {"all-stable-references-resolve"},
    "validation-plan": {
        "dependencies-resolve-after-restart",
        "semantic-digest-reproducible",
    },
    "package-plan": {
        "cook-succeeds",
        "packaged-build-loads-generated-assets",
    },
}


def build_plan_bundle(*, with_import_operation: bool = False) -> dict[str, str]:
    """Return one canonical six-plan bundle keyed by filename."""
    revisions: dict[str, str] = {}
    files: dict[str, str] = {}
    entries: list[JsonValue] = []
    for plan_id, filename, dependency_ids in _PLAN_SPECS:
        operations = (
            [_wav_operation()]
            if with_import_operation and plan_id == "asset-import-plan"
            else []
        )
        dependencies = [
            OrderedDict(
                (
                    ("plan_id", dependency_id),
                    ("revision", revisions[dependency_id]),
                )
            )
            for dependency_id in dependency_ids
        ]
        body = _plan_body(
            plan_id=plan_id,
            revision="",
            dependencies=dependencies,
            operations=operations,
        )
        revision = _digest(_canonical(body))
        revisions[plan_id] = revision
        rendered = _plan_body(
            plan_id=plan_id,
            revision=revision,
            dependencies=dependencies,
            operations=operations,
        )
        files[filename] = f"{_canonical(rendered)}\n"
        entries.append(
            OrderedDict(
                (
                    ("plan_id", plan_id),
                    ("revision", revision),
                    ("filename", filename),
                    ("operation_count", len(operations)),
                )
            )
        )
    index_body = _index_body(revision="", plans=entries)
    index_revision = _digest(_canonical(index_body))
    files["index.json"] = (
        f"{_canonical(_index_body(revision=index_revision, plans=entries))}\n"
    )
    return files


def write_plan_bundle(
    root: Path,
    *,
    with_import_operation: bool = False,
) -> dict[str, str]:
    """Write one canonical synthetic bundle and return its texts."""
    files = build_plan_bundle(with_import_operation=with_import_operation)
    root.mkdir(parents=True)
    for filename, text in files.items():
        _ = (root / filename).write_text(text, encoding="utf-8", newline="\n")
    return files


def _plan_body(
    *,
    plan_id: str,
    revision: str,
    dependencies: list[JsonValue],
    operations: list[JsonValue],
) -> OrderedDict[str, JsonValue]:
    requirements = sorted(_BASE_REQUIREMENTS | _FAMILY_REQUIREMENTS[plan_id])
    return OrderedDict(
        (
            ("schema", "shar-schoenwald.unreal-plan.v1"),
            ("plan_id", plan_id),
            ("revision", revision),
            *_CONTEXT.items(),
            ("dependencies", dependencies),
            (
                "outputs",
                [operation["destination"] for operation in operations],
            ),
            ("operations", operations),
            (
                "validation",
                OrderedDict(
                    (
                        ("operation_count", len(operations)),
                        ("requirements", requirements),
                    )
                ),
            ),
        )
    )


def _wav_operation() -> OrderedDict[str, JsonValue]:
    fields: OrderedDict[str, JsonValue] = OrderedDict(
        (
            ("operation_id", ""),
            ("package_identity", "dialog-package"),
            ("source_identity", "audio-source"),
            ("source_format", "wav"),
            ("target_family", "audio"),
            ("source_path", "extracted/dialog/audio.wav"),
            ("source_revision", "b" * 64),
            (
                "destination",
                "/Game/Generated/SHAR/dialog/dialog/audio_source.audio_source",
            ),
            ("target_class", "SoundWave"),
            ("importer", "sound-wave-factory"),
            ("import_profile", "shar-audio-v1"),
            ("dependencies", []),
            ("readiness", "ready"),
            ("world_owned", False),
            ("runtime_bound", True),
        )
    )
    preimage = "\n".join(
        str(fields[field])
        for field in (
            "package_identity",
            "source_identity",
            "source_format",
            "target_family",
            "source_path",
            "source_revision",
            "destination",
            "target_class",
            "importer",
            "import_profile",
        )
    )
    fields["operation_id"] = f"operation-{_digest(preimage)[:16]}"
    return fields


def _index_body(
    *,
    revision: str,
    plans: list[JsonValue],
) -> OrderedDict[str, JsonValue]:
    return OrderedDict(
        (
            ("schema", "shar-schoenwald.unreal-plan-bundle.v1"),
            ("revision", revision),
            *_CONTEXT.items(),
            ("plans", plans),
        )
    )


def _canonical(value: JsonValue) -> str:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def _digest(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()

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

# ruff: noqa: PLR0913
from collections import OrderedDict
import hashlib
import json
from pathlib import Path

from mcp.domain.json_types import JsonValue

_CONTEXT = OrderedDict((
    ("source_manifest_revision", "a" * 64),
    ("engine_contract_revision", "shar-unreal-porting-contract-v1"),
    ("target_engine_version", "5.8.1"),
    ("target_platform", "editor"),
))
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


def build_plan_bundle(
    *,
    with_import_operation: bool = False,
    import_source_revision: str | None = None,
    with_texture_operation: bool = False,
    texture_source_revision: str | None = None,
    with_static_mesh_operation: bool = False,
    static_mesh_source_revision: str | None = None,
    with_skeletal_mesh_operation: bool = False,
    skeletal_mesh_source_revision: str | None = None,
    with_media_operation: bool = False,
    media_source_revision: str | None = None,
    with_construction_operation: bool = False,
    construction_source_path: str = "manifest.jsonl",
    construction_source_revision: str | None = None,
    semantic_blocker_count: int = 0,
) -> dict[str, str]:
    """Return one canonical six-plan bundle keyed by filename."""
    revisions: dict[str, str] = {}
    files: dict[str, str] = {}
    entries: list[JsonValue] = []
    for plan_id, filename, dependency_ids in _PLAN_SPECS:
        operations: list[JsonValue] = []
        if with_import_operation and plan_id == "asset-import-plan":
            operations.append(
                _wav_operation(import_source_revision or "b" * 64)
            )
        if with_texture_operation and plan_id == "asset-import-plan":
            operations.append(
                _texture_operation(texture_source_revision or "c" * 64)
            )
        if with_static_mesh_operation and plan_id == "asset-import-plan":
            operations.append(
                _static_mesh_operation(static_mesh_source_revision or "e" * 64)
            )
        if with_skeletal_mesh_operation and plan_id == "asset-import-plan":
            operations.append(
                _skeletal_mesh_operation(
                    skeletal_mesh_source_revision or "f" * 64
                )
            )
        if with_media_operation and plan_id == "asset-import-plan":
            operations.append(
                _media_operation(media_source_revision or "d" * 64)
            )
        if with_construction_operation and plan_id == "asset-construction-plan":
            operations.append(
                _json_operation(
                    construction_source_path,
                    construction_source_revision
                    or str(_CONTEXT["source_manifest_revision"]),
                )
            )
        dependencies = [
            OrderedDict((
                ("plan_id", dependency_id),
                ("revision", revisions[dependency_id]),
            ))
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
            OrderedDict((
                ("plan_id", plan_id),
                ("revision", revision),
                ("filename", filename),
                ("operation_count", len(operations)),
            ))
        )
    index_body = _index_body(
        revision="",
        semantic_blocker_count=semantic_blocker_count,
        plans=entries,
    )
    index_revision = _digest(_canonical(index_body))
    rendered_index = _index_body(
        revision=index_revision,
        semantic_blocker_count=semantic_blocker_count,
        plans=entries,
    )
    files["index.json"] = _canonical(rendered_index) + chr(10)
    return files


def write_plan_bundle(
    root: Path,
    *,
    with_import_operation: bool = False,
    import_source_revision: str | None = None,
    with_texture_operation: bool = False,
    texture_source_revision: str | None = None,
    with_static_mesh_operation: bool = False,
    static_mesh_source_revision: str | None = None,
    with_skeletal_mesh_operation: bool = False,
    skeletal_mesh_source_revision: str | None = None,
    with_media_operation: bool = False,
    media_source_revision: str | None = None,
    with_construction_operation: bool = False,
    semantic_blocker_count: int = 0,
) -> dict[str, str]:
    """Write one canonical synthetic bundle and return its texts."""
    files = build_plan_bundle(
        with_import_operation=with_import_operation,
        import_source_revision=import_source_revision,
        with_texture_operation=with_texture_operation,
        texture_source_revision=texture_source_revision,
        with_static_mesh_operation=with_static_mesh_operation,
        static_mesh_source_revision=static_mesh_source_revision,
        with_skeletal_mesh_operation=with_skeletal_mesh_operation,
        skeletal_mesh_source_revision=skeletal_mesh_source_revision,
        with_media_operation=with_media_operation,
        media_source_revision=media_source_revision,
        with_construction_operation=with_construction_operation,
        semantic_blocker_count=semantic_blocker_count,
    )
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
    return OrderedDict((
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
            OrderedDict((
                ("operation_count", len(operations)),
                ("requirements", requirements),
            )),
        ),
    ))


def _wav_operation(source_revision: str) -> OrderedDict[str, JsonValue]:
    fields: OrderedDict[str, JsonValue] = OrderedDict((
        ("operation_id", ""),
        ("package_identity", "dialog-package"),
        ("source_identity", "audio-source"),
        ("source_format", "wav"),
        ("target_family", "audio"),
        ("source_path", "extracted/dialog/audio.wav"),
        ("source_revision", source_revision),
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
    ))
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


def _texture_operation(
    source_revision: str,
) -> OrderedDict[str, JsonValue]:
    fields: OrderedDict[str, JsonValue] = OrderedDict((
        ("operation_id", ""),
        ("package_identity", "texture-package"),
        ("source_identity", "texture-source"),
        ("source_format", "image"),
        ("target_family", "texture"),
        ("source_path", "extracted/texture/image.png"),
        ("source_revision", source_revision),
        (
            "destination",
            "/Game/Generated/SHAR/test/texture_image.texture_image",
        ),
        ("target_class", "Texture2D"),
        ("importer", "texture-factory"),
        ("import_profile", "shar-texture-v1"),
        ("dependencies", []),
        ("readiness", "ready"),
        ("world_owned", False),
        ("runtime_bound", True),
    ))
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


def _static_mesh_operation(
    source_revision: str,
) -> OrderedDict[str, JsonValue]:
    fields: OrderedDict[str, JsonValue] = OrderedDict((
        ("operation_id", ""),
        ("package_identity", "static-mesh-package"),
        ("source_identity", "static-mesh-source"),
        ("source_format", "fbx"),
        ("target_family", "model"),
        ("source_path", "fbx-assets/static/model.fbx"),
        ("source_revision", source_revision),
        (
            "destination",
            "/Game/Generated/SHAR/models/static/model.model",
        ),
        ("target_class", "StaticMesh"),
        ("importer", "asset-tools-fbx"),
        ("import_profile", "shar-fbx-static-v1"),
        ("dependencies", []),
        ("readiness", "ready"),
        ("world_owned", False),
        ("runtime_bound", True),
    ))
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


def _skeletal_mesh_operation(
    source_revision: str,
) -> OrderedDict[str, JsonValue]:
    fields: OrderedDict[str, JsonValue] = OrderedDict((
        ("operation_id", ""),
        ("package_identity", "skeletal-mesh-package"),
        ("source_identity", "skeletal-mesh-source"),
        ("source_format", "fbx"),
        ("target_family", "model"),
        ("source_path", "fbx-assets/skeletal/model.fbx"),
        ("source_revision", source_revision),
        (
            "destination",
            "/Game/Generated/SHAR/models/skeletal/model.model",
        ),
        ("target_class", "SkeletalMesh"),
        ("importer", "asset-tools-fbx"),
        ("import_profile", "shar-fbx-skeletal-v1"),
        ("dependencies", []),
        ("readiness", "ready"),
        ("world_owned", False),
        ("runtime_bound", True),
    ))
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


def _media_operation(source_revision: str) -> OrderedDict[str, JsonValue]:
    fields: OrderedDict[str, JsonValue] = OrderedDict((
        ("operation_id", ""),
        ("package_identity", "intro-movie-package"),
        ("source_identity", "intro-movie-source"),
        ("source_format", "hap"),
        ("target_family", "media"),
        ("source_path", "extracted/movies/intro/movie.mov"),
        ("source_revision", source_revision),
        (
            "destination",
            "/Game/Generated/SHAR/movies/intro/intro_movie.intro_movie",
        ),
        ("target_class", "FileMediaSource"),
        ("importer", "media-source-movie"),
        ("import_profile", "shar-hap-movie-v1"),
        ("dependencies", []),
        ("readiness", "ready"),
        ("world_owned", False),
        ("runtime_bound", True),
    ))
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


def _json_operation(
    source_path: str,
    source_revision: str,
) -> OrderedDict[str, JsonValue]:
    fields: OrderedDict[str, JsonValue] = OrderedDict((
        ("operation_id", ""),
        ("package_identity", "structured-package"),
        ("source_identity", "structured-package-normalized-json"),
        ("source_format", "json"),
        ("target_family", "structured-data"),
        ("source_path", source_path),
        ("source_revision", source_revision),
        (
            "destination",
            "/Game/Generated/SHAR/data/structured_package.structured_package",
        ),
        ("target_class", "WidgetBlueprint"),
        ("importer", "shar-ui-factory"),
        ("import_profile", "shar-ui-v1"),
        ("dependencies", []),
        ("readiness", "requires-editor-factory"),
        ("world_owned", False),
        ("runtime_bound", True),
    ))
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
    semantic_blocker_count: int,
    plans: list[JsonValue],
) -> OrderedDict[str, JsonValue]:
    return OrderedDict((
        ("schema", "shar-schoenwald.unreal-plan-bundle.v2"),
        ("revision", revision),
        *_CONTEXT.items(),
        ("semantic_blocker_count", semantic_blocker_count),
        ("plans", plans),
    ))


def _canonical(value: JsonValue) -> str:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def _digest(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()

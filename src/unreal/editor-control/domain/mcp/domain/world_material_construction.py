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
#   - Typed compilation of verified world-material construction evidence.
# - Must-Not:
#   - Read files, contact Unreal Editor, assign mesh slots, or claim full
#   - fidelity.
# - Allows:
#   - Compile canonical Texture2D, Material, and MaterialInstanceConstant calls.
# - Split-When:
#   - Slot assignment or unsupported shader families gain a native lifecycle.
# - Merge-When:
#   - Another domain module owns identical world-material construction policy.
# - Summary:
#   - World-material native construction compiler.
# - Description:
#   - Preserves generated identities and fail-closed dependency ordering.
# - Usage:
#   - Called after release-index binding and before source or live-tool checks.
# - Defaults:
#   - Only the reviewed simple-unlit representable subset compiles.
#

"""Typed compilation of verified world-material construction evidence."""

from __future__ import annotations

import math
import struct
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import JsonValue
from mcp.domain.json_types import require_json_object

_SCHEMA = "shar-schoenwald.unreal-world-material-evidence.v3"
_TEXTURE_ROOT = "/Game/Generated/SHAR/Textures/World"
_MASTER_ROOT = "/Game/Generated/SHAR/Materials/World/Masters"
_INSTANCE_ROOT = "/Game/Generated/SHAR/Materials/World/Instances"
_TEXTURE_TOOLSET = "editor_toolset.toolsets.texture.TextureTools"
_TEXTURE_TOOL = f"{_TEXTURE_TOOLSET}.import_file"
_MATERIAL_TOOLSET = "SharImportEditor.SharWorldMaterialToolset"
_MASTER_TOOL = f"{_MATERIAL_TOOLSET}.CreateSimpleUnlitWorldMaster"
_INSTANCE_TOOL = (
    f"{_MATERIAL_TOOLSET}.CreateSimpleUnlitWorldMaterialInstance"
)
_SHA256_LENGTH = 64


class WorldMaterialTextureStep(NamedTuple):
    """One content-addressed world Texture2D import."""

    sha256: str
    byte_count: int
    source_path: str
    object_path: str
    package_path: str
    folder_path: str
    asset_name: str

    @property
    def target_class(self) -> str:
        """Expected native output class."""
        return "Texture2D"

    @property
    def toolset_name(self) -> str:
        """Native texture toolset identity."""
        return _TEXTURE_TOOLSET

    @property
    def tool_name(self) -> str:
        """Native texture import tool identity."""
        return _TEXTURE_TOOL

    def arguments(self, source_file: str) -> JsonObject:
        """Build native TextureTools arguments from a verified source path."""
        if not source_file:
            fail_protocol("world texture physical source is empty")
        return {
            "asset_name": self.asset_name,
            "folder_path": self.folder_path,
            "source_file": source_file,
        }


class WorldMaterialMasterStep(NamedTuple):
    """One reviewed simple-unlit native master construction."""

    recipe_identity: str
    object_path: str
    package_path: str
    folder_path: str
    asset_name: str
    blend_family: str
    alpha_test: bool

    @property
    def target_class(self) -> str:
        """Expected native output class."""
        return "Material"

    @property
    def toolset_name(self) -> str:
        """Native material toolset identity."""
        return _MATERIAL_TOOLSET

    @property
    def tool_name(self) -> str:
        """Native master construction tool identity."""
        return _MASTER_TOOL

    def arguments(self) -> JsonObject:
        """Build native master-construction arguments."""
        return {
            "assetName": self.asset_name,
            "bAlphaTest": self.alpha_test,
            "blendFamily": self.blend_family,
            "folderPath": self.folder_path,
        }


class WorldMaterialInstanceStep(NamedTuple):
    """One presentation-addressed MaterialInstanceConstant construction."""

    presentation_sha256: str
    recipe_identity: str
    object_path: str
    package_path: str
    folder_path: str
    asset_name: str
    parent_material_path: str
    base_color_texture_path: str | None
    texture_sha256: str | None
    base_color_tint: tuple[float, float, float, float]
    set_alpha_reference: bool
    alpha_reference: float | None

    @property
    def target_class(self) -> str:
        """Expected native output class."""
        return "MaterialInstanceConstant"

    @property
    def toolset_name(self) -> str:
        """Native material toolset identity."""
        return _MATERIAL_TOOLSET

    @property
    def tool_name(self) -> str:
        """Native instance construction tool identity."""
        return _INSTANCE_TOOL

    def arguments(self) -> JsonObject:
        """Build native Material Instance construction arguments."""
        red, green, blue, alpha = self.base_color_tint
        return {
            "alphaReference": self.alpha_reference or 0.0,
            "assetName": self.asset_name,
            "bSetAlphaReference": self.set_alpha_reference,
            "baseColorTexturePath": self.base_color_texture_path or "",
            "baseColorTint": {
                "a": alpha,
                "b": blue,
                "g": green,
                "r": red,
            },
            "folderPath": self.folder_path,
            "parentMaterialPath": self.parent_material_path,
        }


class WorldMaterialConstructionReport(NamedTuple):
    """Public-safe counts for the representable construction subset."""

    texture_count: int
    master_count: int
    instance_count: int
    blocked_presentation_count: int

    def to_json(self) -> JsonObject:
        """Render representable and blocked world-material counts."""
        return {
            "blockedPresentationCount": self.blocked_presentation_count,
            "instanceCount": self.instance_count,
            "masterCount": self.master_count,
            "textureCount": self.texture_count,
        }


class CompiledWorldMaterialConstruction(NamedTuple):
    """Ordered native requests compiled from one bound semantic sidecar."""

    report: WorldMaterialConstructionReport
    textures: tuple[WorldMaterialTextureStep, ...]
    masters: tuple[WorldMaterialMasterStep, ...]
    instances: tuple[WorldMaterialInstanceStep, ...]


def compile_world_material_construction(
    document: JsonObject,
) -> CompiledWorldMaterialConstruction:
    """Validate and type one v3 world-material construction document."""
    if document.get("schema") != _SCHEMA:
        fail_protocol("world-material construction schema is not supported")
    counts = _object(document.get("counts"), "world-material counts")
    native = _object(
        document.get("native_construction"),
        "world-material native construction",
    )
    raw_textures = _array(native, "texture_requests")
    raw_masters = _array(native, "master_requests")
    raw_instances = _array(native, "instance_requests")
    textures = tuple(_texture(item) for item in raw_textures)
    masters = tuple(_master(item) for item in raw_masters)
    instances = tuple(_instance(item) for item in raw_instances)
    _require_sorted_unique(textures, "sha256", "world texture")
    _require_sorted_unique(masters, "recipe_identity", "world master")
    _require_sorted_unique(
        instances,
        "presentation_sha256",
        "world material instance",
    )
    _require_unique_paths(textures, masters, instances)
    _require_dependency_links(textures, masters, instances)
    expected_texture_count = _integer(counts, "native_texture_requests")
    expected_master_count = _integer(counts, "native_master_requests")
    expected_instance_count = _integer(counts, "native_instance_requests")
    blocked_count = _integer(
        counts,
        "native_master_blocked_presentations",
    )
    ready_count = _integer(counts, "native_master_ready_presentations")
    presentation_count = _integer(counts, "presentations")
    if (
        expected_texture_count != len(textures)
        or expected_master_count != len(masters)
        or expected_instance_count != len(instances)
        or ready_count != len(instances)
        or ready_count + blocked_count != presentation_count
    ):
        fail_protocol(
            "world-material native construction counts are inconsistent"
        )
    return CompiledWorldMaterialConstruction(
        WorldMaterialConstructionReport(
            len(textures),
            len(masters),
            len(instances),
            blocked_count,
        ),
        textures,
        masters,
        instances,
    )


def _texture(value: JsonValue) -> WorldMaterialTextureStep:
    row = require_json_object(value, context="world texture request")
    sha256 = _sha(row, "sha256", "world texture request")
    source_path = _text(row, "source_path", "world texture request")
    expected_source = f"world-assets/textures/texture-{sha256}.png"
    if source_path != expected_source:
        fail_protocol("world texture source path is not content-addressed")
    folder, asset, package, object_path = _destination(
        row,
        _TEXTURE_ROOT,
        "world texture request",
    )
    return WorldMaterialTextureStep(
        sha256,
        _integer(row, "bytes"),
        source_path,
        object_path,
        package,
        folder,
        asset,
    )


def _master(value: JsonValue) -> WorldMaterialMasterStep:
    row = require_json_object(value, context="world master request")
    folder, asset, package, object_path = _destination(
        row,
        _MASTER_ROOT,
        "world master request",
    )
    recipe = _text(row, "recipe_identity", "world master request")
    blend = _text(row, "blend_family", "world master request")
    if blend not in {"additive", "alpha", "opaque"}:
        fail_protocol("world master blend family is unsupported")
    if row.get("render_both_faces") is not True:
        fail_protocol("world master request does not preserve source CullNone")
    alpha_test = _boolean(row, "alpha_test", "world master request")
    return WorldMaterialMasterStep(
        recipe,
        object_path,
        package,
        folder,
        asset,
        blend,
        alpha_test,
    )


def _instance(value: JsonValue) -> WorldMaterialInstanceStep:
    row = require_json_object(value, context="world material instance request")
    folder, asset, package, object_path = _destination(
        row,
        _INSTANCE_ROOT,
        "world material instance request",
    )
    presentation = _sha(
        row,
        "slot_presentation_sha256",
        "world material instance request",
    )
    recipe = _text(
        row,
        "recipe_identity",
        "world material instance request",
    )
    parent = _text(
        row,
        "parent_material_path",
        "world material instance request",
    )
    texture_path = _optional_text(row, "base_color_texture_path")
    texture_sha = _optional_sha(row, "texture_sha256")
    if (texture_path is None) != (texture_sha is None):
        fail_protocol("world material instance texture identity is incomplete")
    tint = _rgba(row.get("base_color_tint"))
    set_alpha = _boolean(
        row,
        "set_alpha_reference",
        "world material instance request",
    )
    alpha_reference = _optional_float(row, "alpha_reference")
    alpha_bits = row.get("alpha_reference_bits")
    if set_alpha:
        if alpha_reference is None or not isinstance(alpha_bits, int):
            fail_protocol(
                "world material instance alpha reference is incomplete"
            )
        packed = struct.unpack("<I", struct.pack("<f", alpha_reference))[0]
        if packed != alpha_bits:
            fail_protocol(
                "world material instance alpha reference bits disagree"
            )
    elif alpha_reference is not None or alpha_bits is not None:
        fail_protocol("world material instance has an inactive alpha reference")
    return WorldMaterialInstanceStep(
        presentation,
        recipe,
        object_path,
        package,
        folder,
        asset,
        parent,
        texture_path,
        texture_sha,
        tint,
        set_alpha,
        alpha_reference,
    )


def _destination(
    row: JsonObject,
    expected_folder: str,
    context: str,
) -> tuple[str, str, str, str]:
    folder = _text(row, "folder_path", context)
    asset = _text(row, "asset_name", context)
    package = _text(row, "package_path", context)
    object_path = _text(row, "object_path", context)
    if folder != expected_folder:
        fail_protocol(f"{context} escaped its generated folder")
    if package != f"{folder}/{asset}" or object_path != f"{package}.{asset}":
        fail_protocol(f"{context} destination is not canonical")
    return folder, asset, package, object_path


def _require_dependency_links(
    textures: tuple[WorldMaterialTextureStep, ...],
    masters: tuple[WorldMaterialMasterStep, ...],
    instances: tuple[WorldMaterialInstanceStep, ...],
) -> None:
    texture_by_sha = {item.sha256: item.object_path for item in textures}
    master_by_recipe = {
        item.recipe_identity: item.object_path for item in masters
    }
    for instance in instances:
        if master_by_recipe.get(instance.recipe_identity) != (
            instance.parent_material_path
        ):
            fail_protocol("world material instance parent is not planned")
        if instance.texture_sha256 is not None and texture_by_sha.get(
            instance.texture_sha256
        ) != instance.base_color_texture_path:
            fail_protocol("world material instance texture is not planned")


def _require_sorted_unique(
    values: tuple[object, ...],
    attribute: str,
    label: str,
) -> None:
    identities = [str(getattr(value, attribute)) for value in values]
    if (
        identities != sorted(identities)
        or len(identities) != len(set(identities))
    ):
        fail_protocol(f"{label} identities are not sorted and unique")


def _require_unique_paths(
    textures: tuple[WorldMaterialTextureStep, ...],
    masters: tuple[WorldMaterialMasterStep, ...],
    instances: tuple[WorldMaterialInstanceStep, ...],
) -> None:
    paths = [
        *(item.object_path for item in textures),
        *(item.object_path for item in masters),
        *(item.object_path for item in instances),
    ]
    if len(paths) != len(set(paths)):
        fail_protocol("world-material native destinations are not unique")


def _object(value: JsonValue | None, context: str) -> JsonObject:
    return require_json_object(value, context=context)


def _array(row: JsonObject, key: str) -> list[JsonValue]:
    value = row.get(key)
    if not isinstance(value, list):
        fail_protocol(f"world-material field {key} is not an array")
    return value


def _text(row: JsonObject, key: str, context: str) -> str:
    value = row.get(key)
    if not isinstance(value, str) or not value:
        fail_protocol(f"{context} field {key} is not non-empty text")
    return value


def _optional_text(row: JsonObject, key: str) -> str | None:
    value = row.get(key)
    if value is None:
        return None
    if not isinstance(value, str) or not value:
        fail_protocol(f"world material instance field {key} is invalid")
    return value


def _sha(row: JsonObject, key: str, context: str) -> str:
    value = _text(row, key, context)
    if len(value) != _SHA256_LENGTH or any(
        character not in "0123456789abcdef" for character in value
    ):
        fail_protocol(f"{context} field {key} is not canonical SHA-256")
    return value


def _optional_sha(row: JsonObject, key: str) -> str | None:
    value = row.get(key)
    if value is None:
        return None
    if not isinstance(value, str):
        fail_protocol(f"world material instance field {key} is invalid")
    return _sha({key: value}, key, "world material instance request")


def _integer(row: JsonObject, key: str) -> int:
    value = row.get(key)
    if isinstance(value, bool) or not isinstance(value, int) or value < 0:
        fail_protocol(
            f"world-material field {key} is not a nonnegative integer"
        )
    return value


def _boolean(row: JsonObject, key: str, context: str) -> bool:
    value = row.get(key)
    if not isinstance(value, bool):
        fail_protocol(f"{context} field {key} is not boolean")
    return value


def _optional_float(row: JsonObject, key: str) -> float | None:
    value = row.get(key)
    if value is None:
        return None
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        fail_protocol(f"world material instance field {key} is not numeric")
    result = float(value)
    if not math.isfinite(result):
        fail_protocol(f"world material instance field {key} is not finite")
    return result


def _rgba(value: JsonValue | None) -> tuple[float, float, float, float]:
    if not isinstance(value, list) or len(value) != 4:
        fail_protocol("world material instance tint is not RGBA")
    components: list[float] = []
    for component in value:
        if isinstance(component, bool) or not isinstance(
            component, (int, float)
        ):
            fail_protocol(
                "world material instance tint component is not numeric"
            )
        number = float(component)
        if not math.isfinite(number) or not 0.0 <= number <= 1.0:
            fail_protocol(
                "world material instance tint component is out of range"
            )
        components.append(number)
    return (components[0], components[1], components[2], components[3])

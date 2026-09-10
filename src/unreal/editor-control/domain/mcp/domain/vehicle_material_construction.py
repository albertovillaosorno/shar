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
#   - Typed compilation of reviewed vehicle material construction requests.
# - Must-Not:
#   - Read files, contact Unreal Editor, assign Skeletal Mesh slots, or promote
#   - blocked vehicle material families.
# - Allows:
#   - Compile verified Texture2D, Material, and MaterialInstanceConstant calls.
# - Split-When:
#   - Source verification or slot application gains independent policy.
# - Merge-When:
#   - Another domain module owns identical vehicle construction compilation.
# - Summary:
#   - Vehicle-material native construction compiler.
# - Description:
#   - Validates v4 generated identities, dependency links, and exact tool wires.
# - Usage:
#   - Called after release-index binding and before filesystem or live checks.
# - Defaults:
#   - Only the planned simple-unlit subset compiles; final readiness stays out.
#

"""Typed compilation of reviewed vehicle material construction requests."""

from __future__ import annotations

import math
from pathlib import PurePosixPath
import struct
from typing import NamedTuple

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import JsonValue
from mcp.domain.json_types import require_json_object

_SCHEMA = "shar-schoenwald.unreal-vehicle-material-evidence.v4"
_SOURCE_SCHEMA = "shar.vehicle-catalog.v8"
_TEXTURE_ROOT = "/Game/Generated/SHAR/Textures/Vehicles"
_MASTER_ROOT = "/Game/Generated/SHAR/Materials/Vehicles/Masters"
_INSTANCE_ROOT = "/Game/Generated/SHAR/Materials/Vehicles/Instances"
_IMPORT_TOOLSET = "SharImportEditor.SharImportToolset"
_TEXTURE_TOOL = f"{_IMPORT_TOOLSET}.ImportBaseColorTexture2D"
_MATERIAL_TOOLSET = "SharImportEditor.SharVehicleMaterialToolset"
_MASTER_TOOL = f"{_MATERIAL_TOOLSET}.CreateSimpleUnlitVehicleMaster"
_INSTANCE_TOOL = (
    f"{_MATERIAL_TOOLSET}.CreateSimpleUnlitVehicleMaterialInstance"
)
_SHA256_LENGTH = 64
_EXPECTED_POLICY = {
    "source_projection": "reviewed-pddi-render-state",
    "source_projection_status": "ready",
    "world_material_policy_reuse": "forbidden",
    "native_construction": "simple-unlit-texture-master-instance-ready",
    "mesh_slot_application": "blocked-pending-reviewed-transaction",
    "dynamic_light_binding": "headlight-sidecar-plus-slot-bound-rear-lights",
    "runtime_shader_mutation": "preserve-separately",
}


class VehicleMaterialTextureStep(NamedTuple):
    """One content-addressed vehicle Texture2D import."""

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
        """Native vehicle texture import toolset."""
        return _IMPORT_TOOLSET

    @property
    def tool_name(self) -> str:
        """Native vehicle texture import tool."""
        return _TEXTURE_TOOL

    def arguments(self, source_file: str) -> JsonObject:
        """Build SHAR PNG-import arguments from verified physical input."""
        if not source_file:
            fail_protocol("vehicle material texture physical source is empty")
        return {
            "assetName": self.asset_name,
            "folderPath": self.folder_path,
            "sourceFile": source_file,
        }


class VehicleMaterialMasterStep(NamedTuple):
    """One deduplicated reviewed vehicle simple-unlit master."""

    recipe_identity: str
    object_path: str
    package_path: str
    folder_path: str
    asset_name: str
    blend_mode: int
    alpha_test: bool
    two_sided: bool

    @property
    def target_class(self) -> str:
        """Expected native output class."""
        return "Material"

    @property
    def toolset_name(self) -> str:
        """Native vehicle material toolset."""
        return _MATERIAL_TOOLSET

    @property
    def tool_name(self) -> str:
        """Native reviewed vehicle master tool."""
        return _MASTER_TOOL

    def arguments(self) -> JsonObject:
        """Build exact simple-unlit vehicle master arguments."""
        return {
            "alphaCompare": 4,
            "assetName": self.asset_name,
            "bAlphaTest": self.alpha_test,
            "blendMode": self.blend_mode,
            "bLit": False,
            "bTwoSided": self.two_sided,
            "folderPath": self.folder_path,
            "shaderFamily": "simple",
        }


class VehicleMaterialInstanceStep(NamedTuple):
    """One graph-ready vehicle slot Material Instance request."""

    request_identity: str
    package_id: str
    source_fbx: str
    slot_index: int
    slot_name: str
    source_material_name: str
    recipe_identity: str
    texture_sha256: str | None
    object_path: str
    package_path: str
    folder_path: str
    asset_name: str
    parent_material_path: str
    base_color_texture_path: str | None
    base_color_tint: tuple[float, float, float, float]
    set_alpha_reference: bool
    alpha_reference: float | None

    @property
    def target_class(self) -> str:
        """Expected native output class."""
        return "MaterialInstanceConstant"

    @property
    def toolset_name(self) -> str:
        """Native vehicle material toolset."""
        return _MATERIAL_TOOLSET

    @property
    def tool_name(self) -> str:
        """Native reviewed vehicle Material Instance tool."""
        return _INSTANCE_TOOL

    def arguments(self) -> JsonObject:
        """Build exact reflected vehicle Material Instance arguments."""
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


class VehicleMaterialConstructionReport(NamedTuple):
    """Public-safe native request counts."""

    texture_count: int
    master_count: int
    instance_count: int
    blocked_slot_count: int

    def to_json(self) -> JsonObject:
        """Render request and final-blocker counts."""
        return {
            "blockedSlotCount": self.blocked_slot_count,
            "instanceCount": self.instance_count,
            "masterCount": self.master_count,
            "textureCount": self.texture_count,
        }


class CompiledVehicleMaterialConstruction(NamedTuple):
    """Typed native vehicle material requests in dependency order."""

    report: VehicleMaterialConstructionReport
    textures: tuple[VehicleMaterialTextureStep, ...]
    masters: tuple[VehicleMaterialMasterStep, ...]
    instances: tuple[VehicleMaterialInstanceStep, ...]


def compile_vehicle_material_construction(
    document: JsonObject,
) -> CompiledVehicleMaterialConstruction:
    """Validate and type one v4 vehicle-material construction document."""
    if document.get("schema") != _SCHEMA:
        fail_protocol("vehicle-material construction schema is not supported")
    if document.get("source_schema") != _SOURCE_SCHEMA:
        fail_protocol("vehicle-material source schema is not supported")
    policy = require_json_object(
        document.get("target_policy"), context="vehicle-material target policy"
    )
    if policy != _EXPECTED_POLICY:
        fail_protocol("vehicle-material target policy drifted")
    counts = require_json_object(
        document.get("counts"), context="vehicle-material counts"
    )
    native = require_json_object(
        document.get("native_construction"),
        context="vehicle-material native construction",
    )
    textures = tuple(
        _texture(item) for item in _array(native, "texture_requests")
    )
    masters = tuple(
        _master(item) for item in _array(native, "master_requests")
    )
    instances = tuple(
        _instance(item) for item in _array(native, "instance_requests")
    )
    _require_sorted_unique(textures, "sha256", "vehicle texture")
    _require_sorted_unique(masters, "recipe_identity", "vehicle master")
    _require_sorted_unique(instances, "request_identity", "vehicle instance")
    _require_unique_paths(textures, masters, instances)
    _require_dependency_links(textures, masters, instances)
    texture_count = _integer(counts, "native_texture_requests")
    master_count = _integer(counts, "native_master_requests")
    instance_count = _integer(counts, "native_instance_requests")
    graph_ready = _integer(counts, "native_graph_ready_slots")
    construction_ready = _integer(counts, "native_construction_ready_slots")
    blocked = _integer(counts, "native_blocked_slots")
    native_ready = _integer(counts, "native_ready_slots")
    slots = _integer(counts, "slots")
    counts_match = all((
        texture_count == len(textures),
        master_count == len(masters),
        instance_count == len(instances),
        graph_ready == instance_count,
        construction_ready == instance_count,
        native_ready == 0,
        blocked == slots,
    ))
    if not counts_match:
        fail_protocol(
            "vehicle-material native construction counts are inconsistent"
        )
    return CompiledVehicleMaterialConstruction(
        VehicleMaterialConstructionReport(
            len(textures), len(masters), len(instances), blocked
        ),
        textures,
        masters,
        instances,
    )


def _texture(value: JsonValue) -> VehicleMaterialTextureStep:
    row = require_json_object(value, context="vehicle texture request")
    sha256 = _sha(row, "sha256", "vehicle texture request")
    source_path = _text(row, "source_path", "vehicle texture request")
    parts = PurePosixPath(source_path).parts
    canonical_shape = len(parts) == 4 and all((
        parts[0] == "vehicle-assets",
        parts[2] == "textures",
        bool(parts[1]),
        parts[3].endswith(".png"),
    ))
    if not canonical_shape or any(part in {".", ".."} for part in parts):
        fail_protocol("vehicle texture source path is not canonical")
    folder, asset, package, object_path = _destination(
        row, _TEXTURE_ROOT, "vehicle texture request"
    )
    if asset != f"T_Vehicle_{sha256}":
        fail_protocol("vehicle texture destination is not content-addressed")
    return VehicleMaterialTextureStep(
        sha256,
        _integer(row, "bytes"),
        source_path,
        object_path,
        package,
        folder,
        asset,
    )


def _master(value: JsonValue) -> VehicleMaterialMasterStep:
    row = require_json_object(value, context="vehicle master request")
    folder, asset, package, object_path = _destination(
        row, _MASTER_ROOT, "vehicle master request"
    )
    if row.get("shader_family") != "simple" or row.get("lit") is not False:
        fail_protocol("vehicle master request escaped simple-unlit policy")
    blend = _integer(row, "blend_mode")
    if blend not in {0, 1, 2} or _integer(row, "alpha_compare") != 4:
        fail_protocol("vehicle master raster policy is unsupported")
    alpha_test = _boolean(row, "alpha_test", "vehicle master request")
    two_sided = _boolean(row, "two_sided", "vehicle master request")
    recipe = _text(row, "recipe_identity", "vehicle master request")
    expected_recipe = _recipe_identity(
        blend, alpha_test=alpha_test, two_sided=two_sided
    )
    expected_asset = _master_asset(
        blend, alpha_test=alpha_test, two_sided=two_sided
    )
    if recipe != expected_recipe or asset != expected_asset:
        fail_protocol("vehicle master recipe identity drifted")
    return VehicleMaterialMasterStep(
        recipe,
        object_path,
        package,
        folder,
        asset,
        blend,
        alpha_test,
        two_sided,
    )


def _instance(value: JsonValue) -> VehicleMaterialInstanceStep:
    row = require_json_object(
        value, context="vehicle material instance request"
    )
    destination = _destination(
        row, _INSTANCE_ROOT, "vehicle material instance request"
    )
    request = _sha(
        row, "request_identity", "vehicle material instance request"
    )
    if destination[1] != f"MI_Vehicle_{request}":
        fail_protocol("vehicle material instance destination is not canonical")
    recipe = _text(
        row, "recipe_identity", "vehicle material instance request"
    )
    parent = _text(
        row, "parent_material_path", "vehicle material instance request"
    )
    texture_path = _optional_text(row, "base_color_texture_path")
    texture_sha = _optional_sha(row, "texture_sha256")
    if (texture_path is None) != (texture_sha is None):
        fail_protocol(
            "vehicle material instance texture identity is incomplete"
        )
    tint = _rgba(row.get("base_color_tint"))
    set_alpha, alpha = _instance_alpha(row)
    return VehicleMaterialInstanceStep(
        request,
        _text(row, "package_id", "vehicle material instance request"),
        _text(row, "source_fbx", "vehicle material instance request"),
        _integer(row, "slot_index"),
        _text(row, "slot_name", "vehicle material instance request"),
        _text(
            row,
            "source_material_name",
            "vehicle material instance request",
        ),
        recipe,
        texture_sha,
        destination[3],
        destination[2],
        destination[0],
        destination[1],
        parent,
        texture_path,
        tint,
        set_alpha,
        alpha,
    )


def _instance_alpha(row: JsonObject) -> tuple[bool, float | None]:
    set_alpha = _boolean(
        row, "set_alpha_reference", "vehicle material instance request"
    )
    alpha = _optional_float(row, "alpha_reference")
    bits = row.get("alpha_reference_bits")
    if set_alpha:
        if alpha is None or isinstance(bits, bool) or not isinstance(bits, int):
            fail_protocol(
                "vehicle material instance alpha reference is incomplete"
            )
        packed = struct.unpack("<I", struct.pack("<f", alpha))[0]
        if packed != bits:
            fail_protocol(
                "vehicle material instance alpha reference bits disagree"
            )
    elif alpha is not None or bits is not None:
        fail_protocol(
            "vehicle material instance has an inactive alpha reference"
        )
    return set_alpha, alpha


def _require_dependency_links(
    textures: tuple[VehicleMaterialTextureStep, ...],
    masters: tuple[VehicleMaterialMasterStep, ...],
    instances: tuple[VehicleMaterialInstanceStep, ...],
) -> None:
    texture_by_sha = {item.sha256: item.object_path for item in textures}
    master_by_recipe = {
        item.recipe_identity: item.object_path for item in masters
    }
    for instance in instances:
        if master_by_recipe.get(instance.recipe_identity) != (
            instance.parent_material_path
        ):
            fail_protocol("vehicle material instance parent is not planned")
        if instance.texture_sha256 is not None and texture_by_sha.get(
            instance.texture_sha256
        ) != instance.base_color_texture_path:
            fail_protocol("vehicle material instance texture is not planned")


def _require_sorted_unique(
    values: tuple[object, ...], attribute: str, label: str
) -> None:
    identities = [str(getattr(value, attribute)) for value in values]
    if (
        identities != sorted(identities)
        or len(identities) != len(set(identities))
    ):
        fail_protocol(f"{label} identities are not sorted and unique")


def _require_unique_paths(
    textures: tuple[VehicleMaterialTextureStep, ...],
    masters: tuple[VehicleMaterialMasterStep, ...],
    instances: tuple[VehicleMaterialInstanceStep, ...],
) -> None:
    paths = [
        *(item.object_path for item in textures),
        *(item.object_path for item in masters),
        *(item.object_path for item in instances),
    ]
    if len(paths) != len(set(paths)):
        fail_protocol("vehicle-material native destinations are not unique")


def _recipe_identity(
    blend: int, *, alpha_test: bool, two_sided: bool
) -> str:
    blend_name = {0: "opaque", 1: "alpha", 2: "additive"}[blend]
    return (
        f"simple-unlit-blend-{blend_name}-alpha-test-"
        f"{'on' if alpha_test else 'off'}-"
        f"{'both-faces' if two_sided else 'one-sided'}"
    )


def _master_asset(
    blend: int, *, alpha_test: bool, two_sided: bool
) -> str:
    blend_name = {0: "Opaque", 1: "Alpha", 2: "Additive"}[blend]
    return (
        f"M_SHAR_Vehicle_SimpleUnlit_{blend_name}_"
        f"AlphaTest{'On' if alpha_test else 'Off'}_"
        f"{'TwoSided' if two_sided else 'OneSided'}"
    )


def _destination(
    row: JsonObject, expected_folder: str, context: str
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


def _array(row: JsonObject, key: str) -> list[JsonValue]:
    value = row.get(key)
    if not isinstance(value, list):
        fail_protocol(f"vehicle-material field {key} is not an array")
    return value


def _text(row: JsonObject, key: str, context: str) -> str:
    value = row.get(key)
    if not isinstance(value, str) or not value or value.strip() != value:
        fail_protocol(f"{context} field {key} is not canonical text")
    return value


def _optional_text(row: JsonObject, key: str) -> str | None:
    value = row.get(key)
    if value is None:
        return None
    if not isinstance(value, str) or not value:
        fail_protocol(f"vehicle material instance field {key} is invalid")
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
        fail_protocol(f"vehicle material instance field {key} is invalid")
    return _sha({key: value}, key, "vehicle material instance request")


def _integer(row: JsonObject, key: str) -> int:
    value = row.get(key)
    if isinstance(value, bool) or not isinstance(value, int) or value < 0:
        fail_protocol(
            f"vehicle-material field {key} is not nonnegative integer"
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
        fail_protocol(f"vehicle material instance field {key} is not numeric")
    result = float(value)
    if not math.isfinite(result) or not 0.0 <= result <= 1.0:
        fail_protocol(
            f"vehicle material instance field {key} is not normalized"
        )
    return result


def _rgba(value: JsonValue | None) -> tuple[float, float, float, float]:
    if not isinstance(value, list) or len(value) != 4:
        fail_protocol("vehicle material instance tint is not RGBA")
    components: list[float] = []
    for component in value:
        if isinstance(component, bool) or not isinstance(
            component, (int, float)
        ):
            fail_protocol(
                "vehicle material instance tint component is not numeric"
            )
        number = float(component)
        if not math.isfinite(number) or not 0.0 <= number <= 1.0:
            fail_protocol("vehicle material instance tint is not normalized")
        components.append(number)
    return components[0], components[1], components[2], components[3]

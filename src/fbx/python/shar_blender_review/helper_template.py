# File:
#   - helper_template.py
# Path:
#   - src/fbx/python/shar_blender_review/helper_template.py
#
# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE
# Path-Rule:
#   - All paths in this header are repository-root relative.
#
# Boundary-Contract:
# - Owns:
#   - The typed standalone Blender review-helper template.
# - Must-Not:
#   - Contain machine-local paths, invoke itself, or alter FBX source data.
# - Allows:
#   - Retain this source-template provenance header in generated copies.
# - Split-When:
#   - Runtime protocols and helper orchestration gain independent contracts.
# - Merge-When:
#   - Another tracked template owns the same generated helper behavior.
# - Summary:
#   - Provides deterministic Blender-side FBX inspection behavior.
# - Description:
#   - Imports one sibling FBX, selects its armature and animation, and preserves
#   - native frame timing without rewriting source animation data.
# - Usage:
#   - Materialized only by the explicit experimental review-helper option.
# - Defaults:
#   - No Python package marker is emitted beside generated helpers.
#
# ADRs:
# - docs/adr/fbx/export/fbx-output-contract-boundary.md
#
# Large file:
#   - false
#
# Standalone generated helper; no Python package marker is emitted.
# ruff: noqa: INP001
"""Experimental unsupported Blender FBX review helper; may not work."""

from __future__ import annotations

from importlib import import_module
from pathlib import Path
from typing import TYPE_CHECKING, Protocol, cast

if TYPE_CHECKING:
    from collections.abc import Iterable, Sequence

FBX_FILE_NAME: str = "__SHAR_FBX_FILE_NAME__"
SOURCE_FPS: int = 1


class _ActionSlot(Protocol):
    """Typed target slot owned by one Blender Action."""

    target_id_type: str


class _Action(Protocol):
    """Subset of Blender Action data required by this helper."""

    name: str
    frame_range: Sequence[float]
    slots: Iterable[_ActionSlot]


class _AnimationData(Protocol):
    """Armature Action assignment state."""

    action: _Action | None
    action_slot: _ActionSlot | None


class _ArmatureData(Protocol):
    """Armature display state required for animation review."""

    pose_position: str


class _Object(Protocol):
    """Subset of Blender Object behavior used by this helper."""

    name: str
    type: str
    id_type: str
    data: _ArmatureData
    show_in_front: bool
    animation_data: _AnimationData | None

    def animation_data_create(self) -> None:
        """Create animation assignment storage when absent."""

    def select_set(self, *, state: bool) -> None:
        """Set viewport selection state."""


class _RenderSettings(Protocol):
    """Scene playback-rate settings."""

    fps: int
    fps_base: float


class _Scene(Protocol):
    """Subset of Blender Scene behavior used by this helper."""

    render: _RenderSettings
    objects: Iterable[_Object]
    frame_start: int
    frame_end: int

    def frame_set(self, frame: int) -> None:
        """Evaluate the scene at one integer frame."""


class _ViewLayerObjects(Protocol):
    """Active-object state for one Blender view layer."""

    active: _Object | None


class _ViewLayer(Protocol):
    """Blender view-layer state used for armature activation."""

    objects: _ViewLayerObjects


class _Context(Protocol):
    """Blender process context used by the helper."""

    scene: _Scene
    view_layer: _ViewLayer


class _WmOperators(Protocol):
    """Blender window-manager operators used by the helper."""

    def read_factory_settings(self, *, use_empty: bool) -> set[str]:
        """Reset Blender to an empty deterministic scene."""
        ...

    def fbx_import(self, *, filepath: str) -> set[str]:
        """Import one FBX file."""
        ...


class _ObjectOperators(Protocol):
    """Blender object operators used by the helper."""

    def select_all(self, *, action: str) -> set[str]:
        """Apply one selection action to every scene object."""
        ...


class _Operators(Protocol):
    """Blender operator namespaces used by the helper."""

    wm: _WmOperators
    object: _ObjectOperators


class _TextBlock(Protocol):
    """Loaded Blender text datablock with its source file identity."""

    name: str
    filepath: str


class _BlendData(Protocol):
    """Blender data collections used by the helper."""

    actions: Iterable[_Action]
    texts: Iterable[_TextBlock]


class _BpyModule(Protocol):
    """Typed runtime surface consumed from Blender's dynamic module."""

    ops: _Operators
    context: _Context
    data: _BlendData


class ReviewHelperError(RuntimeError):
    """Raised when the generated review helper cannot configure Blender."""


def _load_bpy() -> _BpyModule:
    """Load Blender's runtime module behind a strict protocol boundary.

    Returns:
        The typed subset of Blender used by this helper.
    """
    return cast("_BpyModule", import_module("bpy"))


def _resolve_script_path(bpy: _BpyModule) -> Path:
    """Resolve the loaded Text Editor file instead of Blender's short name.

    Returns:
        The absolute generated helper path.

    Raises:
        ReviewHelperError: When no loaded text datablock owns the script.
    """
    direct = Path(__file__)
    if direct.is_absolute() and direct.is_file():
        return direct.resolve()
    matching_paths = sorted(
        (
            Path(text.filepath)
            for text in bpy.data.texts
            if text.filepath
            and (
                text.name == direct.name
                or Path(text.filepath).name == direct.name
            )
        ),
        key=lambda candidate: candidate.as_posix(),
    )
    existing = [
        candidate.resolve()
        for candidate in matching_paths
        if candidate.is_file()
    ]
    if len(existing) != 1:
        message = (
            f"Expected one loaded helper path for {direct.name!r}, "
            f"found {len(existing)}"
        )
        raise ReviewHelperError(message)
    return existing[0]


def _compatible_slot(
    action: _Action,
    armature: _Object,
) -> _ActionSlot | None:
    """Return the Action slot intended for the selected armature."""
    return next(
        (
            slot
            for slot in action.slots
            if slot.target_id_type == armature.id_type
        ),
        None,
    )


def _select_preferred_action(actions: Sequence[_Action]) -> _Action | None:
    """Prefer walking for immediate review, then fall back deterministically.

    Returns:
        The preferred Action, or `None` when no Actions were imported.
    """
    return next(
        (action for action in actions if action.name.endswith("_loco_walk")),
        actions[0] if actions else None,
    )


def _require_single_armature(scene: _Scene) -> _Object:
    """Return the unique imported armature or fail visibly.

    Returns:
        The single imported armature.

    Raises:
        ReviewHelperError: When zero or multiple armatures were imported.
    """
    armatures = sorted(
        (obj for obj in scene.objects if obj.type == "ARMATURE"),
        key=lambda obj: obj.name,
    )
    if len(armatures) != 1:
        message = f"Expected one armature, found {len(armatures)}"
        raise ReviewHelperError(message)
    return armatures[0]


def _configure_action(
    scene: _Scene,
    armature: _Object,
    action: _Action,
) -> None:
    """Assign one Action and its compatible Blender 5 slot.

    Raises:
        ReviewHelperError: When Blender cannot create animation data.
    """
    armature.animation_data_create()
    animation_data = armature.animation_data
    if animation_data is None:
        message = "Blender did not create armature animation data"
        raise ReviewHelperError(message)
    animation_data.action = action
    slot = _compatible_slot(action, armature)
    if slot is not None:
        animation_data.action_slot = slot
    scene.frame_start = int(action.frame_range[0])
    scene.frame_end = int(action.frame_range[1])
    scene.frame_set(scene.frame_start)


def main() -> None:
    """Import the sibling FBX and configure faithful native review.

    Raises:
        FileNotFoundError: When the sibling FBX is missing.
        ReviewHelperError: When import or scene configuration fails.
    """
    if SOURCE_FPS <= 0:
        message = "Generated source frame rate must be positive"
        raise ReviewHelperError(message)
    bpy = _load_bpy()
    script_path = _resolve_script_path(bpy)
    fbx_path = script_path.with_name(FBX_FILE_NAME)
    if not fbx_path.is_file():
        message = f"Missing sibling FBX: {fbx_path}"
        raise FileNotFoundError(message)

    _ = bpy.ops.wm.read_factory_settings(use_empty=True)
    import_result = bpy.ops.wm.fbx_import(filepath=str(fbx_path))
    if "FINISHED" not in import_result:
        message = f"FBX import failed: {sorted(import_result)}"
        raise ReviewHelperError(message)

    scene = bpy.context.scene
    scene.render.fps = SOURCE_FPS
    scene.render.fps_base = 1.0
    actions = sorted(bpy.data.actions, key=lambda action: action.name)

    armature = _require_single_armature(scene)
    armature.data.pose_position = "POSE"
    armature.show_in_front = True
    _ = bpy.ops.object.select_all(action="DESELECT")
    armature.select_set(state=True)
    bpy.context.view_layer.objects.active = armature

    preferred = _select_preferred_action(actions)
    if preferred is not None:
        _configure_action(scene, armature, preferred)


if __name__ == "__main__":
    main()

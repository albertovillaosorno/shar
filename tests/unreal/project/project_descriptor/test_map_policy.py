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
#   - Test map policy test module.
# - Must-Not:
#   - Own unrelated policy, persistence, or external effects.
# - Allows:
#   - Inputs and outputs required by this module boundary.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Test map policy test module.
# - Description:
#   - Implements the declared test module responsibility for project.
# - Usage:
#   - Used through the owning function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Test map policy test module."""

from pathlib import Path

PROJECT_ROOT = (
    Path(__file__).resolve().parents[4]
    / "src/unreal/project/composition/uproject"
)
REPOSITORY_ROOT = Path(__file__).resolve().parents[4]
WORLD_NAME = "W_SHAR_OpenWorld"
CANONICAL_MAP = f"/Game/SHAR/Maps/OpenWorld/{WORLD_NAME}"
GUIDE_IGNORE_RULE = (
    "src/unreal/project/composition/uproject/Content/SHAR/"
    "EditorOnly/StructuralGuide/"
)
WORLD_MAP_IGNORE_RULE = (
    # jig-ignore-next-line: exact value is indivisible
    f"!src/unreal/project/composition/uproject/Content/SHAR/Maps/OpenWorld/{WORLD_NAME}.umap"
)
WORLD_ACTORS_IGNORE_RULE = (
    # jig-ignore-next-line: exact value is indivisible
    "!src/unreal/project/composition/uproject/Content/__ExternalActors__/SHAR/Maps/OpenWorld/"
    f"{WORLD_NAME}/**/*.uasset"
)
WORLD_OBJECTS_IGNORE_RULE = (
    # jig-ignore-next-line: exact value is indivisible
    "!src/unreal/project/composition/uproject/Content/__ExternalObjects__/SHAR/Maps/OpenWorld/"
    f"{WORLD_NAME}/**/*.uasset"
)
WORLD_HLOD_IGNORE_RULE = (
    "!src/unreal/project/composition/uproject/Content/SHAR/Maps/OpenWorld/"
    f"{WORLD_NAME}_HLOD0_Instancing.uasset"
)
WORLD_LFS_RULE = (
    "src/unreal/project/composition/uproject/Content/SHAR/Maps/OpenWorld/"
    f"{WORLD_NAME}.umap "
    "filter=lfs diff=lfs merge=lfs -text"
)


def test_canonical_open_world_is_the_editor_and_game_default() -> None:
    engine_config = (PROJECT_ROOT / "Config/DefaultEngine.ini").read_text(
        encoding="utf-8"
    )

    assert f"EditorStartupMap={CANONICAL_MAP}" in engine_config
    assert f"GameDefaultMap={CANONICAL_MAP}" in engine_config
    assert (
        "GameDefaultMap=/Engine/Maps/Templates/OpenWorld" not in engine_config
    )


def test_structural_guide_content_is_never_cooked() -> None:
    game_config = (PROJECT_ROOT / "Config/DefaultGame.ini").read_text(
        encoding="utf-8"
    )

    assert (
        '+DirectoriesToNeverCook=(Path="/Game/SHAR/EditorOnly")' in game_config
    )


def test_empty_world_shell_retains_only_world_data_layers() -> None:
    content_root = PROJECT_ROOT / "Content"
    actor_root = (
        content_root
        / "__ExternalActors__/SHAR/Maps/OpenWorld"
        / WORLD_NAME
    )
    object_root = (
        content_root
        / "__ExternalObjects__/SHAR/Maps/OpenWorld"
        / WORLD_NAME
    )
    map_root = content_root / "SHAR/Maps/OpenWorld"

    actor_packages = sorted(
        path for path in actor_root.rglob("*.uasset") if path.is_file()
    )
    object_packages = sorted(
        path for path in object_root.rglob("*.uasset") if path.is_file()
    ) if object_root.exists() else []
    hlod_packages = sorted(
        path for path in map_root.glob(f"{WORLD_NAME}_HLOD*.uasset")
        if path.is_file()
    )

    assert len(actor_packages) == 1
    assert b"WorldDataLayers" in actor_packages[0].read_bytes()
    assert object_packages == []
    assert hlod_packages == []


def test_only_empty_world_shell_is_lfs_published() -> None:
    ignore_rules = set(
        (REPOSITORY_ROOT / ".gitignore")
        .read_text(encoding="utf-8")
        .splitlines()
    )
    attribute_rules = set(
        (REPOSITORY_ROOT / ".gitattributes")
        .read_text(encoding="utf-8")
        .splitlines()
    )

    assert GUIDE_IGNORE_RULE in ignore_rules
    assert WORLD_MAP_IGNORE_RULE in ignore_rules
    assert WORLD_ACTORS_IGNORE_RULE not in ignore_rules
    assert WORLD_OBJECTS_IGNORE_RULE not in ignore_rules
    assert WORLD_HLOD_IGNORE_RULE not in ignore_rules
    assert WORLD_LFS_RULE in attribute_rules
    assert not any("__ExternalActors__" in rule for rule in attribute_rules)
    assert not any("__ExternalObjects__" in rule for rule in attribute_rules)
    assert not any("_HLOD" in rule for rule in attribute_rules)

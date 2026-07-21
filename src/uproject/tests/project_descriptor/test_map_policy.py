# File:
#   - test_map_policy.py
# Path:
#   - src/uproject/tests/project_descriptor/test_map_policy.py
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
#   - Startup-map and editor-only cook-exclusion policy.
# - Must-Not:
#   - Invoke Unreal, mutate native assets, or inspect local ignored content.
# - Allows:
#   - Deterministic assertions over tracked project configuration.
# - Split-When:
#   - Another map family gains an independent startup or packaging policy.
# - Merge-When:
#   - Another module proves these exact configuration contracts.
# - Summary:
#   - Regression tests for the canonical open-world configuration.
# - Description:
#   - Locks the official startup map and local structural-guide exclusion.
# - Usage:
#   - Run by pytest before Unreal project configuration is committed.
# - Defaults:
#   - Tracked INI files are the fixtures.
#
# ADRs:
# - docs/adr/pipeline/unreal/world-partition-and-data-layer-import.md
#
# Large file:
#   - false
#

"""Canonical open-world startup and packaging policy."""

from pathlib import Path

PROJECT_ROOT = Path(__file__).resolve().parents[2]
REPOSITORY_ROOT = PROJECT_ROOT.parents[1]
WORLD_NAME = "W_SHAR_OpenWorld"
CANONICAL_MAP = f"/Game/SHAR/Maps/OpenWorld/{WORLD_NAME}"
GUIDE_IGNORE_RULE = "src/uproject/Content/SHAR/EditorOnly/StructuralGuide/"
WORLD_MAP_IGNORE_RULE = (
    f"!src/uproject/Content/SHAR/Maps/OpenWorld/{WORLD_NAME}.umap"
)
WORLD_ACTORS_IGNORE_RULE = (
    "!src/uproject/Content/__ExternalActors__/SHAR/Maps/OpenWorld/"
    f"{WORLD_NAME}/**/*.uasset"
)
WORLD_OBJECTS_IGNORE_RULE = (
    "!src/uproject/Content/__ExternalObjects__/SHAR/Maps/OpenWorld/"
    f"{WORLD_NAME}/**/*.uasset"
)
WORLD_LFS_RULE = (
    f"src/uproject/Content/SHAR/Maps/OpenWorld/{WORLD_NAME}.umap "
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


def test_structural_guide_is_local_and_world_is_lfs_published() -> None:
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
    assert WORLD_ACTORS_IGNORE_RULE in ignore_rules
    assert WORLD_OBJECTS_IGNORE_RULE in ignore_rules
    assert WORLD_LFS_RULE in attribute_rules

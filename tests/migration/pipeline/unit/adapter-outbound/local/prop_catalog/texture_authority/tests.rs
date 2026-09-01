// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::{
    SharedTextureAuthority, TextureSource, phase_three_texture_member_id,
};
use crate::domain::package::PhaseThreePackageRow;

#[test]
fn same_level_terrain_mesh_is_preferred() {
    let authority = SharedTextureAuthority {
        sources: BTreeMap::from([("tree.bmp".to_owned(), vec![
            TextureSource {
                package_id: "level-one".to_owned(),
                package_member_id: "texture-level-one".to_owned(),
                subcategory: "terrain-world/level-01/\
                                          terrain-mesh"
                    .to_owned(),
                member_id: "tree".to_owned(),
                source_ordinal: 1,
                path: PathBuf::from("level-one.png"),
                sha256: "one".to_owned(),
            },
            TextureSource {
                package_id: "level-five".to_owned(),
                package_member_id: "texture-level-five".to_owned(),
                subcategory: "terrain-world/level-05/\
                                          terrain-mesh"
                    .to_owned(),
                member_id: "tree".to_owned(),
                source_ordinal: 5,
                path: PathBuf::from("level-five.png"),
                sha256: "five".to_owned(),
            },
        ])]),
    };

    let result =
        authority.resolve("tree.bmp", "terrain-world/level-01/regions/l1r1");

    assert_eq!(result, Ok(Some(std::path::Path::new("level-one.png"))));
}

#[test]
fn conflicting_same_level_payloads_are_rejected() {
    let authority = SharedTextureAuthority {
        sources: BTreeMap::from([("tree.bmp".to_owned(), vec![
            TextureSource {
                package_id: "region-a".to_owned(),
                package_member_id: "texture-region-a".to_owned(),
                subcategory: "terrain-world/level-01/regions/a".to_owned(),
                member_id: "tree-a".to_owned(),
                source_ordinal: 10,
                path: PathBuf::from("a.png"),
                sha256: "a".to_owned(),
            },
            TextureSource {
                package_id: "region-b".to_owned(),
                package_member_id: "texture-region-b".to_owned(),
                subcategory: "terrain-world/level-01/regions/b".to_owned(),
                member_id: "tree-b".to_owned(),
                source_ordinal: 20,
                path: PathBuf::from("b.png"),
                sha256: "b".to_owned(),
            },
        ])]),
    };

    assert!(
        authority
            .resolve("tree.bmp", "terrain-world/level-01/regions/c",)
            .is_err()
    );
}


#[test]
fn conflicting_preferred_occurrences_remain_available_as_evidence(
) -> Result<(), String> {
    let authority = SharedTextureAuthority {
        sources: BTreeMap::from([("glow.bmp".to_owned(), vec![
            TextureSource {
                package_id: "level-three-terrain".to_owned(),
                subcategory: "terrain-world/level-03/terrain-mesh".to_owned(),
                package_member_id: "texture-member-182".to_owned(),
                member_id: "glow-a".to_owned(),
                source_ordinal: 182,
                path: PathBuf::from("glow-a.png"),
                sha256: "one".to_owned(),
            },
            TextureSource {
                package_id: "level-three-terrain".to_owned(),
                subcategory: "terrain-world/level-03/terrain-mesh".to_owned(),
                package_member_id: "texture-member-10591".to_owned(),
                member_id: "glow-b".to_owned(),
                source_ordinal: 10591,
                path: PathBuf::from("glow-b.png"),
                sha256: "two".to_owned(),
            },
        ])]),
    };

    let occurrences = authority
        .preferred_occurrences(
            "glow.bmp",
            "terrain-world/level-03/regions/l3r1",
        )
        .map_err(|error| error.to_string())?;

    assert_eq!(occurrences.len(), 2);
    assert_eq!(
        occurrences
            .first()
            .map(|occurrence| occurrence.package_member_id.as_str()),
        Some("texture-member-182")
    );
    assert_eq!(
        occurrences.first().map(|occurrence| occurrence.sha256.as_str()),
        Some("one")
    );
    assert_eq!(
        occurrences.get(1).map(|occurrence| occurrence.sha256.as_str()),
        Some("two")
    );
    assert!(
        authority
            .resolve("glow.bmp", "terrain-world/level-03/regions/l3r1")
            .is_err(),
        "evidence access must not weaken fail-closed resolution"
    );
    Ok(())
}


#[test]
fn phase_three_texture_member_requires_exact_path_and_ordinal(
) -> Result<(), String> {
    let package = PhaseThreePackageRow::from_json_line(concat!(
        r#"{"package_id":"extracted-art-l1r1","#,
        r#""package_root":"extracted/art/l1r1","#,
        r#""package_category":"terrain-world","#,
        r#""package_subcategory":"terrain-world/level-01/regions/l1r1","#,
        r#""unit_count":2,"text_key_count":0,"#,
        r#""unit_ids":["texture-a","texture-b"],"world_ids":[],"#,
        r#""texture_ids":["texture-a","texture-b"],"material_ids":[],"#,
        r#""model_ids":[],"physics_ids":[],"animation_ids":[],"#,
        r#""scene_ids":[],"locator_ids":[],"camera_ids":[],"#,
        r#""light_ids":[],"particle_ids":[],"controller_ids":[],"#,
        r#""audio_ids":[],"movie_ids":[],"script_ids":[],"#,
        r#""text_ids":[],"ui_ids":[],"metadata_ids":[],"error_ids":[],"#,
        r#""source_unit_ids":[],"text_key_ids":[],"members":[{"#,
        r#""id":"texture-a","role":"texture","#,
        r#""path":"extracted/art/l1r1/components/texture/glow.png","#,
        r#""type":"image","kind":"p3d-texture","#,
        r#""source_chunk_kind":"texture","source_chunk_ordinal":"182"},{"#,
        r#""id":"texture-b","role":"texture","#,
        r#""path":"extracted/art/l1r1/components/texture/"#,
        r#"glow__ordinal_9.png","#,
        r#""type":"image","kind":"p3d-texture","#,
        r#""source_chunk_kind":"texture","source_chunk_ordinal":"9"}],"#,
        r#""text_keys":[]}"#
    ))
    .map_err(|error| error.to_string())?;

    assert_eq!(
        phase_three_texture_member_id(&package, "texture/glow.png", 182)
            .as_deref(),
        Ok("texture-a")
    );
    assert!(
        phase_three_texture_member_id(&package, "texture/glow.png", 9).is_err()
    );
    assert!(
        phase_three_texture_member_id(
            &package,
            "texture/glow__ordinal_9.png",
            182,
        )
        .is_err()
    );
    Ok(())
}

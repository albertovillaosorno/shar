// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Mission camera level-scoped reference unit regressions.
// - Must-Not:
//   - Infer cross-level precedence or camera runtime behavior.
// - Allows:
//   - Synthetic decoded names and package/member provenance.
// - Split-When:
//   - Catalog and semantic-report tests require independent fixtures.
// - Merge-When:
//   - Mission camera binding loses independent behavior.
// - Summary:
//   - Mission camera reference tests.
// - Description:
//   - Proves exact level/name binding and duplicate rejection.
// - Usage:
//   - Included only by the mission camera reference domain module.
// - Defaults:
//   - Wrong-level references fail closed.
//

use super::*;

fn entry(
    level: &str,
    name: &str,
    kind: MissionCameraComponentKind,
    suffix: &str,
) -> Result<MissionCameraCatalogEntry, String> {
    MissionCameraCatalogEntry::new(
        name.to_owned(),
        kind,
        format!("member-{suffix}"),
        format!("package-{suffix}"),
        format!("extracted/art/missions/{level}/{suffix}"),
        format!(
            "extracted/art/missions/{level}/{suffix}/components/source.json"
        ),
    )
}

#[test]
fn same_name_in_two_levels_remains_distinct() -> Result<(), String> {
    let catalog = MissionCameraCatalog::from_entries(vec![
        entry(
            "level01",
            "mission2camShape",
            MissionCameraComponentKind::Camera,
            "mission2cam",
        )?,
        entry(
            "level02",
            "mission2camShape",
            MissionCameraComponentKind::Camera,
            "mission2cam",
        )?,
    ]);

    let first = catalog.resolve(
        "level01",
        MissionCameraComponentKind::Camera,
        "mission2camShape",
    )?;
    let second = catalog.resolve(
        "level02",
        MissionCameraComponentKind::Camera,
        "mission2camShape",
    )?;
    assert_eq!(
        first.package_root,
        "extracted/art/missions/level01/mission2cam"
    );
    assert_eq!(
        second.package_root,
        "extracted/art/missions/level02/mission2cam"
    );
    Ok(())
}

#[test]
fn duplicate_identity_inside_one_level_fails_closed() -> Result<(), String> {
    let duplicate = entry(
        "level01",
        "mission2cam",
        MissionCameraComponentKind::MultiController,
        "othercam",
    )?;
    let catalog = MissionCameraCatalog::from_entries(vec![
        entry(
            "level01",
            "mission2cam",
            MissionCameraComponentKind::MultiController,
            "mission2cam",
        )?,
        duplicate,
    ]);
    let Err(error) = catalog.resolve(
        "level01",
        MissionCameraComponentKind::MultiController,
        "mission2cam",
    ) else {
        return Err("ambiguous level identity was resolved".to_owned());
    };
    assert!(error.contains("ambiguous level-scoped"));
    Ok(())
}

#[test]
fn wrong_level_reference_does_not_fall_back_globally() -> Result<(), String> {
    let catalog = MissionCameraCatalog::from_entries(vec![entry(
        "level01",
        "mission2camShape",
        MissionCameraComponentKind::Camera,
        "mission2cam",
    )?]);
    assert!(
        catalog
            .resolve(
                "level02",
                MissionCameraComponentKind::Camera,
                "mission2camShape",
            )
            .is_err()
    );
    Ok(())
}

#[test]
fn source_script_level_is_exact_portable_provenance() -> Result<(), String> {
    assert_eq!(
        mission_level_from_script_path(
            "extracted/game/scripts/missions/level07/m4i.mfk.json"
        )?,
        "level07"
    );
    assert!(
        mission_level_from_script_path(
            "extracted/game/scripts/missions/shared/m4i.mfk.json"
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn push_binding_preserves_exact_member_and_package_provenance()
-> Result<(), String> {
    let catalog = MissionCameraCatalog::from_entries(vec![entry(
        "level01",
        "mission2cam",
        MissionCameraComponentKind::MultiController,
        "mission2cam",
    )?]);
    let mut bindings = Vec::new();
    push_binding(
        &mut bindings,
        &catalog,
        "level01",
        41,
        MissionCameraReferenceRole::MissionStartMulticont,
        MissionCameraComponentKind::MultiController,
        "mission2cam",
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("camera binding count changed".to_owned());
    };
    assert_eq!(binding.source_ordinal, 41);
    assert_eq!(binding.source_name, "mission2cam");
    assert_eq!(binding.member_id, "member-mission2cam");
    assert_eq!(binding.package_id, "package-mission2cam");
    assert_eq!(
        binding.package_root,
        "extracted/art/missions/level01/mission2cam"
    );
    Ok(())
}

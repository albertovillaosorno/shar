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
//   - Mission locator package-reference catalog unit regressions.
// - Must-Not:
//   - Read extracted files or invent package precedence.
// - Allows:
//   - Synthetic decoded locator and package-context evidence.
// - Split-When:
//   - Load-context compilation gains independent locator policy.
// - Merge-When:
//   - Mission locator resolution loses independent domain policy.
// - Summary:
//   - Mission locator catalog tests.
// - Description:
//   - Proves exact package-scoped resolution and ambiguity preservation.
// - Usage:
//   - Included only by the mission-locator domain module under cfg(test).
// - Defaults:
//   - Missing and ambiguous identities never become guessed references.
//

use super::*;

fn entry(
    package_id: &str,
    package_root: &str,
    source_name: &str,
    locator_type: u32,
) -> Result<MissionLocatorCatalogEntry, String> {
    MissionLocatorCatalogEntry::new(
        source_name.to_owned(),
        locator_type,
        if locator_type == 3 {
            "car_start".to_owned()
        } else {
            "event".to_owned()
        },
        format!("locator-{package_id}-{source_name}"),
        package_id.to_owned(),
        package_root.to_owned(),
        format!("{package_root}/components/srr_locator/{source_name}.json"),
    )
}

#[test]
fn resolves_exact_name_inside_explicit_package_context() -> Result<(), String> {
    let catalog = MissionLocatorCatalog::from_entries(vec![
        entry(
            "extracted-art-missions-level01-bm1",
            "extracted/art/missions/level01/bm1",
            "l1_tommaco",
            0,
        )?,
        entry(
            "extracted-art-missions-level01-level",
            "extracted/art/missions/level01/level",
            "bm_cletushouse",
            0,
        )?,
    ])?;
    let active = vec![r"EXTRACTED\ART\MISSIONS\LEVEL01\BM1".to_owned()];
    let resolved = catalog.resolve(
        "l1_tommaco",
        &active,
        MissionLocatorTypeConstraint::Exact(0),
    )?;
    let MissionLocatorResolution::Resolved(reference) = resolved else {
        return Err("locator did not resolve uniquely".to_owned());
    };
    assert_eq!(reference.entry().source_name(), "l1_tommaco");
    assert_eq!(reference.entry().locator_type(), 0);
    assert_eq!(
        reference.entry().package_id(),
        "extracted-art-missions-level01-bm1"
    );
    Ok(())
}

#[test]
fn preserves_ambiguity_across_active_packages() -> Result<(), String> {
    let source_name = "bm1_bestside";
    let catalog = MissionLocatorCatalog::from_entries(vec![
        entry(
            "extracted-art-missions-level01-bm1",
            "extracted/art/missions/level01/bm1",
            source_name,
            3,
        )?,
        entry(
            "extracted-art-missions-level01-level",
            "extracted/art/missions/level01/level",
            source_name,
            3,
        )?,
    ])?;
    let active = vec![
        "extracted/art/missions/level01/level".to_owned(),
        "extracted/art/missions/level01/bm1".to_owned(),
    ];
    let resolution =
        catalog.resolve(source_name, &active, MissionLocatorTypeConstraint::Exact(3))?;
    let MissionLocatorResolution::Ambiguous(candidates) = resolution else {
        return Err("duplicate active source name did not remain ambiguous".to_owned());
    };
    let [first, second] = candidates.as_slice() else {
        return Err("ambiguous locator candidate count drifted".to_owned());
    };
    assert_eq!(first.package_id(), "extracted-art-missions-level01-bm1");
    assert_eq!(second.package_id(), "extracted-art-missions-level01-level");
    Ok(())
}

#[test]
fn type_constraint_filters_without_precedence() -> Result<(), String> {
    let source_name = "shared_name";
    let catalog = MissionLocatorCatalog::from_entries(vec![
        entry(
            "extracted-art-missions-level01-bm1",
            "extracted/art/missions/level01/bm1",
            source_name,
            0,
        )?,
        entry(
            "extracted-art-missions-level01-level",
            "extracted/art/missions/level01/level",
            source_name,
            3,
        )?,
    ])?;
    let active = vec![
        "extracted/art/missions/level01/level".to_owned(),
        "extracted/art/missions/level01/bm1".to_owned(),
    ];
    let resolution =
        catalog.resolve(source_name, &active, MissionLocatorTypeConstraint::Exact(0))?;
    let MissionLocatorResolution::Resolved(reference) = resolution else {
        return Err("exact locator type did not leave one candidate".to_owned());
    };
    assert_eq!(reference.entry().locator_type_name(), "event");
    Ok(())
}

#[test]
fn missing_or_inactive_name_stays_missing() -> Result<(), String> {
    let catalog = MissionLocatorCatalog::from_entries(vec![entry(
        "extracted-art-missions-level01-bm1",
        "extracted/art/missions/level01/bm1",
        "l1_tommaco",
        0,
    )?])?;
    let inactive = vec!["extracted/art/missions/level01/level".to_owned()];
    for name in ["l1_tommaco", "not_authored"] {
        assert_eq!(
            catalog.resolve(name, &inactive, MissionLocatorTypeConstraint::Any)?,
            MissionLocatorResolution::Missing
        );
    }
    Ok(())
}

#[test]
fn rejects_duplicate_decoded_name_inside_one_package() -> Result<(), String> {
    let package_id = "extracted-art-missions-level01-bm1";
    let package_root = "extracted/art/missions/level01/bm1";
    let first = entry(package_id, package_root, "duplicate", 0)?;
    let mut second = entry(package_id, package_root, "duplicate", 3)?;
    second.member_id = "locator-other".to_owned();
    second.member_path = format!("{package_root}/components/srr_locator/duplicate2.json");
    let Err(error) = MissionLocatorCatalog::from_entries(vec![first, second]) else {
        return Err("package-local duplicate did not fail closed".to_owned());
    };
    assert_eq!(
        error,
        "mission locator name is duplicated inside one package"
    );
    Ok(())
}

#[test]
fn rejects_locator_member_path_traversal() -> Result<(), String> {
    let package_root = "extracted/art/missions/level01/bm1";
    let Err(error) = MissionLocatorCatalogEntry::new(
        "unsafe".to_owned(),
        0,
        "event".to_owned(),
        "locator-unsafe".to_owned(),
        "extracted-art-missions-level01-bm1".to_owned(),
        package_root.to_owned(),
        format!("{package_root}/components/srr_locator/../unsafe.json"),
    ) else {
        return Err("locator member traversal did not fail closed".to_owned());
    };
    assert_eq!(error, "locator member path is malformed");
    Ok(())
}

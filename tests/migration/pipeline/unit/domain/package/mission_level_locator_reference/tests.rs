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
//   - Unit evidence for level-init locator lookup constraints.
// - Must-Not:
//   - Add runtime lifecycle policy beyond source-backed lookup timing.
// - Allows:
//   - Verify exact CarStart ordering and conservative generic ambiguity.
// - Split-When:
//   - Integration fixtures require independent setup reports.
// - Merge-When:
//   - The level locator domain boundary disappears.
// - Summary:
//   - Level locator reference unit tests.
// - Description:
//   - Locks exact-vs-generic package lookup behavior.
// - Usage:
//   - Compiled as a child of mission_level_locator_reference.
// - Defaults:
//   - Unsupported precedence remains ambiguous.
//

use super::*;
use crate::domain::MissionLocatorCatalogEntry;

fn entry(
    source_name: &str,
    locator_type: u32,
    package_id: &str,
    package_root: &str,
) -> Result<MissionLocatorCatalogEntry, String> {
    MissionLocatorCatalogEntry::new(
        source_name.to_owned(),
        locator_type,
        format!("type-{locator_type}"),
        format!("{package_id}-locator"),
        package_id.to_owned(),
        package_root.to_owned(),
        format!(
            "{package_root}/components/srr_locator/{package_id}-locator.json"
        ),
    )
}

fn catalog() -> Result<MissionLocatorCatalog, String> {
    MissionLocatorCatalog::from_entries(vec![
        entry("shared", 3, "level-a", "extracted/art/level-a")?,
        entry("shared", 3, "level-b", "extracted/art/level-b")?,
        entry("generic", 3, "level-a", "extracted/art/level-a")?,
        entry("generic", 9, "level-b", "extracted/art/level-b")?,
    ])
}

#[test]
fn exact_dialogue_lookup_uses_authored_package_order() -> Result<(), String> {
    let catalog = catalog()?;
    let mut bindings = Vec::new();
    push_reference(
        &mut bindings,
        &catalog,
        &[
            "extracted/art/level-b".to_owned(),
            "extracted/art/level-a".to_owned(),
        ],
        7,
        MissionLevelLocatorRole::BonusDialoguePlayer,
        "shared",
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("exact level locator binding count changed".to_owned());
    };
    let MissionLocatorResolution::Resolved(resolved) = binding.resolution()
    else {
        return Err("exact CarStart lookup did not resolve".to_owned());
    };
    assert_eq!(resolved.entry().package_id(), "level-b");
    Ok(())
}

#[test]
fn generic_lookup_preserves_cross_package_ambiguity() -> Result<(), String> {
    let catalog = catalog()?;
    let mut bindings = Vec::new();
    push_reference(
        &mut bindings,
        &catalog,
        &[
            "extracted/art/level-a".to_owned(),
            "extracted/art/level-b".to_owned(),
        ],
        8,
        MissionLevelLocatorRole::AmbientSpawn,
        "generic",
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("generic level locator binding count changed".to_owned());
    };
    let MissionLocatorResolution::Ambiguous(candidates) = binding.resolution()
    else {
        return Err("generic locator precedence was inferred".to_owned());
    };
    assert_eq!(candidates.len(), 2);
    Ok(())
}

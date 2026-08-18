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
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::super::transform::identity;
use super::{
    LevelMeshSource, WorldObjectRole, explicit_placements,
    is_direct_world_mesh, object_role, package_meshes,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn ledger_root(label: &str, rows: &[&str]) -> Result<PathBuf, String> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "shar-world-inventory-{label}-{}-{sequence}",
        std::process::id()
    ));
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let mut contents = rows.join("\n");
    contents.push('\n');
    fs::write(root.join("components.jsonl"), contents)
        .map_err(|error| error.to_string())?;
    Ok(root)
}

fn cleanup(root: &PathBuf) {
    drop(fs::remove_dir_all(root));
}

fn owner_row(ordinal: usize, name: &str) -> String {
    format!(
        r#"{{"ordinal":{ordinal},"depth":1,"container_ordinal":{ordinal},"name":"{name}","path":"components/srr_entity_dsg/{ordinal:03}.json","kind":"srr_entity_dsg"}}"#
    )
}

fn mesh_row(ordinal: usize, owner: usize, name: &str) -> String {
    format!(
        r#"{{"ordinal":{ordinal},"depth":2,"container_ordinal":{owner},"name":"{name}","path":"components/mesh/{ordinal:03}.json","kind":"mesh"}}"#
    )
}

fn source(kind: &str) -> LevelMeshSource {
    LevelMeshSource {
        ordinal: 1,
        member_id: "house".to_owned(),
        mesh_name: "house".to_owned(),
        owner_name: "house-owner".to_owned(),
        owner_kind: kind.to_owned(),
    }
}

#[test]
fn explicit_placement_prefers_mesh_identity() {
    let mut placements = BTreeMap::new();
    let _previous = placements.insert("house".to_owned(), vec![identity()]);
    assert_eq!(
        explicit_placements(&source("srr_insta_entity_dsg"), &placements,)
            .len(),
        1
    );
}

#[test]
fn direct_entities_are_classified_without_invented_matrix() {
    assert!(is_direct_world_mesh(&source("srr_entity_dsg")));
    assert!(is_direct_world_mesh(&source("srr_static_phys_dsg")));
    assert!(
        explicit_placements(&source("srr_static_phys_dsg"), &BTreeMap::new(),)
            .is_empty()
    );
}

#[test]
fn definition_only_meshes_are_not_direct_world_geometry() {
    assert!(!is_direct_world_mesh(&source("srr_breakable_object")));
}

#[test]
fn source_owner_kinds_preserve_world_interaction_roles() {
    assert_eq!(
        object_role(&source("srr_tree_dsg")),
        WorldObjectRole::Breakable
    );
    assert_eq!(
        object_role(&source("srr_breakable_object")),
        WorldObjectRole::Breakable
    );
    let mut named_tree = source("srr_dyna_phys_dsg");
    named_tree.mesh_name = "l1_treesm_shape".to_owned();
    assert_eq!(object_role(&named_tree), WorldObjectRole::Interactable);
    assert_eq!(
        object_role(&source("srr_static_phys_dsg")),
        WorldObjectRole::Interactable
    );
    assert_eq!(
        object_role(&source("srr_insta_anim_dyna_phys_dsg")),
        WorldObjectRole::Interactable
    );
    assert_eq!(
        object_role(&source("srr_entity_dsg")),
        WorldObjectRole::Static
    );
}


#[test]
fn package_meshes_preserve_source_ordinal_order() -> Result<(), String> {
    let rows = [
        owner_row(10, "z-owner"),
        mesh_row(11, 10, "z-mesh"),
        owner_row(20, "a-owner"),
        mesh_row(21, 20, "a-mesh"),
    ];
    let borrowed = rows.iter().map(String::as_str).collect::<Vec<_>>();
    let root = ledger_root("source-order", &borrowed)?;
    let result = package_meshes(&root).map_err(|error| error.to_string());
    cleanup(&root);
    let meshes = result?;
    let ordinals = meshes.iter().map(|mesh| mesh.ordinal).collect::<Vec<_>>();
    if ordinals != [11, 21] {
        return Err(format!("source mesh order changed: {ordinals:?}"));
    }
    Ok(())
}

#[test]
fn package_meshes_reject_duplicate_component_ordinals() -> Result<(), String> {
    let rows = [
        owner_row(1, "owner"),
        mesh_row(2, 1, "first"),
        mesh_row(2, 1, "second"),
    ];
    let borrowed = rows.iter().map(String::as_str).collect::<Vec<_>>();
    let root = ledger_root("duplicate-ordinal", &borrowed)?;
    let result = package_meshes(&root);
    cleanup(&root);
    let Err(error) = result else {
        return Err("duplicate component ordinal was accepted".to_owned());
    };
    if !error.to_string().contains("repeats component ordinal 2") {
        return Err(format!("unexpected duplicate ordinal error: {error}"));
    }
    Ok(())
}

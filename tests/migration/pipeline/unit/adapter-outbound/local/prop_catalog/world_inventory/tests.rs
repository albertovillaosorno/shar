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
//   - World inventory unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - World inventory unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! World inventory unit tests.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{
    LedgerRow, decoded_mesh_names, deferred_render_bindings,
    source_ordered_mesh_ids,
};
use crate::adapters::driven::local::prop_catalog::inventory_common::{
    CompositeEvidence, CompositePropEvidence,
};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn fixture_root(label: &str) -> Result<PathBuf, String> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "shar-world-inventory-map-{label}-{}-{sequence}",
        std::process::id()
    ));
    fs::create_dir_all(root.join("components/mesh"))
        .map_err(|error| error.to_string())?;
    Ok(root)
}

#[test]
fn decoded_mesh_names_reject_duplicate_source_identity() -> Result<(), String> {
    let root = fixture_root("duplicate-name")?;
    fs::write(root.join("components/mesh/001.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    fs::write(root.join("components/mesh/002.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    // jig-ignore-next-line: literal
    let result = decoded_mesh_names(&root, &["001".to_owned(), "002".to_owned()]);
    drop(fs::remove_dir_all(&root));
    let Err(error) = result else {
        return Err("duplicate decoded mesh identity was accepted".to_owned());
    };
    if !error
        .to_string()
        .contains("world prop repeats mesh identity shared")
    {
        // jig-ignore-next-line: literal
        return Err(format!("unexpected duplicate mesh identity error: {error}"));
    }
    Ok(())
}

#[test]
fn source_mesh_ids_follow_component_ordinals() -> Result<(), String> {
    let rows = vec![
        LedgerRow {
            ordinal: 20,
            depth: 2,
            container_ordinal: 1,
            name: "lexically-first".to_owned(),
            path: "mesh/a.json".to_owned(),
            kind: "mesh".to_owned(),
        },
        LedgerRow {
            ordinal: 10,
            depth: 2,
            container_ordinal: 1,
            name: "source-first".to_owned(),
            path: "mesh/z.json".to_owned(),
            kind: "mesh".to_owned(),
        },
    ];
    let actual = source_ordered_mesh_ids(&rows)
        .map_err(|error| error.to_string())?;
    assert_eq!(actual, ["z", "a"]);
    Ok(())
}

#[test]
fn deferred_bindings_resolve_unique_package_quad_group_occurrence()
-> Result<(), String> {
    let composite = CompositeEvidence {
        member_id: "composite".to_owned(),
        name: "owner".to_owned(),
        skeleton_name: "rig".to_owned(),
        prop_names: vec![
            "body".to_owned(),
            "beam".to_owned(),
            "logical-only".to_owned(),
        ],
        prop_bindings: vec![
            CompositePropEvidence {
                name: "body".to_owned(),
                skeleton_joint_id: 0,
                is_translucent: false,
            },
            CompositePropEvidence {
                name: "beam".to_owned(),
                skeleton_joint_id: 3,
                is_translucent: true,
            },
            CompositePropEvidence {
                name: "logical-only".to_owned(),
                skeleton_joint_id: 4,
                is_translucent: false,
            },
        ],
    };
    let rows = Vec::new();
    let package_quad_groups = vec![LedgerRow {
        ordinal: 22,
        depth: 1,
        container_ordinal: 22,
        name: "beam\x00".to_owned(),
        path: "quad_group/beam__ordinal_22.json".to_owned(),
        kind: "quad_group".to_owned(),
    }];
    let meshes = BTreeMap::from([(
        "body".to_owned(),
        "body__ordinal_10".to_owned(),
    )]);

    let actual = deferred_render_bindings(
        &rows,
        &package_quad_groups,
        &composite,
        &meshes,
    )
        .map_err(|error| error.to_string())?;
    if actual.len() != 2 {
        return Err(format!("unexpected deferred binding count: {actual:?}"));
    }
    let beam = actual
        .first()
        .ok_or_else(|| "resolved deferred binding is missing".to_owned())?;
    if beam.composite_prop_index != 1
        || beam.source_identity != "beam"
        || beam.skeleton_joint_id != 3
        || !beam.is_translucent
        || beam.component_kind.as_deref() != Some("quad_group")
        || beam.component_member_id.as_deref() != Some("beam__ordinal_22")
        || beam.source_ordinal != Some(22)
    {
        return Err(format!("resolved deferred binding changed: {beam:?}"));
    }
    let logical = actual
        .get(1)
        .ok_or_else(|| "logical deferred binding is missing".to_owned())?;
    if logical.composite_prop_index != 2
        || logical.source_identity != "logical-only"
        || logical.skeleton_joint_id != 4
        || logical.is_translucent
        || logical.component_kind.is_some()
        || logical.component_member_id.is_some()
        || logical.source_ordinal.is_some()
    {
        return Err(format!("logical deferred binding changed: {logical:?}"));
    }
    Ok(())
}

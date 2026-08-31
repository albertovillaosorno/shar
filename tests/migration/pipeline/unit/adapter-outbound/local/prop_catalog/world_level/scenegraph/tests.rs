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
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::json;

use super::{collect_scenegraphs, placement_map};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn fixture_root(label: &str) -> PathBuf {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "shar-scenegraph-{label}-{}-{sequence}",
        std::process::id()
    ))
}

fn scenegraph_with_translations(translations: &[i32]) -> serde_json::Value {
    let roots = translations
        .iter()
        .map(|translation| {
            json!({
                "kind": "transform",
                "matrix": [
                    1_i32, 0_i32, 0_i32, 0_i32,
                    0_i32, 1_i32, 0_i32, 0_i32,
                    0_i32, 0_i32, 1_i32, 0_i32,
                    translation, 0_i32, 0_i32, 1_i32
                ],
                "children": [{
                    "kind": "drawable",
                    "drawable_name": "house"
                }]
            })
        })
        .collect::<Vec<_>>();
    json!({"schema": "scenegraph", "roots": roots})
}

fn write_scenegraph_fixture(
    label: &str,
    translations: &[i32],
) -> Result<PathBuf, String> {
    let root = fixture_root(label);
    let directory = root.join("components/scenegraph");
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
        // jig-ignore-next-line: expression
        let document = serde_json::to_vec(&scenegraph_with_translations(translations))
        .map_err(|error| error.to_string())?;
    fs::write(directory.join("000.json"), document)
        .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components.jsonl"),
        concat!(
            r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"#,
            r#""container_ordinal":1,"name":"Scene","#,
            r#""path":"scenegraph/000.json","kind":"scenegraph"}"#,
            "\n"
        ),
    )
    .map_err(|error| error.to_string())?;
    Ok(root)
}

fn cleanup(root: &PathBuf) {
    drop(fs::remove_dir_all(root));
}

#[test]
fn nested_transform_places_one_drawable() -> Result<(), String> {
    let value = json!({
        "schema": "scenegraph",
        "roots": [{
            "kind": "transform",
            "matrix": [
                1_i32, 0_i32, 0_i32, 0_i32,
                0_i32, 1_i32, 0_i32, 0_i32,
                0_i32, 0_i32, 1_i32, 0_i32,
                4_i32, 5_i32, 6_i32, 1_i32
            ],
            "children": [{
                "kind": "drawable",
                "drawable_name": "house"
            }]
        }]
    });
    let mut placements = BTreeMap::new();
    collect_scenegraphs(&value, &mut placements)
        .map_err(|error| error.to_string())?;
    let [matrix] = placements
        .get("house")
        .map(Vec::as_slice)
        .ok_or_else(|| "house placement is missing".to_owned())?
    else {
        return Err("house placement count is not one".to_owned());
    };
    if matrix[12..15] != [4., 5., 6.] {
        return Err("house translation was not preserved".to_owned());
    }
    Ok(())
}


#[test]
// jig-ignore-next-line: long identifier
fn placement_map_preserves_duplicate_authored_placements() -> Result<(), String> {
    let root = write_scenegraph_fixture("duplicates", &[4, 4])?;
    let result = placement_map(&root).map_err(|error| error.to_string());
    cleanup(&root);
    let placements = result?;
    let matrices = placements
        .get("house")
        .ok_or_else(|| "duplicate house placements are missing".to_owned())?;
    if matrices.len() != 2 {
        return Err(format!(
            "duplicate authored placement count changed: {}",
            matrices.len()
        ));
    }
    Ok(())
}

#[test]
fn placement_map_preserves_authored_placement_order() -> Result<(), String> {
    let root = write_scenegraph_fixture("order", &[9, 1])?;
    let result = placement_map(&root).map_err(|error| error.to_string());
    cleanup(&root);
    let placements = result?;
    let matrices = placements
        .get("house")
        .ok_or_else(|| "ordered house placements are missing".to_owned())?;
    let translations = matrices
        .iter()
        .map(|matrix| matrix[12])
        .collect::<Vec<_>>();
    if translations != [9., 1.] {
        return Err(format!(
            "authored placement order changed: {translations:?}"
        ));
    }
    Ok(())
}

#[test]
fn placement_map_orders_documents_by_source_ordinal() -> Result<(), String> {
    let root = fixture_root("document-order");
    let scenegraph_dir = root.join("components/scenegraph");
    let instance_dir = root.join("components/srr_insta_static_phys_dsg");
    fs::create_dir_all(&scenegraph_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&instance_dir).map_err(|error| error.to_string())?;
    let later = serde_json::to_vec(&scenegraph_with_translations(&[20]))
        .map_err(|error| error.to_string())?;
    let earlier = serde_json::to_vec(&scenegraph_with_translations(&[10]))
        .map_err(|error| error.to_string())?;
    fs::write(scenegraph_dir.join("later.json"), later)
        .map_err(|error| error.to_string())?;
    fs::write(instance_dir.join("earlier.json"), earlier)
        .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components.jsonl"),
        concat!(
            r#"{"ordinal":10,"depth":1,"parent_ordinal":0,"#,
            r#""container_ordinal":10,"name":"Earlier","#,
            r#""path":"srr_insta_static_phys_dsg/earlier.json","#,
            r#""kind":"srr_insta_static_phys_dsg"}"#,
            "\n",
            r#"{"ordinal":20,"depth":1,"parent_ordinal":0,"#,
            r#""container_ordinal":20,"name":"Later","#,
            r#""path":"scenegraph/later.json","kind":"scenegraph"}"#,
            "\n"
        ),
    )
    .map_err(|error| error.to_string())?;
    let result = placement_map(&root).map_err(|error| error.to_string());
    cleanup(&root);
    let placements = result?;
    let translations = placements
        .get("house")
        .ok_or_else(|| {
            "cross-document house placements are missing".to_owned()
        })?
        .iter()
        .map(|matrix| matrix[12])
        .collect::<Vec<_>>();
    if translations != [10., 20.] {
        return Err(format!(
            "scenegraph document source order changed: {translations:?}"
        ));
    }
    Ok(())
}

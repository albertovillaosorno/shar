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
//   - Composite transparency tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Composite transparency tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Composite transparency tests unit tests.

use std::fs;
use std::path::PathBuf;

use super::{composite_bindings, mark_transparent_mesh, source_skin_bindings};
use crate::domain::mesh::{MeshAsset, PrimitiveGroup};

fn group(index: usize, shader: &str) -> Result<PrimitiveGroup, String> {
    PrimitiveGroup::new(
        index,
        shader,
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("synthetic primitive group failed: {error:?}"))
}

#[test]
fn composite_transparency_marks_only_single_group_meshes() -> Result<(), String>
{
    let mut isolated = MeshAsset::new("window", vec![group(0, "window_m")?])
        .map_err(|error| format!("single-group fixture failed: {error:?}"))?;
    mark_transparent_mesh(&mut isolated);
    if isolated.name != "window__transparent-source" {
        return Err(format!(
            "single-group transparency marker changed: {}",
            isolated.name
        ));
    }

    let mut mixed = MeshAsset::new("vehicle-body", vec![
        group(0, "body_m")?,
        group(1, "windsheild_m")?,
    ])
    .map_err(|error| format!("multi-group fixture failed: {error:?}"))?;
    mark_transparent_mesh(&mut mixed);
    if mixed.name != "vehicle-body" {
        return Err(format!(
            "multi-group transparency marker changed: {}",
            mixed.name
        ));
    }
    Ok(())
}

fn temp_composite(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fbx-composite-skin-{label}-{}.json",
        std::process::id()
    ))
}

fn composite_skin_fixture(sort: &str, kind: &str) -> String {
    format!(
        concat!(
            r#"{{"schema":"composite_drawable","name":"character","#,
            r#""skeleton_name":"rig","num_skins":1,"skins":[{{"#,
            r#""kind":"{}","name":"SkinShape","is_translucent":1{}"#,
            r#"}}],"num_props":0,"props":[],"num_effects":0,"effects":[]}}"#,
        ),
        kind, sort
    )
}

#[test]
fn composite_skin_provenance_retains_index_translucency_and_sort()
-> Result<(), String> {
    let path = temp_composite("provenance");
    fs::write(&path, composite_skin_fixture(",\"sort_order\":0.1", "skin"))
        .map_err(|error| error.to_string())?;
    let bindings =
        composite_bindings(&path, "rig", &["SkinShape".to_owned()], 1).map_err(
            |error| format!("composite skin decode failed: {error:?}"),
        );
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let bindings = bindings?;
    let source = source_skin_bindings(2, &bindings.skins)
        .map_err(|error| format!("skin provenance failed: {error:?}"))?;
    let [binding] = source.as_slice() else {
        return Err(format!("unexpected skin source bindings: {source:?}"));
    };
    if binding.composite_ordinal() != 2
        || binding.skin_index() != 0
        || binding.skin_identity() != "SkinShape"
        || !binding.translucent()
        || binding.sort_order_bits() != Some(0.1_f32.to_bits())
    {
        return Err(format!("skin provenance changed: {binding:?}"));
    }
    Ok(())
}

#[test]
fn composite_skin_provenance_preserves_missing_sort() -> Result<(), String> {
    let path = temp_composite("missing-sort");
    fs::write(&path, composite_skin_fixture("", "skin"))
        .map_err(|error| error.to_string())?;
    let bindings =
        composite_bindings(&path, "rig", &["SkinShape".to_owned()], 1).map_err(
            |error| format!("missing-sort skin decode failed: {error:?}"),
        );
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let bindings = bindings?;
    let source = source_skin_bindings(0, &bindings.skins).map_err(|error| {
        format!("missing-sort provenance failed: {error:?}")
    })?;
    let binding = source
        .first()
        .ok_or_else(|| "missing-sort skin produced no provenance".to_owned())?;
    if binding.sort_order_bits().is_some() {
        return Err("missing skin sort became authored evidence".to_owned());
    }
    Ok(())
}

#[test]
fn composite_skin_provenance_rejects_null_sort_and_wrong_kind()
-> Result<(), String> {
    for (label, sort, kind) in [
        ("null-sort", ",\"sort_order\":null", "skin"),
        ("wrong-kind", ",\"sort_order\":0.5", "prop"),
    ] {
        let path = temp_composite(label);
        fs::write(&path, composite_skin_fixture(sort, kind))
            .map_err(|error| error.to_string())?;
        let result =
            composite_bindings(&path, "rig", &["SkinShape".to_owned()], 1);
        fs::remove_file(&path).map_err(|error| error.to_string())?;
        if result.is_ok() {
            return Err(format!(
                "invalid composite skin was accepted: {label}"
            ));
        }
    }
    Ok(())
}

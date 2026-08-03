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

use super::mark_transparent_mesh;
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

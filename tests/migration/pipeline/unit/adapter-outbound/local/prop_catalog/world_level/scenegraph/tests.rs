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

use serde_json::json;

use super::collect_scenegraphs;

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

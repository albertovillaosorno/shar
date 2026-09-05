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
//   - World catalog raster-contract regression tests.
// - Must-Not:
//   - Mutate source or generated world content.
// - Allows:
//   - Pure catalog rendering assertions.
// - Split-When:
//   - Another catalog policy gains independent lifecycle.
// - Merge-When:
//   - The world catalog no longer owns rasterization evidence.
// - Summary:
//   - Locks source-backed world culling evidence into the catalog.
// - Description:
//   - Verifies the regular scene records Pure3D CullNone without changing
//     topology or treating special-purpose render passes as equivalent.
// - Usage:
//   - Runs with pipeline unit tests.
// - Defaults:
//   - Any drift from the source-backed cull contract fails closed.
//

//! World catalog raster-contract regression tests.

use super::regular_scene_raster_value;

#[test]
fn regular_world_raster_preserves_source_cull_none() {
    let raster = regular_scene_raster_value();
    assert_eq!(
        raster.get("source_cull_mode"),
        Some(&serde_json::json!("none"))
    );
    assert_eq!(
        raster.get("native_material_requirement"),
        Some(&serde_json::json!(concat!(
            "regular world material families render both polygon faces; ",
            "shadow and special-purpose passes retain their own cull policy"
        )))
    );
}

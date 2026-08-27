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
//   - Metadata-editor component classification unit tests.
// - Must-Not:
//   - Own production behavior or filesystem integration.
// - Allows:
//   - Private assertions for the owning editor module.
// - Split-When:
//   - Component classification gains an independent test lifecycle.
// - Merge-When:
//   - Another test module owns the identical private classification boundary.
// - Summary:
//   - Metadata-editor component classification tests.
// - Description:
//   - Pins generated P3D component metadata used by phase-three planning.
// - Usage:
//   - Included only by the owning editor module under cfg(test).
// - Defaults:
//   - Assertions fail explicitly.
//

//! Metadata-editor component classification tests.

use super::component_bucket;

#[test]
fn embedded_image_component_routes_to_dds_texture_conversion() {
    let bucket = component_bucket("image");
    assert_eq!(bucket.type_, "image");
    assert_eq!(bucket.kind, "p3d-texture");
    assert_eq!(bucket.relation, "compose-into-asset");
    assert_eq!(bucket.future, "dds-to-texture2d");
}

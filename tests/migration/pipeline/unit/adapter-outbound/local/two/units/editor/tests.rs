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

#[test]
fn scrooby_layout_components_remain_composed_ui_evidence() {
    for segment in ["scrooby_screen", "scrooby_page", "scrooby_layer"] {
        let bucket = component_bucket(segment);
        assert_eq!(bucket.type_, "ui", "unexpected type for {segment}");
        assert_eq!(
            bucket.kind, "p3d-scrooby-layout",
            "unexpected kind for {segment}",
        );
        assert_eq!(bucket.relation, "compose-into-asset");
        assert_eq!(bucket.future, "p3d-scrooby-project-to-ui-project");
    }
}

#[test]
fn scrooby_resource_components_remain_composed_ui_evidence() {
    for segment in [
        "scrooby_image_resource",
        "scrooby_pure3d_resource",
        "scrooby_text_style_resource",
        "scrooby_text_bible_resource",
    ] {
        let bucket = component_bucket(segment);
        assert_eq!(bucket.type_, "ui", "unexpected type for {segment}");
        assert_eq!(
            bucket.kind, "p3d-scrooby-resource",
            "unexpected kind for {segment}",
        );
        assert_eq!(bucket.relation, "compose-into-asset");
        assert_eq!(bucket.future, "p3d-scrooby-project-to-ui-project");
    }
}

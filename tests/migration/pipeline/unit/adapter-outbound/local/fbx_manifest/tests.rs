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

use super::PhaseThreeFbxManifest;
use crate::domain::package::FbxModelPlan;

#[test]
fn renders_generic_fbx_manifest() -> Result<(), String> {
    let manifest = PhaseThreeFbxManifest::from_plan(&FbxModelPlan {
        package_id: "pkg".to_owned(),
        subcategory: "props/wrench".to_owned(),
        model_ids: vec!["model-a".to_owned()],
        world_ids: Vec::new(),
        scene_ids: Vec::new(),
        locator_ids: Vec::new(),
        camera_ids: Vec::new(),
        animation_ids: vec!["anim-a".to_owned()],
        texture_ids: vec!["texture-a".to_owned()],
        material_ids: vec!["material-a".to_owned()],
        physics_ids: Vec::new(),
    });
    let json = manifest.to_json();
    if !json.contains("\"output_fbx\": \"props-wrench.fbx\"") {
        return Err("manifest should expose stable FBX name".to_owned());
    }
    if !json.contains("\"model_ids\": [\"model-a\"]") {
        return Err("manifest should include model ids".to_owned());
    }
    Ok(())
}

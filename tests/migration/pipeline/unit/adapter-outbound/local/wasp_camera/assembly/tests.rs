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

use super::BODY_MESH_MEMBERS;

#[test]
fn body_selection_excludes_fx_and_explosion_meshes() {
    assert!(BODY_MESH_MEMBERS.contains(&"components/mesh/BodyShape.json"));
    assert!(BODY_MESH_MEMBERS.contains(&"components/mesh/wasp_armShape5.json"));
    assert!(
        !BODY_MESH_MEMBERS
            .contains(&"components/mesh/head_explosionShape.json")
    );
    assert!(!BODY_MESH_MEMBERS.contains(&"components/mesh/TailShape1.json"));
}

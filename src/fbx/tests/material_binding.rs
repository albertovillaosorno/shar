// File:
//   - material_binding.rs
// Path:
//   - src/fbx/tests/material_binding.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Regression coverage for normalized FBX material-binding invariants.
// - Must-Not:
//   - Access private assets, perform filesystem discovery, or copy textures.
// - Allows:
//   - Synthetic material and texture identities.
// - Split-When:
//   - Texture staging requires an independent adapter integration boundary.
// - Merge-When:
//   - Material-binding regressions no longer need a distinct test target.
// - Summary:
//   - Protects material identities before adapter staging and serialization.
// - Description:
//   - Exercises public material-binding construction with synthetic evidence.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Regression coverage for normalized FBX material-binding invariants.
//!
//! Synthetic identities verify that invalid material and texture names are
//! rejected before adapter staging or deterministic scene serialization.

use fbx::domain::texture::{MaterialBinding, MaterialBindingError};
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;

#[test]
fn rejects_blank_material_binding_identities() {
    assert_eq!(
        MaterialBinding::new(
            "   ", None
        ),
        Err(MaterialBindingError::MissingMaterialName)
    );
    assert_eq!(
        MaterialBinding::new(
            "material",
            Some("   ".to_owned())
        ),
        Err(MaterialBindingError::BlankTextureFileName)
    );
}

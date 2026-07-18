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
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

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

#[test]
fn classifies_shared_glass_mirror_and_light_identities() -> Result<(), String> {
    let glass = MaterialBinding::new(
        "windsheild_glass_m",
        Some("headlight_lens.png".to_owned()),
    )
    .map_err(|error| format!("glass material failed: {error:?}"))?;
    if !glass
        .semantics
        .is_transparent()
        || !glass
            .semantics
            .is_glass()
        || !glass
            .semantics
            .is_light_emitter()
        || glass
            .semantics
            .is_mirror()
    {
        return Err(format!("glass semantics were incomplete: {glass:?}"));
    }
    let mirror = MaterialBinding::new(
        "rearview_mirror_m",
        Some("rearview.png".to_owned()),
    )
    .map_err(|error| format!("mirror material failed: {error:?}"))?;
    if !mirror
        .semantics
        .is_mirror()
        || mirror
            .semantics
            .is_glass()
        || mirror
            .semantics
            .is_light_emitter()
    {
        return Err(format!("mirror semantics were incomplete: {mirror:?}"));
    }
    Ok(())
}

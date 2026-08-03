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

use fbx::adapters::driven::semantic_texture_png::{
    decode_png_bytes, encode_png_bytes,
};
use fbx::domain::texture::MaterialSemantics;
use fbx::domain::texture::semantic::{Rgba8, RgbaImage};

use super::{
    canonical_material_identity, corrected_texture_bytes,
    is_world_analysis_default_shader,
};

#[test]
fn recognizes_only_evidence_backed_neutral_defaults() {
    assert!(is_world_analysis_default_shader("lambert1"));
    assert!(is_world_analysis_default_shader("Pure3DSimpleShader15"));
    assert!(!is_world_analysis_default_shader("lambert"));
    assert!(!is_world_analysis_default_shader("pure3dSimpleShader14"));
    assert!(!is_world_analysis_default_shader("world_button_m"));
}

#[test]
fn canonical_material_identity_separates_surface_semantics() {
    let opaque = canonical_material_identity(
        Some("abc123"),
        MaterialSemantics::default(),
    );
    let glass = canonical_material_identity(
        Some("abc123"),
        MaterialSemantics::default().with_glass(true),
    );
    let emitter = canonical_material_identity(
        Some("abc123"),
        MaterialSemantics::default()
            .with_transparent(true)
            .with_light_emitter(true),
    );
    assert_eq!(opaque, "material-abc123");
    assert_eq!(glass, "material-abc123-glass");
    assert_eq!(emitter, "material-abc123-transparent-light-emitter");
    assert_ne!(opaque, glass);
    assert_ne!(glass, emitter);
}

#[test]
fn lard_lad_texture_is_complemented_without_changing_alpha()
-> Result<(), String> {
    let source = RgbaImage::new(1, 1, vec![Rgba8::new(10, 20, 30, 40)])
        .map_err(|error| format!("source image failed: {error:?}"))?;
    let encoded = encode_png_bytes(&source)
        .map_err(|error| format!("source PNG failed: {error:?}"))?;
    let corrected =
        corrected_texture_bytes("lard_lad_m__", "lard_lad.png", encoded)
            .map_err(|error| error.to_string())?;
    let decoded = decode_png_bytes(&corrected)
        .map_err(|error| format!("corrected PNG failed: {error:?}"))?;
    let expected = [Rgba8::new(245, 235, 225, 40)];
    if decoded.pixels() != expected {
        return Err(format!(
            "unexpected corrected Lard Lad pixel: {:?}",
            decoded.pixels()
        ));
    }
    Ok(())
}

#[test]
fn texture_complement_recipe_is_exact_identity_only() -> Result<(), String> {
    let bytes = vec![1_u8, 2, 3];
    let unchanged =
        corrected_texture_bytes("other_m", "lard_lad.png", bytes.clone())
            .map_err(|error| error.to_string())?;
    if unchanged != bytes {
        return Err("nonmatching texture bytes changed".to_owned());
    }
    Ok(())
}

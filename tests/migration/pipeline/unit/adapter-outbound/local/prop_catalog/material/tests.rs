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

use fbx::domain::texture::MaterialSemantics;

use super::{
    canonical_material_identity, is_world_analysis_default_shader,
    prepare_source_texture,
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
fn prepared_texture_preserves_exact_source_bytes() -> Result<(), String> {
    let bytes = vec![10_u8, 20, 30, 40];
    let prepared = prepare_source_texture(bytes.clone());
    if prepared.bytes != bytes {
        return Err("source texture bytes were rewritten".to_owned());
    }
    let expected_digest = shar_sha256::digest_hex(&bytes);
    if prepared.sha256 != expected_digest
        || prepared.file_name != format!("texture-{expected_digest}.png")
    {
        return Err("source texture content identity changed".to_owned());
    }
    Ok(())
}

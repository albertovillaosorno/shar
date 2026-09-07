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

use super::{
    material_binding_value, material_slot_value, regular_scene_raster_value,
};

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

#[test]
fn world_material_binding_preserves_texture_tint_and_semantics()
-> Result<(), String> {
    use fbx::adapters::driven::decoded_component_source::{
        ShaderParameterEvidence, ShaderSourceEvidence,
    };
    use fbx::domain::texture::{MaterialBinding, MaterialSemantics};
    use super::super::super::material::{
        CanonicalMaterialPresentation, CanonicalShaderPresentationSource,
    };

    let binding = MaterialBinding::new(
        "material-test-transparent",
        Some("texture-test.png".to_owned()),
    )
    .map_err(|error| format!("fixture material binding failed: {error:?}"))?
    .with_base_color_rgba8([12, 34, 56, 78])
    .with_semantics(
        MaterialSemantics::default()
            .with_transparent(true)
            .with_light_emitter(true),
    );
    let presentation = CanonicalMaterialPresentation {
        material_name: binding.material_name.clone(),
        binding_sha256: "binding-test".to_owned(),
        presentation_sha256: "presentation-test".to_owned(),
        texture_sha256: Some("texture-test-sha".to_owned()),
        source_shaders: vec![CanonicalShaderPresentationSource {
            package_id: Some("package-test".to_owned()),
            source_ordinal: Some(42),
            member: Some("source-shader.json".to_owned()),
            evidence: ShaderSourceEvidence {
                schema: Some("shader".to_owned()),
                identity: "source-shader".to_owned(),
                version: 0,
                platform_shader_name: Some("simple".to_owned()),
                translucency: Some(1),
                vertex_needs: Some(0),
                vertex_mask: Some(0),
                parameter_count: Some(1),
                texture_reference: Some("source.bmp".to_owned()),
                params: vec![ShaderParameterEvidence {
                    kind: "int".to_owned(),
                    param: "BLMD".to_owned(),
                    value: serde_json::json!(1),
                }],
            },
        }],
    };
    let value = material_binding_value(&binding, &presentation);
    assert_eq!(
        value.get("material_name"),
        Some(&serde_json::json!("material-test-transparent"))
    );
    assert_eq!(
        value.get("texture_file_name"),
        Some(&serde_json::json!("texture-test.png"))
    );
    assert_eq!(
        value.get("base_color_rgba8"),
        Some(&serde_json::json!([12, 34, 56, 78]))
    );
    assert_eq!(
        value.get("binding_sha256"),
        Some(&serde_json::json!("binding-test"))
    );
    assert_eq!(
        value.get("presentation_sha256"),
        Some(&serde_json::json!("presentation-test"))
    );
    assert_eq!(
        value.get("texture_sha256"),
        Some(&serde_json::json!("texture-test-sha"))
    );
    assert_eq!(
        value.pointer("/source_shaders/0/shader/identity"),
        Some(&serde_json::json!("source-shader"))
    );
    assert_eq!(
        value.pointer("/source_shaders/0/source_ordinal"),
        Some(&serde_json::json!(42))
    );
    assert_eq!(
        value.pointer("/semantics/transparent"),
        Some(&serde_json::json!(true))
    );
    assert_eq!(
        value.pointer("/semantics/light_emitter"),
        Some(&serde_json::json!(true))
    );
    assert_eq!(
        value.pointer("/semantics/glass"),
        Some(&serde_json::json!(false))
    );
    Ok(())
}

#[test]
fn world_material_slot_preserves_exact_writer_identity_and_semantics() {
    use fbx::domain::texture::MaterialSemantics;
    use super::super::model::WorldMaterialSlotRecord;

    let slot = WorldMaterialSlotRecord {
        slot_name: "material-test__glass".to_owned(),
        source_material_name: "material-test".to_owned(),
        binding_sha256: "binding-test".to_owned(),
        presentation_sha256: "presentation-test".to_owned(),
        slot_presentation_sha256: "slot-presentation-test".to_owned(),
        semantics: MaterialSemantics::default().with_glass(true),
    };
    let value = material_slot_value(&slot);
    assert_eq!(
        value.get("slot_name"),
        Some(&serde_json::json!("material-test__glass"))
    );
    assert_eq!(
        value.get("source_material_name"),
        Some(&serde_json::json!("material-test"))
    );
    assert_eq!(
        value.get("binding_sha256"),
        Some(&serde_json::json!("binding-test"))
    );
    assert_eq!(
        value.get("slot_presentation_sha256"),
        Some(&serde_json::json!("slot-presentation-test"))
    );
    assert_eq!(
        value.pointer("/semantics/glass"),
        Some(&serde_json::json!(true))
    );
    assert_eq!(
        value.pointer("/semantics/transparent"),
        Some(&serde_json::json!(true))
    );
}

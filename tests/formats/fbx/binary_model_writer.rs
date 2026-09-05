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
//   - Binary model writer test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Binary model writer test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Binary model writer test module.

#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    // jig-ignore-next-line: exact syntax is indivisible
    reason = "Binary FBX test helpers validate marker offsets before exact byte-range extraction."
)]

use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxError, CharacterBinaryFbxSummary, EmbeddedTexture,
    ModelExportRootPolicy, write_binary_model_fbx,
    write_binary_model_fbx_embedded, write_binary_model_fbx_with_policies,
    write_binary_model_fbx_with_target_surface_frames,
};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::texture::MaterialBinding;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

const BINARY_MAGIC: &[u8; 23] = b"Kaydara FBX Binary  \x00\x1a\x00";

fn model_mesh() -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "material",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        vec![[0., 0.], [1., 0.], [0., 1.]],
        &[0, 1, 2],
    )
    .and_then(|group| {
        group.with_normals(vec![[0., 0., 1.], [0., 0., 1.], [0., 0., 1.]])
    })
    .and_then(|group| {
        group.with_colors(vec![[1., 0., 0., 1.], [0., 1., 0., 1.], [
            0., 0., 1., 1.,
        ]])
    })
    .map_err(|error| format!("static primitive group failed: {error:?}"))?;
    MeshAsset::new("model", vec![group])
        .map_err(|error| format!("static mesh failed: {error:?}"))
}

fn material() -> Result<MaterialBinding, String> {
    MaterialBinding::new("material", None)
        .map_err(|error| format!("static material failed: {error:?}"))
}

fn remove_if_present(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "temporary FBX cleanup failed for {}: {error}",
            path.display()
        )),
    }
}

fn output_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "fbx-binary-model-{label}-{}.fbx",
        std::process::id()
    ))
}

fn find_token(bytes: &[u8], token: &str) -> Option<usize> {
    bytes
        .windows(token.len())
        .position(|window| window == token.as_bytes())
}

fn contains_token(bytes: &[u8], token: &str) -> bool {
    find_token(bytes, token).is_some()
}

fn encoded_vector(value: [f64; 3]) -> Vec<u8> {
    value
        .into_iter()
        .flat_map(|component| {
            std::iter::once(b'D').chain(component.to_le_bytes())
        })
        .collect()
}

fn export_root_bytes(bytes: &[u8]) -> Result<&[u8], String> {
    let start = find_token(bytes, "SHAR_Export_Root")
        .ok_or_else(|| "static FBX export root is missing".to_owned())?;
    let end = bytes[start..]
        .windows("Geometry".len())
        .position(|window| window == b"Geometry")
        .map(|relative| start + relative)
        .ok_or_else(|| {
            "static FBX geometry after export root is missing".to_owned()
        })?;
    Ok(&bytes[start..end])
}

#[test]
fn static_model_is_deterministic_and_has_no_rig_objects() -> Result<(), String>
{
    let first = output_path("first");
    let second = output_path("second");
    remove_if_present(&first)?;
    remove_if_present(&second)?;
    let mesh = model_mesh()?;
    let material = material()?;

    let first_summary = write_binary_model_fbx(
        "static-model",
        std::slice::from_ref(&mesh),
        std::slice::from_ref(&material),
        &first,
    )
    .map_err(|error| format!("first static write failed: {error:?}"))?;
    let second_summary =
        write_binary_model_fbx("static-model", &[mesh], &[material], &second)
            .map_err(|error| format!("second static write failed: {error:?}"))?;
    let expected = CharacterBinaryFbxSummary {
        geometries: 1,
        bones: 0,
        clusters: 0,
        materials: 1,
        textures: 0,
        animations: 0,
    };
    if first_summary != expected {
        return Err(format!("unexpected static summary: {first_summary:?}"));
    }
    if first_summary != second_summary {
        return Err(format!(
            "static summaries differ: {first_summary:?} != \
                 {second_summary:?}"
        ));
    }
    let first_bytes = fs::read(&first)
        .map_err(|error| format!("first static read failed: {error}"))?;
    let second_bytes = fs::read(&second)
        .map_err(|error| format!("second static read failed: {error}"))?;
    if first_bytes != second_bytes {
        return Err("static FBX bytes are not deterministic".to_owned());
    }
    if !first_bytes.starts_with(BINARY_MAGIC) {
        return Err("static FBX binary magic is missing".to_owned());
    }
    for required in [
        "Geometry",
        "Model",
        "Material",
        "ColorSet_1",
        "LayerElementSmoothing",
        "Smoothing",
    ] {
        if !contains_token(&first_bytes, required) {
            return Err(format!("static FBX is missing {required}"));
        }
    }
    for forbidden in [
        "Deformer",
        "Pose",
        "NodeAttribute",
        "LimbNode",
        "AnimationStack",
        "AnimationCurve",
    ] {
        if contains_token(&first_bytes, forbidden) {
            return Err(format!("static FBX contains forbidden {forbidden}"));
        }
    }
    remove_if_present(&first)?;
    remove_if_present(&second)?;
    Ok(())
}

#[test]
fn static_model_retains_p3d_source_mesh_identity() -> Result<(), String> {
    let path = output_path("source-mesh-identity");
    remove_if_present(&path)?;
    let mesh = model_mesh()?
        .with_source_identity("AuthoredMeshShape")
        .map_err(|error| format!("source identity failed: {error:?}"))?;
    let material = material()?;
    let _summary = write_binary_model_fbx(
        "source-mesh-identity",
        &[mesh],
        &[material],
        &path,
    )
    .map_err(|error| format!("source identity FBX write failed: {error:?}"))?;
    let bytes = fs::read(&path)
        .map_err(|error| format!("source identity FBX read failed: {error}"))?;
    remove_if_present(&path)?;

    for token in ["SHAR_P3D_SourceMeshIdentity", "AuthoredMeshShape"] {
        if !contains_token(&bytes, token) {
            return Err(format!("static FBX dropped {token}"));
        }
    }
    Ok(())
}

#[test]
fn static_model_retains_p3d_cast_shadow_metadata() -> Result<(), String> {
    let path = output_path("cast-shadow");
    remove_if_present(&path)?;
    let mesh = model_mesh()?.with_cast_shadow(Some(false));
    let material = material()?;
    let _summary =
        write_binary_model_fbx("cast-shadow", &[mesh], &[material], &path)
            .map_err(|error| {
                format!("CastShadow FBX write failed: {error:?}")
            })?;
    let bytes = fs::read(&path)
        .map_err(|error| format!("CastShadow FBX read failed: {error}"))?;
    remove_if_present(&path)?;

    let property_name = "SHAR_P3D_CastShadow";
    let property = find_token(&bytes, property_name)
        .ok_or_else(|| "P3D CastShadow metadata was dropped".to_owned())?;
    let tail = bytes
        .get(property + property_name.len()..)
        .ok_or_else(|| "P3D CastShadow metadata tail is missing".to_owned())?;
    let encoded_false = [b'S', 1, 0, 0, 0, b'0'];
    if !tail
        .windows(encoded_false.len())
        .take(32)
        .any(|window| window == encoded_false)
    {
        return Err("P3D CastShadow false value changed".to_owned());
    }
    Ok(())
}

#[test]
fn static_model_embeds_exact_png_payload() -> Result<(), String> {
    const PNG: &[u8] = b"\x89PNG\r\n\x1a\nembedded-test";
    let path = output_path("embedded");
    remove_if_present(&path)?;
    let mesh = model_mesh()?;
    let material =
        MaterialBinding::new("material", Some("texture.png".to_owned()))
            .map_err(|error| format!("embedded material failed: {error:?}"))?;
    let texture = EmbeddedTexture {
        file_name: "texture.png".to_owned(),
        content: PNG.to_vec(),
    };

    let summary = write_binary_model_fbx_embedded(
        "embedded-model",
        &[mesh],
        &[material],
        &[texture],
        &path,
    )
    .map_err(|error| format!("embedded static write failed: {error:?}"))?;
    if summary.textures != 1 {
        return Err(format!("unexpected embedded texture count: {summary:?}"));
    }
    let bytes = fs::read(&path)
        .map_err(|error| format!("embedded static read failed: {error}"))?;
    if !contains_token(&bytes, "Content")
        || !bytes.windows(PNG.len()).any(|window| window == PNG)
    {
        return Err("embedded static FBX lost the PNG payload".to_owned());
    }
    remove_if_present(&path)?;
    Ok(())
}

#[test]
fn world_reflection_is_shared_by_exterior_and_interior() -> Result<(), String> {
    let legacy_path = output_path("legacy-root");
    let exterior_path = output_path("exterior-root");
    let interior_path = output_path("interior-root");
    for path in [&legacy_path, &exterior_path, &interior_path] {
        remove_if_present(path)?;
    }
    let mesh = model_mesh()?;
    let material = material()?;
    let _summary = write_binary_model_fbx(
        "legacy-root-model",
        std::slice::from_ref(&mesh),
        std::slice::from_ref(&material),
        &legacy_path,
    )
    .map_err(|error| format!("legacy-root write failed: {error:?}"))?;
    let _summary = write_binary_model_fbx_with_policies(
        "exterior-root-model",
        std::slice::from_ref(&mesh),
        std::slice::from_ref(&material),
        ModelExportRootPolicy::ReflectX,
        &exterior_path,
    )
    .map_err(|error| format!("exterior-root write failed: {error:?}"))?;
    let _summary = write_binary_model_fbx_with_policies(
        "interior-root-model",
        &[mesh],
        &[material],
        ModelExportRootPolicy::ReflectX,
        &interior_path,
    )
    .map_err(|error| format!("interior-root write failed: {error:?}"))?;

    let legacy_bytes = fs::read(&legacy_path)
        .map_err(|error| format!("legacy-root read failed: {error}"))?;
    let exterior_bytes = fs::read(&exterior_path)
        .map_err(|error| format!("exterior-root read failed: {error}"))?;
    let interior_bytes = fs::read(&interior_path)
        .map_err(|error| format!("interior-root read failed: {error}"))?;
    let legacy_root = export_root_bytes(&legacy_bytes)?;
    let exterior_root = export_root_bytes(&exterior_bytes)?;
    let interior_root = export_root_bytes(&interior_bytes)?;

    let identity_scale = encoded_vector([1., 1., 1.]);
    let zero_rotation = encoded_vector([0., 0., 0.]);
    let legacy_rotation = encoded_vector([0., 180., 0.]);
    let reflected_scale = encoded_vector([-1., 1., 1.]);

    if !legacy_root
        .windows(legacy_rotation.len())
        .any(|window| window == legacy_rotation)
        || !legacy_root
            .windows(identity_scale.len())
            .any(|window| window == identity_scale)
    {
        return Err(
            "legacy static root no longer preserves character orientation"
                .to_owned(),
        );
    }
    if !exterior_root
        .windows(zero_rotation.len())
        .any(|window| window == zero_rotation)
        || !exterior_root
            .windows(reflected_scale.len())
            .any(|window| window == reflected_scale)
    {
        return Err("exterior world root lacks the X reflection".to_owned());
    }
    if exterior_root
        .windows(legacy_rotation.len())
        .any(|window| window == legacy_rotation)
    {
        return Err(
            "exterior world root inherited the character rotation".to_owned()
        );
    }
    if !interior_root
        .windows(zero_rotation.len())
        .any(|window| window == zero_rotation)
        || !interior_root
            .windows(reflected_scale.len())
            .any(|window| window == reflected_scale)
    {
        return Err("interior static root lacks the X reflection".to_owned());
    }
    if interior_root
        .windows(legacy_rotation.len())
        .any(|window| window == legacy_rotation)
    {
        return Err(
            "interior static root inherited the character rotation".to_owned()
        );
    }

    if exterior_root != interior_root {
        return Err("exterior and interior world roots diverged".to_owned());
    }

    assert_eq!(
        ModelExportRootPolicy::ReflectX
            .relative_matrix_to(ModelExportRootPolicy::ReflectX),
        [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ],
    );

    for path in [&legacy_path, &exterior_path, &interior_path] {
        remove_if_present(path)?;
    }
    Ok(())
}

#[test]
fn static_model_rejects_invalid_aggregate_identity() -> Result<(), String> {
    let mesh = model_mesh()?;
    let material = material()?;
    let path = output_path("invalid-name");
    let result =
        write_binary_model_fbx(" invalid ", &[mesh], &[material], &path);

    if result != Err(CharacterBinaryFbxError::InvalidModelName) {
        return Err(format!("unexpected invalid-name result: {result:?}"));
    }
    if path.exists() {
        return Err("invalid static model created an artifact".to_owned());
    }
    Ok(())
}

#[test]
fn target_surface_frames_complete_missing_pre_lit_geometry()
-> Result<(), String> {
    let preserve_path = output_path("missing-frames-preserve");
    let target_path = output_path("missing-frames-target");
    remove_if_present(&preserve_path)?;
    remove_if_present(&target_path)?;
    let group = PrimitiveGroup::new(
        0,
        "material",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        vec![[0., 0.], [0., 0.], [0., 0.]],
        &[0, 1, 2],
    )
    .map_err(|error| format!("missing-frame group failed: {error:?}"))?;
    let mesh = MeshAsset::new("missing-frames", vec![group])
        .map_err(|error| format!("missing-frame mesh failed: {error:?}"))?;
    let binding = material()?;
    let _preserve_summary = write_binary_model_fbx_with_policies(
        "missing-frames",
        std::slice::from_ref(&mesh),
        std::slice::from_ref(&binding),
        ModelExportRootPolicy::ReflectX,
        &preserve_path,
    )
    .map_err(|error| format!("preserve write failed: {error:?}"))?;
    let _target_summary = write_binary_model_fbx_with_target_surface_frames(
        "missing-frames",
        &[mesh],
        &[binding],
        ModelExportRootPolicy::ReflectX,
        &target_path,
    )
    .map_err(|error| format!("target write failed: {error:?}"))?;
    let preserve = fs::read(&preserve_path)
        .map_err(|error| format!("preserve read failed: {error}"))?;
    let target = fs::read(&target_path)
        .map_err(|error| format!("target read failed: {error}"))?;
    for token in [
        "LayerElementNormal",
        "LayerElementSmoothing",
        "LayerElementTangent",
        "Tangents",
        "LayerElementBinormal",
        "Binormals",
    ] {
        if contains_token(&preserve, token) {
            return Err(format!("preserve policy synthesized {token}"));
        }
        if !contains_token(&target, token) {
            return Err(format!("target policy omitted {token}"));
        }
    }
    remove_if_present(&preserve_path)?;
    remove_if_present(&target_path)?;
    Ok(())
}

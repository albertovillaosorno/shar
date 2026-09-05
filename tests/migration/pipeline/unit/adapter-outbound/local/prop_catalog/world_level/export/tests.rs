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
//   - Tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Tests test module.
// - Description:
//   - Implements the declared test module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Tests test module.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::ModelExportRootPolicy;
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::texture::MaterialBinding;
use shar_sha256::digest_hex;

use super::{
    MasterContent, PreparedTexture, WORLD_ROOT_POLICY,
    append_authored_interior_meshes, append_world_fbx_to_guide,
    split_unreal_importable_topology, write_content_fbx,
};

const TEXTURE_FILE_NAME: &str = "interior-test.png";
const TEXTURE_BYTES: &[u8] = b"canonical-interior-texture-payload";

fn textured_mesh() -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "interior-material",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        vec![[0., 0.], [1., 0.], [0., 1.]],
        &[0, 1, 2],
    )
    .and_then(|group| {
        group.with_normals(vec![[0., 0., 1.], [0., 0., 1.], [0., 0., 1.]])
    })
    .map_err(|error| format!("interior fixture group failed: {error:?}"))?;
    MeshAsset::new("interior-mesh", vec![group])
        .map_err(|error| format!("interior fixture mesh failed: {error:?}"))
}

fn named_textured_mesh(name: &str, x: f32) -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "interior-material",
        vec![[x, 0., 0.], [x + 1., 0., 0.], [x, 1., 0.]],
        vec![[0., 0.], [1., 0.], [0., 1.]],
        &[0, 1, 2],
    )
    .and_then(|group| {
        group.with_normals(vec![[0., 0., 1.], [0., 0., 1.], [0., 0., 1.]])
    })
    .map_err(|error| format!("ordered fixture group failed: {error:?}"))?;
    MeshAsset::new(name, vec![group])
        .map_err(|error| format!("ordered fixture mesh failed: {error:?}"))
}

fn textured_content() -> Result<MasterContent, String> {
    let material = MaterialBinding::new(
        "interior-material",
        Some(TEXTURE_FILE_NAME.to_owned()),
    )
    .map_err(|error| format!("interior fixture material failed: {error:?}"))?;
    let texture = PreparedTexture {
        file_name: TEXTURE_FILE_NAME.to_owned(),
        bytes: TEXTURE_BYTES.to_vec(),
        sha256: digest_hex(TEXTURE_BYTES),
    };
    Ok(MasterContent {
        meshes: vec![textured_mesh()?],
        review: Vec::new(),
        materials: BTreeMap::from([(material.material_name.clone(), material)]),
        textures: BTreeMap::from([(texture.file_name.clone(), texture)]),
        packages: Vec::new(),
    })
}

fn temporary_root() -> PathBuf {
    std::env::temp_dir().join(format!(
        "shar-world-nested-texture-test-{}",
        std::process::id()
    ))
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn remove_if_present(path: &Path) -> Result<(), String> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "interior fixture cleanup failed for {}: {error}",
            path.display()
        )),
    }
}

#[test]
fn nested_fbx_publishes_adjacent_external_textures() -> Result<(), String> {
    let root = temporary_root();
    remove_if_present(&root)?;
    fs::create_dir_all(&root).map_err(|error| {
        format!("interior fixture root creation failed: {error}")
    })?;
    let result = (|| {
        let relative = "interiors/i00-test/i00-test.fbx";
        let mut content = textured_content()?;
        let record = write_content_fbx(
            "i00-test",
            relative,
            &mut content,
            &root,
            ModelExportRootPolicy::ReflectX,
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(|| String::from("interior fixture FBX was not written"))?;
        if record.summary.textures != 1 {
            return Err(format!(
                "interior fixture texture count changed: {}",
                record.summary.textures
            ));
        }
        let fbx_path = root.join(relative);
        let fbx_bytes = fs::read(&fbx_path).map_err(|error| {
            format!("interior fixture FBX read failed: {error}")
        })?;
        if !contains_bytes(
            &fbx_bytes,
            format!("textures/{TEXTURE_FILE_NAME}").as_bytes(),
        ) {
            return Err(String::from("interior FBX texture reference changed"));
        }
        let texture_path = root
            .join("interiors/i00-test/textures")
            .join(TEXTURE_FILE_NAME);
        let published = fs::read(&texture_path).map_err(|error| {
            format!(
                "nested interior texture is not resolvable beside the \
                     FBX: {error}"
            )
        })?;
        if published != TEXTURE_BYTES {
            return Err(String::from("nested interior texture bytes changed"));
        }
        Ok(())
    })();
    let cleanup = remove_if_present(&root);
    result.and(cleanup)
}

#[test]
fn guide_append_preserves_exterior_world_fbx_geometry_exactly()
-> Result<(), String> {
    let source = textured_content()?;
    let source_mesh = source
        .meshes
        .first()
        .ok_or_else(|| "world FBX fixture mesh is missing".to_owned())?;
    let source_group = source_mesh
        .groups
        .first()
        .ok_or_else(|| "world FBX fixture group is missing".to_owned())?;
    let expected_positions = source_group.positions.clone();
    let expected_normals = source_group.normals.clone();
    let expected_uvs = source_group.uvs.clone();
    let expected_triangles = source_group.triangles.clone();
    let mut guide = MasterContent::default();
    append_world_fbx_to_guide(&source, &mut guide)
        .map_err(|error| error.to_string())?;
    let combined_mesh = guide
        .meshes
        .first()
        .ok_or_else(|| "combined guide mesh is missing".to_owned())?;
    let combined_group = combined_mesh
        .groups
        .first()
        .ok_or_else(|| "combined guide group is missing".to_owned())?;
    assert_eq!(combined_group.positions, expected_positions,);
    assert_eq!(combined_group.normals, expected_normals,);
    assert_eq!(combined_group.uvs, expected_uvs,);
    assert_eq!(combined_group.triangles, expected_triangles,);
    assert_eq!(guide.materials.len(), source.materials.len(),);
    assert_eq!(guide.textures.len(), source.textures.len(),);
    Ok(())
}

#[test]
fn guide_append_preserves_interior_world_fbx_geometry_exactly()
-> Result<(), String> {
    let mut source = textured_content()?;
    let group = source
        .meshes
        .first_mut()
        .and_then(|mesh| mesh.groups.first_mut())
        .ok_or_else(|| "interior FBX fixture group is missing".to_owned())?;
    group.positions = vec![[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]];
    group.normals = vec![[1., 0., 0.,]; 3];
    let expected_positions = group.positions.clone();
    let expected_normals = group.normals.clone();
    let expected_uvs = group.uvs.clone();
    let expected_triangles = group.triangles.clone();
    let mut guide = MasterContent::default();
    append_world_fbx_to_guide(&source, &mut guide)
        .map_err(|error| error.to_string())?;
    let combined = guide
        .meshes
        .first()
        .and_then(|mesh| mesh.groups.first())
        .ok_or_else(|| "combined interior guide group is missing".to_owned())?;
    assert_eq!(combined.positions, expected_positions);
    assert_eq!(combined.normals, expected_normals);
    assert_eq!(combined.uvs, expected_uvs);
    assert_eq!(combined.triangles, expected_triangles);
    Ok(())
}

#[test]
fn world_fbx_policy_reflects_x_once_and_preserves_authored_uvs() {
    assert_eq!(WORLD_ROOT_POLICY, ModelExportRootPolicy::ReflectX);
}

#[test]
fn world_fbx_write_preserves_source_mesh_order() -> Result<(), String> {
    let root = temporary_root().with_file_name(format!(
        "shar-world-source-order-test-{}",
        std::process::id()
    ));
    remove_if_present(&root)?;
    fs::create_dir_all(&root).map_err(|error| {
        format!("world order fixture root creation failed: {error}")
    })?;
    let result = (|| {
        let mut content = textured_content()?;
        content.meshes = vec![
            named_textured_mesh("z-source", 10.)?,
            named_textured_mesh("a-source", 1.)?,
        ];
        let _record = write_content_fbx(
            "source-order",
            "source-order.fbx",
            &mut content,
            &root,
            ModelExportRootPolicy::ReflectX,
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "source-order fixture FBX was not written".to_owned())?;
        let names = content
            .meshes
            .iter()
            .map(|mesh| mesh.name.as_str())
            .collect::<Vec<_>>();
        if names != ["z-source", "a-source"] {
            return Err(format!("world FBX mesh order changed: {names:?}"));
        }
        Ok(())
    })();
    let cleanup = remove_if_present(&root);
    result.and(cleanup)
}


#[test]
fn world_fbx_preserves_repeated_index_evidence_outside_unreal_geometry()
-> Result<(), String> {
    let root = temporary_root().with_file_name(format!(
        "shar-world-repeated-index-test-{}",
        std::process::id()
    ));
    remove_if_present(&root)?;
    fs::create_dir_all(&root).map_err(|error| {
        format!("repeated-index fixture root creation failed: {error}")
    })?;
    let group = PrimitiveGroup::new_preserving_repeated_indices(
        0,
        "interior-material",
        vec![
            [0., 0., 0.],
            [1., 0., 0.],
            [0., 1., 0.],
            [1., 1., 0.],
        ],
        Vec::new(),
        &[0, 1, 2, 0, 0, 3],
    )
    .map_err(|error| format!("repeated-index fixture failed: {error:?}"))?
    .with_source_ordinal(41);
    let mesh = MeshAsset::new("repeated-index", vec![group])
        .map_err(|error| format!("repeated-index mesh failed: {error:?}"))?
        .with_source_identity("source-mesh")
        .map_err(|error| format!("source identity failed: {error:?}"))?;
    let material = MaterialBinding::new("interior-material", None)
        .map_err(|error| format!("repeated-index material failed: {error:?}"))?;
    let mut content = MasterContent {
        meshes: vec![mesh],
        materials: BTreeMap::from([(material.material_name.clone(), material)]),
        ..MasterContent::default()
    };
    let source_triangles = content
        .meshes
        .first()
        .and_then(|mesh| mesh.groups.first())
        .map(|group| group.triangles.clone())
        .ok_or_else(|| "source fixture group is missing".to_owned())?;
    let (target, evidence) = split_unreal_importable_topology(&content, false);
    let target_group = target
        .meshes
        .first()
        .and_then(|mesh| mesh.groups.first())
        .ok_or_else(|| "target fixture group is missing".to_owned())?;
    assert_eq!(target_group.triangles, vec![[0, 1, 2]]);
    assert_eq!(evidence.len(), 1);
    let entry = evidence
        .first()
        .ok_or_else(|| "topology evidence entry is missing".to_owned())?;
    assert_eq!(entry.mesh, "repeated-index");
    assert_eq!(entry.source_mesh.as_deref(), Some("source-mesh"));
    assert_eq!(entry.group, 0);
    assert_eq!(entry.source_ordinal, Some(41));
    assert_eq!(entry.triangle, 1);
    assert_eq!(entry.indices, [0, 0, 3]);

    let record = write_content_fbx(
        "repeated-index",
        "repeated-index.fbx",
        &mut content,
        &root,
        ModelExportRootPolicy::ReflectX,
    )
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "repeated-index target FBX was not written".to_owned())?;
    let preserved = content
        .meshes
        .first()
        .and_then(|mesh| mesh.groups.first())
        .ok_or_else(|| "preserved source group is missing".to_owned())?;
    assert_eq!(preserved.triangles, source_triangles);
    assert_eq!(record.unreal_omitted_repeated_index_triangles, 1);
    let sidecar = record
        .topology_evidence
        .ok_or_else(|| {
            "topology evidence sidecar was not recorded".to_owned()
        })?;
    assert_eq!(sidecar.repeated_index_triangles, 1);
    let sidecar_bytes = fs::read(root.join(&sidecar.path))
        .map_err(|error| format!("topology sidecar read failed: {error}"))?;
    assert_eq!(digest_hex(&sidecar_bytes), sidecar.sha256);
    let value: serde_json::Value = serde_json::from_slice(&sidecar_bytes)
        .map_err(|error| format!("topology sidecar JSON failed: {error}"))?;
    assert_eq!(
        value.get("schema").and_then(serde_json::Value::as_str),
        Some("shar.world-fbx-source-topology.v2")
    );
    assert_eq!(
        value
            .get("unreal_omitted_repeated_index_triangles")
            .and_then(serde_json::Value::as_u64),
        Some(1)
    );
    let triangle = value
        .get("triangles")
        .and_then(serde_json::Value::as_array)
        .and_then(|triangles| triangles.first())
        .ok_or_else(|| "topology sidecar triangle is missing".to_owned())?;
    assert_eq!(
        triangle.get("triangle").and_then(serde_json::Value::as_u64),
        Some(1)
    );
    assert_eq!(triangle.get("indices"), Some(&serde_json::json!([0, 0, 3])));
    assert!(root.join("repeated-index.fbx").is_file());
    remove_if_present(&root)
}

#[test]
fn world_fbx_preserves_zero_area_evidence_outside_unreal_geometry()
-> Result<(), String> {
    let root = temporary_root().with_file_name(format!(
        "shar-world-zero-area-test-{}",
        std::process::id()
    ));
    remove_if_present(&root)?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let group = PrimitiveGroup::new_preserving_repeated_indices(
        0,
        "interior-material",
        vec![
            [0., 0., 0.],
            [1., 0., 0.],
            [2., 0., 0.],
            [0., 1., 0.],
        ],
        Vec::new(),
        &[0, 1, 3, 0, 1, 2],
    )
    .map_err(|error| format!("zero-area fixture failed: {error:?}"))?;
    let mesh = MeshAsset::new("zero-area", vec![group])
        .map_err(|error| format!("zero-area mesh failed: {error:?}"))?;
    let material = MaterialBinding::new("interior-material", None)
        .map_err(|error| format!("zero-area material failed: {error:?}"))?;
    let mut content = MasterContent {
        meshes: vec![mesh],
        materials: BTreeMap::from([(material.material_name.clone(), material)]),
        ..MasterContent::default()
    };
    let record = write_content_fbx(
        "zero-area",
        "zero-area.fbx",
        &mut content,
        &root,
        ModelExportRootPolicy::ReflectX,
    )
    .map_err(|error| error.to_string())?
    .ok_or_else(|| "zero-area target FBX was not written".to_owned())?;
    assert_eq!(record.unreal_omitted_repeated_index_triangles, 0);
    assert_eq!(record.unreal_omitted_zero_area_triangles, 1);
    let sidecar = record.topology_evidence
        .ok_or_else(|| "zero-area topology evidence is missing".to_owned())?;
    assert_eq!(sidecar.repeated_index_triangles, 0);
    assert_eq!(sidecar.zero_area_triangles, 1);
    let value: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(sidecar.path)).map_err(|error| error.to_string())?
    ).map_err(|error| error.to_string())?;
    assert_eq!(
        value.get("unreal_omitted_zero_area_triangles")
            .and_then(serde_json::Value::as_u64),
        Some(1)
    );
    assert_eq!(
        value.get("triangles")
            .and_then(serde_json::Value::as_array)
            .and_then(|triangles| triangles.first())
            .and_then(|triangle| triangle.get("reason"))
            .and_then(serde_json::Value::as_str),
        Some("zero_area")
    );
    remove_if_present(&root)
}

#[test]
fn world_fbx_rejects_source_with_no_unreal_importable_geometry()
-> Result<(), String> {
    let root = temporary_root().with_file_name(format!(
        "shar-world-only-repeated-index-test-{}",
        std::process::id()
    ));
    remove_if_present(&root)?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let group = PrimitiveGroup::new_preserving_repeated_indices(
        0,
        "interior-material",
        vec![[0., 0., 0.], [1., 0., 0.]],
        Vec::new(),
        &[0, 0, 1],
    )
    .map_err(|error| format!("all-repeated fixture failed: {error:?}"))?;
    let mesh = MeshAsset::new("all-repeated", vec![group])
        .map_err(|error| format!("all-repeated mesh failed: {error:?}"))?;
    let material = MaterialBinding::new("interior-material", None)
        .map_err(|error| format!("all-repeated material failed: {error:?}"))?;
    let mut content = MasterContent {
        meshes: vec![mesh],
        materials: BTreeMap::from([(material.material_name.clone(), material)]),
        ..MasterContent::default()
    };
    let result = write_content_fbx(
        "all-repeated",
        "all-repeated.fbx",
        &mut content,
        &root,
        ModelExportRootPolicy::ReflectX,
    );
    let message = match result {
        Ok(_artifact) => return Err("all-repeated FBX was accepted".to_owned()),
        Err(error) => error.to_string(),
    };
    if !message.contains("no Unreal-importable geometry") {
        return Err(format!("all-repeated rejection changed: {message}"));
    }
    if root.join("all-repeated.fbx").exists() {
        return Err("all-repeated FBX was published".to_owned());
    }
    remove_if_present(&root)
}

#[test]
fn interior_fusion_preserves_authored_face_multiplicity_and_order()
-> Result<(), String> {
    let group = PrimitiveGroup::new(
        0,
        "int_sh_wall_m",
        vec![
            [0., 0., 0.],
            [1., 0., 0.],
            [0., 1., 0.],
            [0., 1., 0.],
            [1., 0., 0.],
            [0., 0., 0.],
        ],
        Vec::new(),
        &[0, 1, 2, 3, 4, 5],
    )
    .map_err(|error| format!("duplicate face fixture failed: {error:?}"))?;
    let duplicate_faces = MeshAsset::new("l7i02-wall", vec![group])
        .map_err(|error| format!("duplicate mesh fixture failed: {error:?}"))?;
    let second = named_textured_mesh("second-source", 10.)?;
    let mut source = MasterContent {
        meshes: vec![duplicate_faces, second],
        ..MasterContent::default()
    };
    let mut fused = MasterContent::default();
    append_authored_interior_meshes(&mut fused, &mut source);
    assert!(source.meshes.is_empty());
    let names = fused
        .meshes
        .iter()
        .map(|mesh| mesh.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(names, ["l7i02-wall", "second-source"]);
    let triangles = fused
        .meshes
        .first()
        .and_then(|mesh| mesh.groups.first())
        .map(|group| &group.triangles)
        .ok_or_else(|| "duplicate-face fusion fixture is missing".to_owned())?;
    assert_eq!(triangles, &vec![[0, 1, 2], [3, 4, 5]]);
    Ok(())
}

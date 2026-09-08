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
//   - Generated world material catalog verifier tests.
// - Must-Not:
//   - Read private game assets or contact Unreal Editor.
// - Allows:
//   - Synthetic FBX, material, slot, and shader fixtures.
// - Split-When:
//   - World material plan promotion gains independent tests.
// - Merge-When:
//   - Another adapter test owns identical world presentation verification.
// - Summary:
//   - Generated world material verifier tests.
// - Description:
//   - Proves absence, exact presentation verification, stale FBX rejection,
//   - and source-shader raster unanimity.
// - Usage:
//   - Included only by the owning local adapter under cfg(test).
// - Defaults:
//   - Missing roots remain absent and malformed evidence fails closed.
//

//! Generated world material catalog verifier tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Value, json};
use shar_sha256::digest_hex;

use super::{
    FBX_VERSION, RUNTIME_ERROR_BINDING_SHA256,
    RUNTIME_ERROR_PRESENTATION_SHA256, shader_raster,
    verified_world_material_catalog,
};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);
const BINDING: &str =
    "1111111111111111111111111111111111111111111111111111111111111111";
const PRESENTATION: &str =
    "2222222222222222222222222222222222222222222222222222222222222222";
const EFFECTIVE: &str =
    "3333333333333333333333333333333333333333333333333333333333333333";

#[derive(Clone, Copy)]
enum ShaderFixture {
    One,
    Duplicate,
    Disagree,
    Missing,
    RuntimeFallback,
}

struct TempRoot(PathBuf);

impl TempRoot {
    fn new(label: &str) -> Result<Self, String> {
        let sequence = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
        let path = PathBuf::from(".temp").join(format!(
            "unreal-world-material-{label}-{}-{sequence}",
            std::process::id()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).map_err(|error| error.to_string())?;
        }
        Ok(Self(path))
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _result = fs::remove_dir_all(&self.0);
    }
}

fn fbx_bytes() -> Vec<u8> {
    let mut bytes = b"Kaydara FBX Binary  \0\x1a\0".to_vec();
    bytes.extend_from_slice(&FBX_VERSION.to_le_bytes());
    bytes.extend_from_slice(b"world-material-fixture");
    bytes
}

fn shader_with_sidedness(blend: u8, two_sided: u8) -> Value {
    json!({
        "platform_shader_name": "simple\0\0",
        "params": [
            {"param": "LIT", "value": 0},
            {"param": "2SID", "value": two_sided},
            {"param": "BLMD", "value": blend},
            {"param": concat!("AC", "MP"), "value": 4},
            {"param": concat!("AT", "ST"), "value": 0}
        ]
    })
}

fn shader(blend: u8) -> Value {
    shader_with_sidedness(blend, 0)
}

fn write_catalog(
    root: &Path,
    declared_size: Option<u64>,
    shader_fixture: ShaderFixture,
    texture_bytes: Option<&[u8]>,
    unused_texture: bool,
) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|error| error.to_string())?;
    let bytes = fbx_bytes();
    fs::write(root.join("world.fbx"), &bytes)
        .map_err(|error| error.to_string())?;
    let sources = match shader_fixture {
        ShaderFixture::One => vec![json!({"shader": shader(0)})],
        ShaderFixture::Duplicate => vec![
            json!({"shader": shader(0)}),
            json!({"shader": shader(0)}),
        ],
        ShaderFixture::Disagree => vec![
            json!({"shader": shader(0)}),
            json!({"shader": shader(1)}),
        ],
        ShaderFixture::Missing | ShaderFixture::RuntimeFallback => Vec::new(),
    };
    let (binding_sha256, presentation_sha256) = match shader_fixture {
        ShaderFixture::RuntimeFallback => (
            RUNTIME_ERROR_BINDING_SHA256,
            RUNTIME_ERROR_PRESENTATION_SHA256,
        ),
        _ => (BINDING, PRESENTATION),
    };
    let material = format!("material-{binding_sha256}");
    let texture_digest = texture_bytes.map(digest_hex);
    let texture_file_name = texture_digest
        .as_ref()
        .map(|digest| format!("texture-{digest}.png"));
    if let (Some(texture_bytes), Some(file_name)) =
        (texture_bytes, texture_file_name.as_ref())
    {
        let texture_root = root.join("textures");
        fs::create_dir_all(&texture_root)
            .map_err(|error| error.to_string())?;
        fs::write(texture_root.join(file_name), texture_bytes)
            .map_err(|error| error.to_string())?;
    }
    let artifact = json!({
        "path": "world.fbx",
        "bytes": declared_size.unwrap_or_else(|| {
            u64::try_from(bytes.len()).unwrap_or(u64::MAX)
        }),
        "sha256": digest_hex(&bytes),
        "material_bindings": [{
            "material_name": material,
            "texture_file_name": texture_file_name,
            "texture_sha256": texture_digest,
            "binding_sha256": binding_sha256,
            "presentation_sha256": presentation_sha256,
            "base_color_rgba8": [255, 255, 255, 255],
            "semantics": {
                "transparent": false,
                "glass": false,
                "mirror": false,
                "reflective": false,
                "light_emitter": false,
                "visual_effect": false
            },
            "source_shaders": sources
        }],
        "material_slots": [{
            "slot_name": material,
            "source_material_name": material,
            "binding_sha256": binding_sha256,
            "presentation_sha256": presentation_sha256,
            "slot_presentation_sha256": EFFECTIVE,
            "semantics": {
                "transparent": false,
                "glass": false,
                "mirror": false,
                "reflective": false,
                "light_emitter": false,
                "visual_effect": false
            }
        }]
    });
    let mut textures = Vec::new();
    if let (Some(texture_bytes), Some(digest), Some(file_name)) = (
        texture_bytes,
        texture_digest.as_ref(),
        texture_file_name.as_ref(),
    ) {
        textures.push(json!({
            "file_name": file_name,
            "bytes": u64::try_from(texture_bytes.len()).unwrap_or(u64::MAX),
            "sha256": digest
        }));
    }
    if unused_texture {
        let unused = b"\x89PNG\r\n\x1a\nunused";
        let digest = digest_hex(unused);
        let file_name = format!("texture-{digest}.png");
        let texture_root = root.join("textures");
        fs::create_dir_all(&texture_root)
            .map_err(|error| error.to_string())?;
        fs::write(texture_root.join(&file_name), unused)
            .map_err(|error| error.to_string())?;
        textures.push(json!({
            "file_name": file_name,
            "bytes": unused.len(),
            "sha256": digest
        }));
    }
    let catalog = json!({
        "schema": "shar.world-package-collection.v8",
        "status": "source-authored-fbx-baseline",
        "counts": {"world_fbx_files": 1, "review_fbx_files": 0},
        "textures": textures,
        "packages": [{"world_fbx": artifact, "review_fbx": null}],
        "interiors": []
    });
    fs::write(
        root.join("world.catalog.json"),
        serde_json::to_vec_pretty(&catalog).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())
}

#[test]
fn absent_world_catalog_keeps_material_evidence_absent() -> Result<(), String> {
    let root = TempRoot::new("absent")?;
    if verified_world_material_catalog(&root.0)
        .map_err(|error| error.to_string())?
        .is_some()
    {
        return Err("absent world catalog produced evidence".to_owned());
    }
    Ok(())
}

#[test]
fn verifies_world_material_projection_without_promoting_readiness()
-> Result<(), String> {
    let root = TempRoot::new("valid")?;
    let texture = b"\x89PNG\r\n\x1a\nverified";
    write_catalog(
        &root.0,
        None,
        ShaderFixture::One,
        Some(texture),
        false,
    )?;
    let verified = verified_world_material_catalog(&root.0)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "world material evidence was absent".to_owned())?;
    assert_eq!(verified.artifact_count, 1);
    assert_eq!(verified.binding_count, 1);
    assert_eq!(verified.slot_count, 1);
    assert_eq!(verified.master_family_count, 1);
    assert_eq!(verified.artifacts.len(), 1);
    let artifact = verified
        .artifacts
        .first()
        .ok_or_else(|| "verified world artifact disappeared".to_owned())?;
    assert_eq!(artifact.path, "world.fbx");
    assert_eq!(artifact.projection.presentations().len(), 1);
    assert_eq!(artifact.projection.assignments().len(), 1);
    assert_eq!(verified.textures.len(), 1);
    let verified_texture = verified
        .textures
        .first()
        .ok_or_else(|| "verified world texture disappeared".to_owned())?;
    assert_eq!(verified_texture.sha256, digest_hex(texture));
    Ok(())
}

#[test]
fn source_two_sided_flag_selects_distinct_master_family()
-> Result<(), String> {
    let one_sided = shader_with_sidedness(0, 0);
    let two_sided = shader_with_sidedness(0, 1);
    let one_sided = shader_raster(
        one_sided
            .as_object()
            .ok_or_else(|| {
                "one-sided shader fixture is not an object".to_owned()
            })?,
    )
    .map_err(|error| error.to_string())?;
    let two_sided = shader_raster(
        two_sided
            .as_object()
            .ok_or_else(|| {
                "two-sided shader fixture is not an object".to_owned()
            })?,
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(
        one_sided.master.identity(),
        "simple__blend-none__alpha-test-off__one-sided__unlit"
    );
    assert_eq!(
        two_sided.master.identity(),
        "simple__blend-none__alpha-test-off__two-sided__unlit"
    );
    assert_ne!(one_sided.master, two_sided.master);
    Ok(())
}

#[test]
fn stale_world_fbx_size_fails_closed() -> Result<(), String> {
    let root = TempRoot::new("stale")?;
    write_catalog(
        &root.0,
        Some(999),
        ShaderFixture::One,
        None,
        false,
    )?;
    let error = match verified_world_material_catalog(&root.0) {
        Ok(_value) => {
            return Err("stale world FBX unexpectedly verified".to_owned());
        },
        Err(error) => error,
    };
    assert!(error.to_string().contains("FBX bytes do not match"));
    Ok(())
}

#[test]
fn disagreeing_source_shader_raster_fails_closed() -> Result<(), String> {
    let root = TempRoot::new("disagree")?;
    write_catalog(&root.0, None, ShaderFixture::Disagree, None, false)?;
    let error = match verified_world_material_catalog(&root.0) {
        Ok(_value) => {
            return Err("disagreeing world source shaders verified".to_owned());
        },
        Err(error) => error,
    };
    assert!(error.to_string().contains("shader documents disagree"));
    Ok(())
}


#[test]
fn stale_world_texture_bytes_fail_closed() -> Result<(), String> {
    let root = TempRoot::new("stale-texture")?;
    let texture = b"\x89PNG\r\n\x1a\nworld-texture-fixture";
    write_catalog(&root.0, None, ShaderFixture::One, Some(texture), false)?;
    let digest = digest_hex(texture);
    let path = root
        .0
        .join("textures")
        .join(format!("texture-{digest}.png"));
    fs::write(path, b"tampered-texture")
        .map_err(|error| error.to_string())?;
    let error = match verified_world_material_catalog(&root.0) {
        Ok(_value) => {
            return Err("stale world texture unexpectedly verified".to_owned());
        },
        Err(error) => error,
    };
    assert!(error.to_string().contains("texture bytes do not match"));
    Ok(())
}

#[test]
fn unused_world_texture_table_row_fails_closed() -> Result<(), String> {
    let root = TempRoot::new("unused-texture")?;
    write_catalog(&root.0, None, ShaderFixture::One, None, true)?;
    let error = match verified_world_material_catalog(&root.0) {
        Ok(_value) => {
            return Err(
                "unused world texture row unexpectedly verified".to_owned(),
            );
        },
        Err(error) => error,
    };
    assert!(error.to_string().contains("texture table does not match"));
    Ok(())
}


#[test]
fn duplicate_identical_source_shader_documents_are_accepted()
-> Result<(), String> {
    let root = TempRoot::new("duplicate-shader")?;
    write_catalog(
        &root.0,
        None,
        ShaderFixture::Duplicate,
        None,
        false,
    )?;
    let verified = verified_world_material_catalog(&root.0)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "world material evidence was absent".to_owned())?;
    assert_eq!(verified.binding_count, 1);
    assert_eq!(verified.master_family_count, 1);
    Ok(())
}

#[test]
fn missing_source_shader_requires_exact_runtime_fallback_identity()
-> Result<(), String> {
    let root = TempRoot::new("forged-fallback")?;
    write_catalog(
        &root.0,
        None,
        ShaderFixture::Missing,
        None,
        false,
    )?;
    let error = match verified_world_material_catalog(&root.0) {
        Ok(_value) => {
            return Err("forged missing-shader fallback verified".to_owned());
        },
        Err(error) => error,
    };
    assert!(error.to_string().contains("fallback identity drifted"));
    Ok(())
}


#[test]
fn exact_runtime_missing_shader_fallback_is_accepted() -> Result<(), String> {
    let root = TempRoot::new("runtime-fallback")?;
    write_catalog(
        &root.0,
        None,
        ShaderFixture::RuntimeFallback,
        None,
        false,
    )?;
    let verified = verified_world_material_catalog(&root.0)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "runtime fallback evidence was absent".to_owned())?;
    assert_eq!(verified.binding_count, 1);
    assert_eq!(verified.slot_count, 1);
    assert_eq!(verified.master_family_count, 1);
    Ok(())
}

#[test]
fn non_png_world_texture_fails_closed() -> Result<(), String> {
    let root = TempRoot::new("non-png")?;
    write_catalog(
        &root.0,
        None,
        ShaderFixture::One,
        Some(b"not-a-png"),
        false,
    )?;
    let error = match verified_world_material_catalog(&root.0) {
        Ok(_value) => {
            return Err(
                "non-PNG world texture unexpectedly verified".to_owned(),
            );
        },
        Err(error) => error,
    };
    assert!(error.to_string().contains("not a PNG artifact"));
    Ok(())
}

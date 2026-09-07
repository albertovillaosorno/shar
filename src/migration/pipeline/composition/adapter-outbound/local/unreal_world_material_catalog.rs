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
//   - Verification of generated world material evidence for Unreal planning.
// - Must-Not:
//   - Promote world packages, select Unreal materials, or assemble the world.
// - Allows:
//   - Verify world FBX and adjacent texture bytes and project reviewed raster
//   - state through the asset-conversion domain.
// - Split-When:
//   - World material plan promotion gains an independent lifecycle.
// - Merge-When:
//   - Another adapter owns identical world presentation verification.
// - Summary:
//   - Generated world material catalog verifier.
// - Description:
//   - Verifies v8 world presentation artifacts, source-shader unanimity, exact
//   - writer slots, and reviewed PDDI raster families without claiming native
//   - material construction is complete.
// - Usage:
//   - Used by prepare-unreal as read-only generated presentation evidence.
// - Defaults:
//   - Missing roots remain absent; malformed or stale roots fail closed.
//

//! Generated world material catalog verification.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use shar_sha256::digest_hex;
use shar_unreal_conversion::domain::{
    WORLD_MATERIAL_SOURCE_SCHEMA, WorldMaterialBindingSource,
    WorldMaterialProjection, WorldMaterialRasterProjection,
    WorldMaterialSemantics, WorldMaterialShaderFamily,
    WorldMaterialSlotSource,
};

use super::unreal_fbx_catalog::{
    FBX_VERSION, PNG_MAGIC, binary_fbx_version, io_error,
    validate_ancestor_chain, validate_digest, validate_directory_metadata,
    validate_regular_file, validate_relative_path,
};
use crate::domain::{PipelineError, PipelineOutcome};

const CATALOG_FILE: &str = "world.catalog.json";
const CATALOG_STATUS: &str = "source-authored-fbx-baseline";
const RUNTIME_ERROR_BINDING_SHA256: &str =
    "9c9f4edf0db8ca2fe28f892bdb2e1a88df4445f2649fd665ddd5b78154bc7844";
const RUNTIME_ERROR_PRESENTATION_SHA256: &str =
    "8ee7e678c6a6b1d448a69777cb50aa4aa52dba187f36caa6bed3ff070022b088";

/// Verified aggregate world-presentation evidence for Unreal preflight.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct VerifiedWorldMaterialCatalog {
    pub artifact_count: usize,
    pub binding_count: usize,
    pub slot_count: usize,
    pub master_family_count: usize,
}

/// Verify generated world FBX/material evidence when the root exists.
///
/// # Errors
///
/// Returns an error for malformed catalog structure, stale FBX or PNG bytes,
/// non-unanimous source shader state, unsafe paths, or unsupported raster
/// state.
pub(super) fn verified_world_material_catalog(
    root: &Path,
) -> PipelineOutcome<Option<VerifiedWorldMaterialCatalog>> {
    let metadata = match fs::symlink_metadata(root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        },
        Err(error) => {
            return Err(io_error(
                "inspect generated world catalog root",
                &error,
            ));
        },
        Ok(metadata) => metadata,
    };
    validate_directory_metadata(&metadata)?;
    let catalog_path = root.join(CATALOG_FILE);
    validate_regular_file(&catalog_path, "generated world catalog")?;
    let text = fs::read_to_string(&catalog_path)
        .map_err(|error| io_error("read generated world catalog", &error))?;
    let value = serde_json::from_str::<Value>(&text).map_err(|_error| {
        PipelineError::new("generated world catalog contains invalid JSON")
    })?;
    let object = as_object(&value, "generated world catalog")?;
    if required_string(object, "schema")? != WORLD_MATERIAL_SOURCE_SCHEMA
        || required_string(object, "status")? != CATALOG_STATUS
    {
        return Err(PipelineError::new(
            "generated world catalog schema or status is not supported",
        ));
    }
    let texture_table = verified_texture_table(object)?;
    let mut state = WorldMaterialVerificationState::new();
    let mut artifact_count = 0usize;

    let packages = required_array(object, "packages")?;
    for package in packages {
        let package = as_object(package, "generated world package")?;
        for field in ["world_fbx", "review_fbx"] {
            if let Some(artifact) = optional_object(package, field)? {
                verify_artifact(
                    root,
                    artifact,
                    &texture_table,
                    &mut state,
                )?;
                artifact_count = artifact_count.saturating_add(1);
            }
        }
    }
    let interiors = required_array(object, "interiors")?;
    for interior in interiors {
        let interior = as_object(interior, "generated world interior")?;
        for field in ["base_fbx", "halloween_fbx"] {
            if let Some(artifact) = optional_object(interior, field)? {
                verify_artifact(
                    root,
                    artifact,
                    &texture_table,
                    &mut state,
                )?;
                artifact_count = artifact_count.saturating_add(1);
            }
        }
    }
    validate_declared_artifact_count(object, artifact_count)?;
    let declared_textures = texture_table
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    if state.used_texture_names != declared_textures {
        return Err(PipelineError::new(
            "generated world texture table does not match material bindings",
        ));
    }
    Ok(Some(VerifiedWorldMaterialCatalog {
        artifact_count,
        binding_count: state.binding_count,
        slot_count: state.slot_count,
        master_family_count: state.families.len(),
    }))
}

struct WorldMaterialVerificationState {
    artifact_paths: BTreeSet<String>,
    verified_texture_paths: BTreeSet<PathBuf>,
    used_texture_names: BTreeSet<String>,
    families: BTreeSet<String>,
    binding_count: usize,
    slot_count: usize,
}

impl WorldMaterialVerificationState {
    const fn new() -> Self {
        Self {
            artifact_paths: BTreeSet::new(),
            verified_texture_paths: BTreeSet::new(),
            used_texture_names: BTreeSet::new(),
            families: BTreeSet::new(),
            binding_count: 0,
            slot_count: 0,
        }
    }
}

fn verify_artifact(
    root: &Path,
    artifact: &Map<String, Value>,
    texture_table: &BTreeMap<String, (u64, String)>,
    state: &mut WorldMaterialVerificationState,
) -> PipelineOutcome<()> {
    let relative_path = required_string(artifact, "path")?;
    validate_relative_path(&relative_path)?;
    if Path::new(&relative_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        != Some("fbx")
        || !state.artifact_paths.insert(relative_path.clone())
    {
        return Err(PipelineError::new(
            "generated world FBX path is not canonical or unique",
        ));
    }
    let expected_size = required_u64(artifact, "bytes")?;
    let expected_digest = required_string(artifact, "sha256")?;
    validate_digest(&expected_digest)?;
    let fbx_path = root.join(&relative_path);
    verify_fbx_bytes(root, &fbx_path, expected_size, &expected_digest)?;

    let bindings = required_array(artifact, "material_bindings")?;
    let mut binding_sources = Vec::with_capacity(bindings.len());
    for binding in bindings {
        let binding = as_object(binding, "generated world material binding")?;
        let source = binding_source(binding)?;
        verify_binding_texture(
            root,
            &fbx_path,
            &source,
            texture_table,
            &mut state.verified_texture_paths,
        )?;
        if let Some(file_name) = source.texture_file_name.as_ref() {
            let _inserted = state.used_texture_names.insert(file_name.clone());
        }
        binding_sources.push(source);
    }
    let slots = required_array(artifact, "material_slots")?;
    let slot_sources = slots
        .iter()
        .map(slot_source)
        .collect::<PipelineOutcome<Vec<_>>>()?;
    let projection = WorldMaterialProjection::build(
        &binding_sources,
        &slot_sources,
    )
    .map_err(|_error| {
        PipelineError::new("generated world material projection is invalid")
    })?;
    for presentation in projection.presentations() {
        let _inserted = state
            .families
            .insert(presentation.raster.master.identity());
    }
    state.binding_count = state
        .binding_count
        .saturating_add(binding_sources.len());
    state.slot_count = state.slot_count.saturating_add(slot_sources.len());
    Ok(())
}

fn verify_fbx_bytes(
    root: &Path,
    path: &Path,
    expected_size: u64,
    expected_digest: &str,
) -> PipelineOutcome<()> {
    validate_regular_file(path, "generated world FBX")?;
    validate_ancestor_chain(root, path)?;
    let bytes = fs::read(path)
        .map_err(|error| io_error("read generated world FBX", &error))?;
    let size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if size != expected_size || digest_hex(&bytes) != expected_digest {
        return Err(PipelineError::new(
            "generated world FBX bytes do not match the catalog",
        ));
    }
    if binary_fbx_version(&bytes)? != FBX_VERSION {
        return Err(PipelineError::new(
            "generated world FBX version is not supported",
        ));
    }
    Ok(())
}

fn verified_texture_table(
    catalog: &Map<String, Value>,
) -> PipelineOutcome<BTreeMap<String, (u64, String)>> {
    let mut result = BTreeMap::new();
    for texture in required_array(catalog, "textures")? {
        let texture = as_object(texture, "generated world texture record")?;
        let file_name = required_string(texture, "file_name")?;
        let size = required_u64(texture, "bytes")?;
        let digest = required_string(texture, "sha256")?;
        validate_digest(&digest)?;
        if file_name != format!("texture-{digest}.png")
            || result.insert(file_name, (size, digest)).is_some()
        {
            return Err(PipelineError::new(
                "generated world texture table is not canonical or unique",
            ));
        }
    }
    Ok(result)
}

fn verify_binding_texture(
    root: &Path,
    fbx_path: &Path,
    binding: &WorldMaterialBindingSource,
    texture_table: &BTreeMap<String, (u64, String)>,
    verified: &mut BTreeSet<PathBuf>,
) -> PipelineOutcome<()> {
    let Some(file_name) = binding.texture_file_name.as_deref() else {
        return Ok(());
    };
    let Some(digest) = binding.texture_sha256.as_deref() else {
        return Err(PipelineError::new(
            "generated world material texture evidence is incomplete",
        ));
    };
    let Some((expected_size, table_digest)) = texture_table.get(file_name)
    else {
        return Err(PipelineError::new(
            "generated world material texture is absent from the texture table",
        ));
    };
    if table_digest != digest {
        return Err(PipelineError::new(
            "generated world material texture digest disagrees with the table",
        ));
    }
    let parent = fbx_path.parent().ok_or_else(|| {
        PipelineError::new("generated world FBX has no parent directory")
    })?;
    let path = parent.join("textures").join(file_name);
    if !verified.insert(path.clone()) {
        return Ok(());
    }
    validate_regular_file(&path, "generated world material texture")?;
    validate_ancestor_chain(root, &path)?;
    let bytes = fs::read(&path).map_err(|error| {
        io_error("read generated world material texture", &error)
    })?;
    let size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if size != *expected_size || digest_hex(&bytes) != digest {
        return Err(PipelineError::new(
            "generated world material texture bytes do not match the catalog",
        ));
    }
    if !bytes.starts_with(PNG_MAGIC) {
        return Err(PipelineError::new(
            "generated world material texture is not a PNG artifact",
        ));
    }
    Ok(())
}

fn binding_source(
    object: &Map<String, Value>,
) -> PipelineOutcome<WorldMaterialBindingSource> {
    let material_name = required_string(object, "material_name")?;
    let texture_file_name = optional_string(object, "texture_file_name")?;
    let texture_sha256 = optional_string(object, "texture_sha256")?;
    let binding_sha256 = required_string(object, "binding_sha256")?;
    let presentation_sha256 = required_string(object, "presentation_sha256")?;
    let base_color_rgba8 = rgba8(object.get("base_color_rgba8"))?;
    let semantics = semantics(object.get("semantics"))?;
    let source_shaders = required_array(object, "source_shaders")?;
    let raster = if source_shaders.is_empty() {
        validate_runtime_error_binding(
            &binding_sha256,
            &presentation_sha256,
            texture_file_name.as_deref(),
            texture_sha256.as_deref(),
            base_color_rgba8,
            semantics,
        )?;
        WorldMaterialRasterProjection::runtime_error()
    } else {
        unanimous_raster(source_shaders)?
    };
    Ok(WorldMaterialBindingSource {
        material_name,
        texture_file_name,
        texture_sha256,
        binding_sha256,
        presentation_sha256,
        base_color_rgba8,
        raster,
        semantics,
    })
}

fn validate_runtime_error_binding(
    binding_sha256: &str,
    presentation_sha256: &str,
    texture_file_name: Option<&str>,
    texture_sha256: Option<&str>,
    base_color_rgba8: [u8; 4],
    semantics: WorldMaterialSemantics,
) -> PipelineOutcome<()> {
    if binding_sha256 != RUNTIME_ERROR_BINDING_SHA256
        || presentation_sha256 != RUNTIME_ERROR_PRESENTATION_SHA256
        || texture_file_name.is_some()
        || texture_sha256.is_some()
        || base_color_rgba8 != [255, 255, 255, 255]
        || semantics != WorldMaterialSemantics::default()
    {
        return Err(PipelineError::new(
            "generated world missing-shader fallback identity drifted",
        ));
    }
    Ok(())
}

fn unanimous_raster(
    sources: &[Value],
) -> PipelineOutcome<WorldMaterialRasterProjection> {
    let mut expected_shader = None;
    let mut expected_raster = None;
    for source in sources {
        let source = as_object(source, "generated world source shader")?;
        let shader = source
            .get("shader")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                PipelineError::new(
                    "generated world source shader has no shader document",
                )
            })?;
        if expected_shader.is_some_and(|value| value != shader) {
            return Err(PipelineError::new(
                "generated world binding source shader documents disagree",
            ));
        }
        let raster = shader_raster(shader)?;
        if expected_raster.is_some_and(|value| value != raster) {
            return Err(PipelineError::new(
                "generated world binding source shader raster state disagrees",
            ));
        }
        expected_shader = Some(shader);
        expected_raster = Some(raster);
    }
    expected_raster.ok_or_else(|| {
        PipelineError::new("generated world binding has no source shader")
    })
}

fn shader_raster(
    shader: &Map<String, Value>,
) -> PipelineOutcome<WorldMaterialRasterProjection> {
    let platform_shader = required_string(shader, "platform_shader_name")?;
    let family = match platform_shader.as_str() {
        "simple\0\0" => WorldMaterialShaderFamily::Simple,
        "environment\0" => WorldMaterialShaderFamily::Environment,
        _ => {
            return Err(PipelineError::new(
                "generated world shader family is not reviewed",
            ));
        },
    };
    let mut params = BTreeMap::new();
    for parameter in required_array(shader, "params")? {
        let parameter = as_object(
            parameter,
            "generated world shader parameter",
        )?;
        let name = required_string(parameter, "param")?;
        if params.insert(name, parameter.get("value")).is_some() {
            return Err(PipelineError::new(
                "generated world shader contains a duplicate parameter",
            ));
        }
    }
    let blend = parameter_u8(&params, "BLMD")?;
    let alpha_test = parameter_u8(&params, concat!("AT", "ST"))?;
    let alpha_compare = parameter_u8(&params, concat!("AC", "MP"))?;
    let two_sided = parameter_u8(&params, "2SID")?;
    let lit = parameter_u8(&params, "LIT")?;
    let alpha_reference = params
        .get("ACTH")
        .copied()
        .flatten()
        .map(parameter_f32_bits)
        .transpose()?;
    WorldMaterialRasterProjection::from_pddi(
        family,
        blend,
        alpha_test,
        alpha_compare,
        alpha_reference,
        two_sided,
        lit,
    )
    .map_err(|_error| {
        PipelineError::new(
            "generated world shader raster state is not reviewed",
        )
    })
}

fn slot_source(value: &Value) -> PipelineOutcome<WorldMaterialSlotSource> {
    let object = as_object(value, "generated world material slot")?;
    Ok(WorldMaterialSlotSource {
        slot_name: required_string(object, "slot_name")?,
        source_material_name: required_string(object, "source_material_name")?,
        binding_sha256: required_string(object, "binding_sha256")?,
        presentation_sha256: required_string(object, "presentation_sha256")?,
        slot_presentation_sha256: required_string(
            object,
            "slot_presentation_sha256",
        )?,
        semantics: semantics(object.get("semantics"))?,
    })
}

fn semantics(value: Option<&Value>) -> PipelineOutcome<WorldMaterialSemantics> {
    let object = value.and_then(Value::as_object).ok_or_else(|| {
        PipelineError::new("generated world material semantics are missing")
    })?;
    Ok(WorldMaterialSemantics {
        transparent: required_bool(object, "transparent")?,
        glass: required_bool(object, "glass")?,
        mirror: required_bool(object, "mirror")?,
        reflective: required_bool(object, "reflective")?,
        light_emitter: required_bool(object, "light_emitter")?,
        visual_effect: required_bool(object, "visual_effect")?,
    })
}

fn rgba8(value: Option<&Value>) -> PipelineOutcome<[u8; 4]> {
    let array = value.and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new("generated world material color is missing")
    })?;
    let [red, green, blue, alpha] = array.as_slice() else {
        return Err(PipelineError::new(
            "generated world material color does not have four channels",
        ));
    };
    Ok([
        color_channel(red)?,
        color_channel(green)?,
        color_channel(blue)?,
        color_channel(alpha)?,
    ])
}

fn color_channel(value: &Value) -> PipelineOutcome<u8> {
    let value = value.as_u64().ok_or_else(|| {
        PipelineError::new("generated world material color is not unsigned")
    })?;
    u8::try_from(value).map_err(|_error| {
        PipelineError::new("generated world material color is out of range")
    })
}

fn validate_declared_artifact_count(
    catalog: &Map<String, Value>,
    actual: usize,
) -> PipelineOutcome<()> {
    let counts = catalog
        .get("counts")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new("generated world counts are missing")
        })?;
    let normal = required_u64(counts, "world_fbx_files")?;
    let review = required_u64(counts, "review_fbx_files")?;
    let declared = normal.checked_add(review).ok_or_else(|| {
        PipelineError::new("generated world FBX count overflows")
    })?;
    if declared != u64::try_from(actual).unwrap_or(u64::MAX) {
        return Err(PipelineError::new(
            "generated world FBX count is stale",
        ));
    }
    Ok(())
}

fn parameter_u8(
    params: &BTreeMap<String, Option<&Value>>,
    name: &str,
) -> PipelineOutcome<u8> {
    let value = params
        .get(name)
        .copied()
        .flatten()
        .and_then(Value::as_i64)
        .ok_or_else(|| {
            PipelineError::new("generated world shader parameter is missing")
        })?;
    u8::try_from(value).map_err(|_error| {
        PipelineError::new("generated world shader parameter is out of range")
    })
}

fn parameter_f32_bits(value: &Value) -> PipelineOutcome<u32> {
    let text = value.to_string();
    let value = text.parse::<f32>().map_err(|_error| {
        PipelineError::new("generated world alpha reference is not numeric")
    })?;
    if !value.is_finite() {
        return Err(PipelineError::new(
            "generated world alpha reference is not finite",
        ));
    }
    Ok(value.to_bits())
}

fn optional_object<'a>(
    object: &'a Map<String, Value>,
    field: &str,
) -> PipelineOutcome<Option<&'a Map<String, Value>>> {
    match object.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Object(value)) => Ok(Some(value)),
        Some(_) => Err(PipelineError::new(
            "generated world optional artifact is not an object",
        )),
    }
}

fn optional_string(
    object: &Map<String, Value>,
    field: &str,
) -> PipelineOutcome<Option<String>> {
    match object.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(PipelineError::new(
            "generated world optional string field is malformed",
        )),
    }
}

fn required_string(
    object: &Map<String, Value>,
    field: &str,
) -> PipelineOutcome<String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            PipelineError::new("generated world string field is missing")
        })
}

fn required_u64(
    object: &Map<String, Value>,
    field: &str,
) -> PipelineOutcome<u64> {
    object
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new("generated world unsigned field is missing")
        })
}

fn required_bool(
    object: &Map<String, Value>,
    field: &str,
) -> PipelineOutcome<bool> {
    object
        .get(field)
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            PipelineError::new("generated world boolean field is missing")
        })
}

fn required_array<'a>(
    object: &'a Map<String, Value>,
    field: &str,
) -> PipelineOutcome<&'a [Value]> {
    object
        .get(field)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| {
            PipelineError::new("generated world array field is missing")
        })
}

fn as_object<'a>(
    value: &'a Value,
    label: &str,
) -> PipelineOutcome<&'a Map<String, Value>> {
    value.as_object().ok_or_else(|| {
        PipelineError::new(format!("{label} must be a JSON object"))
    })
}

#[cfg(test)]
// jig-ignore-next-line: repository test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_world_material_catalog/tests.rs"]
mod tests;

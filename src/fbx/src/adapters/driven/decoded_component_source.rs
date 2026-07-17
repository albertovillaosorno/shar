// File:
//   - decoded_component_source.rs
// Path:
//   - src/fbx/src/adapters/driven/decoded_component_source.rs
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
//   - The fbx adapter boundary for adapters driven decoded component source.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when decoded component source contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another fbx module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Decoded component source rooted at one normalized package directory.
// - Description:
//   - Defines decoded component source data and behavior for fbx adapters
//   - driven.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - true
//   - Reason: src/fbx/src/adapters/driven/decoded_component_source.rs has 311
//   - effective lines after the required header and remains cohesive until a
//   - focused split lands.
//

//! Decoded component source rooted at one normalized package directory.
use std::fs;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local;
use serde::Deserialize;
use serde_json::Value;

use crate::domain::mesh::{
    MeshAsset, MeshError, PrimitiveGroup, triangulate_strip,
};
use crate::domain::scene::identity::is_portable_path_segment;
use crate::domain::texture::{MaterialBinding, MaterialBindingError};
use crate::ports::component_source::ComponentSource;

/// Maximum integer value of one packed PDDI color channel.
const MAX_COLOR_CHANNEL: f32 = 255.0;

/// Decoded component source rooted at one normalized package directory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecodedComponentSource {
    /// Normalized package root containing decoded component JSON.
    package_root: PathBuf,
    /// Output root used when material textures are copied for FBX export.
    texture_output_dir: PathBuf,
}

impl DecodedComponentSource {
    /// Create a component source from caller-provided package and output roots.
    #[must_use]
    pub fn new(
        package_root: impl Into<PathBuf>,
        texture_output_dir: impl Into<PathBuf>,
    ) -> Self {
        Self {
            package_root: package_root.into(),
            texture_output_dir: texture_output_dir.into(),
        }
    }

    /// Internal helper for the adapter implementation.
    fn component_path(
        &self,
        family: &str,
        member_id: &str,
        extension: &str,
    ) -> Result<PathBuf, DecodedComponentError> {
        component_path(
            &self.package_root,
            family,
            member_id,
            extension,
        )
    }

    /// Resolve one shader using an exact index-published external PNG source.
    ///
    /// # Errors
    ///
    /// Returns an error when the shader, texture identity, or staging operation
    /// violates the decoded material contract.
    pub fn resolve_material_with_external_texture(
        &self,
        shader_id: &str,
        texture_source: &Path,
    ) -> Result<MaterialBinding, DecodedComponentError> {
        resolve_material_from_source(
            &self.package_root,
            &self.texture_output_dir,
            shader_id,
            Some(texture_source),
        )
    }
}

impl ComponentSource for DecodedComponentSource {
    type Error = DecodedComponentError;

    /// Internal helper for the adapter implementation.
    fn load_mesh(
        &self,
        mesh_member_id: &str,
    ) -> Result<MeshAsset, Self::Error> {
        let path = self.component_path(
            "mesh",
            mesh_member_id,
            "json",
        )?;
        read_mesh(
            &path,
            mesh_member_id,
        )
    }

    /// Internal helper for the adapter implementation.
    fn resolve_material(
        &self,
        shader_id: &str,
    ) -> Result<MaterialBinding, Self::Error> {
        resolve_material(
            &self.package_root,
            &self.texture_output_dir,
            shader_id,
        )
    }
}

/// Build one component path from a validated stable member identity.
fn component_path(
    package_root: &Path,
    family: &str,
    member_id: &str,
    extension: &str,
) -> Result<PathBuf, DecodedComponentError> {
    if !is_single_path_segment(member_id) {
        return Err(
            DecodedComponentError::InvalidMemberId(member_id.to_owned()),
        );
    }
    Ok(
        package_root
            .join("components")
            .join(family)
            .join(format!("{member_id}.{extension}")),
    )
}

/// Resolve one shader JSON path, accepting deterministic fixed-width padding.
fn shader_component_path(
    package_root: &Path,
    shader_name: &str,
) -> Result<PathBuf, DecodedComponentError> {
    let direct = component_path(
        package_root,
        "shader",
        shader_name,
        "json",
    )?;
    if direct.is_file() {
        return Ok(direct);
    }
    let shader_dir = package_root
        .join("components")
        .join("shader");
    let entries = fs::read_dir(&shader_dir).map_err(
        |error| DecodedComponentError::Read {
            path: shader_dir
                .display()
                .to_string(),
            source: error.to_string(),
        },
    )?;
    let mut candidates = entries
        .map(
            |entry| {
                entry.map_err(
                    |error| DecodedComponentError::Read {
                        path: shader_dir
                            .display()
                            .to_string(),
                        source: error.to_string(),
                    },
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|entry| entry.path())
        .filter(
            |path| {
                path.extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(
                        |extension| extension.eq_ignore_ascii_case("json"),
                    )
            },
        )
        .filter(
            |path| {
                path.file_stem()
                    .and_then(|value| value.to_str())
                    .and_then(|stem| stem.strip_prefix(shader_name))
                    .is_some_and(
                        |padding| {
                            !padding.is_empty()
                                && padding
                                    .chars()
                                    .all(|character| character == '_')
                        },
                    )
            },
        )
        .collect::<Vec<_>>();
    candidates.sort();
    match candidates.as_slice() {
        [candidate] => Ok(candidate.clone()),
        [] => Ok(direct),
        _ => Err(
            DecodedComponentError::AmbiguousShaderMember {
                shader: shader_name.to_owned(),
                candidates: candidates
                    .iter()
                    .map(
                        |path| {
                            path.file_name()
                                .and_then(|value| value.to_str())
                                .unwrap_or_default()
                                .to_owned()
                        },
                    )
                    .collect(),
            },
        ),
    }
}

/// Return whether a stable identity is exactly one normal path segment.
fn is_single_path_segment(value: &str) -> bool {
    is_portable_path_segment(value)
}

/// Read one decoded mesh from an explicit component path.
#[expect(
    clippy::too_many_lines,
    reason = "Strict mesh schema validation remains one atomic decode \
              boundary."
)]
pub(super) fn read_mesh(
    path: &Path,
    requested_id: &str,
) -> Result<MeshAsset, DecodedComponentError> {
    let decoded: DecodedMesh = read_json(path)?;
    if decoded.schema != "mesh" {
        return Err(
            DecodedComponentError::Mesh(
                MeshError::UnsupportedSchema(decoded.schema),
            ),
        );
    }
    let decoded_name = decoded_mesh_identity(&decoded.name);
    if decoded_name != requested_id {
        return Err(
            DecodedComponentError::MeshIdentityMismatch {
                requested: requested_id.to_owned(),
                decoded: decoded.name,
            },
        );
    }
    if decoded
        .primitive_group_count
        .is_some_and(
            |declared| {
                usize::try_from(declared)
                    != Ok(
                        decoded
                            .prim_groups
                            .len(),
                    )
            },
        )
    {
        return Err(
            DecodedComponentError::MeshEvidence(
                format!(
                    "mesh {decoded_name} primitive-group count does not match \
                     decoded groups"
                ),
            ),
        );
    }
    let groups = decoded
        .prim_groups
        .into_iter()
        .enumerate()
        .map(
            |(index, group)| {
                if group
                    .vertex_count
                    .is_some_and(
                        |declared| {
                            usize::try_from(declared)
                                != Ok(
                                    group
                                        .positions
                                        .len(),
                                )
                        },
                    )
                {
                    return Err(
                        DecodedComponentError::MeshEvidence(
                            format!(
                                "mesh {decoded_name} group {index} vertex \
                                 count mismatch"
                            ),
                        ),
                    );
                }
                if group
                    .index_count
                    .is_some_and(
                        |declared| {
                            usize::try_from(declared)
                                != Ok(
                                    group
                                        .indices
                                        .len(),
                                )
                        },
                    )
                {
                    return Err(
                        DecodedComponentError::MeshEvidence(
                            format!(
                                "mesh {decoded_name} group {index} index \
                                 count mismatch"
                            ),
                        ),
                    );
                }
                if let Some(channel) = group
                    .uvs
                    .iter()
                    .find(|channel| channel.channel != 0)
                {
                    return Err(
                        DecodedComponentError::UnsupportedUvChannel {
                            group: index,
                            channel: channel.channel,
                        },
                    );
                }
                let mut channel_zero = group
                    .uvs
                    .into_iter()
                    .filter(|channel| channel.channel == 0);
                let first_channel = channel_zero.next();
                if channel_zero
                    .next()
                    .is_some()
                {
                    return Err(
                        DecodedComponentError::DuplicateUvChannel {
                            group: index,
                            channel: 0,
                        },
                    );
                }
                let uvs = match first_channel {
                    Some(channel)
                        if channel
                            .coords
                            .is_empty() =>
                    {
                        return Err(
                            DecodedComponentError::EmptyUvChannel {
                                group: index,
                                channel: 0,
                            },
                        );
                    }
                    Some(channel) => channel.coords,
                    None => Vec::new(),
                };
                let triangles = match group.prim_type {
                    0 => group.indices,
                    1 => triangulate_strip(&group.indices)
                        .map_err(DecodedComponentError::Mesh)?
                        .into_iter()
                        .flatten()
                        .collect(),
                    other => {
                        return Err(
                            DecodedComponentError::MeshEvidence(
                                format!(
                                    "mesh {decoded_name} group {index} uses \
                                     unsupported primitive type {other}"
                                ),
                            ),
                        );
                    }
                };
                let normals = group.normals;
                let colors = group
                    .colours
                    .into_iter()
                    .map(decode_vertex_color)
                    .collect::<Vec<_>>();
                PrimitiveGroup::new(
                    index,
                    decoded_material_identity(&group.shader),
                    group.positions,
                    uvs,
                    &triangles,
                )
                .and_then(
                    |primitive_group| {
                        if normals.is_empty() {
                            Ok(primitive_group)
                        } else {
                            primitive_group.with_normals(normals)
                        }
                    },
                )
                .and_then(
                    |primitive_group| {
                        if colors.is_empty() {
                            Ok(primitive_group)
                        } else {
                            primitive_group.with_colors(colors)
                        }
                    },
                )
                .map_err(DecodedComponentError::Mesh)
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    MeshAsset::new(
        decoded_name,
        groups,
    )
    .map_err(DecodedComponentError::Mesh)
}

/// Internal helper for the adapter implementation.
fn resolve_material(
    package_root: &Path,
    output_texture_dir: &Path,
    shader_name: &str,
) -> Result<MaterialBinding, DecodedComponentError> {
    resolve_material_from_source(
        package_root,
        output_texture_dir,
        shader_name,
        None,
    )
}

/// Resolve and stage one material with an optional index-published texture.
fn resolve_material_from_source(
    package_root: &Path,
    output_texture_dir: &Path,
    shader_name: &str,
    external_texture_source: Option<&Path>,
) -> Result<MaterialBinding, DecodedComponentError> {
    let shader_path = shader_component_path(
        package_root,
        shader_name,
    )?;
    let shader: DecodedShader = read_json(&shader_path)?;
    ensure_shader_evidence(
        &shader,
        shader_name,
    )?;
    let material_name = decoded_material_identity(&shader.name);
    let Some(texture_reference) = texture_name(&shader)? else {
        return MaterialBinding::new(
            material_name,
            None,
        )
        .map_err(DecodedComponentError::Material);
    };
    let texture_stem = texture_stem(&texture_reference)?;
    let expected_file_name = format!("{texture_stem}.png");
    let direct_source = package_root
        .join("components")
        .join("texture")
        .join(&expected_file_name);
    let local_source = local_texture_source(
        package_root,
        &texture_reference,
        &direct_source,
    )?;
    let source = if let Some(local_source) = local_source {
        local_source
    } else if let Some(external_source) = external_texture_source {
        let external_file_name = external_source
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(
                || {
                    DecodedComponentError::InvalidTextureName(
                        external_source
                            .display()
                            .to_string(),
                    )
                },
            )?;
        let accepted_alias = expected_file_name == "char_swatches.png"
            && external_file_name == "char_swatches_lit.png";
        if external_file_name != expected_file_name && !accepted_alias {
            return Err(
                DecodedComponentError::ExternalTextureMismatch {
                    expected: expected_file_name,
                    found: external_file_name.to_owned(),
                },
            );
        }
        if !external_source.is_file() {
            return Err(
                DecodedComponentError::MissingTexture {
                    shader: material_name,
                    texture: texture_reference,
                    searched: external_source
                        .display()
                        .to_string(),
                },
            );
        }
        external_source.to_path_buf()
    } else {
        return Err(
            DecodedComponentError::MissingTexture {
                shader: material_name,
                texture: texture_reference,
                searched: direct_source
                    .display()
                    .to_string(),
            },
        );
    };
    stage_texture_binding(
        output_texture_dir,
        &material_name,
        &source,
    )
}

/// Resolve one local texture by direct file name or exact package-ledger name.
fn local_texture_source(
    package_root: &Path,
    texture_reference: &str,
    direct_source: &Path,
) -> Result<Option<PathBuf>, DecodedComponentError> {
    if direct_source.is_file() {
        return Ok(Some(direct_source.to_path_buf()));
    }
    let manifest = package_root.join("components.jsonl");
    if !manifest.is_file() {
        return Ok(None);
    }
    let text = local::read_utf8(&manifest).map_err(
        |source| DecodedComponentError::Read {
            path: manifest
                .display()
                .to_string(),
            source: source.to_string(),
        },
    )?;
    let expected = texture_reference
        .trim_end_matches('\u{0}')
        .trim();
    let mut candidates = Vec::new();
    for line in text.lines() {
        let value: Value = serde_json::from_str(line).map_err(
            |source| DecodedComponentError::Parse {
                path: manifest
                    .display()
                    .to_string(),
                source: source.to_string(),
            },
        )?;
        if value
            .get("kind")
            .and_then(Value::as_str)
            != Some("texture")
        {
            continue;
        }
        let Some(name) = value
            .get("name")
            .and_then(Value::as_str)
        else {
            continue;
        };
        if name
            .trim_end_matches('\u{0}')
            .trim()
            != expected
        {
            continue;
        }
        let path = value
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(
                || {
                    DecodedComponentError::InvalidTextureReference(
                        name.to_owned(),
                    )
                },
            )?;
        let file_name = path
            .strip_prefix("texture/")
            .filter(|member| is_single_path_segment(member))
            .ok_or_else(
                || {
                    DecodedComponentError::InvalidTextureReference(
                        path.to_owned(),
                    )
                },
            )?;
        candidates.push(
            package_root
                .join("components")
                .join("texture")
                .join(file_name),
        );
    }
    candidates.sort();
    candidates.dedup();
    match candidates.as_slice() {
        [] => Ok(None),
        [candidate] if candidate.is_file() => Ok(Some(candidate.clone())),
        [..] if candidates.len() > 1 => Err(
            DecodedComponentError::AmbiguousTextureMember {
                texture: expected.to_owned(),
                candidates: candidates
                    .iter()
                    .map(
                        |path| {
                            path.file_name()
                                .and_then(|value| value.to_str())
                                .unwrap_or_default()
                                .to_owned()
                        },
                    )
                    .collect(),
            },
        ),
        _ => Ok(None),
    }
}

/// Copy one validated PNG into the FBX texture staging directory.
fn stage_texture_binding(
    output_texture_dir: &Path,
    shader_name: &str,
    source: &Path,
) -> Result<MaterialBinding, DecodedComponentError> {
    local::create_dir_all(output_texture_dir).map_err(
        |error| DecodedComponentError::CreateDir {
            path: output_texture_dir
                .display()
                .to_string(),
            source: error.to_string(),
        },
    )?;
    let file_name = source
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(
            || {
                DecodedComponentError::InvalidTextureName(
                    source
                        .display()
                        .to_string(),
                )
            },
        )?
        .to_owned();
    let target = output_texture_dir.join(&file_name);
    if source != target {
        let _bytes_copied = fs::copy(
            source, &target,
        )
        .map_err(
            |error| DecodedComponentError::CopyTexture {
                from: source
                    .display()
                    .to_string(),
                to: target
                    .display()
                    .to_string(),
                source: error.to_string(),
            },
        )?;
    }
    MaterialBinding::new(
        shader_name,
        Some(file_name),
    )
    .map_err(DecodedComponentError::Material)
}

/// Normalize one decoded texture reference into its staged PNG stem.
fn texture_stem(reference: &str) -> Result<&str, DecodedComponentError> {
    let normalized_reference = reference
        .trim_end_matches('\u{0}')
        .trim();
    if !is_single_path_segment(normalized_reference) {
        return Err(
            DecodedComponentError::InvalidTextureReference(
                reference.to_owned(),
            ),
        );
    }
    let stem = normalized_reference
        .rsplit_once('.')
        .filter(
            |(_, extension)| {
                extension.eq_ignore_ascii_case("bmp")
                    || extension.eq_ignore_ascii_case("png")
                    || extension.eq_ignore_ascii_case("tga")
            },
        )
        .map_or(
            normalized_reference,
            |(stem, _)| stem,
        );
    if stem.is_empty() {
        return Err(
            DecodedComponentError::InvalidTextureReference(
                reference.to_owned(),
            ),
        );
    }
    Ok(stem)
}

/// Reconstruct the portable member-file identity for a fixed-width shader.
fn shader_member_identity(value: &str) -> String {
    let unpadded = value.trim_end_matches('\u{0}');
    let padding_length = value
        .len()
        .saturating_sub(unpadded.len());
    let mut identity = String::with_capacity(value.len());
    identity.push_str(unpadded);
    identity.push_str(&"_".repeat(padding_length));
    identity
}

/// Normalize one fixed-width mesh identity for FBX domain use.
fn decoded_mesh_identity(value: &str) -> String {
    value
        .trim_end_matches('\u{0}')
        .trim()
        .to_owned()
}

/// Normalize one fixed-width shader identity for FBX domain use.
fn decoded_material_identity(value: &str) -> String {
    value
        .trim_end_matches('\u{0}')
        .trim()
        .to_owned()
}

/// Decode one PDDI `0xAARRGGBB` color into normalized FBX RGBA order.
fn decode_vertex_color(value: u32) -> [f32; 4] {
    let [
        alpha,
        red,
        green,
        blue,
    ] = value.to_be_bytes();
    [
        f32::from(red) / MAX_COLOR_CHANNEL,
        f32::from(green) / MAX_COLOR_CHANNEL,
        f32::from(blue) / MAX_COLOR_CHANNEL,
        f32::from(alpha) / MAX_COLOR_CHANNEL,
    ]
}

/// Ensure one decoded shader carries internally consistent source evidence.
fn ensure_shader_evidence(
    shader: &DecodedShader,
    shader_name: &str,
) -> Result<(), DecodedComponentError> {
    let logical_identity = decoded_material_identity(&shader.name);
    let member_identity = shader_member_identity(&shader.name);
    if logical_identity != shader_name && member_identity != shader_name {
        return Err(
            DecodedComponentError::ShaderIdentityMismatch {
                requested: shader_name.to_owned(),
                decoded: shader
                    .name
                    .clone(),
            },
        );
    }
    if let Some(schema) = shader
        .schema
        .as_deref()
        && schema != "shader"
    {
        return Err(
            DecodedComponentError::ShaderSchemaMismatch {
                shader: shader_name.to_owned(),
                schema: schema.to_owned(),
            },
        );
    }
    if shader.version != 0 {
        return Err(
            DecodedComponentError::UnsupportedShaderVersion {
                shader: shader_name.to_owned(),
                version: shader.version,
            },
        );
    }
    if matches!(
        shader.platform_shader_name.as_deref(),
        Some(pddi_name) if pddi_name.trim().is_empty()
    ) {
        return Err(
            DecodedComponentError::BlankPlatformShaderName {
                shader: shader_name.to_owned(),
            },
        );
    }
    if let Some(value @ 2..) = shader.translucency {
        return Err(
            DecodedComponentError::InvalidShaderTranslucency {
                shader: shader_name.to_owned(),
                value,
            },
        );
    }
    if let Some(declared) = shader.parameter_count {
        let actual = shader
            .params
            .len();
        if u32::try_from(actual) != Ok(declared) {
            return Err(
                DecodedComponentError::ShaderParameterCountMismatch {
                    shader: shader_name.to_owned(),
                    declared,
                    actual,
                },
            );
        }
    }
    Ok(())
}

/// Resolve the canonical texture parameter without order-dependent selection.
fn texture_name(
    shader: &DecodedShader
) -> Result<Option<String>, DecodedComponentError> {
    let mut texture_parameters = shader
        .params
        .iter()
        .filter(|param| param.kind == "texture" && param.param == "TEX");
    let texture_parameter = texture_parameters.next();
    if texture_parameters
        .next()
        .is_some()
    {
        return Err(
            DecodedComponentError::DuplicateTextureParameter {
                shader: shader
                    .name
                    .clone(),
            },
        );
    }
    let Some(parameter) = texture_parameter else {
        return Ok(None);
    };
    let value = parameter
        .value
        .as_str()
        .ok_or_else(
            || DecodedComponentError::InvalidTextureParameter {
                shader: shader
                    .name
                    .clone(),
            },
        )?;
    let normalized = value
        .trim_end_matches('\u{0}')
        .trim();
    if normalized.is_empty() {
        Ok(None)
    } else {
        Ok(Some(normalized.to_owned()))
    }
}

/// Internal helper for the adapter implementation.
fn read_json<T>(path: &Path) -> Result<T, DecodedComponentError>
where
    T: for<'de> Deserialize<'de>,
{
    let text = local::read_utf8(path).map_err(
        |source| DecodedComponentError::Read {
            path: path
                .display()
                .to_string(),
            source: source.to_string(),
        },
    )?;
    let json_text = text
        .strip_prefix('\u{feff}')
        .unwrap_or(&text);
    serde_json::from_str(json_text).map_err(
        |source| DecodedComponentError::Parse {
            path: path
                .display()
                .to_string(),
            source: source.to_string(),
        },
    )
}

/// Decoded component adapter error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecodedComponentError {
    /// Component identity was not a single safe path segment.
    InvalidMemberId(String),
    /// Failed to read a decoded component file.
    Read {
        /// Component path.
        path: String,
        /// IO error text.
        source: String,
    },
    /// Failed to parse a decoded component file.
    Parse {
        /// Component path.
        path: String,
        /// JSON error text.
        source: String,
    },
    /// Decoded mesh identity did not match the requested member identity.
    MeshIdentityMismatch {
        /// Mesh member identity requested by the caller.
        requested: String,
        /// Identity declared inside the decoded mesh.
        decoded: String,
    },
    /// Decoded mesh count or topology evidence was internally inconsistent.
    MeshEvidence(String),
    /// One primitive group declared a UV channel without coordinates.
    EmptyUvChannel {
        /// Primitive-group position in the decoded mesh.
        group: usize,
        /// Empty UV channel identity.
        channel: u32,
    },
    /// One primitive group declared a UV channel not supported by the domain.
    UnsupportedUvChannel {
        /// Primitive-group position in the decoded mesh.
        group: usize,
        /// Unsupported UV channel identity.
        channel: u32,
    },
    /// One primitive group repeated a UV channel identity.
    DuplicateUvChannel {
        /// Primitive-group position in the decoded mesh.
        group: usize,
        /// Repeated UV channel identity.
        channel: u32,
    },
    /// Mesh topology validation failed.
    Mesh(MeshError),
    /// Material binding validation failed.
    Material(MaterialBindingError),
    /// Texture output directory could not be created.
    CreateDir {
        /// Directory path.
        path: String,
        /// IO error text.
        source: String,
    },
    /// More than one fixed-width shader member matched one logical identity.
    AmbiguousShaderMember {
        /// Logical shader identity requested by the caller.
        shader: String,
        /// Matching padded member file names.
        candidates: Vec<String>,
    },
    /// Decoded shader identity did not match the requested member identity.
    ShaderIdentityMismatch {
        /// Shader member identity requested by the caller.
        requested: String,
        /// Identity declared inside the decoded shader.
        decoded: String,
    },
    /// Shader declared a schema other than the decoded shader contract.
    ShaderSchemaMismatch {
        /// Shader identity containing the invalid schema.
        shader: String,
        /// Explicit schema declared by the decoded source.
        schema: String,
    },
    /// Shader declared a version unsupported by material translation.
    UnsupportedShaderVersion {
        /// Shader identity containing the unsupported version.
        shader: String,
        /// Version declared by the decoded source.
        version: u32,
    },
    /// Shader declared an empty platform shader identity.
    BlankPlatformShaderName {
        /// Shader identity containing the empty platform binding.
        shader: String,
    },
    /// Shader declared a translucency flag outside the binary source domain.
    InvalidShaderTranslucency {
        /// Shader identity containing the invalid flag.
        shader: String,
        /// Flag declared by the decoded source.
        value: u32,
    },
    /// Shader parameter declaration differed from decoded parameters.
    ShaderParameterCountMismatch {
        /// Shader identity containing the contradictory declaration.
        shader: String,
        /// Count declared by the decoded source.
        declared: u32,
        /// Number of decoded shader parameters.
        actual: usize,
    },
    /// Shader declared the canonical texture parameter more than once.
    DuplicateTextureParameter {
        /// Shader identity containing duplicate parameters.
        shader: String,
    },
    /// Shader texture parameter did not contain a string identity.
    InvalidTextureParameter {
        /// Shader identity containing the invalid parameter.
        shader: String,
    },
    /// Shader referenced a texture that was not available to the adapter.
    MissingTexture {
        /// Shader name.
        shader: String,
        /// Texture name from shader evidence.
        texture: String,
        /// Adapter search path for diagnostics.
        searched: String,
    },
    /// Index-published texture file did not match the shader reference.
    ExternalTextureMismatch {
        /// PNG file name derived from the shader reference.
        expected: String,
        /// File name supplied by package-index evidence.
        found: String,
    },
    /// More than one published texture member matched one logical identity.
    AmbiguousTextureMember {
        /// Logical texture identity requested by shader evidence.
        texture: String,
        /// Matching published PNG member file names.
        candidates: Vec<String>,
    },
    /// Texture reference was not a single safe file identity.
    InvalidTextureReference(String),
    /// Texture file name was not valid UTF-8.
    InvalidTextureName(String),
    /// Texture staging failed.
    CopyTexture {
        /// Source texture path.
        from: String,
        /// Target texture path.
        to: String,
        /// IO error text.
        source: String,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedMesh {
    /// Decoded schema marker.
    schema: String,
    /// Mesh display name.
    name: String,
    /// Decoded mesh version retained for source validation.
    #[serde(
        default,
        rename = "version"
    )]
    _version: u32,
    /// Optional primitive-group count declared by the decoded source.
    #[serde(
        default,
        rename = "num_prim_groups"
    )]
    primitive_group_count: Option<u32>,
    /// Primitive groups carried by the decoded mesh.
    prim_groups: Vec<DecodedPrimitiveGroup>,
    /// Render status retained for schema compatibility.
    #[serde(
        default,
        rename = "render_status"
    )]
    _render_status: Value,
    /// Bounding box retained for source evidence.
    #[serde(
        default,
        rename = "bounding_box"
    )]
    _bounding_box: Value,
    /// Bounding sphere retained for source evidence.
    #[serde(
        default,
        rename = "bounding_sphere"
    )]
    _bounding_sphere: Value,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedPrimitiveGroup {
    /// Shader name referenced by this primitive group.
    shader: String,
    /// Vertex-shader identity retained for source compatibility.
    #[serde(
        default,
        rename = "vertex_shader"
    )]
    _vertex_shader: String,
    /// Primitive topology selector: triangle list or triangle strip.
    #[serde(default)]
    prim_type: u32,
    /// Vertex-format mask retained for source compatibility.
    #[serde(
        default,
        rename = "vertex_format"
    )]
    _vertex_format: u32,
    /// Optional declared vertex count.
    #[serde(
        default,
        rename = "vertex_count"
    )]
    vertex_count: Option<u32>,
    /// Optional declared index count.
    #[serde(
        default,
        rename = "index_count"
    )]
    index_count: Option<u32>,
    /// Matrix count retained for rigid-mesh compatibility.
    #[serde(
        default,
        rename = "matrix_count"
    )]
    _matrix_count: u32,
    /// Vertex positions decoded for this group.
    positions: Vec<[f32; 3]>,
    /// Packed normals retained for source evidence.
    #[serde(
        default,
        rename = "packed_normals"
    )]
    _packed_normals: Value,
    /// Optional per-vertex normals aligned with positions.
    #[serde(default)]
    normals: Vec<[f32; 3]>,
    /// Optional packed vertex colors in PDDI `0xAARRGGBB` order.
    #[serde(default)]
    colours: Vec<u32>,
    /// Primitive index stream decoded for this group.
    indices: Vec<u32>,
    #[serde(default)]
    /// UV channels decoded for this group.
    uvs: Vec<DecodedUvChannel>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedUvChannel {
    /// UV channel index.
    channel: u32,
    /// UV coordinates decoded for the channel.
    coords: Vec<[f32; 2]>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedShader {
    /// Optional decoded schema marker.
    #[serde(
        default,
        rename = "schema"
    )]
    schema: Option<String>,
    /// Shader name used by material lookup.
    name: String,
    /// Decoded shader version.
    #[serde(
        default,
        rename = "version"
    )]
    version: u32,
    /// Optional platform shader identity.
    #[serde(
        default,
        rename = "pddi_shader_name"
    )]
    platform_shader_name: Option<String>,
    /// Optional binary translucency flag.
    #[serde(
        default,
        rename = "has_translucency"
    )]
    translucency: Option<u32>,
    /// Optional vertex-needs mask retained for source evidence.
    #[serde(
        default,
        rename = "vertex_needs"
    )]
    _vertex_needs: Option<u32>,
    /// Optional vertex mask retained for source evidence.
    #[serde(
        default,
        rename = "vertex_mask"
    )]
    _vertex_mask: Option<u32>,
    /// Optional parameter count declared by the decoded source.
    #[serde(
        default,
        rename = "num_params"
    )]
    parameter_count: Option<u32>,
    #[serde(default)]
    /// Shader parameters decoded for material binding.
    params: Vec<DecodedShaderParam>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
/// Internal data shape for the adapter implementation.
struct DecodedShaderParam {
    /// Parameter value kind.
    kind: String,
    /// Parameter name.
    param: String,
    /// Parameter JSON value.
    value: Value,
}

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
//   - Compilation of normalized Pure3D sprite evidence into one raster
//     artifact.
// - Must-Not:
//   - Read sibling source images, infer ambiguous grids, or publish partial
//     output.
// - Allows:
//   - Read normalized sprite JSON and exact child DDS payloads in ledger order.
// - Split-When:
//   - Raster catalog publication or plan promotion gains an independent
//     lifecycle.
// - Merge-When:
//   - Another adapter owns the identical normalized sprite compilation
//     boundary.
// - Summary:
//   - Compile one normalized sprite package into a deterministic PNG artifact.
// - Description:
//   - Binds sprite and ordered image evidence to one revision before encoding.
// - Usage:
//   - Used by the generated UI-raster catalog publisher before Unreal planning.
// - Defaults:
//   - Missing, malformed, duplicated, or ambiguous evidence fails explicitly.
//

//! Normalized `Pure3D` sprite raster compiler.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::semantic_texture_png::{
    decode_png_bytes, encode_png_bytes,
};
use fbx::domain::texture::semantic::{Rgba8, RgbaImage};
use p3d::{
    DecodedRgbaImage, SpriteRasterLayout, assemble_sprite_rgba,
    decode_legacy_dds,
};
use schoenwald_filesystem::resolve_under;
use serde_json::{Value, json};
use shar_sha256::digest_hex;

use crate::domain::{
    PhaseThreePackageIndex, PipelineError, PipelineOutcome,
    UnrealUiRasterArtifactEvidence,
};

/// One complete generated sprite raster before filesystem publication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CompiledUiSpriteRaster {
    /// Stable semantic package identity.
    pub package_id: String,
    /// Canonical generated artifact filename.
    pub filename: String,
    /// Deterministic PNG bytes.
    pub png_bytes: Vec<u8>,
    /// SHA-256 of the generated PNG bytes.
    pub png_sha256: String,
    /// Revision binding the sprite JSON and ordered DDS children.
    pub source_revision: String,
    /// Logical raster width.
    pub width: u32,
    /// Logical raster height.
    pub height: u32,
    /// Number of source-owned DDS tiles consumed.
    pub tile_count: usize,
    source_component_paths: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LedgerRow {
    ordinal: usize,
    parent_ordinal: Option<usize>,
    kind: String,
    path: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SpriteMetadata {
    width: u32,
    height: u32,
    image_count: usize,
    blit_border: u32,
}

/// Compile every indexed UI-sprite package from canonical normalized evidence.
///
/// # Errors
///
/// Returns an error when indexed sprite/image members disagree with the
/// normalized component ledger or any package fails raster compilation.
pub(super) fn compile_ui_sprite_raster_catalog(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
) -> PipelineOutcome<Vec<CompiledUiSpriteRaster>> {
    let mut artifacts = Vec::new();
    for package in index.packages().iter().filter(|package| {
        package.category() == "ui-images"
            && package
                .members()
                .iter()
                .any(|member| member.source_chunk_kind == "sprite")
            && package
                .members()
                .iter()
                .any(|member| member.source_chunk_kind == "image")
            && package.members().iter().all(|member| {
                matches!(member.source_chunk_kind.as_str(), "sprite" | "image")
                    || (member.source_chunk_kind == "none"
                        && member.kind == "package-manifest"
                        && member.role.as_str() == "metadata"
                        && member.unit_type == "metadata")
            })
    }) {
        let package_root = resolve_normalized_package_root(
            extracted_root,
            &package.package_root,
        )?;
        let artifact =
            compile_ui_sprite_raster(&package.package_id, &package_root)?;
        let indexed_paths = package
            .members()
            .iter()
            .filter(|member| {
                matches!(member.source_chunk_kind.as_str(), "sprite" | "image")
            })
            .map(|member| member.path.as_str())
            .collect::<BTreeSet<_>>();
        let expected_paths = artifact
            .source_component_paths
            .iter()
            .map(|relative| {
                format!("{}/components/{relative}", package.package_root)
            })
            .collect::<BTreeSet<_>>();
        let expected_refs = expected_paths
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if indexed_paths != expected_refs {
            return Err(PipelineError::new(
                "UI sprite package index disagrees with normalized components",
            ));
        }
        artifacts.push(artifact);
    }
    Ok(artifacts)
}

fn resolve_normalized_package_root(
    extracted_root: &Path,
    published_root: &str,
) -> PipelineOutcome<PathBuf> {
    let root_name = extracted_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new(
                "UI sprite extracted root has no portable basename",
            )
        })?;
    let prefix = format!("{root_name}/");
    let relative = published_root.strip_prefix(&prefix).ok_or_else(|| {
        PipelineError::new(
            "UI sprite package root is outside extracted evidence",
        )
    })?;
    resolve_under(extracted_root, Path::new(relative)).map_err(|_error| {
        PipelineError::new("UI sprite package root escapes extracted evidence")
    })
}

/// Compile one normalized package root into one deterministic sprite PNG.
///
/// # Errors
///
/// Returns an error when the component ledger, sprite metadata, child DDS
/// payloads, raster assembly, or PNG encoding do not satisfy the closed
/// source-backed contract.
fn compile_ui_sprite_raster(
    package_id: &str,
    normalized_package_root: &Path,
) -> PipelineOutcome<CompiledUiSpriteRaster> {
    validate_package_id(package_id)?;
    let components = normalized_package_root.join("components");
    let ledger_path = normalized_package_root.join("components.jsonl");
    let ledger_text = fs::read_to_string(&ledger_path)
        .map_err(|error| io_error("read sprite component ledger", &error))?;
    let rows = parse_ledger(&ledger_text)?;
    let sprites = rows
        .iter()
        .filter(|row| row.kind == "sprite")
        .collect::<Vec<_>>();
    let [sprite] = sprites.as_slice() else {
        return Err(PipelineError::new(
            "UI sprite package must contain exactly one sprite component",
        ));
    };
    let sprite_path = resolve_component_path(&components, &sprite.path)?;
    let sprite_bytes = fs::read(&sprite_path)
        .map_err(|error| io_error("read normalized sprite metadata", &error))?;
    let metadata = parse_sprite_metadata(&sprite_bytes)?;
    let mut image_rows = rows
        .iter()
        .filter(|row| {
            row.kind == "image" && row.parent_ordinal == Some(sprite.ordinal)
        })
        .collect::<Vec<_>>();
    image_rows.sort_by_key(|row| row.ordinal);
    if image_rows.len() != metadata.image_count {
        return Err(PipelineError::new(
            "UI sprite image child count does not match normalized metadata",
        ));
    }
    if rows.iter().any(|row| {
        row.kind == "image" && row.parent_ordinal != Some(sprite.ordinal)
    }) {
        return Err(PipelineError::new(
            "UI sprite package contains an image outside the owning sprite",
        ));
    }
    let mut revision_preimage = format!(
        "package={package_id}\nsprite={}:{}\n",
        sprite.ordinal,
        digest_hex(&sprite_bytes),
    );
    let mut source_component_paths = vec![sprite.path.clone()];
    let mut tiles = Vec::with_capacity(image_rows.len());
    for row in image_rows {
        let path = resolve_component_path(&components, &row.path)?;
        let bytes = fs::read(&path).map_err(|error| {
            io_error("read normalized sprite image", &error)
        })?;
        writeln!(
            revision_preimage,
            "image={}:{}:{}",
            row.ordinal,
            row.path,
            digest_hex(&bytes),
        )
        .map_err(|_error| {
            PipelineError::new(
                "UI sprite provenance revision formatting failed",
            )
        })?;
        source_component_paths.push(row.path.clone());
        tiles.push(decode_legacy_dds(&bytes).map_err(|error| {
            PipelineError::new(format!("UI sprite DDS decode failed: {error}"))
        })?);
    }
    let raster = assemble_sprite_rgba(
        SpriteRasterLayout {
            width: metadata.width,
            height: metadata.height,
            blit_border: metadata.blit_border,
        },
        &tiles,
    )
    .map_err(|error| {
        PipelineError::new(format!("UI sprite raster assembly failed: {error}"))
    })?;
    let png_bytes = encode_raster_png(&raster)?;
    let png_sha256 = digest_hex(&png_bytes);
    Ok(CompiledUiSpriteRaster {
        package_id: package_id.to_owned(),
        filename: format!("{package_id}.png"),
        png_bytes,
        png_sha256,
        source_revision: digest_hex(revision_preimage.as_bytes()),
        width: raster.width,
        height: raster.height,
        tile_count: tiles.len(),
        source_component_paths,
    })
}

fn parse_ledger(text: &str) -> PipelineOutcome<Vec<LedgerRow>> {
    let mut rows = Vec::new();
    for line in text.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let value = serde_json::from_str::<Value>(line).map_err(|error| {
            PipelineError::new(format!(
                "UI sprite component ledger JSON failed: {error}"
            ))
        })?;
        let object = value.as_object().ok_or_else(|| {
            PipelineError::new(
                "UI sprite component ledger row is not an object",
            )
        })?;
        let ordinal = required_usize(object.get("ordinal"), "ordinal")?;
        let kind = required_string(object.get("kind"), "kind")?;
        let path = required_string(object.get("path"), "path")?;
        let parent_ordinal = match object.get("parent_ordinal") {
            None | Some(Value::Null) => None,
            Some(value) => Some(required_usize(Some(value), "parent_ordinal")?),
        };
        rows.push(LedgerRow {
            ordinal,
            parent_ordinal,
            kind,
            path,
        });
    }
    let mut ordinals = BTreeSet::new();
    for row in &rows {
        if !ordinals.insert(row.ordinal) {
            return Err(PipelineError::new(
                "UI sprite component ledger duplicates an ordinal",
            ));
        }
    }
    Ok(rows)
}

fn parse_sprite_metadata(bytes: &[u8]) -> PipelineOutcome<SpriteMetadata> {
    let value = serde_json::from_slice::<Value>(bytes).map_err(|error| {
        PipelineError::new(format!("normalized sprite JSON failed: {error}"))
    })?;
    let object = value.as_object().ok_or_else(|| {
        PipelineError::new("normalized sprite metadata is not an object")
    })?;
    let size = object
        .get("image_size")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new("normalized sprite image_size is missing")
        })?;
    let [width, height] = size.as_slice() else {
        return Err(PipelineError::new(
            "normalized sprite image_size must contain two values",
        ));
    };
    Ok(SpriteMetadata {
        width: required_u32(Some(width), "image width")?,
        height: required_u32(Some(height), "image height")?,
        image_count: required_usize(object.get("image_count"), "image_count")?,
        blit_border: required_u32(object.get("blit_border"), "blit_border")?,
    })
}

fn encode_raster_png(raster: &DecodedRgbaImage) -> PipelineOutcome<Vec<u8>> {
    let (chunks, remainder) = raster.rgba.as_chunks::<4>();
    let pixels = chunks
        .iter()
        .map(|channels| {
            let red = channels.first().copied().unwrap_or_default();
            let green = channels.get(1).copied().unwrap_or_default();
            let blue = channels.get(2).copied().unwrap_or_default();
            let alpha = channels.get(3).copied().unwrap_or_default();
            Rgba8::new(red, green, blue, alpha)
        })
        .collect::<Vec<_>>();
    if !remainder.is_empty() {
        return Err(PipelineError::new(
            "assembled UI sprite RGBA storage is not pixel-aligned",
        ));
    }
    let image = RgbaImage::new(raster.width, raster.height, pixels).map_err(
        |error| {
            PipelineError::new(format!(
                "UI sprite RGBA image failed: {error:?}"
            ))
        },
    )?;
    encode_png_bytes(&image).map_err(|error| {
        PipelineError::new(format!("UI sprite PNG encode failed: {error:?}"))
    })
}

fn resolve_component_path(
    root: &Path,
    published: &str,
) -> PipelineOutcome<PathBuf> {
    if published.is_empty() || published.contains(char::from(92)) {
        return Err(PipelineError::new(
            "UI sprite component path is not portable",
        ));
    }
    resolve_under(root, Path::new(published)).map_err(|_error| {
        PipelineError::new("UI sprite component path escapes its package")
    })
}

fn validate_package_id(package_id: &str) -> PipelineOutcome<()> {
    if package_id.is_empty()
        || package_id
            .chars()
            .any(|character| !matches!(character, 'a'..='z' | '0'..='9' | '-'))
    {
        return Err(PipelineError::new(
            "UI sprite package id is not a portable generated filename stem",
        ));
    }
    Ok(())
}

fn required_string(
    value: Option<&Value>,
    field: &str,
) -> PipelineOutcome<String> {
    value
        .and_then(Value::as_str)
        .filter(|item| !item.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| {
            PipelineError::new(format!("UI sprite {field} is missing"))
        })
}

fn required_usize(
    value: Option<&Value>,
    field: &str,
) -> PipelineOutcome<usize> {
    let raw = value.and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new(format!(
            "UI sprite {field} is not a nonnegative integer"
        ))
    })?;
    usize::try_from(raw).map_err(|error| {
        PipelineError::new(format!("UI sprite {field} exceeds usize: {error}"))
    })
}

fn required_u32(value: Option<&Value>, field: &str) -> PipelineOutcome<u32> {
    let raw = value.and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new(format!(
            "UI sprite {field} is not a nonnegative integer"
        ))
    })?;
    u32::try_from(raw).map_err(|error| {
        PipelineError::new(format!("UI sprite {field} exceeds u32: {error}"))
    })
}

fn io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed ({:?})", error.kind()))
}

const CATALOG_SCHEMA: &str = "shar-schoenwald.ui-raster-catalog.v1";
const CATALOG_FILE: &str = "catalog.jsonl";
const RASTER_DIR: &str = "rasters";
const LOGICAL_ROOT: &str = "ui-raster-assets";
const PNG_MAGIC: &[u8] = &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

/// Compile, verify, and atomically publish the complete UI-sprite raster
/// catalog.
///
/// # Errors
///
/// Returns an error when compilation, staging, catalog verification,
/// publication, or read-back verification is incomplete or inconsistent.
pub(super) fn publish_complete_ui_sprite_raster_catalog(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
    output_root: &Path,
) -> PipelineOutcome<Vec<UnrealUiRasterArtifactEvidence>> {
    let compiled = compile_ui_sprite_raster_catalog(index, extracted_root)?;
    let (staging, backup) = transaction_paths(output_root)?;
    ensure_absent(&staging, "UI raster staging")?;
    ensure_absent(&backup, "UI raster backup")?;
    ensure_output_parent(output_root)?;
    fs::create_dir_all(staging.join(RASTER_DIR))
        .map_err(|error| io_error("create UI raster staging", &error))?;

    let staged = match stage_catalog(&compiled, &staging) {
        Ok(value) => value,
        Err(error) => {
            let _cleanup = remove_generated_directory(&staging);
            return Err(error);
        }
    };
    let had_output = match fs::symlink_metadata(output_root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => return Err(io_error("inspect UI raster output", &error)),
        Ok(metadata) => {
            validate_directory_metadata(&metadata, "UI raster output")?;
            fs::rename(output_root, &backup).map_err(|error| {
                io_error("back up UI raster output", &error)
            })?;
            true
        }
    };
    if let Err(error) = fs::rename(&staging, output_root) {
        let publish_error = io_error("publish UI raster catalog", &error);
        let rollback = if had_output {
            fs::rename(&backup, output_root)
        } else {
            Ok(())
        };
        let _cleanup = remove_generated_directory(&staging);
        return match rollback {
            Ok(()) => Err(publish_error),
            Err(rollback_error) => Err(PipelineError::new(format!(
                concat!(
                    "{}; restore previous UI raster catalog ",
                    "failed ({:?})",
                ),
                publish_error,
                rollback_error.kind(),
            ))),
        };
    }
    let published = verified_ui_sprite_raster_catalog(output_root);
    match published {
        Ok(Some(evidence)) if evidence == staged => {
            if had_output {
                remove_generated_directory(&backup)?;
            }
            Ok(evidence)
        }
        Ok(Some(_evidence)) => {
            rollback_publication(output_root, &backup, had_output)?;
            Err(PipelineError::new(
                "published UI raster catalog changed during read-back",
            ))
        }
        Ok(None) => {
            rollback_publication(output_root, &backup, had_output)?;
            Err(PipelineError::new(
                "published UI raster catalog disappeared during read-back",
            ))
        }
        Err(error) => {
            rollback_publication(output_root, &backup, had_output)?;
            Err(error)
        }
    }
}

fn stage_catalog(
    compiled: &[CompiledUiSpriteRaster],
    staging: &Path,
) -> PipelineOutcome<Vec<UnrealUiRasterArtifactEvidence>> {
    let mut artifacts = compiled.to_vec();
    artifacts.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    let mut lines = Vec::with_capacity(artifacts.len().saturating_add(1));
    lines.push(json!({
        "schema": CATALOG_SCHEMA,
        "record_type": "header",
        "status": "complete",
        "package_count": artifacts.len(),
    }));
    let mut expected = Vec::with_capacity(artifacts.len());
    for artifact in artifacts {
        let relative_path = format!("{RASTER_DIR}/{}", artifact.filename);
        let destination = staging.join(&relative_path);
        fs::write(&destination, &artifact.png_bytes)
            .map_err(|error| io_error("write UI raster artifact", &error))?;
        let size_bytes =
            u64::try_from(artifact.png_bytes.len()).unwrap_or(u64::MAX);
        lines.push(json!({
            "schema": CATALOG_SCHEMA,
            "record_type": "raster",
            "package_id": artifact.package_id,
            "path": relative_path,
            "size_bytes": size_bytes,
            "sha256": artifact.png_sha256,
            "source_revision": artifact.source_revision,
            "width": artifact.width,
            "height": artifact.height,
            "tile_count": artifact.tile_count,
        }));
        expected.push(UnrealUiRasterArtifactEvidence {
            package_id: artifact.package_id,
            path: format!("{LOGICAL_ROOT}/{relative_path}"),
            size_bytes,
            sha256: artifact.png_sha256,
            source_revision: artifact.source_revision,
            width: artifact.width,
            height: artifact.height,
            tile_count: artifact.tile_count,
        });
    }
    let mut rendered = String::new();
    for line in lines {
        writeln!(
            rendered,
            "{}",
            serde_json::to_string(&line).map_err(|error| {
                PipelineError::new(format!(
                    "UI raster catalog JSON failed: {error}"
                ))
            })?,
        )
        .map_err(|_error| {
            PipelineError::new("UI raster catalog formatting failed")
        })?;
    }
    fs::write(staging.join(CATALOG_FILE), rendered)
        .map_err(|error| io_error("write UI raster catalog", &error))?;
    let verified =
        verified_ui_sprite_raster_catalog(staging)?.ok_or_else(|| {
            PipelineError::new(
                "staged UI raster catalog disappeared during verification",
            )
        })?;
    if verified != expected {
        return Err(PipelineError::new(
            "staged UI raster catalog changed during verification",
        ));
    }
    Ok(verified)
}

/// Read and verify one complete generated UI-sprite raster catalog when
/// present.
fn verified_ui_sprite_raster_catalog(
    root: &Path,
) -> PipelineOutcome<Option<Vec<UnrealUiRasterArtifactEvidence>>> {
    let metadata = match fs::symlink_metadata(root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        }
        Err(error) => {
            return Err(io_error("inspect UI raster catalog root", &error));
        }
        Ok(metadata) => metadata,
    };
    validate_directory_metadata(&metadata, "UI raster catalog root")?;
    let catalog_path = root.join(CATALOG_FILE);
    validate_regular_file(&catalog_path, "UI raster catalog")?;
    let text = fs::read_to_string(&catalog_path)
        .map_err(|error| io_error("read UI raster catalog", &error))?;
    if !text.ends_with('\n') || text.contains('\r') {
        return Err(PipelineError::new(
            "UI raster catalog line endings are not canonical",
        ));
    }
    let mut lines = text.lines();
    let header = lines
        .next()
        .ok_or_else(|| PipelineError::new("UI raster catalog is empty"))?;
    let header = parse_object(header, "UI raster catalog header")?;
    require_exact_fields(
        &header,
        &["schema", "record_type", "status", "package_count"],
        "UI raster catalog header",
    )?;
    if required_string(header.get("schema"), "schema")? != CATALOG_SCHEMA
        || required_string(header.get("record_type"), "record_type")?
            != "header"
        || required_string(header.get("status"), "status")? != "complete"
    {
        return Err(PipelineError::new(
            "UI raster catalog header is not canonical",
        ));
    }
    let declared_count =
        required_u64(&header, "package_count", "UI raster catalog header")?;
    let mut evidence = Vec::new();
    let mut package_ids = BTreeSet::new();
    let mut expected_files = BTreeSet::new();
    for (offset, line) in lines.enumerate() {
        if line.trim().is_empty() {
            return Err(PipelineError::new(
                "UI raster catalog contains a blank row",
            ));
        }
        let label =
            format!("UI raster catalog line {}", offset.saturating_add(2));
        let row = parse_object(line, &label)?;
        require_exact_fields(
            &row,
            &[
                "schema",
                "record_type",
                "package_id",
                "path",
                "size_bytes",
                "sha256",
                "source_revision",
                "width",
                "height",
                "tile_count",
            ],
            &label,
        )?;
        if required_string(row.get("schema"), "schema")? != CATALOG_SCHEMA
            || required_string(row.get("record_type"), "record_type")?
                != "raster"
        {
            return Err(PipelineError::new(
                "UI raster catalog row is not canonical",
            ));
        }
        let package_id = required_string(row.get("package_id"), "package_id")?;
        validate_package_id(&package_id)?;
        if !package_ids.insert(package_id.clone()) {
            return Err(PipelineError::new(
                "UI raster catalog duplicates a package",
            ));
        }
        let relative_path = required_string(row.get("path"), "path")?;
        let expected_path = format!("{RASTER_DIR}/{package_id}.png");
        if relative_path != expected_path {
            return Err(PipelineError::new(
                "UI raster catalog path does not match its package identity",
            ));
        }
        if !expected_files.insert(relative_path.clone()) {
            return Err(PipelineError::new(
                "UI raster catalog duplicates a path",
            ));
        }
        let size_bytes = required_u64(&row, "size_bytes", &label)?;
        let sha256 = required_string(row.get("sha256"), "sha256")?;
        let source_revision =
            required_string(row.get("source_revision"), "source_revision")?;
        validate_digest(&sha256, "UI raster digest")?;
        validate_digest(&source_revision, "UI raster source revision")?;
        let width = required_u32(row.get("width"), "UI raster width")?;
        let height = required_u32(row.get("height"), "UI raster height")?;
        let tile_count =
            required_usize(row.get("tile_count"), "UI raster tile_count")?;
        if width == 0 || height == 0 || tile_count == 0 {
            return Err(PipelineError::new(
                "UI raster dimensions and tile count must be positive",
            ));
        }
        let path = root.join(&relative_path);
        validate_regular_file(&path, "UI raster artifact")?;
        let bytes = fs::read(&path)
            .map_err(|error| io_error("read UI raster artifact", &error))?;
        let actual_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if actual_size != size_bytes || digest_hex(&bytes) != sha256 {
            return Err(PipelineError::new(
                "UI raster artifact disagrees with its catalog identity",
            ));
        }
        if !bytes.starts_with(PNG_MAGIC) {
            return Err(PipelineError::new("UI raster artifact is not a PNG"));
        }
        let decoded = decode_png_bytes(&bytes).map_err(|error| {
            PipelineError::new(format!(
                "UI raster PNG verification failed: {error:?}"
            ))
        })?;
        if decoded.width() != width || decoded.height() != height {
            return Err(PipelineError::new(
                "UI raster PNG dimensions disagree with its catalog",
            ));
        }
        evidence.push(UnrealUiRasterArtifactEvidence {
            package_id,
            path: format!("{LOGICAL_ROOT}/{relative_path}"),
            size_bytes,
            sha256,
            source_revision,
            width,
            height,
            tile_count,
        });
    }
    if declared_count != u64::try_from(evidence.len()).unwrap_or(u64::MAX) {
        return Err(PipelineError::new(
            "UI raster catalog package count is stale",
        ));
    }
    verify_catalog_inventory(root, &expected_files)?;
    evidence.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    Ok(Some(evidence))
}

fn verify_catalog_inventory(
    root: &Path,
    expected_files: &BTreeSet<String>,
) -> PipelineOutcome<()> {
    let mut root_names = fs::read_dir(root)
        .map_err(|error| io_error("read UI raster catalog root", &error))?
        .map(|entry| entry.map(|value| value.file_name()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| io_error("read UI raster root entry", &error))?;
    root_names.sort();
    let expected_root = [
        std::ffi::OsString::from(CATALOG_FILE),
        std::ffi::OsString::from(RASTER_DIR),
    ];
    if root_names.as_slice() != expected_root.as_slice() {
        return Err(PipelineError::new(
            "UI raster catalog root inventory is not exact",
        ));
    }
    let raster_root = root.join(RASTER_DIR);
    let metadata = fs::symlink_metadata(&raster_root)
        .map_err(|error| io_error("inspect UI raster directory", &error))?;
    validate_directory_metadata(&metadata, "UI raster directory")?;
    let mut actual = BTreeSet::new();
    for entry in fs::read_dir(&raster_root)
        .map_err(|error| io_error("read UI raster directory", &error))?
    {
        let entry =
            entry.map_err(|error| io_error("read UI raster entry", &error))?;
        let metadata = fs::symlink_metadata(entry.path())
            .map_err(|error| io_error("inspect UI raster entry", &error))?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(PipelineError::new(
                "UI raster inventory contains a non-file",
            ));
        }
        let name = entry.file_name().into_string().map_err(|_name| {
            PipelineError::new("UI raster filename is not portable Unicode")
        })?;
        let relative = format!("{RASTER_DIR}/{name}");
        let _inserted = actual.insert(relative);
    }
    if &actual != expected_files {
        return Err(PipelineError::new(
            "UI raster artifact inventory is not exact",
        ));
    }
    Ok(())
}

fn parse_object(
    line: &str,
    label: &str,
) -> PipelineOutcome<serde_json::Map<String, Value>> {
    serde_json::from_str::<Value>(line)
        .map_err(|error| {
            PipelineError::new(format!("{label} JSON failed: {error}"))
        })?
        .as_object()
        .cloned()
        .ok_or_else(|| PipelineError::new(format!("{label} is not an object")))
}

fn require_exact_fields(
    object: &serde_json::Map<String, Value>,
    expected: &[&str],
    label: &str,
) -> PipelineOutcome<()> {
    let actual = object.keys().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(PipelineError::new(format!(
            "{label} fields are not exact"
        )));
    }
    Ok(())
}

fn required_u64(
    value: &serde_json::Map<String, Value>,
    field: &str,
    label: &str,
) -> PipelineOutcome<u64> {
    value.get(field).and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new(format!(
            "{label} {field} is not a nonnegative integer"
        ))
    })
}

fn validate_digest(value: &str, label: &str) -> PipelineOutcome<()> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(PipelineError::new(format!("{label} is not canonical")));
    }
    Ok(())
}

fn transaction_paths(
    output_root: &Path,
) -> PipelineOutcome<(PathBuf, PathBuf)> {
    let name = output_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new("UI raster output has no portable name")
        })?;
    let parent = output_root.parent().unwrap_or_else(|| Path::new("."));
    Ok((
        parent.join(format!(".{name}.complete-staging")),
        parent.join(format!(".{name}.complete-backup")),
    ))
}

fn ensure_output_parent(output_root: &Path) -> PipelineOutcome<()> {
    let parent = output_root
        .parent()
        .ok_or_else(|| PipelineError::new("UI raster output has no parent"))?;
    fs::create_dir_all(parent)
        .map_err(|error| io_error("create UI raster output parent", &error))?;
    let metadata = fs::symlink_metadata(parent)
        .map_err(|error| io_error("inspect UI raster output parent", &error))?;
    validate_directory_metadata(&metadata, "UI raster output parent")
}

fn ensure_absent(path: &Path, label: &str) -> PipelineOutcome<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => {
            Err(io_error("inspect UI raster transaction path", &error))
        }
        Ok(_metadata) => {
            Err(PipelineError::new(format!("{label} already exists")))
        }
    }
}

fn validate_directory_metadata(
    metadata: &fs::Metadata,
    label: &str,
) -> PipelineOutcome<()> {
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        Ok(())
    } else {
        Err(PipelineError::new(format!(
            "{label} must be a real directory"
        )))
    }
}

fn validate_regular_file(path: &Path, label: &str) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_error("inspect UI raster file", &error))?;
    if metadata.is_file() && !metadata.file_type().is_symlink() {
        Ok(())
    } else {
        Err(PipelineError::new(format!("{label} must be a real file")))
    }
}

fn remove_generated_directory(path: &Path) -> PipelineOutcome<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(io_error("inspect UI raster cleanup", &error)),
        Ok(metadata) => {
            validate_directory_metadata(&metadata, "UI raster cleanup target")?;
            fs::remove_dir_all(path).map_err(|error| {
                io_error("remove UI raster generated directory", &error)
            })
        }
    }
}

fn rollback_publication(
    output_root: &Path,
    backup: &Path,
    had_output: bool,
) -> PipelineOutcome<()> {
    remove_generated_directory(output_root)?;
    if had_output {
        fs::rename(backup, output_root).map_err(|error| {
            io_error("restore previous UI raster catalog", &error)
        })?;
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: canonical test module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/ui_sprite_raster_tests.rs"]
mod tests;

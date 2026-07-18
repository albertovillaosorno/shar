// File:
//   - source.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/vehicle_catalog/source.rs
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
//   - Fresh vehicle package extraction and exact source-component selection.
// - Must-Not:
//   - Assemble FBX geometry or publish final artifacts.
// - Allows:
//   - Original-game P3D extraction and car-family texture authority.
// - Summary:
//   - Vehicle source evidence resolver.
//
// Large file:
//   - false
//

//! Vehicle source evidence resolver.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::Value;
use shar_sha256::digest_hex;

use super::{VEHICLE_CATEGORY, VEHICLE_COMMON_SUBCATEGORY};
use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

/// One exact texture occurrence available across freshly extracted car packages.
#[derive(Clone, Debug, Eq, PartialEq)]
struct VehicleTextureSource {
    subcategory: String,
    path: PathBuf,
    sha256: String,
}

/// Cross-package texture lookup restricted to the generated car package family.
#[derive(Debug)]
pub(super) struct VehicleTextureAuthority {
    sources: BTreeMap<String, Vec<VehicleTextureSource>>,
}

/// Freshly extract every car package from the user-supplied original game tree.
pub(super) fn extract_vehicle_packages(
    index: &PhaseThreePackageIndex,
    game_root: &Path,
    normalized_root: &Path,
) -> Result<usize, PipelineError> {
    let mut count = 0_usize;
    for package in index
        .packages()
        .iter()
        .filter(|package| package.category == VEHICLE_CATEGORY)
    {
        let relative = relative_art_root(package)?;
        let source = game_root
            .join("art")
            .join(&relative)
            .with_extension("p3d");
        if !source.is_file() {
            return Err(PipelineError::new(format!(
                "vehicle source package is missing: {}",
                source.display()
            )));
        }
        p3d::write_lossless_package(
            &source,
            &normalized_root.join(relative),
        )
        .map_err(
            |error| {
                PipelineError::new(format!(
                    "vehicle extraction failed for {}: {error}",
                    package.package_id
                ))
            },
        )?;
        count = count
            .checked_add(1)
            .ok_or_else(|| PipelineError::new("vehicle package count overflowed"))?;
    }
    Ok(count)
}

/// Return one safe package root relative to extracted/art and game/art.
pub(super) fn relative_art_root(
    package: &PhaseThreePackageRow
) -> Result<PathBuf, PipelineError> {
    let relative = package
        .package_root
        .strip_prefix("extracted/art/")
        .ok_or_else(
            || {
                PipelineError::new(format!(
                    "vehicle package root is outside extracted/art: {}",
                    package.package_root
                ))
            },
        )?;
    let path = Path::new(relative);
    if path.is_absolute()
        || path
            .components()
            .any(|component| component == Component::ParentDir)
    {
        return Err(PipelineError::new(format!(
            "vehicle package root is not portable: {}",
            package.package_root
        )));
    }
    Ok(path.to_path_buf())
}

/// Select the authored render skeleton while excluding collision-volume rigs.
pub(super) fn select_vehicle_skeleton(
    package_root: &Path,
    vehicle: &str,
) -> Result<PathBuf, PipelineError> {
    let directory = package_root.join("components").join("skeleton");
    let mut candidates = json_files(&directory)?
        .into_iter()
        .filter_map(
            |path| {
                decoded_name(&path)
                    .ok()
                    .filter(|name| !is_collision_volume_identity(name))
                    .map(|name| (path, name))
            },
        )
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.0.cmp(&right.0));
    if candidates.len() == 1 {
        return Ok(candidates.remove(0).0);
    }
    let target = identity_key(vehicle);
    let matches = candidates
        .into_iter()
        .filter(|(_path, name)| identity_key(name) == target)
        .collect::<Vec<_>>();
    match matches.as_slice() {
        [(path, _name)] => Ok(path.clone()),
        [] => Err(PipelineError::new(format!(
            "vehicle {vehicle} has no unique render skeleton"
        ))),
        _ => Err(PipelineError::new(format!(
            "vehicle {vehicle} has ambiguous render skeletons"
        ))),
    }
}

/// Select the one authored vehicle composite drawable.
pub(super) fn select_vehicle_composite(
    package_root: &Path,
    vehicle: &str,
) -> Result<PathBuf, PipelineError> {
    let directory = package_root
        .join("components")
        .join("composite_drawable");
    let candidates = json_files(&directory)?;
    match candidates.as_slice() {
        [path] => Ok(path.clone()),
        [] => Err(PipelineError::new(format!(
            "vehicle {vehicle} has no composite drawable"
        ))),
        _ => {
            let target = identity_key(vehicle);
            let matches = candidates
                .into_iter()
                .filter(
                    |path| {
                        decoded_name(path)
                            .is_ok_and(|name| identity_key(&name) == target)
                    },
                )
                .collect::<Vec<_>>();
            match matches.as_slice() {
                [path] => Ok(path.clone()),
                _ => Err(PipelineError::new(format!(
                    "vehicle {vehicle} has ambiguous composite drawables"
                ))),
            }
        }
    }
}

/// Resolve every exact render-mesh path from generated package membership.
pub(super) fn vehicle_mesh_paths(
    package: &PhaseThreePackageRow,
    package_root: &Path,
) -> Result<Vec<PathBuf>, PipelineError> {
    let mut paths = package
        .members()
        .iter()
        .filter(
            |member| {
                member.kind == "p3d-mesh" && member.source_chunk_kind == "mesh"
            },
        )
        .map(
            |member| {
                let file_name = Path::new(&member.path)
                    .file_name()
                    .ok_or_else(
                        || PipelineError::new("vehicle mesh member has no file name"),
                    )?;
                Ok(package_root
                    .join("components")
                    .join("mesh")
                    .join(file_name))
            },
        )
        .collect::<Result<Vec<_>, PipelineError>>()?;
    paths.sort();
    paths.dedup();
    if paths.is_empty() {
        return Err(PipelineError::new(format!(
            "vehicle package {} has no render meshes",
            package.package_id
        )));
    }
    if let Some(path) = paths.iter().find(|path| !path.is_file()) {
        return Err(PipelineError::new(format!(
            "vehicle render mesh is missing: {}",
            path.display()
        )));
    }
    Ok(paths)
}

/// Return sorted JSON files from one optional component directory.
fn json_files(directory: &Path) -> Result<Vec<PathBuf>, PipelineError> {
    files_with_extension(directory, "json")
}

/// Return sorted PNG files from one optional component directory.
pub(super) fn png_files(directory: &Path) -> Result<Vec<PathBuf>, PipelineError> {
    files_with_extension(directory, "png")
}

/// Return sorted files with one extension from an optional directory.
fn files_with_extension(
    directory: &Path,
    extension: &str,
) -> Result<Vec<PathBuf>, PipelineError> {
    if !directory.is_dir() {
        return Ok(Vec::new());
    }
    let mut files = fs::read_dir(directory)
        .map_err(|error| PipelineError::new(error.to_string()))?
        .map(
            |entry| {
                entry
                    .map(|value| value.path())
                    .map_err(|error| PipelineError::new(error.to_string()))
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    files.retain(
        |path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|value| value.eq_ignore_ascii_case(extension))
        },
    );
    files.sort();
    Ok(files)
}

/// Read one decoded component name and remove fixed-width null padding.
pub(super) fn decoded_name(path: &Path) -> Result<String, PipelineError> {
    let value: Value = serde_json::from_slice(
        &fs::read(path).map_err(|error| PipelineError::new(error.to_string()))?,
    )
    .map_err(|error| PipelineError::new(error.to_string()))?;
    let name = value
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(
            || PipelineError::new(format!("component has no name: {}", path.display())),
        )?;
    Ok(name.trim_end_matches('\0').trim().to_owned())
}

/// Return whether one skeleton identity belongs only to collision-volume data.
fn is_collision_volume_identity(value: &str) -> bool {
    value
        .trim_end_matches(' ')
        .to_ascii_lowercase()
        .ends_with("bv")
}

/// Normalize one logical identity for case- and separator-insensitive matching.
fn identity_key(value: &str) -> String {
    value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

/// Normalize one texture reference for exact logical cross-package matching.
fn texture_key(value: &str) -> String {
    let clean = value.trim_end_matches('\0').trim();
    let stem = Path::new(clean)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or(clean);
    stem.trim_end_matches('_').to_ascii_lowercase()
}

impl VehicleTextureAuthority {
    /// Build car-family texture authority from freshly extracted package data.
    pub(super) fn build(
        index: &PhaseThreePackageIndex,
        normalized_root: &Path,
    ) -> Result<Self, PipelineError> {
        let mut sources = BTreeMap::<String, Vec<VehicleTextureSource>>::new();
        for package in index
            .packages()
            .iter()
            .filter(|package| package.category == VEHICLE_CATEGORY)
        {
            let root = normalized_root.join(relative_art_root(package)?);
            for path in png_files(&root.join("components").join("texture"))? {
                let file_name = path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .ok_or_else(
                        || PipelineError::new("vehicle texture has no file name"),
                    )?;
                let bytes = fs::read(&path)
                    .map_err(|error| PipelineError::new(error.to_string()))?;
                sources
                    .entry(texture_key(file_name))
                    .or_default()
                    .push(VehicleTextureSource {
                        subcategory: package.subcategory.clone(),
                        path,
                        sha256: digest_hex(&bytes),
                    });
            }
        }
        for entries in sources.values_mut() {
            entries.sort_by(
                |left, right| {
                    (&left.subcategory, &left.path)
                        .cmp(&(&right.subcategory, &right.path))
                },
            );
            entries.dedup();
        }
        Ok(Self { sources })
    }

    /// Resolve one missing texture without choosing conflicting car variants.
    pub(super) fn resolve(
        &self,
        reference: &str,
        source_subcategory: &str,
    ) -> Result<Option<&Path>, PipelineError> {
        let Some(entries) = self.sources.get(&texture_key(reference)) else {
            return Ok(None);
        };
        let same_package = entries
            .iter()
            .filter(|entry| entry.subcategory == source_subcategory)
            .collect::<Vec<_>>();
        if let Some(path) = unique_texture_path(&same_package)? {
            return Ok(Some(path));
        }
        let common = entries
            .iter()
            .filter(|entry| entry.subcategory == VEHICLE_COMMON_SUBCATEGORY)
            .collect::<Vec<_>>();
        if let Some(path) = unique_texture_path(&common)? {
            return Ok(Some(path));
        }
        unique_texture_path(&entries.iter().collect::<Vec<_>>())
    }
}

/// Select one texture path only when all candidates have identical bytes.
fn unique_texture_path<'source>(
    entries: &[&'source VehicleTextureSource],
) -> Result<Option<&'source Path>, PipelineError> {
    if entries.is_empty() {
        return Ok(None);
    }
    let hashes = entries
        .iter()
        .map(|entry| entry.sha256.as_str())
        .collect::<BTreeSet<_>>();
    if hashes.len() != 1 {
        return Err(PipelineError::new(format!(
            "vehicle shared texture authority is ambiguous across {} payloads",
            entries.len()
        )));
    }
    Ok(entries.first().map(|entry| entry.path.as_path()))
}

#[cfg(test)]
mod tests {
    use super::texture_key;

    #[test]
    fn texture_key_removes_extension_case_and_fixed_width_padding() {
        assert_eq!(texture_key("WindsheildT.bmp\0\0"), "windsheildt");
        assert_eq!(texture_key("homer_vWheel.PNG"), "homer_vwheel");
    }
}

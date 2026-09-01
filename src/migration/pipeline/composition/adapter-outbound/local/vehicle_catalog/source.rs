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
//   - Source outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Source outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde_json::Value;
use shar_sha256::digest_hex;

use super::{VEHICLE_CATEGORY, VEHICLE_COMMON_SUBCATEGORY};
use crate::domain::PipelineError;
use crate::domain::package::{
    PhaseThreePackageIndex, PhaseThreePackageMember, PhaseThreePackageRow,
};

/// One exact texture occurrence available across freshly extracted car
/// packages.
#[derive(Clone, Debug, Eq, PartialEq)]
struct VehicleTextureSource {
    /// Generated package identity owning this texture occurrence.
    package_id: String,
    /// Extracted package subcategory owning this texture occurrence.
    subcategory: String,
    /// Exact normalized physical texture member identity.
    member_id: String,
    /// Exact normalized source component ordinal.
    source_ordinal: usize,
    /// Freshly extracted texture path.
    path: PathBuf,
    /// Exact source texture digest.
    sha256: String,
}

/// One preferred physical vehicle texture occurrence retained without
/// selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct VehicleTextureOccurrenceEvidence {
    /// Generated package identity owning this texture occurrence.
    pub(super) package_id: String,
    /// Generated package subcategory owning this texture occurrence.
    pub(super) subcategory: String,
    /// Exact normalized physical texture member identity.
    pub(super) member_id: String,
    /// Exact normalized source component ordinal.
    pub(super) source_ordinal: usize,
    /// Exact lowercase SHA-256 of the normalized PNG payload.
    pub(super) sha256: String,
}

/// Cross-package texture lookup restricted to the generated car package family.
#[derive(Debug)]
pub(super) struct VehicleTextureAuthority {
    /// Logical texture keys mapped to all freshly extracted occurrences.
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
        let source =
            game_root.join("art").join(&relative).with_extension("p3d");
        if !source.is_file() {
            return Err(PipelineError::new(format!(
                "vehicle source package is missing: {}",
                source.display()
            )));
        }
        p3d::write_lossless_package(&source, &normalized_root.join(relative))
            .map_err(|error| {
            PipelineError::new(format!(
                "vehicle extraction failed for {}: {error}",
                package.package_id
            ))
        })?;
        count = count.checked_add(1).ok_or_else(|| {
            PipelineError::new("vehicle package count overflowed")
        })?;
    }
    Ok(count)
}

/// Return one safe package root relative to extracted/art and game/art.
pub(super) fn relative_art_root(
    package: &PhaseThreePackageRow,
) -> Result<PathBuf, PipelineError> {
    let relative = package
        .package_root
        .strip_prefix("extracted/art/")
        .ok_or_else(|| {
            PipelineError::new(format!(
                "vehicle package root is outside extracted/art: {}",
                package.package_root
            ))
        })?;
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
        .filter_map(|path| {
            decoded_name(&path)
                .ok()
                .filter(|name| !is_collision_volume_identity(name))
                .map(|name| (path, name))
        })
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
    let directory = package_root.join("components").join("composite_drawable");
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
                .filter(|path| {
                    decoded_name(path)
                        .is_ok_and(|name| identity_key(&name) == target)
                })
                .collect::<Vec<_>>();
            match matches.as_slice() {
                [path] => Ok(path.clone()),
                _ => Err(PipelineError::new(format!(
                    "vehicle {vehicle} has ambiguous composite \
                             drawables"
                ))),
            }
        },
    }
}

/// Project selected vehicle members back into exact source chunk order.
fn source_ordered_vehicle_members<'row>(
    package: &'row PhaseThreePackageRow,
    kind: &str,
    predicate: impl Fn(&PhaseThreePackageMember) -> bool,
) -> Result<Vec<&'row PhaseThreePackageMember>, PipelineError> {
    let mut ordered = package
        .members()
        .iter()
        .filter(|member| predicate(member))
        .map(|member| {
            member
                .source_chunk_ordinal
                .map(|ordinal| (ordinal, member))
                .ok_or_else(|| {
                    PipelineError::new(format!(
                        "vehicle {kind} member {} has no source chunk ordinal",
                        member.id
                    ))
                })
        })
        .collect::<Result<Vec<_>, PipelineError>>()?;
    ordered.sort_by_key(|(ordinal, _member)| *ordinal);
    for pair in ordered.windows(2) {
        let [(left_ordinal, _left), (right_ordinal, _right)] = pair else {
            continue;
        };
        if left_ordinal == right_ordinal {
            return Err(PipelineError::new(format!(
                "vehicle package repeats source {kind} ordinal {left_ordinal}"
            )));
        }
    }
    Ok(ordered.into_iter().map(|(_ordinal, member)| member).collect())
}

/// Preserve supplied projection order while rejecting path collisions.
pub(super) fn unique_vehicle_component_paths(
    paths: impl IntoIterator<Item = PathBuf>,
    kind: &str,
) -> Result<Vec<PathBuf>, PipelineError> {
    let mut seen = BTreeSet::new();
    let mut ordered = Vec::new();
    for path in paths {
        if !seen.insert(path.clone()) {
            return Err(PipelineError::new(format!(
                "vehicle package projects duplicate {kind} path: {}",
                path.display()
            )));
        }
        ordered.push(path);
    }
    Ok(ordered)
}

/// Resolve every exact render-mesh path from generated package membership.
pub(super) fn vehicle_mesh_paths(
    package: &PhaseThreePackageRow,
    package_root: &Path,
) -> Result<Vec<PathBuf>, PipelineError> {
    let paths = unique_vehicle_component_paths(
        source_ordered_vehicle_members(package, "mesh", |member| {
            member.kind == "p3d-mesh" && member.source_chunk_kind == "mesh"
        })?
        .into_iter()
        .map(|member| {
                let file_name =
                    Path::new(&member.path).file_name().ok_or_else(|| {
                        PipelineError::new(
                            "vehicle mesh member has no file name",
                        )
                    })?;
                Ok(package_root
                    .join("components")
                    .join("mesh")
                    .join(file_name))
            })
            .collect::<Result<Vec<_>, PipelineError>>()?,
        "mesh",
    )?;
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

/// Resolve every package-local billboard quad-group path.
pub(super) fn vehicle_quad_group_paths(
    package: &PhaseThreePackageRow,
    package_root: &Path,
) -> Result<Vec<PathBuf>, PipelineError> {
    let paths = unique_vehicle_component_paths(
        source_ordered_vehicle_members(package, "quad-group", |member| {
            member.source_chunk_kind == "quad_group"
        })?
        .into_iter()
        .map(|member| {
                let file_name =
                    Path::new(&member.path).file_name().ok_or_else(|| {
                        PipelineError::new(
                            "vehicle quad-group member has no file name",
                        )
                    })?;
                Ok(package_root
                    .join("components")
                    .join("quad_group")
                    .join(file_name))
            })
            .collect::<Result<Vec<_>, PipelineError>>()?,
        "quad-group",
    )?;
    if let Some(path) = paths.iter().find(|path| !path.is_file()) {
        return Err(PipelineError::new(format!(
            "vehicle quad-group source is missing: {}",
            path.display()
        )));
    }
    Ok(paths)
}

/// Resolve every package-local animation path in exact source chunk order.
pub(super) fn vehicle_animation_paths(
    package: &PhaseThreePackageRow,
    package_root: &Path,
) -> Result<Vec<PathBuf>, PipelineError> {
    let paths = unique_vehicle_component_paths(
        source_ordered_vehicle_members(package, "animation", |member| {
            member.kind == "p3d-animation"
                && member.source_chunk_kind == "animation"
        })?
        .into_iter()
        .map(|member| {
                let file_name =
                    Path::new(&member.path).file_name().ok_or_else(|| {
                        PipelineError::new(
                            "vehicle animation has no file name",
                        )
                    })?;
                Ok(package_root
                    .join("components")
                    .join("animation")
                    .join(file_name))
            })
            .collect::<Result<Vec<_>, PipelineError>>()?,
        "animation",
    )?;
    if let Some(path) = paths.iter().find(|path| !path.is_file()) {
        return Err(PipelineError::new(format!(
            "vehicle animation source is missing: {}",
            path.display()
        )));
    }
    Ok(paths)
}

/// Return the three original runtime headlight groups from the common package.
pub(super) fn common_headlight_quad_groups(
    normalized_root: &Path,
) -> Result<(PathBuf, Vec<PathBuf>), PipelineError> {
    let common_root = normalized_root.join("cars").join("common");
    let directory = common_root.join("components").join("quad_group");
    let required = ["headlightShape8", "headlight2Shape", "glowGroupShape2"];
    let candidates = json_files(&directory)?;
    let mut selected = Vec::new();
    for identity in required {
        let matches = candidates
            .iter()
            .filter(|path| {
                decoded_name(path)
                    .is_ok_and(|name| name.eq_ignore_ascii_case(identity))
            })
            .cloned()
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [path] => selected.push(path.clone()),
            [] => {
                return Err(PipelineError::new(format!(
                    "common vehicle headlight group is missing: \
                             {identity}"
                )));
            },
            _ => {
                return Err(PipelineError::new(format!(
                    "common vehicle headlight group is ambiguous: \
                             {identity}"
                )));
            },
        }
    }
    let selected = source_ordered_common_quad_groups(&common_root, selected)?;
    Ok((common_root, selected))
}

/// Restore source-chunk order for selected common-package quad groups.
fn source_ordered_common_quad_groups(
    common_root: &Path,
    selected: Vec<PathBuf>,
) -> Result<Vec<PathBuf>, PipelineError> {
    let manifest = common_root.join("components.jsonl");
    let text = fs::read_to_string(&manifest).map_err(|error| {
        PipelineError::new(format!(
            "common vehicle component ledger read failed: {error}"
        ))
    })?;
    let selected_paths = selected
        .iter()
        .map(|path| {
            path.file_name()
                .and_then(|value| value.to_str())
                .map(|file_name| format!("quad_group/{file_name}"))
                .ok_or_else(|| {
                    PipelineError::new(format!(
                        concat!(
                            "common vehicle headlight path has no UTF-8 ",
                            "file name: {}"
                        ),
                        path.display()
                    ))
                })
        })
        .collect::<Result<BTreeSet<_>, PipelineError>>()?;
    let mut ordinals = BTreeMap::<String, u64>::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let value: Value = serde_json::from_str(line).map_err(|error| {
            PipelineError::new(format!(
                "common vehicle component ledger parse failed: {error}"
            ))
        })?;
        if value.get("kind").and_then(Value::as_str) != Some("quad_group") {
            continue;
        }
        let Some(path) = value.get("path").and_then(Value::as_str) else {
            continue;
        };
        if !selected_paths.contains(path) {
            continue;
        }
        let ordinal = value
            .get("ordinal")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "common vehicle headlight {path} has no source ordinal"
                ))
            })?;
        if ordinals.insert(path.to_owned(), ordinal).is_some() {
            return Err(PipelineError::new(format!(
                "common vehicle headlight ledger path repeats: {path}"
            )));
        }
    }
    let mut ordered = selected
        .into_iter()
        .map(|path| {
            let file_name = path
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or_else(|| {
                    PipelineError::new(format!(
                        concat!(
                            "common vehicle headlight path has no UTF-8 ",
                            "file name: {}"
                        ),
                        path.display()
                    ))
                })?;
            let relative = format!("quad_group/{file_name}");
            let ordinal = ordinals.get(&relative).copied().ok_or_else(|| {
                PipelineError::new(format!(
                    "common vehicle headlight has no ledger relationship: \
                     {relative}"
                ))
            })?;
            Ok((ordinal, path))
        })
        .collect::<Result<Vec<_>, PipelineError>>()?;
    ordered.sort_by_key(|(ordinal, _path)| *ordinal);
    for pair in ordered.windows(2) {
        let [(left, _), (right, _)] = pair else {
            continue;
        };
        if left == right {
            return Err(PipelineError::new(format!(
                "common vehicle headlight source ordinal repeats: {left}"
            )));
        }
    }
    Ok(ordered.into_iter().map(|(_ordinal, path)| path).collect())
}

/// Return sorted JSON files from one optional component directory.
fn json_files(directory: &Path) -> Result<Vec<PathBuf>, PipelineError> {
    files_with_extension(directory, "json")
}

/// Return sorted PNG files from one optional component directory.
pub(super) fn png_files(
    directory: &Path,
) -> Result<Vec<PathBuf>, PipelineError> {
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
        .map(|entry| {
            entry
                .map(|value| value.path())
                .map_err(|error| PipelineError::new(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    files.retain(|path| {
        path.is_file()
            && path
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|value| value.eq_ignore_ascii_case(extension))
    });
    files.sort();
    Ok(files)
}

/// Read one decoded component name and remove fixed-width null padding.
pub(super) fn decoded_name(path: &Path) -> Result<String, PipelineError> {
    let value: Value = serde_json::from_slice(
        &fs::read(path)
            .map_err(|error| PipelineError::new(error.to_string()))?,
    )
    .map_err(|error| PipelineError::new(error.to_string()))?;
    let name = value.get("name").and_then(Value::as_str).ok_or_else(|| {
        PipelineError::new(format!("component has no name: {}", path.display()))
    })?;
    let clean = name.trim_end_matches('\0');
    if clean.is_empty()
        || clean != clean.trim()
        || clean.chars().any(char::is_control)
    {
        return Err(PipelineError::new(
            "vehicle component identity is non-canonical",
        ));
    }
    Ok(clean.to_owned())
}

/// Return whether one skeleton identity belongs only to collision-volume data.
fn is_collision_volume_identity(value: &str) -> bool {
    value
        .trim_end_matches('\u{0}')
        .to_ascii_lowercase()
        .ends_with("bv")
}

/// Normalize one logical identity for case- and separator-insensitive matching.
fn identity_key(value: &str) -> String {
    value
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .flat_map(char::to_lowercase)
        .collect()
}

/// Normalize one texture reference for exact logical cross-package matching.
fn texture_key(value: &str) -> Result<String, PipelineError> {
    let clean = value.trim_end_matches('\0');
    if clean != clean.trim() || clean.chars().any(char::is_control) {
        return Err(PipelineError::new(
            "vehicle texture identity is non-canonical",
        ));
    }
    let stem = Path::new(clean)
        .file_stem()
        .and_then(|component| component.to_str())
        .unwrap_or(clean);
    Ok(stem.trim_end_matches('_').to_ascii_lowercase())
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
            for (key, source) in
                vehicle_package_texture_sources(package, &root)?
            {
                sources.entry(key).or_default().push(source);
            }
        }
        for entries in sources.values_mut() {
            entries.sort_by(|left, right| {
                (&left.subcategory, &left.path)
                    .cmp(&(&right.subcategory, &right.path))
            });
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
        let preferred = self.preferred_sources(reference, source_subcategory)?;
        unique_texture_path(&preferred)
    }

    /// Retain every preferred physical occurrence without choosing one member.
    pub(super) fn preferred_occurrences(
        &self,
        reference: &str,
        source_subcategory: &str,
    ) -> Result<Vec<VehicleTextureOccurrenceEvidence>, PipelineError> {
        let mut occurrences = self
            .preferred_sources(reference, source_subcategory)?
            .into_iter()
            .map(|source| VehicleTextureOccurrenceEvidence {
                package_id: source.package_id.clone(),
                subcategory: source.subcategory.clone(),
                member_id: source.member_id.clone(),
                source_ordinal: source.source_ordinal,
                sha256: source.sha256.clone(),
            })
            .collect::<Vec<_>>();
        occurrences.sort_by(|left, right| {
            (
                &left.package_id,
                &left.subcategory,
                left.source_ordinal,
                &left.member_id,
            )
                .cmp(&(
                    &right.package_id,
                    &right.subcategory,
                    right.source_ordinal,
                    &right.member_id,
                ))
        });
        Ok(occurrences)
    }

    /// Select the same-package, common, or global candidate scope.
    fn preferred_sources(
        &self,
        reference: &str,
        source_subcategory: &str,
    ) -> Result<Vec<&VehicleTextureSource>, PipelineError> {
        let key = texture_key(reference)?;
        let Some(entries) = self.sources.get(&key) else {
            return Ok(Vec::new());
        };
        let same_package = entries
            .iter()
            .filter(|entry| entry.subcategory == source_subcategory)
            .collect::<Vec<_>>();
        if !same_package.is_empty() {
            return Ok(same_package);
        }
        let common = entries
            .iter()
            .filter(|entry| entry.subcategory == VEHICLE_COMMON_SUBCATEGORY)
            .collect::<Vec<_>>();
        if !common.is_empty() {
            return Ok(common);
        }
        Ok(entries.iter().collect())
    }
}

/// Read exact normalized texture occurrence authority from one package ledger.
fn vehicle_package_texture_sources(
    package: &PhaseThreePackageRow,
    root: &Path,
) -> Result<Vec<(String, VehicleTextureSource)>, PipelineError> {
    let ledger_path = root.join("components.jsonl");
    let text = fs::read_to_string(&ledger_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
    let mut paths = BTreeSet::new();
    let mut ordinals = BTreeSet::new();
    let mut sources = Vec::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let row: Value = serde_json::from_str(line)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        if row.get("kind").and_then(Value::as_str) != Some("texture") {
            continue;
        }
        let name = row.get("name").and_then(Value::as_str).ok_or_else(|| {
            PipelineError::new("vehicle texture ledger row has no identity")
        })?;
        let relative = row.get("path").and_then(Value::as_str).ok_or_else(|| {
            PipelineError::new("vehicle texture ledger row has no path")
        })?;
        let member = relative.strip_prefix("texture/").ok_or_else(|| {
            PipelineError::new("vehicle texture ledger path is outside texture")
        })?;
        let member_path = Path::new(member);
        if member_path.is_absolute()
            || member_path.components().count() != 1
            || member_path.file_name().and_then(|value| value.to_str())
                != Some(member)
            || member_path
                .extension()
                .and_then(|value| value.to_str())
                .is_none_or(|extension| !extension.eq_ignore_ascii_case("png"))
        {
            return Err(PipelineError::new(
                "vehicle texture ledger path is not one PNG member",
            ));
        }
        let source_ordinal = row
            .get("ordinal")
            .and_then(Value::as_u64)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or_else(|| {
                PipelineError::new(
                    "vehicle texture ledger row has no source ordinal",
                )
            })?;
        if !ordinals.insert(source_ordinal) {
            return Err(PipelineError::new(format!(
                "vehicle texture ledger repeats source ordinal {source_ordinal}"
            )));
        }
        let path = root.join("components").join(relative);
        if !paths.insert(path.clone()) {
            return Err(PipelineError::new(format!(
                "vehicle texture ledger repeats physical path: {}",
                path.display()
            )));
        }
        let member_id = member_path
            .file_stem()
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                PipelineError::new("vehicle texture member has no identity")
            })?
            .to_owned();
        let bytes = fs::read(&path)
            .map_err(|error| PipelineError::new(error.to_string()))?;
        sources.push((
            texture_key(name)?,
            VehicleTextureSource {
                package_id: package.package_id.clone(),
                subcategory: package.subcategory.clone(),
                member_id,
                source_ordinal,
                path,
                sha256: digest_hex(&bytes),
            },
        ));
    }
    Ok(sources)
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
            "vehicle shared texture authority is ambiguous across {} \
                     payloads",
            entries.len()
        )));
    }
    Ok(entries.first().map(|entry| entry.path.as_path()))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/vehicle_catalog/source/tests.rs"]
mod tests;

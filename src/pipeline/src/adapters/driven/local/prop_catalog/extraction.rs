// File:
//   - extraction.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/extraction.rs
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
//   - Re-extraction of card and mission P3D packages for prop discovery.
// - Must-Not:
//   - Select model members, write FBX, or publish normalized staging.
// - Allows:
//   - Generated package-index selection and lossless P3D extraction.
// - Split-When:
//   - Source location policy differs by package category.
// - Merge-When:
//   - A shared batch extraction stage owns the same selected-package contract.
// - Summary:
//   - Materializes non-world prop components from the original game tree.
// - Description:
//   - Derives each P3D source from the generated package root without
//     hardcoding.
// - Usage:
//   - Called before non-world prop inventory.
// - Defaults:
//   - The `cards` and `missions` package categories are selected.
//
// ADRs:
// - docs/adr/fbx/extraction/source-discovery-boundary.md
// - docs/adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md
//
// Large file:
//   - false
//

//! Re-extraction of non-world prop P3D packages for discovery.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

/// Return whether one package belongs to the non-world prop scan.
pub(super) fn is_selected_package(package: &PhaseThreePackageRow) -> bool {
    matches!(
        package
            .category
            .as_str(),
        "cards" | "missions"
    )
}

/// Return whether one package belongs to the world-prop scan.
pub(super) fn is_world_package(package: &PhaseThreePackageRow) -> bool {
    package.category == "terrain-world"
}

/// Return whether one package belongs to one of the seven main game levels.
pub(super) fn is_world_level_package(package: &PhaseThreePackageRow) -> bool {
    package.category == "terrain-world"
        && package
            .subcategory
            .starts_with("terrain-world/level-")
}

/// Return one package root relative to `extracted/art` and `game/art`.
///
/// # Errors
///
/// Returns an error when the generated root leaves the canonical asset tree.
pub(super) fn relative_art_root(
    package: &PhaseThreePackageRow,
) -> Result<PathBuf, PipelineError> {
    relative_art_root_value(&package.package_root)
}

/// Convert one package-root string to a safe path below the art directory.
fn relative_art_root_value(root: &str) -> Result<PathBuf, PipelineError> {
    let relative = root
        .strip_prefix("extracted/art/")
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "prop package root is outside extracted/art: {root}"
                    ),
                )
            },
        )?;
    let path = Path::new(relative);
    if path.is_absolute()
        || path
            .components()
            .any(
                |component| {
                    matches!(
                        component,
                        Component::ParentDir
                    )
                },
            )
    {
        return Err(
            PipelineError::new(
                format!("prop package root is not portable: {root}"),
            ),
        );
    }
    Ok(path.to_path_buf())
}

/// Re-extract every selected source package from the original game tree.
///
/// # Errors
///
/// Returns an error when one source is missing or P3D extraction fails.
pub(super) fn extract_selected_packages(
    index: &PhaseThreePackageIndex,
    game_root: &Path,
    normalized_root: &Path,
) -> Result<usize, PipelineError> {
    extract_packages(
        index,
        game_root,
        normalized_root,
        is_selected_package,
    )
}

/// Re-extract every terrain-world source package.
pub(super) fn extract_world_packages(
    index: &PhaseThreePackageIndex,
    game_root: &Path,
    normalized_root: &Path,
) -> Result<usize, PipelineError> {
    extract_packages(
        index,
        game_root,
        normalized_root,
        is_world_package,
    )
}

/// Re-extract only the seven main-level world packages.
pub(super) fn extract_world_level_packages(
    index: &PhaseThreePackageIndex,
    game_root: &Path,
    normalized_root: &Path,
) -> Result<usize, PipelineError> {
    extract_packages(
        index,
        game_root,
        normalized_root,
        is_world_level_package,
    )
}

/// Re-extract coordinate-reference P3D packages available for the seven levels.
///
/// Missing reference packages are accepted because the canonical game package
/// remains the lossless fallback for those exact package identities.
///
/// # Errors
///
/// Returns an error when the reference directory is unreadable or contains
/// ambiguous case-insensitive P3D names. Individual undecodable references are
/// cleaned up and left to canonical coordinate fallback.
pub(super) fn extract_world_level_coordinate_packages(
    index: &PhaseThreePackageIndex,
    coordinate_root: &Path,
    normalized_root: &Path,
) -> Result<BTreeSet<String>, PipelineError> {
    let sources = coordinate_sources(coordinate_root)?;
    let mut referenced = BTreeSet::new();
    for package in index
        .packages()
        .iter()
        .filter(|package| is_world_level_package(package))
    {
        let relative = relative_art_root(package)?;
        let file_name = relative
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(
                || {
                    PipelineError::new(
                        "world coordinate package has no file name",
                    )
                },
            )?;
        let Some(source) = sources.get(&file_name.to_ascii_lowercase()) else {
            continue;
        };
        let destination = normalized_root.join(&relative);
        match p3d::write_lossless_package(
            source,
            &destination,
        ) {
            Ok(_summary) => {
                let _inserted = referenced.insert(
                    package
                        .package_id
                        .clone(),
                );
            }
            Err(_error) => {
                drop(fs::remove_dir_all(&destination));
            }
        }
    }
    Ok(referenced)
}

/// Index direct child P3D files by portable case-insensitive stem.
fn coordinate_sources(
    root: &Path,
) -> Result<BTreeMap<String, PathBuf>, PipelineError> {
    let mut sources = BTreeMap::new();
    for entry in fs::read_dir(root).map_err(
        |error| {
            PipelineError::new(
                format!("world coordinate directory read failed: {error}"),
            )
        },
    )? {
        let path = entry
            .map_err(|error| PipelineError::new(error.to_string()))?
            .path();
        if !path.is_file()
            || path
                .extension()
                .and_then(|value| value.to_str())
                .is_none_or(|value| !value.eq_ignore_ascii_case("p3d"))
        {
            continue;
        }
        let stem = path
            .file_stem()
            .and_then(|value| value.to_str())
            .ok_or_else(
                || PipelineError::new("world coordinate P3D has no file stem"),
            )?
            .to_ascii_lowercase();
        if sources
            .insert(
                stem.clone(),
                path,
            )
            .is_some()
        {
            return Err(
                PipelineError::new(
                    format!("world coordinate P3D stem is ambiguous: {stem}"),
                ),
            );
        }
    }
    Ok(sources)
}

/// Re-extract every package accepted by one category predicate.
///
/// # Errors
///
/// Returns an error when a source package is missing or extraction fails.
fn extract_packages(
    index: &PhaseThreePackageIndex,
    game_root: &Path,
    normalized_root: &Path,
    selected: fn(&PhaseThreePackageRow) -> bool,
) -> Result<usize, PipelineError> {
    let mut count = 0_usize;
    for package in index
        .packages()
        .iter()
        .filter(|package| selected(package))
    {
        let relative = relative_art_root(package)?;
        let source = game_root
            .join("art")
            .join(&relative)
            .with_extension("p3d");
        if !source.is_file() {
            return Err(
                PipelineError::new(
                    format!(
                        "prop source package is missing: {}",
                        source.display()
                    ),
                ),
            );
        }
        p3d::write_lossless_package(
            &source,
            &normalized_root.join(relative),
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "prop source extraction failed for {}: {error}",
                        package.package_id
                    ),
                )
            },
        )?;
        count = count
            .checked_add(1)
            .ok_or_else(
                || PipelineError::new("prop source package count overflowed"),
            )?;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::relative_art_root_value;

    #[test]
    fn art_root_is_made_relative() {
        assert_eq!(
            relative_art_root_value("extracted/art/missions/flag"),
            Ok(std::path::PathBuf::from("missions/flag"))
        );
    }

    #[test]
    fn art_root_rejects_parent_traversal() {
        assert!(relative_art_root_value("extracted/art/../private").is_err());
    }
}

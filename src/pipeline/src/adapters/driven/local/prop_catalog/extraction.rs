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
//   - Re-extraction of mission P3D packages for prop discovery.
// - Must-Not:
//   - Select model members, write FBX, or publish normalized staging.
// - Allows:
//   - Generated package-index selection and lossless P3D extraction.
// - Split-When:
//   - Source location policy differs by package category.
// - Merge-When:
//   - A shared batch extraction stage owns the same selected-package contract.
// - Summary:
//   - Materializes mission model components from the original game tree.
// - Description:
//   - Derives each P3D source from the generated package root without
//     hardcoding.
// - Usage:
//   - Called before mission prop inventory.
// - Defaults:
//   - Only the `missions` package category is selected.
//
// ADRs:
// - docs/adr/fbx/extraction/source-discovery-boundary.md
// - docs/adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md
//
// Large file:
//   - false
//

//! Re-extraction of mission P3D packages for prop discovery.

use std::path::{Component, Path, PathBuf};

use crate::domain::PipelineError;
use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};

/// Return whether one package belongs to the mission prop scan.
pub(super) fn is_selected_package(package: &PhaseThreePackageRow) -> bool {
    package.category == "missions"
}

/// Return one package root relative to `extracted/art` and `game/art`.
///
/// # Errors
///
/// Returns an error when the generated root leaves the canonical asset tree.
pub(super) fn relative_art_root(
    package: &PhaseThreePackageRow
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
    let mut count = 0_usize;
    for package in index
        .packages()
        .iter()
        .filter(|package| is_selected_package(package))
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

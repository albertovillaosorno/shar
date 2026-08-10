// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Canonical package resolution for authored mission P3D path references.
// - Must-Not:
//   - Interpret why a P3D is referenced or assign runtime loading semantics.
// - Allows:
//   - Normalize portable source paths and bind them to phase-three packages.
// - Split-When:
//   - Another source transport needs a different path-to-package convention.
// - Merge-When:
//   - All mission P3D consumers move behind one final definition compiler.
// - Summary:
//   - Shared mission P3D package-reference catalog.
// - Description:
//   - Centralizes portable P3D path normalization and canonical package lookup
//     for explicit loads, presentation bitmaps, and future typed references.
// - Usage:
//   - Built once from validated phase-three package-index evidence.
// - Defaults:
//   - Unsafe, non-P3D, missing, or normalization-colliding references fail.
//

//! Shared canonical package lookup for authored mission P3D references.

use std::collections::BTreeMap;
use std::path::Path;

use super::PhaseThreePackageIndex;

#[derive(Clone, Debug, Eq, PartialEq)]
struct MissionP3dCatalogEntry {
    package_id: String,
    package_root: String,
}

/// One authored P3D path bound to a canonical phase-three package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionP3dPackageReference {
    source_reference: String,
    package_id: String,
    package_root: String,
}

impl MissionP3dPackageReference {
    /// Return the exact authored P3D path.
    #[must_use]
    pub fn source_reference(&self) -> &str {
        &self.source_reference
    }

    /// Return the canonical phase-three package id.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the canonical phase-three package root.
    #[must_use]
    pub fn package_root(&self) -> &str {
        &self.package_root
    }
}

/// Case-insensitive package lookup for portable authored P3D paths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionP3dReferenceCatalog {
    by_root: BTreeMap<String, MissionP3dCatalogEntry>,
}

impl MissionP3dReferenceCatalog {
    /// Build canonical P3D lookup from validated package-index evidence.
    ///
    /// # Errors
    ///
    /// Returns an error when extracted package roots collide after portable
    /// normalization or contain unsafe path structure.
    pub fn from_package_index(
        index: &PhaseThreePackageIndex,
    ) -> Result<Self, String> {
        let mut by_root = BTreeMap::new();
        for package in index.packages() {
            let Some(normalized) = normalized_candidate_package_root(
                &package.package_root,
            )? else {
                continue;
            };
            let entry = MissionP3dCatalogEntry {
                package_id: package.package_id.clone(),
                package_root: package.package_root.clone(),
            };
            if by_root.insert(normalized, entry).is_some() {
                return Err(
                    "phase-three package roots collide after normalization"
                        .to_owned(),
                );
            }
        }
        Ok(Self { by_root })
    }

    #[cfg(test)]
    pub(crate) const fn empty_for_tests() -> Self {
        Self {
            by_root: BTreeMap::new(),
        }
    }

    #[cfg(test)]
    pub(super) fn from_entries_for_tests(
        entries: &[(&str, &str, &str)],
    ) -> Self {
        let by_root = entries
            .iter()
            .map(|(root, package_id, package_root)| {
                (
                    (*root).to_owned(),
                    MissionP3dCatalogEntry {
                        package_id: (*package_id).to_owned(),
                        package_root: (*package_root).to_owned(),
                    },
                )
            })
            .collect();
        Self { by_root }
    }

    /// Resolve one exact authored P3D path to its canonical package.
    ///
    /// # Errors
    ///
    /// Returns an error for unsafe/non-P3D paths or paths absent from the
    /// validated package index.
    pub fn resolve(
        &self,
        source_reference: &str,
    ) -> Result<MissionP3dPackageReference, String> {
        let package_root = normalized_p3d_package_root(source_reference)?;
        let entry = self.by_root.get(&package_root).ok_or_else(|| {
            "mission P3D reference has no canonical package".to_owned()
        })?;
        Ok(MissionP3dPackageReference {
            source_reference: source_reference.to_owned(),
            package_id: entry.package_id.clone(),
            package_root: entry.package_root.clone(),
        })
    }
}

pub(super) fn normalized_p3d_package_root(
    reference: &str,
) -> Result<String, String> {
    if reference.is_empty()
        || reference != reference.trim()
        || reference.chars().any(char::is_control)
    {
        return Err(
            "mission P3D path is blank, padded, or control-bearing".to_owned()
        );
    }
    let normalized = reference
        .replace(char::from(92), "/")
        .to_ascii_lowercase();
    validate_relative_transport_path(&normalized)?;
    if !Path::new(&normalized)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("p3d"))
    {
        return Err("mission P3D reference is not a P3D path".to_owned());
    }
    let without_extension = normalized
        .strip_suffix(".p3d")
        .ok_or_else(|| {
            "mission P3D extension normalization failed".to_owned()
        })?;
    Ok(format!("extracted/{without_extension}"))
}

pub(super) fn normalized_candidate_package_root(
    root: &str,
) -> Result<Option<String>, String> {
    if root.is_empty()
        || root != root.trim()
        || root.chars().any(char::is_control)
    {
        return Err("phase-three package root is malformed".to_owned());
    }
    let normalized = root
        .replace(char::from(92), "/")
        .to_ascii_lowercase();
    if normalized == "extracted" || !normalized.starts_with("extracted/") {
        return Ok(None);
    }
    Ok(Some(normalized_package_root(&normalized)?))
}

pub(super) fn normalized_package_root(root: &str) -> Result<String, String> {
    if root.is_empty()
        || root != root.trim()
        || root.chars().any(char::is_control)
    {
        return Err("phase-three package root is malformed".to_owned());
    }
    let normalized = root
        .replace(char::from(92), "/")
        .to_ascii_lowercase();
    if !normalized.starts_with("extracted/") {
        return Err(
            "phase-three package root is outside extracted evidence".to_owned()
        );
    }
    validate_relative_transport_path(&normalized)?;
    Ok(normalized)
}

fn validate_relative_transport_path(path: &str) -> Result<(), String> {
    if path.starts_with('/')
        || path.contains(':')
        || path.contains(';')
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err("mission P3D path is unsafe".to_owned());
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_p3d_reference/tests.rs"]
mod tests;

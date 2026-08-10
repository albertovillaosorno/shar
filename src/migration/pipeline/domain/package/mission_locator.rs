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
//   - Package-scoped canonical lookup for decoded mission locator identities.
// - Must-Not:
//   - Read files, infer package load precedence, or derive names from filenames.
// - Allows:
//   - Resolve exact decoded locator names against explicit active package roots.
//   - Preserve missing and ambiguous lookup outcomes as typed evidence.
// - Split-When:
//   - Mission package-load context gains an independent runtime lifecycle.
// - Merge-When:
//   - Final mission-definition compilation owns locator identity directly.
// - Summary:
//   - Mission locator package-reference catalog.
// - Description:
//   - Binds decoded srr_locator identities to phase-three package evidence while
//     refusing global-name and package-precedence guesses.
// - Usage:
//   - Constructed by a filesystem adapter from validated decoded locator JSON.
// - Defaults:
//   - Missing and ambiguous identities remain explicit and unresolved.
//

//! Package-scoped canonical mission locator lookup.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// One decoded locator identity bound to canonical package evidence.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MissionLocatorCatalogEntry {
    source_name: String,
    locator_type: u32,
    locator_type_name: String,
    member_id: String,
    package_id: String,
    package_root: String,
    member_path: String,
}

impl MissionLocatorCatalogEntry {
    /// Build one validated decoded locator catalog entry.
    ///
    /// # Errors
    ///
    /// Returns an error when identity, package, type, or member evidence is
    /// blank, padded, control-bearing, or internally inconsistent.
    pub fn new(
        source_name: String,
        locator_type: u32,
        locator_type_name: String,
        member_id: String,
        package_id: String,
        package_root: String,
        member_path: String,
    ) -> Result<Self, String> {
        validate_identity(&source_name, "locator source name")?;
        validate_identity(&locator_type_name, "locator type name")?;
        validate_identity(&member_id, "locator member id")?;
        validate_identity(&package_id, "locator package id")?;
        validate_package_root(&package_root)?;
        validate_member_path(&package_root, &member_path)?;
        Ok(Self {
            source_name,
            locator_type,
            locator_type_name,
            member_id,
            package_id,
            package_root,
            member_path,
        })
    }

    /// Return the exact decoded locator name.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    /// Return the exact decoded numeric locator type.
    #[must_use]
    pub const fn locator_type(&self) -> u32 {
        self.locator_type
    }

    /// Return the exact decoded locator type label.
    #[must_use]
    pub fn locator_type_name(&self) -> &str {
        &self.locator_type_name
    }

    /// Return the canonical minor-unit member id.
    #[must_use]
    pub fn member_id(&self) -> &str {
        &self.member_id
    }

    /// Return the canonical phase-three package id.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the exact published package root.
    #[must_use]
    pub fn package_root(&self) -> &str {
        &self.package_root
    }

    /// Return the exact published decoded member path.
    #[must_use]
    pub fn member_path(&self) -> &str {
        &self.member_path
    }
}

/// Optional exact decoded locator-type constraint.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionLocatorTypeConstraint {
    /// Accept any decoded locator type.
    Any,
    /// Accept only one exact numeric source locator type.
    Exact(u32),
}

impl MissionLocatorTypeConstraint {
    const fn accepts(self, locator_type: u32) -> bool {
        match self {
            Self::Any => true,
            Self::Exact(expected) => locator_type == expected,
        }
    }
}

/// One exact locator reference that resolved inside explicit package context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionResolvedLocatorReference {
    entry: MissionLocatorCatalogEntry,
}

impl MissionResolvedLocatorReference {
    /// Return the canonical decoded locator catalog entry.
    #[must_use]
    pub const fn entry(&self) -> &MissionLocatorCatalogEntry {
        &self.entry
    }
}

/// Typed result of one package-context locator lookup.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionLocatorResolution {
    /// No active package contains the exact decoded source name and type.
    Missing,
    /// Exactly one active package contains the exact decoded identity.
    Resolved(MissionResolvedLocatorReference),
    /// More than one active package contains the exact decoded identity.
    Ambiguous(Vec<MissionLocatorCatalogEntry>),
}

/// Package-backed decoded locator lookup catalog.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLocatorCatalog {
    by_source_name: BTreeMap<String, Vec<MissionLocatorCatalogEntry>>,
}

impl MissionLocatorCatalog {
    /// Build a deterministic package-scoped catalog from decoded locator rows.
    ///
    /// # Errors
    ///
    /// Returns an error when two entries publish the same decoded locator name
    /// from one package or when the catalog is empty.
    pub fn from_entries(entries: Vec<MissionLocatorCatalogEntry>) -> Result<Self, String> {
        if entries.is_empty() {
            return Err("mission locator catalog is empty".to_owned());
        }
        let mut by_source_name = BTreeMap::<String, Vec<MissionLocatorCatalogEntry>>::new();
        let mut package_names = BTreeSet::<(String, String)>::new();
        for entry in entries {
            let package_key = normalized_package_root(&entry.package_root)?;
            let identity = (package_key, entry.source_name.clone());
            if !package_names.insert(identity) {
                return Err("mission locator name is duplicated inside one package".to_owned());
            }
            by_source_name
                .entry(entry.source_name.clone())
                .or_default()
                .push(entry);
        }
        for entries in by_source_name.values_mut() {
            entries.sort_by(|left, right| {
                left.package_id
                    .cmp(&right.package_id)
                    .then_with(|| left.member_id.cmp(&right.member_id))
            });
        }
        Ok(Self { by_source_name })
    }

    /// Resolve one exact decoded locator name inside explicit active packages.
    ///
    /// # Errors
    ///
    /// Returns an error when one active package root is malformed.
    pub fn resolve(
        &self,
        source_name: &str,
        active_package_roots: &[String],
        type_constraint: MissionLocatorTypeConstraint,
    ) -> Result<MissionLocatorResolution, String> {
        validate_identity(source_name, "mission locator reference")?;
        let active = active_package_roots
            .iter()
            .map(|root| normalized_package_root(root))
            .collect::<Result<BTreeSet<_>, _>>()?;
        let Some(entries) = self.by_source_name.get(source_name) else {
            return Ok(MissionLocatorResolution::Missing);
        };
        let candidates = entries
            .iter()
            .filter(|entry| {
                type_constraint.accepts(entry.locator_type)
                    && normalized_package_root(&entry.package_root)
                        .is_ok_and(|root| active.contains(&root))
            })
            .cloned()
            .collect::<Vec<_>>();
        match candidates.as_slice() {
            [] => Ok(MissionLocatorResolution::Missing),
            [entry] => Ok(MissionLocatorResolution::Resolved(
                MissionResolvedLocatorReference {
                    entry: entry.clone(),
                },
            )),
            _ => Ok(MissionLocatorResolution::Ambiguous(candidates)),
        }
    }

    /// Resolve one exact locator through explicit package lookup precedence.
    ///
    /// Unlike [`Self::resolve`], this method treats caller-supplied package
    /// order as authoritative lookup evidence and returns the first matching
    /// package. Callers without proven lookup order must use [`Self::resolve`]
    /// so cross-package duplicates remain ambiguous.
    ///
    /// # Errors
    ///
    /// Returns an error when one package root is malformed.
    pub fn resolve_in_package_order(
        &self,
        source_name: &str,
        ordered_package_roots: &[String],
        type_constraint: MissionLocatorTypeConstraint,
    ) -> Result<MissionLocatorResolution, String> {
        validate_identity(source_name, "mission locator reference")?;
        let Some(entries) = self.by_source_name.get(source_name) else {
            return Ok(MissionLocatorResolution::Missing);
        };
        let mut seen = BTreeSet::new();
        for package_root in ordered_package_roots {
            let package_root = normalized_package_root(package_root)?;
            if !seen.insert(package_root.clone()) {
                continue;
            }
            let candidates = entries
                .iter()
                .filter(|entry| {
                    type_constraint.accepts(entry.locator_type)
                        && normalized_package_root(&entry.package_root)
                            .is_ok_and(|root| root == package_root)
                })
                .cloned()
                .collect::<Vec<_>>();
            match candidates.as_slice() {
                [] => {}
                [entry] => {
                    return Ok(MissionLocatorResolution::Resolved(
                        MissionResolvedLocatorReference {
                            entry: entry.clone(),
                        },
                    ));
                }
                _ => return Ok(MissionLocatorResolution::Ambiguous(candidates)),
            }
        }
        Ok(MissionLocatorResolution::Missing)
    }

    /// Return the number of decoded physical locator entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.by_source_name.values().map(Vec::len).sum()
    }

    /// Return whether no decoded locator entries are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_source_name.is_empty()
    }
}

fn validate_identity(value: &str, role: &str) -> Result<(), String> {
    if value.is_empty() || value != value.trim() {
        return Err(format!("{role} is blank or padded"));
    }
    if value.chars().any(char::is_control) {
        return Err(format!("{role} contains control characters"));
    }
    Ok(())
}

fn validate_package_root(root: &str) -> Result<(), String> {
    let normalized = normalized_package_root(root)?;
    if !normalized.starts_with("extracted/") {
        return Err("locator package root is outside extracted evidence".to_owned());
    }
    Ok(())
}

fn normalized_package_root(root: &str) -> Result<String, String> {
    if root.is_empty() || root != root.trim() || root.chars().any(char::is_control) {
        return Err("mission locator package root is malformed".to_owned());
    }
    if root.contains(':') || root.starts_with('/') || root.ends_with('/') {
        return Err("mission locator package root is not relative".to_owned());
    }
    let normalized = root.replace('\\', "/").to_ascii_lowercase();
    if normalized
        .split('/')
        .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        return Err("mission locator package root allows traversal".to_owned());
    }
    Ok(normalized)
}

fn validate_member_path(package_root: &str, member_path: &str) -> Result<(), String> {
    if member_path.is_empty()
        || member_path != member_path.trim()
        || member_path.contains('\\')
        || member_path.contains(':')
        || member_path
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
        || member_path.chars().any(char::is_control)
    {
        return Err("locator member path is malformed".to_owned());
    }
    let expected_prefix = format!("{package_root}/components/srr_locator/");
    if !member_path.starts_with(&expected_prefix)
        || !Path::new(member_path)
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
    {
        return Err("locator member path escaped its package locator namespace".to_owned());
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_locator/tests.rs"]
mod tests;

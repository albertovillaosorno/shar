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
//   - Canonical package binding for explicit mission LoadP3DFile commands.
// - Must-Not:
//   - Read files, infer implicit loads, or interpret optional load groups.
// - Allows:
//   - Bind exact authored P3D paths to phase-three package identities.
// - Split-When:
//   - Active mission package lifetime gains an independent state model.
// - Merge-When:
//   - Mission package loading is owned directly by final runtime compilation.
// - Summary:
//   - Mission P3D package-load semantic preflight.
// - Description:
//   - Converts explicit LoadP3DFile path arguments into canonical package ids
//     while preserving the optional second source argument as opaque evidence.
// - Usage:
//   - Called after normalized mission-script and package-index intake.
// - Defaults:
//   - Unsupported arity, unsafe paths, or unindexed P3D loads fail closed.
//

//! Canonical package binding for explicit mission P3D loads.

use std::collections::BTreeMap;
use std::path::Path;

use super::{MissionScriptEvidence, PhaseThreePackageIndex};

/// One exact `LoadP3DFile` command bound to one indexed package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPackageLoadBinding {
    ordinal: usize,
    source_reference: String,
    source_group: Option<String>,
    package_id: String,
    package_root: String,
}

impl MissionPackageLoadBinding {
    /// Return the source command ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// Return the exact authored P3D path argument.
    #[must_use]
    pub fn source_reference(&self) -> &str {
        &self.source_reference
    }

    /// Return the optional second authored argument without interpretation.
    #[must_use]
    pub fn source_group(&self) -> Option<&str> {
        self.source_group.as_deref()
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

/// Deterministic explicit package-load report for one normalized script.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPackageLoadReport {
    bindings: Vec<MissionPackageLoadBinding>,
}

impl MissionPackageLoadReport {
    /// Return explicit `LoadP3DFile` bindings in source order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionPackageLoadBinding] {
        &self.bindings
    }
}

/// Bind all explicit `LoadP3DFile` commands to canonical package identities.
///
/// # Errors
///
/// Returns an error for unsupported command arity, unsafe/non-P3D paths,
/// duplicate normalized package roots, or a P3D path absent from the index.
pub fn preflight_mission_package_loads(
    evidence: &MissionScriptEvidence,
    index: &PhaseThreePackageIndex,
) -> Result<MissionPackageLoadReport, String> {
    let packages = indexed_package_roots(index)?;
    let mut bindings = Vec::new();
    for invocation in evidence
        .invocations()
        .iter()
        .filter(|invocation| invocation.name() == "loadp3dfile")
    {
        let arguments = invocation.arguments();
        if !(1..=2).contains(&arguments.len()) {
            return Err("LoadP3DFile must have one or two arguments".to_owned());
        }
        let source_reference = arguments
            .first()
            .cloned()
            .ok_or_else(|| "LoadP3DFile first argument is missing".to_owned())?;
        let package_root = normalized_p3d_package_root(&source_reference)?;
        let package = packages
            .get(&package_root)
            .ok_or_else(|| "LoadP3DFile reference has no canonical package".to_owned())?;
        let source_group = arguments.get(1).cloned();
        if let Some(group) = source_group.as_deref() {
            validate_opaque_group(group)?;
        }
        bindings.push(MissionPackageLoadBinding {
            ordinal: invocation.ordinal(),
            source_reference,
            source_group,
            package_id: package.0.clone(),
            package_root: package.1.clone(),
        });
    }
    Ok(MissionPackageLoadReport { bindings })
}

fn indexed_package_roots(
    index: &PhaseThreePackageIndex,
) -> Result<BTreeMap<String, (String, String)>, String> {
    let mut roots = BTreeMap::new();
    for package in index.packages() {
        let Some(normalized) = normalized_candidate_package_root(&package.package_root)? else {
            continue;
        };
        if roots
            .insert(
                normalized,
                (package.package_id.clone(), package.package_root.clone()),
            )
            .is_some()
        {
            return Err("phase-three package roots collide after normalization".to_owned());
        }
    }
    Ok(roots)
}

fn normalized_p3d_package_root(reference: &str) -> Result<String, String> {
    if reference.is_empty()
        || reference != reference.trim()
        || reference.chars().any(char::is_control)
    {
        return Err("LoadP3DFile path is blank, padded, or control-bearing".to_owned());
    }
    let normalized = reference.replace(char::from(92), "/").to_ascii_lowercase();
    validate_relative_transport_path(&normalized)?;
    if !Path::new(&normalized)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("p3d"))
    {
        return Err("LoadP3DFile first argument is not a P3D path".to_owned());
    }
    let without_extension = normalized
        .strip_suffix(".p3d")
        .ok_or_else(|| "LoadP3DFile P3D extension normalization failed".to_owned())?;
    Ok(format!("extracted/{without_extension}"))
}

fn normalized_candidate_package_root(root: &str) -> Result<Option<String>, String> {
    if root.is_empty() || root != root.trim() || root.chars().any(char::is_control) {
        return Err("phase-three package root is malformed".to_owned());
    }
    let normalized = root.replace(char::from(92), "/").to_ascii_lowercase();
    if normalized == "extracted" || !normalized.starts_with("extracted/") {
        return Ok(None);
    }
    Ok(Some(normalized_package_root(&normalized)?))
}

fn normalized_package_root(root: &str) -> Result<String, String> {
    if root.is_empty() || root != root.trim() || root.chars().any(char::is_control) {
        return Err("phase-three package root is malformed".to_owned());
    }
    let normalized = root.replace(char::from(92), "/").to_ascii_lowercase();
    if !normalized.starts_with("extracted/") {
        return Err("phase-three package root is outside extracted evidence".to_owned());
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
        return Err("mission package load path is unsafe".to_owned());
    }
    Ok(())
}

fn validate_opaque_group(group: &str) -> Result<(), String> {
    if group.is_empty() || group != group.trim() || group.chars().any(char::is_control) {
        return Err("LoadP3DFile optional source group is malformed".to_owned());
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_load/tests.rs"]
mod tests;

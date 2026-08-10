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
//   - Read files, infer implicit loads, or preserve legacy heaps as target
//     allocation authority.
// - Allows:
//   - Bind exact authored P3D paths to phase-three package identities.
//   - Preserve validated heap and inventory-section source provenance.
// - Split-When:
//   - Active mission package lifetime gains an independent state model.
// - Merge-When:
//   - Mission package loading is owned directly by final runtime compilation.
// - Summary:
//   - Mission P3D package-load semantic preflight.
// - Description:
//   - Converts explicit LoadP3DFile paths into canonical package ids while
//     preserving optional heap and inventory-section source provenance.
// - Usage:
//   - Called after normalized mission-script and package-index intake.
// - Defaults:
//   - Unsupported arity, unsafe paths, or unindexed P3D loads fail closed.
//

//! Canonical package binding for explicit mission P3D loads.

use super::{
    MissionP3dReferenceCatalog, MissionScriptEvidence, PhaseThreePackageIndex,
};

#[cfg(test)]
use super::mission_p3d_reference::{
    normalized_candidate_package_root, normalized_p3d_package_root,
    normalized_package_root,
};

/// One exact `LoadP3DFile` command bound to one indexed package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPackageLoadBinding {
    ordinal: usize,
    source_reference: String,
    source_heap: Option<String>,
    source_inventory_section: Option<String>,
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

    /// Return the optional validated source-era heap name.
    #[must_use]
    pub fn source_heap(&self) -> Option<&str> {
        self.source_heap.as_deref()
    }

    /// Return the optional source-era inventory-section override.
    #[must_use]
    pub fn source_inventory_section(&self) -> Option<&str> {
        self.source_inventory_section.as_deref()
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
    let catalog = MissionP3dReferenceCatalog::from_package_index(index)?;
    preflight_mission_package_loads_with_catalog(evidence, &catalog)
}

/// Bind explicit `LoadP3DFile` commands through a prebuilt P3D catalog.
///
/// # Errors
///
/// Returns an error for unsupported command arity, unsafe/non-P3D paths,
/// unknown heap names, malformed section overrides, or paths absent from the
/// canonical catalog.
pub fn preflight_mission_package_loads_with_catalog(
    evidence: &MissionScriptEvidence,
    catalog: &MissionP3dReferenceCatalog,
) -> Result<MissionPackageLoadReport, String> {
    let mut bindings = Vec::new();
    for invocation in evidence
        .invocations()
        .iter()
        .filter(|invocation| invocation.name() == "loadp3dfile")
    {
        let arguments = invocation.arguments();
        if !(1..=3).contains(&arguments.len()) {
            return Err("LoadP3DFile must have one to three arguments".to_owned());
        }
        let source_reference = arguments
            .first()
            .cloned()
            .ok_or_else(|| "LoadP3DFile first argument is missing".to_owned())?;
        let package = catalog.resolve(&source_reference)?;
        let source_heap = arguments.get(1).cloned();
        if let Some(heap) = source_heap.as_deref() {
            validate_legacy_heap(heap)?;
        }
        let source_inventory_section = arguments.get(2).cloned();
        if let Some(section) = source_inventory_section.as_deref() {
            validate_inventory_section(section)?;
        }
        bindings.push(MissionPackageLoadBinding {
            ordinal: invocation.ordinal(),
            source_reference,
            source_heap,
            source_inventory_section,
            package_id: package.package_id().to_owned(),
            package_root: package.package_root().to_owned(),
        });
    }
    Ok(MissionPackageLoadReport { bindings })
}

fn validate_legacy_heap(heap: &str) -> Result<(), String> {
    const KNOWN_HEAPS: [&str; 15] = [
        "GMA_DEFAULT",
        "GMA_TEMP",
        "GMA_GC_VMM",
        "GMA_PERSISTENT",
        "GMA_LEVEL",
        "GMA_LEVEL_MOVIE",
        "GMA_LEVEL_FE",
        "GMA_LEVEL_ZONE",
        "GMA_LEVEL_OTHER",
        "GMA_LEVEL_HUD",
        "GMA_LEVEL_MISSION",
        "GMA_LEVEL_AUDIO",
        "GMA_DEBUG",
        "GMA_SPECIAL",
        "GMA_XBOX_SOUND_MEMORY",
    ];
    if KNOWN_HEAPS.contains(&heap) {
        Ok(())
    } else {
        Err("LoadP3DFile optional heap name is not recognized".to_owned())
    }
}

fn validate_inventory_section(section: &str) -> Result<(), String> {
    if section.is_empty()
        || section != section.trim()
        || section.chars().any(char::is_control)
    {
        return Err(
            "LoadP3DFile optional inventory section is malformed".to_owned(),
        );
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_load/tests.rs"]
mod tests;

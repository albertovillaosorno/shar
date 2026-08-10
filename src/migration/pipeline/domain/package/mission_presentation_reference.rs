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
//   - Canonical package binding for typed mission presentation P3D references.
// - Must-Not:
//   - Infer UI timing, drawable identity, or runtime presentation behavior.
// - Allows:
//   - Resolve typed SetPresentationBitmap paths through the shared P3D catalog.
// - Split-When:
//   - Presentation assets gain an independent runtime or Unreal lifecycle.
// - Merge-When:
//   - Final mission-definition compilation owns this exact reference boundary.
// - Summary:
//   - Mission presentation package-reference preflight.
// - Description:
//   - Binds initialization, stage, and objective presentation bitmap paths to
//     canonical phase-three package identities before mission asset emission.
// - Usage:
//   - Runs after typed mission semantic reports and P3D catalog construction.
// - Defaults:
//   - Missing or unsafe presentation P3D paths fail closed.
//

//! Canonical package binding for mission presentation bitmap references.

use super::{
    MissionInitializationDirective, MissionInitializationReport,
    MissionObjectiveDirective, MissionObjectiveSemanticReport,
    MissionP3dReferenceCatalog, MissionStageDirective,
    MissionStageSemanticReport,
};

/// Semantic scope that authored one presentation bitmap reference.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MissionPresentationRole {
    /// Mission-scope initialization command.
    Initialization,
    /// Stage-scope command.
    Stage,
    /// Objective-scope command.
    Objective,
}

/// One typed presentation bitmap bound to a canonical package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPresentationPackageReference {
    source_ordinal: usize,
    role: MissionPresentationRole,
    source_reference: String,
    package_id: String,
    package_root: String,
}

impl MissionPresentationPackageReference {
    /// Return the source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the semantic scope that owns this reference.
    #[must_use]
    pub const fn role(&self) -> MissionPresentationRole {
        self.role
    }

    /// Return the exact authored P3D path.
    #[must_use]
    pub fn source_reference(&self) -> &str {
        &self.source_reference
    }

    /// Return the canonical package id.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the canonical package root.
    #[must_use]
    pub fn package_root(&self) -> &str {
        &self.package_root
    }
}

/// Canonical presentation package references for one normalized mission source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPresentationReferenceReport {
    bindings: Vec<MissionPresentationPackageReference>,
}

impl MissionPresentationReferenceReport {
    /// Return references sorted by source ordinal and semantic scope.
    #[must_use]
    pub fn bindings(&self) -> &[MissionPresentationPackageReference] {
        &self.bindings
    }
}

/// Bind every typed presentation bitmap to one canonical P3D package.
///
/// # Errors
///
/// Returns an error when any typed presentation path is unsafe, non-P3D, or
/// absent from the shared canonical P3D package catalog.
pub fn preflight_mission_presentation_references(
    catalog: &MissionP3dReferenceCatalog,
    initialization: &MissionInitializationReport,
    stages: &MissionStageSemanticReport,
    objectives: &MissionObjectiveSemanticReport,
) -> Result<MissionPresentationReferenceReport, String> {
    let mut bindings = Vec::new();
    for mission in initialization.missions() {
        for directive in mission.directives() {
            if let MissionInitializationDirective::PresentationBitmap {
                source_ordinal,
                p3d_path,
            } = directive
            {
                push_binding(
                    &mut bindings,
                    catalog,
                    *source_ordinal,
                    MissionPresentationRole::Initialization,
                    p3d_path,
                )?;
            }
        }
    }
    for stage in stages.stages() {
        for directive in stage.directives() {
            if let MissionStageDirective::StagePresentationBitmap {
                source_ordinal,
                p3d_path,
            } = directive
            {
                push_binding(
                    &mut bindings,
                    catalog,
                    *source_ordinal,
                    MissionPresentationRole::Stage,
                    p3d_path,
                )?;
            }
        }
    }
    for objective in objectives.objectives() {
        for directive in objective.directives() {
            if let MissionObjectiveDirective::PresentationBitmap {
                source_ordinal,
                p3d_path,
            } = directive
            {
                push_binding(
                    &mut bindings,
                    catalog,
                    *source_ordinal,
                    MissionPresentationRole::Objective,
                    p3d_path,
                )?;
            }
        }
    }
    bindings.sort_by_key(|binding| (binding.source_ordinal, binding.role));
    Ok(MissionPresentationReferenceReport { bindings })
}

fn push_binding(
    bindings: &mut Vec<MissionPresentationPackageReference>,
    catalog: &MissionP3dReferenceCatalog,
    source_ordinal: usize,
    role: MissionPresentationRole,
    source_reference: &str,
) -> Result<(), String> {
    let reference = catalog.resolve(source_reference)?;
    bindings.push(MissionPresentationPackageReference {
        source_ordinal,
        role,
        source_reference: reference.source_reference().to_owned(),
        package_id: reference.package_id().to_owned(),
        package_root: reference.package_root().to_owned(),
    });
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_presentation_reference/tests.rs"]
mod tests;

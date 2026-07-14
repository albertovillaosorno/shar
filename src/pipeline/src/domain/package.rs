// File:
//   - package.rs
// Path:
//   - src/pipeline/src/domain/package.rs
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
//   - The package contract for pipeline phase three.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute package.
// - Split-When:
//   - Split when package contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Phase-three package intake.
// - Description:
//   - Defines package data and behavior for pipeline phase three.
// - Usage:
//   - Used by pipeline phase three code that needs package.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Phase-three package intake.
//! Phase-three package intake.
//!
//! Phase three starts from the package index instead of extraction internals.
//! The modules below keep package reading and conversion planning reusable so
//! future CLI commands, FBX adapters, and Unreal adapters share one contract.

/// Package-index reader.
pub mod index;
/// Package conversion planner.
pub mod plan;
/// Typed package selectors.
pub mod selector;

// Re-exporting the domain-qualified names keeps downstream imports explicit
// while preserving one public package boundary instead of exposing file layout.
#[expect(
    clippy::module_name_repetitions,
    reason = "Re-exports preserve explicit package-domain names for \
              downstream callers."
)]
pub use index::{
    PackageMemberRef, PackageRole, PhaseThreePackageIndex,
    PhaseThreePackageMember, PhaseThreePackageRow,
};
pub use plan::{
    ConversionFamily, FbxModelPlan, PhaseThreePackagePlan,
    PhaseThreePackagePlanner, UnrealNativePlan, UnrealTargetKind,
};
pub use selector::PhaseThreePackageSelector;

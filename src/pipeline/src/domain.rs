// File:
//   - domain.rs
// Path:
//   - src/pipeline/src/domain.rs
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
//   - Pure pipeline state, output evidence, and JSON encoding support.
// - Must-Not:
//   - Perform IO, invoke tools, or select concrete adapters.
// - Allows:
//   - Expose stable values used across pipeline application slices.
// - Split-When:
//   - Split when one domain family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same pipeline invariants.
// - Summary:
//   - Pipeline domain facade.
// - Description:
//   - Exposes pure state and encoding behavior to inward-facing layers.
// - Usage:
//   - Imported by application, ports, adapters, and public callers.
// - Defaults:
//   - Domain modules have no process or storage side effects.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Pure domain values for the extraction and planning pipeline.
//!
//! Filesystem and process behavior remain in adapters.
mod json;
mod output_summary;
pub mod package;
mod pipeline;

/// Escapes text for insertion inside one JSON string value.
pub(crate) fn escape_json(value: &str) -> String {
    json::escape(value)
}
pub use output_summary::{DirectorySummary, OutputSummary};
pub use package::{
    ConversionFamily, FbxModelPlan, PackageMemberRef, PackageRole,
    PhaseThreePackageIndex, PhaseThreePackageMember, PhaseThreePackagePlan,
    PhaseThreePackagePlanner, PhaseThreePackageRow, PhaseThreePackageSelector,
    UnrealNativePlan, UnrealTargetKind,
};
pub use pipeline::{
    PipelineConfig, PipelineError, PipelineOutcome, PipelineReport, StageReport,
};

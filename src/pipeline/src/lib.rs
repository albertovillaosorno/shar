// File:
//   - lib.rs
// Path:
//   - src/pipeline/src/lib.rs
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
//   - The pipeline public hexagonal facade and compatibility re-exports.
// - Must-Not:
//   - Implement use cases, providers, process parsing, or domain policy.
// - Allows:
//   - Expose domain, application, ports, adapters, and stable public APIs.
// - Split-When:
//   - Split when one public API family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same crate-level contracts.
// - Summary:
//   - Pipeline public hexagonal facade.
// - Description:
//   - Keeps layer ownership explicit while preserving deliberate caller
//   - APIs.
// - Usage:
//   - Imported by workspace crates and the pipeline process entrypoint.
// - Defaults:
//   - Concrete adapters are selected only by driving composition.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Public hexagonal facade for the extraction and planning pipeline.
//!
//! Compatibility re-exports preserve stable callers without exposing old paths.
pub mod adapters;
pub mod application;
pub mod domain;
pub mod ports;

pub use application::{PipelineService, SummarizeOutput};
pub use domain::{
    ConversionFamily, DirectorySummary, FbxModelPlan, OutputSummary,
    PackageMemberRef, PackageRole, PhaseThreePackageIndex,
    PhaseThreePackagePlan, PhaseThreePackagePlanner, PhaseThreePackageRow,
    PhaseThreePackageSelector, PipelineConfig, PipelineError, PipelineOutcome,
    PipelineReport, StageReport, UnrealNativePlan, UnrealTargetKind,
};
pub use ports::{OutputInventory, PipelineOperations};

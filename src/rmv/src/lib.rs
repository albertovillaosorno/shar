// File:
//   - lib.rs
// Path:
//   - src/rmv/src/lib.rs
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
//   - The rmv public library facade for lib.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute public crate facade.
// - Split-When:
//   - Split when public crate facade contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another rmv module owns the same library facade boundary with no distinct
//   - invariant.
// - Summary:
//   - Exposes the RMV library surface for deterministic video-audit workflows.
// - Description:
//   - Defines public crate facade data and behavior for rmv root.
// - Usage:
//   - Imported by workspace crates through the public library surface.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! The RMV library facade exposes planning modules without choosing a runtime
//! encoder.
pub mod adapters;
pub mod application;
#[path = "domain/domain.rs"]
pub mod domain;
/// Outbound contracts for source discovery and artifact publication.
pub mod ports;

pub use adapters::{FilesystemMovieAuditor, TsvAuditManifestSink};
pub use application::{
    AuditReport, MovieAuditor, MovieRecord, RunMovieAudit,
    RuntimeCompletionAction, RuntimeCompletionPlan, RuntimeCompletionPlanner,
    UnrealHapPackagePlan,
};
pub use domain::{
    CinematicTarget, MovieEvidence, MovieKind, RmvError,
    RuntimeCompletionDecision, RuntimeCompletionRule, RuntimeMovieCandidate,
    Sha256, TargetDecision,
};

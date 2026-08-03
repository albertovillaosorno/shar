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
//   - Rmv lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Rmv lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Rmv lib.rs.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application/mod.rs"]
pub mod application;
#[path = "../domain/mod.rs"]
pub mod domain;
/// Outbound contracts for source discovery and artifact publication.
#[path = "ports/mod.rs"]
pub mod ports;
mod sha256;

pub use adapters::{FilesystemMovieAuditor, TsvAuditManifestSink};
pub use application::{
    AuditReport, MovieAuditor, MovieRecord, RunMovieAudit,
    RuntimeCompletionAction, RuntimeCompletionPlan, RuntimeCompletionPlanner,
    UnrealHapPackagePlan,
};
pub use domain::{
    CinematicTarget, IoFailure, MovieEvidence, MovieKind, RmvError,
    RuntimeCompletionDecision, RuntimeCompletionRule, RuntimeMovieCandidate,
    Sha256, TargetDecision,
};

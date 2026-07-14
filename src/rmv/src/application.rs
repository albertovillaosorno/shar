// File:
//   - application.rs
// Path:
//   - src/rmv/src/application.rs
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
//   - rmv module behavior for application.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute application.
// - Split-When:
//   - Split when application contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - RMV audit and conversion planning use cases.
// - Description:
//   - Defines application data and behavior for rmv root.
// - Usage:
//   - Used by rmv root code that needs application.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! RMV audit and conversion planning use cases.
//!
//! This boundary keeps rmv audit and conversion planning use cases explicit
//! and returns deterministic results to rmv callers.
mod audit;
mod package_plan;
mod runtime_completion;

pub use audit::RunMovieAudit;
pub use package_plan::UnrealHapPackagePlan;
pub use runtime_completion::{
    RuntimeCompletionAction, RuntimeCompletionPlan, RuntimeCompletionPlanner,
};

pub use crate::domain::{AuditReport, MovieRecord};
pub use crate::ports::MovieAuditor;

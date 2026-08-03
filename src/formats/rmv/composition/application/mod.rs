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
//   - Application application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Application application service.
// - Description:
//   - Implements the declared application service responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Application application service.

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

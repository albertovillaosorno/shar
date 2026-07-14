// File:
//   - driving.rs
// Path:
//   - src/pipeline/src/adapters/driving.rs
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
//   - Inbound adapters translating operator requests into pipeline use
//   - cases.
// - Must-Not:
//   - Implement filesystem providers or pipeline phase behavior.
// - Allows:
//   - Expose the command-line composition entrypoint.
// - Split-When:
//   - Split when another inbound protocol gains a distinct adapter family.
// - Merge-When:
//   - Another facade owns the same inbound adapters.
// - Summary:
//   - Pipeline driving-adapter facade.
// - Description:
//   - Publishes operator-facing composition without process duplication.
// - Usage:
//   - Imported by the thin binary and integration tests.
// - Defaults:
//   - Current-process execution delegates to `schoenwald-cli`.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Inbound adapters for pipeline application use cases.
//!
//! Process mechanics are delegated to the shared CLI crate.
mod cli;

pub use cli::{PipelineCli, run_env};

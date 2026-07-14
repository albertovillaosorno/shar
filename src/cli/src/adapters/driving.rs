// File:
//   - driving.rs
// Path:
//   - src/cli/src/adapters/driving.rs
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
//   - Inbound current-process composition for shared CLI commands.
// - Must-Not:
//   - Implement output sinks or caller command policy.
// - Allows:
//   - Expose one minimal process runner.
// - Split-When:
//   - Split when another inbound protocol gains a distinct adapter family.
// - Merge-When:
//   - Another facade owns the same process composition surface.
// - Summary:
//   - Shared CLI driving-adapter facade.
// - Description:
//   - Publishes current-process composition without hiding the use case.
// - Usage:
//   - Imported by the crate facade and thin binaries.
// - Defaults:
//   - Uses environment arguments and standard streams.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driving adapters for shared CLI mechanisms.
//!
//! Composition selects process providers while caller policy stays external.
mod process;

pub use process::run_process;

// File:
//   - adapters.rs
// Path:
//   - src/cli/src/adapters.rs
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
//   - Shared CLI inbound and outbound adapter families.
// - Must-Not:
//   - Own command policy or process-neutral domain values.
// - Allows:
//   - Separate current-process composition from process mechanisms.
// - Split-When:
//   - Split when one adapter family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same adapter families.
// - Summary:
//   - Shared CLI adapter facade.
// - Description:
//   - Exposes driving process composition and driven process providers.
// - Usage:
//   - Imported by the public crate facade and integration tests.
// - Defaults:
//   - Core layers select no concrete adapter.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Inbound and outbound adapters for shared CLI mechanisms.
//!
//! Driving adapters compose use cases while driven adapters implement ports.
mod driven;
mod driving;

pub use driven::{EnvironmentArguments, StandardStreams};
pub use driving::run_process;

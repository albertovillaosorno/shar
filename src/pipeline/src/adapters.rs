// File:
//   - adapters.rs
// Path:
//   - src/pipeline/src/adapters.rs
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
//   - Pipeline driving and driven adapter families.
// - Must-Not:
//   - Own domain state or application orchestration.
// - Allows:
//   - Separate process composition from provider implementations.
// - Split-When:
//   - Split when one adapter family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same pipeline adapters.
// - Summary:
//   - Pipeline adapter facade.
// - Description:
//   - Exposes inbound CLI composition and outbound provider adapters.
// - Usage:
//   - Imported by the binary, application composition, and tests.
// - Defaults:
//   - Core layers select no concrete provider.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Inbound and outbound adapters for the pipeline crate.
//!
//! Process and storage mechanisms remain outside domain and application code.
pub mod driven;
pub mod driving;

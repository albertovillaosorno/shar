// File:
//   - adapters.rs
// Path:
//   - src/filesystem/src/adapters.rs
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
//   - Shared filesystem inbound and outbound adapter families.
// - Must-Not:
//   - Own pure invariants or caller-specific application policy.
// - Allows:
//   - Separate local composition from concrete storage providers.
// - Split-When:
//   - Split when one adapter family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same adapter families.
// - Summary:
//   - Shared filesystem adapter facade.
// - Description:
//   - Exposes driving composition and driven provider implementations.
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

//! Inbound and outbound adapters for shared filesystem mechanisms.
//!
//! Driving adapters compose use cases while driven adapters implement ports.
pub mod driven;
pub mod driving;

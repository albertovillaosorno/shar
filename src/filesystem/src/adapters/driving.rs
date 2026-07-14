// File:
//   - driving.rs
// Path:
//   - src/filesystem/src/adapters/driving.rs
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
//   - Inbound composition adapters for filesystem mechanisms.
// - Must-Not:
//   - Implement storage providers or caller-specific workflow policy.
// - Allows:
//   - Expose local application composition to consuming adapters.
// - Split-When:
//   - Split when another inbound protocol needs a distinct composition adapter.
// - Merge-When:
//   - Another facade owns the same inbound composition surface.
// - Summary:
//   - Filesystem driving-adapter facade.
// - Description:
//   - Publishes local composition without hiding application use cases.
// - Usage:
//   - Imported by the crate facade and advanced callers.
// - Defaults:
//   - Local composition uses the standard provider.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driving adapters for shared filesystem mechanisms.
//!
//! Composition selects providers while preserving explicit caller policy.
pub mod local;

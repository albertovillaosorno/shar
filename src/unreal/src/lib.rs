// File:
//   - lib.rs
// Path:
//   - src/unreal/src/lib.rs
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
//   - The public facade for deterministic Unreal asset conversion planning.
// - Must-Not:
//   - Connect to Unreal Engine, MCP, HTTP, editor processes, or live project
//   - state.
// - Allows:
//   - Pure conversion models and ports consumed by pipeline orchestration.
// - Split-When:
//   - Another independently versioned conversion family requires a separate
//   - crate.
// - Merge-When:
//   - Another facade owns the same JSON, WAV, HAP, and FBX conversion boundary.
// - Summary:
//   - Unreal asset-conversion planning library.
// - Description:
//   - Exposes pure conversion plans and artifact-storage contracts for
//   - pipeline.
// - Usage:
//   - Imported by pipeline code that translates normalized evidence into
//   - native Unreal target plans.
// - Defaults:
//   - No IO, process, network, MCP, or editor behavior is selected implicitly.
//
// ADRs:
// - docs/adr/unreal/architecture.md
// - docs/adr/pipeline/eleven-phase-remake-delivery-roadmap.md
//
// Large file:
//   - false
//

//! Deterministic conversion planning for native Unreal assets.
//!
//! The crate accepts normalized JSON, PCM WAV, HAP, and binary FBX 7.7
//! evidence.
//! It never connects to Unreal Engine or an MCP server.
/// Pure conversion domain.
#[path = "domain/domain.rs"]
pub mod domain;
/// Conversion artifact-storage ports.
pub mod ports;

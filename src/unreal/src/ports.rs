// File:
//   - ports.rs
// Path:
//   - src/unreal/src/ports.rs
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
//   - Hexagonal ports for persisted conversion artifacts.
// - Must-Not:
//   - Define MCP, editor, process, network, or runtime-gameplay interfaces.
// - Allows:
//   - Storage contracts for deterministic conversion plans and reports.
// - Split-When:
//   - Binary and text conversion artifacts require independent storage ports.
// - Merge-When:
//   - Another port owns the same persisted conversion artifact contract.
// - Summary:
//   - Unreal conversion ports facade.
// - Description:
//   - Exposes storage boundaries used by pipeline-owned adapters.
// - Usage:
//   - Implemented outside this crate by pipeline composition roots.
// - Defaults:
//   - No storage adapter is selected implicitly.
//
// ADRs:
// - docs/adr/unreal/architecture.md
//
// Large file:
//   - false
//

//! Storage ports for deterministic Unreal conversion artifacts.
//!
//! Pipeline composition roots provide the concrete storage adapters.
/// Conversion artifact storage boundary.
pub mod artifact_store;

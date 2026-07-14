// File:
//   - domain.rs
// Path:
//   - src/unreal/src/domain/domain.rs
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
//   - Pure domain models for Unreal asset conversion planning.
// - Must-Not:
//   - Perform filesystem, process, network, MCP, or editor operations.
// - Allows:
//   - Validated value objects and deterministic evidence-to-plan translation.
// - Split-When:
//   - A conversion family gains independently testable domain invariants.
// - Merge-When:
//   - Another domain module owns the same conversion-plan invariant.
// - Summary:
//   - Unreal conversion domain facade.
// - Description:
//   - Exposes conversion plans for JSON, WAV, HAP, and FBX evidence.
// - Usage:
//   - Consumed by pipeline application code before any editor-side operation.
// - Defaults:
//   - Unsupported source extensions fail closed.
//
// ADRs:
// - docs/adr/unreal/architecture.md
//
// Large file:
//   - false
//

//! Pure domain models for deterministic Unreal asset conversion.
/// Source-format validation and native target planning.
pub mod conversion_plan;

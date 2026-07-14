// File:
//   - driving.rs
// Path:
//   - src/game-manifest/src/adapters/driving.rs
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
//   - Inbound CLI adapters for all game-manifest commands.
// - Must-Not:
//   - Traverse filesystems or publish artifacts without application commands.
// - Allows:
//   - Decode arguments, compose adapters, and present results.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter family.
// - Merge-When:
//   - Another facade owns the same inbound command contracts.
// - Summary:
//   - Driving adapter facade for manifest commands.
// - Description:
//   - Exposes generator, validator, expanded-ledger, and audit CLIs.
// - Usage:
//   - Called by thin executable entrypoints.
// - Defaults:
//   - Legacy path defaults remain local to CLI adapters.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driving adapters for game-manifest operator commands.
//!
//! Each command owns only inbound decoding, composition, and presentation.
pub mod expanded_cli;
pub mod generate_cli;
pub mod structural_audit_cli;
mod support;
pub mod validate_cli;

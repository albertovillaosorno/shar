// File:
//   - driving.rs
// Path:
//   - src/rmv/src/adapters/driving.rs
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
//   - Inbound adapters translating operator requests into RMV use cases.
// - Must-Not:
//   - Discover movies or publish artifacts without application orchestration.
// - Allows:
//   - CLI decoding, dependency composition, and result presentation.
// - Split-When:
//   - Split when another inbound protocol needs its own adapter family.
// - Merge-When:
//   - Another facade owns the same inbound request contract.
// - Summary:
//   - Driving adapter facade for RMV commands.
// - Description:
//   - Exposes the CLI composition root outside the process entrypoint.
// - Usage:
//   - Called by the thin RMV executable or adapter-level tests.
// - Defaults:
//   - Invalid requests fail with deterministic usage diagnostics.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driving adapters for RMV operator requests.
//!
//! Process and presentation concerns stay outside domain, application, and port
//! modules.
pub mod cli;

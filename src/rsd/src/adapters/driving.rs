// File:
//   - driving.rs
// Path:
//   - src/rsd/src/adapters/driving.rs
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
//   - Inbound adapters translating operator requests into RSD use cases.
// - Must-Not:
//   - Traverse source trees or publish WAV files directly.
// - Allows:
//   - CLI decoding, dependency composition, and report presentation.
// - Split-When:
//   - Split when another inbound protocol needs its own adapter family.
// - Merge-When:
//   - Another facade owns the same inbound request contract.
// - Summary:
//   - Driving adapter facade for RSD commands.
// - Description:
//   - Exposes the CLI composition root outside the process entrypoint.
// - Usage:
//   - Called by the thin RSD executable or adapter-level tests.
// - Defaults:
//   - Invalid requests fail with deterministic usage diagnostics.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driving adapters for RSD operator requests.
//!
//! Process and presentation concerns remain outside the core layers.
pub mod cli;

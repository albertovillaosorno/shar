// File:
//   - driving.rs
// Path:
//   - src/lmlm/src/adapters/driving.rs
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
//   - Inbound adapters translating operator requests into LMLM use cases.
// - Must-Not:
//   - Parse archive bytes or materialize entries directly.
// - Allows:
//   - CLI decoding, dependency composition, and result presentation.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter family.
// - Merge-When:
//   - Another facade owns the same inbound request contract.
// - Summary:
//   - Driving adapter facade for LMLM extraction.
// - Description:
//   - Exposes the CLI composition root outside the process entrypoint.
// - Usage:
//   - Called by the thin executable or adapter-level tests.
// - Defaults:
//   - Invalid requests fail with deterministic usage diagnostics.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driving adapters for LMLM operator requests.
//!
//! Process and presentation concerns remain outside the core layers.
pub mod cli;

// File:
//   - driving.rs
// Path:
//   - src/rcf/src/adapters/driving.rs
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
//   - Inbound adapters that translate operator requests into RCF use cases.
// - Must-Not:
//   - Parse archive bytes or perform filesystem extraction directly.
// - Allows:
//   - CLI argument decoding, composition, and presentation of use-case results.
// - Split-When:
//   - Split when another inbound protocol needs its own adapter family.
// - Merge-When:
//   - Another driving facade owns the same inbound request contract.
// - Summary:
//   - Driving adapter facade for the RCF command line.
// - Description:
//   - Exposes the CLI composition root without leaking it into the binary.
// - Usage:
//   - Called by the thin `rcf` executable or by adapter-level tests.
// - Defaults:
//   - Invalid invocations fail with deterministic usage text.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driving adapters for RCF operator requests.
//!
//! This facade keeps process and presentation concerns outside archive domain
//! and application modules.
pub mod cli;

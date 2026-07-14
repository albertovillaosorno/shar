// File:
//   - driving.rs
// Path:
//   - src/rtf/src/adapters/driving.rs
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
//   - Inbound adapters translating operator requests into RTF use cases.
// - Must-Not:
//   - Parse RTF bytes or write Markdown without ports.
// - Allows:
//   - CLI decoding, dependency composition, and result presentation.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter family.
// - Merge-When:
//   - Another facade owns the same inbound request contract.
// - Summary:
//   - Driving adapter facade for RTF conversion.
// - Description:
//   - Exposes the CLI composition root outside the process entrypoint.
// - Usage:
//   - Called by the thin executable or adapter-level tests.
// - Defaults:
//   - The legacy input default remains local to the CLI adapter.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driving adapters for RTF operator requests.
//!
//! Process defaults and presentation remain outside the core layers.
pub mod cli;

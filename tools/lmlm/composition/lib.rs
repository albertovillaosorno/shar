// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Public facade for LMLM compatibility parsing and conversion.
// - Must-Not:
//   - Read private imports implicitly or execute legacy package code.
// - Allows:
//   - Expose reviewed archive, batch, conversion, and report services.
// - Split-When:
//   - A public compatibility surface gains an independent lifecycle.
// - Merge-When:
//   - Another facade owns the identical compatibility API.
// - Summary:
//   - LMLM compatibility crate facade.
// - Description:
//   - Wires pure archive parsing to review-only conversion services.
// - Usage:
//   - Used by the tool CLI and external compatibility tests.
// - Defaults:
//   - Unsupported or ambiguous legacy input fails explicitly.
//

//! Standalone Rust compatibility converter for supported legacy LMLM mods.

#[path = "../domain/archive/mod.rs"]
pub mod archive;
pub mod batch;
pub mod convert;
pub mod report;

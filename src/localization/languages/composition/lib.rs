// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Public facade for canonical official-language mod composition.
// - Must-Not:
//   - Expose private source paths or invent unavailable localization.
// - Allows:
//   - Expose pure language records and deterministic export behavior.
// - Split-When:
//   - Public language query and export surfaces gain independent lifecycles.
// - Merge-When:
//   - Another facade owns the identical official-language API.
// - Summary:
//   - Canonical official-language crate facade.
// - Description:
//   - Combines the pure domain model with source-backed export.
// - Usage:
//   - Used by canonical language-mod generation and its integration tests.
// - Defaults:
//   - Missing source evidence fails closed before publication.
//

//! Canonical official-language mod composition.

#[path = "../domain/mod.rs"]
pub mod domain;
mod export;

pub use domain::{Language, LanguageManifest};
pub use export::{ExportError, export_language};

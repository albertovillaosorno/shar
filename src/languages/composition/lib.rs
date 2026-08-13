// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT

//! Canonical official-language mod composition.

#[path = "../domain/mod.rs"]
pub mod domain;

pub use domain::{ExportError, Language, LanguageManifest, export_language};

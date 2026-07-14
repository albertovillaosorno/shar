// File:
//   - model.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/localization/model.rs
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
//   - Normalized localization value objects.
// - Must-Not:
//   - Read files, classify package categories, or select output adapters.
// - Allows:
//   - Immutable source identity, decoded values, and overlay provenance.
// - Split-When:
//   - One value object gains behavior or an independent invariant.
// - Merge-When:
//   - Another localization model owns the same identity and provenance
//   - fields.
// - Summary:
//   - Value objects shared by localization parsers and planners.
// - Description:
//   - Carries decoded data without filesystem or package-index
//   - responsibilities.
// - Usage:
//   - Constructed by source adapters and consumed by phase-two derivation.
// - Defaults:
//   - Malformed source data fails closed without implicit output.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Localization value objects carry decoded source identity only.

use std::path::PathBuf;

/// Parsed `TextBible` package with every declared language payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct TextBibleDocument {
    /// Source package used to build the document.
    pub source_path: PathBuf,
    /// Package-level `TextBible` name.
    pub name: String,
    /// Language identifiers declared by the package header.
    pub declared_language_ids: String,
    /// Parsed language payloads owned by the package.
    pub languages: Vec<LanguageDocument>,
}

/// One decoded language payload from a `TextBible` package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LanguageDocument {
    /// Legacy one-letter language identifier.
    pub id: char,
    /// Stable language label used by downstream planners.
    pub language: &'static str,
    /// Source name stored in the language chunk.
    pub source_name: String,
    /// Nonzero hash modulus used by language keys.
    pub modulo: u32,
    /// Decoded text entries in source order.
    pub entries: Vec<LanguageEntry>,
}

/// One decoded language entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LanguageEntry {
    /// Hashed language key.
    pub hash: u32,
    /// Byte offset into the shared UTF-16 string buffer.
    pub offset: u32,
    /// Decoded language value.
    pub value: String,
}

/// One key-value row from a custom text overlay.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CustomTextEntry {
    /// Overlay key or explicit hexadecimal hash.
    pub key: String,
    /// Replacement text value.
    pub value: String,
    /// One-based source line used for diagnostics.
    pub line: usize,
}

/// Effective overlay value associated with one base language entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OverlayEntry {
    /// Base language hash.
    pub hash: u32,
    /// Base language byte offset.
    pub offset: u32,
    /// Effective text value after overlay selection.
    pub value: String,
    /// Stable provenance label for the effective value.
    pub value_source: &'static str,
    /// Matching overlay key when a replacement exists.
    pub overlay_key: Option<String>,
}

/// Overlay merge result with matched and unmatched records separated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct OverlayMerge {
    /// Effective entries in base-language order.
    pub entries: Vec<OverlayEntry>,
    /// Custom records that did not match a base hash.
    pub unmatched: Vec<CustomTextEntry>,
}

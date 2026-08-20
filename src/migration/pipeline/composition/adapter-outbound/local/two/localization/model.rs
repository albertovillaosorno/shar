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
//   - Model outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Model outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Model outbound adapter.

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

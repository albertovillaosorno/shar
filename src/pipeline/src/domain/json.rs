// File:
//   - json.rs
// Path:
//   - src/pipeline/src/domain/json.rs
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
//   - Lossless JSON string-content escaping for pipeline renderers.
// - Must-Not:
//   - Render complete JSON documents or replace source characters.
// - Allows:
//   - Encoding JSON string control characters and delimiters.
// - Split-When:
//   - Split when JSON document rendering requires a separate typed boundary.
// - Merge-When:
//   - Another pipeline module owns identical JSON string escaping.
// - Summary:
//   - Canonical JSON text escaping for pipeline-owned renderers.
// - Description:
//   - Preserves source text while encoding JSON delimiters and controls.
// - Usage:
//   - Imported privately by phase-one, phase-two, and phase-three renderers.
// - Defaults:
//   - Every input Unicode scalar is preserved or losslessly escaped.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Canonical lossless JSON string-content escaping.
//!
//! This boundary preserves every source character while encoding only the
//! delimiters and controls required inside a JSON string value.

/// Escape string content for insertion between JSON quotation marks.
pub(super) fn escape(value: &str) -> String {
    shar_json_text::escape(value)
}

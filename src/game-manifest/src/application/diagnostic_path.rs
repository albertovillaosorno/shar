// File:
//   - diagnostic_path.rs
// Path:
//   - src/game-manifest/src/application/diagnostic_path.rs
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
//   - The application-local compatibility function for diagnostic paths.
// - Must-Not:
//   - Reimplement native path escaping, access storage, or normalize identity.
// - Allows:
//   - Return the shared filesystem renderer as owned diagnostic text.
// - Split-When:
//   - Game-manifest requires a genuinely distinct path presentation grammar.
// - Merge-When:
//   - Application call sites can consume the shared display wrapper directly.
// - Summary:
//   - Shared diagnostic path compatibility facade.
// - Description:
//   - Keeps application call sites stable while the filesystem domain owns
//   - platform-aware and reversible native path rendering.
// - Usage:
//   - Used by game-manifest application errors and invalid-path diagnostics.
// - Defaults:
//   - Rendering behavior is exactly the shared filesystem contract.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Application-local facade for shared diagnostic path rendering.

use std::path::Path;

use schoenwald_filesystem::DiagnosticPath;

/// Renders one native path through the shared reversible contract.
#[must_use]
pub(super) fn escaped_path(path: &Path) -> String {
    DiagnosticPath::new(path).to_string()
}

/// Renders untrusted source text without raw control characters.
#[must_use]
pub(super) fn escaped_text(value: &str) -> String {
    value
        .chars()
        .flat_map(char::escape_default)
        .collect()
}

// File:
//   - source.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/text/source.rs
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
//   - Custom-text source intake for derived language packages.
// - Must-Not:
//   - Own encoding grammar, overlay semantics, or package classification
//   - policy.
// - Allows:
//   - Read one source and expose its validated keys to package derivation.
// - Split-When:
//   - Another source family requires an independent intake adapter.
// - Merge-When:
//   - The localization boundary consumes the same source for the same
//   - caller.
// - Summary:
//   - Validated custom-text keys for minor-unit package derivation.
// - Description:
//   - Delegates source grammar to phase-two localization normalization.
// - Usage:
//   - Called by language-text draft construction for one physical source
//   - unit.
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

//! Custom-text package derivation consumes the normalized localization
//! boundary.

use std::path::Path;

use super::super::localization::read_custom_text_keys as read_validated_keys;
use super::PipelineOutcome;

/// Read validated custom-text keys without duplicating source grammar.
pub(super) fn read_custom_text_keys(
    path: &Path
) -> PipelineOutcome<Vec<String>> {
    read_validated_keys(path)
}

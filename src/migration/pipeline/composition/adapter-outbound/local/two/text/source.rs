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
//   - Source outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Source outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source outbound adapter.

use std::path::Path;

use super::super::localization::read_custom_text_keys as read_validated_keys;
use super::PipelineOutcome;

/// Read validated custom-text keys without duplicating source grammar.
pub(super) fn read_custom_text_keys(
    path: &Path,
) -> PipelineOutcome<Vec<String>> {
    read_validated_keys(path)
}

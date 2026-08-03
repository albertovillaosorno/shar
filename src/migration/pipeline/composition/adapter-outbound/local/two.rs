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
//   - Two outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Two outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Two outbound adapter.

mod text;
// Retain the complete localization decoder family and its focused regression
// suite until a dedicated port consumes every decoder variant.
#[expect(
    dead_code,
    unused_imports,
    reason = "Retained decoders remain regression-tested for a planned \
              adapter."
)]
mod localization;
/// Stragglers.
mod stragglers;
pub(in crate::adapters::driven::local) mod units;

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
//   - Geometry domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Geometry domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Geometry domain module.

pub mod bounds;
pub mod payload;
pub mod polygon;

// Explicit facade names keep downstream imports unambiguous across domain
// modules.
#[expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these explicit facade names preserve stable domain \
              imports."
)]
pub use bounds::GeometryBounds;
pub use payload::Geometry;
pub use polygon::Polygon;

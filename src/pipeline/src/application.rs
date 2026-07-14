// File:
//   - application.rs
// Path:
//   - src/pipeline/src/application.rs
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
//   - Pipeline extraction, normalization, planning, and summary use cases.
// - Must-Not:
//   - Parse process arguments or select concrete providers.
// - Allows:
//   - Coordinate domain values, ports, and ordered pipeline phases.
// - Split-When:
//   - Split when one application family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same pipeline use cases.
// - Summary:
//   - Pipeline application facade.
// - Description:
//   - Exposes phase workflows and process-neutral output orchestration.
// - Usage:
//   - Called by driving adapters and public library callers.
// - Defaults:
//   - Concrete adapter selection remains outside this layer.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Application use cases for the ordered extraction and planning pipeline.
//!
//! Process composition remains in adapters while phase behavior lives here.
mod execute;
mod output_summary;

pub use execute::PipelineService;
pub use output_summary::{STANDARD_OUTPUT_DIRECTORIES, SummarizeOutput};

// File:
//   - ports.rs
// Path:
//   - src/rtf/src/ports.rs
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
//   - RTF conversion outbound port declarations.
// - Must-Not:
//   - Implement storage behavior or command-line policy.
// - Allows:
//   - Traits and DTOs isolating use cases from external mechanisms.
// - Split-When:
//   - Split when one port family becomes an independent context.
// - Merge-When:
//   - Another facade owns the same port declarations.
// - Summary:
//   - Hexagonal ports for RTF conversion.
// - Description:
//   - Exposes replaceable source and Markdown publication boundaries.
// - Usage:
//   - Implemented by driven adapters and supplied by driving adapters.
// - Defaults:
//   - Ports infer no paths.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Hexagonal ports for RTF conversion workflows.
//!
//! Application code depends on source and sink contracts rather than concrete
//! filesystem operations.
mod markdown_sink;
mod rtf_source;

pub use markdown_sink::MarkdownSink;
pub use rtf_source::{RtfSnapshot, RtfSource};

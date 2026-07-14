// File:
//   - driven.rs
// Path:
//   - src/rtf/src/adapters/driven.rs
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
//   - Replaceable outbound adapters for RTF source and Markdown ports.
// - Must-Not:
//   - Parse CLI requests or alter conversion semantics.
// - Allows:
//   - Filesystem source loading and explicit document publication.
// - Split-When:
//   - Split when another storage provider gains an independent adapter family.
// - Merge-When:
//   - Another facade owns the same outbound implementations.
// - Summary:
//   - Driven adapter facade for RTF conversion.
// - Description:
//   - Exposes concrete source and sink implementations.
// - Usage:
//   - Constructed by driving adapters and integration tests.
// - Defaults:
//   - No input or output path is selected implicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Driven adapters implementing RTF outbound ports.
//!
//! Filesystem mechanisms remain outside the domain and application layers.
mod file_markdown_sink;
mod file_rtf_source;

pub use file_markdown_sink::FileMarkdownSink;
pub use file_rtf_source::FileRtfSource;

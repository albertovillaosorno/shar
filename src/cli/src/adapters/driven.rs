// File:
//   - driven.rs
// Path:
//   - src/cli/src/adapters/driven.rs
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
//   - Concrete process argument and output adapter implementations.
// - Must-Not:
//   - Own caller command policy or process orchestration.
// - Allows:
//   - Expose environment arguments and standard streams.
// - Split-When:
//   - Split when another process provider gains a distinct adapter family.
// - Merge-When:
//   - Another facade owns the same outbound process implementations.
// - Summary:
//   - Shared CLI driven-adapter facade.
// - Description:
//   - Keeps operating-system process mechanisms outside core layers.
// - Usage:
//   - Imported by process driving composition and tests.
// - Defaults:
//   - Both adapters are stateless.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driven adapters implementing shared CLI ports.
//!
//! Process mechanisms remain replaceable behind narrow contracts.
mod environment_arguments;
mod standard_streams;

pub use environment_arguments::EnvironmentArguments;
pub use standard_streams::StandardStreams;

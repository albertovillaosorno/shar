// File:
//   - output_error_cyclic_source.rs
// Path:
//   - src/cli/tests/output_error_cyclic_source.rs
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
//   - Regression coverage for cyclic provider source traversal.
// - Must-Not:
//   - Access operating-system arguments or streams.
// - Allows:
//   - Use one deterministic self-referential provider error.
// - Split-When:
//   - Another source-chain topology needs independent coverage.
// - Merge-When:
//   - Output diagnostics no longer inspect provider source chains.
// - Summary:
//   - Cyclic provider source regression.
// - Description:
//   - Proves output rendering stops after trait-object canonicalization.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The provider error source points directly back to itself.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for cyclic provider source traversal.
//!
//! Custom error sources may form cycles and must not be revisited repeatedly.

use std::error::Error;
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, OutputSink,
    OutputStream, RunInvocation,
};

struct EmptyArguments;

impl ArgumentSource for EmptyArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Ok(Vec::new())
    }
}

struct DiagnosticProgram;

impl CliProgram for DiagnosticProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure().stderr("diagnostic")
    }
}

#[derive(Debug)]
struct CyclicError {
    /// Number of self-source requests observed by the fixture.
    source_calls: Arc<AtomicUsize>,
}

impl core::fmt::Display for CyclicError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        formatter.write_str("cyclic provider error")
    }
}

impl Error for CyclicError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let _previous_call_count = self
            .source_calls
            .fetch_add(
                1,
                Ordering::SeqCst,
            );
        Some(self)
    }
}

struct CyclicErrorSink {
    /// Shared source-call counter retained by the test.
    source_calls: Arc<AtomicUsize>,
}

impl OutputSink for CyclicErrorSink {
    fn write(
        &mut self,
        _stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        Err(
            io::Error::other(
                CyclicError {
                    source_calls: Arc::clone(&self.source_calls),
                },
            ),
        )
    }
}

#[test]
fn cyclic_provider_source_stops_after_canonicalization() {
    let source_calls = Arc::new(AtomicUsize::new(0));
    let mut arguments = EmptyArguments;
    let mut output = CyclicErrorSink {
        source_calls: Arc::clone(&source_calls),
    };

    let result = RunInvocation::execute(
        &DiagnosticProgram,
        &mut arguments,
        &mut output,
    );

    assert!(result.is_err());
    let Some(error) = result.err() else {
        return;
    };
    let _diagnostic = error.to_string();
    assert_eq!(
        source_calls.load(Ordering::SeqCst),
        2
    );
}

static FIRST_SOURCE_CALLS: AtomicUsize = AtomicUsize::new(0);
static SECOND_SOURCE_CALLS: AtomicUsize = AtomicUsize::new(0);
static FIRST_ERROR: FirstCycleError = FirstCycleError;
static SECOND_ERROR: SecondCycleError = SecondCycleError;

#[derive(Debug)]
struct MultiCycleRoot;

impl core::fmt::Display for MultiCycleRoot {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        formatter.write_str("multi-node cycle root")
    }
}

impl Error for MultiCycleRoot {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&FIRST_ERROR)
    }
}

#[derive(Debug)]
struct FirstCycleError;

impl core::fmt::Display for FirstCycleError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        formatter.write_str("first cycle node")
    }
}

impl Error for FirstCycleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let _previous_call_count = FIRST_SOURCE_CALLS.fetch_add(
            1,
            Ordering::SeqCst,
        );
        Some(&SECOND_ERROR)
    }
}

#[derive(Debug)]
struct SecondCycleError;

impl core::fmt::Display for SecondCycleError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        formatter.write_str("second cycle node")
    }
}

impl Error for SecondCycleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        let _previous_call_count = SECOND_SOURCE_CALLS.fetch_add(
            1,
            Ordering::SeqCst,
        );
        Some(&FIRST_ERROR)
    }
}

struct MultiCycleSink;

impl OutputSink for MultiCycleSink {
    fn write(
        &mut self,
        _stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        Err(io::Error::other(MultiCycleRoot))
    }
}

#[test]
fn multi_node_provider_cycle_visits_each_node_once() {
    FIRST_SOURCE_CALLS.store(
        0,
        Ordering::SeqCst,
    );
    SECOND_SOURCE_CALLS.store(
        0,
        Ordering::SeqCst,
    );
    let mut arguments = EmptyArguments;
    let mut output = MultiCycleSink;

    let result = RunInvocation::execute(
        &DiagnosticProgram,
        &mut arguments,
        &mut output,
    );

    assert!(result.is_err());
    let Some(error) = result.err() else {
        return;
    };
    let _diagnostic = error.to_string();
    assert_eq!(
        FIRST_SOURCE_CALLS.load(Ordering::SeqCst),
        1
    );
    assert_eq!(
        SECOND_SOURCE_CALLS.load(Ordering::SeqCst),
        1
    );
}

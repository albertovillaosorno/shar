// File:
//   - tests.rs
// Path:
//   - src/cli/src/application/tests.rs
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
//   - Unit regressions for process-neutral invocation orchestration.
// - Must-Not:
//   - Access current-process arguments or operating-system streams.
// - Allows:
//   - Use deterministic in-memory command, argument, and output ports.
// - Split-When:
//   - One orchestration behavior needs an independent fixture family.
// - Merge-When:
//   - The application runner no longer owns invocation orchestration.
// - Summary:
//   - Shared CLI application tests.
// - Description:
//   - Verifies command execution, argument diagnostics, and output order.
// - Usage:
//   - Compiled with the schoenwald-cli unit test target.
// - Defaults:
//   - All ports remain process neutral and deterministic.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Unit regressions for shared CLI invocation orchestration.
//!
//! Test ports stay deterministic and avoid current-process mechanisms.

mod cases {
    use std::cell::RefCell;
    use std::io;

    use super::super::RunInvocation;
    use crate::domain::{
        ArgumentError, CommandOutcome, ExitStatus, OutputStream,
    };
    use crate::ports::{ArgumentSource, CliProgram, OutputSink};

    struct SuppliedArguments {
        /// Complete argument result supplied to the runner.
        values: Result<Vec<String>, ArgumentError>,
    }

    impl ArgumentSource for SuppliedArguments {
        fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
            self.values
                .clone()
        }
    }

    struct EchoProgram;

    impl CliProgram for EchoProgram {
        fn execute(
            &self,
            arguments: &[String],
        ) -> CommandOutcome {
            CommandOutcome::success()
                .stdout_line(arguments.join("|"))
                .stderr("diagnostic")
        }
    }

    #[derive(Default)]
    struct RecordingOutput {
        /// Exact chunks observed in presentation order.
        chunks: RefCell<
            Vec<(
                OutputStream,
                String,
            )>,
        >,
    }

    impl OutputSink for RecordingOutput {
        fn write(
            &mut self,
            stream: OutputStream,
            text: &str,
        ) -> io::Result<()> {
            self.chunks
                .borrow_mut()
                .push(
                    (
                        stream,
                        text.to_owned(),
                    ),
                );
            Ok(())
        }
    }

    #[test]
    fn command_receives_arguments_and_output_order_is_preserved() {
        let mut arguments = SuppliedArguments {
            values: Ok(
                vec![
                    "first".to_owned(),
                    "second".to_owned(),
                ],
            ),
        };
        let mut output = RecordingOutput::default();

        let result = RunInvocation::execute(
            &EchoProgram,
            &mut arguments,
            &mut output,
        );

        assert!(
            matches!(
                result,
                Ok(ExitStatus::Success)
            )
        );
        assert_eq!(
            output
                .chunks
                .borrow()
                .as_slice(),
            &[
                (
                    OutputStream::Stdout,
                    "first|second\n".to_owned()
                ),
                (
                    OutputStream::Stderr,
                    "diagnostic".to_owned()
                ),
            ]
        );
    }

    #[test]
    fn invalid_argument_is_presented_as_a_failed_diagnostic() {
        let mut arguments = SuppliedArguments {
            values: Err(ArgumentError::non_unicode(2)),
        };
        let mut output = RecordingOutput::default();

        let result = RunInvocation::execute(
            &EchoProgram,
            &mut arguments,
            &mut output,
        );

        assert!(
            matches!(
                result,
                Ok(ExitStatus::Failure)
            )
        );
        assert_eq!(
            output
                .chunks
                .borrow()
                .as_slice(),
            &[
                (
                    OutputStream::Stderr,
                    "command argument 3 is not valid Unicode\n".to_owned(),
                )
            ]
        );
    }
    struct EmptyOutputProgram;

    impl CliProgram for EmptyOutputProgram {
        fn execute(
            &self,
            _arguments: &[String],
        ) -> CommandOutcome {
            CommandOutcome::success().stdout("")
        }
    }

    #[derive(Default)]
    struct RejectingOutput {
        /// Number of sink calls attempted by the runner.
        calls: usize,
    }

    impl OutputSink for RejectingOutput {
        fn write(
            &mut self,
            _stream: OutputStream,
            _text: &str,
        ) -> io::Result<()> {
            self.calls = self
                .calls
                .saturating_add(1);
            Err(
                io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "sink rejected output",
                ),
            )
        }
    }

    #[test]
    fn empty_output_does_not_touch_the_sink() {
        let mut arguments = SuppliedArguments {
            values: Ok(Vec::new()),
        };
        let mut output = RejectingOutput::default();

        let result = RunInvocation::execute(
            &EmptyOutputProgram,
            &mut arguments,
            &mut output,
        );

        assert!(
            matches!(
                result,
                Ok(ExitStatus::Success)
            )
        );
        assert_eq!(
            output.calls,
            0
        );
    }
}

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
//   - Tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Tests test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Tests test module.

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
            self.values.clone()
        }
    }

    struct EchoProgram;

    impl CliProgram for EchoProgram {
        fn execute(&self, arguments: &[String]) -> CommandOutcome {
            CommandOutcome::success()
                .stdout_line(arguments.join("|"))
                .stderr("diagnostic")
        }
    }

    #[derive(Default)]
    struct RecordingOutput {
        /// Exact chunks observed in presentation order.
        chunks: RefCell<Vec<(OutputStream, String)>>,
    }

    impl OutputSink for RecordingOutput {
        fn write(
            &mut self,
            stream: OutputStream,
            text: &str,
        ) -> io::Result<()> {
            self.chunks.borrow_mut().push((stream, text.to_owned()));
            Ok(())
        }
    }

    #[test]
    fn command_receives_arguments_and_output_order_is_preserved() {
        let mut arguments = SuppliedArguments {
            values: Ok(vec!["first".to_owned(), "second".to_owned()]),
        };
        let mut output = RecordingOutput::default();

        let result =
            RunInvocation::execute(&EchoProgram, &mut arguments, &mut output);

        assert!(matches!(result, Ok(ExitStatus::Success)));
        assert_eq!(output.chunks.borrow().as_slice(), &[
            (OutputStream::Stdout, "first|second\n".to_owned()),
            (OutputStream::Stderr, "diagnostic".to_owned()),
        ]);
    }

    #[test]
    fn invalid_argument_is_presented_as_a_failed_diagnostic() {
        let mut arguments = SuppliedArguments {
            values: Err(ArgumentError::non_unicode(2)),
        };
        let mut output = RecordingOutput::default();

        let result =
            RunInvocation::execute(&EchoProgram, &mut arguments, &mut output);

        assert!(matches!(result, Ok(ExitStatus::Failure)));
        assert_eq!(output.chunks.borrow().as_slice(), &[(
            OutputStream::Stderr,
            "command argument 3 is not valid Unicode\n".to_owned(),
        )]);
    }
    struct EmptyOutputProgram;

    impl CliProgram for EmptyOutputProgram {
        fn execute(&self, _arguments: &[String]) -> CommandOutcome {
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
            self.calls = self.calls.saturating_add(1);
            Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "sink rejected output",
            ))
        }
    }

    #[test]
    fn empty_output_does_not_touch_the_sink() {
        let mut arguments = SuppliedArguments { values: Ok(Vec::new()) };
        let mut output = RejectingOutput::default();

        let result = RunInvocation::execute(
            &EmptyOutputProgram,
            &mut arguments,
            &mut output,
        );

        assert!(matches!(result, Ok(ExitStatus::Success)));
        assert_eq!(output.calls, 0);
    }
}

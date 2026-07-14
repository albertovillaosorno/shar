// File:
//   - options_tests.rs
// Path:
//   - src/pipeline/src/adapters/driving/cli/options_tests.rs
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
//   - Regression coverage for common pipeline CLI options.
// - Must-Not:
//   - Execute commands or infer machine-specific paths.
// - Allows:
//   - Construct explicit argument vectors and compare typed results.
// - Split-When:
//   - Split when another option family gains independent tests.
// - Merge-When:
//   - Tests no longer obscure the production parser.
// - Summary:
//   - Common pipeline option parser regressions.
// - Description:
//   - Verifies progress, logging, helper, and positional contracts.
// - Usage:
//   - Included by options.rs under cfg(test).
// - Defaults:
//   - Cases are deterministic and filesystem-free.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression tests for common pipeline command options.
//!
//! Each case protects one process-boundary parsing invariant.

mod cases {
    use std::path::PathBuf;

    use super::super::{
        DEFAULT_LOG_FILE, ParsedArguments, parse_common_arguments,
    };
    use crate::adapters::driven::ProgressVerbosity;

    #[test]
    fn defaults_to_detailed_progress_and_latest_run_log() {
        assert_eq!(
            parse_common_arguments(
                &[
                    String::from("game"),
                    String::from("extracted"),
                ],
            ),
            Ok(
                ParsedArguments {
                    positionals: vec![
                        String::from("game"),
                        String::from("extracted"),
                    ],
                    verbosity: ProgressVerbosity::Detailed,
                    log_file: Some(PathBuf::from(DEFAULT_LOG_FILE)),
                    blender_helper: false,
                    maya: false,
                },
            )
        );
    }

    #[test]
    fn minimal_progress_and_disabled_log_are_explicit() {
        assert_eq!(
            parse_common_arguments(
                &[
                    String::from("--minimal"),
                    String::from("--no-log"),
                    String::from("game"),
                ],
            ),
            Ok(
                ParsedArguments {
                    positionals: vec![String::from("game")],
                    verbosity: ProgressVerbosity::Minimal,
                    log_file: None,
                    blender_helper: false,
                    maya: false,
                },
            )
        );
    }

    #[test]
    fn explicit_verbosity_and_log_path_are_preserved() {
        assert_eq!(
            parse_common_arguments(
                &[
                    String::from("--verbosity=minimal"),
                    String::from("--log=logs/custom/run.jsonl"),
                ],
            ),
            Ok(
                ParsedArguments {
                    positionals: Vec::new(),
                    verbosity: ProgressVerbosity::Minimal,
                    log_file: Some(PathBuf::from("logs/custom/run.jsonl")),
                    blender_helper: false,
                    maya: false,
                },
            )
        );
    }

    #[test]
    fn option_separator_preserves_dash_prefixed_positionals() {
        assert_eq!(
            parse_common_arguments(
                &[
                    String::from("--"),
                    String::from("--minimal"),
                ],
            ),
            Ok(
                ParsedArguments {
                    positionals: vec![String::from("--minimal")],
                    verbosity: ProgressVerbosity::Detailed,
                    log_file: Some(PathBuf::from(DEFAULT_LOG_FILE)),
                    blender_helper: false,
                    maya: false,
                },
            )
        );
    }

    #[test]
    fn blender_helper_is_explicit_and_preserves_positionals() {
        assert_eq!(
            parse_common_arguments(
                &[
                    String::from("--blender-helper"),
                    String::from("index.jsonl"),
                ],
            ),
            Ok(
                ParsedArguments {
                    positionals: vec![String::from("index.jsonl")],
                    verbosity: ProgressVerbosity::Detailed,
                    log_file: Some(PathBuf::from(DEFAULT_LOG_FILE)),
                    blender_helper: true,
                    maya: false,
                },
            )
        );
    }

    #[test]
    fn maya_is_explicit_and_preserves_positionals() {
        assert_eq!(
            parse_common_arguments(
                &[
                    String::from("--maya"),
                    String::from("index.jsonl"),
                ],
            ),
            Ok(
                ParsedArguments {
                    positionals: vec![String::from("index.jsonl")],
                    verbosity: ProgressVerbosity::Detailed,
                    log_file: Some(PathBuf::from(DEFAULT_LOG_FILE)),
                    blender_helper: false,
                    maya: true,
                },
            )
        );
    }

    #[test]
    fn unknown_option_is_rejected() {
        assert_eq!(
            parse_common_arguments(&[String::from("--silent")],),
            Err(String::from("unknown option: --silent"))
        );
    }

    #[test]
    fn repeated_verbosity_selectors_are_rejected() {
        for arguments in [
            vec![
                String::from("--minimal"),
                String::from("--detailed"),
            ],
            vec![
                String::from("--verbosity=minimal"),
                String::from("--minimal"),
            ],
            vec![
                String::from("--verbosity"),
                String::from("minimal"),
                String::from("--detailed"),
            ],
        ] {
            assert!(
                parse_common_arguments(&arguments).is_err_and(
                    |error| error == "verbosity may be specified only once",
                )
            );
        }
    }

    #[test]
    fn repeated_log_selectors_are_rejected() {
        for arguments in [
            vec![
                String::from("--log=logs/first.jsonl"),
                String::from("--log=logs/second.jsonl"),
            ],
            vec![
                String::from("--log=logs/run.jsonl"),
                String::from("--no-log"),
            ],
            vec![
                String::from("--no-log"),
                String::from("--log"),
                String::from("logs/run.jsonl"),
            ],
        ] {
            assert!(
                parse_common_arguments(&arguments).is_err_and(
                    |error| error == "logging may be specified only once",
                )
            );
        }
    }

    #[test]
    fn separated_log_rejects_option_token_as_missing_path() {
        assert!(
            parse_common_arguments(
                &[
                    String::from("--log"),
                    String::from("--no-log"),
                ],
            )
            .is_err_and(
                |error| error == "--log requires a path before --no-log",
            )
        );
    }

    #[test]
    fn separated_verbosity_rejects_option_token_as_missing_value() {
        assert!(
            parse_common_arguments(
                &[
                    String::from("--verbosity"),
                    String::from("--minimal"),
                ],
            )
            .is_err_and(
                |error| {
                    error == "--verbosity requires a value before --minimal"
                },
            )
        );
    }

    #[test]
    fn log_paths_are_lexically_normalized() -> Result<(), String> {
        let parsed = parse_common_arguments(
            &[String::from("--log=logs/./pipeline//run.jsonl")],
        )?;
        let Some(actual) = parsed.log_file else {
            return Err(String::from("normalized log path is missing"));
        };
        let expected = PathBuf::from("logs")
            .join("pipeline")
            .join("run.jsonl");
        let actual_identity = actual.into_os_string();
        let expected_identity = expected.into_os_string();
        if actual_identity != expected_identity {
            return Err(String::from("normalized log path identity mismatch"));
        }
        Ok(())
    }

    #[test]
    fn parent_log_paths_are_rejected() {
        assert!(
            parse_common_arguments(&[String::from("--log=../escape.jsonl")],)
                .is_err()
        );
    }

    #[test]
    fn absolute_log_paths_are_rejected() {
        assert!(
            parse_common_arguments(&[String::from("--log=/escape.jsonl")],)
                .is_err()
        );
    }

    #[test]
    fn directory_shaped_log_paths_are_rejected() {
        for arguments in [
            vec![String::from("--log=logs/")],
            vec![String::from("--log=logs\\")],
        ] {
            assert!(
                parse_common_arguments(&arguments).is_err_and(
                    |error| error == "--log path must identify a file",
                )
            );
        }
    }

    #[test]
    fn whitespace_only_log_paths_are_rejected() {
        for arguments in [
            vec![String::from("--log=   ")],
            vec![
                String::from("--log"),
                String::from("   "),
            ],
        ] {
            assert!(
                parse_common_arguments(&arguments).is_err_and(
                    |error| error == "--log path must not be blank",
                )
            );
        }
    }

    #[test]
    fn empty_log_path_is_rejected() {
        assert_eq!(
            parse_common_arguments(&[String::from("--log=")],),
            Err(String::from("--log path must not be empty"))
        );
    }
}

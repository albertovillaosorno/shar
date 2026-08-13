//! Algorithm command composition root.

use std::process::ExitCode;

use chacha20poly1305 as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn main() -> ExitCode {
    shar_algorithm::adapters::driving::cli::run_env()
}

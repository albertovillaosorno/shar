//! Read-only deep source validation composition root.

use std::process::ExitCode;

use p3d as _;
use rcf as _;
use rmv as _;
use rsd as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

fn main() -> ExitCode {
    shar_source_audit::adapters::driving::cli::run_env()
}

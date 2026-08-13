//! Standalone LMLM compatibility process entry point.

fn main() -> std::process::ExitCode {
    schoenwald_cli::run_process(&shar_lmlm::cli::LmlmProgram)
}

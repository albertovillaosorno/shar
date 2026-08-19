// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT

//! Contract tests for read-only deep source validation.

use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use p3d as _;
use rcf as _;
use rmv as _;
use rsd as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use shar_source_audit as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn create(label: &str) -> io::Result<Self> {
        let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "shar-source-audit-{label}-{}-{sequence}",
            std::process::id()
        ));
        match fs::remove_dir_all(&root) {
            Ok(()) => {},
            Err(error) if error.kind() == ErrorKind::NotFound => {},
            Err(error) => return Err(error),
        }
        fs::create_dir_all(&root)?;
        Ok(Self { root })
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _cleanup_result = fs::remove_dir_all(&self.root);
    }
}

fn run_validator(root: &Path, extra: Option<&str>) -> io::Result<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_validate-source-deep"));
    let _source = command.arg(root);
    if let Some(value) = extra {
        let _extra = command.arg(value);
    }
    command.output()
}

#[test]
fn empty_source_tree_is_valid_and_reports_zero_counts() -> io::Result<()> {
    let fixture = Fixture::create("empty")?;
    let output = run_validator(&fixture.root, None)?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success(), "empty source tree should be valid");
    assert_eq!(stdout, "deep-source\tfiles=0\tp3d=0\trcf=0\trsd=0\trmv=0\n");
    Ok(())
}

#[test]
fn unsupported_files_are_ignored() -> io::Result<()> {
    let fixture = Fixture::create("unsupported")?;
    fs::write(
        fixture.root.join("note.txt"),
        b"not a structured source container",
    )?;
    let output = run_validator(&fixture.root, None)?;

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "deep-source\tfiles=0\tp3d=0\trcf=0\trsd=0\trmv=0\n"
    );
    Ok(())
}

#[test]
fn malformed_supported_formats_fail_without_private_paths() -> io::Result<()> {
    for extension in ["p3d", "rcf", "rsd", "rmv"] {
        let fixture = Fixture::create(extension)?;
        let file = fixture.root.join(format!("malformed.{extension}"));
        fs::write(&file, b"not a valid structured source container")?;
        let before = fs::read(&file)?;
        let output = run_validator(&fixture.root, None)?;
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(!output.status.success(), "malformed {extension} must fail");
        assert!(
            stderr.contains(&format!(
                "deep source validation failed for {extension} input"
            )),
            "failure should classify the malformed format: {stderr}"
        );
        assert!(
            !stderr.contains(&fixture.root.display().to_string()),
            "failure must not disclose the selected source path"
        );
        assert_eq!(
            fs::read(&file)?,
            before,
            "deep validation must not modify {extension} input"
        );
    }
    Ok(())
}

#[test]
fn extra_arguments_are_rejected_by_usage_contract() -> io::Result<()> {
    let fixture = Fixture::create("extra")?;
    let output = run_validator(&fixture.root, Some("unexpected"))?;

    assert!(!output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "usage: validate-source-deep [game-directory]\n"
    );
    Ok(())
}

#[cfg(unix)]
#[test]
fn redirected_source_entry_fails_closed_without_private_path(
) -> io::Result<()> {
    use std::os::unix::fs::symlink;

    let fixture = Fixture::create("redirect")?;
    let outside = Fixture::create("redirect-outside")?;
    let target = outside.root.join("outside.p3d");
    fs::write(&target, b"not inspected through redirect")?;
    symlink(&target, fixture.root.join("linked.p3d"))?;

    let output = run_validator(&fixture.root, None)?;
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success(), "redirected source entry must fail");
    assert_eq!(
        stderr,
        "deep source validation could not scan source directory\n"
    );
    assert!(!stderr.contains(&fixture.root.display().to_string()));
    assert!(!stderr.contains(&outside.root.display().to_string()));
    Ok(())
}

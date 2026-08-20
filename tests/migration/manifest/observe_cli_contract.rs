// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Observed manifest CLI contract integration tests.
// - Must-Not:
//   - Own production behavior or publish caller source paths.
// - Allows:
//   - Temporary lawful-source fixtures and process assertions.
// - Split-When:
//   - Observation gains independently versioned integration surfaces.
// - Merge-When:
//   - Another manifest CLI test owns the same read-only contract.
// - Summary:
//   - Proves observed count output is public-safe and source read-only.
// - Description:
//   - Exercises the observed manifest CLI through its external process
//     boundary.
// - Usage:
//   - Run through the game_manifest package integration suite.
// - Defaults:
//   - Fixtures are removed after each process assertion.
//

//! Observed manifest CLI contract integration tests.

use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn create() -> io::Result<Self> {
        let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "game-manifest-observe-{}-{sequence}",
            std::process::id()
        ));
        match fs::remove_dir_all(&root) {
            Ok(()) => {},
            Err(error) if error.kind() == ErrorKind::NotFound => {},
            Err(error) => return Err(error),
        }
        fs::create_dir_all(&root)?;
        write_required_fixture(&root)?;
        let assets = root.join("art/cars");
        fs::create_dir_all(&assets)?;
        fs::write(assets.join("first.p3d"), b"first-observed-payload")?;
        fs::write(assets.join("second.p3d"), b"second-observed-payload")?;
        Ok(Self { root })
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.root));
    }
}

fn write_required_fixture(root: &Path) -> io::Result<()> {
    for path in ["README.rtf", "Simpsons.exe", "Simpsons.ico", "dialog.rcf"] {
        fs::write(root.join(path), b"required")?;
    }
    let textbible = root.join("art/frontend/scrooby2/resource/txtbible");
    fs::create_dir_all(&textbible)?;
    fs::write(textbible.join("srr2.E"), b"required")?;
    fs::write(textbible.join("srr2.txt"), b"required")?;
    Ok(())
}

fn run_observer(root: &Path, extra: Option<&str>) -> io::Result<Output> {
    let executable = env!("CARGO_BIN_EXE_observe-manifest-counts");
    let mut command = Command::new(executable);
    let _root = command.arg(root);
    if let Some(argument) = extra {
        let _extra = command.arg(argument);
    }
    command.output()
}

#[test]
fn observation_emits_counts_without_writing_source() -> io::Result<()> {
    let fixture = Fixture::create()?;
    let first = fixture.root.join("art/cars/first.p3d");
    let before = fs::read(&first)?;

    let output = run_observer(&fixture.root, None)?;

    assert!(output.status.success());
    assert_eq!(fs::read(&first)?, before);
    assert!(!fixture.root.join("manifest").exists());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout,
        concat!(
            "{\"dir\":\"at/cs\",\"ext\":\"p3d\",",
            "\"count\":2,\"kind\":\"p3d_container\"}\n"
        )
    );
    assert_eq!(output.stderr, b"");
    Ok(())
}

#[test]
fn observation_failure_does_not_disclose_source_path() -> io::Result<()> {
    let fixture = Fixture::create()?;
    fs::remove_file(fixture.root.join("Simpsons.exe"))?;

    let output = run_observer(&fixture.root, None)?;

    assert!(!output.status.success());
    assert_eq!(output.stdout, b"");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(stderr, "source count observation failed\n");
    assert!(!stderr.contains(&fixture.root.display().to_string()));
    assert!(!fixture.root.join("manifest").exists());
    Ok(())
}

#[test]
fn observation_rejects_extra_arguments() -> io::Result<()> {
    let fixture = Fixture::create()?;

    let output = run_observer(&fixture.root, Some("unexpected"))?;

    assert!(!output.status.success());
    assert_eq!(output.stdout, b"");
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "usage: observe-manifest-counts [game-directory]\n"
    );
    assert!(!fixture.root.join("manifest").exists());
    Ok(())
}

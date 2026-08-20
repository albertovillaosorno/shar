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
//   - Expanded cli contract test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Expanded cli contract test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Expanded cli contract test module.

use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct ExpandedFixture {
    root: PathBuf,
    game: PathBuf,
    extracted: PathBuf,
    output: PathBuf,
}

impl ExpandedFixture {
    fn new(label: &str) -> io::Result<Self> {
        let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "game-manifest-expanded-{label}-{}-{sequence}",
            std::process::id()
        ));
        match fs::remove_dir_all(&root) {
            Ok(()) => {},
            Err(error) if error.kind() == ErrorKind::NotFound => {},
            Err(error) => return Err(error),
        }
        let game = root.join("input");
        let extracted = root.join("decoded");
        let output = root.join("result.jsonl");
        fs::create_dir_all(&game)?;
        fs::create_dir_all(&extracted)?;
        Ok(Self {
            root,
            game,
            extracted,
            output,
        })
    }

    fn run(&self, extra_argument: Option<&str>) -> io::Result<Output> {
        let mut command =
            Command::new(env!("CARGO_BIN_EXE_generate-expanded-manifest"));
        let _game = command.arg(&self.game);
        let _extracted = command.arg(&self.extracted);
        let _output = command.arg(&self.output);
        if let Some(extra) = extra_argument {
            let _extra = command.arg(extra);
        }
        command.output()
    }

    fn game(&self) -> &Path {
        &self.game
    }
}

impl Drop for ExpandedFixture {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.root));
    }
}

#[test]
fn expanded_generator_rejects_extra_arguments() {
    let result = (|| {
        let fixture = ExpandedFixture::new("extra-args")?;
        fs::write(fixture.game().join("asset.p3d"), b"fixture")?;
        fixture.run(Some("unexpected"))
    })();
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn expanded_generator_rejects_identical_input_roots() {
    let result = (|| {
        let fixture = ExpandedFixture::new("same-roots")?;
        fs::write(fixture.game.join("asset.p3d"), b"fixture")?;
        let output =
            Command::new(env!("CARGO_BIN_EXE_generate-expanded-manifest"))
                .arg(&fixture.game)
                .arg(&fixture.game)
                .arg(&fixture.output)
                .output()?;
        Ok::<_, io::Error>(output)
    })();
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn expanded_generator_rejects_nested_input_roots() {
    let result = (|| {
        let fixture = ExpandedFixture::new("nested-roots")?;
        let nested = fixture.game.join("decoded");
        fs::create_dir_all(&nested)?;
        fs::write(fixture.game.join("asset.p3d"), b"fixture")?;
        fs::write(nested.join("decoded.p3d"), b"fixture")?;
        let output =
            Command::new(env!("CARGO_BIN_EXE_generate-expanded-manifest"))
                .arg(&fixture.game)
                .arg(&nested)
                .arg(&fixture.output)
                .output()?;
        Ok::<_, io::Error>(output)
    })();
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn expanded_generator_requires_rcf_extraction_root() {
    let result = (|| {
        let fixture = ExpandedFixture::new("missing-rcf-root")?;
        fs::write(fixture.game.join("archive.rcf"), b"fixture")?;
        fs::remove_dir_all(&fixture.extracted)?;
        fixture.run(None)
    })();
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn expanded_generator_rejects_orphan_rcf_extraction() {
    let result = (|| {
        let fixture = ExpandedFixture::new("orphan-rcf")?;
        fs::write(fixture.game.join("asset.p3d"), b"fixture")?;
        fs::write(fixture.extracted.join("orphan.p3d"), b"fixture")?;
        fixture.run(None)
    })();
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn expanded_generator_rejects_empty_rcf_extraction() {
    let result = (|| {
        let fixture = ExpandedFixture::new("empty-rcf-root")?;
        fs::write(fixture.game.join("archive.rcf"), b"fixture")?;
        fixture.run(None)
    })();
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn expanded_generator_rejects_file_as_optional_rcf_root() {
    let result = (|| {
        let fixture = ExpandedFixture::new("file-rcf-root")?;
        fs::write(fixture.game.join("asset.p3d"), b"fixture")?;
        fs::remove_dir_all(&fixture.extracted)?;
        fs::write(&fixture.extracted, b"not-a-directory")?;
        fixture.run(None)
    })();
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn expanded_generator_rejects_empty_game_directory() {
    let result = (|| {
        let fixture = ExpandedFixture::new("empty-game")?;
        fixture.run(None)
    })();
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

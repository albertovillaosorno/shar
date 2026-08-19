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
//   - Redirect regression evidence for the manifest filesystem adapter.
// - Must-Not:
//   - Depend on proprietary source data or follow redirects during scanning.
// - Allows:
//   - Build disposable synthetic filesystem trees for adapter validation.
// - Split-When:
//   - Filesystem tree entry policy gains an independent test lifecycle.
// - Merge-When:
//   - Another integration test owns the same manifest redirect contract.
// - Summary:
//   - Manifest filesystem redirect regression tests.
// - Description:
//   - Proves source-manifest traversal rejects redirected tree entries.
// - Usage:
//   - Run through the game_manifest integration test target.
// - Defaults:
//   - Fixtures contain only synthetic local files.
//

//! Redirect regression tests for the manifest filesystem tree adapter.

#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::io;
#[cfg(unix)]
use std::os::unix::fs::symlink;
#[cfg(unix)]
use std::path::PathBuf;
#[cfg(unix)]
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(unix)]
use game_manifest::adapters::FilesystemGameTree;
#[cfg(unix)]
use game_manifest::GameTree as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[cfg(unix)]
static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

#[cfg(unix)]
fn fixture_root() -> PathBuf {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "shar-manifest-redirect-{}-{sequence}",
        std::process::id()
    ))
}

#[cfg(unix)]
#[test]
fn filesystem_tree_rejects_redirected_entry() -> io::Result<()> {
    let root = fixture_root();
    let source = root.join("source");
    let outside = root.join("outside.bin");
    fs::create_dir_all(&source)?;
    fs::write(&outside, b"outside")?;
    symlink(&outside, source.join("linked.bin"))?;

    let result = FilesystemGameTree.files(&source);
    fs::remove_dir_all(&root)?;

    let Err(error) = result else {
        return Err(io::Error::other(
            "manifest traversal accepted a redirect",
        ));
    };
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    Ok(())
}

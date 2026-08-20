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
//   - Audit cli contract test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Audit cli contract test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Audit cli contract test module.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn run_audit(
    extension: &str,
    extra_argument: Option<&str>,
) -> io::Result<Output> {
    let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "game-manifest-audit-{}-{sequence}",
        std::process::id()
    ));
    match fs::remove_dir_all(&root) {
        Ok(()) => {},
        Err(error) if error.kind() == ErrorKind::NotFound => {},
        Err(error) => return Err(error),
    }
    fs::create_dir_all(&root)?;
    fs::write(root.join(format!("asset.{extension}")), b"fixture")?;
    let mut command =
        Command::new(env!("CARGO_BIN_EXE_ephemeral_structural_audit"));
    let _root = command.arg(&root);
    if let Some(extra) = extra_argument {
        let _extra = command.arg(extra);
    }
    let result = command.output();
    drop(fs::remove_dir_all(&root));
    result
}

#[test]
fn structural_audit_rejects_extra_arguments() {
    let result = run_audit("png", Some("unexpected"));
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(!output.status.success());
}

#[test]
fn structural_audit_ignores_local_backups() {
    let result = run_audit("schoenwald-original", None);
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "total_dirty_extensions\t0
"
    );
}

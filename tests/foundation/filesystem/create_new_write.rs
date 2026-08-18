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
//   - Regression evidence for create-new local filesystem writes.
// - Must-Not:
//   - Depend on repository content or replace an existing destination.
// - Allows:
//   - Use one disposable operating-system temporary fixture.
// - Split-When:
//   - Additional exclusive-write policies gain independent contracts.
// - Merge-When:
//   - Another test module owns the identical create-new write evidence.
// - Summary:
//   - Create-new filesystem write regression tests.
// - Description:
//   - Proves exclusive writes preserve an existing destination atomically.
// - Usage:
//   - Run through the schoenwald_filesystem integration test target.
// - Defaults:
//   - Fixtures contain only synthetic text.
//

//! Create-new filesystem write regression tests.

use std::fs;
use std::io;
use std::sync::atomic::{AtomicU64, Ordering};

use schoenwald_filesystem::adapters::driving::local;

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn create_new_text_preserves_existing_destination() -> Result<(), String> {
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "schoenwald-create-new-{}-{sequence}",
        std::process::id()
    ));
    let path = root.join("nested").join("report.txt");
    let cleanup = || fs::remove_dir_all(&root);
    drop(cleanup());

    local::write_new_text(&path, "first\n", true)
        .map_err(|error| error.to_string())?;
    let second = local::write_new_text(&path, "second\n", true);
    let preserved = fs::read_to_string(&path)
        .map_err(|error| error.to_string())?;
    drop(cleanup());

    let Err(error) = second else {
        let message = "create-new write replaced an existing destination";
        return Err(message.to_owned());
    };
    if error.kind() != io::ErrorKind::AlreadyExists {
        return Err(format!(
            "unexpected create-new error kind: {:?}",
            error.kind()
        ));
    }
    if preserved != "first\n" {
        return Err("create-new failure changed existing bytes".to_owned());
    }
    Ok(())
}

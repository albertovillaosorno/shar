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
//   - Unit evidence for captured deep-audit RCF bytes.
// - Must-Not:
//   - Read proprietary source data or own production parsing policy.
// - Allows:
//   - Exercise the source-audit snapshot reader through the RCF byte port.
// - Split-When:
//   - Snapshot parsing gains an independent test lifecycle.
// - Merge-When:
//   - Another test module owns identical source-audit byte evidence.
// - Summary:
//   - RCF snapshot reader unit tests.
// - Description:
//   - Proves deep RCF validation consumes one immutable captured byte image.
// - Usage:
//   - Included only by the owning audit application module under cfg(test).
// - Defaults:
//   - Synthetic bytes only.
//

//! Unit tests for source-audit RCF byte snapshots.

use rcf::ports::ArchiveByteReader;

use super::SnapshotReader;

#[test]
fn snapshot_reader_returns_exact_captured_ranges() -> Result<(), String> {
    let bytes = b"captured-source";
    let mut reader = SnapshotReader { bytes };

    let length = reader.len().map_err(|error| error.to_string())?;
    let range = reader
        .read_exact_range(3, 5)
        .map_err(|error| error.to_string())?;

    assert_eq!(length, 15);
    assert_eq!(range, b"tured");
    Ok(())
}

#[test]
fn snapshot_reader_rejects_ranges_outside_captured_bytes() {
    let mut reader = SnapshotReader { bytes: b"short" };

    let result = reader.read_range(4, 2);

    assert!(result.is_err());
}

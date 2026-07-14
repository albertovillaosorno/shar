// File:
//   - hard_link_identity.rs
// Path:
//   - src/rsd/tests/hard_link_identity.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Public regression coverage for physical RSD source identity.
// - Must-Not:
//   - Exercise destination replacement or audio codec edge cases.
// - Allows:
//   - Synthetic hard links and deterministic temporary source trees.
// - Split-When:
//   - Split when another physical identity family needs independent fixtures.
// - Merge-When:
//   - Another RSD test module owns hard-linked source deduplication.
// - Summary:
//   - Verifies one physical RSD source is exported once.
// - Description:
//   - Exercises hard-link identity independently from path aliases.
// - Usage:
//   - Executed through cargo test for the rsd crate.
// - Defaults:
//   - Fixtures remain synthetic, deterministic, and repository-local.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Public regression coverage for hard-linked RSD source identity.

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use rsd::Exporter;
use rsd::adapters::FilesystemExporter;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct TempTree {
    path: PathBuf,
}

impl TempTree {
    fn create() -> Result<Self, std::io::Error> {
        let sequence = TEMP_SEQUENCE.fetch_add(
            1_u64,
            Ordering::Relaxed,
        );
        let path = std::env::temp_dir().join(
            format!(
                "schoenwald-rsd-hard-link-{}-{sequence}",
                std::process::id()
            ),
        );
        if path.exists() {
            fs::remove_dir_all(&path)?;
        }
        fs::create_dir_all(&path)?;
        Ok(
            Self {
                path,
            },
        )
    }
}

impl Drop for TempTree {
    fn drop(&mut self) {
        let _cleanup_result = fs::remove_dir_all(&self.path);
    }
}

fn copy_fixture_bytes(
    data: &mut [u8],
    start: usize,
    bytes: &[u8],
) -> bool {
    let Some(end) = start.checked_add(bytes.len()) else {
        return false;
    };
    let Some(target) = data.get_mut(start..end) else {
        return false;
    };
    target.copy_from_slice(bytes);
    true
}

fn compact_pcm() -> Vec<u8> {
    let mut data = vec![0_u8; 0x80];
    assert!(
        copy_fixture_bytes(
            &mut data, 0, b"RSD4"
        ),
        "fixture magic should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data, 4, b"PCM "
        ),
        "fixture encoding should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data,
            8,
            &1_u32.to_le_bytes(),
        ),
        "fixture channel count should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data,
            12,
            &16_u32.to_le_bytes(),
        ),
        "fixture bit depth should fit"
    );
    assert!(
        copy_fixture_bytes(
            &mut data,
            16,
            &24_000_u32.to_le_bytes(),
        ),
        "fixture sample rate should fit"
    );
    data.extend_from_slice(
        &[
            1_u8, 0_u8,
        ],
    );
    data
}

#[test]
fn hard_link_sources_export_once() -> Result<(), Box<dyn Error>> {
    let temp = TempTree::create()?;
    let first = temp
        .path
        .join("source-a");
    let second = temp
        .path
        .join("source-b");
    let output = temp
        .path
        .join("output");
    fs::create_dir_all(&first)?;
    fs::create_dir_all(&second)?;
    let original = first.join("tone.rsd");
    let alias = second.join("alias.rsd");
    fs::write(
        &original,
        compact_pcm(),
    )?;
    fs::hard_link(
        &original, &alias,
    )?;

    let report = FilesystemExporter.export_roots(
        &[
            first, second,
        ],
        &output,
    )?;

    if report.total_files != 1_usize {
        return Err("hard-linked source exported twice".into());
    }
    Ok(())
}

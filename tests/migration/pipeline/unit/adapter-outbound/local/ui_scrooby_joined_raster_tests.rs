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
//   - Joined Scrooby raster publisher unit regressions.
// - Must-Not:
//   - Depend on proprietary source fixtures or external cache state.
// - Allows:
//   - Synthetic generated PNGs and temporary publication roots.
// - Split-When:
//   - Compiler fixtures gain an independent lifecycle.
// - Merge-When:
//   - Another test module owns the identical publisher boundary.
// - Summary:
//   - Pins deterministic joined-raster publication and repair.
// - Description:
//   - Exercises exact inventory, reuse, replacement, and debris rejection.
// - Usage:
//   - Included by the joined Scrooby raster publisher under cfg(test).
// - Defaults:
//   - Extra, changed, or interrupted generated output fails closed.
//

//! Joined Scrooby raster publisher tests.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use fbx::adapters::driven::semantic_texture_png::encode_png_bytes;
use fbx::domain::texture::semantic::{Rgba8, RgbaImage};
use shar_sha256::digest_hex;

use super::{publish_catalog, render_catalog, summarize, transaction_paths};
use crate::adapters::driven::local::ui_sprite_raster::
    CompiledScroobyJoinedRaster;

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

type TestResult = Result<(), String>;

fn case_dir(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "shar-scrooby-joined-raster-{label}-{}-{}",
        std::process::id(),
        CASE_ID.fetch_add(1, Ordering::Relaxed),
    ))
}

fn artifact(
    package_id: &str,
    sprite_ordinal: usize,
    red: u8,
) -> Result<CompiledScroobyJoinedRaster, String> {
    let image = RgbaImage::new(1, 1, vec![Rgba8::new(red, 0, 0, 255)])
        .map_err(|error| format!("{error:?}"))?;
    let png_bytes = encode_png_bytes(&image)
        .map_err(|error| format!("{error:?}"))?;
    Ok(CompiledScroobyJoinedRaster {
        package_id: package_id.to_owned(),
        sprite_ordinal,
        filename: format!("{package_id}--sprite-{sprite_ordinal}.png"),
        png_sha256: digest_hex(&png_bytes),
        source_revision: digest_hex(
            format!("{package_id}:{sprite_ordinal}").as_bytes(),
        ),
        png_bytes,
        width: 1,
        height: 1,
        tile_count: 1,
    })
}

#[test]
fn publishes_reuses_and_repairs_exact_joined_raster_catalog() -> TestResult {
    let output = case_dir("publish");
    if output.exists() {
        fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    }
    let compiled = vec![
        artifact("package-a", 4, 10)?,
        artifact("package-a", 9, 20)?,
    ];
    let summary = summarize(&compiled).map_err(|error| error.to_string())?;
    if summary.package_count != 1
        || summary.raster_count != 2
        || summary.tile_count != 2
        || summary.total_bytes == 0
    {
        return Err(format!("unexpected joined raster summary: {summary:?}"));
    }
    let rendered = render_catalog(&compiled, summary)
        .map_err(|error| error.to_string())?;
    publish_catalog(&output, &compiled, &rendered)
        .map_err(|error| error.to_string())?;
    publish_catalog(&output, &compiled, &rendered)
        .map_err(|error| error.to_string())?;
    fs::write(output.join("rasters/unclaimed.png"), b"extra")
        .map_err(|error| error.to_string())?;
    fs::write(output.join("unclaimed.txt"), b"extra")
        .map_err(|error| error.to_string())?;
    publish_catalog(&output, &compiled, &rendered)
        .map_err(|error| error.to_string())?;
    if output.join("rasters/unclaimed.png").exists()
        || output.join("unclaimed.txt").exists()
    {
        return Err("joined raster repair retained extra inventory".to_owned());
    }
    let (staging, _backup) = transaction_paths(&output)
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&staging).map_err(|error| error.to_string())?;
    let blocked = publish_catalog(&output, &compiled, &rendered);
    fs::remove_dir_all(&staging).map_err(|error| error.to_string())?;
    fs::remove_dir_all(&output).map_err(|error| error.to_string())?;
    if blocked.is_ok() {
        return Err("joined raster transaction debris was accepted".to_owned());
    }
    Ok(())
}

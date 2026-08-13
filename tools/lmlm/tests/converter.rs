use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use schoenwald_cli::CliProgram;
use shar_lmlm::batch::convert_folders;
use shar_lmlm::cli::LmlmProgram;
use shar_lmlm::convert::{convert, inspect};

static NEXT_ROOT: AtomicU64 = AtomicU64::new(0);

fn temp_root(label: &str) -> PathBuf {
    let sequence = NEXT_ROOT.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "shar-lmlm-{label}-{}-{sequence}",
        std::process::id()
    ))
}

fn remove_if_present(path: &Path) -> Result<(), String> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

fn fixture(payload: &[u8], name: &str) -> Vec<u8> {
    const BLOCK: usize = 0x200;
    const ROOT_BLOCK: usize = 0x400;
    const FIRST_ENTRY: usize = 0x600;
    const PAYLOAD_OFFSET: usize = 0x3000;
    let mut archive = vec![0_u8; 0x4000];
    archive[0..4].copy_from_slice(b"LSPA");
    archive[4..8].copy_from_slice(&5_u32.to_le_bytes());
    archive[0x0c..0x10].copy_from_slice(&0x0200_0000_u32.to_le_bytes());
    archive[ROOT_BLOCK + 2..ROOT_BLOCK + 4].copy_from_slice(&1_u16.to_le_bytes());
    archive[FIRST_ENTRY..FIRST_ENTRY + 2].copy_from_slice(&2_u16.to_le_bytes());
    let mut encoded = Vec::new();
    for unit in name.encode_utf16() {
        encoded.extend_from_slice(&unit.to_le_bytes());
    }
    encoded.extend_from_slice(&0_u16.to_le_bytes());
    archive[FIRST_ENTRY + 2..FIRST_ENTRY + 2 + encoded.len()].copy_from_slice(&encoded);
    let metadata = FIRST_ENTRY + BLOCK;
    archive[metadata + 0x0c..metadata + 0x14].copy_from_slice(
        &u64::try_from(payload.len())
            .unwrap_or(u64::MAX)
            .to_le_bytes(),
    );
    archive[metadata + 0x14..metadata + 0x1c]
        .copy_from_slice(&(PAYLOAD_OFFSET as u64).to_le_bytes());
    archive[PAYLOAD_OFFSET..PAYLOAD_OFFSET + payload.len()].copy_from_slice(payload);
    archive
}

#[test]
fn inspect_is_read_only_and_hashes_exact_payload() -> Result<(), String> {
    let root = temp_root("inspect");
    remove_if_present(&root)?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let input = root.join("fixture.lmlm");
    fs::write(&input, fixture(b"payload", "Meta.ini")).map_err(|error| error.to_string())?;
    let before = fs::read(&input).map_err(|error| error.to_string())?;

    let report = inspect(&input).map_err(|error| error.to_string())?;

    let after = fs::read(&input).map_err(|error| error.to_string())?;
    let result = if before == after
        && report.entries.len() == 1
        && report.entries[0].path == "Meta.ini"
        && report.entries[0].sha256 == shar_sha256::digest_hex(b"payload")
    {
        Ok(())
    } else {
        Err(format!("unexpected inspection report: {report:?}"))
    };
    remove_if_present(&root)?;
    result
}

#[test]
fn convert_publishes_content_and_refuses_overwrite() -> Result<(), String> {
    let root = temp_root("convert");
    remove_if_present(&root)?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let input = root.join("fixture.lmlm");
    let output = root.join("converted");
    fs::write(&input, fixture(b"payload", "Meta.ini")).map_err(|error| error.to_string())?;

    convert(&input, &output).map_err(|error| error.to_string())?;
    let payload = fs::read(output.join("content/Meta.ini")).map_err(|error| error.to_string())?;
    let report_exists = output.join("conversion-report.json").is_file();
    let second = convert(&input, &output);
    let result = if payload == b"payload" && report_exists && second.is_err() {
        Ok(())
    } else {
        Err("conversion publication contract failed".to_owned())
    };
    remove_if_present(&root)?;
    result
}

#[test]
fn p3d_entries_use_shared_parser_and_cli_usage_is_stable() -> Result<(), String> {
    let root = temp_root("p3d");
    remove_if_present(&root)?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let input = root.join("fixture.lmlm");
    fs::write(&input, fixture(b"not-p3d", "model.p3d")).map_err(|error| error.to_string())?;

    let report = inspect(&input).map_err(|error| error.to_string())?;
    let evidence = report.entries[0]
        .p3d
        .as_ref()
        .ok_or_else(|| "missing shared P3D evidence".to_owned())?;
    let usage = LmlmProgram.execute(&["invalid".to_owned()]);
    let result = if !evidence.valid
        && evidence.diagnostic.is_some()
        && usage.is_failure_with_stderr_line(
            "usage: shar-lmlm [batch] | inspect INPUT.lmlm | convert INPUT.lmlm OUTPUT_DIR",
        ) {
        Ok(())
    } else {
        Err(format!("unexpected P3D/CLI evidence: {report:?}"))
    };
    remove_if_present(&root)?;
    result
}

#[test]
fn batch_conversion_keeps_wip_and_export_separate() -> Result<(), String> {
    let root = temp_root("batch");
    remove_if_present(&root)?;
    let import = root.join("tools/lmlm/import");
    let export = root.join("tools/lmlm/export");
    let wip = root.join(".cache/lmlm/wip");
    fs::create_dir_all(&import).map_err(|error| error.to_string())?;
    fs::create_dir_all(&export).map_err(|error| error.to_string())?;
    let input = import.join("Example Legacy.lmlm");
    fs::write(&input, fixture(b"payload", "Meta.ini")).map_err(|error| error.to_string())?;

    let first =
        convert_folders(&root, &import, &export, &wip).map_err(|error| error.to_string())?;
    let second =
        convert_folders(&root, &import, &export, &wip).map_err(|error| error.to_string())?;

    let item = first
        .packages
        .first()
        .ok_or_else(|| "missing first batch package".to_owned())?;
    let second_item = second
        .packages
        .first()
        .ok_or_else(|| "missing second batch package".to_owned())?;
    let result = if first.packages.len() == 1
        && !item.wip_reused
        && !item.export_reused
        && second_item.wip_reused
        && second_item.export_reused
        && root.join(&item.wip).join("content/Meta.ini").is_file()
        && root.join(&item.export).join("content/Meta.ini").is_file()
        && input.is_file()
    {
        Ok(())
    } else {
        Err(format!("unexpected batch reports: {first:?} / {second:?}"))
    };
    remove_if_present(&root)?;
    result
}

//! Read-only inspection and contained conversion workspace publication.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;

use crate::archive::{FileEntry, LmlmError, entry_bytes, parse};
use crate::report::{ConversionReport, build_report};

/// Failure from one inspection or conversion request.
#[derive(Debug)]
pub enum ConvertError {
    /// Invalid caller input or output contract.
    Contract(String),
    /// Local filesystem operation failed.
    Io(io::Error),
    /// The LSPA package was rejected.
    Archive(LmlmError),
    /// JSON report serialization failed.
    Json(serde_json::Error),
    /// Shared Pure3D decompilation failed.
    P3d(p3d::P3dError),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contract(message) => formatter.write_str(message),
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Archive(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
            Self::P3d(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for ConvertError {}

impl From<io::Error> for ConvertError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<LmlmError> for ConvertError {
    fn from(error: LmlmError) -> Self {
        Self::Archive(error)
    }
}

impl From<serde_json::Error> for ConvertError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<p3d::P3dError> for ConvertError {
    fn from(error: p3d::P3dError) -> Self {
        Self::P3d(error)
    }
}

fn load(input: &Path) -> Result<(Vec<u8>, Vec<FileEntry>), ConvertError> {
    let is_lmlm = input
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("lmlm"));
    if !is_lmlm {
        return Err(ConvertError::Contract(
            "input must have the .lmlm extension".to_owned(),
        ));
    }
    if local::path_kind(input)? != PathKind::File {
        return Err(ConvertError::Contract(
            "input must be an existing regular file".to_owned(),
        ));
    }
    let data = local::read_bytes(input)?;
    let entries = parse(&data)?;
    Ok((data, entries))
}

/// Performs read-only validation and evidence generation.
pub fn inspect(input: &Path) -> Result<ConversionReport, ConvertError> {
    let (data, entries) = load(input)?;
    Ok(build_report(&data, &entries))
}

fn staging_path(output: &Path) -> Result<PathBuf, ConvertError> {
    let name = output
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            ConvertError::Contract("output must have a portable final component".to_owned())
        })?;
    Ok(output.with_file_name(format!(".{name}.lmlm-{}.tmp", std::process::id())))
}

fn is_p3d(path: &str) -> bool {
    path.rsplit('.')
        .next()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("p3d"))
}

fn decompile_p3d(root: &Path, entry: &FileEntry, input_path: &Path) -> Result<(), ConvertError> {
    if !is_p3d(&entry.path) {
        return Ok(());
    }
    let decompiled_root = root.join("decompiled").join("p3d");
    let output = schoenwald_filesystem::resolve_under(&decompiled_root, Path::new(&entry.path))
        .map_err(|error| ConvertError::Contract(error.to_string()))?;
    if let Some(parent) = output.parent().filter(|path| !path.as_os_str().is_empty()) {
        local::create_dir_all(parent)?;
    }
    let exporter = p3d::LosslessPackageExporter;
    p3d::ExportPackage::execute(&exporter, input_path, &output)?;
    Ok(())
}

fn publish_workspace(data: &[u8], entries: &[FileEntry], root: &Path) -> Result<(), ConvertError> {
    let content = root.join("content");
    local::create_dir_all(&content)?;
    for entry in entries {
        let payload = entry_bytes(data, entry).ok_or_else(|| {
            ConvertError::Contract(format!("invalid payload range for {}", entry.path))
        })?;
        let destination = schoenwald_filesystem::resolve_under(&content, Path::new(&entry.path))
            .map_err(|error| ConvertError::Contract(error.to_string()))?;
        local::write_bytes(&destination, payload, true)?;
        decompile_p3d(root, entry, &destination)?;
    }
    let report = build_report(data, entries);
    let mut json = serde_json::to_string_pretty(&report)?;
    json.push('\n');
    local::write_text(&root.join("conversion-report.json"), &json, false)?;
    Ok(())
}

/// Creates one atomic, inspectable conversion workspace without installing it.
pub fn convert(input: &Path, output: &Path) -> Result<ConversionReport, ConvertError> {
    if output.as_os_str().is_empty() {
        return Err(ConvertError::Contract(
            "output path must not be empty".to_owned(),
        ));
    }
    if local::path_kind(output)? != PathKind::Missing {
        return Err(ConvertError::Contract(
            "conversion output already exists".to_owned(),
        ));
    }
    let staging = staging_path(output)?;
    if local::path_kind(&staging)? != PathKind::Missing {
        return Err(ConvertError::Contract(
            "conversion staging path already exists".to_owned(),
        ));
    }
    let (data, entries) = load(input)?;
    if let Some(parent) = output.parent().filter(|path| !path.as_os_str().is_empty()) {
        local::create_dir_all(parent)?;
    }
    fs::create_dir(&staging)?;
    if let Err(error) = publish_workspace(&data, &entries, &staging) {
        let _cleanup_result = fs::remove_dir_all(&staging);
        return Err(error);
    }
    if let Err(error) = fs::rename(&staging, output) {
        let _cleanup_result = fs::remove_dir_all(&staging);
        return Err(ConvertError::Io(error));
    }
    Ok(build_report(&data, &entries))
}

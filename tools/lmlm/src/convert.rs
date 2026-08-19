//! Read-only inspection and contained conversion workspace publication.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;
use shar_mod_package::{
    CONTRACT_VERSION as MOD_CONTRACT_VERSION, Member, PackageKind,
    PackageManifest, Provenance, TrustLevel, content_revision, member_from_bytes,
};

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
    /// Shared SHAR package validation or serialization failed.
    Package(shar_mod_package::PackageError),
}

impl fmt::Display for ConvertError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contract(message) => formatter.write_str(message),
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Archive(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
            Self::Package(error) => write!(formatter, "{error}"),
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

impl From<shar_mod_package::PackageError> for ConvertError {
    fn from(error: shar_mod_package::PackageError) -> Self {
        Self::Package(error)
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
    let file_name = output
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| ConvertError::Contract("P3D output has no portable name".to_owned()))?;
    let staging = output.with_file_name(format!(".{file_name}.decompile.tmp"));
    let exporter = p3d::LosslessPackageExporter;
    match p3d::ExportPackage::execute(&exporter, input_path, &staging) {
        Ok(()) => {
            fs::rename(&staging, &output)?;
        }
        Err(error) => {
            if let Ok(metadata) = fs::symlink_metadata(&staging)
                && metadata.is_dir()
                && !metadata.file_type().is_symlink()
            {
                fs::remove_dir_all(&staging)?;
            }
            let diagnostic = output.with_extension("decompile-error.txt");
            local::write_text(
                &diagnostic,
                &format!(
                    "{error}
"
                ),
                false,
            )?;
        }
    }
    Ok(())
}

fn package_member_metadata(path: &str) -> (&'static str, &'static str) {
    if path == "conversion-report.json" {
        ("application/json", "legacy/evidence")
    } else if path.starts_with("decompiled/") {
        ("application/octet-stream", "legacy/decompiled")
    } else {
        ("application/octet-stream", "legacy/source")
    }
}

fn package_members(root: &Path) -> Result<Vec<Member>, ConvertError> {
    let mut members = Vec::new();
    for path in local::strict_regular_files(root)? {
        let relative = path.strip_prefix(root).map_err(|_error| {
            ConvertError::Contract("package member escaped conversion root".to_owned())
        })?;
        let portable = relative.to_string_lossy().replace('\\', "/");
        if portable == "mod.json" {
            continue;
        }
        let bytes = local::read_bytes(&path)?;
        let (media_type, role) = package_member_metadata(&portable);
        members.push(member_from_bytes(&portable, media_type, role, &bytes)?);
    }
    members.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(members)
}

pub(crate) fn package_manifest(
    root: &Path,
    source_sha256: &str,
) -> Result<PackageManifest, ConvertError> {
    let source_prefix = source_sha256.get(..16).ok_or_else(|| {
        ConvertError::Contract("source SHA-256 is unexpectedly short".to_owned())
    })?;
    let members = package_members(root)?;
    let package_revision = content_revision(&members)?;
    let manifest = PackageManifest {
        contract_version: MOD_CONTRACT_VERSION.to_owned(),
        canonical_id: format!("shar.legacy.lmlm.{source_prefix}"),
        package_revision,
        package_kind: PackageKind::Content,
        priority: 0,
        dependencies: Vec::new(),
        conflicts: Vec::new(),
        supersedes: Vec::new(),
        required_capabilities: vec!["legacy.lmlm.review.v1".to_owned()],
        supported_targets: Vec::new(),
        members,
        provenance: Provenance {
            authors: vec!["source-package-rightsholders".to_owned()],
            source: "converted-from-user-supplied-legacy-lmlm".to_owned(),
            license: "NOASSERTION".to_owned(),
        },
        trust_level: TrustLevel::ContentOnly,
    };
    manifest.validate()?;
    Ok(manifest)
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
    let package = package_manifest(root, &report.source_sha256)?;
    local::write_text(
        &root.join("mod.json"),
        &package.to_pretty_json()?,
        false,
    )?;
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

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
//   - Filesystem outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for rcf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem outbound adapter.

use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use schoenwald_filesystem::DiagnosticPath;
use schoenwald_filesystem::adapters::driving::local;

use crate::domain::path_policy::contains_unsafe_unicode_path_control;
use crate::domain::{ArchiveEntry, ArchiveError};
use crate::ports::{ArchiveByteReader, ArchiveSource, EntrySink};

/// Characters rejected by the supported Windows extraction target.
const WINDOWS_INVALID_PATH_CHARACTERS: [char; 7] =
    [':', '<', '>', '"', '|', '?', '*'];
/// Maximum UTF-16 code units in one supported Windows component.
const WINDOWS_COMPONENT_LIMIT: usize = 255;
/// Suffixes recognized as DOS communication and printer device numbers.
const WINDOWS_RESERVED_DEVICE_NUMBERS: [&str; 12] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "\u{00b9}", "\u{00b2}",
    "\u{00b3}",
];

/// File-backed archive source.
#[derive(Debug, Clone)]
pub struct FileArchiveSource {
    /// Kept private so all archive reads flow through the range-checked port.
    path: PathBuf,
}

impl FileArchiveSource {
    /// Creates a file-backed archive source.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns the source path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl ArchiveSource for FileArchiveSource {
    fn open_reader(
        &self,
    ) -> Result<Box<dyn ArchiveByteReader + '_>, ArchiveError> {
        let file = File::open(&self.path)
            .map_err(|source| ArchiveError::io(&self.path, source))?;
        let metadata = file
            .metadata()
            .map_err(|source| ArchiveError::io(&self.path, source))?;
        let archive_length = metadata.len();
        Ok(Box::new(FileByteReader {
            path: self.path.clone(),
            file,
            archive_length,
        }))
    }

    fn archive_stem(&self) -> Result<String, ArchiveError> {
        let stem = self
            .path
            .file_stem()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                ArchiveError::invalid_archive(format!(
                    "archive path has no UTF-8 stem: {}",
                    DiagnosticPath::new(&self.path)
                ))
            })?;
        Ok(stem.to_owned())
    }
}

/// Owns the file handle so archive offsets are read through one checked cursor.
struct FileByteReader {
    /// Preserved for IO error context without exposing local paths in domain
    /// types.
    path: PathBuf,
    /// Mutated only by `read_range` so callers cannot bypass bounds-aware
    /// reads.
    file: File,
    /// Length captured from the opened file handle for stable range checks.
    archive_length: u64,
}

impl ArchiveByteReader for FileByteReader {
    fn len(&self) -> Result<u64, ArchiveError> {
        Ok(self.archive_length)
    }

    fn read_range(
        &mut self,
        offset: u64,
        length: u64,
    ) -> Result<Vec<u8>, ArchiveError> {
        let archive_length = self.len()?;
        let range_end = offset.checked_add(length).ok_or_else(|| {
            ArchiveError::invalid_archive("archive read range overflow")
        })?;
        if range_end > archive_length {
            return Err(ArchiveError::invalid_archive(
                "archive read range exceeds archive length",
            ));
        }
        let _position = self
            .file
            .seek(SeekFrom::Start(offset))
            .map_err(|source| ArchiveError::io(&self.path, source))?;
        let range_length = usize::try_from(length).map_err(|source| {
            ArchiveError::invalid_archive(format!(
                "range is too large to allocate: {length}: {source}"
            ))
        })?;
        let mut payload = vec![0; range_length];
        self.file.read_exact(&mut payload).map_err(|source| {
            if source.kind() == ErrorKind::UnexpectedEof {
                return ArchiveError::invalid_archive(
                    "archive changed after reader opened",
                );
            }
            ArchiveError::io(&self.path, source)
        })?;
        Ok(payload)
    }
}

/// Filesystem sink that writes entries under an output root.
#[derive(Debug, Clone)]
pub struct FileEntrySink {
    /// Root is private so every written entry passes through path traversal
    /// checks.
    output_root: PathBuf,
}

impl FileEntrySink {
    /// Creates a filesystem entry sink.
    #[must_use]
    pub fn new(output_root: impl Into<PathBuf>) -> Self {
        Self {
            output_root: output_root.into(),
        }
    }

    /// Returns the output root.
    #[must_use]
    pub fn output_root(&self) -> &Path {
        &self.output_root
    }
}

impl EntrySink for FileEntrySink {
    fn prepare_archive(
        &mut self,
        archive_stem: &str,
        entries: &[ArchiveEntry],
    ) -> Result<(), ArchiveError> {
        let _archive_directory = safe_archive_stem(archive_stem)?;
        for entry in entries {
            let entry_name = &entry.name;
            let _relative = safe_relative_path(entry_name)?;
        }
        Ok(())
    }

    fn write_entry(
        &mut self,
        archive_stem: &str,
        entry_name: &str,
        payload: &[u8],
    ) -> Result<PathBuf, ArchiveError> {
        let archive_directory = safe_archive_stem(archive_stem)?;
        let relative = safe_relative_path(entry_name)?;
        let output_path =
            self.output_root.join(archive_directory).join(relative);
        local::write_bytes(&output_path, payload, true)
            .map_err(|source| ArchiveError::io(output_path.clone(), source))?;
        Ok(output_path)
    }
}

/// Validates the archive directory as one relative path component.
///
/// # Errors
///
/// Returns an error when the stem can escape or subdivide the output root.
fn safe_archive_stem(archive_stem: &str) -> Result<&str, ArchiveError> {
    if archive_stem.is_empty()
        || archive_stem == "."
        || archive_stem == ".."
        || archive_stem.contains(['/', '\\'])
        || contains_windows_invalid_character(archive_stem)
        || has_windows_trimmed_suffix(archive_stem)
        || is_windows_reserved_component(archive_stem)
        || exceeds_windows_component_limit(archive_stem)
        || archive_stem.chars().any(char::is_control)
        || contains_unsafe_unicode_path_control(archive_stem)
    {
        return Err(ArchiveError::unsafe_entry_path(archive_stem));
    }
    Ok(archive_stem)
}

/// Reports characters that the supported Windows target cannot store.
fn contains_windows_invalid_character(value: &str) -> bool {
    value
        .chars()
        .any(|character| WINDOWS_INVALID_PATH_CHARACTERS.contains(&character))
}

/// Reports suffixes that Windows removes while resolving a component.
fn has_windows_trimmed_suffix(value: &str) -> bool {
    value.ends_with([' ', '.'])
}

/// Reports names reserved for Windows device namespaces.
fn is_windows_reserved_component(value: &str) -> bool {
    let base = value.split('.').next().unwrap_or(value);
    let folded = base.to_ascii_lowercase();
    if matches!(folded.as_str(), "con" | "prn" | "aux" | "nul")
        || matches!(folded.as_str(), "conin$" | "conout$" | "clock$")
    {
        return true;
    }
    folded
        .strip_prefix("com")
        .or_else(|| folded.strip_prefix("lpt"))
        .is_some_and(|number| WINDOWS_RESERVED_DEVICE_NUMBERS.contains(&number))
}

/// Reports components that exceed the supported Windows UTF-16 limit.
fn exceeds_windows_component_limit(value: &str) -> bool {
    value.encode_utf16().count() > WINDOWS_COMPONENT_LIMIT
}

/// Keeps archive-controlled names below the extraction root before any IO
/// occurs.
fn safe_relative_path(entry_name: &str) -> Result<PathBuf, ArchiveError> {
    let normalized = entry_name.replace('\\', "/");
    let mut path = PathBuf::new();
    for part in normalized.split('/') {
        if part.is_empty()
            || part == "."
            || part == ".."
            || contains_windows_invalid_character(part)
            || has_windows_trimmed_suffix(part)
            || is_windows_reserved_component(part)
            || exceeds_windows_component_limit(part)
            || part.chars().any(char::is_control)
            || contains_unsafe_unicode_path_control(part)
        {
            return Err(ArchiveError::unsafe_entry_path(entry_name));
        }
        path.push(part);
    }
    if path.as_os_str().is_empty() {
        return Err(ArchiveError::unsafe_entry_path(entry_name));
    }
    Ok(path)
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/formats/rcf/unit/adapter-outbound/filesystem/tests.rs"]
mod tests;

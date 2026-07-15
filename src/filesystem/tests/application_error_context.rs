// File:
//   - application_error_context.rs
// Path:
//   - src/filesystem/tests/application_error_context.rs
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
//   - Regression coverage for application-owned filesystem diagnostics.
// - Must-Not:
//   - Depend on localized native error wording or caller-domain policy.
// - Allows:
//   - Assert stable operation and path context for application failures.
// - Split-When:
//   - Split when another application error family needs independent fixtures.
// - Merge-When:
//   - Another test target owns the same application diagnostic contract.
// - Summary:
//   - Filesystem application error context tests.
// - Description:
//   - Protects path validation, decoding, and tree-entry diagnostics.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Assertions inspect stable context rather than native source wording.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for application-owned filesystem diagnostics.
//!
//! Every rejected operation must identify its action and failing path.
use std::error::Error;
use std::path::{Path, PathBuf, StripPrefixError};
use std::string::FromUtf8Error;
use std::{fs, io};

use schoenwald_filesystem::{DiagnosticPath, RootedPathError};
use schoenwald_filesystem::adapters::driving::local;
use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

fn require_context(
    error: &io::Error,
    operation: &str,
    path: &Path,
) -> Result<(), String> {
    let rendered = error.to_string();
    if !rendered.contains(operation) {
        return Err(format!("missing operation context: {rendered}"));
    }
    let displayed = DiagnosticPath::new(path).to_string();
    if !rendered.contains(&displayed) {
        return Err(format!("missing path context: {rendered}"));
    }
    Ok(())
}

fn contextual_source(error: &io::Error) -> Option<&(dyn Error + 'static)> {
    error
        .get_ref()
        .and_then(|context| context.source())
}

fn is_utf8_source(source: &(dyn Error + 'static)) -> bool {
    source.is::<FromUtf8Error>()
}

fn is_rooted_path_source(source: &(dyn Error + 'static)) -> bool {
    source.is::<RootedPathError>()
}

fn is_strip_prefix_source(source: &(dyn Error + 'static)) -> bool {
    source.is::<StripPrefixError>()
}

struct SiblingTree;

impl TreeReader for SiblingTree {
    fn regular_files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(vec![PathBuf::from("root-sibling/file.bin")])
    }
}

struct EscapingTree;

impl TreeReader for EscapingTree {
    fn regular_files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(vec![PathBuf::from("root/../escape.bin")])
    }
}

#[test]
fn invalid_utf8_error_includes_operation_and_path() -> Result<(), String> {
    let path = std::env::temp_dir().join(
        format!(
            "schoenwald-filesystem-invalid-utf8-{}",
            std::process::id()
        ),
    );
    fs::write(
        &path,
        [0xff_u8],
    )
    .map_err(|error| error.to_string())?;

    let result = local::read_utf8(&path);

    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("invalid UTF-8 unexpectedly decoded".to_owned());
    };
    require_context(
        &error,
        "decode UTF-8 file",
        &path,
    )?;
    let source = contextual_source(&error);
    let has_expected_source = source.is_some_and(is_utf8_source);
    if !has_expected_source {
        return Err("UTF-8 error source was discarded".to_owned());
    }
    Ok(())
}

#[test]
fn portable_path_error_includes_operation_and_path() -> Result<(), String> {
    let path = Path::new("NUL");
    let result = local::path_kind(path);
    let Err(error) = result else {
        return Err("reserved host alias unexpectedly inspected".to_owned());
    };
    require_context(
        &error,
        "inspect path metadata",
        path,
    )?;
    let source = contextual_source(&error);
    if !source.is_some_and(is_rooted_path_source) {
        return Err("rooted-path error source was discarded".to_owned());
    }
    Ok(())
}

#[test]
fn control_path_error_is_single_line_and_reversible() -> Result<(), String> {
    let path = Path::new("bad\npath");
    let result = local::path_kind(path);
    let Err(error) = result else {
        return Err("control-bearing path unexpectedly inspected".to_owned());
    };
    let rendered = error.to_string();
    if rendered.contains('\n') {
        return Err(
            format!("path diagnostic contains a raw newline: {rendered:?}"),
        );
    }
    if !rendered.contains(r"bad\npath") {
        return Err(
            format!("path diagnostic lost escaped identity: {rendered}"),
        );
    }
    Ok(())
}

#[test]
fn diagnostic_path_distinguishes_literal_escape_from_control() {
    let literal = DiagnosticPath::new(Path::new(r"bad\npath")).to_string();
    let control = DiagnosticPath::new(Path::new("bad\npath")).to_string();

    assert_eq!(literal, r"bad\\npath");
    assert_eq!(control, r"bad\npath");
    assert_ne!(literal, control);
}

#[test]
fn root_creation_error_includes_operation_and_path() -> Result<(), String> {
    let path = if cfg!(windows) {
        Path::new(r"C:\")
    } else {
        Path::new("/")
    };
    let result = local::create_dir_all(path);
    let Err(error) = result else {
        return Err(
            "filesystem root unexpectedly reported creation".to_owned(),
        );
    };
    require_context(
        &error,
        "create directory tree",
        path,
    )
}

#[test]
fn tree_entry_error_includes_operation_and_path() -> Result<(), String> {
    let path = Path::new("root/../escape.bin");
    let result = CollectRegularFiles::execute(
        &EscapingTree,
        Path::new("root"),
    );
    let Err(error) = result else {
        return Err("escaping tree entry unexpectedly accepted".to_owned());
    };
    require_context(
        &error,
        "validate tree entry",
        path,
    )?;
    let source = contextual_source(&error);
    if !source.is_some_and(is_rooted_path_source) {
        return Err("tree rooted-path source was discarded".to_owned());
    }
    Ok(())
}

#[test]
fn sibling_tree_error_preserves_strip_prefix_source() -> Result<(), String> {
    let path = Path::new("root-sibling/file.bin");
    let result = CollectRegularFiles::execute(
        &SiblingTree,
        Path::new("root"),
    );
    let Err(error) = result else {
        return Err("sibling tree entry unexpectedly accepted".to_owned());
    };
    require_context(
        &error,
        "validate tree entry",
        path,
    )?;
    let source = contextual_source(&error);
    if !source.is_some_and(is_strip_prefix_source) {
        return Err("strip-prefix error source was discarded".to_owned());
    }
    Ok(())
}

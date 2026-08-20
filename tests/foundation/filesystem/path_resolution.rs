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
//   - Path resolution test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Path resolution test module.
// - Description:
//   - Implements the declared test module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Path resolution test module.

use std::path::Path;

use schoenwald_filesystem::{RootedPathError, resolve_under};

#[test]
fn empty_relative_path_is_rejected() -> Result<(), String> {
    let result = resolve_under(Path::new("output"), Path::new(""));

    if result != Err(RootedPathError::Empty) {
        return Err(format!("unexpected empty-path resolution: {result:?}"));
    }
    Ok(())
}

#[test]
fn current_directory_relative_path_is_rejected() -> Result<(), String> {
    let result = resolve_under(Path::new("output"), Path::new("././"));

    if result != Err(RootedPathError::Empty) {
        return Err(format!(
            "unexpected current-directory resolution: {result:?}"
        ));
    }
    Ok(())
}

#[test]
fn empty_root_is_rejected() -> Result<(), String> {
    let result = resolve_under(Path::new(""), Path::new("file.bin"));

    if result != Err(RootedPathError::EmptyRoot) {
        return Err(format!("unexpected empty-root resolution: {result:?}"));
    }
    Ok(())
}

#[test]
fn trailing_dot_component_is_rejected() -> Result<(), String> {
    let result = resolve_under(Path::new("output"), Path::new("report."));

    if result != Err(RootedPathError::TrailingDot) {
        return Err(format!("unexpected trailing-dot resolution: {result:?}"));
    }
    Ok(())
}

#[test]
fn trailing_space_component_is_rejected() -> Result<(), String> {
    let result = resolve_under(Path::new("output"), Path::new("report\u{20}"));

    if result != Err(RootedPathError::TrailingSpace) {
        return Err(format!(
            "unexpected trailing-space resolution: {result:?}"
        ));
    }
    Ok(())
}

#[test]
fn alternate_data_stream_component_is_rejected() -> Result<(), String> {
    let result =
        resolve_under(Path::new("output"), Path::new("report.txt:hidden"));

    if result != Err(RootedPathError::AlternateDataStream) {
        return Err(format!(
            "unexpected alternate-stream resolution: {result:?}"
        ));
    }
    Ok(())
}

#[test]
fn forbidden_host_character_is_rejected() -> Result<(), String> {
    let result = resolve_under(Path::new("output"), Path::new("report?.txt"));

    if result != Err(RootedPathError::ForbiddenCharacter) {
        return Err(format!(
            "unexpected forbidden-character resolution: {result:?}"
        ));
    }
    Ok(())
}

#[test]
fn control_character_component_is_rejected() -> Result<(), String> {
    let result =
        resolve_under(Path::new("output"), Path::new("report\u{0001}.txt"));

    if result != Err(RootedPathError::ControlCharacter) {
        return Err(format!(
            "unexpected control-character resolution: {result:?}"
        ));
    }
    Ok(())
}

#[test]
fn unicode_path_modifier_is_rejected() -> Result<(), String> {
    let result =
        resolve_under(Path::new("output"), Path::new("report\u{202e}.txt"));

    if result != Err(RootedPathError::UnicodePathModifier) {
        return Err(format!(
            "unexpected Unicode-modifier resolution: {result:?}"
        ));
    }
    Ok(())
}

#[test]
fn overlong_component_is_rejected() -> Result<(), String> {
    let component = "a".repeat(256);
    let result = resolve_under(Path::new("output"), Path::new(&component));

    if result != Err(RootedPathError::ComponentTooLong) {
        return Err(format!(
            "unexpected overlong-component resolution: {result:?}"
        ));
    }
    Ok(())
}

#[test]
fn reserved_host_alias_root_is_rejected() -> Result<(), String> {
    let result = resolve_under(Path::new("NUL"), Path::new("report.txt"));

    if result != Err(RootedPathError::ReservedHostAlias) {
        return Err(format!("unexpected reserved-root resolution: {result:?}"));
    }
    Ok(())
}

#[cfg(not(windows))]
#[test]
fn backslash_component_is_rejected() -> Result<(), String> {
    let result =
        resolve_under(Path::new("output"), Path::new(r"folder\file.bin"));

    if result != Err(RootedPathError::ForbiddenCharacter) {
        return Err(format!("unexpected backslash resolution: {result:?}"));
    }
    Ok(())
}

#[test]
fn parent_traversal_in_root_is_rejected() -> Result<(), String> {
    let result = resolve_under(Path::new("output/.."), Path::new("escape.bin"));

    if result != Err(RootedPathError::ParentTraversal) {
        return Err(format!(
            "unexpected traversing-root resolution: {result:?}"
        ));
    }
    Ok(())
}

#[test]
fn unicode_line_separator_is_rejected() -> Result<(), String> {
    let result = resolve_under(
        Path::new("output"),
        Path::new("report\u{2028}hidden.txt"),
    );

    if result != Err(RootedPathError::UnicodePathModifier) {
        return Err(format!(
            "unexpected line-separator resolution: {result:?}"
        ));
    }
    Ok(())
}

#[test]
fn unicode_variation_selector_is_rejected() -> Result<(), String> {
    let result =
        resolve_under(Path::new("output"), Path::new("report\u{fe0f}.txt"));

    if result != Err(RootedPathError::UnicodePathModifier) {
        return Err(format!(
            "unexpected variation-selector resolution: {result:?}"
        ));
    }
    Ok(())
}

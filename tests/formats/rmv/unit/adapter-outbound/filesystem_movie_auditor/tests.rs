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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use super::{FilesystemMovieAuditor, destination_path};
use crate::domain::{MovieKind, RmvError};
use crate::ports::MovieAuditor as _;

static TEMP_ID: AtomicU64 = AtomicU64::new(0);

fn temp_root(label: &str) -> PathBuf {
    let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "rmv-filesystem-{label}-{}-{id}",
        std::process::id()
    ))
}

#[test]
fn rejects_windows_unsafe_output_components() {
    for (root_name, relative) in [
        ("CON", "movie.rmv"),
        ("movies", "AUX/movie.rmv"),
        ("movies", "movie?.rmv"),
    ] {
        assert!(
            destination_path(
                Path::new("out"),
                OsStr::new(root_name),
                Path::new(relative),
            )
            .is_err(),
            "unsafe output component was accepted: {root_name}/{relative}"
        );
    }
}

#[test]
fn excludes_nested_output_directories_from_source_audits() {
    let root = temp_root("nested-output");
    let output = root.join("generated");
    assert!(fs::create_dir_all(&output).is_ok());
    assert!(fs::write(root.join("source.rmv"), b"BIKi-source").is_ok());
    assert!(fs::write(output.join("stale.rmv"), b"BIKi-stale").is_ok());

    let audit_result = FilesystemMovieAuditor
        .audit_roots(std::slice::from_ref(&root), &output);
    let _cleanup = fs::remove_dir_all(&root);
    assert!(audit_result.is_ok(), "nested output should be excluded");
    let Ok(report) = audit_result else {
        return;
    };
    assert_eq!(report.records.len(), 1);
    let Some(record) = report.records.first() else {
        return;
    };
    assert_eq!(record.relative_path, PathBuf::from("source.rmv"));
}

#[test]
fn rejects_input_roots_inside_the_output_tree() {
    let output = temp_root("input-inside-output");
    let input = output.join("source");
    assert!(fs::create_dir_all(&input).is_ok());
    assert!(fs::write(input.join("movie.rmv"), b"BIKi-source").is_ok());

    let audit_result = FilesystemMovieAuditor
        .audit_roots(std::slice::from_ref(&input), &output);
    let _cleanup = fs::remove_dir_all(&output);
    assert!(
        matches!(
            audit_result,
            Err(RmvError::InputRootInsideOutput(path)) if path == input
        ),
        "an input inside the output tree must be rejected"
    );
}

#[test]
fn audits_roots_with_parent_components() {
    let parent = temp_root("parent-component");
    let root = parent.join("movies");
    let child = root.join("child");
    let root_with_parent = child.join("..");
    let output = temp_root("parent-component-output");
    assert!(fs::create_dir_all(&child).is_ok());
    assert!(fs::write(root.join("movie.rmv"), b"BIKi-source").is_ok());

    let audit_result = FilesystemMovieAuditor
        .audit_roots(std::slice::from_ref(&root_with_parent), &output);
    let _cleanup_input = fs::remove_dir_all(&parent);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(
        audit_result.is_ok(),
        "canonical parent components should remain valid roots"
    );
    let Ok(report) = audit_result else {
        return;
    };
    let Some(record) = report.records.first() else {
        return;
    };
    assert_eq!(record.output_path, output.join("movies").join("movie.bk2"));
}

#[test]
fn audits_overlapping_roots_only_once_per_physical_source() {
    let root = temp_root("overlap");
    let nested = root.join("movies");
    let output = temp_root("overlap-output");
    assert!(fs::create_dir_all(&nested).is_ok());
    assert!(fs::write(nested.join("movie.rmv"), b"BIKi-source").is_ok());

    let audit_result =
        FilesystemMovieAuditor.audit_roots(&[root.clone(), nested], &output);
    let _cleanup_input = fs::remove_dir_all(&root);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(audit_result.is_ok(), "overlapping roots should audit");
    let Ok(report) = audit_result else {
        return;
    };
    assert_eq!(report.records.len(), 1);
}

#[test]
fn overlapping_root_order_does_not_change_source_identity() {
    let root = temp_root("overlap-order");
    let nested = root.join("movies");
    let first_output = temp_root("overlap-order-first-output");
    let second_output = temp_root("overlap-order-second-output");
    assert!(fs::create_dir_all(&nested).is_ok());
    assert!(fs::write(nested.join("movie.rmv"), b"BIKi-source").is_ok());

    let parent_first = FilesystemMovieAuditor
        .audit_roots(&[root.clone(), nested.clone()], &first_output);
    let child_first = FilesystemMovieAuditor
        .audit_roots(&[nested, root.clone()], &second_output);
    let _cleanup_input = fs::remove_dir_all(&root);
    let _cleanup_first_output = fs::remove_dir_all(&first_output);
    let _cleanup_second_output = fs::remove_dir_all(&second_output);
    assert!(parent_first.is_ok());
    assert!(child_first.is_ok());
    let Ok(parent_first_report) = parent_first else {
        return;
    };
    let Ok(child_first_report) = child_first else {
        return;
    };
    let Some(parent_first_record) = parent_first_report.records.first() else {
        return;
    };
    let Some(child_first_record) = child_first_report.records.first() else {
        return;
    };
    assert_eq!(
        parent_first_record.source_root,
        child_first_record.source_root
    );
    assert_eq!(
        parent_first_record.relative_path,
        child_first_record.relative_path
    );
}

#[test]
fn rejects_bink_v1_inputs_with_mismatched_declared_lengths() {
    let root = temp_root("bink-length-mismatch");
    let output = temp_root("bink-length-mismatch-output");
    assert!(fs::create_dir_all(&root).is_ok());
    assert!(
        fs::write(root.join("movie.rmv"), b"BIKi\0\0\0\0\x01\x02\x03\x04")
            .is_ok()
    );

    let audit_result = FilesystemMovieAuditor
        .audit_roots(std::slice::from_ref(&root), &output);
    let _cleanup_input = fs::remove_dir_all(&root);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(
        audit_result.is_ok(),
        "mismatched Bink length should produce an audit record"
    );
    let Ok(report) = audit_result else {
        return;
    };
    assert_eq!(
        report.records.len(),
        1,
        "one input must produce one audit record"
    );
    let Some(record) = report.records.first() else {
        return;
    };
    assert_eq!(record.kind, MovieKind::Unknown);
}

#[test]
fn counts_signature_padded_bink_outputs_as_missing() {
    let root = temp_root("signature-only-output");
    let output = temp_root("signature-only-output-root");
    assert!(fs::create_dir_all(&root).is_ok());
    assert!(fs::write(root.join("movie.rmv"), b"BIKi-source").is_ok());
    let Some(root_name) = root.file_name() else {
        return;
    };
    let expected_output = output.join(root_name).join("movie.bk2");
    let Some(parent) = expected_output.parent() else {
        return;
    };
    assert!(fs::create_dir_all(parent).is_ok());
    let mut fake_output = [0_u8; MovieKind::HEADER_PROBE_LEN];
    for (target, source) in fake_output.iter_mut().zip(*b"KB2i") {
        *target = source;
    }
    assert!(fs::write(&expected_output, fake_output).is_ok());

    let audit_result = FilesystemMovieAuditor
        .audit_roots(std::slice::from_ref(&root), &output);
    let _cleanup_input = fs::remove_dir_all(&root);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(audit_result.is_ok(), "temporary RMV tree should audit");
    let Ok(report) = audit_result else {
        return;
    };
    assert_eq!(report.missing_bk2_outputs, 1);
}

#[test]
fn counts_truncated_bink_output_paths_as_missing() {
    let root = temp_root("truncated-output");
    let output = temp_root("truncated-output-root");
    assert!(fs::create_dir_all(&root).is_ok());
    assert!(fs::write(root.join("movie.rmv"), b"BIKi-source").is_ok());
    let Some(root_name) = root.file_name() else {
        return;
    };
    let expected_output = output.join(root_name).join("movie.bk2");
    let Some(parent) = expected_output.parent() else {
        return;
    };
    assert!(fs::create_dir_all(parent).is_ok());
    assert!(fs::write(&expected_output, b"KB2iX").is_ok());

    let audit_result = FilesystemMovieAuditor
        .audit_roots(std::slice::from_ref(&root), &output);
    let _cleanup_input = fs::remove_dir_all(&root);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(audit_result.is_ok(), "temporary RMV tree should audit");
    let Ok(report) = audit_result else {
        return;
    };
    assert_eq!(report.missing_bk2_outputs, 1);
}

#[test]
fn counts_non_bink_output_paths_as_missing() {
    let root = temp_root("invalid-output");
    let output = temp_root("invalid-output-root");
    assert!(fs::create_dir_all(&root).is_ok());
    assert!(fs::write(root.join("movie.rmv"), b"BIKi-source").is_ok());
    let Some(root_name) = root.file_name() else {
        return;
    };
    let expected_output = output.join(root_name).join("movie.bk2");
    assert!(fs::create_dir_all(&expected_output).is_ok());

    let audit_result = FilesystemMovieAuditor
        .audit_roots(std::slice::from_ref(&root), &output);
    let _cleanup_input = fs::remove_dir_all(&root);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(audit_result.is_ok(), "temporary RMV tree should audit");
    let Ok(report) = audit_result else {
        return;
    };
    assert_eq!(report.missing_bk2_outputs, 1);
}

#[cfg(windows)]
#[test]
fn preserves_distinct_unicode_expansion_output_paths() {
    assert_ne!(
        super::output_identity(Path::new("straße")),
        super::output_identity(Path::new("strasse"))
    );
}

#[cfg(windows)]
#[test]
fn rejects_unicode_case_aliases_of_windows_output_paths() {
    let left_parent = temp_root("unicode-collision-left");
    let right_parent = temp_root("unicode-collision-right");
    let left = left_parent.join("MÖVIES");
    let right = right_parent.join("mövies");
    let output = temp_root("unicode-collision-output");
    assert!(fs::create_dir_all(&left).is_ok());
    assert!(fs::create_dir_all(&right).is_ok());
    assert!(fs::write(left.join("intro.rmv"), b"BIKi-left").is_ok());
    assert!(fs::write(right.join("intro.rmv"), b"BIKi-right").is_ok());

    let audit_result =
        FilesystemMovieAuditor.audit_roots(&[left, right], &output);
    let _cleanup_left = fs::remove_dir_all(&left_parent);
    let _cleanup_right = fs::remove_dir_all(&right_parent);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(
        audit_result.is_err(),
        "Unicode case-aliasing Windows outputs must fail closed"
    );
}

#[cfg(windows)]
#[test]
fn rejects_case_aliases_of_the_same_windows_output_path() {
    let left_parent = temp_root("case-collision-left");
    let right_parent = temp_root("case-collision-right");
    let left = left_parent.join("Movies");
    let right = right_parent.join("movies");
    let output = temp_root("case-collision-output");
    assert!(fs::create_dir_all(&left).is_ok());
    assert!(fs::create_dir_all(&right).is_ok());
    assert!(fs::write(left.join("intro.rmv"), b"BIKi-left").is_ok());
    assert!(fs::write(right.join("intro.rmv"), b"BIKi-right").is_ok());

    let audit_result =
        FilesystemMovieAuditor.audit_roots(&[left, right], &output);
    let _cleanup_left = fs::remove_dir_all(&left_parent);
    let _cleanup_right = fs::remove_dir_all(&right_parent);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(
        audit_result.is_err(),
        "case-aliasing Windows outputs must fail closed"
    );
}

#[test]
fn rejects_distinct_sources_with_the_same_output_path() {
    let left_parent = temp_root("collision-left");
    let right_parent = temp_root("collision-right");
    let left = left_parent.join("movies");
    let right = right_parent.join("movies");
    let output = temp_root("collision-output");
    assert!(fs::create_dir_all(&left).is_ok());
    assert!(fs::create_dir_all(&right).is_ok());
    assert!(fs::write(left.join("intro.rmv"), b"BIKi-left").is_ok());
    assert!(fs::write(right.join("intro.rmv"), b"BIKi-right").is_ok());

    let audit_result =
        FilesystemMovieAuditor.audit_roots(&[left, right], &output);
    let _cleanup_left = fs::remove_dir_all(&left_parent);
    let _cleanup_right = fs::remove_dir_all(&right_parent);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(
        audit_result.is_err(),
        "colliding output identities must fail closed"
    );
}

#[test]
fn returns_records_in_relative_path_order() {
    let root = temp_root("order");
    let output = temp_root("order-output");
    let first = root.join("a");
    let second = root.join("b");
    assert!(fs::create_dir_all(&first).is_ok());
    assert!(fs::create_dir_all(&second).is_ok());
    assert!(fs::write(first.join("movie.rmv"), b"BIKi-a").is_ok());
    assert!(fs::write(second.join("movie.rmv"), b"BIKi-b").is_ok());

    let audit_result = FilesystemMovieAuditor
        .audit_roots(std::slice::from_ref(&root), &output);
    let _cleanup_input = fs::remove_dir_all(&root);
    let _cleanup_output = fs::remove_dir_all(&output);
    assert!(audit_result.is_ok(), "temporary RMV tree should audit");
    let Ok(report) = audit_result else {
        return;
    };
    let relative_paths = report
        .records
        .iter()
        .map(|record| record.relative_path.clone())
        .collect::<Vec<_>>();
    assert_eq!(relative_paths, vec![
        PathBuf::from("a/movie.rmv"),
        PathBuf::from("b/movie.rmv")
    ]);
}

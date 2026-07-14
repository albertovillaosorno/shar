// File:
//   - filesystem_movie_auditor.rs
// Path:
//   - src/rmv/src/adapters/driven/filesystem_movie_auditor.rs
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
//   - The rmv adapter boundary for adapters filesystem.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when filesystem contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Filesystem implementation of RMV audit and dedupe.
// - Description:
//   - Defines filesystem data and behavior for rmv adapters.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Filesystem discovery, output probing, and their focused adapter
//   - regressions share one outbound audit-port implementation.
//

//! Filesystem implementation of RMV audit and dedupe.
//!
//! This boundary keeps filesystem implementation of rmv audit and dedupe
//! explicit and returns deterministic results to rmv callers.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local;

use crate::domain::{
    AuditReport, MovieKind, MovieRecord, ProvenanceEvidence, RmvError, Sha256,
    is_windows_safe_component,
};
use crate::ports::MovieAuditor;

#[derive(Debug, Default, Clone, Copy)]
/// Filesystemmovieauditor.
pub struct FilesystemMovieAuditor;

impl MovieAuditor for FilesystemMovieAuditor {
    fn audit_roots(
        &self,
        roots: &[PathBuf],
        output_root: &Path,
    ) -> Result<AuditReport, RmvError> {
        if roots.is_empty() {
            return Err(RmvError::NoInputRoots);
        }
        let mut report = AuditReport::default();
        let mut source_identities = BTreeSet::new();
        let excluded_output_root = local::canonicalize(output_root).ok();
        let mut ordered_roots = roots.to_vec();
        ordered_roots.sort();
        for root in &ordered_roots {
            let canonical_root = local::canonicalize(root).map_err(
                |source| RmvError::Io {
                    path: root.clone(),
                    source,
                },
            )?;
            if excluded_output_root
                .as_ref()
                .is_some_and(
                    |output_identity| {
                        canonical_root.starts_with(output_identity)
                    },
                )
            {
                return Err(RmvError::InputRootInsideOutput(root.clone()));
            }
            let Some(root_name) = canonical_root.file_name() else {
                return Err(RmvError::InvalidRootName(root.clone()));
            };
            audit_root(
                root,
                root_name,
                output_root,
                excluded_output_root.as_deref(),
                &mut report,
                &mut source_identities,
            )?;
        }
        report
            .records
            .sort_by(
                |left, right| {
                    left.source_root
                        .cmp(&right.source_root)
                        .then_with(
                            || {
                                left.relative_path
                                    .cmp(&right.relative_path)
                            },
                        )
                },
            );
        let mut output_paths = BTreeSet::new();
        for record in &report.records {
            if !output_paths.insert(output_identity(&record.output_path)) {
                return Err(
                    RmvError::OutputPathCollision(
                        record
                            .output_path
                            .clone(),
                    ),
                );
            }
        }
        if report
            .records
            .is_empty()
        {
            return Err(RmvError::NoMovieInputs);
        }
        let mut counts = BTreeMap::new();
        for record in &report.records {
            let count = counts
                .entry(record.hash)
                .or_insert(0_usize);
            *count = (*count).saturating_add(1);
            if !is_valid_bink2_output(&record.output_path) {
                report.missing_bk2_outputs = report
                    .missing_bk2_outputs
                    .saturating_add(1);
            }
        }
        report.duplicate_inputs = counts
            .values()
            .map(|count| count.saturating_sub(1))
            .sum();
        Ok(report)
    }
}

/// Reports whether an expected output is a regular Bink2 file.
fn is_valid_bink2_output(path: &Path) -> bool {
    let Ok(bytes) = local::read_bytes(path) else {
        return false;
    };
    MovieKind::from_bytes(&bytes) == MovieKind::BinkV2
}

/// Returns a one-to-one uppercase mapping for Windows path identity.
#[cfg(windows)]
fn windows_case_character(character: char) -> char {
    let mut uppercase = character.to_uppercase();
    let first_uppercase = uppercase
        .next()
        .unwrap_or(character);
    if uppercase
        .next()
        .is_some()
    {
        character
    } else {
        first_uppercase
    }
}

/// Produces a platform-accurate output identity for collision checks.
#[cfg(windows)]
fn output_identity(path: &Path) -> Vec<u32> {
    use std::os::windows::ffi::OsStrExt as _;

    let mut identity = Vec::new();
    for decoded in char::decode_utf16(
        path.as_os_str()
            .encode_wide(),
    ) {
        match decoded {
            Ok(character) => {
                let identity_character = windows_case_character(character);
                identity.push(u32::from(identity_character));
            }
            Err(error) => {
                identity
                    .push(0x11_0000 + u32::from(error.unpaired_surrogate()));
            }
        }
    }
    identity
}

/// Produces a byte-exact Unix output identity for collision checks.
#[cfg(unix)]
fn output_identity(path: &Path) -> Vec<u32> {
    use std::os::unix::ffi::OsStrExt as _;

    path.as_os_str()
        .as_bytes()
        .iter()
        .copied()
        .map(u32::from)
        .collect()
}

/// Produces a stable fallback output identity on other platforms.
#[cfg(
    not(
        any(
            unix, windows
        )
    )
)]
fn output_identity(path: &Path) -> Vec<u32> {
    path.to_string_lossy()
        .chars()
        .map(u32::from)
        .collect()
}

/// Audit root.
fn audit_root(
    root: &Path,
    root_name: &std::ffi::OsStr,
    output_root: &Path,
    excluded_output_root: Option<&Path>,
    report: &mut AuditReport,
    source_identities: &mut BTreeSet<PathBuf>,
) -> Result<(), RmvError> {
    let mut stack = vec![root.to_path_buf()];
    while let Some(current) = stack.pop() {
        for dir_entry in fs::read_dir(&current).map_err(
            |source| RmvError::Io {
                path: current.clone(),
                source,
            },
        )? {
            let entry = dir_entry.map_err(
                |source| RmvError::Io {
                    path: current.clone(),
                    source,
                },
            )?;
            let path = entry.path();
            let file_type = entry
                .file_type()
                .map_err(
                    |source| RmvError::Io {
                        path: path.clone(),
                        source,
                    },
                )?;
            if file_type.is_dir() {
                let is_output_root = excluded_output_root.is_some_and(
                    |excluded| {
                        local::canonicalize(&path)
                            .is_ok_and(|identity| identity == excluded)
                    },
                );
                if !is_output_root {
                    stack.push(path);
                }
            } else if file_type.is_file() && has_rmv_extension(&path) {
                let source_identity = local::canonicalize(&path).map_err(
                    |source| RmvError::Io {
                        path: path.clone(),
                        source,
                    },
                )?;
                if !source_identities.insert(source_identity) {
                    continue;
                }
                let bytes = local::read_bytes(&path).map_err(
                    |source| RmvError::Io {
                        path: path.clone(),
                        source,
                    },
                )?;
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_error| RmvError::InvalidPath(path.clone()))?;
                let output_path = destination_path(
                    output_root,
                    root_name,
                    relative,
                )?;
                report
                    .records
                    .push(
                        MovieRecord {
                            source_root: root.to_path_buf(),
                            source_path: path.clone(),
                            relative_path: relative.to_path_buf(),
                            output_path,
                            bytes: u64::try_from(bytes.len())
                                .unwrap_or(u64::MAX),
                            kind: MovieKind::from_bytes(&bytes),
                            hash: Sha256::digest(&bytes),
                            provenance: ProvenanceEvidence::from_bytes(&bytes),
                        },
                    );
            }
        }
    }
    Ok(())
}

/// Has rmv extension.
fn has_rmv_extension(path: &Path) -> bool {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(|extension| extension.eq_ignore_ascii_case("rmv"))
}

/// Destination path.
fn destination_path(
    output_root: &Path,
    root_name: &std::ffi::OsStr,
    relative: &Path,
) -> Result<PathBuf, RmvError> {
    let Some(root_name_text) = root_name.to_str() else {
        return Err(RmvError::InvalidRootName(PathBuf::from(root_name)));
    };
    if !is_windows_safe_component(root_name_text) {
        return Err(RmvError::InvalidRootName(PathBuf::from(root_name)));
    }
    let mut out = output_root.to_path_buf();
    out.push(root_name);
    for component in relative.components() {
        match component {
            Component::Normal(part) => {
                let Some(part_text) = part.to_str() else {
                    return Err(RmvError::InvalidPath(relative.to_path_buf()));
                };
                if !is_windows_safe_component(part_text) {
                    return Err(RmvError::InvalidPath(relative.to_path_buf()));
                }
                out.push(part);
            }
            _ => return Err(RmvError::InvalidPath(relative.to_path_buf())),
        }
    }
    let _replaced_extension = out.set_extension("bk2");
    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{FilesystemMovieAuditor, destination_path};
    use crate::domain::{MovieKind, RmvError};
    use crate::ports::MovieAuditor as _;

    static TEMP_ID: AtomicU64 = AtomicU64::new(0);

    fn temp_root(label: &str) -> PathBuf {
        let id = TEMP_ID.fetch_add(
            1,
            Ordering::Relaxed,
        );
        std::env::temp_dir().join(
            format!(
                "rmv-filesystem-{label}-{}-{id}",
                std::process::id()
            ),
        )
    }

    #[test]
    fn rejects_windows_unsafe_output_components() {
        for (root_name, relative) in [
            (
                "CON",
                "movie.rmv",
            ),
            (
                "movies",
                "AUX/movie.rmv",
            ),
            (
                "movies",
                "movie?.rmv",
            ),
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
        assert!(
            fs::write(
                root.join("source.rmv"),
                b"BIKi-source"
            )
            .is_ok()
        );
        assert!(
            fs::write(
                output.join("stale.rmv"),
                b"BIKi-stale"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            std::slice::from_ref(&root),
            &output,
        );
        let _cleanup = fs::remove_dir_all(&root);
        assert!(
            audit_result.is_ok(),
            "nested output should be excluded"
        );
        let Ok(report) = audit_result else {
            return;
        };
        assert_eq!(
            report
                .records
                .len(),
            1
        );
        let Some(record) = report
            .records
            .first()
        else {
            return;
        };
        assert_eq!(
            record.relative_path,
            PathBuf::from("source.rmv")
        );
    }

    #[test]
    fn rejects_input_roots_inside_the_output_tree() {
        let output = temp_root("input-inside-output");
        let input = output.join("source");
        assert!(fs::create_dir_all(&input).is_ok());
        assert!(
            fs::write(
                input.join("movie.rmv"),
                b"BIKi-source"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            std::slice::from_ref(&input),
            &output,
        );
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
        assert!(
            fs::write(
                root.join("movie.rmv"),
                b"BIKi-source"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            std::slice::from_ref(&root_with_parent),
            &output,
        );
        let _cleanup_input = fs::remove_dir_all(&parent);
        let _cleanup_output = fs::remove_dir_all(&output);
        assert!(
            audit_result.is_ok(),
            "canonical parent components should remain valid roots"
        );
        let Ok(report) = audit_result else {
            return;
        };
        let Some(record) = report
            .records
            .first()
        else {
            return;
        };
        assert_eq!(
            record.output_path,
            output
                .join("movies")
                .join("movie.bk2")
        );
    }

    #[test]
    fn audits_overlapping_roots_only_once_per_physical_source() {
        let root = temp_root("overlap");
        let nested = root.join("movies");
        let output = temp_root("overlap-output");
        assert!(fs::create_dir_all(&nested).is_ok());
        assert!(
            fs::write(
                nested.join("movie.rmv"),
                b"BIKi-source"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            &[
                root.clone(),
                nested,
            ],
            &output,
        );
        let _cleanup_input = fs::remove_dir_all(&root);
        let _cleanup_output = fs::remove_dir_all(&output);
        assert!(
            audit_result.is_ok(),
            "overlapping roots should audit"
        );
        let Ok(report) = audit_result else {
            return;
        };
        assert_eq!(
            report
                .records
                .len(),
            1
        );
    }

    #[test]
    fn overlapping_root_order_does_not_change_source_identity() {
        let root = temp_root("overlap-order");
        let nested = root.join("movies");
        let first_output = temp_root("overlap-order-first-output");
        let second_output = temp_root("overlap-order-second-output");
        assert!(fs::create_dir_all(&nested).is_ok());
        assert!(
            fs::write(
                nested.join("movie.rmv"),
                b"BIKi-source"
            )
            .is_ok()
        );

        let parent_first = FilesystemMovieAuditor.audit_roots(
            &[
                root.clone(),
                nested.clone(),
            ],
            &first_output,
        );
        let child_first = FilesystemMovieAuditor.audit_roots(
            &[
                nested,
                root.clone(),
            ],
            &second_output,
        );
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
        let Some(parent_first_record) = parent_first_report
            .records
            .first()
        else {
            return;
        };
        let Some(child_first_record) = child_first_report
            .records
            .first()
        else {
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
            fs::write(
                root.join("movie.rmv"),
                b"BIKi\0\0\0\0\x01\x02\x03\x04"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            std::slice::from_ref(&root),
            &output,
        );
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
            report
                .records
                .len(),
            1,
            "one input must produce one audit record"
        );
        let Some(record) = report
            .records
            .first()
        else {
            return;
        };
        assert_eq!(
            record.kind,
            MovieKind::Unknown
        );
    }

    #[test]
    fn counts_signature_padded_bink_outputs_as_missing() {
        let root = temp_root("signature-only-output");
        let output = temp_root("signature-only-output-root");
        assert!(fs::create_dir_all(&root).is_ok());
        assert!(
            fs::write(
                root.join("movie.rmv"),
                b"BIKi-source"
            )
            .is_ok()
        );
        let Some(root_name) = root.file_name() else {
            return;
        };
        let expected_output = output
            .join(root_name)
            .join("movie.bk2");
        let Some(parent) = expected_output.parent() else {
            return;
        };
        assert!(fs::create_dir_all(parent).is_ok());
        let mut fake_output = [0_u8; MovieKind::HEADER_PROBE_LEN];
        for (target, source) in fake_output
            .iter_mut()
            .zip(*b"KB2i")
        {
            *target = source;
        }
        assert!(
            fs::write(
                &expected_output,
                fake_output
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            std::slice::from_ref(&root),
            &output,
        );
        let _cleanup_input = fs::remove_dir_all(&root);
        let _cleanup_output = fs::remove_dir_all(&output);
        assert!(
            audit_result.is_ok(),
            "temporary RMV tree should audit"
        );
        let Ok(report) = audit_result else {
            return;
        };
        assert_eq!(
            report.missing_bk2_outputs,
            1
        );
    }

    #[test]
    fn counts_truncated_bink_output_paths_as_missing() {
        let root = temp_root("truncated-output");
        let output = temp_root("truncated-output-root");
        assert!(fs::create_dir_all(&root).is_ok());
        assert!(
            fs::write(
                root.join("movie.rmv"),
                b"BIKi-source"
            )
            .is_ok()
        );
        let Some(root_name) = root.file_name() else {
            return;
        };
        let expected_output = output
            .join(root_name)
            .join("movie.bk2");
        let Some(parent) = expected_output.parent() else {
            return;
        };
        assert!(fs::create_dir_all(parent).is_ok());
        assert!(
            fs::write(
                &expected_output,
                b"KB2iX"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            std::slice::from_ref(&root),
            &output,
        );
        let _cleanup_input = fs::remove_dir_all(&root);
        let _cleanup_output = fs::remove_dir_all(&output);
        assert!(
            audit_result.is_ok(),
            "temporary RMV tree should audit"
        );
        let Ok(report) = audit_result else {
            return;
        };
        assert_eq!(
            report.missing_bk2_outputs,
            1
        );
    }

    #[test]
    fn counts_non_bink_output_paths_as_missing() {
        let root = temp_root("invalid-output");
        let output = temp_root("invalid-output-root");
        assert!(fs::create_dir_all(&root).is_ok());
        assert!(
            fs::write(
                root.join("movie.rmv"),
                b"BIKi-source"
            )
            .is_ok()
        );
        let Some(root_name) = root.file_name() else {
            return;
        };
        let expected_output = output
            .join(root_name)
            .join("movie.bk2");
        assert!(fs::create_dir_all(&expected_output).is_ok());

        let audit_result = FilesystemMovieAuditor.audit_roots(
            std::slice::from_ref(&root),
            &output,
        );
        let _cleanup_input = fs::remove_dir_all(&root);
        let _cleanup_output = fs::remove_dir_all(&output);
        assert!(
            audit_result.is_ok(),
            "temporary RMV tree should audit"
        );
        let Ok(report) = audit_result else {
            return;
        };
        assert_eq!(
            report.missing_bk2_outputs,
            1
        );
    }

    #[cfg(windows)]
    #[test]
    fn preserves_distinct_unicode_expansion_output_paths() {
        assert_ne!(
            // cspell:disable-next-line -- straße
            super::output_identity(Path::new("straße")),
            // cspell:disable-next-line -- strasse
            super::output_identity(Path::new("strasse"))
        );
    }

    #[cfg(windows)]
    #[test]
    fn rejects_unicode_case_aliases_of_windows_output_paths() {
        let left_parent = temp_root("unicode-collision-left");
        let right_parent = temp_root("unicode-collision-right");
        // cspell:disable-next-line -- MÖVIES
        let left = left_parent.join("MÖVIES");
        // cspell:disable-next-line -- mövies
        let right = right_parent.join("mövies");
        let output = temp_root("unicode-collision-output");
        assert!(fs::create_dir_all(&left).is_ok());
        assert!(fs::create_dir_all(&right).is_ok());
        assert!(
            fs::write(
                left.join("intro.rmv"),
                b"BIKi-left"
            )
            .is_ok()
        );
        assert!(
            fs::write(
                right.join("intro.rmv"),
                b"BIKi-right"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            &[
                left, right,
            ],
            &output,
        );
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
        assert!(
            fs::write(
                left.join("intro.rmv"),
                b"BIKi-left"
            )
            .is_ok()
        );
        assert!(
            fs::write(
                right.join("intro.rmv"),
                b"BIKi-right"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            &[
                left, right,
            ],
            &output,
        );
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
        assert!(
            fs::write(
                left.join("intro.rmv"),
                b"BIKi-left"
            )
            .is_ok()
        );
        assert!(
            fs::write(
                right.join("intro.rmv"),
                b"BIKi-right"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            &[
                left, right,
            ],
            &output,
        );
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
        assert!(
            fs::write(
                first.join("movie.rmv"),
                b"BIKi-a"
            )
            .is_ok()
        );
        assert!(
            fs::write(
                second.join("movie.rmv"),
                b"BIKi-b"
            )
            .is_ok()
        );

        let audit_result = FilesystemMovieAuditor.audit_roots(
            std::slice::from_ref(&root),
            &output,
        );
        let _cleanup_input = fs::remove_dir_all(&root);
        let _cleanup_output = fs::remove_dir_all(&output);
        assert!(
            audit_result.is_ok(),
            "temporary RMV tree should audit"
        );
        let Ok(report) = audit_result else {
            return;
        };
        let relative_paths = report
            .records
            .iter()
            .map(
                |record| {
                    record
                        .relative_path
                        .clone()
                },
            )
            .collect::<Vec<_>>();
        assert_eq!(
            relative_paths,
            vec![
                PathBuf::from("a/movie.rmv"),
                PathBuf::from("b/movie.rmv")
            ]
        );
    }
}

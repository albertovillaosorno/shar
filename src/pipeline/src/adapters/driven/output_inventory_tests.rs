// File:
//   - output_inventory_tests.rs
// Path:
//   - src/pipeline/src/adapters/driven/output_inventory_tests.rs
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
//   - Regression coverage for local output inventory behavior.
// - Must-Not:
//   - Implement production inventory policy or inspect repository data.
// - Allows:
//   - Create isolated temporary fixtures removed before return.
// - Split-When:
//   - Split when another provider gains independent regressions.
// - Merge-When:
//   - Tests no longer obscure the production inventory adapter.
// - Summary:
//   - Local output inventory regression tests.
// - Description:
//   - Verifies fail-closed root and directory classification.
// - Usage:
//   - Included by output_inventory.rs under cfg(test).
// - Defaults:
//   - Fixtures are process-scoped and explicitly removed.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression tests for local output inventory behavior.
//!
//! Each case protects one caller-visible storage invariant.

mod cases {
    use std::fs;
    use std::path::Path;

    use super::super::{
        FilesystemOutputInventory, OutputInventory, checked_byte_total,
    };

    const RESERVED_HOST_ALIAS: &[&str] = &["CON"];
    const TRAILING_DOT: &[&str] = &["art."];
    const RESERVED_PUNCTUATION: &[&str] = &["art?"];

    #[test]
    fn rejects_output_byte_total_overflow() -> Result<(), String> {
        let result = checked_byte_total(
            u64::MAX,
            1,
            Path::new("overflow.bin"),
        );
        if result.is_ok() {
            return Err(
                String::from("output byte total overflow was accepted"),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_invalid_labels_before_root_inspection() -> Result<(), String> {
        let root = std::env::temp_dir().join(
            format!(
                "pipeline-output-invalid-label-order-{}",
                std::process::id(),
            ),
        );
        fs::write(
            &root,
            b"root must not be inspected",
        )
        .map_err(|error| error.to_string())?;

        let result = FilesystemOutputInventory.summarize(
            &root,
            &["../outside"],
        );
        fs::remove_file(&root).map_err(|error| error.to_string())?;

        let actual = result.map_err(|error| error.to_string());
        let expected_message =
            String::from(r#"invalid named output directory: "../outside""#);
        if actual != Err(expected_message) {
            return Err(format!("unexpected validation order: {actual:?}"));
        }
        Ok(())
    }

    #[test]
    fn rejects_regular_file_output_roots() -> Result<(), String> {
        let root = std::env::temp_dir().join(
            format!(
                "pipeline-output-root-file-{}",
                std::process::id(),
            ),
        );
        fs::write(
            &root,
            b"not a directory",
        )
        .map_err(|error| error.to_string())?;

        let result = FilesystemOutputInventory.summarize(
            &root,
            &[],
        );
        fs::remove_file(&root).map_err(|error| error.to_string())?;

        if result.is_ok() {
            return Err(
                "a regular file must not be accepted as an output root"
                    .to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_file_collisions_for_named_directories() -> Result<(), String> {
        let root = std::env::temp_dir().join(
            format!(
                "pipeline-output-directory-file-{}",
                std::process::id(),
            ),
        );
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
        fs::create_dir_all(&root).map_err(|error| error.to_string())?;
        fs::write(
            root.join("artifacts"),
            b"not a directory",
        )
        .map_err(|error| error.to_string())?;

        let result = FilesystemOutputInventory.summarize(
            &root,
            &["artifacts"],
        );
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;

        if result.is_ok() {
            return Err(
                "a named output directory file collision must be rejected"
                    .to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_named_directory_parent_traversal() -> Result<(), String> {
        let base = std::env::temp_dir().join(
            format!(
                "pipeline-output-directory-traversal-{}",
                std::process::id(),
            ),
        );
        match fs::remove_dir_all(&base) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }
        let root = base.join("root");
        let outside = base.join("outside");
        fs::create_dir_all(&root).map_err(|error| error.to_string())?;
        fs::create_dir_all(&outside).map_err(|error| error.to_string())?;
        fs::write(
            outside.join("evidence.bin"),
            b"outside root",
        )
        .map_err(|error| error.to_string())?;

        let result = FilesystemOutputInventory.summarize(
            &root,
            &["../outside"],
        );
        fs::remove_dir_all(&base).map_err(|error| error.to_string())?;

        if result.is_ok() {
            return Err(
                "named output directory traversal must be rejected".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_nonportable_named_directories() -> Result<(), String> {
        let root = std::env::temp_dir().join(
            format!(
                "pipeline-output-directory-nonportable-{}",
                std::process::id(),
            ),
        );
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }

        for directories in [
            RESERVED_HOST_ALIAS,
            TRAILING_DOT,
            RESERVED_PUNCTUATION,
        ] {
            if FilesystemOutputInventory
                .summarize(
                    &root,
                    directories,
                )
                .is_ok()
            {
                let name = directories
                    .first()
                    .copied()
                    .ok_or_else(
                        || String::from("nonportable fixture has no name"),
                    )?;
                return Err(
                    format!("nonportable named output was accepted: {name:?}"),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_case_aliased_named_directories() -> Result<(), String> {
        let root = std::env::temp_dir().join(
            format!(
                "pipeline-output-directory-case-alias-{}",
                std::process::id(),
            ),
        );
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }

        let result = FilesystemOutputInventory.summarize(
            &root,
            &[
                "artifacts",
                "ARTIFACTS",
            ],
        );

        if result.is_ok() {
            return Err(
                "case-aliased named outputs must be rejected".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_duplicate_named_directories() -> Result<(), String> {
        let root = std::env::temp_dir().join(
            format!(
                "pipeline-output-directory-duplicate-{}",
                std::process::id(),
            ),
        );
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.to_string()),
        }

        let result = FilesystemOutputInventory.summarize(
            &root,
            &[
                "artifacts",
                "artifacts",
            ],
        );

        if result.is_ok() {
            return Err("duplicate named outputs must be rejected".to_owned());
        }
        Ok(())
    }
}

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
//   - Unit evidence for algorithm application byte codecs.
// - Must-Not:
//   - Perform reconstruction I/O or depend on proprietary source.
// - Allows:
//   - Exercise private hexadecimal helpers through the source-owned test
//     module.
// - Split-When:
//   - Additional application codecs gain independent test contracts.
// - Merge-When:
//   - Another test module owns the identical codec evidence.
// - Summary:
//   - Algorithm application codec unit tests.
// - Description:
//   - Proves hexadecimal encoding and decoding preserve exact bytes.
// - Usage:
//   - Loaded by the algorithm engine module under cfg(test).
// - Defaults:
//   - Synthetic byte payloads only.
//

//! Algorithm application codec unit tests.

use crate::document::{
    ALGORITHM_SCHEMA, AlgorithmDocument, ProtectedTarget, SourceRecord,
    TargetDescriptor, TargetKind,
};
use crate::domain::Settings;

use super::{decode_hex, hex_bytes, settings_sha256, validate_document};

#[test]
fn hexadecimal_round_trip_is_exact() {
    let input = b"source-bound\0payload";
    let encoded = hex_bytes(input);
    assert_eq!(
        decode_hex(&encoded),
        Ok(input.to_vec()),
        "hex round trip must preserve bytes"
    );
}

fn settings() -> Result<Settings, String> {
    let text = r#"{
      "schema":"shar.algorithm.settings.v1",
      "minimum_source_files":1,
      "minimum_source_bytes":1024,
      "maximum_source_files":16,
      "maximum_target_files":16,
      "maximum_file_bytes":1048576,
      "maximum_source_bytes":4194304,
      "maximum_target_bytes":4194304
    }"#;
    Settings::from_json(text).map_err(|error| error.to_string())
}

fn protected_target(path: &str) -> ProtectedTarget {
    ProtectedTarget {
        descriptor: TargetDescriptor {
            path: path.to_owned(),
            bytes: 1,
            sha256: "0".repeat(64),
        },
        nonce: "00".repeat(12),
        ciphertext: "00".to_owned(),
    }
}

#[test]
fn directory_target_ancestor_collision_is_rejected() -> Result<(), String> {
    let settings = settings()?;
    let document = AlgorithmDocument {
        schema: ALGORITHM_SCHEMA.to_owned(),
        settings_sha256: settings_sha256(&settings)
            .map_err(|error| error.to_string())?,
        source: vec![SourceRecord {
            input: 0,
            path: String::new(),
            bytes: 1024,
            sha256: "0".repeat(64),
        }],
        target_kind: TargetKind::Directory,
        target: vec![protected_target("a"), protected_target("a/b")],
    };

    if validate_document(&document, &settings).is_ok() {
        return Err("ancestor target path collision was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn directory_target_portable_identity_collision_is_rejected(
) -> Result<(), String> {
    let settings = settings()?;
    let document = AlgorithmDocument {
        schema: ALGORITHM_SCHEMA.to_owned(),
        settings_sha256: settings_sha256(&settings)
            .map_err(|error| error.to_string())?,
        source: vec![SourceRecord {
            input: 0,
            path: String::new(),
            bytes: 1024,
            sha256: "0".repeat(64),
        }],
        target_kind: TargetKind::Directory,
        target: vec![
            protected_target("Folder/File.bin"),
            protected_target("folder/file.bin"),
        ],
    };

    if validate_document(&document, &settings).is_ok() {
        let message = "portable target identity collision was accepted";
        return Err(message.to_owned());
    }
    Ok(())
}

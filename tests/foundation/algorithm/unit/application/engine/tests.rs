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

use std::path::PathBuf;

use super::{
    InputFile, decode_hex, hex_bytes, settings_sha256,
    sort_files_by_logical_path, source_key, validate_document,
};
use crate::document::{
    ALGORITHM_SCHEMA, AlgorithmDocument, ProtectedTarget, SourceRecord,
    TargetDescriptor, TargetKind,
};
use crate::domain::Settings;

#[test]
fn logical_file_order_is_portable_wire_order() {
    let mut files = [
        InputFile {
            input: 0,
            logical_path: "😀.bin".to_owned(),
            path: PathBuf::from("unused-emoji"),
            bytes: 0,
            sha256: String::new(),
            data: Vec::new(),
            projection: None,
        },
        InputFile {
            input: 0,
            logical_path: "\u{e000}.bin".to_owned(),
            path: PathBuf::from("unused-private-use"),
            bytes: 0,
            sha256: String::new(),
            data: Vec::new(),
            projection: None,
        },
    ];

    sort_files_by_logical_path(&mut files);

    assert_eq!(files[0].logical_path, "\u{e000}.bin");
    assert_eq!(files[1].logical_path, "😀.bin");
}

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
        ciphertext: "00".repeat(17),
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
            sha256: Some("0".repeat(64)),
            projection: None,
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
fn directory_target_records_must_match_collector_order() -> Result<(), String> {
    let settings = settings()?;
    let document = AlgorithmDocument {
        schema: ALGORITHM_SCHEMA.to_owned(),
        settings_sha256: settings_sha256(&settings)
            .map_err(|error| error.to_string())?,
        source: vec![SourceRecord {
            input: 0,
            path: String::new(),
            bytes: 1024,
            sha256: Some("0".repeat(64)),
            projection: None,
        }],
        target_kind: TargetKind::Directory,
        target: vec![protected_target("z.bin"), protected_target("a.bin")],
    };

    if validate_document(&document, &settings).is_ok() {
        return Err("out-of-order target records were accepted".to_owned());
    }
    Ok(())
}

#[test]
fn directory_target_portable_identity_collision_is_rejected()
-> Result<(), String> {
    let settings = settings()?;
    let document = AlgorithmDocument {
        schema: ALGORITHM_SCHEMA.to_owned(),
        settings_sha256: settings_sha256(&settings)
            .map_err(|error| error.to_string())?,
        source: vec![SourceRecord {
            input: 0,
            path: String::new(),
            bytes: 1024,
            sha256: Some("0".repeat(64)),
            projection: None,
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
#[test]
fn hexadecimal_metadata_must_be_canonical_lowercase() -> Result<(), String> {
    let settings = settings()?;
    let source_hash = AlgorithmDocument {
        schema: ALGORITHM_SCHEMA.to_owned(),
        settings_sha256: settings_sha256(&settings)
            .map_err(|error| error.to_string())?,
        source: vec![SourceRecord {
            input: 0,
            path: String::new(),
            bytes: 1024,
            sha256: Some("A".repeat(64)),
            projection: None,
        }],
        target_kind: TargetKind::Directory,
        target: vec![protected_target("asset.bin")],
    };
    if validate_document(&source_hash, &settings).is_ok() {
        return Err("uppercase source hash was accepted".to_owned());
    }

    let mut protected = protected_target("asset.bin");
    protected.nonce = "AA".repeat(12);
    let protected_nonce = AlgorithmDocument {
        schema: ALGORITHM_SCHEMA.to_owned(),
        settings_sha256: settings_sha256(&settings)
            .map_err(|error| error.to_string())?,
        source: vec![SourceRecord {
            input: 0,
            path: String::new(),
            bytes: 1024,
            sha256: Some("0".repeat(64)),
            projection: None,
        }],
        target_kind: TargetKind::Directory,
        target: vec![protected],
    };
    if validate_document(&protected_nonce, &settings).is_ok() {
        return Err("uppercase protected nonce was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn source_metadata_must_match_collector_contract() -> Result<(), String> {
    let settings = settings()?;
    let malformed = [
        vec![SourceRecord {
            input: 1,
            path: String::new(),
            bytes: 1024,
            sha256: Some("0".repeat(64)),
            projection: None,
        }],
        vec![SourceRecord {
            input: 0,
            path: "../private.bin".to_owned(),
            bytes: 1024,
            sha256: Some("0".repeat(64)),
            projection: None,
        }],
        vec![SourceRecord {
            input: 0,
            path: "folder//asset.bin".to_owned(),
            bytes: 1024,
            sha256: Some("0".repeat(64)),
            projection: None,
        }],
        vec![SourceRecord {
            input: 0,
            path: "folder/".to_owned(),
            bytes: 1024,
            sha256: Some("0".repeat(64)),
            projection: None,
        }],
        vec![SourceRecord {
            input: 0,
            path: String::new(),
            bytes: 1,
            sha256: Some("0".repeat(64)),
            projection: None,
        }],
        vec![
            SourceRecord {
                input: 0,
                path: String::new(),
                bytes: 1024,
                sha256: Some("0".repeat(64)),
                projection: None,
            },
            SourceRecord {
                input: 0,
                path: "asset.bin".to_owned(),
                bytes: 1024,
                sha256: Some("0".repeat(64)),
                projection: None,
            },
        ],
        vec![
            SourceRecord {
                input: 0,
                path: "z.bin".to_owned(),
                bytes: 512,
                sha256: Some("0".repeat(64)),
                projection: None,
            },
            SourceRecord {
                input: 0,
                path: "a.bin".to_owned(),
                bytes: 512,
                sha256: Some("0".repeat(64)),
                projection: None,
            },
        ],
        vec![
            SourceRecord {
                input: 0,
                path: "A.bin".to_owned(),
                bytes: 512,
                sha256: Some("0".repeat(64)),
                projection: None,
            },
            SourceRecord {
                input: 0,
                path: "a.bin".to_owned(),
                bytes: 512,
                sha256: Some("0".repeat(64)),
                projection: None,
            },
        ],
        vec![
            SourceRecord {
                input: 0,
                path: "a".to_owned(),
                bytes: 512,
                sha256: Some("0".repeat(64)),
                projection: None,
            },
            SourceRecord {
                input: 0,
                path: "a/b".to_owned(),
                bytes: 512,
                sha256: Some("0".repeat(64)),
                projection: None,
            },
        ],
    ];
    for source in malformed {
        let document = AlgorithmDocument {
            schema: ALGORITHM_SCHEMA.to_owned(),
            settings_sha256: settings_sha256(&settings)
                .map_err(|error| error.to_string())?,
            source,
            target_kind: TargetKind::Directory,
            target: vec![protected_target("asset.bin")],
        };
        if validate_document(&document, &settings).is_ok() {
            return Err("malformed source metadata was accepted".to_owned());
        }
    }
    Ok(())
}

#[test]
fn protected_target_length_must_match_declared_bytes() -> Result<(), String> {
    let settings = settings()?;
    let document = AlgorithmDocument {
        schema: ALGORITHM_SCHEMA.to_owned(),
        settings_sha256: settings_sha256(&settings)
            .map_err(|error| error.to_string())?,
        source: vec![SourceRecord {
            input: 0,
            path: String::new(),
            bytes: 1024,
            sha256: Some("0".repeat(64)),
            projection: None,
        }],
        target_kind: TargetKind::Directory,
        target: vec![{
            let mut target = protected_target("asset.bin");
            target.ciphertext = "00".to_owned();
            target
        }],
    };

    if validate_document(&document, &settings).is_ok() {
        let message = "mismatched protected target length was accepted";
        return Err(message.to_owned());
    }
    Ok(())
}

#[test]
fn source_key_uses_captured_bytes_without_reopening_path() -> Result<(), String>
{
    let file = InputFile {
        input: 0,
        logical_path: String::new(),
        path: std::env::temp_dir().join("shar-algorithm-missing-source.bin"),
        bytes: 4,
        sha256: "unused-after-capture".to_owned(),
        data: b"seed".to_vec(),
        projection: None,
    };

    source_key(&[file])
        .map(|_key| ())
        .map_err(|error| error.to_string())
}

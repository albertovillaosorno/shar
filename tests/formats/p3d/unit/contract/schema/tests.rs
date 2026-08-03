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

use super::{
    CHUNK_ID_CONSTANT_COUNT, CHUNK_ID_CONSTANTS, SCHEMA_CHUNK_COUNT,
    SCHEMA_FILE_COUNT, chunk_constants_by_value, schema_by_chunk_name,
    schema_ref_for_kind, schemas_by_chunk_name,
};

#[test]
fn chunk_constants_have_complete_identity_metadata() {
    for constant in CHUNK_ID_CONSTANTS {
        assert!(!constant.authority_key.is_empty());
        assert!(!constant.scope.is_empty());
        assert!(!constant.name.is_empty());
    }
}

#[test]
fn singular_schema_lookup_rejects_ambiguous_names() {
    assert!(schema_by_chunk_name("tlCompositeSkinProp").is_none());
    assert_eq!(schemas_by_chunk_name("tlCompositeSkinProp").count(), 2);
}

#[test]
fn registry_covers_all_schema16_files() {
    assert_eq!(SCHEMA_FILE_COUNT, 88);
    assert_eq!(SCHEMA_CHUNK_COUNT, 293);
    const { assert!(CHUNK_ID_CONSTANT_COUNT > 200) };
    assert!(schema_by_chunk_name("texture").is_some());
    assert!(schema_by_chunk_name("mesh").is_some());
    assert!(schema_by_chunk_name("fence_dsg").is_some());
    assert_eq!(schema_ref_for_kind("mesh"), Some("mesh"));
    assert!(
        chunk_constants_by_value(0x03f0_0007)
            .any(|constant| constant.name.contains("FENCE"))
    );
}

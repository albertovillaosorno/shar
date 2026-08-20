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

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::{SharedTextureAuthority, TextureSource};

#[test]
fn same_level_terrain_mesh_is_preferred() {
    let authority = SharedTextureAuthority {
        sources: BTreeMap::from([("tree.bmp".to_owned(), vec![
            TextureSource {
                subcategory: "terrain-world/level-01/\
                                          terrain-mesh"
                    .to_owned(),
                path: PathBuf::from("level-one.png"),
                sha256: "one".to_owned(),
            },
            TextureSource {
                subcategory: "terrain-world/level-05/\
                                          terrain-mesh"
                    .to_owned(),
                path: PathBuf::from("level-five.png"),
                sha256: "five".to_owned(),
            },
        ])]),
    };

    let result =
        authority.resolve("tree.bmp", "terrain-world/level-01/regions/l1r1");

    assert_eq!(result, Ok(Some(std::path::Path::new("level-one.png"))));
}

#[test]
fn conflicting_same_level_payloads_are_rejected() {
    let authority = SharedTextureAuthority {
        sources: BTreeMap::from([("tree.bmp".to_owned(), vec![
            TextureSource {
                subcategory: "terrain-world/level-01/regions/a".to_owned(),
                path: PathBuf::from("a.png"),
                sha256: "a".to_owned(),
            },
            TextureSource {
                subcategory: "terrain-world/level-01/regions/b".to_owned(),
                path: PathBuf::from("b.png"),
                sha256: "b".to_owned(),
            },
        ])]),
    };

    assert!(
        authority
            .resolve("tree.bmp", "terrain-world/level-01/regions/c",)
            .is_err()
    );
}

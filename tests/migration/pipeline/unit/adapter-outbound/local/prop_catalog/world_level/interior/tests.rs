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

use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};

use super::{geometry_key, identity_for_package, is_halloween_package};

#[test]
fn all_nineteen_source_packages_have_stable_interior_identities() {
    let packages = [
        "extracted-art-l1i00",
        "extracted-art-l1i01",
        "extracted-art-l1i02",
        "extracted-art-l2i03",
        "extracted-art-l2i04",
        "extracted-art-l3i05",
        "extracted-art-l3i06",
        "extracted-art-l4i00",
        "extracted-art-l4i01",
        "extracted-art-l4i02",
        "extracted-art-l4i07",
        "extracted-art-l5i03",
        "extracted-art-l5i04",
        "extracted-art-l6i05",
        "extracted-art-l6i06",
        "extracted-art-l7i00",
        "extracted-art-l7i01",
        "extracted-art-l7i02",
        "extracted-art-l7i07",
    ];
    assert_eq!(packages.len(), 19);
    for package in packages {
        assert!(identity_for_package(package).is_some(), "{package}");
    }
}

#[test]
fn only_level_seven_halloween_packages_are_overlays() {
    assert!(is_halloween_package("extracted-art-l7i00"));
    assert!(is_halloween_package("extracted-art-l7i01"));
    assert!(is_halloween_package("extracted-art-l7i02"));
    assert!(is_halloween_package("extracted-art-l7i07"));
    assert!(!is_halloween_package("extracted-art-l4i07"));
    assert!(!is_halloween_package("extracted-art-l6i06"));
}

#[test]
fn geometry_key_ignores_vertex_order_but_preserves_position()
-> Result<(), String> {
    let first_group = PrimitiveGroup::new(
        0,
        "material",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("first group failed: {error:?}"))?;
    let reordered_group = PrimitiveGroup::new(
        0,
        "other-material",
        vec![[0., 1., 0.], [0., 0., 0.], [1., 0., 0.]],
        Vec::new(),
        &[1, 2, 0],
    )
    .map_err(|error| format!("reordered group failed: {error:?}"))?;
    let shifted_group = PrimitiveGroup::new(
        0,
        "material",
        vec![[0.002, 0., 0.], [1.002, 0., 0.], [0.002, 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("shifted group failed: {error:?}"))?;
    let first = MeshAsset::new("first", vec![first_group])
        .map_err(|error| format!("first mesh failed: {error:?}"))?;
    let reordered = MeshAsset::new("reordered", vec![reordered_group])
        .map_err(|error| format!("reordered mesh failed: {error:?}"))?;
    let shifted = MeshAsset::new("shifted", vec![shifted_group])
        .map_err(|error| format!("shifted mesh failed: {error:?}"))?;
    let first_key = geometry_key(&first).map_err(|error| error.to_string())?;
    let reordered_key =
        geometry_key(&reordered).map_err(|error| error.to_string())?;
    let shifted_key =
        geometry_key(&shifted).map_err(|error| error.to_string())?;
    if first_key != reordered_key {
        return Err(String::from("vertex ordering changed the geometry key"));
    }
    if first_key == shifted_key {
        return Err(String::from(
            "world placement did not change the geometry key",
        ));
    }
    Ok(())
}

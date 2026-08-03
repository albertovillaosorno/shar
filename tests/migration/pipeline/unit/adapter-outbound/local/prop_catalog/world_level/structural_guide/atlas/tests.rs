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

use super::*;

#[test]
fn source_average_combines_different_presentations() {
    let mut aggregate = TintAggregate::default();
    assert!(aggregate.add([255, 0, 0, 255], 1, false).is_ok());
    assert!(aggregate.add([0, 0, 255, 255], 1, false).is_ok());
    assert_eq!(
        aggregate.finish(),
        Ok(VertexColorBake::SourceAverage {
            rgba8: [128, 0, 128, 255],
            sample_count: 2,
        })
    );
}

#[test]
fn one_exact_source_presentation_remains_uniform() {
    let mut aggregate = TintAggregate::default();
    assert!(aggregate.add([64, 96, 128, 255], 3, false).is_ok());
    assert_eq!(
        aggregate.finish(),
        Ok(VertexColorBake::Uniform {
            rgba8: [64, 96, 128, 255],
            sample_count: 3,
        })
    );
}

#[test]
fn per_polygon_wrap_is_preserved_without_duplicate_tiles() {
    let surfaces = vec![
        SurfaceKey {
            material: "repeat".to_owned(),
            repeat: true,
        },
        SurfaceKey {
            material: "clamp".to_owned(),
            repeat: false,
        },
    ];
    assert_eq!(wrap_mode(&surfaces), "perPolygon");
}

#[test]
fn material_and_vertex_tints_are_combined_before_source_average() {
    assert_eq!(combine_tints([128, 255, 64, 255], [128, 64, 255, 128]), [
        64, 64, 64, 128
    ]);
}

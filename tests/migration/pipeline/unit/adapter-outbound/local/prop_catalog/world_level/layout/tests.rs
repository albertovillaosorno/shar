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

use std::collections::BTreeMap;

use super::{MapBounds, placement_for_scope, validate_group_bounds};

#[test]
fn recurring_levels_share_groups_without_artificial_offsets()
-> Result<(), String> {
    for (levels, group) in [
        (&["level-01", "level-04", "level-07"][..], "map-01-04-07"),
        (&["level-02", "level-05"][..], "map-02-05"),
        (&["level-03", "level-06"][..], "map-03-06"),
    ] {
        for level in levels {
            let placement = placement_for_scope(level)
                .map_err(|error| error.to_string())?;
            if placement.group != Some(group) || placement.offset != [0, 0, 0] {
                return Err(format!("invalid zone grouping for {level}"));
            }
        }
    }
    Ok(())
}

#[test]
fn connected_overlap_is_allowed_but_invalid_bounds_fail() -> Result<(), String>
{
    let mut bounds = BTreeMap::new();
    let _ = bounds.insert("map-01-04-07", MapBounds {
        low: [0., 0., 0.],
        high: [10., 10., 10.],
    });
    let _ = bounds.insert("map-02-05", MapBounds {
        low: [9., 0., 0.],
        high: [20., 10., 10.],
    });
    validate_group_bounds(&bounds).map_err(|error| error.to_string())?;
    let _ = bounds.insert("map-03-06", MapBounds {
        low: [5., 0., 0.],
        high: [4., 10., 10.],
    });
    if validate_group_bounds(&bounds).is_ok() {
        return Err(String::from("inverted bounds were accepted"));
    }
    Ok(())
}

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
//   - Source-preserving world layout tests.
// - Must-Not:
//   - Transform geometry or broaden production APIs.
// - Allows:
//   - Pure family-grouping and bounds-validation assertions.
// - Split-When:
//   - Split when another grouping fixture gains independent ownership.
// - Merge-When:
//   - Merge when another module owns identical layout evidence.
// - Summary:
//   - Source-preserving world layout tests.
// - Description:
//   - Proves recurring levels share metadata without artificial offsets.
// - Usage:
//   - Included only by the owning layout module under cfg(test).
// - Defaults:
//   - Unknown scopes and malformed bounds fail explicitly.
//

//! Source-preserving world layout tests.

use std::collections::BTreeMap;

use super::{MapBounds, placement_for_scope, validate_group_bounds};

#[test]
fn recurring_levels_share_groups_without_offsets() -> Result<(), String> {
    for (levels, group) in [
        (&["level-01", "level-04", "level-07"][..], "map-01-04-07"),
        (&["level-02", "level-05"][..], "map-02-05"),
        (&["level-03", "level-06"][..], "map-03-06"),
    ] {
        for level in levels {
            let placement = placement_for_scope(level)
                .map_err(|error| error.to_string())?;
            if placement.group != group {
                return Err(format!("invalid narrative family for {level}"));
            }
        }
    }
    Ok(())
}

#[test]
fn source_overlap_is_allowed_but_invalid_bounds_fail() -> Result<(), String> {
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
        return Err("inverted bounds were accepted".to_owned());
    }
    Ok(())
}

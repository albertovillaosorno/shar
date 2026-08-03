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
//   - Raw os error application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Raw os error application service.
// - Description:
//   - Implements the declared responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Raw os error application service.

use std::error::Error;
use std::io;

/// Maximum provider source links inspected for raw OS evidence.
const MAX_SOURCE_DEPTH: usize = 16;

/// Returns whether two trait objects identify the same error value.
fn same_error_value(
    left: &(dyn Error + 'static),
    right: &(dyn Error + 'static),
) -> bool {
    std::ptr::eq(left, right)
}

/// Maximum distinct errors retained for one bounded traversal.
const MAX_TRACKED_ERRORS: usize = MAX_SOURCE_DEPTH + 1;

/// Fixed source identities retained without heap allocation.
struct SourceHistory {
    /// Canonical trait-object pointers in visitation order.
    errors: [Option<*const (dyn Error + 'static)>; MAX_TRACKED_ERRORS],
}

impl SourceHistory {
    /// Creates one empty fixed-capacity history.
    const fn new() -> Self {
        Self {
            errors: [None; MAX_TRACKED_ERRORS],
        }
    }

    /// Records one unseen identity and rejects repeats or exhausted capacity.
    fn record(&mut self, candidate: &(dyn Error + 'static)) -> bool {
        let candidate_pointer = std::ptr::from_ref(candidate);
        for slot in &mut self.errors {
            if let Some(error) = slot {
                if std::ptr::eq(*error, candidate_pointer) {
                    return false;
                }
                continue;
            }
            *slot = Some(candidate_pointer);
            return true;
        }
        false
    }
}

/// Finds one raw operating-system code in a bounded provider source chain.
pub(super) fn find(source: &(dyn Error + 'static)) -> Option<i32> {
    let mut current = Some(source);
    let mut remaining_depth = MAX_SOURCE_DEPTH;
    let mut visited = SourceHistory::new();
    while let Some(error) = current {
        if !visited.record(error) {
            return None;
        }
        if let Some(io_error) = error.downcast_ref::<io::Error>()
            && let Some(raw_os_error) = io_error.raw_os_error()
        {
            return Some(raw_os_error);
        }
        if remaining_depth == 0 {
            return None;
        }
        remaining_depth = remaining_depth.saturating_sub(1);
        let next = error.source();
        if let Some(next_error) = next
            && same_error_value(error, next_error)
        {
            return None;
        }
        current = next;
    }
    None
}

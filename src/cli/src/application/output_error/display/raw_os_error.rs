// File:
//   - raw_os_error.rs
// Path:
//   - src/cli/src/application/output_error/display/raw_os_error.rs
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
//   - Bounded discovery of raw operating-system codes in error chains.
// - Must-Not:
//   - Render diagnostics or mutate provider errors.
// - Allows:
//   - Inspect typed source links for nested std::io::Error values.
// - Split-When:
//   - Another independent source-chain datum needs traversal.
// - Merge-When:
//   - Output diagnostics no longer expose raw operating-system codes.
// - Summary:
//   - Raw operating-system error discovery.
// - Description:
//   - Finds one nested raw code without permitting unbounded cyclic traversal.
// - Usage:
//   - Used privately by output-error display rendering.
// - Defaults:
//   - At most sixteen source links are inspected.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Bounded raw operating-system error discovery.
//!
//! Arbitrary provider errors may expose cyclic or excessively deep chains.

use std::error::Error;
use std::io;

/// Maximum provider source links inspected for raw OS evidence.
const MAX_SOURCE_DEPTH: usize = 16;

/// Returns whether two trait objects identify the same error value.
fn same_error_value(
    left: &(dyn Error + 'static),
    right: &(dyn Error + 'static),
) -> bool {
    std::ptr::eq(
        left, right,
    )
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
    fn record(
        &mut self,
        candidate: &(dyn Error + 'static),
    ) -> bool {
        let candidate_pointer = std::ptr::from_ref(candidate);
        for slot in &mut self.errors {
            if let Some(error) = slot {
                if std::ptr::eq(
                    *error,
                    candidate_pointer,
                ) {
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
            && same_error_value(
                error, next_error,
            )
        {
            return None;
        }
        current = next;
    }
    None
}

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
//   - Render outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Render outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Render outbound adapter.

use std::time::Duration;

/// Tenths-of-a-percent scale for `100.0%` rendering.
const PERCENT_TENTHS_SCALE: u128 = 1_000;
/// Base used to split tenths into whole and fractional percentage digits.
const PERCENT_DECIMAL_BASE: u128 = 10;
/// Seconds in one minute.
const SECONDS_PER_MINUTE: u64 = 60;
/// Seconds in one hour.
const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;
/// Maximum current-item characters rendered on one terminal line.
const MAX_ITEM_CHARACTERS: usize = 96;

/// Render one detailed progress line.
pub(super) fn progress_line(
    stage: &str,
    done: usize,
    total: Option<usize>,
    elapsed: Duration,
    item: &str,
) -> String {
    total.map_or_else(
        || {
            format!(
                "[{stage}] {done} items elapsed {}{}",
                format_duration(elapsed.as_secs()),
                current_item_suffix(item),
            )
        },
        |item_total| known_total_line(stage, done, item_total, elapsed, item),
    )
}

/// Render progress when an exact total is available.
fn known_total_line(
    stage: &str,
    done: usize,
    item_total: usize,
    elapsed: Duration,
    item: &str,
) -> String {
    let (percent_whole, percent_fraction) = percentage_parts(done, item_total);
    let current = current_item_suffix(item);
    eta_duration(elapsed, done, item_total).map_or_else(
        || {
            format!(
                concat!("[{}] {}.{}% ({}/{}) elapsed {}{}"),
                stage,
                percent_whole,
                percent_fraction,
                done,
                item_total,
                format_duration(elapsed.as_secs()),
                current,
            )
        },
        |eta| {
            format!(
                concat!("[{}] {}.{}% ({}/{}) elapsed {} eta {}{}"),
                stage,
                percent_whole,
                percent_fraction,
                done,
                item_total,
                format_duration(elapsed.as_secs()),
                format_duration(eta.as_secs()),
                current,
            )
        },
    )
}

/// Return whole and fractional tenths for a bounded percentage.
fn percentage_parts(done: usize, total: usize) -> (u128, u128) {
    if total == 0 {
        return (100, 0);
    }
    let completed = u128::try_from(done.min(total)).unwrap_or(u128::MAX);
    let available = u128::try_from(total).unwrap_or(u128::MAX);
    let tenths = completed
        .saturating_mul(PERCENT_TENTHS_SCALE)
        .checked_div(available)
        .unwrap_or_default();
    (
        tenths.div_euclid(PERCENT_DECIMAL_BASE),
        tenths.rem_euclid(PERCENT_DECIMAL_BASE),
    )
}

/// Estimate remaining duration from the observed item rate.
fn eta_duration(
    elapsed: Duration,
    done: usize,
    total: usize,
) -> Option<Duration> {
    if done == 0 || total <= done {
        return None;
    }
    let completed = u128::try_from(done).unwrap_or(u128::MAX);
    let remaining =
        u128::try_from(total.saturating_sub(done)).unwrap_or(u128::MAX);
    let remaining_millis = elapsed
        .as_millis()
        .saturating_mul(remaining)
        .checked_div(completed)?;
    Some(Duration::from_millis(
        u64::try_from(remaining_millis).unwrap_or(u64::MAX),
    ))
}

/// Format whole seconds as stable `HH:MM:SS` text.
pub(super) fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds.div_euclid(SECONDS_PER_HOUR);
    let remaining_seconds = total_seconds.rem_euclid(SECONDS_PER_HOUR);
    let minutes = remaining_seconds.div_euclid(SECONDS_PER_MINUTE);
    let seconds = remaining_seconds.rem_euclid(SECONDS_PER_MINUTE);
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

/// Bound one current-item label without assuming ASCII input.
pub(super) fn shorten_item(item: &str) -> String {
    let mut output = String::with_capacity(item.len());
    let mut remaining = MAX_ITEM_CHARACTERS;
    for character in item.chars() {
        if character.is_control() {
            let escaped = character.escape_unicode();
            let escaped_length = escaped.len();
            if escaped_length > remaining {
                output.push_str("...");
                return output;
            }
            output.extend(escaped);
            remaining = remaining.saturating_sub(escaped_length);
        } else if remaining == 0 {
            output.push_str("...");
            return output;
        } else {
            output.push(character);
            remaining = remaining.saturating_sub(1);
        }
    }
    output
}

/// Render the optional current item suffix.
fn current_item_suffix(item: &str) -> String {
    if item.is_empty() {
        String::new()
    } else {
        format!(" current={item}")
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/progress/render/tests.rs"]
mod tests;

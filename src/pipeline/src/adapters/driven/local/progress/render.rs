// File:
//   - render.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/progress/render.rs
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
//   - Pure terminal progress text, elapsed-time, and ETA calculations.
// - Must-Not:
//   - Read clocks, write streams, access files, or mutate extraction state.
// - Allows:
//   - Deterministic formatting from explicit counters and elapsed durations.
// - Split-When:
//   - Split when another independently reusable presentation gains policy.
// - Merge-When:
//   - The progress facade can own pure rendering without violating SRP.
// - Summary:
//   - Renders stable pipeline progress percentages, elapsed time, and ETA.
// - Description:
//   - Converts explicit progress values into concise terminal text.
// - Usage:
//   - Called by the stage-progress lifecycle after throttling decisions.
// - Defaults:
//   - Unknown totals show counts and elapsed time without a fabricated ETA.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Renders stable progress percentages, elapsed time, ETA, and current items.
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
        |item_total| {
            known_total_line(
                stage, done, item_total, elapsed, item,
            )
        },
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
    let (percent_whole, percent_fraction) = percentage_parts(
        done, item_total,
    );
    let current = current_item_suffix(item);
    eta_duration(
        elapsed, done, item_total,
    )
    .map_or_else(
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
fn percentage_parts(
    done: usize,
    total: usize,
) -> (
    u128,
    u128,
) {
    if total == 0 {
        return (
            100, 0,
        );
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
    Some(
        Duration::from_millis(
            u64::try_from(remaining_millis).unwrap_or(u64::MAX),
        ),
    )
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
mod tests {
    use std::time::Duration;

    use super::{eta_duration, format_duration, progress_line, shorten_item};

    #[test]
    fn duration_uses_stable_clock_text() {
        assert_eq!(
            format_duration(0),
            "00:00:00"
        );
        assert_eq!(
            format_duration(61),
            "00:01:01"
        );
        assert_eq!(
            format_duration(3_661),
            "01:01:01"
        );
    }

    #[test]
    fn eta_uses_observed_item_rate() {
        assert_eq!(
            eta_duration(
                Duration::from_secs(10),
                50,
                100,
            ),
            Some(Duration::from_secs(10))
        );
        assert_eq!(
            eta_duration(
                Duration::from_secs(10),
                0,
                100,
            ),
            None
        );
        assert_eq!(
            eta_duration(
                Duration::from_secs(10),
                100,
                100,
            ),
            None
        );
    }

    #[test]
    fn detailed_line_includes_percent_eta_and_current_item() {
        let line = progress_line(
            "p3d",
            50,
            Some(100),
            Duration::from_secs(10),
            "art/sample.p3d",
        );
        assert_eq!(
            line,
            concat!(
                "[p3d] 50.0% (50/100) elapsed 00:00:10 ",
                "eta 00:00:10 current=art/sample.p3d"
            )
        );
    }

    #[test]
    fn unknown_total_does_not_fabricate_percent_or_eta() {
        assert_eq!(
            progress_line(
                "rcf",
                4,
                None,
                Duration::from_secs(2),
                "entry.bin",
            ),
            "[rcf] 4 items elapsed 00:00:02 current=entry.bin"
        );
    }

    #[test]
    fn current_item_is_unicode_safe_and_bounded() {
        let input = "é".repeat(120);
        let output = shorten_item(&input);
        assert!(output.ends_with("..."));
        assert_eq!(
            output
                .chars()
                .count(),
            99
        );
    }

    #[test]
    fn current_item_escapes_terminal_control_characters() {
        assert_eq!(
            shorten_item("entry\nnext\u{1b}[31m"),
            "entry\\u{a}next\\u{1b}[31m"
        );
    }

    #[test]
    fn current_item_does_not_split_control_escapes() {
        let mut input = "a".repeat(95);
        input.push(char::from(10));
        let mut expected = "a".repeat(95);
        expected.push_str("...");
        assert_eq!(
            shorten_item(&input),
            expected
        );
    }
}

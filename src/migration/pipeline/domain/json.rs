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
//   - Json domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Json domain module.
// - Description:
//   - Implements the declared domain module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Json domain module.

/// Escape one string for inclusion in a JSON string literal.
#[must_use]
pub(super) fn escape(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' | '\\' => {
                output.push('\\');
                output.push(character);
            },
            '\u{8}' => output.push_str("\\b"),
            '\u{c}' => output.push_str("\\f"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            control if control <= '\u{1f}' => {
                push_control(&mut output, control);
            },
            other => output.push(other),
        }
    }
    output
}

/// Append one JSON Unicode escape for an unhandled C0 control.
fn push_control(output: &mut String, control: char) {
    let code = u32::from(control);
    output.push_str("\\u00");
    output.push(hex_digit(code >> 4));
    output.push(hex_digit(code & 0x0f));
}

/// Convert one four-bit value to its lowercase hexadecimal digit.
const fn hex_digit(value: u32) -> char {
    match value {
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => '0',
    }
}

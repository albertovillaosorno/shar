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
//   - Sha256 domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Sha256 domain module.
// - Description:
//   - Implements the declared domain module responsibility for sha256.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Sha256 domain module.

const INITIAL: [u32; 8] = [
    0x6a09_e667,
    0xbb67_ae85,
    0x3c6e_f372,
    0xa54f_f53a,
    0x510e_527f,
    0x9b05_688c,
    0x1f83_d9ab,
    0x5be0_cd19,
];

/// Standard SHA-256 round constants.
const ROUND_CONSTANTS: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];

/// Incremental dependency-free SHA-256 state.
#[derive(Debug, Clone, Copy)]
pub struct Sha256 {
    state: [u32; 8],
    buffer: [u8; 64],
    buffer_len: usize,
    length_bytes: u64,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha256 {
    /// Create one empty SHA-256 state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            state: INITIAL,
            buffer: [0; 64],
            buffer_len: 0,
            length_bytes: 0,
        }
    }

    /// Append exact bytes to this digest state.
    pub fn update(&mut self, data: &[u8]) {
        self.length_bytes = self
            .length_bytes
            .saturating_add(u64::try_from(data.len()).unwrap_or(u64::MAX));
        let mut remaining = data;
        if self.buffer_len != 0 {
            let available = 64_usize.saturating_sub(self.buffer_len);
            let take = available.min(remaining.len());
            let end = self.buffer_len.saturating_add(take);
            if let (Some(target), Some(source)) = (
                self.buffer.get_mut(self.buffer_len..end),
                remaining.get(..take),
            ) {
                target.copy_from_slice(source);
            }
            self.buffer_len = end;
            remaining = remaining.get(take..).unwrap_or_default();
            if self.buffer_len < 64 {
                return;
            }
            compress(&mut self.state, &self.buffer);
            self.buffer_len = 0;
        }
        let (chunks, tail) = remaining.as_chunks::<64>();
        for block in chunks {
            compress(&mut self.state, block);
        }
        if let Some(target) = self.buffer.get_mut(..tail.len()) {
            target.copy_from_slice(tail);
            self.buffer_len = tail.len();
        }
    }

    /// Finalize this state into the standard 32-byte SHA-256 digest.
    #[must_use]
    pub fn finalize(mut self) -> [u8; 32] {
        let bit_length = self.length_bytes.saturating_mul(8);
        if let Some(marker) = self.buffer.get_mut(self.buffer_len) {
            *marker = 0x80;
        }
        self.buffer_len = self.buffer_len.saturating_add(1);
        if self.buffer_len > 56 {
            if let Some(padding) = self.buffer.get_mut(self.buffer_len..) {
                padding.fill(0);
            }
            compress(&mut self.state, &self.buffer);
            self.buffer.fill(0);
            self.buffer_len = 0;
        }
        if let Some(padding) = self.buffer.get_mut(self.buffer_len..56) {
            padding.fill(0);
        }
        if let Some(length) = self.buffer.get_mut(56..64) {
            length.copy_from_slice(&bit_length.to_be_bytes());
        }
        compress(&mut self.state, &self.buffer);
        let mut output = [0u8; 32];
        for (chunk, value) in output.chunks_mut(4).zip(self.state) {
            chunk.copy_from_slice(&value.to_be_bytes());
        }
        output
    }

    /// Finalize this state as 64 lowercase hexadecimal characters.
    #[must_use]
    pub fn finalize_hex(self) -> String {
        hex(self.finalize())
    }
}

/// Hash exact bytes into the standard 32-byte SHA-256 digest.
#[must_use]
pub fn digest(data: &[u8]) -> [u8; 32] {
    let mut state = Sha256::new();
    state.update(data);
    state.finalize()
}

/// Render one digest as 64 lowercase hexadecimal characters.
#[must_use]
pub fn hex(digest: [u8; 32]) -> String {
    let mut output = String::with_capacity(64);
    for byte in digest {
        use core::fmt::Write as _;
        if write!(output, "{byte:02x}").is_err() {
            return output;
        }
    }
    output
}

/// Hash exact bytes and render the lowercase hexadecimal digest.
#[must_use]
pub fn digest_hex(data: &[u8]) -> String {
    hex(digest(data))
}

/// Apply one standard SHA-256 compression block.
#[expect(
    clippy::many_single_char_names,
    clippy::min_ident_chars,
    clippy::indexing_slicing,
    clippy::missing_asserts_for_indexing,
    clippy::arithmetic_side_effects,
    reason = "SHA-256 compression uses fixed-size schedule and state arrays \
              with standard algorithm variable names."
)]
fn compress(state: &mut [u32; 8], block: &[u8]) {
    let mut words = [0u32; 64];
    for (word, chunk) in words.iter_mut().take(16).zip(block.chunks(4)) {
        *word = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
    }
    for index in 16..64 {
        let first = words[index - 15].rotate_right(7)
            ^ words[index - 15].rotate_right(18)
            ^ (words[index - 15] >> 3u32);
        let second = words[index - 2].rotate_right(17)
            ^ words[index - 2].rotate_right(19)
            ^ (words[index - 2] >> 10u32);
        words[index] = words[index - 16]
            .wrapping_add(first)
            .wrapping_add(words[index - 7])
            .wrapping_add(second);
    }
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;
    for index in 0..64 {
        let upper_sigma =
            e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let choice = (e & f) ^ ((!e) & g);
        let first = h
            .wrapping_add(upper_sigma)
            .wrapping_add(choice)
            .wrapping_add(ROUND_CONSTANTS[index])
            .wrapping_add(words[index]);
        let lower_sigma =
            a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let majority = (a & b) ^ (a & c) ^ (b & c);
        let second = lower_sigma.wrapping_add(majority);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(first);
        d = c;
        c = b;
        b = a;
        a = first.wrapping_add(second);
    }
    for (target, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
        *target = target.wrapping_add(value);
    }
}

#[cfg(test)]
#[path = "../../../../tests/foundation/sha256/unit/domain/sha256/tests.rs"]
mod tests;

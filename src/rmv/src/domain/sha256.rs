// File:
//   - sha256.rs
// Path:
//   - src/rmv/src/domain/sha256.rs
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
//   - Pure rmv domain rules for domain sha256.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when sha256 contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Small SHA-256 implementation used to deduplicate movie inputs without a.
// - Description:
//   - Defines sha256 data and behavior for rmv domain.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Small SHA-256 implementation used to deduplicate movie inputs without a
//! network dependency.
/// Initial.
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
/// K.
const K: [u32; 64] = [
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Sha256.
pub struct Sha256(pub [u8; 32]);

impl Sha256 {
    #[must_use]
    /// Digest.
    pub fn digest(data: &[u8]) -> Self {
        let mut state = INITIAL;
        let bit_len = u64::try_from(data.len())
            .unwrap_or(u64::MAX)
            .saturating_mul(8);
        let mut padded = Vec::with_capacity(
            data.len()
                .saturating_add(72),
        );
        padded.extend_from_slice(data);
        padded.push(0x80_u8);
        while (padded.len() % 64_usize) != 56_usize {
            padded.push(0);
        }
        padded.extend_from_slice(&bit_len.to_be_bytes());
        for block in padded.chunks(64) {
            compress(
                &mut state, block,
            );
        }
        let mut out = [0_u8; 32];
        for (chunk, value) in out
            .chunks_mut(4)
            .zip(state)
        {
            chunk.copy_from_slice(&value.to_be_bytes());
        }
        Self(out)
    }

    #[must_use]
    /// Hex.
    pub fn hex(self) -> String {
        let mut out = String::with_capacity(64);
        for byte in self.0 {
            use core::fmt::Write as _;
            if write!(
                out,
                "{byte:02x}"
            )
            .is_err()
            {
                return out;
            }
        }
        out
    }
}

/// Compress.
// This scoped expectation preserves a documented boundary with explicit
// validation.
#[expect(
    clippy::many_single_char_names,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    reason = "SHA-256 compression uses fixed-size schedule/state arrays and \
              standard variable names from the algorithm."
)]
fn compress(
    state: &mut [u32; 8],
    block: &[u8],
) {
    let mut words = [0_u32; 64];
    for (word, chunk) in words
        .iter_mut()
        .take(16)
        .zip(block.chunks(4))
    {
        *word = u32::from_be_bytes(
            [
                chunk[0], chunk[1], chunk[2], chunk[3],
            ],
        );
    }
    for index in 16..64 {
        let s0 = words[index - 15].rotate_right(7)
            ^ words[index - 15].rotate_right(18)
            ^ (words[index - 15] >> 3_u32);
        let s1 = words[index - 2].rotate_right(17)
            ^ words[index - 2].rotate_right(19)
            ^ (words[index - 2] >> 10_u32);
        words[index] = words[index - 16]
            .wrapping_add(s0)
            .wrapping_add(words[index - 7])
            .wrapping_add(s1);
    }
    let [
        mut a,
        mut b,
        mut c,
        mut d,
        mut e,
        mut f,
        mut g,
        mut h,
    ] = *state;
    for index in 0..64 {
        let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(K[index])
            .wrapping_add(words[index]);
        let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }
    for (target, value) in state
        .iter_mut()
        .zip(
            [
                a, b, c, d, e, f, g, h,
            ],
        )
    {
        *target = target.wrapping_add(value);
    }
}

#[cfg(test)]
mod tests {
    use super::Sha256;

    #[test]
    fn matches_known_empty_digest() {
        assert_eq!(
            Sha256::digest(b"").hex(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn matches_known_abc_digest() {
        assert_eq!(
            Sha256::digest(b"abc").hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}

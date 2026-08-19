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
//   - Format domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Format domain module.
// - Description:
//   - Implements the declared domain module responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Format domain module.

const BINK_FILE_SIZE_BIAS: u64 = 8;
/// Maximum accepted Bink frame count.
const BINK_MAX_FRAMES: u32 = 1_000_000;
/// Maximum supported Bink frame width.
const BINK_MAX_WIDTH: u32 = 7_680;
/// Maximum supported Bink frame height.
const BINK_MAX_HEIGHT: u32 = 4_800;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Moviekind.
pub enum MovieKind {
    /// Item.
    BinkV1,
    /// Item.
    BinkV2,
    /// Item.
    OggNamedRmv,
    /// Item.
    XboxXmvLike,
    /// Item.
    RadicalMovieHeader,
    /// Item.
    Unknown,
}

impl MovieKind {
    /// Number of bytes required to validate every supported Bink header field.
    pub const HEADER_PROBE_LEN: usize = 36;

    #[must_use]
    /// Classifies complete movie bytes and validates available container size
    /// fields.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let Ok(actual_size) = u64::try_from(bytes.len()) else {
            return Self::Unknown;
        };
        Self::from_sized_header(bytes, actual_size)
    }

    #[must_use]
    /// Classifies a movie header against the complete file length.
    pub fn from_sized_header(header: &[u8], actual_size: u64) -> Self {
        let kind = Self::from_prefix(header);
        let valid = match kind {
            Self::BinkV1 | Self::BinkV2 => {
                bink_header_is_valid(header, actual_size)
            },
            Self::OggNamedRmv => ogg_stream_is_valid(header, actual_size),
            Self::XboxXmvLike => xmv_header_is_valid(header, actual_size),
            Self::RadicalMovieHeader | Self::Unknown => false,
        };
        if valid {
            kind
        } else {
            Self::Unknown
        }
    }

    #[must_use]
    /// Classifies a movie signature without claiming complete validity.
    pub fn from_prefix(prefix: &[u8]) -> Self {
        let bink = bink_kind(prefix);
        if bink != Self::Unknown {
            bink
        } else if prefix.starts_with(b"OggS") {
            Self::OggNamedRmv
        } else if prefix.get(12..16) == Some(b"xobX") {
            Self::XboxXmvLike
        } else if prefix.starts_with(b"rmv") {
            Self::RadicalMovieHeader
        } else {
            Self::Unknown
        }
    }

    #[must_use]
    /// Label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::BinkV1 => "bink-v1",
            Self::BinkV2 => "bink-v2",
            Self::OggNamedRmv => "ogg-named-rmv",
            Self::XboxXmvLike => "xbox-xmv-like",
            Self::RadicalMovieHeader => "radical-movie-header",
            Self::Unknown => "unknown",
        }
    }
}

/// Returns the supported Bink kind encoded by a four-byte signature.
fn bink_kind(prefix: &[u8]) -> MovieKind {
    let Some(signature_bytes) = prefix.get(..4) else {
        return MovieKind::Unknown;
    };
    let Ok(signature) = <[u8; 4]>::try_from(signature_bytes) else {
        return MovieKind::Unknown;
    };
    match signature {
        [b'B', b'I', b'K', b'b' | b'f' | b'g' | b'h' | b'i' | b'k'] => {
            MovieKind::BinkV1
        },
        [
            b'K',
            b'B',
            b'2',
            b'a' | b'd' | b'f' | b'g' | b'h' | b'i' | b'j' | b'k',
        ] => MovieKind::BinkV2,
        _ => MovieKind::Unknown,
    }
}

const OGG_FIXED_HEADER_LEN: usize = 27;
const OGG_CRC_OFFSET: usize = 22;
const OGG_CRC_LEN: usize = 4;
const OGG_CRC_POLYNOMIAL: u32 = 0x04c1_1db7;
const XMV_MIN_HEADER_SIZE: usize = 36;
const XMV_AUDIO_HEADER_SIZE: usize = 12;

/// Verifies complete Ogg page framing and page checksums.
fn ogg_stream_is_valid(bytes: &[u8], actual_size: u64) -> bool {
    if u64::try_from(bytes.len()).ok() != Some(actual_size) {
        return false;
    }
    let mut offset = 0_usize;
    let mut pages = 0_usize;
    while offset < bytes.len() {
        let Some(fixed_end) = offset.checked_add(OGG_FIXED_HEADER_LEN) else {
            return false;
        };
        let Some(fixed) = bytes.get(offset..fixed_end) else {
            return false;
        };
        let version = fixed.get(4).copied();
        let header_type = fixed.get(5).copied();
        let segment_count = fixed.get(26).copied();
        if !fixed.starts_with(b"OggS")
            || version != Some(0)
            || header_type.is_none_or(|value| value & !0x07 != 0)
        {
            return false;
        }
        let Some(segment_count) = segment_count.map(usize::from) else {
            return false;
        };
        let Some(table_start) = offset.checked_add(OGG_FIXED_HEADER_LEN) else {
            return false;
        };
        let Some(table_end) = table_start.checked_add(segment_count) else {
            return false;
        };
        let Some(segment_table) = bytes.get(table_start..table_end) else {
            return false;
        };
        let Some(body_len) =
            segment_table.iter().try_fold(0_usize, |total, value| {
                total.checked_add(usize::from(*value))
            })
        else {
            return false;
        };
        let Some(page_end) = table_end.checked_add(body_len) else {
            return false;
        };
        let Some(page) = bytes.get(offset..page_end) else {
            return false;
        };
        let Some(expected_crc) = read_header_u32(page, OGG_CRC_OFFSET) else {
            return false;
        };
        if ogg_page_crc(page) != expected_crc {
            return false;
        }
        let Some(next_pages) = pages.checked_add(1) else {
            return false;
        };
        pages = next_pages;
        offset = page_end;
    }
    pages > 0
}

fn ogg_page_crc(page: &[u8]) -> u32 {
    let mut crc = 0_u32;
    for (index, value) in page.iter().copied().enumerate() {
        let checksum_end = OGG_CRC_OFFSET.saturating_add(OGG_CRC_LEN);
        let byte = if (OGG_CRC_OFFSET..checksum_end).contains(&index) {
            0
        } else {
            value
        };
        crc ^= u32::from(byte) << 24;
        for _ in 0..8 {
            crc = if crc & 0x8000_0000 != 0 {
                (crc << 1) ^ OGG_CRC_POLYNOMIAL
            } else {
                crc << 1
            };
        }
    }
    crc
}

/// Verifies the complete fixed and per-track XMV header fits the first packet.
fn xmv_header_is_valid(header: &[u8], actual_size: u64) -> bool {
    if header.len() < XMV_MIN_HEADER_SIZE {
        return false;
    }
    let Some(version) = read_header_u32(header, 16) else {
        return false;
    };
    if !(1..=4).contains(&version) {
        return false;
    }
    let Some(audio_tracks) = read_header_u16(header, 32) else {
        return false;
    };
    let Some(audio_bytes) =
        usize::from(audio_tracks).checked_mul(XMV_AUDIO_HEADER_SIZE)
    else {
        return false;
    };
    let Some(header_size) = XMV_MIN_HEADER_SIZE.checked_add(audio_bytes) else {
        return false;
    };
    if header.len() < header_size {
        return false;
    }
    let Some(first_packet_size) = read_header_u32(header, 4) else {
        return false;
    };
    let Ok(header_size) = u64::try_from(header_size) else {
        return false;
    };
    u64::from(first_packet_size) >= header_size
        && u64::from(first_packet_size) <= actual_size
}

fn read_header_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let field = bytes.get(offset..end)?;
    Some(u16::from_le_bytes(<[u8; 2]>::try_from(field).ok()?))
}

/// Verifies mandatory Bink header fields shared by supported revisions.
fn bink_header_is_valid(header: &[u8], actual_size: u64) -> bool {
    if header.len() < MovieKind::HEADER_PROBE_LEN {
        return false;
    }
    let Some(declared_size) = read_header_u32(header, 4) else {
        return false;
    };
    let Some(frame_count) = read_header_u32(header, 8) else {
        return false;
    };
    let Some(largest_frame_size) = read_header_u32(header, 12) else {
        return false;
    };
    let Some(width) = read_header_u32(header, 20) else {
        return false;
    };
    let Some(height) = read_header_u32(header, 24) else {
        return false;
    };
    let Some(frame_rate_numerator) = read_header_u32(header, 28) else {
        return false;
    };
    let Some(frame_rate_denominator) = read_header_u32(header, 32) else {
        return false;
    };
    let declared_size_matches = u64::from(declared_size)
        .checked_add(BINK_FILE_SIZE_BIAS)
        == Some(actual_size);
    declared_size_matches
        && frame_count > 0
        && frame_count <= BINK_MAX_FRAMES
        && u64::from(largest_frame_size) <= actual_size
        && width > 0
        && width <= BINK_MAX_WIDTH
        && height > 0
        && height <= BINK_MAX_HEIGHT
        && frame_rate_numerator > 0
        && frame_rate_denominator > 0
}

/// Reads one little-endian 32-bit Bink header field.
fn read_header_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let field = bytes.get(offset..end)?;
    let field_bytes = <[u8; 4]>::try_from(field).ok()?;
    let value = u32::from_le_bytes(field_bytes);
    Some(value)
}

#[cfg(test)]
#[path = "../../../../tests/formats/rmv/unit/domain/format_tests.rs"]
mod tests;

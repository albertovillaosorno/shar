// File:
//   - rsd.rs
// Path:
//   - src/rsd/src/domain/rsd.rs
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
//   - Pure rsd domain rules for domain rsd.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - A supported codec gains an independent frame layout or predictor state
//   - that no longer shares the current bounded decode contract.
// - Merge-When:
//   - Another rsd module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Parser and decoder for Radical Sound `.rsd` containers.
// - Description:
//   - Validates RSD headers and decodes supported sample frames into PCM while
//   - keeping malformed payloads outside the filesystem boundary.
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
//   - true
//   - Reason: src/rsd/src/domain/rsd.rs has 514 effective lines after the
//   - required header and remains cohesive until a focused split lands.
//

//! Parser and decoder for Radical Sound `.rsd` containers.
//!
//! Fixed header and frame invariants stay together so corrupt sample data fails
//! closed before a validated PCM model is handed to the WAV serializer.
use super::{RsdError, WavAudio, byte_buffer};

/// Header magic keeps the parser scoped to the supported RSD revision.
const MAGIC: &[u8; 4] = b"RSD4";
/// Padded RSD4 payloads begin after the full legacy header block.
const PADDED_DATA_OFFSET: usize = 0x800;
/// Compact PCM payloads begin after the short RSD4 header block.
const COMPACT_DATA_OFFSET: usize = 0x80;
/// Legacy padded headers fill the complete compact-to-sector gap with dashes.
const LEGACY_PADDING_BYTE: u8 = b'-';
/// Legacy writers fill unused fixed-header metadata with asterisks.
const LEGACY_RESERVED_BYTE: u8 = b'*';
/// The fixed fields needed to select and validate a payload layout.
const MINIMUM_HEADER_SIZE: usize = 20;
/// RADP decoder state supports channel indexes below the legacy limit.
const RADP_MAX_CHANNELS: u16 = 15;
/// RADP stores a compact fixed-size frame for each channel.
const RADP_FRAME_SIZE_PER_CHANNEL: usize = 20;
/// Each RADP frame expands to a fixed PCM sample count.
const RADP_SAMPLES_PER_FRAME: usize = 32;
/// IMA-style ADPCM index deltas are table-driven by nibble value.
const INDEX_ADJUST_TABLE: [i32; 16] = [
    -1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8,
];
/// Decoder step sizes are fixed by the ADPCM predictor state.
const STEP_TABLE: [i32; 89] = [
    7, 8, 9, 10, 11, 12, 13, 14, 16, 17, 19, 21, 23, 25, 28, 31, 34, 37, 41,
    45, 50, 55, 60, 66, 73, 80, 88, 97, 107, 118, 130, 143, 157, 173, 190, 209,
    230, 253, 279, 307, 337, 371, 408, 449, 494, 544, 598, 658, 724, 796, 876,
    963, 1060, 1166, 1282, 1411, 1552, 1707, 1878, 2066, 2272, 2499, 2749,
    3024, 3327, 3660, 4026, 4428, 4871, 5358, 5894, 6484, 7132, 7845, 8630,
    9493, 10442, 11487, 12635, 13899, 15289, 16818, 18500, 20350, 22385, 24623,
    27086, 29794, 32767,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Encoding tag determines whether bytes can be copied or decoded before WAV
/// output.
/// Encoding tags select the only conversion paths this exporter can prove.
pub enum RsdEncoding {
    /// Native little-endian PCM can be preserved directly.
    /// Native little-endian PCM can be copied after alignment checks.
    PcmLittleEndian,
    /// Big-endian PCM must be byte-swapped into WAV order.
    /// Big-endian PCM must be byte-swapped before WAV emission.
    PcmBigEndian,
    /// RADP ADPCM must be expanded into PCM samples.
    /// RADP needs predictor expansion before WAV emission.
    RadicalAdpcm,
}

/// Converts one platform-sized format constant for report arithmetic.
fn report_u64(
    value: usize,
    overflow: &'static str,
) -> Result<u64, RsdError> {
    match u64::try_from(value) {
        Ok(converted) => Ok(converted),
        Err(_conversion_error) => Err(RsdError::ReportOverflow(overflow)),
    }
}

/// Converts the fixed RADP decoded sample count for report arithmetic.
fn radp_samples_per_frame_u64() -> Result<u64, RsdError> {
    report_u64(
        RADP_SAMPLES_PER_FRAME,
        "RADP sample count exceeds report capacity",
    )
}

impl RsdEncoding {
    /// Converts raw header tags into explicit codec branches.
    const fn from_tag(tag: [u8; 4]) -> Result<Self, RsdError> {
        match &tag {
            b"PCM " => Ok(Self::PcmLittleEndian),
            b"PCMB" => Ok(Self::PcmBigEndian),
            b"RADP" => Ok(Self::RadicalAdpcm),
            _ => Err(RsdError::UnsupportedEncoding(tag)),
        }
    }

    #[must_use]
    /// Stable diagnostic name keeps reports independent from raw header bytes.
    /// Gives reports stable codec labels without exposing raw tag bytes.
    pub const fn name(self) -> &'static str {
        match self {
            Self::PcmLittleEndian => "PCM",
            Self::PcmBigEndian => "PCMB",
            Self::RadicalAdpcm => "RADP",
        }
    }
}

/// Header fields are normalized before payload decoding so raw offsets stay
/// local.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Header values are retained so export summaries can prove format coverage.
pub struct RsdHeader {
    /// Encoding decides the lossless conversion path.
    /// Encoding decides whether payload bytes are copied, swapped, or decoded.
    pub encoding: RsdEncoding,
    /// Channel count is narrowed after validation for frame-size arithmetic.
    /// Channel count is preserved because WAV output must match the source.
    pub channels: u16,
    /// Bit depth is preserved so WAV output matches source PCM depth.
    /// Bit depth is preserved to reject lossy or unsupported conversion.
    pub bits_per_sample: u16,
    /// Sample rate is copied unchanged to preserve playback speed.
    /// Sample rate is kept unchanged so 21 kHz sources stay 21 kHz.
    pub sample_rate: u32,
}

impl RsdHeader {
    /// Verifies one public header can describe a supported RSD/WAV stream.
    ///
    /// # Errors
    ///
    /// Returns [`RsdError`] when channels, bit depth, sample rate, byte rate,
    /// or codec-specific limits are unsupported.
    pub fn validate(&self) -> Result<(), RsdError> {
        if !(1_u16..=16_u16).contains(&self.channels) {
            return Err(
                RsdError::UnsupportedChannels(u32::from(self.channels)),
            );
        }
        if self.bits_per_sample != 16_u16 {
            return Err(
                RsdError::UnsupportedBitDepth(u32::from(self.bits_per_sample)),
            );
        }
        if self.sample_rate == 0_u32 || i32::try_from(self.sample_rate).is_err()
        {
            return Err(RsdError::UnsupportedSampleRate(self.sample_rate));
        }
        if self.encoding == RsdEncoding::RadicalAdpcm
            && self.channels > RADP_MAX_CHANNELS
        {
            return Err(
                RsdError::UnsupportedChannels(u32::from(self.channels)),
            );
        }
        let bytes_per_sample = self
            .bits_per_sample
            .div_euclid(8_u16);
        let block_align = u32::from(self.channels)
            .checked_mul(u32::from(bytes_per_sample))
            .ok_or(RsdError::UnsupportedSampleRate(self.sample_rate))?;
        if self
            .sample_rate
            .checked_mul(block_align)
            .is_none()
        {
            return Err(RsdError::UnsupportedSampleRate(self.sample_rate));
        }
        Ok(())
    }

    /// Returns the smallest complete source byte count for this format.
    pub(crate) fn minimum_source_file_bytes(self) -> Result<u64, RsdError> {
        let bytes_per_sample = self
            .bits_per_sample
            .div_euclid(8_u16);
        let header_bytes = match self.encoding {
            RsdEncoding::PcmLittleEndian => report_u64(
                COMPACT_DATA_OFFSET,
                "compact RSD header exceeds report capacity",
            )?,
            RsdEncoding::PcmBigEndian | RsdEncoding::RadicalAdpcm => {
                report_u64(
                    PADDED_DATA_OFFSET,
                    "padded RSD header exceeds report capacity",
                )?
            }
        };
        let channel_frame_bytes = match self.encoding {
            RsdEncoding::PcmLittleEndian | RsdEncoding::PcmBigEndian => {
                u64::from(bytes_per_sample)
            }
            RsdEncoding::RadicalAdpcm => report_u64(
                RADP_FRAME_SIZE_PER_CHANNEL,
                "RADP frame size exceeds report capacity",
            )?,
        };
        let frame_bytes = u64::from(self.channels)
            .checked_mul(channel_frame_bytes)
            .ok_or(
                RsdError::ReportOverflow(
                    "RSD minimum source frame byte count overflow",
                ),
            )?;
        header_bytes
            .checked_add(frame_bytes)
            .ok_or(
                RsdError::ReportOverflow(
                    "RSD minimum source file byte count overflow",
                ),
            )
    }

    /// Returns the smallest complete WAV byte count for this channel layout.
    pub(crate) fn minimum_wav_file_bytes(self) -> Result<u64, RsdError> {
        let bytes_per_sample = self
            .bits_per_sample
            .div_euclid(8_u16);
        let channels = u64::from(self.channels);
        let samples_per_frame = match self.encoding {
            RsdEncoding::RadicalAdpcm => radp_samples_per_frame_u64()?,
            RsdEncoding::PcmLittleEndian | RsdEncoding::PcmBigEndian => 1_u64,
        };
        let channel_samples = channels
            .checked_mul(samples_per_frame)
            .ok_or(
                RsdError::ReportOverflow(
                    "RSD minimum WAV sample count overflow",
                ),
            )?;
        let frame_bytes = channel_samples
            .checked_mul(u64::from(bytes_per_sample))
            .ok_or(
                RsdError::ReportOverflow(
                    "RSD minimum WAV frame byte count overflow",
                ),
            )?;
        WavAudio::HEADER_BYTES
            .checked_add(frame_bytes)
            .ok_or(
                RsdError::ReportOverflow(
                    "RSD minimum WAV file byte count overflow",
                ),
            )
    }
}

/// Parsed audio pairs trusted header metadata with the owned source payload.
///
/// Parsed format metadata is immutable outside this domain boundary.
///
/// ```compile_fail
/// fn replace_header(audio: &mut rsd::RsdAudio, header: rsd::RsdHeader) {
///     audio.header = header;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
/// Parsed audio keeps header and payload together to avoid format drift.
pub struct RsdAudio {
    /// Header travels with payload so conversion cannot mix file metadata.
    header: RsdHeader,
    /// Payload remains private so callers cannot bypass codec validation.
    payload: Vec<u8>,
}

impl RsdAudio {
    /// Returns the immutable format metadata paired with this payload.
    #[must_use]
    pub const fn header(&self) -> RsdHeader {
        self.header
    }

    /// Parses container bytes before any decoder can assume frame alignment.
    ///
    /// # Errors
    ///
    /// Returns an error when the header is malformed or the encoding is
    /// unsupported.
    /// Parses an RSD byte slice into a validated audio model.
    ///
    /// # Errors
    ///
    /// Returns [`RsdError`] when the header, codec, or payload shape is
    /// unsupported.
    pub fn parse(data: &[u8]) -> Result<Self, RsdError> {
        if data.len() < MINIMUM_HEADER_SIZE {
            return Err(RsdError::TruncatedHeader);
        }
        if data.get(..MAGIC.len()) != Some(MAGIC.as_slice()) {
            return Err(RsdError::BadMagic);
        }
        let tag = read_fixed_array::<4>(
            data,
            MAGIC.len(),
        )?;
        let encoding = RsdEncoding::from_tag(tag)?;
        let raw_channels = read_u32(
            data, 8,
        )?;
        let raw_bits_per_sample = read_u32(
            data, 12,
        )?;
        let sample_rate = read_u32(
            data, 16,
        )?;
        let Ok(channels) = u16::try_from(raw_channels) else {
            return Err(RsdError::UnsupportedChannels(raw_channels));
        };
        let Ok(bits_per_sample) = u16::try_from(raw_bits_per_sample) else {
            return Err(RsdError::UnsupportedBitDepth(raw_bits_per_sample));
        };
        let header = RsdHeader {
            encoding,
            channels,
            bits_per_sample,
            sample_rate,
        };
        header.validate()?;
        let has_legacy_padding = data
            .get(COMPACT_DATA_OFFSET..PADDED_DATA_OFFSET)
            .is_some_and(
                |padding| {
                    padding
                        .iter()
                        .all(|byte| *byte == LEGACY_PADDING_BYTE)
                },
            );
        let has_legacy_reserved = has_legacy_reserved_header(data);
        if has_legacy_padding != has_legacy_reserved {
            return Err(RsdError::InvalidHeaderPadding);
        }
        if encoding != RsdEncoding::PcmLittleEndian && !has_legacy_padding {
            return Err(RsdError::InvalidHeaderPadding);
        }
        let data_offset = match encoding {
            RsdEncoding::PcmLittleEndian | RsdEncoding::PcmBigEndian
                if !has_legacy_padding =>
            {
                COMPACT_DATA_OFFSET
            }
            _ => PADDED_DATA_OFFSET,
        };
        let payload_bytes = data
            .get(data_offset..)
            .ok_or(RsdError::TruncatedData)?;
        if payload_bytes.is_empty() {
            return Err(RsdError::TruncatedData);
        }
        let _frame_size = validate_payload_alignment(
            payload_bytes,
            header,
        )?;
        let mut payload = byte_buffer(payload_bytes.len())?;
        payload.extend_from_slice(payload_bytes);
        Ok(
            Self {
                header,
                payload,
            },
        )
    }

    /// Converts the payload into the WAV representation used by export output.
    ///
    /// # Errors
    ///
    /// Returns an error when payload alignment or ADPCM state is invalid.
    /// Converts validated source audio into a WAV model without resampling.
    ///
    /// # Errors
    ///
    /// Returns [`RsdError`] when payload alignment or codec expansion fails.
    pub fn to_wav(&self) -> Result<WavAudio, RsdError> {
        let pcm = match self
            .header
            .encoding
        {
            RsdEncoding::PcmLittleEndian => validate_pcm_payload(
                &self.payload,
                self.header,
            )?,
            RsdEncoding::PcmBigEndian => decode_big_endian_pcm(
                &self.payload,
                self.header,
            )?,
            RsdEncoding::RadicalAdpcm => decode_radical_adpcm(
                &self.payload,
                self.header,
            )?,
        };
        Ok(
            WavAudio {
                channels: self
                    .header
                    .channels,
                bits_per_sample: self
                    .header
                    .bits_per_sample,
                sample_rate: self
                    .header
                    .sample_rate,
                pcm,
            },
        )
    }
}

/// Reads one fixed array with checked end arithmetic for every header field.
fn read_fixed_array<const SIZE: usize>(
    data: &[u8],
    pos: usize,
) -> Result<[u8; SIZE], RsdError> {
    let end = pos
        .checked_add(SIZE)
        .ok_or(RsdError::TruncatedHeader)?;
    let bytes = data
        .get(pos..end)
        .ok_or(RsdError::TruncatedHeader)?;
    let Ok(array) = <[u8; SIZE]>::try_from(bytes) else {
        return Err(RsdError::TruncatedHeader);
    };
    Ok(array)
}

/// Reads fixed little-endian header fields through checked slices.
fn read_u32(
    data: &[u8],
    pos: usize,
) -> Result<u32, RsdError> {
    Ok(
        u32::from_le_bytes(
            read_fixed_array::<4>(
                data, pos,
            )?,
        ),
    )
}

/// Verifies the fixed metadata region of one legacy padded header.
fn has_legacy_reserved_header(data: &[u8]) -> bool {
    data.get(MINIMUM_HEADER_SIZE..COMPACT_DATA_OFFSET)
        .is_some_and(
            |reserved| {
                reserved
                    .iter()
                    .all(|byte| *byte == LEGACY_RESERVED_BYTE)
            },
        )
}

/// Validates codec frame alignment and returns the complete frame size.
fn validate_payload_alignment(
    payload: &[u8],
    header: RsdHeader,
) -> Result<usize, RsdError> {
    let frame_size = match header.encoding {
        RsdEncoding::PcmLittleEndian | RsdEncoding::PcmBigEndian => {
            let bytes_per_sample = header
                .bits_per_sample
                .div_euclid(8_u16);
            usize::from(header.channels)
                .checked_mul(usize::from(bytes_per_sample))
        }
        RsdEncoding::RadicalAdpcm => RADP_FRAME_SIZE_PER_CHANNEL
            .checked_mul(usize::from(header.channels)),
    }
    .ok_or_else(
        || RsdError::UnalignedPayload {
            encoding: header
                .encoding
                .name(),
            bytes: payload.len(),
            frame_size: usize::MAX,
        },
    )?;
    if !payload
        .len()
        .is_multiple_of(frame_size)
    {
        return Err(
            RsdError::UnalignedPayload {
                encoding: header
                    .encoding
                    .name(),
                bytes: payload.len(),
                frame_size,
            },
        );
    }
    Ok(frame_size)
}

/// Copies native PCM only after the payload is frame-aligned.
fn validate_pcm_payload(
    payload: &[u8],
    header: RsdHeader,
) -> Result<Vec<u8>, RsdError> {
    let _frame_size = validate_payload_alignment(
        payload, header,
    )?;
    let mut pcm = byte_buffer(payload.len())?;
    pcm.extend_from_slice(payload);
    Ok(pcm)
}

/// Swaps big-endian PCM only after the payload is known frame-aligned.
fn decode_big_endian_pcm(
    payload: &[u8],
    header: RsdHeader,
) -> Result<Vec<u8>, RsdError> {
    let mut pcm = validate_pcm_payload(
        payload, header,
    )?;
    for sample in pcm.chunks_mut(2) {
        sample.swap(
            0, 1,
        );
    }
    Ok(pcm)
}

#[derive(Clone, Copy)]
/// RADP predictor data is per-channel and evolves per decoded nibble.
struct RadpDecoder {
    /// Step-table index must stay in the decoder table range.
    index: i32,
    /// Previous sample anchors the differential predictor.
    previous: i32,
}

/// Expands RADP frames to interleaved 16-bit PCM.
fn decode_radical_adpcm(
    payload: &[u8],
    header: RsdHeader,
) -> Result<Vec<u8>, RsdError> {
    let channels = usize::from(header.channels);
    let frame_size = validate_payload_alignment(
        payload, header,
    )?;
    let frame_count = payload
        .len()
        .div_euclid(frame_size);
    let capacity = frame_count
        .checked_mul(RADP_SAMPLES_PER_FRAME)
        .and_then(|value| value.checked_mul(channels))
        .and_then(|value| value.checked_mul(2_usize))
        .ok_or(RsdError::WavTooLarge(payload.len()))?;
    let mut pcm = byte_buffer(capacity)?;
    for frame in payload.chunks_exact(frame_size) {
        decode_radp_frame(
            frame, channels, &mut pcm,
        )?;
    }
    Ok(pcm)
}

/// Reads one signed RADP header word without exposing raw frame indexing.
fn read_frame_i16(
    frame: &[u8],
    pos: usize,
) -> Result<i16, RsdError> {
    let end = pos
        .checked_add(2_usize)
        .ok_or(RsdError::TruncatedData)?;
    let bytes = frame
        .get(pos..end)
        .ok_or(RsdError::TruncatedData)?;
    let Ok(array) = <[u8; 2]>::try_from(bytes) else {
        return Err(RsdError::TruncatedData);
    };
    Ok(i16::from_le_bytes(array))
}

/// Decodes one multi-channel RADP frame while preserving sample ordering.
fn decode_radp_frame(
    frame: &[u8],
    channels: usize,
    pcm: &mut Vec<u8>,
) -> Result<(), RsdError> {
    let mut states = Vec::with_capacity(channels);
    let mut cursor = 0_usize;
    for _ in 0..channels {
        let index = i32::from(
            read_frame_i16(
                frame, cursor,
            )?,
        );
        if !(0_i32..=88_i32).contains(&index) {
            return Err(RsdError::InvalidStepIndex(index));
        }
        let previous_offset = cursor
            .checked_add(2_usize)
            .ok_or(RsdError::TruncatedData)?;
        let previous = i32::from(
            read_frame_i16(
                frame,
                previous_offset,
            )?,
        );
        states.push(
            RadpDecoder {
                index,
                previous,
            },
        );
        cursor = cursor
            .checked_add(4_usize)
            .ok_or(RsdError::TruncatedData)?;
    }

    while cursor < frame.len() {
        let mut pairs = Vec::with_capacity(channels);
        for state in states
            .iter_mut()
            .take(channels)
        {
            let byte = *frame
                .get(cursor)
                .ok_or(RsdError::TruncatedData)?;
            cursor = cursor
                .checked_add(1_usize)
                .ok_or(RsdError::TruncatedData)?;
            let first = decode_nibble(
                byte & 0x0f_u8,
                state,
            )?;
            let second = decode_nibble(
                (byte >> 4_u32) & 0x0f_u8,
                state,
            )?;
            pairs.push(
                (
                    first, second,
                ),
            );
        }
        for (first, _) in &pairs {
            pcm.extend_from_slice(&first.to_le_bytes());
        }
        for (_, second) in &pairs {
            pcm.extend_from_slice(&second.to_le_bytes());
        }
    }
    Ok(())
}

/// Applies one ADPCM nibble to the predictor state.
fn decode_nibble(
    delta: u8,
    state: &mut RadpDecoder,
) -> Result<i16, RsdError> {
    if !(0_i32..=88_i32).contains(&state.index) {
        return Err(RsdError::InvalidStepIndex(state.index));
    }
    let Ok(step_index) = usize::try_from(state.index) else {
        return Err(RsdError::InvalidStepIndex(state.index));
    };
    let Some(&step) = STEP_TABLE.get(step_index) else {
        return Err(RsdError::InvalidStepIndex(state.index));
    };
    let mut difference = step >> 3_u32;
    if delta & 1_u8 != 0_u8 {
        difference = difference.saturating_add(step >> 2_u32);
    }
    if delta & 2_u8 != 0_u8 {
        difference = difference.saturating_add(step >> 1_u32);
    }
    if delta & 4_u8 != 0_u8 {
        difference = difference.saturating_add(step);
    }
    if delta & 8_u8 != 0_u8 {
        difference = difference.saturating_neg();
    }
    state.previous = state
        .previous
        .saturating_add(difference)
        .clamp(
            i32::from(i16::MIN),
            i32::from(i16::MAX),
        );
    let delta_index = usize::from(delta);
    let Some(&adjustment) = INDEX_ADJUST_TABLE.get(delta_index) else {
        return Err(RsdError::InvalidStepIndex(state.index));
    };
    state.index = state
        .index
        .saturating_add(adjustment)
        .clamp(
            0_i32, 88_i32,
        );
    let Ok(sample) = i16::try_from(state.previous) else {
        return Err(RsdError::InvalidSample(state.previous));
    };
    Ok(sample)
}

#[cfg(test)]
mod tests {
    use super::{PADDED_DATA_OFFSET, RsdAudio, RsdEncoding, RsdError};

    fn copy_fixture_bytes(
        data: &mut [u8],
        start: usize,
        bytes: &[u8],
    ) -> bool {
        let Some(end) = start.checked_add(bytes.len()) else {
            return false;
        };
        let Some(target) = data.get_mut(start..end) else {
            return false;
        };
        target.copy_from_slice(bytes);
        true
    }

    fn rsd_with(
        tag: [u8; 4],
        payload: &[u8],
    ) -> Vec<u8> {
        let mut data = vec![0_u8; PADDED_DATA_OFFSET];
        assert!(
            copy_fixture_bytes(
                &mut data, 0, b"RSD4"
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data, 4, &tag
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data,
                8,
                &1_u32.to_le_bytes(),
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data,
                12,
                &16_u32.to_le_bytes(),
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data,
                16,
                &24_000_u32.to_le_bytes(),
            )
        );
        let reserved = vec![b'*'; 0x80 - 20];
        assert!(
            copy_fixture_bytes(
                &mut data, 20, &reserved,
            )
        );
        let padding = vec![b'-'; 0x800 - 0x80];
        assert!(
            copy_fixture_bytes(
                &mut data, 0x80, &padding,
            )
        );
        data.extend_from_slice(payload);
        data
    }

    #[test]
    fn unsupported_encoding_precedes_numeric_narrowing() {
        let mut data = vec![0_u8; 20];
        assert!(
            copy_fixture_bytes(
                &mut data, 0, b"RSD4"
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data, 4, b"BAD!"
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data,
                8,
                &u32::MAX.to_le_bytes(),
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data,
                12,
                &16_u32.to_le_bytes(),
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data,
                16,
                &24_000_u32.to_le_bytes(),
            )
        );

        assert!(
            matches!(
                RsdAudio::parse(&data),
                Err(RsdError::UnsupportedEncoding(tag)) if tag == *b"BAD!"
            ),
            "unsupported codecs must fail before numeric field narrowing"
        );
    }

    #[test]
    fn invalid_header_is_rejected_before_body_access() {
        let mut data = vec![0_u8; 20];
        assert!(
            copy_fixture_bytes(
                &mut data, 0, b"RSD4"
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data, 4, b"PCM "
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data,
                8,
                &0_u32.to_le_bytes(),
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data,
                12,
                &16_u32.to_le_bytes(),
            )
        );
        assert!(
            copy_fixture_bytes(
                &mut data,
                16,
                &24_000_u32.to_le_bytes(),
            )
        );

        let result = RsdAudio::parse(&data);

        assert!(
            matches!(
                result,
                Err(RsdError::UnsupportedChannels(value))
                    if value == 0_u32
            ),
            "invalid header fields must fail before payload validation"
        );
    }

    #[test]
    fn parses_pcm_header_and_preserves_native_encoding() {
        let data = rsd_with(
            *b"PCM ",
            &[
                1, 0, 2, 0,
            ],
        );
        let parsed = RsdAudio::parse(&data);
        assert!(
            parsed.is_ok(),
            "RSD header should parse"
        );
        let Ok(audio) = parsed else {
            return;
        };
        assert_eq!(
            audio
                .header
                .encoding,
            RsdEncoding::PcmLittleEndian
        );
        assert_eq!(
            audio
                .header
                .channels,
            1
        );
        assert_eq!(
            audio
                .header
                .bits_per_sample,
            16
        );
        assert_eq!(
            audio
                .header
                .sample_rate,
            24_000
        );
        let converted = audio.to_wav();
        assert!(
            converted.is_ok(),
            "PCM audio should convert to WAV"
        );
        let Ok(wav) = converted else {
            return;
        };
        assert_eq!(
            wav.pcm,
            vec![
                1, 0, 2, 0
            ]
        );
    }

    #[test]
    fn decodes_silent_radical_adpcm_frame_to_silent_pcm() {
        let mut frame = Vec::new();
        frame.extend_from_slice(&0_i16.to_le_bytes());
        frame.extend_from_slice(&0_i16.to_le_bytes());
        frame.extend(
            std::iter::repeat_n(
                0_u8, 16,
            ),
        );
        let data = rsd_with(
            *b"RADP", &frame,
        );
        let parsed = RsdAudio::parse(&data);
        assert!(
            parsed.is_ok(),
            "RSD header should parse"
        );
        let Ok(audio) = parsed else {
            return;
        };
        let converted = audio.to_wav();
        assert!(
            converted.is_ok(),
            "RADP audio should decode to WAV"
        );
        let Ok(wav) = converted else {
            return;
        };
        assert_eq!(
            wav.pcm
                .len(),
            32 * 2
        );
        assert!(
            wav.pcm
                .chunks(2)
                .all(
                    |sample| sample
                        == [
                            0, 0
                        ]
                ),
            "silent RADP frame should decode to zero PCM samples"
        );
    }
}

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
//   - Rsd domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Rsd domain module.
// - Description:
//   - Implements the declared domain module responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Rsd domain module.

use super::{RsdError, WavAudio, byte_buffer};

/// Shared Radical Sound container prefix for supported revisions.
const MAGIC_PREFIX: &[u8; 3] = b"RSD";
/// RSD3 stores the PCM payload start explicitly in its header.
const RSD3_VERSION: u8 = b'3';
/// RSD4 uses the later fixed-layout container contract.
const RSD4_VERSION: u8 = b'4';
/// Byte containing the ASCII container revision after the shared prefix.
const VERSION_OFFSET: usize = 3;
/// Byte where the four-character encoding tag begins.
const ENCODING_TAG_OFFSET: usize = 4;
/// RSD3 PCM/PCMB stores the payload start after one legacy unknown field.
const RSD3_DATA_OFFSET_FIELD: usize = 0x18;
/// RSD3 needs the explicit data-offset field before payload selection.
const RSD3_MINIMUM_HEADER_SIZE: usize = 0x1c;
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
const INDEX_ADJUST_TABLE: [i32; 16] =
    [-1, -1, -1, -1, 2, 4, 6, 8, -1, -1, -1, -1, 2, 4, 6, 8];
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
fn report_u64(value: usize, overflow: &'static str) -> Result<u64, RsdError> {
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
            return Err(RsdError::UnsupportedChannels(u32::from(
                self.channels,
            )));
        }
        if self.bits_per_sample != 16_u16 {
            return Err(RsdError::UnsupportedBitDepth(u32::from(
                self.bits_per_sample,
            )));
        }
        if self.sample_rate == 0_u32 || i32::try_from(self.sample_rate).is_err()
        {
            return Err(RsdError::UnsupportedSampleRate(self.sample_rate));
        }
        if self.encoding == RsdEncoding::RadicalAdpcm
            && self.channels > RADP_MAX_CHANNELS
        {
            return Err(RsdError::UnsupportedChannels(u32::from(
                self.channels,
            )));
        }
        let bytes_per_sample = self.bits_per_sample.div_euclid(8_u16);
        let block_align = u32::from(self.channels)
            .checked_mul(u32::from(bytes_per_sample))
            .ok_or(RsdError::UnsupportedSampleRate(self.sample_rate))?;
        if self.sample_rate.checked_mul(block_align).is_none() {
            return Err(RsdError::UnsupportedSampleRate(self.sample_rate));
        }
        Ok(())
    }

    /// Returns the smallest complete source byte count for this format.
    pub(crate) fn minimum_source_file_bytes(self) -> Result<u64, RsdError> {
        let bytes_per_sample = self.bits_per_sample.div_euclid(8_u16);
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
            },
        };
        let channel_frame_bytes = match self.encoding {
            RsdEncoding::PcmLittleEndian | RsdEncoding::PcmBigEndian => {
                u64::from(bytes_per_sample)
            },
            RsdEncoding::RadicalAdpcm => report_u64(
                RADP_FRAME_SIZE_PER_CHANNEL,
                "RADP frame size exceeds report capacity",
            )?,
        };
        let frame_bytes = u64::from(self.channels)
            .checked_mul(channel_frame_bytes)
            .ok_or(RsdError::ReportOverflow(
                "RSD minimum source frame byte count overflow",
            ))?;
        header_bytes
            .checked_add(frame_bytes)
            .ok_or(RsdError::ReportOverflow(
                "RSD minimum source file byte count overflow",
            ))
    }

    /// Returns the smallest complete WAV byte count for this channel layout.
    pub(crate) fn minimum_wav_file_bytes(self) -> Result<u64, RsdError> {
        let bytes_per_sample = self.bits_per_sample.div_euclid(8_u16);
        let channels = u64::from(self.channels);
        let samples_per_frame = match self.encoding {
            RsdEncoding::RadicalAdpcm => radp_samples_per_frame_u64()?,
            RsdEncoding::PcmLittleEndian | RsdEncoding::PcmBigEndian => 1_u64,
        };
        let channel_samples = channels.checked_mul(samples_per_frame).ok_or(
            RsdError::ReportOverflow("RSD minimum WAV sample count overflow"),
        )?;
        let frame_bytes = channel_samples
            .checked_mul(u64::from(bytes_per_sample))
            .ok_or(RsdError::ReportOverflow(
                "RSD minimum WAV frame byte count overflow",
            ))?;
        WavAudio::HEADER_BYTES.checked_add(frame_bytes).ok_or(
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
        if data.get(..MAGIC_PREFIX.len()) != Some(MAGIC_PREFIX.as_slice()) {
            return Err(RsdError::BadMagic);
        }
        let version =
            *data.get(VERSION_OFFSET).ok_or(RsdError::TruncatedHeader)?;
        if !matches!(version, RSD3_VERSION | RSD4_VERSION) {
            return Err(RsdError::BadMagic);
        }
        let tag = read_fixed_array::<4>(data, ENCODING_TAG_OFFSET)?;
        let encoding = RsdEncoding::from_tag(tag)?;
        let raw_channels = read_u32(data, 8)?;
        let raw_bits_per_sample = read_u32(data, 12)?;
        let sample_rate = read_u32(data, 16)?;
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
        let data_offset = match version {
            RSD3_VERSION => rsd3_data_offset(data, tag, encoding)?,
            RSD4_VERSION => rsd4_data_offset(data, encoding)?,
            _ => return Err(RsdError::BadMagic),
        };
        let payload_bytes =
            data.get(data_offset..).ok_or(RsdError::TruncatedData)?;
        if payload_bytes.is_empty() {
            return Err(RsdError::TruncatedData);
        }
        let _frame_size = validate_payload_alignment(payload_bytes, header)?;
        let mut payload = byte_buffer(payload_bytes.len())?;
        payload.extend_from_slice(payload_bytes);
        Ok(Self { header, payload })
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
        let pcm = match self.header.encoding {
            RsdEncoding::PcmLittleEndian => {
                validate_pcm_payload(&self.payload, self.header)?
            },
            RsdEncoding::PcmBigEndian => {
                decode_big_endian_pcm(&self.payload, self.header)?
            },
            RsdEncoding::RadicalAdpcm => {
                decode_radical_adpcm(&self.payload, self.header)?
            },
        };
        Ok(WavAudio {
            channels: self.header.channels,
            bits_per_sample: self.header.bits_per_sample,
            sample_rate: self.header.sample_rate,
            pcm,
        })
    }
}

/// Select the explicit payload start used by RSD3 PCM containers.
fn rsd3_data_offset(
    data: &[u8],
    tag: [u8; 4],
    encoding: RsdEncoding,
) -> Result<usize, RsdError> {
    if data.len() < RSD3_MINIMUM_HEADER_SIZE {
        return Err(RsdError::TruncatedHeader);
    }
    if !matches!(
        encoding,
        RsdEncoding::PcmLittleEndian | RsdEncoding::PcmBigEndian
    ) {
        return Err(RsdError::UnsupportedEncoding(tag));
    }
    let raw_offset = read_u32(data, RSD3_DATA_OFFSET_FIELD)?;
    let offset = usize::try_from(raw_offset)
        .map_err(|_error| RsdError::InvalidDataOffset(raw_offset))?;
    if offset < RSD3_MINIMUM_HEADER_SIZE || offset > data.len() {
        return Err(RsdError::InvalidDataOffset(raw_offset));
    }
    Ok(offset)
}

/// Select the compact or padded payload start used by RSD4 containers.
fn rsd4_data_offset(
    data: &[u8],
    encoding: RsdEncoding,
) -> Result<usize, RsdError> {
    let has_legacy_padding = data
        .get(COMPACT_DATA_OFFSET..PADDED_DATA_OFFSET)
        .is_some_and(|padding| {
            padding.iter().all(|byte| *byte == LEGACY_PADDING_BYTE)
        });
    let has_legacy_reserved = has_legacy_reserved_header(data);
    if has_legacy_padding != has_legacy_reserved {
        return Err(RsdError::InvalidHeaderPadding);
    }
    if encoding != RsdEncoding::PcmLittleEndian && !has_legacy_padding {
        return Err(RsdError::InvalidHeaderPadding);
    }
    Ok(match encoding {
        RsdEncoding::PcmLittleEndian | RsdEncoding::PcmBigEndian
            if !has_legacy_padding =>
        {
            COMPACT_DATA_OFFSET
        },
        _ => PADDED_DATA_OFFSET,
    })
}

/// Reads one fixed array with checked end arithmetic for every header field.
fn read_fixed_array<const SIZE: usize>(
    data: &[u8],
    pos: usize,
) -> Result<[u8; SIZE], RsdError> {
    let end = pos.checked_add(SIZE).ok_or(RsdError::TruncatedHeader)?;
    let bytes = data.get(pos..end).ok_or(RsdError::TruncatedHeader)?;
    let Ok(array) = <[u8; SIZE]>::try_from(bytes) else {
        return Err(RsdError::TruncatedHeader);
    };
    Ok(array)
}

/// Reads fixed little-endian header fields through checked slices.
fn read_u32(data: &[u8], pos: usize) -> Result<u32, RsdError> {
    Ok(u32::from_le_bytes(read_fixed_array::<4>(data, pos)?))
}

/// Verifies the fixed metadata region of one legacy padded header.
fn has_legacy_reserved_header(data: &[u8]) -> bool {
    data.get(MINIMUM_HEADER_SIZE..COMPACT_DATA_OFFSET)
        .is_some_and(|reserved| {
            reserved.iter().all(|byte| *byte == LEGACY_RESERVED_BYTE)
        })
}

/// Validates codec frame alignment and returns the complete frame size.
fn validate_payload_alignment(
    payload: &[u8],
    header: RsdHeader,
) -> Result<usize, RsdError> {
    let frame_size = match header.encoding {
        RsdEncoding::PcmLittleEndian | RsdEncoding::PcmBigEndian => {
            let bytes_per_sample = header.bits_per_sample.div_euclid(8_u16);
            usize::from(header.channels)
                .checked_mul(usize::from(bytes_per_sample))
        },
        RsdEncoding::RadicalAdpcm => RADP_FRAME_SIZE_PER_CHANNEL
            .checked_mul(usize::from(header.channels)),
    }
    .ok_or_else(|| RsdError::UnalignedPayload {
        encoding: header.encoding.name(),
        bytes: payload.len(),
        frame_size: usize::MAX,
    })?;
    if !payload.len().is_multiple_of(frame_size) {
        return Err(RsdError::UnalignedPayload {
            encoding: header.encoding.name(),
            bytes: payload.len(),
            frame_size,
        });
    }
    Ok(frame_size)
}

/// Copies native PCM only after the payload is frame-aligned.
fn validate_pcm_payload(
    payload: &[u8],
    header: RsdHeader,
) -> Result<Vec<u8>, RsdError> {
    let _frame_size = validate_payload_alignment(payload, header)?;
    let mut pcm = byte_buffer(payload.len())?;
    pcm.extend_from_slice(payload);
    Ok(pcm)
}

/// Swaps big-endian PCM only after the payload is known frame-aligned.
fn decode_big_endian_pcm(
    payload: &[u8],
    header: RsdHeader,
) -> Result<Vec<u8>, RsdError> {
    let mut pcm = validate_pcm_payload(payload, header)?;
    for sample in pcm.chunks_mut(2) {
        sample.swap(0, 1);
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
    let frame_size = validate_payload_alignment(payload, header)?;
    let frame_count = payload.len().div_euclid(frame_size);
    let capacity = frame_count
        .checked_mul(RADP_SAMPLES_PER_FRAME)
        .and_then(|value| value.checked_mul(channels))
        .and_then(|value| value.checked_mul(2_usize))
        .ok_or(RsdError::WavTooLarge(payload.len()))?;
    let mut pcm = byte_buffer(capacity)?;
    for frame in payload.chunks_exact(frame_size) {
        decode_radp_frame(frame, channels, &mut pcm)?;
    }
    Ok(pcm)
}

/// Reads one signed RADP header word without exposing raw frame indexing.
fn read_frame_i16(frame: &[u8], pos: usize) -> Result<i16, RsdError> {
    let end = pos.checked_add(2_usize).ok_or(RsdError::TruncatedData)?;
    let bytes = frame.get(pos..end).ok_or(RsdError::TruncatedData)?;
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
        let index = i32::from(read_frame_i16(frame, cursor)?);
        if !(0_i32..=88_i32).contains(&index) {
            return Err(RsdError::InvalidStepIndex(index));
        }
        let previous_offset =
            cursor.checked_add(2_usize).ok_or(RsdError::TruncatedData)?;
        let previous = i32::from(read_frame_i16(frame, previous_offset)?);
        states.push(RadpDecoder { index, previous });
        cursor = cursor.checked_add(4_usize).ok_or(RsdError::TruncatedData)?;
    }

    while cursor < frame.len() {
        let mut pairs = Vec::with_capacity(channels);
        for state in states.iter_mut().take(channels) {
            let byte = *frame.get(cursor).ok_or(RsdError::TruncatedData)?;
            cursor =
                cursor.checked_add(1_usize).ok_or(RsdError::TruncatedData)?;
            let first = decode_nibble(byte & 0x0f_u8, state)?;
            let second = decode_nibble((byte >> 4_u32) & 0x0f_u8, state)?;
            pairs.push((first, second));
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
fn decode_nibble(delta: u8, state: &mut RadpDecoder) -> Result<i16, RsdError> {
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
        .clamp(i32::from(i16::MIN), i32::from(i16::MAX));
    let delta_index = usize::from(delta);
    let Some(&adjustment) = INDEX_ADJUST_TABLE.get(delta_index) else {
        return Err(RsdError::InvalidStepIndex(state.index));
    };
    state.index = state.index.saturating_add(adjustment).clamp(0_i32, 88_i32);
    let Ok(sample) = i16::try_from(state.previous) else {
        return Err(RsdError::InvalidSample(state.previous));
    };
    Ok(sample)
}

#[cfg(test)]
#[path = "../../../../tests/formats/rsd/unit/domain/rsd/tests.rs"]
mod tests;

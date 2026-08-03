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
//   - Wav domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Wav domain module.
// - Description:
//   - Implements the declared domain module responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Wav domain module.

use super::{RsdError, byte_buffer};

/// WAV output is built in memory so filesystem adapters write only validated
/// bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
/// WAV model keeps decoded PCM plus the source-preserved playback parameters.
pub struct WavAudio {
    /// Channel count is copied from the validated RSD header.
    /// Channel count is copied from the RSD header to avoid remixing.
    pub channels: u16,
    /// Bit depth is copied from the validated RSD header.
    /// Bit depth is copied from the RSD header to avoid lossy conversion.
    pub bits_per_sample: u16,
    /// Sample rate is copied from the validated RSD header.
    /// Sample rate is copied from the RSD header to avoid resampling.
    pub sample_rate: u32,
    /// PCM bytes are owned so serialization cannot outlive decoded audio.
    /// PCM bytes are already decoded and ready for RIFF wrapping.
    pub pcm: Vec<u8>,
}

impl WavAudio {
    /// Fixed RIFF/WAVE header bytes before PCM sample data.
    pub(crate) const HEADER_BYTES: u64 = 44_u64;
    /// Smallest complete RIFF/WAVE file for one 16-bit mono sample frame.
    pub(crate) const MINIMUM_FILE_BYTES: u64 = Self::HEADER_BYTES + 2_u64;

    /// Rejects invalid public model values before RIFF fields are derived.
    fn validate_model(&self) -> Result<(), RsdError> {
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
        if self.pcm.is_empty() {
            return Err(RsdError::TruncatedData);
        }
        let bytes_per_sample = self.bits_per_sample.div_euclid(8_u16);
        let frame_size = usize::from(self.channels)
            .checked_mul(usize::from(bytes_per_sample))
            .ok_or(RsdError::UnalignedPayload {
                encoding: "WAV PCM",
                bytes: self.pcm.len(),
                frame_size: usize::MAX,
            })?;
        if !self.pcm.len().is_multiple_of(frame_size) {
            return Err(RsdError::UnalignedPayload {
                encoding: "WAV PCM",
                bytes: self.pcm.len(),
                frame_size,
            });
        }
        let Ok(block_align) = u32::try_from(frame_size) else {
            return Err(RsdError::UnsupportedSampleRate(self.sample_rate));
        };
        if self.sample_rate.checked_mul(block_align).is_none() {
            return Err(RsdError::UnsupportedSampleRate(self.sample_rate));
        }
        Ok(())
    }

    /// Serializes the RIFF wrapper around decoded PCM bytes.
    ///
    /// # Errors
    ///
    /// Returns an error when the PCM payload is too large for WAV length
    /// fields.
    /// Serializes the model as a RIFF/WAVE byte stream.
    ///
    /// # Errors
    ///
    /// Returns [`RsdError`] when the PCM payload is too large for RIFF sizes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, RsdError> {
        self.validate_model()?;
        let Ok(data_size) = u32::try_from(self.pcm.len()) else {
            return Err(RsdError::WavTooLarge(self.pcm.len()));
        };
        let riff_size = 36_u32
            .checked_add(data_size)
            .ok_or(RsdError::WavTooLarge(self.pcm.len()))?;
        let bytes_per_sample = self.bits_per_sample.div_euclid(8);
        let block_align = self
            .channels
            .checked_mul(bytes_per_sample)
            .ok_or(RsdError::WavTooLarge(self.pcm.len()))?;
        let byte_rate = self
            .sample_rate
            .checked_mul(u32::from(block_align))
            .ok_or(RsdError::UnsupportedSampleRate(self.sample_rate))?;
        let output_capacity = 44_usize
            .checked_add(self.pcm.len())
            .ok_or(RsdError::WavTooLarge(self.pcm.len()))?;
        let mut out = byte_buffer(output_capacity)?;
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&riff_size.to_le_bytes());
        out.extend_from_slice(b"WAVE");
        out.extend_from_slice(b"fmt ");
        out.extend_from_slice(&16_u32.to_le_bytes());
        out.extend_from_slice(&1_u16.to_le_bytes());
        out.extend_from_slice(&self.channels.to_le_bytes());
        out.extend_from_slice(&self.sample_rate.to_le_bytes());
        out.extend_from_slice(&byte_rate.to_le_bytes());
        out.extend_from_slice(&block_align.to_le_bytes());
        out.extend_from_slice(&self.bits_per_sample.to_le_bytes());
        out.extend_from_slice(b"data");
        out.extend_from_slice(&data_size.to_le_bytes());
        out.extend_from_slice(&self.pcm);
        Ok(out)
    }
}

#[cfg(test)]
#[path = "../../../../tests/formats/rsd/unit/domain/wav/tests.rs"]
mod tests;

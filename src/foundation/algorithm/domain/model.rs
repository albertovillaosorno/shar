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
//   - Pure mechanism settings and stable algorithm errors.
// - Must-Not:
//   - Own JSON serialization, filesystem effects, or product policy.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Algorithm domain records.
// - Description:
//   - Pure mechanism settings and stable algorithm errors.
// - Usage:
//   - Used through the owning algorithm function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Generic algorithm domain records.

use core::fmt;

const SETTINGS_SCHEMA: &str = "shar.algorithm.settings.v1";
const MAX_SOURCE_PROJECTION_ALTERNATIVES: usize = 256;

/// Public-safe positional projections for variant source files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceProjection {
    alternatives: Vec<SourceProjectionAlternative>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceProjectionAlternative {
    span_bytes: u64,
    mask: Vec<u8>,
}

impl SourceProjection {
    /// Builds one projection with a single offset mask.
    ///
    /// # Errors
    /// Returns an error when the span or mask is invalid.
    pub fn offset_mask(
        span_bytes: u64,
        mask: Vec<u8>,
    ) -> Result<Self, AlgorithmError> {
        Self::offset_masks(vec![(span_bytes, mask)])
    }

    /// Builds one projection from alternative offset masks.
    ///
    /// Each alternative must select the same positive number of bytes. Exact
    /// duplicate layouts are collapsed so multiple source editions can share
    /// one public projection alternative.
    ///
    /// # Errors
    /// Returns an error when no alternative exists, one span or mask is
    /// invalid, or alternatives select different byte counts.
    pub fn offset_masks(
        alternatives: Vec<(u64, Vec<u8>)>,
    ) -> Result<Self, AlgorithmError> {
        if alternatives.is_empty()
            || alternatives.len() > MAX_SOURCE_PROJECTION_ALTERNATIVES
        {
            return Err(AlgorithmError::new(
                "source projection alternative count is invalid",
            ));
        }
        let mut validated = Vec::with_capacity(alternatives.len());
        let mut selected_bytes = None;
        for (span_bytes, mask) in alternatives {
            let alternative =
                validate_projection_alternative(span_bytes, mask)?;
            let selected = projection_selected_bytes(&alternative.mask);
            if selected_bytes.is_some_and(|expected| expected != selected) {
                return Err(AlgorithmError::new(concat!(
                    "source projection alternatives must select equal ",
                    "byte counts",
                )));
            }
            selected_bytes = Some(selected);
            if !validated.contains(&alternative) {
                validated.push(alternative);
            }
        }
        Ok(Self { alternatives: validated })
    }

    /// Derives deterministic earliest-match layouts for one common byte
    /// sequence across one or more source variants.
    ///
    /// # Errors
    /// Returns an error when the common sequence or variants are empty, a
    /// variant does not contain the complete common sequence in order, or a
    /// resulting layout exceeds host limits.
    pub fn ordered_subsequence(
        common: &[u8],
        variants: &[&[u8]],
    ) -> Result<Self, AlgorithmError> {
        if common.is_empty() {
            return Err(AlgorithmError::new(
                "source projection common bytes must not be empty",
            ));
        }
        if variants.is_empty() {
            return Err(AlgorithmError::new(
                "source projection variants must not be empty",
            ));
        }
        let mut alternatives = Vec::with_capacity(variants.len());
        for variant in variants {
            let (span_bytes, mask) = ordered_subsequence_mask(common, variant)?;
            alternatives.push((span_bytes, mask));
        }
        Self::offset_masks(alternatives)
    }

    pub(crate) fn alternatives(&self) -> impl Iterator<Item = (u64, &[u8])> {
        self.alternatives.iter().map(|alternative| {
            (alternative.span_bytes, alternative.mask.as_slice())
        })
    }

    pub(crate) fn selected_bytes(&self) -> u64 {
        self.alternatives.first().map_or(0, |alternative| {
            projection_selected_bytes(&alternative.mask)
        })
    }

    pub(crate) fn mask_bytes(&self) -> u64 {
        self.alternatives.iter().fold(0_u64, |total, alternative| {
            let bytes =
                u64::try_from(alternative.mask.len()).unwrap_or(u64::MAX);
            total.saturating_add(bytes)
        })
    }
}

fn validate_projection_alternative(
    span_bytes: u64,
    mask: Vec<u8>,
) -> Result<SourceProjectionAlternative, AlgorithmError> {
    if span_bytes == 0 {
        return Err(AlgorithmError::new(
            "source projection span must be positive",
        ));
    }
    let expected =
        usize::try_from(span_bytes.div_ceil(8)).map_err(|_error| {
            AlgorithmError::new("source projection span is too large")
        })?;
    if mask.len() != expected {
        return Err(AlgorithmError::new(
            "source projection mask length does not match span",
        ));
    }
    let remainder = u8::try_from(span_bytes % 8).unwrap_or_default();
    if remainder != 0 {
        let invalid_bits = 8_u8.saturating_sub(remainder);
        let invalid_mask = (1_u8 << invalid_bits).saturating_sub(1);
        if mask.last().is_some_and(|byte| byte & invalid_mask != 0) {
            return Err(AlgorithmError::new(
                "source projection mask selects beyond its span",
            ));
        }
    }
    if mask.iter().all(|byte| *byte == 0) {
        return Err(AlgorithmError::new(
            "source projection must select at least one byte",
        ));
    }
    Ok(SourceProjectionAlternative { span_bytes, mask })
}

fn projection_selected_bytes(mask: &[u8]) -> u64 {
    mask.iter().map(|byte| u64::from(byte.count_ones())).sum()
}

fn ordered_subsequence_mask(
    common: &[u8],
    variant: &[u8],
) -> Result<(u64, Vec<u8>), AlgorithmError> {
    let mut cursor = 0_usize;
    let mut selected = Vec::new();
    for expected in common {
        let relative = variant
            .get(cursor..)
            .and_then(|remaining| {
                remaining.iter().position(|byte| byte == expected)
            })
            .ok_or_else(|| {
                AlgorithmError::new(
                    "source variant does not contain common bytes in order",
                )
            })?;
        let offset = cursor.checked_add(relative).ok_or_else(|| {
            AlgorithmError::new("source projection offset exceeds host limits")
        })?;
        selected.push(offset);
        cursor = offset.checked_add(1).ok_or_else(|| {
            AlgorithmError::new("source projection span exceeds host limits")
        })?;
    }
    let span_bytes = u64::try_from(cursor).map_err(|_error| {
        AlgorithmError::new("source projection span exceeds 64-bit limits")
    })?;
    let mut mask = vec![0_u8; cursor.div_ceil(8)];
    for offset in selected {
        let mask_index = offset.checked_div(8).ok_or_else(|| {
            AlgorithmError::new("source projection mask index is invalid")
        })?;
        let byte = mask.get_mut(mask_index).ok_or_else(|| {
            AlgorithmError::new("source projection mask index is invalid")
        })?;
        let bit_index = offset.checked_rem(8).ok_or_else(|| {
            AlgorithmError::new("source projection bit index is invalid")
        })?;
        let shift = 7_usize.checked_sub(bit_index).ok_or_else(|| {
            AlgorithmError::new("source projection bit index is invalid")
        })?;
        *byte |= 1_u8 << shift;
    }
    Ok((span_bytes, mask))
}

/// Generic authoring and replay resource limits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub(crate) schema: String,
    pub(crate) minimum_source_files: u64,
    pub(crate) minimum_source_bytes: u64,
    pub(crate) maximum_source_files: u64,
    pub(crate) maximum_target_files: u64,
    pub(crate) maximum_file_bytes: u64,
    pub(crate) maximum_source_bytes: u64,
    pub(crate) maximum_target_bytes: u64,
}

impl Settings {
    pub(crate) fn schema(&self) -> &str {
        &self.schema
    }

    /// Returns the minimum admitted source-file count.
    #[must_use]
    pub const fn minimum_source_files(&self) -> u64 {
        self.minimum_source_files
    }

    /// Returns the minimum admitted aggregate source byte count.
    #[must_use]
    pub const fn minimum_source_bytes(&self) -> u64 {
        self.minimum_source_bytes
    }

    pub(crate) const fn maximum_source_files(&self) -> u64 {
        self.maximum_source_files
    }

    pub(crate) const fn maximum_target_files(&self) -> u64 {
        self.maximum_target_files
    }

    pub(crate) const fn maximum_file_bytes(&self) -> u64 {
        self.maximum_file_bytes
    }

    pub(crate) const fn maximum_source_bytes(&self) -> u64 {
        self.maximum_source_bytes
    }

    pub(crate) const fn maximum_target_bytes(&self) -> u64 {
        self.maximum_target_bytes
    }

    pub(crate) fn validate(&self) -> Result<(), AlgorithmError> {
        if self.schema != SETTINGS_SCHEMA {
            return Err(AlgorithmError::new(format!(
                "unsupported settings schema: {}",
                self.schema
            )));
        }
        if self.minimum_source_files == 0 {
            return Err(AlgorithmError::new(
                "minimum_source_files must be positive",
            ));
        }
        if self.minimum_source_bytes == 0 {
            return Err(AlgorithmError::new(
                "minimum_source_bytes must be positive",
            ));
        }
        if self.maximum_source_files < self.minimum_source_files {
            return Err(AlgorithmError::new(
                "maximum_source_files must admit minimum_source_files",
            ));
        }
        if self.maximum_target_files == 0 {
            return Err(AlgorithmError::new(
                "maximum_target_files must be positive",
            ));
        }
        if self.maximum_file_bytes == 0 {
            return Err(AlgorithmError::new(
                "maximum_file_bytes must be positive",
            ));
        }
        if self.maximum_source_bytes < self.minimum_source_bytes {
            return Err(AlgorithmError::new(
                "maximum_source_bytes must admit minimum_source_bytes",
            ));
        }
        let source_capacity = self
            .maximum_source_files
            .saturating_mul(self.maximum_file_bytes);
        if source_capacity < self.minimum_source_bytes {
            return Err(AlgorithmError::new(
                "source file limits cannot satisfy minimum_source_bytes",
            ));
        }
        if self.maximum_target_bytes == 0 {
            return Err(AlgorithmError::new(
                "maximum_target_bytes must be positive",
            ));
        }
        Ok(())
    }
}

/// Stable algorithm engine error without private source contents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlgorithmError {
    message: String,
}

impl AlgorithmError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }
}

impl fmt::Display for AlgorithmError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AlgorithmError {}

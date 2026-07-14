// File:
//   - image_bmp.rs
// Path:
//   - src/p3d/src/adapters/driven/image_bmp.rs
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
//   - Pure structural validation for supported BMP file and DIB headers.
// - Must-Not:
//   - Read files, decode pixels, or classify non-BMP image containers.
// - Allows:
//   - Validate bounded BMP sizes, offsets, dimensions, planes, and depths.
// - Split-When:
//   - One DIB family requires independently versioned validation behavior.
// - Merge-When:
//   - BMP evidence no longer obscures the shared image classifier.
// - Summary:
//   - Validates supported BMP image headers.
// - Description:
//   - Provides bounded BMP structure evidence to the shared classifier.
// - Usage:
//   - Called by image.rs before publishing or reusing BMP payloads.
// - Defaults:
//   - Truncated, unsupported, or contradictory headers fail closed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Pure structural validation for supported BMP image headers.
//!
//! File-header and DIB-header evidence is decoded through checked offsets
//! before a payload can be classified as BMP.

/// Coherent size and offset evidence decoded from one BMP header.
struct BmpHeader {
    /// Total file size declared by the BMP file header.
    file_size: usize,
    /// First byte of encoded pixel data.
    pixel_offset: usize,
    /// DIB header size used to select the supported layout.
    dib_size: u32,
    /// First byte after the DIB header.
    dib_end: usize,
}

/// Returns whether one payload has coherent BMP file and DIB headers.
pub(super) fn looks_like_bmp(payload: &[u8]) -> bool {
    let Some(header) = parse_bmp_header(payload) else {
        return false;
    };
    let dib_size_supported = matches!(
        header.dib_size,
        12 | 40 | 52 | 56 | 64 | 108 | 124
    );
    dib_size_supported
        && bmp_dimensions_are_supported(
            payload,
            header.dib_size,
        )
        && bmp_plane_count_is_supported(
            payload,
            header.dib_size,
        )
        && bmp_pixel_depth_is_supported(
            payload,
            header.dib_size,
        )
        && bmp_compression_is_supported(
            payload,
            header.dib_size,
        )
        && bmp_image_size_is_supported(
            payload, &header,
        )
        && header.dib_end <= header.pixel_offset
        && header.pixel_offset < header.file_size
        && header.file_size == payload.len()
}

/// Returns whether one BMP image size fits the available pixel payload.
fn bmp_image_size_is_supported(
    payload: &[u8],
    header: &BmpHeader,
) -> bool {
    if header.dib_size == 12 {
        return true;
    }
    let Some(image_size_u32) = read_u32_at(
        payload, 34,
    ) else {
        return false;
    };
    let Ok(image_size) = usize::try_from(image_size_u32) else {
        return false;
    };
    let Some(compression) = read_u32_at(
        payload, 30,
    ) else {
        return false;
    };
    if matches!(
        compression,
        1_u32 | 2 | 4 | 5
    ) && image_size == 0
    {
        return false;
    }
    let available_size_result = header
        .file_size
        .checked_sub(header.pixel_offset);
    let Some(available_size) = available_size_result else {
        return false;
    };
    image_size <= available_size
}

/// Returns whether one BMP DIB header uses a known compression mode.
fn bmp_compression_is_supported(
    payload: &[u8],
    dib_size: u32,
) -> bool {
    if dib_size == 12 {
        return true;
    }
    let Some(compression) = read_u32_at(
        payload, 30,
    ) else {
        return false;
    };
    let Some(pixel_depth) = read_u16_at(
        payload, 28,
    ) else {
        return false;
    };
    let Some(height) = read_i32_at(
        payload, 22,
    ) else {
        return false;
    };
    if height < 0_i32
        && !matches!(
            compression,
            0_u32 | 3
        )
    {
        return false;
    }
    match compression {
        0 | 4 | 5 => true,
        1 => pixel_depth == 8,
        2 => pixel_depth == 4,
        3 => matches!(
            pixel_depth,
            16 | 32
        ),
        _ => false,
    }
}

/// Returns whether one BMP DIB header uses a supported pixel depth.
fn bmp_pixel_depth_is_supported(
    payload: &[u8],
    dib_size: u32,
) -> bool {
    let depth_offset = if dib_size == 12 {
        24
    } else {
        28
    };
    let Some(pixel_depth) = read_u16_at(
        payload,
        depth_offset,
    ) else {
        return false;
    };
    matches!(
        pixel_depth,
        1 | 4 | 8 | 16 | 24 | 32
    )
}

/// Returns whether one BMP DIB header declares exactly one color plane.
fn bmp_plane_count_is_supported(
    payload: &[u8],
    dib_size: u32,
) -> bool {
    let plane_offset = if dib_size == 12 {
        22
    } else {
        26
    };
    let Some(plane_count) = read_u16_at(
        payload,
        plane_offset,
    ) else {
        return false;
    };
    plane_count == 1
}

/// Returns whether one supported BMP DIB header has usable dimensions.
fn bmp_dimensions_are_supported(
    payload: &[u8],
    dib_size: u32,
) -> bool {
    if dib_size == 12 {
        let Some(width) = read_u16_at(
            payload, 18,
        ) else {
            return false;
        };
        let Some(height) = read_u16_at(
            payload, 20,
        ) else {
            return false;
        };
        return width > 0 && height > 0;
    }
    let Some(width) = read_i32_at(
        payload, 18,
    ) else {
        return false;
    };
    let Some(height) = read_i32_at(
        payload, 22,
    ) else {
        return false;
    };
    width > 0 && height != 0
}

/// Decodes checked BMP size and offset fields from one candidate payload.
fn parse_bmp_header(payload: &[u8]) -> Option<BmpHeader> {
    if payload.len() < 26 || !payload.starts_with(b"BM") {
        return None;
    }
    let file_size_u32 = read_u32_at(
        payload, 2,
    )?;
    let pixel_offset_u32 = read_u32_at(
        payload, 10,
    )?;
    let dib_size = read_u32_at(
        payload, 14,
    )?;
    let file_size = usize::try_from(file_size_u32).ok()?;
    let pixel_offset = usize::try_from(pixel_offset_u32).ok()?;
    let dib_size_usize = usize::try_from(dib_size).ok()?;
    let dib_end = 14_usize.checked_add(dib_size_usize)?;
    Some(
        BmpHeader {
            file_size,
            pixel_offset,
            dib_size,
            dib_end,
        },
    )
}

/// Reads one checked little-endian 16-bit field.
fn read_u16_at(
    payload: &[u8],
    offset: usize,
) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let slice = payload.get(offset..end)?;
    Some(
        u16::from_le_bytes(
            slice
                .try_into()
                .ok()?,
        ),
    )
}

/// Reads one checked little-endian signed 32-bit field.
fn read_i32_at(
    payload: &[u8],
    offset: usize,
) -> Option<i32> {
    let end = offset.checked_add(4)?;
    let slice = payload.get(offset..end)?;
    Some(
        i32::from_le_bytes(
            slice
                .try_into()
                .ok()?,
        ),
    )
}

/// Reads one checked little-endian 32-bit field.
fn read_u32_at(
    payload: &[u8],
    offset: usize,
) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let slice = payload.get(offset..end)?;
    Some(
        u32::from_le_bytes(
            slice
                .try_into()
                .ok()?,
        ),
    )
}
#[cfg(test)]
#[path = "image_bmp_tests.rs"]
mod tests;

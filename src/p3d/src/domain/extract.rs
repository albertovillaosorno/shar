// File:
//   - extract.rs
// Path:
//   - src/p3d/src/domain/extract.rs
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
//   - Pure p3d domain rules for domain extract.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when extract contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Returns an error when source parsing or filesystem output fails.
// - Description:
//   - Defines extract data and behavior for p3d domain.
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
//   - Reason: src/p3d/src/domain/extract.rs has 405 effective lines after the
//   - required header and remains cohesive until a focused split lands.
//

//! Returns an error when source parsing or filesystem output fails.
// This parser performs byte-accurate offset arithmetic and slicing at one
// boundary until chunk-family decoders replace the compatibility code.
#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::as_conversions,
    clippy::default_numeric_fallback,
    reason = "P3D binary parser code mirrors fixed on-disk offsets and \
              generated chunk taxonomy; follow-up tranche work replaces stubs \
              with typed decoders."
)]
use std::borrow::Cow;

use super::chunk::P3dError;

#[derive(Debug, Clone)]
/// Prepared P3D.
pub struct PreparedP3d<'a> {
    /// Bytes.
    pub bytes: Cow<'a, [u8]>,
    /// Compression.
    pub compression: &'static str,
}

/// Prepare p3d bytes.
///
/// # Errors
///
/// Returns an error when source parsing or filesystem output fails.
pub fn prepare_p3d_bytes(source: &[u8]) -> Result<PreparedP3d<'_>, P3dError> {
    if source.len() < 12 {
        return Err(
            P3dError::invalid_source("file is too small for a P3D header"),
        );
    }
    let magic = source
        .get(0..4)
        .ok_or_else(|| P3dError::invalid_source("missing P3D magic"))?;
    if magic
        == [
            0x50, 0x33, 0x44, 0xFF,
        ]
        || magic
            == [
                0xFF, 0x44, 0x33, 0x50,
            ]
    {
        return Ok(
            PreparedP3d {
                bytes: Cow::Borrowed(source),
                compression: "none",
            },
        );
    }
    if magic
        == [
            0x50, 0x33, 0x44, 0x5A,
        ]
    {
        return Ok(
            PreparedP3d {
                bytes: Cow::Owned(decompress_p3dz(source)?),
                compression: "p3dz_lzrf",
            },
        );
    }
    Err(P3dError::invalid_source("unsupported Pure3D root magic"))
}

/// Decompress p3dz.
fn decompress_p3dz(source: &[u8]) -> Result<Vec<u8>, P3dError> {
    let expected_size = read_u32(
        source, 4,
    )? as usize;
    let mut cursor = 8;
    let mut output = Vec::with_capacity(expected_size);
    while output.len() < expected_size {
        let compressed_size = read_u32(
            source, cursor,
        )? as usize;
        cursor += 4;
        let decompressed_size = read_u32(
            source, cursor,
        )? as usize;
        cursor += 4;
        if compressed_size == 0 || decompressed_size == 0 {
            return Err(
                P3dError::invalid_source("P3DZ blocks must have nonzero sizes"),
            );
        }
        let remaining_size = expected_size
            .checked_sub(output.len())
            .ok_or_else(
                || P3dError::invalid_source("P3DZ output size overflow"),
            )?;
        if decompressed_size > remaining_size {
            return Err(
                P3dError::invalid_source(
                    "P3DZ block exceeds declared output size",
                ),
            );
        }
        let end = cursor
            .checked_add(compressed_size)
            .ok_or_else(
                || P3dError::invalid_source("P3DZ compressed block overflow"),
            )?;
        let block = source
            .get(cursor..end)
            .ok_or_else(
                || {
                    P3dError::invalid_source(
                        "P3DZ compressed block out of bounds",
                    )
                },
            )?;
        let mut decompressed = vec![0_u8; decompressed_size];
        if lzrf_decompress(
            block,
            &mut decompressed,
        )
        .is_err()
        {
            decompressed.fill(0);
            lzr_decompress(
                block,
                &mut decompressed,
            )?;
        }
        output.extend_from_slice(&decompressed);
        cursor = end;
    }
    if output.len() != expected_size {
        return Err(
            P3dError::invalid_source("P3DZ decompressed size mismatch"),
        );
    }
    if cursor != source.len() {
        return Err(
            P3dError::invalid_source("trailing bytes after P3DZ block stream"),
        );
    }
    if output.get(0..4)
        != Some(
            &[
                0x50, 0x33, 0x44, 0xFF,
            ],
        )
        && output.get(0..4)
            != Some(
                &[
                    0xFF, 0x44, 0x33, 0x50,
                ],
            )
    {
        return Err(
            P3dError::invalid_source("P3DZ payload did not decompress to P3D"),
        );
    }
    Ok(output)
}

/// Read u32.
fn read_u32(
    source: &[u8],
    offset: usize,
) -> Result<u32, P3dError> {
    let end = offset
        .checked_add(4)
        .ok_or_else(
            || P3dError::invalid_source("P3DZ header offset overflow"),
        )?;
    let bytes = source
        .get(offset..end)
        .ok_or_else(
            || P3dError::invalid_source("unexpected end of P3DZ header"),
        )?;
    Ok(
        u32::from_le_bytes(
            [
                bytes[0], bytes[1], bytes[2], bytes[3],
            ],
        ),
    )
}

/// Read and advance one compressed-stream byte.
fn take_stream_byte(
    input: &[u8],
    cursor: &mut usize,
) -> Option<u8> {
    let value = *input.get(*cursor)?;
    *cursor = cursor.checked_add(1)?;
    Some(value)
}

/// Reject compressed blocks that leave ambiguous trailing bytes.
fn require_complete_stream(
    consumed: usize,
    total: usize,
    format_name: &str,
) -> Result<(), P3dError> {
    if consumed == total {
        return Ok(());
    }
    Err(
        P3dError::invalid_source(
            format!("trailing bytes after {format_name} stream"),
        ),
    )
}

/// Lzrf decompress.
fn lzrf_decompress(
    input: &[u8],
    output: &mut [u8],
) -> Result<(), P3dError> {
    let mut input_cursor = 0_usize;
    let mut output_cursor = 0_usize;
    while output_cursor < output.len() {
        let code = take_stream_byte(
            input,
            &mut input_cursor,
        )
        .ok_or_else(
            || P3dError::invalid_source("unexpected end of LZRF stream"),
        )?;
        if code & 0x80 != 0 {
            decompress_lzrf_match(
                input,
                &mut input_cursor,
                output,
                &mut output_cursor,
                code,
            )?;
        } else {
            decompress_lzrf_literals(
                input,
                &mut input_cursor,
                output,
                &mut output_cursor,
                code,
            )?;
        }
    }
    require_complete_stream(
        input_cursor,
        input.len(),
        "LZRF",
    )
}

/// Decode one LZRF match command.
fn decompress_lzrf_match(
    input: &[u8],
    input_cursor: &mut usize,
    output: &mut [u8],
    output_cursor: &mut usize,
    code: u8,
) -> Result<(), P3dError> {
    let mut match_length = usize::from(code & 0x7f);
    if match_length == 0 {
        match_length += 127;
        loop {
            let value = take_stream_byte(
                input,
                input_cursor,
            )
            .ok_or_else(
                || {
                    P3dError::invalid_source(
                        "unexpected end of LZRF match length",
                    )
                },
            )?;
            if value != 0 {
                match_length += usize::from(value);
                break;
            }
            match_length += 255;
        }
    }
    let offset_code = take_stream_byte(
        input,
        input_cursor,
    )
    .ok_or_else(|| P3dError::invalid_source("unexpected end of LZRF offset"))?;
    let offset = if offset_code & 0x80 != 0 {
        let high = take_stream_byte(
            input,
            input_cursor,
        )
        .ok_or_else(
            || P3dError::invalid_source("unexpected end of LZRF long offset"),
        )?;
        (usize::from(high) << 4) + usize::from(offset_code & 0x7f)
    } else {
        usize::from(offset_code)
    };
    copy_match(
        output,
        output_cursor,
        offset,
        match_length,
    )
}

/// Decode one LZRF literal command.
fn decompress_lzrf_literals(
    input: &[u8],
    input_cursor: &mut usize,
    output: &mut [u8],
    output_cursor: &mut usize,
    code: u8,
) -> Result<(), P3dError> {
    let mut run_length = usize::from(code);
    if run_length == 0 {
        loop {
            let value = take_stream_byte(
                input,
                input_cursor,
            )
            .ok_or_else(
                || {
                    P3dError::invalid_source(
                        "unexpected end of LZRF literal length",
                    )
                },
            )?;
            if value != 0 {
                run_length += usize::from(value);
                break;
            }
            run_length += 255;
        }
        copy_literals(
            input,
            input_cursor,
            output,
            output_cursor,
            127,
        )?;
    }
    copy_literals(
        input,
        input_cursor,
        output,
        output_cursor,
        run_length,
    )
}

/// Copy literals.
fn copy_literals(
    input: &[u8],
    input_cursor: &mut usize,
    output: &mut [u8],
    output_cursor: &mut usize,
    count: usize,
) -> Result<(), P3dError> {
    let input_end = input_cursor
        .checked_add(count)
        .ok_or_else(
            || P3dError::invalid_source("LZRF literal input overflow"),
        )?;
    let output_end = output_cursor
        .checked_add(count)
        .ok_or_else(
            || P3dError::invalid_source("LZRF literal output overflow"),
        )?;
    let input_slice = input
        .get(*input_cursor..input_end)
        .ok_or_else(
            || P3dError::invalid_source("LZRF literal input out of bounds"),
        )?;
    let output_slice = output
        .get_mut(*output_cursor..output_end)
        .ok_or_else(
            || P3dError::invalid_source("LZRF literal output out of bounds"),
        )?;
    output_slice.copy_from_slice(input_slice);
    *input_cursor = input_end;
    *output_cursor = output_end;
    Ok(())
}

/// Copy match.
fn copy_match(
    output: &mut [u8],
    output_cursor: &mut usize,
    offset: usize,
    count: usize,
) -> Result<(), P3dError> {
    if offset == 0 || offset > *output_cursor {
        return Err(P3dError::invalid_source("invalid LZRF match offset"));
    }
    for _ in 0..count {
        let value = output[*output_cursor - offset];
        let slot = output
            .get_mut(*output_cursor)
            .ok_or_else(
                || P3dError::invalid_source("LZRF match output out of bounds"),
            )?;
        *slot = value;
        *output_cursor += 1;
    }
    Ok(())
}

/// Lzr decompress.
fn lzr_decompress(
    input: &[u8],
    output: &mut [u8],
) -> Result<(), P3dError> {
    let mut input_cursor = 0_usize;
    let mut output_cursor = 0_usize;
    while output_cursor < output.len() {
        let code = take_stream_byte(
            input,
            &mut input_cursor,
        )
        .ok_or_else(
            || P3dError::invalid_source("unexpected end of LZR stream"),
        )?;
        if code > 15 {
            let mut match_length = usize::from(code & 15);
            if match_length == 0 {
                match_length += 15;
                loop {
                    let value = take_stream_byte(
                        input,
                        &mut input_cursor,
                    )
                    .ok_or_else(
                        || {
                            P3dError::invalid_source(
                                "unexpected end of LZR match length",
                            )
                        },
                    )?;
                    if value != 0 {
                        match_length += usize::from(value);
                        break;
                    }
                    match_length += 255;
                }
            }
            let next = usize::from(
                take_stream_byte(
                    input,
                    &mut input_cursor,
                )
                .ok_or_else(
                    || P3dError::invalid_source("unexpected end of LZR offset"),
                )?,
            );
            let offset = usize::from(code >> 4) | (next << 4);
            copy_match(
                output,
                &mut output_cursor,
                offset,
                match_length,
            )?;
        } else {
            let mut run_length = usize::from(code);
            if run_length == 0 {
                loop {
                    let value = take_stream_byte(
                        input,
                        &mut input_cursor,
                    )
                    .ok_or_else(
                        || {
                            P3dError::invalid_source(
                                "unexpected end of LZR literal length",
                            )
                        },
                    )?;
                    if value != 0 {
                        run_length += usize::from(value);
                        break;
                    }
                    run_length += 255;
                }
                copy_literals(
                    input,
                    &mut input_cursor,
                    output,
                    &mut output_cursor,
                    15,
                )?;
            }
            copy_literals(
                input,
                &mut input_cursor,
                output,
                &mut output_cursor,
                run_length,
            )?;
        }
    }
    require_complete_stream(
        input_cursor,
        input.len(),
        "LZR",
    )
}

#[cfg(test)]
#[test]
fn p3dz_header_reader_rejects_offset_overflow() -> Result<(), String> {
    let read = read_u32(
        &[],
        usize::MAX,
    );
    if read.is_ok() {
        return Err(
            String::from(
                "P3DZ header reads must reject an offset that cannot contain \
                 a u32",
            ),
        );
    }
    Ok(())
}

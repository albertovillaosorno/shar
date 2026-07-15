# DDS, TGA, And BMP Raster Texture Inputs

This non-governing record documents a related family of raster texture inputs
without granting rights in source artwork, compression technology, metadata, or
third-party decoders.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository references verified;
  authoritative DDS and preservation references identified; exact TGA variant
  evidence remains sample-specific.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local texture decoding and migration input.
- Subject class: Raster texture container and pixel-storage family.

## Covered Material

DirectDraw Surface files, Truevision TGA files, and Windows bitmap files to the
extent encountered or recognized by SHAR texture pipelines, including headers,
dimensions, pixel formats, compression identifiers, palettes, alpha, mipmaps,
and row-order conventions.

## Repository Use And Scope

SHAR may identify or decode these formats while normalizing user-supplied local
texture data to PNG and embedding reviewed media in FBX output. Support for one
variant does not imply complete support for every header revision, compression
mode, color layout, or extension block.

## Provenance And Version History

DDS is a Microsoft DirectX container with documented programming guidance and
format tables. BMP is a Microsoft and Windows bitmap family with multiple header
revisions. TGA originated with Truevision and has multiple commonly encountered
variants. Exact sample headers and compression modes control.

## Authorship, Ownership, And Attribution

Microsoft, Truevision successors, specification authors, decoder authors, and
other contributors retain applicable rights in their materials. Rights in the
stored artwork remain separate. SHAR contributors retain rights in independently
authored parsing and normalization code.

## License Or Terms Basis

Public format documentation does not license source textures, trademarks,
compression patents, third-party libraries, or copied implementation code. Each
decoder and bundled dependency requires separate license review.

## Distribution, Modification, And Compatibility

A successfully decoded texture may remain protected game artwork. Converting it
to PNG or embedding it in FBX changes representation, not ownership.
Compressed-size, decompressed-size, mipmap, pitch, and dimension calculations
must fail closed on malformed input.

## Compliance Posture

- Record the exact source extension, header variant, pixel format, and hash.
- Validate dimensions, row pitch, mip levels, palette bounds, and output size.
- Preserve unknown metadata as evidence or reject it; do not invent semantics.
- Keep original and normalized protected textures outside public publication
  and distribution surfaces.
- Review compression and decoder licensing before redistribution.

## Technical Baseline And SHAR Profile

### DDS and BMP public baseline

Microsoft documents DDS as a binary texture container beginning with the `DDS`
magic value and a `DDS_HEADER` plus `DDS_PIXELFORMAT`, with an optional
`DDS_HEADER_DXT10` extension. The documented family includes uncompressed and
block-compressed textures, mipmaps, cube maps, volume textures, and texture
arrays, subject to header and consumer differences.

Microsoft's Windows Imaging Component identifies BMP/DIB as the Windows Bitmap
Format and cites BMP Specification v5 for its native codec profile.

### TGA evidence boundary

No current first-party TGA-owner specification suitable for an acceptance-grade
normative citation has been verified. This record therefore does not claim one
universal TGA contract or imply that historical third-party summaries are
controlling specifications.

### Use-specific decoder evidence

Before claiming support for any raster family, maintain a test-backed profile of
accepted magic values, header versions, pixel formats, compression modes, row
order, alpha interpretation, palettes, mipmaps, dimensions, allocation limits,
malformed input, and unsupported variants. DDS and BMP documentation does not
establish the repository's actual decoder subset, and TGA remains outside the
verified subset until equivalent evidence exists.

### Verified sources

- Microsoft (2020), *Programming Guide for DDS*.
  <!-- markdownlint-disable-next-line MD013 -->
  <https://learn.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds-pguide>
- Microsoft, *BMP Format Overview*.
  <https://learn.microsoft.com/en-us/windows/win32/wic/bmp-format-overview>

## Source References

- Microsoft (n.d.) *Programming Guide for DDS*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://learn.microsoft.com/en-us/windows/win32/direct3ddds/dx-graphics-dds-pguide>
  (Accessed: 12 July 2026).
- Microsoft (n.d.) *Bitmap Storage*. Available at:
  <https://learn.microsoft.com/en-us/windows/win32/gdi/bitmap-storage>
  (Accessed: 12 July 2026).
- Library of Congress (n.d.) *Truevision TGA, version 2.0*. Available at:
  <https://www.loc.gov/preservation/digital/formats/fdd/fdd000180.shtml>
  (Accessed: 12 July 2026).
- SHAR repository (2026), texture and image-data decoding contracts.

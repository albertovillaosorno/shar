# Portable Network Graphics

This non-governing record documents a W3C raster-image standard without granting
rights in image content, metadata, encoders, or embedded media.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository use and the current W3C Recommendation
  verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Texture interchange and embedded-media output.
- Subject class: W3C lossless raster-image standard.

## Covered Material

PNG files emitted or embedded by SHAR for decoded textures, review artifacts,
and FBX media, including the signature, chunks, dimensions, color information,
transparency, checksums, and compressed image data.

## Repository Use And Scope

SHAR produces PNG texture artifacts from independently decoded image data and
may embed them in FBX output. The repository must preserve pixel provenance and
must not imply rights in source textures merely because they were re-encoded.

## Provenance And Version History

The W3C published the PNG Third Edition as a Recommendation in 2025. A SHAR
encoder or decoder may intentionally support only a bounded subset, so each
implementation must record supported color types, bit depths, chunks, and
metadata rather than claim complete conformance.

## Authorship, Ownership, And Attribution

The W3C and specification contributors retain applicable rights in the standard
publication. Implementations and image content retain separate rights. SHAR
contributors retain rights in independently authored serializers and validators.

## License Or Terms Basis

The PNG specification is publicly available under W3C document terms. Use of the
format does not license image content, third-party encoders, compression
libraries, trademarks, or ancillary metadata.

## Distribution, Modification, And Compatibility

A valid PNG may still contain protected artwork, personal data, misleading
metadata, or unsupported color information. Re-encoding does not eliminate
rights in the source pixels or make an extracted game texture distributable.

## Compliance Posture

- Validate signatures, chunk lengths, CRCs, dimensions, and decompressed bounds.
- Record supported color types, bit depths, and ancillary chunks.
- State whether metadata is preserved, normalized, or intentionally omitted.
- Keep original game textures outside Git unless publication rights exist.
- Treat PNG content embedded in FBX as separately reviewable material.

## Technical Baseline And SHAR Profile

### Public baseline

The W3C *Portable Network Graphics (PNG) Specification (Third Edition)* is a W3C
Recommendation dated 24 June 2025. It defines the PNG datastream, `image/png`
and `image/apng`, the permitted IHDR color-type and sample-depth combinations,
and compression method 0.

### SHAR profile

The current repository classifies `.png` as a generated-image extension, but the
reviewed authored source did not expose a repository-owned production PNG
encoder or a byte-level emitted-chunk contract. SHAR therefore makes no claim of
full PNG encoder conformance and does not infer an ancillary-chunk, metadata,
color-management, APNG, or interlace profile from the filename alone.

### Acceptance boundary

An emitted-output claim must identify the exact encoder, color types, sample
depths, filter and interlace behavior, compression implementation, critical and
ancillary chunks, metadata policy, maximum dimensions, and conformance fixtures.
Until that evidence exists, the standard is a public reference rather than proof
of emitted-output compliance.

### Verified sources

- W3C (2025), *Portable Network Graphics (PNG) Specification (Third Edition)*.
  <https://www.w3.org/TR/png-3/>
- SHAR repository evidence: `src/game-manifest/src/domain/domain.rs`.

## Source References

- W3C (2025) *Portable Network Graphics PNG Specification, Third Edition*.
  Available at: <https://www.w3.org/TR/png/> (Accessed: 12 July 2026).
- SHAR repository (2026), texture decoding, PNG output, and FBX embedded-media
  contracts.

# RIFF, WAVE, And PCM

This non-governing record documents an audio container and sample-format family
without granting rights in source recordings, performances, codecs, or converted
media.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository serializer behavior and authoritative
  Microsoft and preservation references verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Deterministic local audio migration output.
- Subject class: RIFF-based audio container and pulse-code modulation formats.

## Covered Material

RIFF WAVE files emitted by `src/rsd/` and the pipeline, including the `RIFF` and
`WAVE` identifiers, `fmt` and `data` chunks, channel count, sample rate, byte
rate, block alignment, sample width, PCM frames, padding, and file-size limits.

## Repository Use And Scope

SHAR converts supported user-supplied RSD audio into deterministic PCM WAVE
files and emits numbered WAVE tracks alongside converted cinematics. It does not
claim ownership of source audio, voices, music, or performances.

## Provenance And Version History

RIFF originated as an IBM and Microsoft multimedia container family. WAVE uses a
RIFF form type and registered audio-format structures. The repository currently
emits bounded PCM output rather than claiming support for every WAVE codec,
extension, metadata chunk, RF64 variant, or platform convention.

## Authorship, Ownership, And Attribution

Microsoft, IBM, standards contributors, and codec authors retain applicable
rights in specifications and implementations. Rights in the recorded content
remain with the relevant authors, performers, producers, publishers, and other
rights holders. SHAR contributors retain rights in independently authored
serialization code.

## License Or Terms Basis

Public documentation of RIFF and WAVE structures does not license source audio,
codec patents, trademarks, third-party libraries, or protected recordings. The
exact codec and distribution context control.

## Distribution, Modification, And Compatibility

A valid WAVE container may contain content that cannot lawfully be distributed.
Conversion to PCM changes encoding, not ownership. Size arithmetic, padding,
frame alignment, and sample metadata must be validated before publication.

## Compliance Posture

- Emit only supported PCM forms and fail closed on impossible metadata.
- Validate RIFF sizes, chunk padding, block alignment, byte rate, and frame
  count.
- Record channels, sample rate, sample width, source hash, and output hash.
- Keep original and converted protected recordings outside public publication
  and distribution surfaces.
- Review patent and codec issues separately for non-PCM formats.

## Technical Baseline And SHAR Profile

### Public baseline

Microsoft documents WAVE audio as a RIFF chunked format. The common `RIFF`,
`fmt`, and `data` chunks identify the WAVE container, format description, and
audio payload. RIFF chunk data is padded to a WORD boundary, while the stored
chunk size excludes padding. `WAVEFORMATEX` and `WAVEFORMATEXTENSIBLE` define
application profiles beyond the basic PCM header.

### SHAR writer profile

The current RSD writer emits a deliberately narrow classic PCM WAVE profile:

- a fixed 44-byte `RIFF`/`WAVE` header;
- one 16-byte `fmt` chunk;
- format tag `1` for linear PCM;
- 16-bit samples only;
- 1 through 16 channels copied from validated source evidence;
- a nonzero sample rate representable as a signed 32-bit value;
- little-endian `byte_rate`, `block_align`, and size fields;
- one `data` chunk containing nonempty frame-aligned PCM bytes; and
- checked 32-bit RIFF and data-size arithmetic.

The writer does not currently emit `WAVEFORMATEXTENSIBLE`, channel masks,
metadata chunks, fact chunks, broadcast metadata, or non-PCM codecs. Because
16-bit frame sizes are even, accepted payloads are already WORD-aligned.

### Use-specific evidence limits

Before expanding beyond the current basic PCM profile or declaring compatibility
with another consumer, define the maximum-file policy, record the consumer
matrix, and decide whether multichannel output requires
`WAVEFORMATEXTENSIBLE` with a channel mask.

### Verified sources

- Microsoft (2021), *Resource Interchange File Format (RIFF)*.
  <!-- markdownlint-disable-next-line MD013 -->
  <https://learn.microsoft.com/en-us/windows/win32/xaudio2/resource-interchange-file-format--riff->
- Microsoft, *WAVEFORMATEXTENSIBLE structure*.
  <!-- markdownlint-disable-next-line MD013 -->
  <https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatextensible>
- SHAR repository evidence: `src/rsd/src/domain/wav.rs` and
  `src/rsd/tests/wav_validation.rs`.

## Source References

- Microsoft (n.d.) *WAVEFORMATEX structure*. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformatex>
  (Accessed: 12 July 2026).
- Library of Congress (n.d.) *WAVE Audio File Format with LPCM Audio*. Available
  at: <https://www.loc.gov/preservation/digital/formats/fdd/fdd000001.shtml>
  (Accessed: 12 July 2026).
- SHAR repository (2026) `src/rsd/`, WAVE validation tests, and media pipeline.

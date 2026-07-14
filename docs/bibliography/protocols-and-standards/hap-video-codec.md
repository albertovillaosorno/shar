# HAP Video Codec

This non-governing record documents a GPU-oriented video codec family without
granting rights in source cinematics, audio, codec implementations, or converted
media.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository use and the official Hap reference
  repository verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local cinematic migration output.
- Subject class: Openly documented GPU-oriented video codec family.

## Covered Material

Hap, Hap Alpha, Hap Q, and related codec identifiers to the extent used or
recognized by SHAR, together with container, frame, pixel-format, timing, and
decoder-compatibility evidence.

## Repository Use And Scope

SHAR converts supported user-supplied cinematic input into HAP video packages
with synchronized WAVE tracks by invoking external FFmpeg tooling. The
repository does not implement the codec from copied third-party source and does
not publish original cinematics.

## Provenance And Version History

Vidvox publishes the Hap codec project, implementation references, and related
technical material. FFmpeg support is a separate implementation and build
configuration. A working FFmpeg command does not prove compatibility with every
Hap variant, container, playback engine, or GPU path.

## Authorship, Ownership, And Attribution

Vidvox and contributors retain applicable rights in Hap project material. FFmpeg
and linked-library contributors retain rights in their implementations. Rights
in the source and converted audiovisual content remain separate.

## License Or Terms Basis

The exact Hap reference implementation and FFmpeg build licenses control any
redistribution of codec software. Use of the codec does not license source
cinematics, performances, music, trademarks, or patented technology that might
apply in a particular implementation or jurisdiction.

## Distribution, Modification, And Compatibility

Transcoding changes representation, not ownership. A valid HAP stream may still
contain protected audiovisual expression. Codec and container compatibility,
patent exposure, FFmpeg GPL obligations, and rights in the content must be
reviewed separately.

## Compliance Posture

- Record the exact FFmpeg build, configuration, command, codec profile, and
  hash.
- Validate dimensions, frame rate, duration, pixel format, and frame count.
- Keep source and converted protected cinematics outside public publication and
  distribution surfaces.
- Preserve synchronized-audio provenance independently from video provenance.
- Recheck codec, patent, and implementation-license information before bundling.

## Technical Baseline And SHAR Profile

### Public baseline

The Vidvox `hap` repository identifies itself as the HAP specification and
reference source. It is publicly available under the BSD-2-Clause license. That
source is suitable for precise implementation and interoperability statements,
subject to the exact revision and codec variant used.

### SHAR profile

SHAR treats HAP as the reproducible default cinematic target when the official
Bink 2 encoder is unavailable. The current package plan specifies:

- a QuickTime movie path named `movie.mov`;
- HAP Q as the requested video format (`hap_q`);
- separately generated PCM WAVE audio tracks named with the
  `audio_track_%02d.wav` pattern;
- source-probe, decode-report, manifest, and timing evidence; and
- an optional `movie.bk2` path kept separate from the HAP package.

This is a planning and packaging contract. It is not, by itself, proof that a
particular FFmpeg build emitted a conforming HAP stream or that every decoder
will accept the result.

### Use-specific evidence limits

Before approving HAP output for a supported workflow, capture the exact
specification revision, encoder identity and build configuration, frame and
alpha variant, container metadata, timebase, audio-synchronization rules,
deterministic command evidence, and decoder-backed sample tests.

### Verified sources

- Vidvox, *HAP specification and reference source*.
  <https://github.com/Vidvox/hap>
- SHAR repository evidence: `src/rmv/src/domain/target.rs` and
  `src/rmv/src/application/package_plan.rs`.

## Source References

- Vidvox (n.d.) *Hap video codec official GitHub repository*. Available at:
  <https://github.com/Vidvox/hap> (Accessed: 12 July 2026).
- FFmpeg Project (n.d.) *FFmpeg documentation*. Available at:
  <https://ffmpeg.org/documentation.html> (Accessed: 12 July 2026).
- SHAR repository (2026), RMV conversion and HAP packaging contracts.

# Bink Video

This non-governing record documents a proprietary video codec and file family
without granting rights in Epic or RAD code, tools, SDKs, documentation, marks,
source cinematics, audio, or encoded output.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository use and first-party Epic/RAD product
  documentation verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Input recognition and optional licensed output target.
- Subject class: Proprietary game-video codec, container, encoder, and playback
  technology.

## Covered Material

Bink 1 and Bink 2 identities recognized by SHAR, including `.bik` and `.bk2`
files, signatures, header evidence, video and audio tracks, optional language
tracks, frame and timing metadata, and the official encoder and Unreal playback
path.

## Repository Use And Scope

The RMV crate recognizes bounded Bink signatures and treats official Bink 2 as
an optional output target that requires official tooling. SHAR's reproducible
default remains HAP video plus WAVE audio when the licensed official encoder is
not available. The repository does not bundle Bink SDKs, private encoder
binaries, or encoded game cinematics.

The game manifest classifies `.bik` and `.bk2` as movie resources. Recognition
and classification do not imply complete decoding support, ownership, license,
or redistribution authority.

## Provenance And Version History

The first-party RAD/Epic product page distinguishes Bink 2 from Bink 1 and
presents Bink as a commercial game-video technology. Epic's Unreal Engine
documentation states that the Bink Media plugin is built into Unreal Engine and
that Unreal installations include a Bink 2 Encoder for Unreal under the engine's
third-party binaries.

The exact Bink release, SDK, encoder, plugin, platform entitlement, and license
must be established from the installed Unreal distribution and current Epic
terms. A product page is not a normative public byte-level specification.

## Authorship, Ownership, And Attribution

Epic Games, Epic Games Tools, former RAD Game Tools contributors, licensors, and
other rights holders retain applicable rights in Bink code, SDKs, encoders,
plugins, documentation, names, marks, and codec technology. Rights in source and
encoded audiovisual content remain separate.

SHAR contributors retain rights only in independently authored recognition,
validation, planning, and integration code.

## License Or Terms Basis

Bink is proprietary commercial technology. Availability through Unreal Engine or
an Epic installation does not establish unrestricted redistribution rights for
the encoder, SDK, plugin source, standalone tools, or encoded third-party
content. The applicable Epic license, product terms, platform restrictions, and
installed notices control.

## Distribution, Modification, And Compatibility

A valid Bink file may contain protected audiovisual works. Encoding, decoding,
transcoding, or playback changes representation or access, not ownership. Use of
an official encoder must remain within the applicable license and platform
entitlement.

SHAR must distinguish Bink 1 input recognition, Bink 2 optional output, Unreal
plugin playback, FFmpeg capabilities, and HAP fallback. Those are different
technical and legal surfaces.

## Compliance Posture

- Do not bundle Bink SDKs, private encoders, or proprietary plugin source.
- Record the exact installed Epic or Bink component, version, notices, and
  terms.
- Treat official Bink 2 generation as optional and license-dependent.
- Keep original and generated protected cinematics outside public publication
  and distribution surfaces.
- Validate signatures, lengths, dimensions, frame counts, and track metadata.
- Do not claim complete Bink conformance or public format specification access.
- Reverify Epic and Bink licensing before any distribution or commercial use.

## Technical Baseline And SHAR Profile

### Public baseline

Epic documents use of the built-in Bink Media plugin in Unreal Engine, creation
of `.bk2` files with the Bink 2 Encoder for Unreal, and placement of encoded
movies under a project's `Content/Movies` directory. The reviewed first-party
documentation supports tooling and Unreal integration claims, not a claim that a
complete public Bink wire-format specification is available.

### SHAR profile

SHAR separates recognition from decoding and encoding:

- the RMV domain recognizes selected Bink 1 signatures and selected Bink 2
  signatures;
- recognition validates the declared file size against the complete file,
  requires a complete header probe, and applies bounded frame-count and frame-
  dimension checks;
- the recognized limits currently include no more than 1,000,000 frames,
  7,680-pixel width, and 4,800-pixel height;
- recognition does not decode the Bink payload or establish file authenticity;
- Bink 2 output remains optional and requires official Epic/RAD tooling; and
- the portable default remains HAP Q video plus PCM WAVE audio.

No Bink encoder, proprietary SDK, or complete Bink specification is represented
as repository-owned or freely redistributable.

### Use-specific evidence limits

Before enabling a Bink encoding workflow, capture the exact encoder entitlement
and terms, installed tooling identity, supported input and output profiles,
Unreal plugin configuration, generated-file tests, and the boundary between
signature recognition, structural validation, decoding, and encoding.

### Verified sources

- Epic Games, *Bink Video for Unreal Engine*.
  <https://dev.epicgames.com/documentation/unreal-engine/bink-video-for-unreal-engine>
- SHAR repository evidence: `src/rmv/src/domain/format.rs`,
  `src/rmv/src/domain/target.rs`, and `src/rmv/src/application/package_plan.rs`.

## Source References

- Epic Games Tools (2026) *Bink Video: The Video Codec for Games*. Available at:
  <https://www.radgametools.com/bnkmain.htm> (Accessed: 12 July 2026).
- Epic Games (n.d.) *Bink Video for Unreal Engine*. Available at:
  <https://dev.epicgames.com/documentation/en-us/unreal-engine/bink-video-for-unreal-engine>
  (Accessed: 12 July 2026).
- SHAR repository (2026), RMV Bink recognition, cinematic target selection, and
  game-manifest classification tests.

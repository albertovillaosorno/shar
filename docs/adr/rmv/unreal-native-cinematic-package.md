# Native cinematic package strategy

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Cinematic migration and target-native media packaging

## Context

Cinematic migration must preserve track identity, timing, synchronization, and
provenance while keeping source media outside public history. Manual Sequencer
assembly would make those relationships editor-order dependent.

The runtime targets Windows, Linux, macOS, and Android. Unreal media players,
operating-system decoders, supported containers, codec profiles, audio routing,
resolution limits, and frame-rate limits differ by target. A normalized HAP
video
with separate WAV tracks is useful conversion and review evidence, but it cannot
be assumed to be the universal packaged playback format.

## Decision

Each cinematic has one platform-neutral canonical timeline with explicit
cinematic identity, rational timebase, duration, video presentation track,
numbered audio tracks, subtitle and localization identities, event markers, and
provenance.

Normalized HAP video and separate numbered WAV tracks remain deterministic
intermediate and review evidence. HAP is not a runtime package invariant, and no
package relies on embedded HAP audio or timecode.

Packaging generates a target media variant for each claimed native target. A
variant declares the exact target identifier, Unreal media player or plugin,
container, video codec and profile, audio strategy, dimensions, frame rate,
bitrate or quality parameters, lengths, and hashes. The selected combination
must
be supported by the packaged Unreal build and verified on representative native
hardware.

A target variant may change codec, container, bitrate, dimensions, and other
presentation costs to satisfy native decoder and Android Low constraints. It may
not change canonical duration, narrative content, dialogue selection, subtitle
timing, event timing, or progression behavior. Frame-rate conversion is allowed
only when presentation timestamps preserve the canonical timeline and playback
verification proves no material drift or missing content.

Audio tracks remain explicit synchronized assets rather than incidental embedded
streams. Platform-specific media audio routing cannot redefine track identity,
locale fallback, volume policy, pause, seek, or resume behavior.

Packaged cinematics are local product media. No target depends on an Internet
stream, external codec installation, runtime download, or network service for
required story playback.

If the selected target lacks a verified native playback route, packaging either
performs a deterministic target transcode from normalized evidence or rejects
the
target as unsupported. It does not silently ship an unverified file or fall back
to missing cinematic content.

## Consequences

- Cinematic identity, timeline, tracks, localization, and event markers are
  shared across Windows, Linux, macOS, Android, x64, and ARM64.
- HAP and WAV remain reproducible normalized evidence without constraining every
  product target to the same decoder.
- Target-specific media variants are derived artifacts with explicit provenance
  and deterministic packaging parameters.
- Android Low may use a lower-cost verified variant without changing story or
  timing contracts.
- Video and audio synchronization is verified from the canonical timeline,
  normalized evidence, packaged variant, and native playback read-back.
- Decoder, container, codec, or audio-routing failure blocks availability for
  the
  affected target instead of producing a partial cinematic state.
- Source media remains user evidence and is not published with repository
  documentation or source history.

## Rejected alternatives

- Treating HAP as the mandatory packaged codec on every platform.
- Selecting one container or media player without target-native evidence.
- Depending on user-installed external codec packs for required playback.
- Streaming required cinematics from a network service.
- Dropping, shortening, retiming, or replacing story content to meet a target
  performance budget.
- Relying on embedded media audio to infer numbered track identity or locale.
- Building production cinematics through manual Sequencer assembly.
- Inferring track identity or timing from editor ordering.
- Copying proprietary source media or editor projects into the repository.

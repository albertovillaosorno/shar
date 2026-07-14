# Platform cinematic media packaging

- Status: Active
- Last reviewed: 2026-07-13

## Governing decisions and evidence

- [Native cinematic package strategy](../../adr/rmv/unreal-native-cinematic-package.md)
- [Local cinematic overrides](../../adr/rmv/local-movie-overrides.md)
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
- [Unreal Engine](../../bibliography/engine-and-plugins/unreal-engine.md)

## Purpose

This specification defines the canonical cinematic timeline, normalized media
evidence, target-variant manifest, packaging, playback, synchronization,
lifecycle, fallback, and verification contracts for required local cinematics.

## Canonical cinematic timeline

Every cinematic has one platform-neutral definition containing at least:

| Field | Contract |
| :--- | :--- |
| `CinematicId` | Stable canonical identity. |
| `Timebase` | Exact rational timeline unit. |
| `Duration` | Canonical presentation duration in timeline units. |
| `VideoTrackId` | Canonical visual presentation identity. |
| `AudioTrackIds` | Ordered explicit audio identities by role and locale. |
| `SubtitleTrackIds` | Ordered subtitle and localization identities. |
| `EventMarkers` | Deterministic gameplay, fade, transition, and completion events. |
| `SourceRevision` | Normalized evidence revision and provenance. |
| `VariantRequirements` | Required target variants and their acceptance state. |

Timeline identity is independent of codec, container, file extension, media
player, storage path, target architecture, graphics preset, and physical media
member name.

## Normalized evidence

The pipeline may preserve a normalized HAP video plus separate numbered PCM WAV
tracks as deterministic conversion and review evidence. The normalized package
records exact timing, frame count, dimensions, frame rate, audio sample counts,
channel layout, lengths, hashes, and provenance.

HAP is not assumed to be supported by every product target. Embedded audio and
embedded timecode are not required or treated as authority. The canonical
timeline and explicit numbered audio tracks remain authoritative.

Normalized evidence is private generated input to native asset and package
creation. It is not public repository content and is not copied into source
history.

## Target media variant

Each target variant has a deterministic manifest containing:

| Field | Contract |
| :--- | :--- |
| `TargetId` | Exact canonical target identifier. |
| `VariantRevision` | Deterministic derived-media revision. |
| `MediaPlayer` | Exact enabled Unreal player or plugin selected for playback. |
| `Container` | Packaged local container identity. |
| `VideoCodec` | Exact codec, profile, level, and pixel-format requirements. |
| `AudioStrategy` | Explicit external Unreal audio tracks or verified packaged stream strategy. |
| `Dimensions` | Encoded width, height, and pixel-aspect contract. |
| `FrameRate` | Exact encoded rate and timestamp mapping. |
| `QualityParameters` | Deterministic bitrate, quality, keyframe, and conversion settings. |
| `Members` | Lengths, hashes, roles, and package identities for every file. |
| `VerificationRevision` | Native playback evidence accepted for this target. |

The packaging plan resolves the media player, codec, container, and decoder
availability from the selected Unreal build and native target. A player listed by
engine documentation is still unproved for SHAR until the packaged target passes
native playback verification.

Support for one target or operating-system decoder does not imply support on
another target. A target with no verified route remains blocked or receives a
deterministically transcoded variant; it never inherits another target's file by
assumption.

## Target conversion

Target conversion consumes only validated normalized evidence and a complete
variant policy. Equivalent evidence, policy, encoder build, and target produce
equivalent logical manifests and media hashes where the encoder contract is
deterministic.

Conversion may change:

- container and codec;
- codec profile or level;
- chroma and pixel format within the accepted visual contract;
- dimensions and bitrate for target performance and Android Low;
- keyframe interval and decoder-oriented packaging; and
- the player-specific local media-source representation.

Conversion may not change:

- canonical duration or event timing;
- narrative shots or their order;
- dialogue, locale, subtitle, or audio-track identity;
- progression, skip, pause, resume, or completion semantics;
- required fades and transitions; or
- source provenance and revision linkage.

Frame-rate conversion maps every output presentation timestamp to the canonical
timeline. It fails when required content is dropped, duplicated, reordered, or
materially drifted.

## Audio and localization

Audio track identity is explicit and independent of video packaging. Numbered
tracks preserve locale, role, channel, sample-rate, sample-count, and start-time
metadata. A target may use Unreal-native audio assets synchronized to video rather
than relying on media-player audio routing.

Locale resolution follows the accepted audio fallback decisions. A platform
cannot substitute a different language or collapse distinct tracks because its
media player exposes a different embedded-stream order.

Pause, resume, seek, skip, application backgrounding, and decoder recovery keep
video, audio, subtitles, and event markers synchronized to the canonical
timeline. Recovery never fires a completion or progression event twice.

## Android Low policy

Android packages use only the Low graphics policy, but cinematic presentation
remains complete. The Android variant may reduce dimensions, bitrate, decoder
cost, and storage footprint when native verification shows that the change
preserves readable imagery, dialogue, subtitles, timing, and narrative content.

Android foreground, background, suspension, audio-focus, and process-restart
events are explicit playback states. Backgrounding may pause playback. Resume
continues from a validated timeline position or restarts according to the
cinematic's declared policy; it does not guess from decoder-local state.

Required cinematics are packaged locally. Android playback does not require a
network stream, runtime download, external provider URI, or user-installed codec.

## Local overrides

A validated local cinematic override targets the canonical cinematic and track
identities. It passes the same timeline, integrity, target-compatibility, and
native-playback checks as a generated variant.

A desktop file import or Android managed document import copies accepted override
media into controlled local storage. The external path or provider URI never
becomes override identity. Invalid or unavailable overrides leave the accepted
canonical target variant active.

## Invariants

- Every target variant maps to one canonical cinematic timeline.
- Codec, container, player, path, and architecture never redefine cinematic or
  track identity.
- HAP remains normalized evidence rather than a universal product dependency.
- Required audio, subtitles, events, and narrative content are preserved across
  target variants.
- Playback completion and progression events fire exactly once.
- Required cinematics are local and do not require network or external codecs.
- Android Low may reduce media cost only within the accepted presentation and
  timing contract.
- A target is not available until every required cinematic passes native package
  verification.
- Invalid local overrides cannot replace a valid canonical variant.

## Failure behavior

Packaging or playback fails closed on:

- unknown cinematic, track, locale, target, or variant identity;
- missing or inconsistent normalized timing evidence;
- unsupported media player, container, codec, profile, level, dimensions, frame
  rate, audio route, or decoder capability;
- absent required engine plugin or target decoder;
- nondeterministic or incomplete target conversion;
- duration, timestamp, subtitle, event, or audio-sample drift beyond the accepted
  tolerance;
- missing, duplicate, reordered, or hash-mismatched members;
- reliance on an external codec installation, network source, or runtime download;
- Android lifecycle recovery without a validated timeline position;
- playback completion firing more than once; or
- an override that fails integrity, timeline, locale, target, or native-playback
  validation.

The affected target remains unsupported or the valid canonical variant remains
active. Missing cinematic content is never reported as successful playback.

## Verification

- Timeline tests prove rational time conversion, duration, event ordering, skip,
  pause, resume, seek, and exactly-once completion.
- Normalized-evidence tests verify HAP frame metadata and every numbered WAV
  track's samples, channels, start time, length, and hash.
- Target packaging tests record the selected player, plugin, codec, container,
  profile, dimensions, frame rate, quality parameters, members, and provenance.
- Native package tests play every required cinematic on each claimed target and
  verify first frame, final frame, duration, audio sync, subtitles, events, pause,
  resume, seek, skip, restart, and clean shutdown.
- Cross-target comparison proves equivalent canonical event and progression
  results even when media encoding differs.
- Android tests cover Low variants, audio focus, foreground and background,
  suspension, decoder recreation, process restart, storage pressure, and no-
  network operation.
- Resource tests measure decode CPU, GPU, memory, storage throughput, package
  size, startup latency, dropped frames, and audio drift on representative
  hardware.
- Override tests import equivalent media through desktop and Android adapters and
  prove stable identity, validation, fallback, and rollback.

## Known limits

No codec, container, player, maximum resolution, or maximum frame rate is claimed
portable merely because it appears in engine documentation. Exact target media
profiles remain evidence-driven and may change with the selected Unreal build,
platform SDK, operating-system decoder, or native hardware validation.

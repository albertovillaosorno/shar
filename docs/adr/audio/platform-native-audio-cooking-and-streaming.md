# Platform-native audio cooking and streaming

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Runtime audio identity, target cooking, streaming, and lifecycle

## Context

The pipeline preserves decoded audio as normalized PCM WAV evidence with
explicit
sample rate, bit depth, channels, sample count, timing, identity, and
provenance.
The runtime targets Windows, Linux, macOS, and Android across x64 and ARM64.
Those targets have different native audio devices, cooked representations,
decoder costs, memory limits, storage limits, lifecycle behavior, and output
capabilities.

Shipping every normalized WAV unchanged would confuse evidence with product
representation and could waste memory or storage. Selecting one compressed
format
for every platform would make an unverified portability promise. Performance
work
must not remove dialogue, music, effects, localization, or gameplay timing.

## Decision

Every audio asset has one platform-neutral canonical identity and normalized PCM
contract. The contract records role, locale, channels, sample rate, sample
count,
duration, loop points, synchronization markers, loudness and gain metadata when
available, dependencies, provenance, and revision.

Normalized PCM WAV remains deterministic conversion and review evidence. Product
packaging cooks a target-native Unreal audio representation according to an
explicit target policy. The policy selects Sound Wave loading behavior,
compression quality, streaming or residency, chunking, seek behavior,
concurrency, Sound Class loading, native quality, and platform audio routing
without changing canonical identity or gameplay meaning.

Unreal's Audio Mixer, Audio Components, native stream cache, Sound Classes,
Sound Mixes, Audio Modulation, submixes, effects, and platform device backend
are
the default runtime execution layer. The repository does not ship a translated
clip/stream player pool, custom decoder and memory-region manager,
script-created
resource graph, or platform-specific renderer fork.

Audio roles are explicit. At minimum, dialogue, music, cinematic audio, vehicle
loops, ambient loops, UI cues, and short gameplay effects may use different
loading and quality policies. A policy is selected from measured duration,
latency, memory, storage, repetition, seek, synchronization, and audibility
requirements rather than filename or directory.

Dialogue and cinematic synchronization preserve canonical sample timing and
locale identity. Music and required loops preserve declared loop boundaries.
Short latency-sensitive cues may be resident when measured budgets allow it;
longer assets may stream or use native cache facilities. No category may load
all
unrelated audio eagerly.

Android uses target-specific cooked audio and bounded memory, stream-cache,
voice,
and decoder policies. Low graphics does not imply missing or semantically
reduced
audio. Android lifecycle and audio-focus events may pause, duck, stop, or resume
according to explicit category policy, but cannot advance gameplay, fire an
audio-
driven event twice, substitute a different locale, or corrupt synchronization.

Required audio is packaged locally. Gameplay, dialogue, music, and cinematics do
not depend on a network stream, runtime download, user-installed codec, or
external audio service.

If a target lacks a verified cook or playback route for a required contract, the
target remains unsupported or receives a deterministic target representation. It
does not silently drop the asset, alter the locale fallback chain, or accept
unverified playback.

## Consequences

- Windows, Linux, macOS, Android, x64, and ARM64 share canonical audio and event
  identities while using verified target-native cooked representations.
- PCM WAV remains normalized evidence rather than a universal shipped format.
- Dialogue, music, loops, cinematics, effects, and UI cues use explicit policies
  based on their runtime requirements.
- Locale fallback and user-provided overrides resolve before target cooking and
  do not depend on physical filenames or platform paths.
- Memory, storage, stream-cache, decoder, and concurrency budgets are measured
  per
  target and representative scene.
- Optimization may reduce encoded cost within an accepted target policy but
  cannot remove required content, break loops, change event timing, obscure
  dialogue, or substitute another locale.
- Audio device or lifecycle failure produces a typed platform result instead of
  changing simulation or progression.

## Rejected alternatives

- Shipping every normalized WAV unchanged on every target.
- Mandating one compressed codec or container without native target evidence.
- Loading all audio eagerly or inferring load policy from folders and filenames.
- Treating Android Low as permission to omit dialogue, music, effects, or
  localization.
- Changing sample timing, loop boundaries, event order, or locale to meet a
  performance target.
- Requiring an Internet stream, external codec pack, or third-party runtime
  audio
  service for required playback.
- Allowing audio focus, suspension, or device loss to fire gameplay events twice
  or silently select a different track.

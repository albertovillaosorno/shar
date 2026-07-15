# Platform audio cooking and streaming

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Platform-native audio cooking and streaming](../../adr/audio/platform-native-audio-cooking-and-streaming.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Event-driven music and ambience](../../adr/unreal/runtime/event-driven-music-and-ambience.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Latin American Spanish audio fallback](../../adr/audio/lmlm-spanish-latam-audio-fallback.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native cinematic package strategy](../../adr/rmv/unreal-native-cinematic-package.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)

## Purpose

This specification defines normalized audio evidence, canonical audio identity,
role policies, target cooking, streaming and residency, lifecycle behavior,
budgeting, failure, and verification for native Unreal packages.

## Normalized audio evidence

Each decoded source produces one normalized PCM record containing at least:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `AudioId` | Stable canonical identity. |
| `Role` | Dialogue, music, cinematic, vehicle, ambient, UI, or gameplay effect. |
| `Locale` | Canonical locale or locale-neutral identity. |
| `Channels` | Exact channel count and semantic layout when known. |
| `SampleRate` | Exact samples per second. |
| `BitDepth` | Exact normalized PCM depth. |
| `SampleCount` | Exact sample count per channel. |
| `Duration` | Derived rational duration. |
| `LoopPoints` | Optional exact start and end sample positions. |
| `SyncMarkers` | Optional canonical sample-aligned events. |
| `GainMetadata` | Source gain and measured loudness evidence when available. |
| `SourceRevision` | Provenance and deterministic source revision. |
| `IntegrityRecord` | Length and hash evidence for metadata and PCM payload. |

<!-- markdownlint-enable MD013 -->

PCM WAV is a review and conversion representation. Runtime identity is the
canonical audio record, not the WAV filename, storage route, Unreal object path,
or cooked extension.

## Role policies

Music composition, semantic state, event binding, Quartz timing, graph
parameters, and transition behavior follow
[Music state and transition runtime](music-state-and-transition-runtime.md).
This specification remains authority over the normalized audio evidence and
platform-native cook used by those states.

Every audio role resolves to an explicit target policy. A policy declares:

- target identifier and policy revision;
- loading behavior: resident, streaming, or target-native cached;
- initial and subsequent chunk requirements when applicable;
- seek, loop, and restart behavior;
- compression quality or equivalent target setting;
- channel and sample-rate transformation rules;
- concurrency and voice limits;
- priority and eviction behavior;
- spatialization and attenuation contract where applicable;
- audio-focus and lifecycle behavior;
- required latency and synchronization bounds; and
- measured memory, storage, decode, and bandwidth budgets.

Role policy is not inferred from a folder, filename, duration alone, or graphics
preset. Asset-specific overrides are explicit, bounded, and validated.

Dialogue prioritizes intelligibility, locale correctness, bounded start latency,
and event timing. Music and ambient loops preserve declared loop sample
boundaries. Cinematic audio follows the canonical cinematic timeline. Vehicle
loops preserve pitch and transition behavior. UI and short gameplay cues
preserve
latency and priority. No policy may remove a required category.

## Target cooking

Target cooking consumes normalized PCM evidence, canonical metadata, a complete
role policy, and the selected Unreal target toolchain. It produces a
target-native
cooked representation and manifest with:

- target identifier;
- canonical audio identity and source revision;
- resolved policy revision;
- cooked representation and Unreal asset identity;
- transformed channels, sample rate, duration, and loop points;
- loading, chunking, cache, and concurrency settings;
- encoded or cooked member lengths and hashes;
- required plugins or native capabilities; and
- native verification revision.

The exact cooked codec or storage representation is evidence-driven and may vary
by target and selected Unreal build. A successful cook on one target does not
prove another. Target transformation must preserve accepted duration, loop,
locale, event, and audibility contracts.

A target with no verified representation remains unsupported or receives a
deterministically generated target variant. It never silently ships raw
evidence,
drops the audio, or substitutes an unrelated format.

## Streaming, residency, and cache

Streaming and residency are selected from measured runtime behavior. The system
tracks at least:

- resident decoded memory;
- compressed or cooked storage;
- active decoder and voice count;
- stream-cache occupancy and eviction;
- read throughput and latency;
- start latency, underruns, and dropped or starved voices;
- seek and loop accuracy; and
- concurrent level, mission, cinematic, and UI demand.

Preloading is bounded to the active world, mission, cinematic, UI, and immediate
transition scopes. Unrelated locales, characters, levels, and cinematics are not
loaded eagerly. Locale changes invalidate only the affected presentation scopes
and do not change canonical gameplay identity.

Eviction cannot remove an asset required for an active dialogue line, cinematic,
mission event, vehicle loop, or uninterruptible UI contract. A policy either
pins
that scope or fails activation before playback begins.

## Android lifecycle and audio focus

Android foreground, background, suspension, low-memory, output-device, and
audio-
focus events map to typed platform audio events.

Each role declares whether focus loss pauses, ducks, stops, or continues. The
adapter preserves canonical playback and event position when resumption is
supported. It never infers progression from decoder completion alone and never
fires an audio-driven gameplay or cinematic event twice.

Low-memory handling may evict inactive cached audio according to explicit
priority. It cannot discard active required dialogue, corrupt loop state, change
locale, or report a successful playback that did not occur.

Android Low uses target-specific audio budgets independent of the graphics
preset
name. The visual Low requirement is not authority to lower audio semantics.
Required audio remains local and playable without network access.

## Locale and overrides

Locale resolution occurs before target cooking or runtime selection. The Latin
American Spanish override, base Spanish, and global fallback chain resolves by
canonical identity. Physical filename, import path, and platform language labels
are not authority.

A local audio override passes identity, PCM metadata, duration, loop, locale,
target-cook, integrity, and native-playback validation. Desktop file import and
Android managed import produce the same logical override. Invalid overrides
leave
the valid canonical target representation active.

## Invariants

- Canonical audio, locale, loop, and event identity are shared across targets.
- PCM WAV remains normalized evidence rather than a universal shipped format.
- Target cooking does not change gameplay meaning, required content, or locale
  fallback.
- Active required audio cannot be evicted or silently dropped.
- Loop and synchronization positions remain sample-defined through cooking.
- No role policy is inferred from incidental filenames or directories.
- Android lifecycle and audio focus cannot duplicate events or progression.
- Required audio needs no network stream, runtime download, or external codec.
- A target is not supported until required audio passes native package playback.
- Optimization remains within the selected role policy and cannot obscure
  dialogue or remove music, effects, or localization.

## Failure behavior

Cooking, loading, or playback fails closed on:

- unknown audio, role, locale, target, or policy identity;
- invalid PCM metadata, sample count, channel layout, loop points, or hashes;
- unsupported target cook, plugin, native audio capability, or output route;
- nondeterministic target representation;
- duration, loop, synchronization, or locale drift;
- missing required chunks or cache capacity;
- stream underrun beyond the accepted contract;
- concurrency or eviction that removes required active audio;
- Android focus or lifecycle recovery without a valid canonical position;
- duplicate audio-driven event delivery;
- reliance on a network service, runtime download, or external codec; or
- an override that fails identity, integrity, target, or playback validation.

Failure returns a typed target, asset, policy, and invariant diagnostic. The
runtime preserves the last valid active representation or rejects the affected
scope before it begins.

## Verification

- PCM fixtures verify channels, sample rate, bit depth, sample count, duration,
  loop points, synchronization markers, lengths, and hashes.
- Target-cook tests record policy, cooked members, transformations,
  dependencies,
  and reproducible provenance for every claimed target.
- Native package tests verify dialogue, music, ambient, vehicle, UI, gameplay,
  and cinematic roles on Windows, Linux, macOS, and Android packages.
- Cross-target tests compare canonical duration, locale, event order, loop
  boundaries, and progression results.
- Streaming tests measure latency, throughput, cache occupancy, eviction,
  underruns, voice count, decoder cost, and memory under representative scenes.
- Locale tests cover Latin American Spanish overrides, base Spanish, global
  fallback, runtime locale changes, and missing overrides.
- Android tests cover focus gain and loss, output-device changes, foreground,
  background, suspension, low memory, process restart, and no-network playback.
- Fault injection covers missing chunks, storage denial, corrupt data, decoder
  failure, cache exhaustion, and lifecycle interruption.
- Override tests prove equivalent desktop and Android import, stable identity,
  target cooking, fallback, and rollback.

## Known limits

This specification does not promise one codec, compression setting, sample rate,
channel layout, cache size, or concurrency limit across every platform. Exact
values remain target-policy evidence derived from the selected Unreal build and
representative native hardware.

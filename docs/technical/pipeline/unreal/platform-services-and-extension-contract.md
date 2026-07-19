# Audio, UI, mods, networking, rendering, and platforms

- Status: Active
- Last reviewed: 2026-07-18

## Audio normalized inputs

Normalized production audio uses uncompressed PCM WAV:

<!-- markdownlint-disable MD013 -->
| Family | Channels | Sample rate | Source depth | Loop metadata |
| :--- | :--- | :--- | :--- | :--- |
| Dialogue | Mono unless stereo is semantically required | 48 kHz | 24-bit PCM | No |
| Ordinary SFX | Mono for positional, stereo for non-positional | 48 kHz | 24-bit PCM | Declared |
| Vehicle loops | Mono layers or declared stereo | 48 kHz | 24-bit PCM | Exact loop points |
| Music | Stereo | 48 kHz | 24-bit PCM | Segment and transition metadata |
| UI | Mono or stereo | 48 kHz | 24-bit PCM | No unless declared |
<!-- markdownlint-enable MD013 -->

Final platform compression, streaming, seek tables, chunking, and cache policy
are native cook outputs. Runtime never decodes proprietary source audio formats.
Dialogue lines, Sound Waves, MetaSound Sources, attenuation, concurrency, Sound
Classes, Sound Mixes, submixes, modulation, buses, vehicle profiles, ambience,
and music transitions have stable typed identities.

## Cinematic and media inputs

The normalized media master is a declared MOV container plus synchronized PCM
WAV tracks, subtitles, timing markers, and digest evidence. The pipeline
generates a verified target variant per platform. A target is accepted only when
container, codec, resolution, frame rate, color, audio synchronization, seek
behavior, subtitles, and packaged playback work without network or an undeclared
external codec.

HAP may remain normalized evidence where already available, but it is not
assumed to be the shipping format on every platform.

## UI architecture

Menus and HUD use Common UI and view models. Screens consume catalog and runtime
state through typed projections. They do not hardcode character, vehicle,
mission, or mod lists.

The game includes professional catalog flows for:

- character selection and unlock state;
- vehicle browsing, ownership, purchase, and phone-booth retrieval;
- missions and replay;
- collectibles, costumes, achievements, abilities, and world completion;
- self-hosted community-session configuration where enabled;
- mods, package compatibility, and trust status.

UI textures use PNG normalized inputs, sRGB color, explicit alpha, and
dimensions multiples of four. Nine-slice margins, DPI rules, target aspect
behavior, safe-zone behavior, and minimum readable size are manifest fields.
Player-facing text uses localization keys and String Tables; canonical identity
is never localized.

## Accessibility

Definitions and UI flows declare support for subtitles, speaker labels, scalable
text, contrast, color-independent cues, remapping, hold or toggle alternatives,
motion reduction, camera shake reduction, field-of-view range, audio
dynamic-range modes, and assist policy. Essential information has at least one
non-color-only and one non-audio-only representation.

## Mod package model

Cooked content mods use validated Game Feature packages mounted below:

```text
/Game/Mods/<namespace>
```

Each mod has one `SharModDescriptor` containing namespace, version, package-set
digest, required game compatibility, dependencies, conflicts, trust tier,
activation actions, catalog additions, replacements, save policy, network
policy, and teardown evidence.

Mods can add or replace definitions, presentations, missions, vehicles,
characters, world layers, UI, audio, abilities, camera profiles, and self-hosted
modes through registered extension points. They cannot rely on arbitrary load
order, editor scans, or mutable base assets.

## Trust tiers

- data-only: definitions, tables, assets, StateTree parameters, and registered
  Game Feature actions;
- Blueprint: reviewed Blueprint classes within declared APIs and budgets;
- native: signed or explicitly user-approved code with full process trust
  warning;
- server-required: package set must match the authoritative community server.

The base product does not pretend native code can be sandboxed safely.

## Replacement rules

A replacement declares target Primary Asset identity, replacement scope,
priority, compatibility range, required dependencies, and rollback. Visual
replacement may change presentation while preserving gameplay and save identity.
Gameplay replacement declares its authority and achievement effects.

Two active mods cannot replace the same exclusive slot unless a registered merge
policy resolves them deterministically.

## Self-hosted multiplayer foundation

The runtime is designed for server authority even though the base campaign
remains single-player. Replicated state uses stable domain identities and
explicit network schemas. Actor package paths, editor names, and source
filenames never cross the wire as authority.

The project may provide:

- dedicated-server and listen-server build targets;
- direct IP or hostname connection;
- LAN discovery;
- server configuration files;
- package-set digest negotiation;
- protocol and schema compatibility checks;
- server-authoritative spawning, progression policy, inventory, damage,
  vehicles, abilities, world state, and mod-defined rules;
- client prediction and reconciliation only for approved systems.

The project does not provide official hosted servers, cloud accounts, global
matchmaking, a centralized server list, monetized hosting, moderation service,
anti-cheat service, backups, or uptime guarantees. Community operators provide
and administer their own infrastructure in the practical style of self-hosted
sandbox servers.

Before joining, client and server exchange protocol version, game definition
revision, required mod descriptors, package digests, target platform
capabilities, and optional cosmetic allowances. Required mismatch rejects before
world travel. Cosmetic differences are allowed only when server policy declares
the slots non- authoritative.

## Rendering baseline

Rendering uses native Unreal 5.8 systems and a capability-driven policy.
Gameplay, asset identity, materials, camera, UI, and saves never depend on one
GPU vendor or upscaler.

TSR is the guaranteed temporal upscaler and anti-aliasing baseline on supported
platforms. Vendor technologies are optional adapters selected only after runtime
capability detection and project validation.

The settings UI exposes semantic modes such as Native AA, Quality, Balanced,
Performance, and Dynamic. A provider adapter maps those modes to supported
native or vendor settings. Unsupported providers or modes fall back
deterministically.

Current integration targets are:

- Unreal TSR as the required baseline;
- NVIDIA DLSS 4.5 where the official plugin supports the engine, target,
  hardware, driver, and project configuration;
- AMD FSR Upscaling and Frame Generation where the official plugin supports the
  engine and target; UE 5.8 integration remains disabled until an official or
  separately reviewed compatible package exists;
- Intel XeSS as an optional future adapter after reviewed engine compatibility;
- native spatial fallback only for unsupported low-end targets.

There is no `DLSS 5` contract because that is not an official supported product
identity at this review date. Version identity is never guessed from marketing
shorthand.

## Frame generation

Frame generation is optional, user-controlled, and never counted as simulation
rate. Physics, input, networking, animation state, mission timing, UI state, and
server authority use real rendered or simulation frames and real time.

Frame generation requires a minimum real frame-rate threshold, latency support,
valid motion vectors, stable UI composition, and a compatible presentation mode.
It disables itself when those invariants fail.

## Graphics presets

Low, Medium, High, Epic, and Ultra use one gameplay contract. Presets control
only rendering and presentation budgets, including:

- internal resolution and temporal upscaler mode;
- Lumen or fallback lighting policy;
- hardware ray tracing and ray reconstruction when supported;
- Nanite fallback policy;
- Virtual Shadow Map quality;
- reflection, translucency, volumetric, fog, cloud, and post-process quality;
- foliage, crowd, traffic representation, VFX, and animation update budgets;
- texture streaming pool, anisotropy, LOD, HLOD, and view distance;
- motion blur, depth of field, film grain, chromatic aberration, and camera
  shake according to accessibility settings.

Ultra may enable expensive ray-traced effects but cannot alter collision,
visibility authority, mission outcomes, AI perception, or multiplayer state.

## Platform profiles

Each target profile declares renderer, RHI, shader model, upscalers, frame-
generation support, ray tracing, Nanite, Lumen, Virtual Shadow Maps, texture
formats, memory budgets, target frame rate, and fallback paths. The pipeline
produces platform-independent definitions plus target-cooked variants; it does
not bake one PC GPU assumption into source assets.

## Validation

Platform publication proves native startup, asset cooking, streaming, UI safe
zones, input, audio, media, save storage, network-offline behavior, and every
claimed rendering capability. Optional provider failure must return to a
supported baseline without changing gameplay or corrupting settings.

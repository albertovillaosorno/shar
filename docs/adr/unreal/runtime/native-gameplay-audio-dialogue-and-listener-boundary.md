# Native gameplay audio, dialogue, and listener boundary

- Status: Accepted
- Decision date: 2026-07-16
- Scope: Vehicle audio, gameplay dialogue, spatial listeners, and positional
  sources

## Context

Gameplay audio crosses vehicle simulation, character identity, mission events,
local players, cameras, world streaming, localization, subtitles, animation,
platform focus, and presentation budgets. The source implementation combines
mutable singleton players, filename-derived dialogue metadata, process-global
queues, raw callback ownership, one hard-coded listener, and audio completion
callbacks that can be mistaken for gameplay results.

A faithful Unreal implementation must preserve audible behavior without carrying
forward those ownership and identity defects. Native engine audio facilities are
required for playback, spatialization, attenuation, concurrency, routing, and
mixing, while project services remain authority over semantic event selection,
vehicle observations, dialogue arbitration, local-player policy, and durable
results.

## Decision

The runtime uses Unreal's native audio engine for `USoundWave`, Sound Cue,
MetaSound, `UAudioComponent`, Sound Attenuation, Sound Concurrency, Sound Class,
Sound Mix, submix, modulation, spatialization, occlusion, reverb send, and
platform output behavior.

Repository-owned definitions and services provide stable audio identities,
revisioned requests, deterministic selection, typed parameters, bounded leases,
world and feature ownership, immutable observations, completion correlation,
and diagnostics. Raw filenames, source path fragments, array positions, object
addresses, callback pointers, and legacy enum ordinals are not runtime identity.

Vehicle audio is a presentation state machine driven by immutable vehicle,
movement, surface, damage, door, horn, local-player, and world observations. It
may project engine, idle, shift, reverse, in-air, skid, horn, damage, overlay,
backup, and door sounds. It cannot change vehicle gear, speed, damage, contact,
mission, or persistence state.

Dialogue uses imported typed line, conversation, selection-group, event-binding,
priority, probability, interruption, subtitle, positional, and mouth-animation
definitions. Runtime matching never reparses a cooked asset filename.
Eligibility
and ordering are deterministic from canonical event, participant, role, world,
level, mission, locale, conversation, usage, and request revisions. Optional
variation uses a stable declared seed rather than process-global random state.

Dialogue queue admission, interruption, expiry, pause, resume, cancellation,
preemption, and completion are typed transactions. Playback completion may
advance the accepted dialogue presentation sequence, release ducking, or publish
an observation. It cannot complete a mission, interaction, conversation domain
transaction, reward, save operation, or progression result.

Each local player owns a revisioned audio-listener candidate derived from the
accepted camera, controlled participant, world, view, and application mode.
The platform audio policy explicitly chooses independent listeners, a shared
listener, a primary listener, or another validated split-screen mix strategy.
No implementation silently assumes player zero or treats one camera pointer as
process-wide listener authority.

Positional sources use native attached or world-space audio components with
validated attenuation and concurrency assets. Source transforms, velocities,
attachments, world ownership, listener policy, virtualization, and teardown are
revisioned. A late source or listener callback cannot mutate a released world,
replacement Actor, different local player, or superseding playback request.

Audio quality settings may change codec, sample rate, voice count, streaming,
virtualization, attenuation implementation, effect cost, and optional cosmetic
layers within accepted policy. They cannot remove required dialogue, alter line
selection, change subtitles, modify vehicle state interpretation, or change
canonical event results.

## Consequences

- Vehicle audio consumes vehicle observations through typed presentation ports.
- Engine pitch, shift, skid, reverse, in-air, damage, horn, overlay, backup, and
  door behavior are data-driven and independently testable.
- Dialogue metadata is cooked into typed assets and never inferred from runtime
  filenames.
- Dialogue matching, variation, queue ordering, interruption, and expiry are
  deterministic and revisioned.
- Sound Concurrency and project queue policy cooperate without becoming mission
  or dialogue-domain authority.
- Subtitles, localized audio, speaker identity, and mouth-animation observations
  stay correlated to one accepted line revision.
- Local players and frontend presentation use explicit listener policy.
- Positional audio follows native attenuation, spatialization, occlusion, and
  audio-component lifetime.
- World unload, feature removal, vehicle replacement, player removal, and locale
  change cancel or migrate only explicitly owned playback state.
- Headless servers execute semantic gameplay without requiring an audio device.
- Development diagnostics are read-only and cannot force playback or alter
  selection history.

## Rejected alternatives

- A process-wide sound singleton that manually updates every source each frame.
- Runtime dialogue identity or event parsing from filenames and directory text.
- Global `rand()` calls for dialogue selection or occasional-line admission.
- One raw linked-list queue whose callback order selects audible results.
- Audio completion, mouth animation, or subtitle completion as gameplay success.
- Hard-coding player zero as the only listener for every mode and local player.
- Clamping one global listener to an arbitrary participant without declared
  split-screen policy.
- Fixed arrays of engine, skid, horn, damage, overlay, backup, or door players
  as
  durable channel identity.
- Vehicle-audio code mutating gear, speed, damage, terrain, or mission state.
- Positional sources retaining raw Actor or camera pointers across world unload.
- Graphics or mobile presets dropping required dialogue or changing line choice.
- Shipping source decoders, dialogue parsers, or legacy audio object factories.

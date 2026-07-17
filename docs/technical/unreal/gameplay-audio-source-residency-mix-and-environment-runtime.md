# Gameplay audio source, residency, mix, and environment runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Native gameplay audio, dialogue, and listener boundary](../../adr/unreal/runtime/native-gameplay-audio-dialogue-and-listener-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform audio cooking and streaming](platform-audio-cooking-and-streaming.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Presentation playback runtime](presentation-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Physical material and impact-response runtime](physical-material-and-impact-response-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Developer command and diagnostic runtime](developer-command-and-diagnostic-runtime.md)

## Purpose

This specification defines native Unreal gameplay sound-source requests, generic
one-shot and continuous playback, asynchronous readiness, source parameters,
audio-content residency, scope-owned asset bundles, Sound Class and mix policy,
submix routing, environmental reverb, Audio Volume integration, collision-audio
selection, concurrency, voice pressure, diagnostics, platform behavior, feature
overlays, failure, and teardown.

It replaces process-global generic sound players, fixed source slots, raw
resource
names, callback pointers, fixed-size sound clusters, enum-indexed residency,
manual player dumping, one-frame service loops, platform-specific reverb
controllers, mutable debug pages, and audio behavior that depends on array or
callback order.

The runtime presents accepted gameplay and application observations. It never
becomes mission, interaction, damage, vehicle, physics, progression, save,
world,
or renderer authority.

## Native Unreal foundation

The boundary uses native Unreal facilities:

- `USoundWave`, Sound Cue, MetaSound Source, Dialogue Wave, or another validated
  native sound source;
- `UAudioComponent` for controlled source lifetime and parameter updates;
- Sound Attenuation and Sound Concurrency assets;
- Sound Classes and Sound Mixes for role grouping and scoped mix changes;
- Sound Submixes, submix sends, effect presets, Audio Buses, and modulation
  where
  the selected build and plugins support them;
- Audio Volumes or validated room and portal adapters for environmental effects;
- Asset Manager primary assets, bundles, soft references, and retained
  streamable
  handles;
- native audio virtualization, voice management, and output-device lifecycle;
- game-instance, world, local-player, application-mode, and Game Feature
  subsystem
  lifetimes; and
- development-only audio statistics, logs, profiling, and overlays.

Repository code supplies stable identities, definitions, deterministic
selection,
load and playback transactions, ownership, correlation, bounded policy, and
validation. It does not implement a second audio renderer, mixer, stream cache,
spatializer, or reverberation engine.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Gameplay and application services | Own accepted semantic events, mode state, world state, collisions, interactions, and durable results. |
| Audio-source catalog | Owns stable source definitions, parameters, role, attenuation, concurrency, routing, residency, and fallback policy. |
| Audio-residency service | Owns scope requests, primary-asset bundles, retained handles, readiness, reference counts, eviction, and teardown. |
| Gameplay-audio source service | Validates playback requests, allocates source leases, applies parameters, correlates callbacks, and returns typed terminal results. |
| Mix and environment service | Resolves Sound Class, Sound Mix, submix, bus, Audio Volume, room, reverb, ducking, and transition policy. |
| Spatial-audio service | Owns listeners, positional-source projection, attenuation, occlusion, and attachment policy. |
| Unreal Audio Engine | Owns decoding, native source playback, mixing, voice management, spatialization, effects, virtualization, and hardware output. |
| Platform audio policy | Owns target cooking, quality, output-device, focus, and suspension behavior. |
| Developer diagnostics | Observe accepted state without mutating playback, residency, mix, or gameplay. |

<!-- markdownlint-enable MD013 -->

A loaded asset, active voice, audible source, effect tail, or completion
callback
is presentation evidence only.

## Runtime identities

The boundary uses stable identities for:

- `FSharAudioSourceDefinitionId`;
- `FSharAudioSourceDefinitionRevision`;
- `FSharAudioRequestId`;
- `FSharAudioPlaybackId`;
- `FSharAudioPlaybackRevision`;
- `FSharAudioLeaseId`;
- `FSharAudioRoleId`;
- `FSharAudioParameterSchemaId`;
- `FSharAudioResidencyDefinitionId`;
- `FSharAudioResidencyScopeId`;
- `FSharAudioResidencyRevision`;
- `FSharAudioBundleId`;
- `FSharAudioMixPolicyId`;
- `FSharAudioMixRevision`;
- `FSharAudioEnvironmentId`;
- `FSharAudioEnvironmentRevision`;
- `FSharCollisionAudioProfileId`;
- `FSharWorldCompositionRevision`;
- `FSharApplicationModeRevision`;
- `FSharLocalPlayerId`;
- `FSharFeatureRevision`; and
- `FSharAudioResultId`.

Raw strings, hashed names, fixed arrays, cluster ordinals, namespace pointers,
player slots, callback user data, platform enum values, and debug-page positions
are not durable identity.

## Audio-source definition

`USharGameplayAudioSourceDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `DefinitionId` | Canonical semantic source identity. |
| `SourceAsset` | Validated Sound Wave, Sound Cue, MetaSound, Dialogue Wave, or registered source adapter. |
| `RoleId` | Gameplay effect, UI, ambience, vehicle, dialogue, cinematic, music, or another closed role. |
| `PlaybackPolicy` | One-shot, finite loop, leased continuous, queued, attached, or owner-scoped persistent behavior. |
| `ParameterSchema` | Allowed pitch, gain, filters, switches, triggers, and typed graph parameters. |
| `AttenuationPolicy` | Positional requirements and approved Sound Attenuation asset. |
| `ConcurrencyPolicy` | Sound Concurrency asset and project significance policy. |
| `RoutingPolicy` | Sound Class, submix, bus, reverb-send, and modulation identities. |
| `ResidencyPolicy` | Required audio bundle, preload, retention, and eviction behavior. |
| `PausePolicy` | Pause, continue, duck, virtualize, stop, or restart behavior. |
| `CompletionPolicy` | Observable completion, ignored completion, chained presentation, or barrier behavior. |
| `NetworkPolicy` | Authority, replication, prediction, local-only, or owner-only presentation. |
| `QualityPolicy` | Required and optional target variants. |
| `FallbackPolicy` | Missing optional layer, alternate source, silent typed result, or activation failure. |
| `DefinitionRevision` | Immutable revision for stale-result rejection. |

<!-- markdownlint-enable MD013 -->

Every definition has bounded lifetime, routing, concurrency, and residency. A
raw
clip name cannot bypass the catalog.

## Parameter schema

A source parameter schema declares:

- canonical parameter identity;
- scalar, integer, Boolean, enum, trigger, object, or curve type;
- finite bounds and units;
- default value;
- update cadence and smoothing policy;
- whether updates are accepted before or after playback starts;
- source roles and graph kinds that support the parameter;
- replication or local-only behavior; and
- diagnostic redaction.

Pitch and gain use bounded linear or logarithmic policy as declared. Invalid,
non-finite, stale, or unsupported parameters fail the update without corrupting
the active source.

## Playback request

`FSharGameplayAudioRequest` contains:

- request, source-definition, owner, and causation identities;
- expected definition, owner, world, mode, local-player, and feature revisions;
- semantic event and role;
- optional positional-source or attachment request;
- requested start, seek, loop, and duration policy;
- typed parameter values;
- mix, routing, and residency expectations;
- priority, deadline, cancellation token, and diagnostics correlation; and
- completion-observation policy.

Requests are immutable. Changes create revisioned parameter, routing, pause,
position, or stop transactions.

## Playback lifecycle

The closed lifecycle is:

1. `requested`;
1. `validating`;
1. `waiting_for_residency`;
1. `prepared`;
1. `starting`;
1. `playing`;
1. `virtualized`;
1. `paused`;
1. `stopping`;
1. `completed`;
1. `cancelled`;
1. `released`; or
1. `failed`.

Every accepted request reaches one terminal result exactly once. Native delegate
order cannot create duplicate completion, start-after-cancel, or release-after-
reuse behavior.

## Generic one-shot playback

One-shot playback is used for bounded effects such as impacts, doors, switches,
rewards, pickups, menu cues, explosions, and authored world events.

The request declares:

- source definition and semantic reason;
- owner and world revision;
- positional or non-positional policy;
- required residency and start deadline;
- concurrency and significance;
- parameter snapshot; and
- whether completion is observable.

A one-shot may outlive the originating event only for its declared finite tail.
It cannot retain a world, Actor, feature, or local player after teardown.

## Controlled and continuous playback

Continuous or looping playback requires an explicit `FSharAudioLeaseId`. The
lease declares owner, source, world, mode, feature, start revision, heartbeat or
replacement policy when needed, maximum unconfirmed lifetime, and release
reason.

Per-frame calls are not lifetime authority. An owner renews or replaces a lease
through typed state changes. Missing updates use the declared hold, interpolate,
virtualize, fade, or stop behavior.

## Queued playback

A queued request reserves definition, owner, residency, and parameter state but
does not claim an audible native voice until admitted.

Queue admission validates:

- the request is still current;
- required assets remain resident or loadable;
- concurrency and priority policy still permit playback;
- owner, world, mode, and feature revisions still match;
- deadline and lifetime have not expired; and
- no superseding request has replaced it.

Queue position is not identity. Callback order or linked-list insertion order
cannot decide between equal-priority requests; the policy uses canonical
priority, semantic age, deterministic tie-breaks, and explicit fairness.

## Asynchronous readiness

Asset load and source preparation callbacks carry request, bundle, owner, world,
mode, and feature revisions. A callback is accepted only when every expected
revision still matches.

Readiness means the required native assets and policy are available. It does not
mean the source became audible, completed, or satisfied gameplay.

Cancellation before readiness releases retained handles and returns one terminal
result. A late callback cannot start playback in a replacement world or source
lease.

## Positional settings

Legacy minimum distance, maximum distance, playback probability, and resource
name are converted into validated source definitions, Sound Attenuation assets,
concurrency policy, and deterministic optional-selection policy.

Playback probability is never evaluated with process-global random state. An
optional source derives its decision from declared request, event, owner,
location, usage, and policy revisions plus a stable seed.

Distance controls audibility through native attenuation and significance. An ad
hoc distance check cannot pause or resume a source outside the accepted
virtualization policy.

## Collision audio

Collision and impact audio consume accepted evidence from
<!-- markdownlint-disable-next-line MD013 -->
[Physical material and impact-response runtime](physical-material-and-impact-response-runtime.md).

`FSharCollisionAudioObservation` includes:

- impact, body, participant, and physical-material identities;
- contact position, normal, relative velocity, impulse, and accepted magnitude;
- surface pair and collision-audio profile;
- world and physics revisions;
- cooldown and duplicate-suppression evidence; and
- presentation priority.

The audio adapter resolves one source definition and typed parameters. It cannot
apply damage, impulse, breakage, notoriety, mission progress, or persistence.

## Audio-residency definition

`USharAudioResidencyDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ResidencyId` | Canonical residency-policy identity. |
| `Bundles` | Primary-asset bundle identities and required source definitions. |
| `EligibleScopes` | Process, frontend, gameplay, world region, mission, character, vehicle, cinematic, local player, or feature. |
| `LoadPolicy` | Resident, prime, stream, on-demand, or target-native cache behavior. |
| `RetentionPolicy` | Scope reference, pin, grace interval, stream cache, or immediate release. |
| `Priority` | Load and eviction priority. |
| `BudgetClass` | Memory, decoder, stream, and voice budget classification. |
| `FailurePolicy` | Required activation failure, partial optional readiness, or typed degradation. |
| `DefinitionRevision` | Immutable revision and content digest. |

<!-- markdownlint-enable MD013 -->

A residency definition has no fixed resource limit and no array slot identity.
Capacity is measured and bounded by platform policy.

## Residency scopes

Typical scopes include:

- always-required platform and interface audio;
- frontend shell and menu audio;
- active gameplay-common audio;
- current world or region audio;
- mission, race, interior, or cinematic audio;
- controlled and nearby vehicle audio;
- active character and dialogue audio;
- local-player-specific presentation; and
- Game Feature or mod overlay audio.

Scope names are semantic identities, not ordinal cluster numbers. The same
source
may be retained by several scopes without duplicate ownership.

## Residency request transaction

A residency request executes:

1. validate definition, scope, target, culture, and expected revisions;
1. resolve primary assets, bundles, soft references, and optional variants;
1. calculate incremental memory, decoder, and stream demand;
1. retain required streamable handles;
1. issue bounded asynchronous loads or primes;
1. validate every completion against the active request;
1. publish immutable readiness evidence;
1. commit the scope lease atomically; and
1. release replaced or cancelled preparation state.

No playback request observes partially committed required residency.

## Shared references and release

Residency ownership is reference-counted by stable scope leases, not raw object
reference counts or namespace membership.

Releasing one scope:

- prevents new scope-owned playback;
- cancels pending loads and starts;
- waits for or explicitly terminates protected active sources according to
  policy;
- releases only handles no longer required by another scope;
- invalidates stale callbacks; and
- publishes one terminal scope result.

An active required dialogue line, cinematic section, music state, or protected
vehicle loop cannot be evicted silently.

## Sound roles and class hierarchy

Every source resolves to an approved Sound Class hierarchy. Common semantic
classes include:

- master;
- music;
- ambience;
- dialogue;
- cinematic;
- vehicle;
- gameplay effects;
- user interface;
- accessibility; and
- development-only diagnostics.

Sound Classes group role policy and may supply volume, pitch, filters, routing,
loading, and passive mix behavior. They do not define gameplay identity.

## Mix policy

`USharAudioMixPolicy` declares:

- canonical mix identity and revision;
- eligible application modes and semantic reasons;
- Sound Mix, modulation, Control Bus, or registered native implementation;
- affected Sound Classes and submixes;
- volume, pitch, filter, and send targets;
- fade-in, hold, fade-out, and supersession policy;
- local-player or global scope;
- priority and composition behavior;
- pause, focus, output-device, and recovery behavior; and
- required teardown and fallback.

Mix transitions use seconds or sample-aligned clocks where required. Frame count
and callback order cannot alter fade duration or priority.

## Application-mode mixes

Application modes request semantic mix states such as:

- boot;
- frontend;
- loading;
- gameplay;
- pause;
- cinematic;
- store or reward browser;
- local multiplayer;
- demonstration;
- focus loss;
- background; and
- recovery.

The application coordinator requests a mix revision but does not modify Sound
Classes or submix effect chains directly.

## Ducking

Dialogue, cinematics, mission presentation, accessibility, or other accepted
roles may request a bounded ducking lease.

A ducking lease declares:

- requesting role and owner;
- affected classes or submixes;
- target gain and optional filter policy;
- attack, hold, release, and interruption behavior;
- local-player or global scope; and
- automatic teardown reason.

Duplicate requests compose according to policy. One completion callback cannot
release another owner's ducking lease.

## Submix graph

The submix graph is an authored native asset graph. It may provide:

- master routing;
- music, dialogue, vehicle, ambience, effects, UI, and cinematic branches;
- reverb and room sends;
- sidechain or ducking buses;
- platform output variants;
- accessibility processing;
- development analysis; and
- capture or streaming adapters where separately authorized.

Runtime code selects approved sends and effect-chain overrides. It does not
reconstruct a platform-specific manual DSP graph.

## Environmental audio definition

`USharAudioEnvironmentDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `EnvironmentId` | Canonical room, tunnel, interior, exterior, or authored environment identity. |
| `VolumePolicy` | Audio Volume, room, portal, Data Layer, or fixed semantic-region binding. |
| `Priority` | Deterministic overlap and nesting priority. |
| `ReverbPolicy` | Reverb effect, plugin effect, submix send, or effect-chain override. |
| `WetLevel` | Bounded reverb or send level. |
| `TransitionPolicy` | Entry, exit, interpolation, hold, and supersession behavior. |
| `SourcePolicy` | Which source roles and positions are affected. |
| `ListenerPolicy` | Which listeners or local players are affected. |
| `InteriorPolicy` | Filtering, ambience, portals, and room relationships. |
| `PlatformPolicy` | Verified target implementation and fallback. |
| `EnvironmentRevision` | Immutable revision for stale transition rejection. |

<!-- markdownlint-enable MD013 -->

Environment identity is semantic. Source platform room names and hard-coded
string comparisons remain conversion evidence only.

## Audio Volumes and room policy

Audio Volumes or registered room adapters provide spatially authored environment
membership. Overlap resolution is deterministic from priority, containment,
listener identity, world revision, and policy.

An accepted transition may update:

- reverb effect and send;
- submix effect-chain override;
- source and listener filtering;
- interior and exterior ambience;
- room and portal routing; and
- fade or crossfade state.

Entering a volume cannot complete an objective, prove an interior transaction,
or
change world authority.

## Reverb transitions

Reverb transitions are revisioned mix transactions. They declare current and
target environments, wet level, fade duration, time source, local-player scope,
output-device revision, and cancellation policy.

Pause may freeze, continue, or replace the transition according to application
policy. Unpause resumes from accepted state rather than restarting an arbitrary
platform controller.

A stale fade update cannot overwrite a newer environment or output-device
revision.

## Platform behavior

One semantic environment policy may select different verified native effects or
plugins by platform. Target differences are implementation details with explicit
validation and fallback.

The runtime does not ship separate GameCube, PlayStation, Windows, or Xbox
reverberation controllers as domain authority. Unsupported effects degrade to a
validated native fallback or fail the owning optional presentation feature.

## Pause, suspension, and output devices

Each source, residency scope, mix, and environment declares behavior for:

- gameplay pause;
- frontend modal pause;
- cinematic pause;
- application focus loss;
- platform suspension;
- background audio permission;
- output-device removal or replacement; and
- audio-device recovery.

Resume validates source, owner, world, mode, listener, output, and asset
revisions.
No source resumes into a replacement world merely because a native player was
previously paused.

## Concurrency and voice pressure

Native Sound Concurrency assets declare maximum counts, resolution policy,
retrigger behavior, virtualization, and volume scaling. Project policy adds
semantic priority, protected roles, local-player scope, and typed diagnostics.

When capacity is unavailable, the result is explicit:

- admitted;
- virtualized;
- queued;
- rejected by concurrency;
- rejected by voice budget;
- evicted by policy;
- cancelled;
- superseded; or
- failed.

Required dialogue and accepted cinematic barriers follow their protected policy.
Optional ambience or repeated effects may be dropped deterministically. A silent
drop cannot be reported as successful audible playback.

## Memory and stream pressure

The residency service measures:

- compressed and decoded memory;
- stream-cache occupancy;
- retained handles and bundle references;
- active and queued decoders;
- start and seek latency;
- underruns and starvation;
- active, virtualized, and stopping voices; and
- per-role and per-scope demand.

Pressure response follows explicit eviction and quality policy. It never removes
required active content, changes semantic selection, or leaks another scope's
assets.

## Development diagnostics

Development-only diagnostics may expose:

- audio-device and output revision;
- active, virtualized, paused, stopping, and queued sources;
- source definitions, owners, roles, worlds, local players, and features;
- Sound Classes, mixes, submixes, sends, buses, and effect chains;
- residency scopes, bundles, handles, memory, cache, and readiness;
- attenuation, concurrency, significance, and voice decisions;
- listener and nearby-source transforms;
- environment membership, priority, reverb, and fade state;
- load, start, underrun, starvation, and completion timing; and
- stale callback, capacity, fallback, and teardown findings.

Diagnostics use native audio statistics, repository snapshots, logs, and bounded
overlays. A debug page cannot retain raw source pointers, force playback, change
mixes, load bundles, or become shipping runtime authority.

## Diagnostic overlays

Nearby-source labels, voice counts, memory summaries, role pages, and
environment
visualization are registered with
<!-- markdownlint-disable-next-line MD013 -->
[Developer command and diagnostic runtime](developer-command-and-diagnostic-runtime.md).

Overlay visibility, page order, screen position, line count, and color are user
or
development settings. They cannot alter source significance, listener position,
concurrency, or residency.

## Headless and server operation

Dedicated servers execute semantic events and domain transactions without an
audio device. Audio requests may be omitted, recorded as non-authoritative
telemetry, or forwarded as client presentation cues according to network policy.

Server success never depends on a source becoming audible or an audio callback.

## Networking

Networked semantic events carry canonical source, owner, world, participant, and
causation identities. Clients validate local policy, assets, listener, quality,
and relevance before presentation.

Replicated playback does not replicate native component pointers, current voice
slots, stream handles, random-call state, or effect-chain instances.

## Feature and mod overlays

Validated overlays may add namespaced:

- source definitions;
- residency definitions and bundles;
- Sound Classes beneath approved extension points;
- Sound Mix and modulation policies;
- submix sends or effect presets at registered extension points;
- environment definitions;
- collision-audio profiles; and
- diagnostic views.

They cannot replace the master audio device, base class hierarchy, unrelated
sources, residency scopes, platform policy, or another feature's active leases.

Feature removal cancels requests, stops or migrates owned sources, releases
residency and mix leases, clears environment overrides, releases handles,
unregisters definitions, and rejects stale callbacks as one transaction.

## Failure behavior

The boundary fails closed on:

- missing or duplicate canonical identity;
- unsupported source class, parameter, route, attenuation, concurrency, or mix;
- raw resource name used as runtime authority;
- invalid or non-finite pitch, gain, filter, distance, probability, fade, or
  send;
- unbounded continuous source lifetime;
- required playback without a source lease;
- required source without accepted residency;
- fixed cluster or player capacity without typed failure;
- stale load, start, stop, parameter, pause, environment, or completion
  callback;
- queue order depending on insertion or callback order;
- global random state selecting optional playback;
- protected active content evicted silently;
- mix or reverb state retained after scope teardown;
- platform-specific controller selected without verified policy;
- debug state changing playback or residency; and
- audio completion used as gameplay success.

Failure returns typed evidence and releases all partially acquired native
resources.

## Validation

Definition and cook validation prove:

- every source, role, parameter, bundle, mix, class, submix, environment, and
  collision-audio identity resolves;
- source assets and target variants cook for every supported platform;
- every continuous source has bounded lease and teardown policy;
- every positional source has valid attenuation and listener policy;
- concurrency and priority policy is complete;
- optional probability selection is deterministic;
- residency bundles contain every required dependency;
- required activation fits measured memory, decoder, stream, and voice budgets;
- shared scope references retain and release assets correctly;
- Sound Class and submix graphs contain no cycles or unauthorized routes;
- mix and environment transitions have finite times and deterministic priority;
- platform effects have validated fallback;
- diagnostics are excluded or inert in shipping builds;
- no raw resource, pointer, array, callback, or platform enum becomes identity;
  and
- identical inputs produce identical definition and bundle digests.

## Tests

Required automated, integration, platform, and visual tests include:

- one-shot request, start, completion, cancellation, and teardown;
- queued request admission, expiry, supersession, and deterministic tie-break;
- continuous source lease renewal, replacement, fade, and release;
- parameter bounds, smoothing, and stale update rejection;
- deterministic optional-playback probability;
- positional and non-positional source selection;
- collision-audio lookup and duplicate suppression;
- residency load, readiness, overlapping scopes, cancellation, and release;
- required and optional bundle failure;
- stream-cache pressure and protected source retention;
- Sound Class, Sound Mix, submix, bus, and modulation routing;
- dialogue and cinematic ducking composition and independent release;
- Audio Volume overlap and deterministic environment priority;
- reverb entry, exit, pause, resume, cancellation, and output-device
  replacement;
- platform fallback without semantic change;
- concurrency admission, virtualization, queue, eviction, and rejection;
- world, local-player, mode, and feature teardown;
- headless server execution without an audio device;
- diagnostics with no behavior change; and
- identical results across clean cooks and deterministic replay.

## Invariants

- Unreal owns decoding, playback, mixing, effects, voices, and hardware output.
- Canonical source definitions replace raw resource names and player slots.
- Every continuous source has a bounded typed lease.
- Every accepted request reaches one terminal result exactly once.
- Residency is scope-owned, revisioned, reference-safe, and handle-retained.
- Fixed sound clusters and ordinal cluster names are not runtime architecture.
- Sound Classes, mixes, submixes, buses, and environments are authored native
  policy, not gameplay identity.
- Optional source selection is deterministic and independent of global random
  order.
- Reverb and environmental effects are presentation only.
- Diagnostics are read-only and development-scoped.
- Audio completion cannot commit gameplay, progression, save, or world state.
- World, mode, player, output, and feature teardown release every owned source,
  handle, mix, environment, callback, and diagnostic binding.

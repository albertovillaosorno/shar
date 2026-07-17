# Spatial audio listener and positional-source runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Native gameplay audio, dialogue, and listener boundary](../../adr/unreal/runtime/native-gameplay-audio-dialogue-and-listener-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native render-frame, view, and layer runtime](native-render-frame-view-and-layer-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Camera rig, preset, and arbitration runtime](camera-rig-preset-and-arbitration-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform audio cooking and streaming](platform-audio-cooking-and-streaming.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Gameplay audio source, residency, mix, and environment runtime](gameplay-audio-source-residency-mix-and-environment-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)

## Purpose

This specification defines native Unreal audio listeners, local-player and
frontend listener policy, shared split-screen mixing, moving positional sources,
Actor and component attachment, transforms, velocities, attenuation,
spatialization, focus, occlusion, filtering, reverb sends, concurrency,
virtualization, streaming, networking, diagnostics, and teardown.

It replaces one process-global listener object, hard-coded player-zero camera
selection, cached raw frontend camera pointers, arbitrary camera-to-avatar
clamps,
manual sound-vector conversion, raw moving-Actor pointers, and per-frame source
managers that can outlive their world owners.

The audio subsystem projects accepted views and sources. It does not own camera,
player, Actor, movement, world, mission, or persistence state.

## Native Unreal foundation

The boundary uses native Unreal facilities:

- audio-device listener transforms and supported listener overrides;
- `ULocalPlayer`, player controllers, and accepted camera-manager output;
- `UAudioComponent` for attached and controlled positional sources;
- Sound Attenuation assets for volume, spatialization, focus, occlusion,
  filtering,
  priority, reverb sends, and source behavior;
- Sound Concurrency and platform voice management;
- Actor, scene-component, socket, and bone attachment;
- native audio virtualization and component lifecycle;
- World Partition, Runtime Data Layers, level instances, and Game Features;
- Asset Manager bundles and retained handles; and
- game-instance, world, and local-player subsystem lifetimes.

Repository code supplies stable identities, listener arbitration, source policy,
immutable snapshots, typed lifecycle results, and diagnostics. It does not
replace
native spatialization or the platform audio device.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Camera and view runtime | Owns accepted camera transforms, velocities, local-player views, and presentation-camera revisions. |
| Participant and vehicle services | Own accepted focus anchors, controlled participants, transforms, and movement observations. |
| Listener-policy catalog | Owns frontend, gameplay, cinematic, split-screen, accessibility, and platform listener rules. |
| Spatial-audio subsystem | Validates listener candidates, selects the accepted mix policy, and projects listener state. |
| Positional-source service | Validates source definitions, attachments, leases, updates, and teardown. |
| Unreal Audio Engine | Owns listener consumption, spatialization, attenuation, occlusion, routing, concurrency, mixing, virtualization, and output. |
| World-composition service | Owns source-region readiness, streaming, overlays, and teardown. |
| Domain services | Own gameplay, missions, movement, damage, progression, and persistence. |

<!-- markdownlint-enable MD013 -->

A listener or source transform is presentation evidence, never world authority.

## Runtime identities

The boundary uses stable identities for:

- `FSharAudioListenerPolicyId`;
- `FSharAudioListenerPolicyRevision`;
- `FSharAudioListenerId`;
- `FSharAudioListenerRevision`;
- `FSharAudioListenerCandidateId`;
- `FSharAudioListenerMixId`;
- `FSharLocalPlayerId`;
- `FSharViewId`;
- `FSharCameraLeaseId`;
- `FSharPositionalSourceDefinitionId`;
- `FSharPositionalSourceId`;
- `FSharPositionalSourceRevision`;
- `FSharAudioSourceLeaseId`;
- `FSharAttachmentRevision`;
- `FSharWorldCompositionRevision`;
- `FSharFeatureRevision`; and
- `FSharSpatialAudioResultId`.

Player array indices, raw camera pointers, cached Actor addresses,
source-manager
slots, sound-renderer handles, and callback order are not durable identity.

## Listener-policy definition

`USharAudioListenerPolicy` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `PolicyId` | Canonical listener-policy identity. |
| `ApplicationModes` | Frontend, loading, gameplay, pause, cinematic, capture, or recovery eligibility. |
| `CandidateSources` | Accepted camera, participant, vehicle, cinematic, frontend, or fixed listener candidates. |
| `SplitScreenMode` | Independent, shared, primary, weighted, focus-owner, or another registered platform strategy. |
| `TransformPolicy` | Position, orientation, velocity, focus-anchor, and bounded offset rules. |
| `CameraParticipantPolicy` | Relationship between camera and controlled participant for audible focus. |
| `TransitionPolicy` | Crossfade, interpolation, snap, hold, and supersession behavior. |
| `AttenuationScalePolicy` | Optional world, mode, accessibility, or platform scaling. |
| `InteriorPolicy` | Interior, vehicle cabin, room, portal, and camera-volume mix behavior. |
| `AccessibilityPolicy` | Mono, focus, reduced dynamic range, subtitle, and orientation alternatives. |
| `PlatformPolicy` | Supported listener count, output device, latency, and fallback. |
| `DiagnosticsPolicy` | Read-only inspection and capture permissions. |
| `DefinitionRevision` | Immutable revision for stale-result rejection. |

<!-- markdownlint-enable MD013 -->

Policies reject unknown candidate classes, ambiguous split-screen behavior,
invalid transforms, unbounded offsets, missing platform fallback, and incomplete
transition or teardown.

## Listener candidate

`FSharAudioListenerCandidate` contains:

- candidate, listener-policy, local-player, view, and camera identities;
- expected camera and view revisions;
- application mode, world, region, feature, and presentation revisions;
- position, forward, right, up, and velocity in canonical world units;
- controlled participant and optional focus-anchor identities;
- participant and focus-anchor positions and revisions;
- frontend, gameplay, cinematic, capture, or fixed-source classification;
- interior, vehicle-cabin, room, and audio-volume evidence;
- validity interval and timestamp;
- output-device and platform context; and
- diagnostics correlation.

Candidates are immutable. The spatial-audio subsystem never reads a mutable
camera or participant pointer after acceptance.

## Listener validation

A candidate is valid only when:

- local-player and view ownership agree;
- camera and view revisions are current;
- position and orientation vectors are finite;
- forward and up vectors are non-zero and form a valid basis after bounded
  normalization;
- velocity is finite and belongs to the same coordinate and time revision;
- world and application mode are accepted;
- participant or focus-anchor references resolve when required;
- platform listener policy supports the selected strategy; and
- teardown has not begun.

An invalid candidate returns typed evidence. It is not silently replaced with a
magic axis or stale camera pointer unless the policy declares a bounded
fallback.

## Listener transform

The accepted transform contains:

- world-space position;
- normalized forward and up vectors;
- derived right vector;
- linear velocity;
- optional focus direction;
- world-to-meters and coordinate revision;
- interpolation state; and
- listener revision.

Coordinate conversion occurs once at the adapter boundary. Repository services
do
not maintain a second audio coordinate system.

## Camera and participant relationship

A gameplay listener may use the accepted camera transform directly or a declared
camera-participant policy. Supported policies include:

- camera position and orientation;
- participant position with camera orientation;
- bounded camera position relative to participant;
- vehicle cabin anchor with camera orientation;
- cinematic camera;
- fixed authored listener; and
- non-spatialized presentation.

A bounded camera-to-participant offset is content and mode policy, not a
hard-coded
process constant. It records maximum distance, interpolation, obstruction,
vehicle, split-screen, and exception rules.

The listener never moves the camera or participant to satisfy audio policy.

## Frontend listener

Frontend audio uses an accepted frontend camera or authored listener definition.
It does not locate one object by a magic runtime name and cache the raw pointer
across screen or world replacement.

The frontend listener becomes active only after shell, camera, viewport, audio,
and mode revisions agree. Leaving the frontend releases its exact listener lease
and invalidates late callbacks.

## Gameplay listener

Each local player publishes one candidate from its accepted view and controlled
participant context. The listener policy then selects how candidates contribute
to the platform audio mix.

A player without a valid camera may retain the last accepted transform only for
a
bounded policy interval. Beyond that interval the listener is rejected or uses
an
explicit non-spatialized fallback.

## Split-screen policy

Split-screen never silently collapses to player zero. The selected platform and
output policy declares one of:

- independent listener transforms when supported and validated;
- one shared transform derived from bounded candidate aggregation;
- a primary local-player listener with declared secondary-player compensation;
- focus-owner arbitration that changes only at a stable safe point;
- non-spatialized shared presentation for selected roles; or
- another registered and tested policy.

Aggregation uses deterministic weights and stable player identities. Camera
creation order cannot select the primary listener.

Required local-player dialogue, vehicle, UI, and accessibility presentation must
remain intelligible under the chosen policy.

## Listener arbitration

When multiple candidates compete for one listener role, arbitration uses:

1. application-mode eligibility;
1. explicit ownership or focus lease;
1. local-player priority policy;
1. candidate validity;
1. stable view and candidate identities; and
1. declared fallback.

Arbitration commits at one audio safe point. A late camera update cannot
partially
replace position without orientation or velocity from the same revision.

## Listener transitions

A listener change declares:

- source and destination listener revisions;
- transition reason;
- interpolation duration and curve;
- position, orientation, velocity, focus, and mix behavior;
- teleport or snap threshold;
- pause and focus-loss behavior;
- stale-result policy; and
- terminal result.

Teleport, vehicle entry, interior travel, respawn, camera cut, local-player
change,
and frontend transition are explicit reasons. Interpolation cannot smear a
required hard camera cut beyond policy.

## Positional-source definition

`USharPositionalAudioSourceDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `DefinitionId` | Canonical positional-source identity. |
| `SoundSource` | Class-validated Sound Wave, Sound Cue, MetaSound, or dialogue source. |
| `AttachmentPolicy` | Actor, component, socket, bone, world snapshot, spline, or fixed authored anchor. |
| `AttenuationPolicy` | Required Sound Attenuation asset and optional bounded overrides. |
| `ConcurrencyPolicy` | Native Sound Concurrency and project significance policy. |
| `LifetimePolicy` | One-shot, finite, leased continuous, attached continuous, or owner-scoped persistent. |
| `VelocityPolicy` | Component, movement observation, derived, fixed, or disabled Doppler evidence. |
| `OcclusionPolicy` | Native asynchronous trace, portal, room, or declared plugin policy. |
| `ReverbPolicy` | Audio-volume, attenuation-send, room, and submix behavior. |
| `ListenerPolicy` | Local-player, shared, primary, cinematic, or global audibility. |
| `VirtualizationPolicy` | Offscreen, inaudible, distance, pause, and resume behavior. |
| `StreamingPolicy` | Asset bundles, preload, residency, and eviction. |
| `NetworkPolicy` | Authority, replication, prediction, and local-only presentation. |
| `QualityPolicy` | Platform variants and optional fallbacks. |
| `DefinitionRevision` | Immutable revision for stale-result rejection. |

<!-- markdownlint-enable MD013 -->

Definitions reject missing attenuation for required positional playback,
unbounded continuous lifetime, invalid attachments, ambiguous listener
ownership,
and incomplete teardown.

## Source request

`FSharPositionalAudioRequest` contains:

- request, definition, owner, and source identities;
- expected definition and owner revisions;
- world, region, mode, local-player, and feature revisions;
- Sound source and bundle identity;
- Actor, component, socket, or world-space snapshot;
- position, orientation, and velocity evidence;
- start time, duration, looping, and presentation-time policy;
- typed source parameters;
- attenuation, concurrency, listener, and mix policy;
- cancellation token, deadline, and priority; and
- diagnostics correlation.

Requests are immutable. An update publishes a replacement source snapshot or
parameter transaction rather than mutating an unvalidated pointer.

## Source lifecycle

The lifecycle uses the closed states:

1. `requested`;
1. `resolving`;
1. `loading`;
1. `prepared`;
1. `starting`;
1. `playing`;
1. `virtualized`;
1. `paused`;
1. `stopping`;
1. `completed`;
1. `cancelled`;
1. `released`; and
1. `failed`.

Every accepted source reaches one terminal result exactly once.

## Actor and component attachment

Attached sources resolve a weak native Actor or scene-component reference only
at
commit. The binding records:

- owner and component identities and revisions;
- socket or bone identity;
- relative transform;
- attachment and detachment rules;
- owner hidden, disabled, destroyed, pooled, or replaced behavior;
- world and feature ownership; and
- fallback when the attachment is unavailable.

A stale source cannot reattach to a replacement Actor that reused memory or a
pool
slot.

## World-space sources

A world-space source uses an immutable transform and optional velocity snapshot.
It may remain after the originating event only for its declared finite lifetime.

Moving world-space sources publish revisioned transform updates. Update rate and
interpolation are bounded by presentation policy. A per-frame callback is not
the
source's lifetime authority.

## Moving Actor source player

A moving Actor source adapter observes one accepted Actor or component revision
and projects it into a native audio component. It tracks:

- source and owner identities;
- attachment or transform snapshot;
- velocity source and timestamp;
- audio parameters and playback state;
- listener and significance policy;
- world and feature revisions; and
- cancellation and teardown evidence.

The adapter does not poll every possible Actor through one process-global
manager. World and feature subsystems own bounded source collections.

## Moving-source families

Registered moving-source families include:

- ambient and artificial-intelligence traffic vehicles;
- mission, chase, emergency, or scripted vehicles;
- the currently controlled avatar vehicle and its positional idle layer;
- animated props and collision entities with optional joint attachment;
- moving platforms and mechanical world objects;
- flying hazards and stateful hostile actors;
- vehicle horns, sirens, overlays, backup warnings, and explosion tails; and
- namespaced Game Feature source families.

Each family resolves a source definition, attachment class, listener policy,
priority, concurrency, residency, and lifetime. Source strings and source-class
branches are import evidence only.

Traffic and artificial-intelligence vehicles publish immutable spawn, despawn,
movement, horn, overlay, damage, and destruction observations. Presentation may
start, replace, virtualize, or release a source lease, but it cannot spawn or
remove the vehicle, choose its route, apply damage, or commit destruction.

The avatar-vehicle positional layer follows the accepted controlled-vehicle and
local-player revisions. Vehicle entry, exit, replacement, respawn, world travel,
and local-player removal release or transfer the lease through an explicit
transaction; no global event listener keeps the previous vehicle alive.

Animated-object and platform sources bind to a validated Actor, component,
socket,
bone, or pose joint at commit. Missing authored settings produce a typed
optional
failure or owning-feature activation failure rather than a raw lookup warning.

## Stateful hazard audio

A stateful hazard may define an audio presentation graph with states such as
idle,
fade-in, charging, charged, attack, damage, destroyed, and fade-out. Each edge
declares trigger, source and destination state, interruption, source asset,
parameters, fade, looping, and terminal behavior.

Native playback completion may advance only the accepted presentation edge. It
cannot change hazard artificial intelligence, health, attack eligibility,
destruction, reward, mission, or persistence state.

Late completion from a replaced hazard, retired source lease, unloaded world, or
removed feature is rejected by source, Actor, world, and feature revisions.

## Capacity and source admission

Moving-source capacity is governed by Sound Concurrency, significance, voice,
stream, and project source-budget policy rather than fixed traffic, artificial-
intelligence, platform, animated-object, or hazard arrays.

Admission returns an explicit result such as admitted, virtualized, queued,
rejected by concurrency, rejected by source budget, cancelled, or superseded.
An optional source may be dropped deterministically; a required source cannot be
silently lost because the first free array slot was unavailable.

Horn cooldowns, traffic overlays, sirens, and timed source changes use typed
timers
or leases owned by the source instance. A process-global timer list cannot
mutate
a source after its owner or world has been released.

## Velocity and Doppler evidence

Velocity may come from an accepted movement observation, native component
velocity, or bounded transform difference. The policy declares units, timestamp,
filtering, teleport rejection, and maximum magnitude.

A transform jump beyond the teleport threshold resets derived velocity. Invalid
or stale velocity disables Doppler for that update rather than emitting an
unbounded value.

## Attenuation

Positional sources use validated Sound Attenuation assets for:

- distance-volume curves;
- attenuation shape and extents;
- spatialization;
- non-spatialized radius;
- listener focus;
- distance filtering;
- occlusion;
- priority scaling;
- reverb sends;
- air absorption;
- virtualization; and
- plugin-specific settings when accepted.

Repository code may select or parameterize an approved asset. It does not
reimplement attenuation with ad hoc distance checks.

## Spatialization

Spatialization is enabled only for roles and source formats that support it.
The platform policy selects native panning or an accepted plugin. Mono, stereo,
multichannel, binaural, ambisonic, and non-spatialized sources follow verified
target rules.

Changing the spatialization implementation cannot change source identity,
semantic event, subtitle, or gameplay result.

## Occlusion and obstruction

The default policy prefers native asynchronous occlusion traces through Sound
Attenuation. More advanced portal, room, or plugin behavior requires an accepted
adapter and typed configuration.

Occlusion affects presentation only. It cannot prove line of sight, interaction,
mission reachability, damage, stealth, or artificial-intelligence perception.

A stale trace result is rejected by source, listener, world, and query revision.

## Listener focus

Listener-focus policy may adjust volume, priority, filtering, or other approved
presentation based on source direction relative to the listener. It cannot
become
camera targeting, aim, interaction, or artificial-intelligence authority.

Focus settings are bounded and included in the attenuation-policy revision.

## Reverb, rooms, and interiors

Reverb and interior presentation may use Audio Volumes, attenuation sends,
submixes, room or portal adapters, and typed interior observations.

A vehicle cabin, interior portal, tunnel, room, or frontend shell changes audio
presentation only after its owning world or application transaction commits.
Audio-volume overlap cannot move the participant or complete an interior
transition.

## Concurrency and significance

Native Sound Concurrency assets limit source count and define resolution
behavior.
Project significance may additionally consider:

- semantic role;
- local-player ownership;
- mission or accessibility importance;
- distance and audibility;
- source age;
- world-region readiness;
- platform voice budget; and
- stable source identity.

Capacity outcomes are typed. Required dialogue or local-player vehicle audio is
protected according to explicit policy, never by array position.

## Virtualization

Virtualization may preserve a looping source when inaudible or over budget.
The definition declares whether time advances, parameters continue, restart is
allowed, and resumption must preserve phase.

Virtualization is not completion. It cannot release a gameplay lease or publish
a
successful dialogue result.

## Local-player and shared sources

Every source declares local-player ownership or shared audibility. A local
source
cannot leak private UI, navigation, accessibility, or dialogue presentation to
another local player through an implicit global listener.

Shared world sources use the accepted split-screen listener policy and remain
one
semantic source identity even when the engine evaluates more than one listener.

## Frontend and cinematic sources

Frontend and cinematic sources bind to their accepted listener and application
scope. Camera cuts, media timelines, and shell transitions are revisioned.

A cached frontend camera or cinematic Actor pointer cannot survive scope
teardown.
Non-positional fallback is explicit when spatial presentation is unnecessary.

## Pause, focus, suspension, and output devices

Each listener and source definition declares pause, focus-loss, suspension,
output
change, and resume behavior.

Output-device change may rebuild platform audio state while preserving semantic
listener and source identities. Resume validates every world, player, view,
camera, source, attachment, and feature revision before restoring playback.

## Streaming and world lifecycle

World-region activation prepares eligible source definitions and optional
resident assets. Source commit requires the target region and owner to remain
accepted.

Region unload:

1. rejects new owned requests;
1. cancels pending loads;
1. stops, virtualizes, migrates, or releases active sources by policy;
1. releases retained handles;
1. invalidates listener and source callbacks; and
1. verifies zero owned registrations.

A late load cannot recreate a source in an unloaded world.

## Networking

The authoritative owner replicates semantic source events and accepted
transforms
when required. Clients create local native audio components from canonical
identities and definitions.

Replication does not carry raw component pointers, audio-device handles, source
paths, or listener transforms owned by another client. Late updates are rejected
by source and world revision.

Headless servers may validate and route semantic audio events without creating
listeners or native playback components.

## Feature and mod overlays

A validated feature may add namespaced listener policies, positional-source
definitions, attenuation assets, concurrency assets, mixes, and platform
fallbacks. It cannot replace the process audio device, intercept unrelated
sources, mutate a base listener policy in place, or leave callbacks after
removal.

Feature removal cancels owned requests, releases sources and handles, restores
scoped listener and mix state, unregisters definitions, and rejects stale
results
atomically.

## Platform and quality policy

Quality policy may change:

- spatialization implementation;
- occlusion frequency and method;
- reverb and filtering cost;
- voice and concurrency budgets;
- virtualization distance;
- interpolation and update frequency;
- optional source layers; and
- diagnostic visualization.

It cannot change listener ownership, semantic source identity, required
dialogue,
local-player privacy, world attachment, or gameplay results.

## Diagnostics

Development diagnostics expose:

- listener policy, candidate, listener, mix, local-player, view, camera, world,
  feature, source, definition, and lease revisions;
- accepted and rejected listener candidates;
- position, orientation, velocity, focus, and interpolation state;
- split-screen policy and deterministic weights;
- source attachments, transforms, velocities, parameters, and lifecycle states;
- attenuation, spatialization, occlusion, focus, reverb, concurrency, and
  virtualization policy;
- active native audio components and retained handles;
- stale callback, stale trace, and invalid transform counts;
- capacity, fallback, cancellation, and teardown outcomes; and
- last terminal result.

Diagnostics are read-only. They cannot select a player, move a listener or
source,
force audibility, bypass attenuation, or alter source lifetime.

## Failure behavior

The subsystem fails closed on:

- unknown or duplicate listener or source identity;
- hard-coded player-zero ownership without explicit policy;
- stale view, camera, participant, world, feature, source, or attachment
  revision;
- non-finite position, orientation, velocity, or transform;
- zero-length or invalid orientation basis with no declared fallback;
- unbounded camera-to-participant offset;
- unsupported split-screen listener policy;
- missing required attenuation or concurrency asset;
- positional source with no valid attachment or world snapshot;
- continuous source without bounded lease and teardown;
- stale occlusion, interpolation, or load result;
- listener or source callback attempting to mutate gameplay;
- duplicate terminal result;
- output or focus transition with unresolved required source; and
- world or feature teardown with retained listeners, components, or handles.

Failure leaves camera, participant, world, and gameplay state unchanged.

## Validation

Cook and content validation proves:

- every listener and source definition has stable identity;
- every application mode and local-player policy is explicit;
- every split-screen strategy has platform support or deterministic fallback;
- every orientation and coordinate policy is valid;
- every positional source has attenuation, concurrency, lifetime, listener, and
  teardown policy;
- every attachment class and socket resolves or has explicit fallback;
- every required audio asset cooks for each target;
- every continuous source has bounded lease and virtualization behavior;
- every feature namespace unregisters completely; and
- no runtime source manager or raw camera lookup is required.

## Tests

Required automated tests include:

- frontend listener activation and teardown;
- gameplay listener from camera and participant policy;
- invalid and zero-length orientation rejection;
- bounded camera-to-participant offset;
- teleport and camera-cut transitions;
- local-player creation, removal, and focus arbitration;
- independent, shared, primary, and weighted split-screen policies;
- deterministic candidate tie-breaking;
- Actor, component, socket, and world-space source attachment;
- owner replacement and stale attachment rejection;
- derived velocity, teleport reset, and Doppler policy;
- attenuation, focus, filtering, occlusion, and reverb sends;
- stale occlusion result rejection;
- concurrency, significance, virtualization, and resumption;
- frontend, gameplay, cinematic, and non-positional sources;
- pause, focus loss, suspension, output change, and resume;
- region unload during load and playback;
- feature removal and callback invalidation;
- network late-update rejection;
- headless semantic event routing; and
- complete listener, source, component, and handle teardown.

## Invariants

- Camera and participant services own transforms; audio only projects them.
- No listener policy silently assumes player zero.
- Split-screen behavior is explicit, deterministic, and platform-validated.
- Listener updates are atomic snapshots of position, orientation, and velocity.
- Positional sources use native attachment, attenuation, spatialization,
  concurrency, and output facilities.
- Occlusion and listener focus never become gameplay perception authority.
- Continuous sources have bounded leases and complete teardown.
- Virtualization is not playback completion.
- Raw camera, Actor, component, and audio-device pointers are never durable
  identity.
- Every accepted listener and source transaction reaches one typed terminal
  result.
- Stale callbacks cannot affect replacement players, views, worlds, sources, or
  features.

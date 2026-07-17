# Vehicle audio and avatar-sound runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Native gameplay audio, dialogue, and listener boundary](../../adr/unreal/runtime/native-gameplay-audio-dialogue-and-listener-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform audio cooking and streaming](platform-audio-cooking-and-streaming.md)
- [Vehicle AI and route runtime](vehicle-ai-and-route-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Physical material and impact-response runtime](physical-material-and-impact-response-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)

## Purpose

This specification defines native Unreal vehicle and avatar audio for engine,
idle, gear transition, reverse, in-air, skid, burnout, powerslide, horn, damage,
overlay, backup, and door presentation.

It replaces script-created mutable parameter objects, process-global vehicle
players, fixed player arrays, raw sound-resource names, per-frame manager
polling,
callbacks that inspect mutable vehicle pointers, and audio state machines that
can
be mistaken for vehicle simulation authority.

Vehicle audio consumes accepted vehicle observations. It never selects vehicle
gear, changes speed, applies damage, chooses a physical surface, completes a
mission, or writes persistence.

## Native Unreal foundation

The boundary uses native Unreal facilities:

- `USoundWave`, Sound Cue, or MetaSound source assets;
- `UAudioComponent` for controlled, attached, and looping playback;
- Sound Attenuation assets for distance, focus, occlusion, filtering, and reverb
  behavior;
- Sound Concurrency assets and platform voice management;
- Sound Classes, Sound Mixes, submixes, and parameter modulation;
- Actor, component, socket, and bone attachment;
- native completion, virtualization, and audio-component delegates normalized by
  repository adapters;
- Asset Manager bundles and retained handles; and
- world, local-player, and Game Feature subsystem lifetimes.

Repository code supplies semantic identities, typed parameter schemas, immutable
vehicle observations, deterministic state projection, lifecycle validation,
and diagnostics. It does not implement a second audio mixer.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Vehicle and movement services | Own speed, RPM evidence, throttle, brake, gear, reverse, contact, airborne, skid, damage, horn, door, and replacement state. |
| Physical-material service | Owns accepted tire-contact and surface identity. |
| Vehicle-audio catalog | Owns stable vehicle-audio profiles, layer definitions, curves, assets, and platform policy. |
| Vehicle-audio subsystem | Validates observations, resolves profiles, projects audio state, and manages presentation leases. |
| Unreal Audio Engine | Owns source playback, parameter evaluation, spatialization, attenuation, concurrency, routing, mixing, and output. |
| Spatial-audio subsystem | Owns listener policy and positional-source projection. |
| Application lifecycle | Owns gameplay, pause, frontend, focus, suspension, and teardown leases. |
| Domain services | Own missions, rewards, damage, progression, and persistence. |

<!-- markdownlint-enable MD013 -->

A sound component can observe a vehicle but cannot become its state owner.

## Runtime identities

The boundary uses stable identities for:

- `FSharVehicleAudioProfileId`;
- `FSharVehicleAudioProfileRevision`;
- `FSharVehicleAudioInstanceId`;
- `FSharVehicleAudioInstanceRevision`;
- `FSharVehicleId`;
- `FSharVehicleRevision`;
- `FSharVehicleObservationId`;
- `FSharVehicleAudioLayerId`;
- `FSharVehicleAudioStateId`;
- `FSharVehicleAudioRequestId`;
- `FSharAudioLeaseId`;
- `FSharSurfaceProfileId`;
- `FSharLocalPlayerId`;
- `FSharWorldCompositionRevision`;
- `FSharFeatureRevision`; and
- `FSharVehicleAudioResultId`.

Fixed player slots, gear-array positions, raw resource names, linked parameter
objects, callback addresses, source object pointers, and debug-page indices are
not durable identity.

## Vehicle-audio profile

`USharVehicleAudioProfile` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ProfileId` | Canonical vehicle-audio identity. |
| `CompatibleVehicleTags` | Allowed vehicle archetypes and presentation variants. |
| `EngineLayer` | Required engine source, parameter schema, looping, and pitch policy. |
| `IdleLayer` | Optional dedicated idle source and transition policy. |
| `ShiftLayer` | Optional upshift and downshift sources or graph triggers. |
| `ReverseLayer` | Reverse source, pitch curve, activation, and release policy. |
| `InAirLayer` | Airborne source or engine-parameter override. |
| `SkidLayers` | Surface-group sources, pitch, gain, and contact policy. |
| `HornLayer` | Horn source, repeat, cooldown, concurrency, and ownership policy. |
| `DamageLayer` | Damage source, threshold, gain curve, and recovery policy. |
| `OverlayLayer` | Optional vehicle-specific continuous or one-shot presentation. |
| `BackupLayer` | Optional reverse warning source and vehicle-class eligibility. |
| `DoorLayers` | Open, close, entry, exit, and latch presentation definitions. |
| `PitchModel` | RPM, speed, gear, throttle, and state parameter curves. |
| `ShiftModel` | Gear thresholds, hysteresis, attack, hold, decay, and pitch-drop policy. |
| `SpatialPolicy` | Attachment, attenuation, occlusion, interior, and listener policy. |
| `ConcurrencyPolicy` | Native concurrency assets and project queue limits. |
| `MixPolicy` | Sound Class, submix, ducking, interior, and local-player routing. |
| `StreamingPolicy` | Required bundles, preload, residency, and eviction behavior. |
| `QualityPolicy` | Platform variants and declared optional fallbacks. |
| `DiagnosticsPolicy` | Development telemetry and capture permissions. |
| `DefinitionRevision` | Immutable revision used for stale-result rejection. |

<!-- markdownlint-enable MD013 -->

Profiles reject missing required sources, invalid curves, negative timing,
ambiguous surface groups, incompatible parameter types, unbounded pitch or gain,
undeclared concurrency, and incomplete teardown.

## Audio-layer roles

The closed initial roles are:

- `engine`;
- `idle`;
- `shift_up`;
- `shift_down`;
- `reverse`;
- `in_air`;
- `skid_road`;
- `skid_loose_surface`;
- `burnout`;
- `powerslide`;
- `horn`;
- `damage`;
- `overlay`;
- `backup_warning`;
- `door_open`;
- `door_close`;
- `vehicle_enter`; and
- `vehicle_exit`.

A profile may combine roles in one MetaSound or separate native components, but
role identities and lifecycle remain explicit.

## Vehicle observation

`FSharVehicleAudioObservation` is immutable and contains:

- vehicle and observation identities and revisions;
- world, feature, local-player, and application-mode revisions;
- active, possessed, visible, streamed, and presentation eligibility;
- speed in canonical units and normalized forward speed;
- accepted engine or drivetrain RPM evidence when available;
- throttle, brake, handbrake, and reverse inputs as observations;
- accepted current and requested gear identities;
- grounded, airborne, wheel-contact, and suspension evidence;
- tire slip, longitudinal slip, lateral slip, burnout, and powerslide evidence;
- accepted physical-surface identity for each relevant contact;
- damage, health, disabled, destroyed, and replacement revisions;
- horn, door, entry, exit, overlay, and backup-warning observations;
- transform, linear velocity, attachment component, and socket identity;
- camera-interior and listener-interest evidence;
- simulation time, presentation time, and pause policy; and
- correlation and deduplication keys.

The observation is projected from the accepted native simulation and
presentation
snapshots in
<!-- markdownlint-disable-next-line MD013 -->
[Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md).
The audio subsystem does not query mutable vehicle internals, native solver
objects, wheel arrays, or controller state after accepting it. A new revision
replaces the old snapshot.

## Instance lifecycle

The lifecycle uses the closed states:

1. `requested`;
1. `resolving_profile`;
1. `loading`;
1. `prepared`;
1. `starting`;
1. `active`;
1. `suspended`;
1. `stopping`;
1. `released`; and
1. `failed`.

A vehicle becomes audible only after profile, assets, world, source attachment,
listener policy, concurrency, and mix readiness agree on one revision.

Vehicle replacement, world unload, player removal, feature removal, or profile
replacement invalidates stale callbacks and releases the exact owned components
and handles.

## Engine presentation state

The semantic engine phases are:

- `inactive`;
- `starting`;
- `idle`;
- `driving`;
- `shifting_up`;
- `shifting_down`;
- `reverse`;
- `airborne`;
- `stopping`; and
- `disabled`.

These phases select audio presentation only. They do not overwrite the accepted
vehicle gear or movement state.

Phase arbitration uses priority, expected vehicle revision, entry conditions,
minimum hold, exit conditions, and stable tie-breaking. Callback arrival order
cannot choose the phase.

## Engine source and parameters

The engine layer receives typed parameters such as:

- normalized RPM or speed;
- current gear and gear-normalized progress;
- throttle and load;
- acceleration and deceleration evidence;
- shift phase and progress;
- airborne blend;
- reverse blend;
- damage blend;
- interior blend;
- local-player interest; and
- start, stop, and one-shot triggers.

A MetaSound may evaluate these parameters sample-accurately. A Sound Cue or
multiple audio components may implement an equivalent validated policy. Asset
choice is not gameplay identity.

## Gear and pitch model

Pitch is evaluated from declared curves and bounded parameters. The default
model
may use normalized RPM, normalized speed, current gear, or a validated blend.

Every gear range defines:

- stable gear identity;
- entry and exit thresholds;
- minimum and maximum pitch;
- optional load response;
- optional shift pitch drop;
- hysteresis or damping; and
- reverse incompatibility.

Thresholds must be strictly ordered where order is required. Pitch values,
interpolation, and response times are finite and bounded.

The audio subsystem never infers a vehicle gear solely from sound pitch when an
accepted drivetrain gear exists. A presentation-only estimated gear is allowed
only when the vehicle definition explicitly lacks drivetrain gear evidence and
cannot be reported as vehicle state.

## Shift transitions

Upshift and downshift presentation declares:

- source and destination gear identities;
- trigger evidence;
- attack duration;
- optional delay or hold;
- decay duration;
- pitch-drop curve;
- gain envelope;
- interruptibility;
- supersession rules; and
- terminal fallback.

Hysteresis prevents oscillation around a threshold. A reverse transition,
airborne transition, vehicle reset, or accepted drivetrain change may supersede
a shift according to policy.

A shift sound ending cannot commit the vehicle gear. The next observation is the
only authority for accepted gear state.

## Idle, startup, and shutdown

Idle presentation activates from accepted low-speed, low-RPM, throttle,
grounded,
and engine-enabled evidence. A separate idle source may crossfade with the main
engine layer or the engine graph may expose an idle parameter.

Startup and shutdown are presentation sequences with declared overlap and
interruptibility. Entering a vehicle, respawning, streaming in, or reacquiring a
local player cannot replay startup unless the accepted profile and vehicle
revision permit it.

## Reverse and backup warning

Reverse presentation requires accepted reverse state or validated negative drive
intent according to the vehicle contract. Its pitch curve is bounded by declared
reverse speed and cannot read a magic negative gear ordinal as durable identity.

Backup warning is a separate eligibility-controlled role. It may be used for
specific vehicle classes, accessibility, or traffic policy. It cannot create or
cancel reverse motion.

## Airborne presentation

Airborne presentation consumes grounded and wheel-contact evidence from vehicle
physics. It may blend engine pitch, reduce load, trigger wind or suspension
layers, or use a dedicated source.

A transient contact loss below the policy threshold does not thrash the state.
Landing is accepted only from a new observation and may publish a separate
impact
request to the physical-material response runtime.

## Skid, burnout, and powerslide

Skid presentation consumes accepted slip and surface observations. The policy
separates at minimum:

- road or hard-surface skid;
- dirt or loose-surface skid;
- burnout; and
- powerslide.

Each role declares minimum slip, activation and release hysteresis, speed range,
pitch and gain curves, wheel aggregation, surface fallback, and concurrency.

The audio subsystem does not trace terrain to redefine the accepted physical
surface. A bounded confirmation query may be used only when the observation
explicitly lacks optional presentation metadata and the result cannot affect
physics.

Surface replacement during an active skid crossfades or restarts according to
policy. It never leaves two unbounded loops active.

## Damage presentation

Damage audio consumes accepted health, damage-band, disabled, and destroyed
revisions. The profile declares:

- activation threshold;
- maximum-presentation threshold;
- gain and pitch curves;
- damaged-engine source or graph parameter;
- attack and release timing;
- repair and replacement behavior; and
- interaction with engine and overlay layers.

Damage audio cannot apply damage, request repair, award currency, or decide that
a vehicle was destroyed.

## Horn presentation

Horn requests include vehicle, participant, local-player, world, request, and
input revisions. The horn policy defines one-shot or held behavior, repeat rate,
minimum interval, maximum continuous duration, concurrency, spatial policy, and
release behavior.

Network replication carries semantic horn observations when required. It does
not replicate raw audio-component state or playback position unless an accepted
presentation policy explicitly requires synchronization.

## Door and avatar transitions

Door and avatar-transition presentation consumes accepted entry, exit, open,
close, latch, and occupant observations. Each request names the vehicle, door or
socket, participant, action, world, and result revisions.

Audio completion cannot unlock a door, transfer possession, place a character,
or complete the entry or exit transaction. The action owner commits first and
audio projects the accepted result.

## Overlay layer

An overlay is an optional vehicle-specific continuous or event-driven source,
such as a siren, machinery loop, electric whine, character-specific layer, or
special mission presentation.

The overlay requires a closed parameter schema, explicit owner, bounded
lifetime,
concurrency, mix, network, quality, and teardown policy. It cannot become a free
string hook into unrelated sound assets.

## Avatar ownership and vehicle switching

Each local player's avatar-audio adapter observes the accepted controlled
participant and current vehicle. Switching vehicles performs a revisioned
handoff:

1. freeze new requests against the old vehicle revision;
1. stop or release old owned layers according to fade policy;
1. invalidate old callbacks;
1. resolve the new vehicle profile;
1. prepare required assets and source attachment;
1. commit the new audio instance; and
1. publish one terminal handoff result.

A transient null vehicle during entry, exit, respawn, or streaming does not
allow
an old player to attach to a replacement Actor.

## Artificial-intelligence vehicle audibility

Ambient and artificial-intelligence vehicles use the same profile model with a
separate ownership and significance policy. Audibility may depend on distance,
listener interest, world region, vehicle importance, and platform budget.

A proximity test is presentation evidence only. It cannot spawn, remove, wake,
stop, or reroute the vehicle. Native attenuation, virtualization, and
concurrency
remain the first budget controls.

## Positional-source integration

Every controlled vehicle source follows
<!-- markdownlint-disable-next-line MD013 -->
[Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md).
The binding declares Actor, component, socket, transform, velocity, attenuation,
occlusion, reverb, local-player, and teardown policy.

Interior cameras may apply an accepted interior mix or parameter blend. Camera
state cannot directly mutate the vehicle-audio profile.

## Mixing and concurrency

Vehicle roles use declared Sound Classes, submixes, modulation, and Sound
Concurrency assets. Concurrency may limit by owner, role, importance, distance,
priority, or platform policy.

When capacity is reached, the result is explicit:

- `started`;
- `virtualized`;
- `rejected_by_concurrency`;
- `replaced_lower_priority`;
- `deferred`;
- `cancelled`; or
- `failed`.

A rejected cosmetic layer cannot alter vehicle state. Required local-player
engine presentation either retains capacity or fails activation visibly before
play.

## Pause, focus, and application modes

Each layer declares game-time, real-time, pause, focus-loss, suspension, and
frontend behavior.

Gameplay pause may pause, duck, or continue selected layers. Platform focus loss
uses the platform audio contract. Resumption validates vehicle, world, player,
profile, and source revisions before continuing.

No audio layer continues from a stale vehicle pointer after world or mode
transition.

## Streaming and bundles

Vehicle profiles reference semantic bundles such as:

- `vehicle_audio_definition`;
- `vehicle_audio_core`;
- `vehicle_audio_surface`;
- `vehicle_audio_damage`;
- `vehicle_audio_optional`;
- `vehicle_audio_locale`; and
- `vehicle_audio_diagnostics`.

The local player's active vehicle pins required core assets. Ambient vehicles
use
bounded residency and may fall back to a declared lower-cost profile.

Eviction cannot remove an active required source before a replacement or stop
transaction commits.

## Networking

The authoritative simulation replicates semantic vehicle observations required
by remote presentation. Clients evaluate local audio from accepted observations,
profile revision, listener policy, and quality policy.

Network packets do not transmit raw audio pointers, source filenames, random
seeds without context, or per-frame pitch updates when deterministic local
evaluation is sufficient.

Late observations are rejected by vehicle and instance revision. Local-only
horn, door, or accessibility presentation is explicitly marked.

## Platform and quality policy

Quality policy may change:

- source codec and sample rate;
- optional layers;
- MetaSound graph complexity;
- update frequency within bounded interpolation;
- spatialization implementation;
- occlusion cost;
- voice and concurrency limits;
- virtualization thresholds; and
- ambient-vehicle significance.

It cannot change required local-player engine audibility, shift meaning, surface
classification, horn semantics, damage thresholds, door-event identity, or
vehicle gameplay results.

## Diagnostics

Development diagnostics expose:

- vehicle, profile, observation, instance, world, feature, and player revisions;
- active layers and native component identities;
- semantic engine phase and transition progress;
- speed, RPM, gear, throttle, reverse, airborne, slip, and damage observations;
- evaluated pitch, gain, curves, and clamping;
- surface and skid-role selection;
- horn, door, overlay, and backup requests;
- attenuation, listener, concurrency, virtualization, and mix state;
- retained and released asset handles;
- stale callback and rejected observation counts;
- lifecycle and terminal results; and
- last failure or fallback.

Diagnostics are read-only and cannot force a gear, surface, damage band, horn,
source, or state transition.

## Failure behavior

The subsystem fails closed on:

- unknown or duplicate profile identity;
- stale vehicle, profile, world, feature, player, or observation revision;
- missing required engine source or parameter schema;
- invalid curve, threshold, timing, pitch, gain, or gear ordering;
- unsupported native source class;
- unresolved attenuation, concurrency, class, submix, or bundle dependency;
- source attachment to the wrong vehicle revision;
- unbounded continuous source or horn request;
- skid presentation with no accepted or declared fallback surface;
- raw filename or array index used as runtime identity;
- audio callback attempting to mutate vehicle or gameplay state;
- required local-player audio rejected without policy fallback;
- duplicate terminal result; and
- teardown with retained components, listeners, or handles.

Failure leaves the accepted vehicle state unchanged and emits typed diagnostics.

## Validation

Cook and content validation proves:

- every profile and role has stable identity;
- every required source resolves and cooks for each target;
- parameter names and types match the native source;
- pitch, gain, gear, shift, reverse, skid, damage, and timing curves are finite
  and
  bounded;
- shift thresholds and hysteresis are coherent;
- every continuous layer has activation, release, virtualization, and teardown;
- every surface role has explicit fallback;
- every role has Sound Class, concurrency, attenuation, and mix policy;
- all local-player and ambient ownership rules are explicit;
- all required bundles and platform variants are complete; and
- no runtime source parser or script-created parameter object is packaged.

## Tests

Required automated tests include:

- profile resolution and target cook coverage;
- startup, idle, driving, and shutdown transitions;
- normalized speed and RPM pitch evaluation;
- ordered gear ranges and shift hysteresis;
- upshift and downshift attack, hold, decay, and supersession;
- reverse entry, pitch cap, and release;
- airborne debounce and landing transition;
- road, loose-surface, burnout, and powerslide selection;
- surface change during an active skid;
- damage activation, gain curve, repair, destruction, and replacement;
- horn press, hold, repeat, cooldown, and concurrency;
- door open, close, entry, and exit correlation;
- overlay lifecycle and feature removal;
- vehicle switching and stale callback rejection;
- local-player and ambient-vehicle significance policy;
- pause, focus loss, suspension, and resume;
- concurrency rejection and required-layer protection;
- world unload and handle release;
- deterministic evaluation across supported frame rates; and
- headless server operation with no audio device.

## Invariants

- Vehicle audio consumes immutable observations and never owns vehicle state.
- Semantic audio roles and profile identities are independent of source
  filenames.
- Gear and pitch presentation cannot commit drivetrain state.
- Surface audio cannot redefine physical-material identity.
- Damage audio cannot apply damage or destruction.
- Horn and door playback cannot commit input or interaction transactions.
- Continuous sources have bounded leases and complete teardown.
- Native attenuation, concurrency, routing, mixing, and output remain
  engine-owned.
- Local-player and ambient ownership are explicit and revisioned.
- Every accepted request reaches one typed terminal result.
- Stale callbacks cannot affect replacement vehicles, players, worlds, or
  features.

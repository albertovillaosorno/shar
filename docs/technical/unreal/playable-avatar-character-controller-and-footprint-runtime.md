# Playable avatar, character controller, and footprint runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
<!-- markdownlint-disable-next-line MD013 -->
- [World render-entity and physics runtime](world-render-entity-and-physics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Physical material and impact-response runtime](physical-material-and-impact-response-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Ambient population and named-character runtime](ambient-population-and-named-character-runtime.md)
- [Pedestrian path runtime](pedestrian-path-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle retrieval and phone-booth runtime](vehicle-retrieval-and-phone-booth-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle AI and route runtime](vehicle-ai-and-route-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Camera rig, preset, and arbitration runtime](camera-rig-preset-and-arbitration-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md)

## Purpose

This specification defines playable-avatar ownership, local-player and
controller
assignment, character construction, asset loading, possession, on-foot movement,
input, camera-relative control, artificial-intelligence and non-player-character
control, vehicle enter and exit handoff, transforms, collision, ground support,
physical reactions, animation, rendering, level of detail, materials, shadows,
props, interaction handlers, camera targeting, footprints, streaming,
multiplayer, diagnostics, and teardown.

It replaces source-era process-global avatar and character managers, fixed
avatar
and character arrays, raw controller IDs, custom mappable classes, manual input
button tables, manual locomotion controllers, character pointer lookup, deferred
load slots, per-frame manager loops, custom renderables, raw prop pointers,
manual
collision volumes, one-off ground-plane objects, fixed action-handler arrays,
and
fixed footprint pools.

The target uses native Unreal player, Pawn, Character, movement, input,
animation,
rendering, collision, asset, and lifecycle facilities while preserving stable
semantic identities and project-owned gameplay rules.

## Native Unreal foundation

The implementation uses native facilities where applicable:

- `ULocalPlayer`, Player Controller, Player State, Pawn, and Character;
- Enhanced Input actions, mapping contexts, modifiers, triggers, and per-local-
  player subsystems;
- `UCharacterMovementComponent` or an accepted native movement component;
- Capsule, Skeletal Mesh, Physics Asset, collision profiles, and movement modes;
- Anim Blueprint, Animation Sequence, Montage, Control Rig, root motion, motion
  warping, pose warping, and animation notifies as appropriate;
- AI Controller, StateTree, navigation, and project path adapters;
- Asset Manager bundles, soft references, retained handles, and asynchronous
  loading;
- Gameplay Cameras or the project camera arbitration adapter;
- sockets and components for attached props;
- decals, Niagara, or approved instanced presentation for footprints;
- World Partition, Game Features, Actor and component lifecycle;
- native replication, prediction, and correction for network-enabled mods; and
- Insights, visual logging, collision diagnostics, and development overlays.

A custom movement or animation path requires a separate accepted decision
proving
that native facilities cannot satisfy a required behavior. Source-era class
names
or algorithms are not sufficient justification.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Local-player service | Owns player identity, controller assignment, input scope, and split-screen isolation. |
| Avatar service | Owns the accepted controlled participant, on-foot or vehicle mode, handoff transactions, and camera-target intent. |
| Character definition | Declares model, skeleton, animation, movement, collision, materials, roles, props, effects, and fallbacks. |
| Character Actor | Owns native component lifecycle, accepted transform, movement projection, collision, animation, and presentation. |
| Character movement service | Owns project movement rules and configures the native movement component. |
| Vehicle service | Owns vehicles, seats, enter and exit eligibility, vehicle movement, and vehicle state. |
| AI and NPC services | Own decision-making, path intent, reactions, and conversation intent for non-human characters. |
| Camera service | Owns camera rigs, accepted targets, blends, split-screen views, and first-person offsets. |
| Asset service | Owns model, animation, material, prop, and effect readiness and retained handles. |
| Footprint service | Owns bounded footprint presentation leases derived from accepted contact observations. |
| Domain services | Own damage, missions, interactions, rewards, progression, notoriety, and persistence. |
| Developer diagnostics | Observe immutable avatar, character, controller, movement, collision, loading, render, and footprint state. |

<!-- markdownlint-enable MD013 -->

Character animation, render visibility, collision callbacks, camera completion,
footprints, or sound cannot commit gameplay results.

## Runtime identities

Stable identities include:

- `FSharAvatarId`;
- `FSharAvatarRevision`;
- `FSharLocalPlayerId`;
- `FSharControllerAssignmentRevision`;
- `FSharCharacterDefinitionId`;
- `FSharCharacterDefinitionRevision`;
- `FSharCharacterInstanceId`;
- `FSharCharacterInstanceRevision`;
- `FSharCharacterRoleId`;
- `FSharCharacterMovementProfileId`;
- `FSharCharacterMovementRevision`;
- `FSharCharacterControllerId`;
- `FSharCharacterControllerRevision`;
- `FSharInputContextId`;
- `FSharInputLeaseId`;
- `FSharVehicleId`;
- `FSharVehicleSeatId`;
- `FSharVehicleHandoffId`;
- `FSharCameraTargetId`;
- `FSharCameraLeaseId`;
- `FSharCharacterAssetScopeId`;
- `FSharCharacterPropLeaseId`;
- `FSharFootprintLeaseId`;
- `FSharWorldRevision`;
- `FSharFeatureRevision`; and
- `FSharPersistenceRevision`.

Source array indexes, controller integers, model-name buffers, animation-name
buffers, object pointers, manager slots, action-handler positions, raw locator
names, drawable element indexes, and footprint-pool positions are provenance
only.

## Avatar record

`FSharAvatarState` contains:

- avatar and revision identities;
- local-player and controller assignment;
- accepted controlled character identity;
- optional accepted vehicle and seat identity;
- control mode;
- active input-context revision;
- movement and camera-target revisions;
- enter or exit handoff state;
- race or mission participation references;
- world and feature revisions;
- suspension and reconnect state; and
- diagnostics.

The closed control modes are:

- `inactive`;
- `on_foot`;
- `entering_vehicle`;
- `in_vehicle`;
- `exiting_vehicle`;
- `cinematic_controlled`;
- `temporarily_disabled`;
- `reconnecting`;
- `respawning`; and
- `tearing_down`.

## Avatar service

The avatar service is scoped to a game instance and local player. It does not
expose one process-global avatar array or assume player zero.

It accepts typed commands to:

- activate or deactivate an avatar;
- assign or replace a character;
- begin vehicle entry;
- commit vehicle seating;
- begin vehicle exit;
- commit on-foot placement;
- change camera-target intent;
- apply a race or mission participation reference;
- suspend or resume input; and
- tear down the avatar.

Every command checks expected avatar, character, vehicle, seat, controller,
world,
and feature revisions.

## Local-player and controller assignment

Each playable avatar belongs to one `ULocalPlayer`. Enhanced Input mapping
contexts are installed through the owning local-player subsystem and removed by
lease.

A controller assignment contains:

- local-player identity;
- input-device identity where available;
- assignment revision;
- active mapping contexts and priorities;
- accessibility and remapping profile;
- focus and suspension state; and
- reconnect policy.

Input is never routed by first connected device, raw platform button name, fixed
controller index, or pointer equality.

## Input actions

Canonical input actions include semantic intents such as:

- move;
- look or camera orbit;
- jump;
- sprint or turbo on foot;
- attack or kick;
- interact or action;
- enter or exit vehicle;
- camera toggle;
- pause; and
- accessibility alternatives.

Input modifiers own dead zones, axis inversion, sensitivity, response curves,
and
camera-relative transformation. Input triggers own press, release, hold, repeat,
chord, tap, and gating behavior.

Gameplay receives typed action observations with local-player, avatar,
controller, context, frame, and world revisions. Raw button callback order
cannot
select state.

## Input contexts

Mapping contexts are mode-scoped:

- frontend;
- on foot;
- entering or exiting vehicle;
- in vehicle;
- cinematic;
- supersprint;
- pause;
- accessibility overlay; and
- development diagnostics.

A transition installs the target context and removes incompatible contexts as
one
transaction. Stale input from a removed context is ignored.

## Character definition

`USharCharacterDefinition` declares:

- canonical identity and revision;
- character roles and tags;
- native Actor or Character class;
- skeletal mesh, skeleton, Physics Asset, and animation assets;
- movement profile;
- collision and physical-material profile;
- capsule and mesh offsets;
- render, level-of-detail, shadow, fade, and material policy;
- swatch or outfit variants;
- eye and facial presentation policy;
- enter and exit vehicle offsets and timing;
- interaction and action capabilities;
- prop attachment definitions;
- audio, dialogue, and VFX bindings;
- ground-support and surface policy;
- artificial-intelligence eligibility;
- streaming and persistence behavior;
- accessibility and quality policy; and
- deterministic fallback.

Source model and animation name conventions are import provenance. Runtime does
not construct asset paths by prefix, suffix, or character-name buffer.

## Character asset scope

Character construction requests one semantic asset scope containing required and
optional bundles for:

- mesh and skeleton;
- animation graph and sequences;
- materials and swatches;
- physics and collision;
- props;
- facial and eye presentation;
- audio and dialogue;
- footprints and effects; and
- feature overlays.

The scope returns retained handles and one typed readiness result. Deferred
loads,
model slots, animation slots, garbage states, callback order, and distance-based
pointer reuse are not public lifecycle.

## Construction transaction

Character construction:

1. validates the definition and world revision;
1. acquires required asset bundles;
1. spawns or obtains the native Actor;
1. creates and configures required components;
1. applies skeleton, animation, materials, collision, and movement profile;
1. establishes controller or AI ownership;
1. applies initial transform and ground support;
1. registers camera, interaction, audio, and VFX adapters;
1. publishes one ready revision; and
1. releases temporary preparation state.

A required failure compensates every component, controller, asset handle, and
registration. The runtime never leaves a half-configured character in the world.

## Character manager boundary

Repository coordination may index active characters by stable identity and
support semantic queries such as:

- character for a local player;
- character for a mission role;
- character by placement identity;
- characters within a world scope;
- current playable character;
- current named non-player character; and
- character owning a prop or interaction lease.

It does not expose a fixed character array, maximum slot index, raw model cache,
manual garbage state, or pointer-owned deferred-load record as authority.

## Replacement and skin changes

A character replacement or outfit change is a transaction:

- validate the requested definition or variant;
- acquire replacement assets;
- preserve allowed avatar, mission, transform, movement, and interaction state;
- construct and verify the replacement projection;
- atomically transfer control and camera target;
- cancel old props, footprints, audio, VFX, and callbacks;
- release old assets and Actor; and
- publish one replacement revision.

A skin change cannot silently change skeleton compatibility, collision,
movement,
character identity, mission role, or save semantics.

## On-foot movement

Project movement rules configure a native movement component. The movement
profile may define:

- walking, running, sprinting, falling, and custom movement modes;
- maximum speed and acceleration;
- braking and deceleration;
- rotation and facing policy;
- air control and gravity;
- jump, double-jump, stomp, and dash behavior;
- slope, step, ledge, floor, and perch behavior;
- ground friction;
- collision response;
- root-motion policy;
- network prediction policy; and
- accessibility assists.

Movement uses simulation time and native prerequisites. Render cadence and
animation playback do not integrate position independently.

## Camera-relative control

Camera-relative movement converts the accepted two-dimensional input vector
through the owning local player's current camera basis, projected according to
movement policy.

The calculation checks camera, avatar, input, and world revisions. A stale
camera
pointer or camera from another local player cannot steer the character.

## Facing and desired movement

Facing, desired direction, desired speed, actual velocity, movement mode, and
control rotation are distinct typed observations.

Animation consumes movement observations. It cannot overwrite authoritative
velocity or teleport the character unless an accepted root-motion or motion-
warping transaction owns the movement.

## Jump, sprint, attack, and kick

Jump, sprint, attack, kick, stomp, dash, and other actions are typed ability or
movement requests with:

- avatar and character revisions;
- input cause;
- state eligibility;
- cooldown and resource policy;
- movement and collision parameters;
- animation and presentation bindings;
- target and interaction policy; and
- terminal result.

A kick or shockwave publishes a domain request against validated targets. VFX,
animation, camera shake, or collision presentation cannot apply damage, impulse,
coins, notoriety, or mission state independently.

## Collision and physical state

Character collision follows native capsule, mesh, Physics Asset, movement, and
scene-query facilities. The character runtime owns accepted collision profile
and
movement response, while domain services own damage and persistent results.

Collision observations contain stable participants, shapes, physical materials,
contact point, normal, relative velocity, impulse, movement mode, world
revision,
and simulation ordinal.

Raw collision-volume pointers, fixed collision arrays, manual static submission,
or per-render collision updates are prohibited.

## Ground support

Ground support uses native floor queries and movement-component state. Project
policy may add validated support observations for special geometry, moving
platforms, vehicles, or imported world surfaces.

A support observation contains:

- character identity;
- support component and body identities;
- contact point and normal;
- surface and physical-material identities;
- support velocity;
- movement and world revisions; and
- query evidence.

A per-character manually allocated ground plane is not the primary movement
architecture. Compatibility support planes, when required, use the bounded pool
and lifecycle in
<!-- markdownlint-disable-next-line MD013 -->
[World render-entity and physics runtime](world-render-entity-and-physics-runtime.md).

## Ground snapping and relocation

Spawn, teleport, reset, vehicle exit, streaming activation, and recovery may
request a bounded ground-resolution transaction. It validates clearance, legal
surface, slope, collision, route, mission, and world revision before committing
a
transform.

Failure returns a typed result or declared fallback. It never repeatedly edits a
raw transform until an intersection appears.

## Transform and world-scene lifecycle

Accepted transform state flows through native Actor movement and component
updates. Scene registration, bounds, collision, animation, shadows, and camera
observation use declared tick prerequisites.

Repository code does not maintain a parallel parent transform, manually submit
static, animated, and dynamic geometry each frame, or call a custom display
method.

## Vehicle entry transaction

Vehicle entry requires:

- current on-foot avatar and character revisions;
- target vehicle and seat identities;
- ownership and mission eligibility;
- distance, orientation, obstruction, and vehicle-state validation;
- character and vehicle animation readiness;
- input and camera handoff plan;
- collision and movement transition plan;
- cancellation token; and
- fallback.

The transaction states are:

- `requested`;
- `approaching`;
- `aligning`;
- `animating`;
- `seating`;
- `committed`; or
- `rejected`, `cancelled`, `superseded`, or `failed`.

The avatar becomes in-vehicle only when seat ownership, vehicle control, input,
camera target, character attachment or hiding, collision, and movement state are
committed together.

Vehicle construction, seat and hardpoint definitions, controller leases, native
movement state, collision, damage, destruction, reset, parked and traffic mode,
and vehicle-side teardown follow
<!-- markdownlint-disable-next-line MD013 -->
[Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md).
Neither runtime may commit only its half of the handoff.

## Vehicle exit transaction

Vehicle exit validates:

- current seat and vehicle revisions;
- requested side or exit definition;
- legal world position and clearance;
- ground support;
- traffic and hazard policy;
- camera and input handoff;
- character readiness; and
- cancellation and fallback.

The avatar returns on foot only after the character has a valid transform,
movement state, collision profile, input context, and camera target. A door
animation or camera blend cannot commit exit alone.

## Vehicle and character lookups

Queries such as avatar for character, avatar for vehicle, or whether an avatar
is
entering or exiting return stable identities and revisions. Pointer equality is
not durable authority.

One vehicle or character cannot belong to two conflicting avatar handoff
transactions. Contention returns a closed rejection.

## Artificial-intelligence controller

AI-controlled characters use typed controller definitions. Simple ambient actors
may use StateTree, navigation, or a bounded wander policy. Named and mission
characters use explicit goals, route references, and interaction state.

Random choices use stable declared seeds. AI update order, global random state,
or pointer order cannot alter persistent or mission results.

## Non-player-character states

Closed NPC movement and interaction states may include:

- following path;
- stopped;
- dodging;
- cringing;
- talking;
- standing;
- panicking;
- talking with player;
- temporary goal;
- returning to path; and
- inactive.

State transitions are typed and deterministic. Source enum positions and fixed
waypoint arrays are provenance only.

## Path following and recovery

NPC path behavior follows
[Pedestrian path runtime](pedestrian-path-runtime.md).
The controller consumes stable path, waypoint, route, and world revisions.

Off-path recovery, teleport, dodge, panic, and temporary-goal behavior use
bounded
policies. An NPC cannot teleport to an arbitrary stale path pointer or silently
rewrite mission placement.

## Awareness, dodge, panic, and conversation

Awareness observations contain stable character, vehicle, event, distance,
direction, line-of-sight, world, and time evidence. They may propose dodge,
cringe, panic, look-at, or dialogue intent.

Dialogue and animation are presentation or interaction projections. They cannot
change mission or notoriety state without an accepted domain transaction.

## Character roles

Character roles are stable tags or closed identities such as playable, driver,
reward, active bonus, completed bonus, pedestrian, mission, ambient, cinematic,
or feature-defined namespaced roles.

Role controls eligibility and queries. It does not rely on source enum order,
model name, current controller class, or array placement.

## Interaction handlers

Interactions register through scoped leases that declare:

- interaction identity;
- owning service;
- character and world scope;
- eligibility and priority;
- prompt and input action;
- target and distance policy;
- lifetime and cancellation; and
- terminal result schema.

There is no fixed action-handler array. Priority and conflict resolution are
explicit. Removing a lease during delivery cannot invalidate iteration.

## Attached props

Props use native child components, sockets, or spawned Actors with
`FSharCharacterPropLease` ownership. A prop definition declares asset, socket,
relative transform, visibility, collision, animation, audio, VFX, interaction,
replication, and teardown.

A character replacement, vehicle handoff, world unload, feature removal, or prop
request cancellation releases the prop. Raw prop pointers and one fixed prop
slot
are not durable ownership.

## Rendering

The character Actor uses native mesh and component rendering. The render policy
may define:

- skeletal mesh and material variants;
- level-of-detail thresholds;
- animation budget and significance;
- visibility and fade;
- shadows;
- shock or damage material effects;
- outline and cel-shading parameters;
- local-player visibility policy; and
- fallback representation.

A custom CharacterRenderable or direct display call is not the shipping
renderer.

## Materials, swatches, and variants

Character swatches and outfits resolve stable material or texture variant
identities. Import validates semantic regions and compatibility.

Runtime variant selection cannot replace canonical character identity, skeleton,
collision, role, or progression. Missing variants use an explicit fallback and
do not construct raw texture names.

## Shadows and visibility

Native shadow and visibility settings follow camera, quality, accessibility,
streaming, and gameplay policy. A simplified shadow may be used when validated.

Culling or hidden presentation cannot deactivate gameplay, collision, mission,
interaction, or network authority. Visibility observations are not existence.

## Camera target adapter

A character camera target publishes immutable position, heading, up vector,
velocity, first-person position, terrain, stability, movement, and identity
observations to
<!-- markdownlint-disable-next-line MD013 -->
[Camera rig, preset, and arbitration runtime](camera-rig-preset-and-arbitration-runtime.md).

The camera adapter cannot mutate movement, infer vehicle ownership, or retain
raw
character pointers across replacement.

## Foot contact observations

Foot contact is derived from accepted animation markers and verified world
support evidence. It contains:

- character and foot identity;
- contact transform and normal;
- surface and physical-material identity;
- movement mode and speed;
- local-player and camera relevance;
- world, animation, movement, and character revisions; and
- deterministic cause identity.

Animation markers without valid support do not create authoritative footsteps or
footprints. Physics contact without an accepted gait marker may create impact
feedback but not a walking footprint unless policy declares it.

## Footprint definition

`USharFootprintDefinition` declares:

- stable identity and revision;
- eligible character, footwear, surface, and movement tags;
- material, decal, Niagara, or instanced representation;
- size, orientation, offset, and projection policy;
- lifetime and fade;
- maximum slope;
- wet, dirt, snow, or other surface parameters;
- local-player and camera policy;
- pooling and scalability;
- accessibility behavior; and
- fallback.

A source texture name and one global footprint texture are provenance only.

## Footprint lease and lifecycle

A footprint request creates a bounded `FSharFootprintLease`:

1. validate the foot-contact observation;
1. resolve the definition and surface policy;
1. acquire or create native presentation;
1. place and orient it against the accepted surface;
1. publish active presentation evidence;
1. fade or expire according to simulation or presentation time policy; and
1. release or return native resources.

Pool capacity is a quality budget, not identity. Overflow drops lower-priority
presentation and records a diagnostic; it never overwrites an active footprint
or changes movement.

## Character update ordering

The declared order is:

1. accept input or AI intent;
1. validate abilities and interactions;
1. update movement through the native movement component;
1. resolve collision and ground support;
1. publish immutable movement state;
1. update animation and pose;
1. accept correlated animation markers;
1. publish audio, VFX, footprint, camera, and UI observations;
1. update bounds and rendering through native prerequisites; and
1. process terminal handoff or lifecycle results.

Repository code does not separately move the renderable, collision volume,
physics object, camera target, and shadow from uncorrelated per-frame loops.

## Streaming and garbage collection

Character asset and Actor lifetimes are explicit. Streaming distance may make an
ambient character eligible for representation change or removal, but playable,
mission, interaction, camera, vehicle-handoff, dialogue, or network ownership
protects required state.

Unreal garbage collection manages UObject memory. Project code manages semantic
leases and retained handles. It does not implement model and animation garbage
states or reuse raw load slots.

## Local multiplayer

Each local player owns independent avatar, input, camera, HUD, and
audio-listener
context. Shared world characters and vehicles use explicit contention and seat
ownership.

Input, camera target, character replacement, footprints, and visibility cannot
leak between local players. Quality policy may share presentation resources only
when identity and isolation remain correct.

## Networking and mod boundary

The base game may run local-only, while multiplayer mods use a separate
authority
adapter. Networked character movement uses native prediction, correction, or an
accepted replacement with explicit server authority.

Mods may add namespaced character definitions, movement profiles, input
contexts,
props, variants, AI definitions, footprint definitions, and presentation assets.
They cannot replace local-player identity, native input globally, base movement
authority, or unrelated character definitions.

## Persistence and respawn

Portable persistence stores canonical character selection, allowed variant,
mission or placement role, and other declared durable domain state. It does not
serialize Actor pointers, movement-component internals, animation time,
collision
arrays, controller pointers, prop pointers, or footprint pools.

Respawn creates a new character instance and revision, validates the target
world
placement, reassigns the avatar, input, and camera, and rejects stale callbacks
from the old instance.

## Accessibility and quality

Accessibility may provide remapping, movement assists, hold alternatives, camera
comfort, stronger prompts, captions, reduced motion, and additional contact
cues.

Quality may change level of detail, shadows, secondary materials, distant
animation evaluation, footprint density, lifetime, VFX, and optional audio. It
cannot alter movement physics, input meaning, vehicle handoff, collision,
interaction eligibility, local-player isolation, mission state, or required
feedback.

## Diagnostics

Read-only diagnostics expose:

- avatar, local-player, controller, character, vehicle, and seat identities;
- control mode and handoff state;
- input contexts and actions;
- movement mode, velocity, acceleration, facing, floor, and support;
- collision profile and contacts;
- asset scopes and readiness;
- controller or AI state;
- camera-target observations;
- animation, material, level-of-detail, visibility, and shadow state;
- attached prop and interaction leases;
- footprint requests, capacity, and lifecycle;
- streaming, persistence, and feature ownership; and
- stale callback, replacement, and teardown findings.

Diagnostics cannot possess a character, inject shipping input, teleport, change
movement, enter a vehicle, attach a prop, create rewards, or mutate footprints
in
shipping runtime.

## Failure behavior

Closed failures include:

- `avatar_missing`;
- `controller_conflict`;
- `input_context_invalid`;
- `character_definition_missing`;
- `character_asset_not_ready`;
- `construction_failed`;
- `movement_profile_invalid`;
- `collision_invalid`;
- `ground_support_missing`;
- `spawn_invalid`;
- `vehicle_invalid`;
- `seat_conflict`;
- `entry_blocked`;
- `exit_blocked`;
- `handoff_stale`;
- `camera_stale`;
- `ai_definition_invalid`;
- `path_stale`;
- `prop_invalid`;
- `foot_contact_stale`;
- `footprint_capacity`;
- `world_stale`;
- `feature_stale`;
- `cancelled`;
- `superseded`; and
- `internal_failure`.

Required playable-character failures use an explicit safe recovery or prevent
activation. Optional ambient presentation may degrade only through a declared
fallback. Missing data never silently selects array zero or retains a stale
pointer.

## Validation

Validation proves:

- avatar, local-player, controller, character, vehicle, seat, movement, input,
  camera, prop, interaction, and footprint identities are stable;
- every character definition resolves compatible native assets and components;
- movement profiles contain finite bounded values;
- input actions and contexts are complete and non-conflicting;
- vehicle entry and exit have complete compensation paths;
- collision, ground support, and spawn policy are valid;
- AI definitions reference legal paths and states;
- render, material, variant, shadow, and level-of-detail fallbacks exist;
- prop sockets and footprint surfaces resolve;
- local-player isolation is complete;
- persistence migrations are defined;
- feature teardown releases every lease and callback; and
- no fixed manager array, raw name, pointer identity, callback order, or manual
  render submission remains runtime authority.

## Tests

Required tests cover:

- activation for each local player and controller assignment;
- input remapping, dead zones, inversion, accessibility, and context
  transitions;
- walking, sprinting, jumping, falling, collision, slopes, moving support, and
  ground recovery;
- camera-relative movement and stale-camera rejection;
- attack, kick, stomp, dash, shock, and presentation separation;
- vehicle entry, cancellation, contention, seating, exit, blocked exit, and
  fallback placement;
- asynchronous character loading in every callback order;
- character replacement and variant changes;
- AI wander, path following, dodge, cringe, panic, talk, and off-path recovery;
- interaction lease priority and removal;
- prop attach, replacement, vehicle handoff, and teardown;
- render level of detail, materials, fade, shadows, and visibility invariance;
- foot-contact validation, footprint placement, fade, pooling, overflow, and
  teardown;
- split-screen input and camera isolation;
- world unload, respawn, and feature removal during active operations;
- network-mod authority adapter behavior;
- deterministic movement and handoff replay within declared tolerances; and
- headless semantic execution without meshes, animation, cameras, or footprints.

## Invariants

- Avatar identity is never a player-array position.
- Controller assignment is always revisioned and local-player scoped.
- Native movement owns movement integration; rendering and animation do not.
- Vehicle entry and exit commit control, input, camera, collision, and placement
  atomically.
- Character asset readiness never depends on callback order or raw name
  construction.
- AI and human characters share stable movement and world identities.
- Render visibility never determines gameplay existence.
- Attached props and interactions are scoped leases, not fixed pointer arrays.
- Footprints are presentation only and cannot change movement or surface state.
- Replacement, respawn, world unload, and feature removal reject stale callbacks
  and leave no owned input, camera, asset, prop, interaction, audio, VFX, or
  footprint leases.

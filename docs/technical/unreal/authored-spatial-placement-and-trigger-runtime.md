# Authored spatial placement and trigger runtime

- Status: Active
- Last reviewed: 2026-07-15

<!-- markdownlint-disable MD013 -->

## Governing decisions and specifications

- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
- [Contextual interaction query and transaction boundary](../../adr/unreal/runtime/contextual-interaction-query-and-transaction.md)
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
- [Camera rig, preset, and arbitration runtime](camera-rig-preset-and-arbitration-runtime.md)

## Purpose

This specification defines native authored spatial placements, trigger shapes,
registration, occupancy, enter and exit observations, filtering, streaming,
diagnostics, and domain adapters.

It replaces one mutable locator inheritance hierarchy, one process-wide trigger
tracker, fixed-capacity player and artificial-intelligence arrays, raw camera and
character pointers, filename-owned interiors, ordinal event dispatch, and direct
streaming or gameplay mutation from overlap callbacks.

The spatial runtime detects and reports accepted spatial facts. It does not own
mission completion, camera mode, dialogue, audio, interiors, population,
streaming policy, progression, rewards, or vehicle behavior.

## Ownership

| Authority | Responsibility |
| :--- | :--- |
| Import pipeline | Convert source placement and volume evidence into canonical definitions. |
| Asset Manager | Load definition and presentation bundles by stable identity. |
| World spatial subsystem | Register active definitions and produce typed observations. |
| Unreal collision and query systems | Broad phase, overlap, sweep, and shape-query evidence. |
| Domain adapters | Interpret accepted observations for interactions, missions, cameras, audio, streaming, population, and presentation. |
| Domain services | Validate commands and own resulting state transitions. |
| World Partition and Data Layers | Composition availability and streaming lifecycle. |

A placement definition can be shared by several consumers through typed roles,
but it never invokes those consumers directly.

## Runtime topology

The spatial module owns:

- `FSharSpatialPlacementId`, a stable namespaced placement identity;
- `FSharSpatialVolumeId`, a stable shape identity;
- `FSharSpatialRoleId`, a semantic role identity;
- `FSharSpatialParticipantId`, a stable observed-entity identity;
- `USharSpatialPlacementDefinition`, immutable authored placement data;
- `FSharSpatialVolumeDefinition`, immutable shape and filter data;
- `FSharSpatialObservation`, immutable enter, stay, or exit evidence;
- `FSharSpatialOccupancyToken`, revision-bound participant occupancy;
- `FSharSpatialRegistrationHandle`, move-only active registration ownership;
- `USharWorldSpatialObservationSubsystem`, one authority per world; and
- typed adapters owned by interaction, mission, camera, audio, interior,
  population, streaming, traffic, and diagnostics modules.

Runtime actors or components are projections. Canonical identity remains in the
definition and registration revision, not an object pointer or actor name.

## Placement definition

Every placement definition contains:

| Field | Contract |
| :--- | :--- |
| `PlacementId` | Stable canonical identity. |
| `OwnerId` | Owning content, mission, chapter, structure, route, or feature identity. |
| `RoleIds` | One or more registered semantic roles. |
| `Transform` | Finite canonical world or owner-relative transform. |
| `VolumeIds` | Ordered trigger or query shapes. |
| `ActivationPredicate` | Chapter, mission, discovery, gameplay-state, layer, feature, and mod requirements. |
| `ParticipantFilterId` | Accepted pawn, vehicle, AI, local-player, or custom participant policy. |
| `ObservationPolicyId` | Enter, stay, exit, dwell, cooldown, hysteresis, and duplicate rules. |
| `DataLayerIds` | Exact Runtime Data Layer dependencies. |
| `BundleIds` | Required gameplay, presentation, audio, or diagnostic bundles. |
| `RevisionToken` | Definition and conversion revision. |
| `SourceAliases` | Optional conversion aliases with no runtime authority. |

Display names, source type numbers, event ordinals, array positions, filenames,
and package load order are not placement identity.

## Spatial roles

Initial semantic roles include:

- generic anchor;
- directional anchor;
- vehicle start or recovery transform;
- character, traffic, pedestrian, mission, race, or collectible spawn;
- contextual interaction placement;
- mission start, objective destination, checkpoint, or failure boundary;
- interior entry, exit, return, and transition anchor;
- static camera, rail camera, camera cut, FOV region, or camera observation;
- population group and ambient placement;
- occlusion, visibility, ambience, and presentation region;
- world-streaming or Data Layer observation region;
- route, spline, shortcut, zip-line, and traversal anchor;
- damage, death, radiation, bounce, or other typed hazard region; and
- diagnostic-only review placement.

A role selects a registered schema and adapter. It does not encode behavior in an
integer subtype or generic script string.

## Volume definitions

Supported authored volume shapes are:

- point or transform-only placement;
- sphere;
- oriented box;
- capsule when required by native behavior;
- convex volume from reviewed bounded evidence;
- spline corridor or distance field for route-sensitive observation; and
- compound ordered shapes referencing one placement.

A volume definition declares transform, dimensions, collision/query channel,
participant filters, boundary tolerance, hysteresis, dwell duration, cooldown,
priority, enabled policy, and diagnostic presentation.

Dimensions and transforms are finite. Negative radius, inverted extent,
degenerate basis, non-invertible transform, unbounded convex data, and NaN values
fail conversion or activation.

Sphere and oriented-box containment, intersection, line, sweep, and bounds use
Unreal geometry and collision facilities. Repository-owned code supplies semantic
policy and deterministic normalization, not a second general-purpose physics
library.

## Import and conversion

Source placement and trigger records convert before cooking into canonical data
assets or generated rows. Conversion:

1. resolves one canonical placement identity;
1. maps source type and event aliases to semantic roles;
1. normalizes transform and coordinate basis;
1. converts every supported shape and dimension;
1. resolves referenced interaction, mission, camera, interior, population,
   streaming, audio, and presentation identities;
1. validates activation and participant policies;
1. records provenance and revision evidence;
1. rejects unresolved or ambiguous behavior; and
1. emits deterministic definitions and manifests.

Runtime packages do not parse source locator chunks, event enums, raw strings,
or nested trigger records.

## Registration lifecycle

A definition becomes active only after its required world, Data Layers, feature,
chapter, mission, discovery, and bundles are ready. Registration returns a
move-only handle containing placement identity, world revision, definition
revision, active shape set, and owner.

Releasing the handle is idempotent. World teardown, cell unload, Data Layer
deactivation, feature removal, mission release, mod deactivation, and definition
replacement release affected registrations and occupancy tokens.

A late overlap callback for a released registration or earlier world revision is
ignored and recorded. It cannot activate a replacement placement.

Registration order does not select behavior. Equivalent active definitions are
ordered by canonical placement and volume identity.

## Participant identity and filters

A spatial participant observation contains:

- stable actor, vehicle, pawn, Mass entity, or custom participant identity;
- optional validated weak runtime handle;
- local-player and platform-user identity when applicable;
- participant kind and Gameplay Tags;
- transform, bounds, velocity, and movement revision;
- world and session revision; and
- controlling mission or feature identity when declared.

Filters may require or forbid participant kinds, roles, tags, local-player
ownership, vehicle state, character state, mission state, chapter availability,
or artificial-intelligence category.

Raw pointer type tests, fixed player indices, vehicle-array slots, and one global
locator-type bit mask are not filter authority.

## Observation model

A spatial observation contains:

| Field | Contract |
| :--- | :--- |
| `ObservationKind` | Enter, stay, exit, dwell-complete, enabled, disabled, or invalidated. |
| `PlacementId` | Canonical placement identity. |
| `VolumeId` | Exact contributing shape identity. |
| `RoleId` | Semantic role delivered to the adapter. |
| `ParticipantId` | Stable observed participant identity. |
| `OccupancyToken` | Unique revision-bound occupancy identity. |
| `WorldRevision` | Exact world and composition revision. |
| `RegistrationRevision` | Exact active placement registration. |
| `Sequence` | Monotonic sequence within the placement registration. |
| `ContactEvidence` | Optional point, normal, distance, approach, and velocity. |
| `Cause` | Physics overlap, sweep, query, streaming reconciliation, or explicit test. |

Observations are immutable facts. A spatial callback cannot grant a reward,
complete an objective, change camera mode, load an interior, or stream a zone by
itself.

## Enter, stay, and exit

One participant and volume pair owns at most one active occupancy token.
Duplicate begin-overlap callbacks do not create another enter. Duplicate end
callbacks do not create another exit.

A compound placement additionally owns one participant-level aggregate occupancy
projection. Unless a role explicitly requests per-shape observations, its enter
is emitted when the participant enters the first accepted shape and its exit is
emitted only after the participant leaves or invalidates the last accepted
shape. Crossing between overlapping shapes of the same placement emits neither a
second enter nor a transient exit. Shape identity remains available as evidence,
but array order and callback order cannot change the aggregate result.

An enter is accepted only after registration, participant, filter, and geometry
revisions match. A stay observation is emitted only when the role requests it and
at the declared bounded cadence. Exit occurs when the participant leaves,
becomes ineligible, unloads, is destroyed, changes world, or the placement
invalidates.

Streaming reconciliation may synthesize an explicit invalidated exit when a
participant or placement disappears while occupied. It cannot synthesize a domain
success.

Hysteresis and boundary tolerance prevent repeated enter and exit at one floating
point boundary. Dwell and cooldown use simulation or presentation time according
to role policy, never frame count.

## Local-player isolation

Each local player has independent participant identity and occupancy projection.
A volume may observe one, several, or all local players according to policy.

One player's enter cannot mark another player as present. Shared-world adapters
may aggregate accepted occupancies only through an explicit policy such as any
player, all players, first eligible player, or specific participant.

Split-screen count is product and platform policy, not a fixed compile-time
trigger-array dimension.

## Artificial-intelligence observation

Vehicles, pedestrians, named actors, Mass entities, and other artificial-
intelligence participants register through stable participant observations or
native query sources. The spatial subsystem does not retain raw vehicle pointers
in a fixed registry.

AI-specific roles declare exact participant and behavior tags. Route, traffic,
population, and mission systems decide what an accepted observation means.
Registering an AI participant for one role cannot expose every locator type.

## Contextual interaction placements

An interaction placement references one canonical interaction definition,
interaction slot transform, prompt role, participant filter, reservation policy,
and optional presentation anchor.

Object name, joint name, action name, input-button ordinal, and mutable character
handler pointers are conversion evidence only. Conversion resolves them to
canonical actor/component identities, semantic actions, typed interaction kinds,
and verified transforms.

The spatial adapter updates the interactor's bounded candidate set. The
interaction subsystem performs deterministic selection and owns the transaction.

## Interior portals

An interior placement references canonical structure, interior, portal, load
bundle, entry, return, chapter, discovery, gameplay-state, and transition-policy
identities.

A raw interior filename is invalid runtime authority. Enter observations request
an eligibility query; the interior subsystem validates and performs the atomic
transition. An overlap cannot leave both interior and exterior compositions
active.

Exit and return anchors are separate definitions. Mission-only interiors and
breakable-window routes follow their declared structure and mission predicates.

## Camera and field-of-view regions

Camera placements reference camera request, rig, preset, rail, static-shot, FOV,
transition, participant, priority, one-shot, cut, reset, and fallback policies.

The spatial adapter submits or releases a typed camera request. It does not hold a
mutable camera pointer, switch a global camera directly, or restore a previous
mode by raw pointer.

Car-only and on-foot-only behavior becomes participant filtering. One-shot,
dwell, cut-in, cut-out, and reset behavior is explicit request policy.

Unloading a camera placement releases its request. FOV duration and rate use
bounded physical units and the camera subsystem's arbitration contract.

## Vehicle and character starts

Vehicle, character, traffic, mission, recovery, and race starts contain canonical
spawn role, transform, orientation, participant or vehicle predicate, safety
profile, chapter, mission, world composition, and fallback identities.

A start definition is queried by the owning spawn or recovery service. Merely
streaming the placement does not spawn an entity.

Directional anchors expose a normalized transform or heading observation. They do
not mutate the target actor directly.

## Mission, race, and checkpoint placements

Mission start, objective, destination, checkpoint, route, failure, and recovery
placements reference canonical mission, stage, route, checkpoint, participant,
vehicle, approach, dwell, and gameplay-state policies.

The mission subsystem validates every observation against the active mission and
stage revision. An overlap from an inactive mission, old checkpoint, unloaded
route, wrong vehicle, or wrong participant cannot complete an objective.

Race checkpoints additionally declare order, lap, direction, crossing plane,
reset, shortcut, and missed-checkpoint behavior. The race runtime owns progress
and result state.

## Population and ambient regions

Population placements reference canonical population profile, zone, archetype
group, density, path, chapter, clock, weather, mission, and layer policies.

An accepted observation may activate or suppress a population projection through
the population subsystem. The spatial runtime does not spawn pedestrians or
select archetypes itself.

Ambient audio, music, occlusion, reverb, visibility, weather, and presentation
regions reference typed profile identities and priorities. Their adapters submit
or release requests to the owning systems. A single event ordinal cannot stand
for several unrelated presentation effects.

## Streaming and zone observations

Streaming placements reference canonical world region, World Partition cell,
Runtime Data Layer, load bundle, dependency, priority, prefetch, release,
interior-section, timeout, and cancellation policies.

An enter or proximity observation may request a streaming plan. The native load
coordinator and world-composition owner decide admission and readiness. The
spatial subsystem never loads or dumps source-named zones directly.

A streaming request is correlated by participant, placement, plan, world,
composition, and request revisions. Late readiness cannot activate a replacement
zone or dismiss a newer loading state.

Nearest-interior or recovery queries use deterministic distance, eligibility,
priority, and canonical identity ordering. They do not scan mutable active-volume
arrays or force a hidden load zone active.

## Occlusion and visibility

Occlusion placements define visibility set, participant or camera predicate,
transition policy, priority, and owning presentation scope.

The renderer, camera, audio, or presentation adapter consumes the observation.
The placement does not count active triggers in mutable local state to decide
permanent occlusion behavior.

Conflicting regions use declared priority and deterministic identity ordering.
World unload releases all requests and restores the owner's fallback state.

## Script and generic-event conversion

A source script or generic-event placement must convert to one registered typed
role with a known owner and bounded schema. Arbitrary script text, command names,
positional sound strings, or untyped payloads cannot execute at runtime.

When conversion resolves a sound, interaction, camera, mission, streaming, or
presentation action, it emits that exact typed definition. Evidence that cannot
resolve uniquely remains unavailable and produces a conversion finding.

## Action and presentation anchors

A presentation anchor may identify an object/component attachment, joint or bone
reference, local transform, semantic action, icon, decal, particle, sound, or UI
role.

Object, component, bone, and socket references resolve through canonical prepared
asset metadata. A missing optional presentation anchor may suppress presentation;
it cannot change interaction or mission authority.

Character animation, skeleton, eye, outfit, and prop preparation remain owned by
the FBX preparation contract. A placement cannot invent a runtime prop attachment
or change deformation behavior.

## Event routing

Accepted observations publish through registered typed channels after spatial
validation. Each role maps to one compatible payload schema, scope, delivery
phase, and owner.

Listener order, raw payload pointers, global event arrays, and numeric event
ranges cannot select behavior. Commands returned by consumers enter their normal
application ports and transactions.

A required domain transition is not considered complete merely because the
spatial event was delivered.

## World Partition lifecycle

Definitions are registered when their owning composition becomes ready and
released before that composition becomes unavailable to ordinary interaction.

Persistent placement identity survives actor recreation. Occupancy does not.
Reloading a cell reconstructs registration from canonical definitions and current
world state.

A participant straddling a streaming boundary receives at most one accepted
transition per registration revision. Streaming order cannot change placement
identity or replay one-time domain transactions.

## Performance and budgets

The spatial subsystem uses engine broad-phase and overlap notifications for
ordinary active volumes. Explicit bounded queries are used for participants that
cannot provide native overlap callbacks.

Budgets are data and platform policy, not fixed source array capacities. They
include active registrations, compound shapes, participant observations, queued
observations, diagnostic drawing, and per-role query cadence.

Exceeding a soft budget produces a measured finding and deterministic priority
policy. Exceeding a hard correctness limit rejects activation. It never writes
past an array, drops an exit silently, or changes mission behavior by container
order.

Large worlds partition registrations by World Partition composition and spatial
index. A process-wide scan of every placement each frame is forbidden.

## Presentation and diagnostics

Development diagnostics may display placement identity, roles, shape, transform,
participant filters, current occupancies, registration revision, observations,
consumer requests, streaming state, and rejection reasons.

Debug geometry uses the shared diagnostic runtime and explicit handles. It is
excluded from unauthorized shipping packages and cannot change collision,
activation, or observation results.

A development query may select placements by canonical identity, role, owner,
world region, or active participant. It cannot simulate an arbitrary ordinal
event or call a consumer with a null payload.

## Mods and game features

A validated package may add or override namespaced placement, volume, role,
filter, and observation-policy definitions when it declares:

- canonical owner and identities;
- target world and composition;
- required role schemas and capabilities;
- activation and participant policies;
- dependencies, conflicts, and priority;
- package and streaming bundles;
- save, achievement, and multiplayer compatibility where applicable;
- teardown behavior; and
- tests.

An overlay cannot replace a base role with an incompatible schema, broaden local
scope silently, create an arbitrary script role, or leave registrations active
after deactivation.

## Failure behavior

Activation or observation fails closed on:

- unknown, duplicate, or ambiguous identity;
- unsupported source type or event mapping;
- invalid transform or shape;
- missing owner, role, schema, filter, or activation policy;
- unresolved interior filename, script string, camera pointer, or raw object
  reference;
- stale world, registration, participant, mission, or composition revision;
- duplicate enter or exit without a matching occupancy token;
- participant from the wrong world, local player, mission, or role;
- missing required Data Layer or bundle;
- recursive or unauthorized event behavior;
- budget hard-limit violation;
- incomplete teardown; or
- a domain mutation attempted directly from the spatial callback.

Failure returns a typed finding and leaves domain state unchanged. Invalidating an
occupied placement releases its observation and consumer requests safely.

## Validation

Import and catalog validation prove:

- every source record maps uniquely or is explicitly unavailable;
- every placement and volume identity is unique;
- shapes, transforms, bounds, and units are finite;
- role schemas and owners resolve;
- referenced mission, interaction, camera, interior, population, streaming,
  audio, and presentation identities exist;
- activation and participant filters are bounded;
- streaming and game-feature teardown release every registration;
- no source filename, pointer, ordinal, or array index is runtime authority; and
- diagnostic-only definitions are excluded from unauthorized packages.

## Tests

Required tests include:

- deterministic import and identity lookup;
- sphere, oriented-box, capsule, convex, spline, and compound shape behavior;
- transform, line, sweep, containment, and bounds validation;
- enter, duplicate enter, stay, dwell, exit, duplicate exit, and invalidation;
- boundary hysteresis and cooldown;
- local-player isolation and shared aggregation policies;
- AI, vehicle, pedestrian, pawn, and Mass participant filters;
- world, cell, Data Layer, mission, feature, and mod registration lifecycle;
- contextual interaction candidate integration;
- interior portal admission and rollback;
- camera request activation and release;
- mission destination and race-checkpoint ordering;
- population and ambient profile requests;
- streaming request correlation and late-result rejection;
- occupancy teardown during participant and placement destruction;
- deterministic priority under soft budget pressure;
- hard-limit rejection without partial registration;
- mod overlay activation, replacement, and removal; and
- shipping exclusion of diagnostics.

## Invariants

- Spatial detection publishes facts; domain services own behavior.
- Placement identity is stable and independent of runtime actor lifetime.
- Source type and event ordinals are conversion provenance only.
- One participant-volume pair has at most one active occupancy token.
- Local players and worlds remain isolated.
- Registration and listener order never select behavior.
- A raw filename, script string, camera pointer, or AI pointer is never authority.
- Streaming cannot replay one-time progression or mission completion.
- Fixed source array capacities do not define product limits.
- Every registration and consumer request has explicit teardown.

<!-- markdownlint-enable MD013 -->

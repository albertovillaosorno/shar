# Native vehicle physics, control, damage, and presentation runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Driving, traffic, and vehicle behavior parity](../../adr/gameplay/vehicles/driving-traffic-and-vehicle-ai.md)
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
- [Road-network geometry and traffic runtime](road-network-geometry-and-traffic-runtime.md)
- [Vehicle AI and route runtime](vehicle-ai-and-route-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle retrieval and phone-booth runtime](vehicle-retrieval-and-phone-booth-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Character animation clip catalog and vehicle-handoff choreography runtime](character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Semantic input, device, and haptics runtime](semantic-input-device-and-haptics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle audio and avatar-sound runtime](vehicle-audio-and-avatar-sound-runtime.md)

## Purpose

This specification defines the native Unreal boundary for drivable vehicle
construction, Chaos physics, wheels, suspension, powertrain, steering, braking,
human and artificial-intelligence control, traffic locomotion, collisions,
damage, detachable presentation, occupants, resets, destroyed-vehicle husks,
parked vehicles, pursuit vehicles, rendering, lights, materials, effects,
streaming, networking, diagnostics, and teardown.

It replaces a source-era stack that combines one custom vehicle object, parallel
physics and traffic locomotion classes, hand-written tire and suspension forces,
manual wheel and pose drivers, fixed controller mappings, raw event listeners,
custom render drawables, damage texture manipulation, parked-car arrays, husk
pools, and pursuit-vehicle pools.

The target uses Unreal's native vehicle, physics, movement, input, animation,
rendering, asset, and lifecycle facilities. Project code owns stable semantic
definitions, accepted gameplay commands, artificial-intelligence intent, damage
rules, recovery policy, and presentation requests. It does not implement a
second tire, suspension, transmission, collision, or rigid-body solver.

## Native Unreal foundation

The default fixed-topology vehicle implementation uses native Unreal facilities
where applicable:

- `AWheeledVehiclePawn` or an equivalent project Pawn composition;
- `UChaosVehicleMovementComponent` and
  `UChaosWheeledVehicleMovementComponent`;
- `UChaosVehicleWheel` definitions;
- Chaos rigid-body simulation and asynchronous physics;
- Skeletal Mesh, Physics Asset, Animation Blueprint, and the native wheel
  controller animation path;
- Curve assets for engine torque and other validated tuning curves;
- Enhanced Input actions, mapping contexts, modifiers, and triggers;
- Physical Materials and physical-surface identities;
- native collision profiles, traces, overlap observations, and hit results;
- Asset Manager primary assets, bundles, soft references, and retained handles;
- Niagara, decals, materials, lights, and Audio Components for presentation;
- World Partition, level streaming, Game Features, and Actor lifecycle;
- native replication, prediction, correction, and physics resimulation where a
  network mod enables them; and
- Insights, visual logging, Chaos diagnostics, and project read-only overlays.

Chaos Modular Vehicles is not the default. It is an experimental system intended
for runtime construction and destruction of modular vehicle components. It may
be adopted only through a separate accepted decision for a validated feature
that requires that model and proves target support, deterministic behavior,
network policy, cooking, fallback, and migration.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Vehicle definition | Declares stable vehicle identity, compatible native class, physics assets, movement configuration, wheels, damage, presentation, seats, and fallbacks. |
| Vehicle simulation component | Owns native source forces, suspension, tires, steering, brakes, transmission, engine, wheel contacts, rigid-body state, and physics-thread integration. |
| Vehicle application service | Owns construction, mode, controller, occupant, damage, reset, destruction, replacement, parking, and teardown transactions. |
| Human input service | Owns per-local-player input actions, contexts, modifiers, triggers, device assignment, accessibility, and haptics requests. |
| Vehicle AI service | Owns driving intent, route intent, obstacle response, chase, evade, race, traffic, and recovery decisions. |
| Road and traffic services | Own lanes, legal movements, reservations, route projections, density, and traffic spawn policy. |
| Damage and impact services | Own accepted damage, breakage eligibility, gameplay consequences, notoriety observations, and reward transactions. |
| Render and presentation services | Own mesh, wheel animation, materials, lights, particles, skid presentation, shadows, and camera-facing presentation. |
| Audio service | Owns engine, skid, horn, collision, damage, door, and environmental playback from immutable observations. |
| Persistence and roster services | Own access class, acquired vehicle state, persistent health, repair, and durable placement state. |
| Developer diagnostics | Observe immutable definitions, commands, physics state, contacts, damage, controllers, presentation, capacity, and failures. |

<!-- markdownlint-enable MD013 -->

A collision callback, wheel contact, animation event, material change, sound,
particle, camera, or destroyed mesh cannot commit gameplay, persistence,
notoriety, currency, mission, or roster state.

## Runtime identities

Stable identities include:

- `FSharVehicleDefinitionId`;
- `FSharVehicleDefinitionRevision`;
- `FSharVehicleInstanceId`;
- `FSharVehicleInstanceRevision`;
- `FSharVehicleSimulationId`;
- `FSharVehicleSimulationRevision`;
- `FSharVehicleMovementProfileId`;
- `FSharVehicleWheelDefinitionId`;
- `FSharVehicleWheelInstanceId`;
- `FSharVehicleWheelRevision`;
- `FSharVehicleControllerLeaseId`;
- `FSharVehicleInputRevision`;
- `FSharVehicleAIRevision`;
- `FSharVehicleOccupantRevision`;
- `FSharVehicleDamageRevision`;
- `FSharVehiclePresentationRevision`;
- `FSharVehicleResetTransactionId`;
- `FSharVehicleDestructionTransactionId`;
- `FSharVehicleHuskLeaseId`;
- `FSharParkedVehiclePlacementId`;
- `FSharPursuitVehicleLeaseId`;
- `FSharRoadProjectionRevision`;
- `FSharWorldRevision`;
- `FSharFeatureRevision`; and
- `FSharPhysicsStepRevision`.

Source pointer values, active-list indexes, pool slots, wheel-array positions,
joint indexes, controller integers, enum ordinals, model-name buffers, drawable
pointers, and generated numeric names are provenance or native implementation
details only.

## Vehicle definition

`USharVehicleDefinition` contains:

- canonical identity, revision, aliases, access classes, and content tags;
- compatible Pawn, movement component, and simulation mode;
- Skeletal Mesh, Physics Asset, Animation Blueprint, and material set;
- native wheel definitions and bone or socket bindings;
- chassis mass, center of mass, inertia, drag, downforce, and aerofoils;
- engine torque curve, idle and maximum rotation speed, engine braking, and
  throttle response;
- transmission, forward and reverse gears, final drive, automatic or manual
  policy, shift thresholds, and shift duration;
- differential and driven-wheel policy;
- tire friction, lateral and longitudinal grip, slip, load sensitivity, and
  physical-surface modifiers;
- steering geometry, maximum angle, speed-sensitive reduction, rise and fall
  rates, and controller-device modifiers;
- service brake, handbrake, reverse, traction, and stability policies;
- suspension travel, spring rate, preload, damping, wheel load, trace shape, and
  contact filtering;
- top-speed, acceleration, recovery, and reset policies;
- turbo, jump-boost, horn, and special ability definitions;
- occupant seats, enter and exit anchors, camera targets, and collectible
  hardpoints;
- collision profiles, damage zones, damage stages, detachable-presentation
  bindings, health, and destruction policy;
- lights, materials, colors, shadows, wheel animation, roof-fade, damage,
  particle, skid, audio, and accessibility presentation;
- traffic, race, pursuit, mission, parked, secret, and retrieved variants;
- target quality, network, streaming, and teardown policy; and
- validation and fallback behavior.

Every physical quantity declares units. Unlabelled tuning constants, magic
thresholds, compile-time platform branches, and mutable singleton parameters are
not target architecture.

## Fixed and modular composition

The standard vehicle system uses one fixed Skeletal Mesh topology with a Physics
Asset, wheel definitions, and an Animation Blueprint. Damage may change
material,
visibility, animation, collision enablement, detachable cosmetic presentation,
or replace the vehicle with a declared terminal representation.

A requirement to add or remove simulated structural components at runtime is not
silently emulated through hidden bones or arbitrary component deletion. It
requires an explicit modular-vehicle definition and the separate accepted
experimental-system decision described above.

## Construction transaction

Vehicle construction follows this sequence:

1. resolve the canonical definition, placement, access, world, feature, quality,
   and target revisions;
1. validate the native class, movement component, Skeletal Mesh, Physics Asset,
   Animation Blueprint, wheels, materials, collision, and required bundles;
1. acquire retained asset handles;
1. spawn a candidate Actor under the owning world and streaming scope;
1. create and validate native physics state;
1. configure chassis, center of mass, engine, transmission, differential,
   steering, brakes, suspension, wheel, and surface policy;
1. bind seats, cameras, lights, presentation, audio, VFX, and diagnostic
   identity;
1. establish the requested controller and simulation mode without enabling
   gameplay input;
1. apply the accepted transform and verify no invalid penetration or unsupported
   floor state;
1. publish readiness and atomically commit the vehicle instance revision; and
1. release temporary construction handles while retaining the instance scope.

Failure destroys the candidate, releases every handle and delegate, and returns
a
closed result. A partially initialized native vehicle never enters an active
vehicle list, traffic lane, race grid, retrieval result, or player handoff.

## Vehicle lifecycle states

The closed application states are:

- `constructing`;
- `ready_uncontrolled`;
- `controlled_human`;
- `controlled_ai`;
- `traffic_projected`;
- `parked`;
- `entering_or_exiting`;
- `disabled`;
- `recovering`;
- `destroyed`;
- `husk_presentation`;
- `unloading`;
- `cancelled`; and
- `failed`.

Native physics activation, sleep, wake, or component visibility is not an
application state by itself. State transitions validate the expected vehicle,
controller, occupant, world, feature, and physics revisions.

## Simulation modes

A vehicle may use one declared simulation mode at a time:

- native dynamic Chaos vehicle simulation;
- road-projected traffic movement;
- parked or query-only presentation;
- cinematic or authored presentation; or
- disabled terminal presentation.

Road-projected traffic movement may use a lightweight project adapter to consume
canonical lane and intersection state, but it cannot become a second general
vehicle physics solver. A transition from traffic projection to dynamic Chaos
simulation validates the current transform, velocity, lane projection, wheel
support, collision environment, controller, and world revision and publishes one
correlated handoff result.

A transition back to traffic projection requires a supported traffic state,
accepted lane and route projection, no player occupancy, no incompatible mission
or damage lease, and a stable native snapshot. Directly swapping raw locomotion
pointers is prohibited.

## Native physics state

The native movement component owns:

- chassis rigid-body state;
- wheel contacts and suspension outputs;
- engine, transmission, differential, and drivetrain simulation;
- tire forces and slip;
- steering, throttle, brake, handbrake, and reverse input application;
- aerodynamics, drag, and configured external forces;
- asynchronous physics input and output;
- sleep, wake, and simulation-state creation and destruction; and
- physics snapshots supported by the engine.

Project code consumes immutable snapshots. It cannot directly integrate a second
rigid body, add one hand-written tire force per wheel, manually solve collision,
or mutate physics-thread objects from gameplay callbacks.

## Simulation snapshot

`FSharVehicleSimulationSnapshot` contains:

- vehicle, simulation, world, feature, and physics-step revisions;
- native transform, linear velocity, angular velocity, acceleration, and sleep;
- forward, up, and lateral basis;
- speed, signed forward speed, and movement direction;
- throttle, brake, handbrake, steering, reverse, turbo, and horn projections;
- engine rotation speed, normalized engine load, current gear, and shift state;
- wheel contact, suspension, slip, load, rotation, steering, and surface values;
- grounded, airborne, unstable, overturned, stuck, and recoverable observations;
- collision and damage summary;
- occupant and controller summary; and
- confidence and fallback state.

Snapshots are observations, not mutable shared structures. Presentation and AI
consume a published revision and cannot retain native solver pointers.

## Wheel definition and state

Each wheel definition declares:

- stable identity and axle role;
- wheel bone or socket;
- radius, width, mass, collision, and trace shape;
- driven, steering, service-brake, and handbrake participation;
- suspension axis, travel, spring, preload, damping, and force offset;
- tire friction and slip policy;
- maximum steering angle and speed-sensitive steering behavior;
- wheel animation and presentation binding;
- surface, skid, particle, audio, and haptics output policy; and
- validation and fallback.

`FSharVehicleWheelSnapshot` contains contact state, contact point and normal,
physical surface, suspension offset and velocity, normal load, longitudinal and
lateral slip, steering angle, rotation angle, angular velocity, brake state,
drive state, and expected revisions.

A wheel-array index is never wheel identity. A missing required wheel
definition,
bone, Physics Asset body, or movement binding fails construction. An optional
presentation wheel may use a declared fallback without changing simulation.

Wheel presentation may project closed semantic states such as normal contact,
slip, lightweight traffic slide, or presentation-only free spin. These states
are
derived from native wheel and vehicle snapshots. They cannot replace tire,
suspension, brake, drivetrain, road, or rigid-body simulation.

## Suspension and ground support

Suspension, wheel contact, spring force, damping, load transfer, and tire force
are native movement-component responsibilities. Project tuning configures those
facilities and validates resulting behavior against parity tests.

Ground support consumes native wheel contacts and the world physics contract. A
compatibility support plane may be used only through the bounded support-plane
lease in
<!-- markdownlint-disable-next-line MD013 -->
[World render-entity and physics runtime](world-render-entity-and-physics-runtime.md).
It cannot replace missing world collision or become the ordinary wheel-contact
system.

Support loss, airborne state, landing, rollover risk, and instability are typed
observations. They do not directly trigger mission failure, reset, audio, VFX,
or
camera state.

## Powertrain

Engine and drivetrain configuration is data-driven and uses native facilities:

- engine torque curve;
- engine idle and maximum rotation speed;
- engine braking and throttle response;
- forward and reverse gear ratios;
- final drive;
- differential and driven wheels;
- automatic or manual shifting;
- shift thresholds and duration; and
- damage or special-mode modifiers.

Gear and engine-speed observations are read from the accepted simulation state.
Audio cannot calculate or write authoritative gear state, and presentation
cannot
force a shift because one source animation or sound completed.

## Steering, throttle, brake, handbrake, and reverse

`FSharVehicleControlCommand` contains:

- command, controller lease, local-player or AI, vehicle, and input revisions;
- normalized throttle, brake, handbrake, steering, reverse, turbo, horn, and
  special-action values;
- steering-device kind and raw-device evidence when needed for diagnostics;
- command simulation timestamp;
- accessibility and assist policy;
- expected application mode, world, feature, and physics revisions; and
- cancellation or replacement identity.

The vehicle application service validates and forwards one accepted command to
the native movement component. Human and artificial-intelligence controllers
produce the same semantic command shape.

Controller dead zones, steering curves, speed-sensitive reduction, rise and fall
rates, wheel-device scaling, trigger interpretation, and accessibility assists
are
authored settings. Device-specific force feedback is requested through the
haptics runtime, not generated by direct platform spring, damper, or constant-
force APIs inside the vehicle controller.

## Human control

Human control requires:

- one valid local-player identity;
- one current device assignment;
- one vehicle-controller lease;
- one accepted possession or control mode;
- one enabled Enhanced Input context;
- a valid vehicle and movement revision; and
- no conflicting transition, pause, cinematic, arrest, or teardown lease.

Control acquisition atomically binds the Player Controller or accepted adapter,
input context, camera target, haptics scope, and vehicle controller lease.
Control
release removes those bindings before the vehicle or local player is destroyed.

Raw button IDs, platform button names, one global mappable object, controller
integer slots, and callback order are not semantic input authority.

## Artificial-intelligence control

Vehicle AI publishes intent, not physics forces. `FSharVehicleAIControlIntent`
contains:

- AI, vehicle, route, target, traffic, world, and feature revisions;
- controller mode;
- desired speed and acceleration policy;
- steering or target direction;
- brake, handbrake, reverse, turbo, and horn intent;
- lane, intersection, waypoint, chase, evade, race, or recovery context;
- obstacle and collision evidence;
- confidence, timeout, and fallback; and
- deterministic decision timestamp.

The controller adapter converts accepted intent into the same semantic control
command used by a human controller. AI code cannot write wheel forces,
suspension
offsets, native solver state, or render transforms.

Controller modes and route behavior follow
[Vehicle AI and route runtime](vehicle-ai-and-route-runtime.md).

## Traffic locomotion

Traffic vehicles consume immutable road, lane, legal-movement, reservation,
density, and route state from
<!-- markdownlint-disable-next-line MD013 -->
[Road-network geometry and traffic runtime](road-network-geometry-and-traffic-runtime.md).

A traffic projection may contain:

- current and next lane identities;
- current road segment and normalized progress;
- intersection reservation and legal movement;
- desired speed and stop state;
- lane-change curve and revision;
- recent path history for presentation and collision handoff;
- traffic-model and driver definition;
- active, paused, hidden, or removal policy; and
- deterministic update timestamp.

Traffic presentation may move through an accepted spline or native movement
adapter when full dynamic simulation is unnecessary. It must preserve collision,
player interaction, hijack, mission, visibility, and transition requirements.
When a traffic vehicle becomes player-controlled, damaged beyond lightweight
policy, or otherwise requires physical simulation, it transitions through the
validated simulation-mode handoff rather than silently changing locomotion.

## Road, intersection, and lane-change handoff

Entering an intersection requires a current legal movement and reservation. Lane
changes require compatible lane topology, sufficient distance, no conflicting
reservation, and an accepted curve revision. Traffic speed or transform cannot
be
advanced from an unvalidated previous-lane pointer or stale road segment.

Road projection and dynamic physics may disagree temporarily during handoff. The
transaction resolves one accepted transform, velocity, orientation, and support
state before enabling collision or control. Failure leaves the previous mode
active or returns a typed removal result.

## Collision observations

Vehicle collision observations follow
<!-- markdownlint-disable-next-line MD013 -->
[Physical material and impact-response runtime](physical-material-and-impact-response-runtime.md)
and contain:

- contact and causation identities;
- both entity and body revisions;
- contact points, normals, relative velocity, impulse, and severity;
- physical-surface and collision-class identities;
- vehicle zone or component evidence;
- occupant, player, traffic, pursuit, and mission context;
- world, feature, and physics-step revisions; and
- accepted or stale result.

The native Chaos solver owns contact resolution. Project collision adapters may
classify gameplay effects, damage, audio, VFX, haptics, notoriety, or recovery,
but cannot return a source-era custom solving answer or modify the native
contact
manifold from an uncorrelated gameplay callback.

## Damage model

`USharVehicleDamageDefinition` declares:

- health and damage-stage policy;
- collision severity bands;
- damage zones such as chassis, hood, trunk, driver side, passenger side,
  wheels, lights, or definition-owned regions;
- material and texture state;
- deform, flap, hide, detach, and terminal presentation bindings;
- wheel, engine, steering, braking, or movement impairment when gameplay
  requires
  it;
- smoke, sparks, debris, fire, audio, haptics, and camera observations;
- occupant, mission, traffic, pursuit, and roster consequences;
- repair, reset, persistence, and destruction behavior; and
- validation and fallback.

Accepted damage is one revisioned transaction. Native collision evidence
proposes
it; the damage service validates health, zone, immunity, duplicate suppression,
world, mission, and vehicle state and commits one new damage revision.

Presentation changes occur only after commit. A material toggle, hidden joint,
particle, flapping animation, detached cosmetic, or sound cannot create damage
or
destruction authority.

## Detachable and flapping presentation

A fixed-topology vehicle may declare detachable or flapping cosmetic parts such
as doors, hood, trunk, or definition-owned components. Each binding contains:

- stable presentation-part identity;
- compatible bone, body, socket, material, and damage zone;
- closed states such as intact, damaged, flapping, detached, and absent;
- native animation, constraint, visibility, collision, and VFX policy;
- force or damage threshold expressed with units;
- reset, repair, pooling, and teardown; and
- fallback when the presentation part is unavailable.

This presentation cannot mutate the Skeletal Mesh asset, remove arbitrary solver
bodies, or imply that Chaos Modular Vehicles is active.

## Destruction transaction

A vehicle destruction transaction:

1. validates vehicle, health, damage, collision, mission, world, feature, and
   persistence revisions;
1. commits terminal gameplay and persistence effects exactly once;
1. freezes new controller and damage commands;
1. releases human or AI control through a correlated handoff;
1. publishes occupant and camera recovery intent;
1. stops or transitions native movement safely;
1. publishes audio, VFX, debris, light, material, and HUD presentation requests;
1. creates or acquires a declared destroyed representation when required;
1. withdraws traffic, pursuit, race, parking, and route reservations; and
1. completes only after authoritative state and required cleanup verify.

Destruction audio, explosion VFX, hidden geometry, a removed Actor, or a husk
cannot commit the terminal result.

## Destroyed-vehicle husks

A husk is a terminal presentation lease associated with one destroyed vehicle.
It
is not a second vehicle identity and cannot be driven, retrieved, repaired,
counted as traffic, targeted as the original mission vehicle, or grant duplicate
rewards.

`FSharVehicleHuskLease` contains:

- lease, original vehicle, destruction transaction, definition, and presentation
  revisions;
- transform, velocity, material, collision, and lifetime policy;
- world, mission, feature, quality, and streaming revisions;
- optional native Actor or component weak identity;
- visibility and removal policy; and
- terminal result.

Pooling is optional. A pooled husk resets transform, velocity, physics,
collision,
materials, mesh, lights, audio, VFX, callbacks, original-vehicle mapping, world,
feature, and ownership before reuse. Capacity exhaustion returns an explicit
fallback or omission result; it cannot prevent the original destruction
transaction from completing.

A source-to-husk or husk-to-source lookup is valid only while the lease and both
expected revisions remain active. Pool slots are never mapping identity.

## Parked vehicles

A parked vehicle is an authored or policy-generated placement, not a process-
global free-car slot. `FSharParkedVehiclePlacement` declares:

- stable placement and vehicle-definition identities;
- world, level, zone, feature, and streaming ownership;
- transform, floor, collision, and clearance requirements;
- access class and enterability;
- driver, damage, persistence, respawn, and mission policy;
- required assets and retained handles;
- activation, visibility, and distance policy; and
- removal, replacement, and teardown behavior.

A zone may declare weighted eligible parked definitions and a bounded active
budget. Selection is deterministic for the placement or session seed. Runtime
model-name arrays, locator-array positions, fixed zone counts, and first-free
vehicle slots are provenance only.

Construction validates clearance from players, other vehicles, required routes,
and mission-critical placements. Removal cannot delete an occupied, reserved,
mission-owned, or persistent vehicle. A parked vehicle that becomes controlled
transitions through the ordinary controller and simulation transaction.

## Pursuit vehicles

Pursuit vehicle creation and withdrawal are requested by the notoriety service.
`FSharPursuitVehicleLease` contains:

- notoriety session and pursuit-wave identities;
- pursuer vehicle definition and controller profile;
- target avatar or vehicle identity;
- route, spawn, world, feature, and simulation revisions;
- active budget and replacement policy;
- visibility, distance, removal, and out-of-sight policy;
- arrest eligibility and capture-volume policy;
- cancellation, destruction, resolution, and teardown; and
- terminal result.

The pursuit service chooses spawn candidates through road, visibility, distance,
clearance, and active-budget validation. Hard-coded player zero, fixed arrays,
spawn-radius constants, first-free slots, raw target pointers, and wall-clock
removal timers are not target authority.

Destroyed or withdrawn pursuers release route, controller, vehicle, arrest, and
presentation leases. A destroyed pursuer cannot remain active merely because its
pool slot is marked active, and an out-of-sight timer cannot remove a current
mission or arrest participant without validating ownership.

## Notoriety integration

Qualifying vehicle contact, vehicle destruction, pedestrian impact, and other
accepted offenses publish typed observations to
<!-- markdownlint-disable-next-line MD013 -->
[Mission, interaction, interior, and notoriety runtime](mission-interaction-and-notoriety-runtime.md).
The notoriety service owns value, decay, warning, pursuit, resolution, arrest,
and fine transactions.

Vehicle simulation cannot directly adjust notoriety, decide coin rewards, exempt
arbitrary object names, or declare an arrest. Pursuit vehicles consume accepted
notoriety state and publish only movement, contact, capture-candidate,
destruction,
evasion, and withdrawal observations.

## Reset and recovery

`FSharVehicleResetRequest` contains:

- transaction, vehicle, controller, occupant, route, mission, world, and feature
  revisions;
- reason such as manual reset, out of bounds, invalid support, penetration,
  rollover, stuck, mission retry, race retry, or arrest recovery;
- candidate safe transform and source identity;
- velocity and damage preservation policy;
- road projection and lane policy;
- camera, input, haptics, audio, and presentation barriers; and
- timeout, fallback, and cancellation.

The transaction freezes new control, validates or finds a safe transform, checks
world readiness and collision clearance, moves or recreates native physics state
through supported facilities, restores permitted velocity and damage, updates
route and occupant state, and re-enables control only after verification.

Reset-on-spot heuristics, arbitrary ground offsets, direct transform writes, and
manual velocity zeroing are migration evidence only. A reset cannot repair
damage,
clear notoriety, duplicate rewards, or change mission state unless its accepted
policy explicitly includes the corresponding authoritative transaction.

## Overturn, stuck, and instability observations

Overturn, side-rest, prolonged no-progress, high angular velocity, invalid wheel
support, non-finite transform, and penetration produce typed recovery
candidates.
Thresholds are authored, unit-labelled, target-tested, and expressed in
simulation
time. They do not depend on frame count, camera visibility, or presentation.

A vehicle may apply native stabilization or assists when declared by the
movement
profile, but project code cannot inject undocumented impulses to reproduce one
source implementation. Required handling is verified behaviorally.

## Occupants and seats

Vehicle occupant handoff follows
<!-- markdownlint-disable-next-line MD013 -->
[Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md).
The vehicle exposes validated seat, entry, exit, passenger, driver, and camera
anchors by stable identity.

Entering and exiting are atomic transactions across avatar, character, vehicle,
controller, input, camera, collision, animation, and seat revisions. Vehicle
physics cannot infer an occupant from a character pointer or seat transform, and
occupant animation cannot commit possession.

Vehicle height class, seat role, side, door capability, hardpoint transforms,
occupancy, collision, motion, and world readiness are immutable inputs to
<!-- markdownlint-disable-next-line MD013 -->
[Character animation clip catalog and vehicle-handoff choreography runtime](character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md).
The vehicle accepts only revision-correlated phase observations and application
commands; a door, seat, or attachment marker cannot mutate native vehicle state
directly.

Occupant displacement from collisions is derived from accepted vehicle movement
and seat policy. It cannot directly edit a character pose or root transform from
the physics solver.

## Attached collectibles and payloads

A vehicle may expose stable hardpoints for mission payloads or collectibles.
Each
attachment lease declares payload, vehicle, hardpoint, mission, world, feature,
and physics revisions; attachment transform; collision policy; release velocity;
and teardown.

A raw drawable pointer, one hard-coded joint, force threshold, or direct detach
callback cannot commit mission delivery, destruction, or collection.

## Rendering and wheel animation

Vehicle rendering uses native components and animation. The presentation
projection may include:

- body and wheel mesh pose;
- wheel rotation, steering, and suspension offsets;
- animation-controller parameters;
- material and color variants;
- damage materials and part visibility;
- brake, reverse, headlight, emergency, ghost, or definition-owned lights;
- roof-fade and interior visibility;
- shadows and quality policy;
- attached payload presentation;
- smoke, sparks, backfire, dust, and other VFX; and
- diagnostic overlays.

Wheel pose is derived from the accepted native wheel snapshot and an Animation
Blueprint or approved component adapter. Manual root-matrix and suspension-joint
pose drivers are import evidence only.

Presentation cannot call a second renderer, directly mutate shared source
materials, select identity from shader names, or write vehicle simulation state.

## Lights and materials

Lights and material states are definition-owned semantic roles. A definition may
declare headlights, brake lights, reverse lights, indicators, emergency lights,
glow, damage, color, roof, window, and accessibility variants.

Native materials and light components consume the accepted vehicle presentation
revision. Joint-name searches, shader-name conventions, raw texture replacement,
and billboard-array positions are import provenance that must be converted into
validated bindings.

## Damage and surface presentation

Damage smoke, wheel smoke, sparks, skid effects, dust, backfire, and collision
presentation are typed requests to the VFX and audio services. Requests contain
vehicle, wheel or zone, damage, contact, physical-surface, world, local-player,
quality, and feature revisions.

Skid-mark geometry and long-lived surface presentation follow the dedicated
skid-mark contract when defined. Until then, this runtime owns only the
immutable
skid and surface observation, not a custom skid-mark renderer.

## Vehicle audio

Vehicle audio consumes immutable simulation and presentation observations
through
<!-- markdownlint-disable-next-line MD013 -->
[Vehicle audio and avatar-sound runtime](vehicle-audio-and-avatar-sound-runtime.md).
Observations include engine load, engine rotation speed, gear, throttle, speed,
reverse, airborne, skid, burnout, powerslide, horn, damage, door, surface, and
proximity state.

Audio cannot modify input, engine, gear, wheel slip, damage, movement,
controller,
or traffic state.

## Camera and HUD

The vehicle publishes immutable camera-target observations containing transform,
velocity, heading, up vector, stability, reverse, first-person anchors,
occupant,
controller, world, and simulation revisions. Camera services own rigs and
blends.

HUD and radar consume stable vehicle, route, damage, speed, mission, race,
notoriety, and target observations. A rendered gauge, icon, camera state, or
minimap pointer cannot commit vehicle behavior.

## Streaming and world lifecycle

Vehicle scopes may be owned by world, level, region, mission, race, traffic,
pursuit, retrieval, parking, feature, or local-player transactions. Each scope
retains required assets and native objects until every dependent lease is
released.

Streaming unload:

1. freezes new controller, damage, route, and presentation requests;
1. resolves or cancels occupant and mission ownership;
1. withdraws traffic, pursuit, parking, and road reservations;
1. disables input, camera, audio, VFX, and diagnostic leases;
1. destroys native physics state through supported component lifecycle;
1. clears delegates and rejects late callbacks;
1. unregisters the Actor and components;
1. releases retained assets; and
1. proves zero owned vehicle resources.

Actor destruction, visibility loss, distance culling, or Data Layer deactivation
cannot be interpreted as vehicle destruction, roster removal, mission
completion,
or repair.

## Pause, focus, and suspension

Pause policy distinguishes gameplay simulation pause, presentation pause,
frontend or modal pause, platform focus loss, and application suspension.

Native vehicle simulation, input, AI, audio, VFX, and haptics follow the
accepted
application-mode policy. Resume validates device, controller, vehicle, world,
feature, and physics revisions before applying new commands. Wall-clock elapsed
time cannot produce a physics, pursuit, removal, or damage jump after
suspension.

## Networking and multiplayer mods

The base game does not provide a campaign multiplayer mode. A validated mod may
add server-authoritative or peer policy through the multiplayer adapter.

Network vehicle support declares:

- authority and ownership;
- input command and simulation timestamp policy;
- native replication, prediction, correction, and resimulation configuration;
- vehicle, wheel, damage, occupant, route, and controller snapshots;
- spawn, reset, destruction, parking, pursuit, and handoff transactions;
- relevance, dormancy, streaming, and bandwidth budgets;
- anti-cheat and validation boundaries;
- disconnect, migration, timeout, and recovery; and
- target support and fallback.

A mod cannot publish raw solver pointers, trust client damage or position
without
validation, use render transforms as simulation authority, or silently replace
the
base vehicle definition.

## Game Feature and mod overlays

A validated Game Feature may add namespaced:

- vehicle definitions and variants;
- movement, wheel, engine, transmission, differential, suspension, steering,
  brake, tire, damage, and recovery profiles;
- parked, traffic, pursuit, mission, race, and retrieval policies;
- input mappings and haptics profiles;
- materials, lights, audio, VFX, camera, HUD, and diagnostic presentation;
- modular-vehicle definitions only when the separate accepted decision permits
  them; and
- target quality and network variants.

An overlay cannot replace Chaos, the native physics scene, protected base
vehicles, local-player identity, collision authority, persistent roster state,
notoriety, currency, mission results, or another feature's active leases.

Feature removal cancels owned construction, control, AI, route, parking,
pursuit,
husk, damage, presentation, and network requests; releases native physics and
asset resources; restores scoped base state; rejects stale callbacks; and proves
zero owned vehicle resources.

## Platform and quality policy

Quality may scale:

- mesh and material level of detail;
- shadow, light, reflection, and particle cost;
- optional damage and debris presentation;
- skid and smoke density;
- distant traffic representation and update rate;
- optional audio layers;
- diagnostic detail; and
- native vehicle quality settings that preserve behavior within validated
  tolerances.

Quality cannot change:

- canonical vehicle identity;
- required collision and wheel topology;
- controller meaning;
- access, ownership, mission, race, retrieval, or notoriety rules;
- route legality and checkpoint semantics;
- required damage, repair, destruction, or reset results;
- local-player isolation;
- deterministic selection and transactions; or
- gameplay outcomes.

## Capacity and pooling

Vehicle, traffic, pursuit, parking, husk, component, and presentation capacities
are explicit target budgets. Capacity is not identity.

Every acquisition returns a closed result such as accepted, queued, rejected,
fallback, degraded, or cancelled. First-free array scans and silent omission are
not architecture. Required gameplay vehicles fail the owning transaction safely;
optional cosmetic presentation may use its declared fallback.

A reusable Actor or component must reset every controller, input, physics,
transform, velocity, wheel, damage, material, light, audio, VFX, occupant,
route,
mission, parking, pursuit, husk, world, feature, delegate, and diagnostic field
before reuse.

The world vehicle registry is revisioned and identity-based. It owns active,
constructing, suspended, retired, and destroyed instance records plus
controller,
occupant, route, parking, pursuit, mission, retrieval, and world leases. It does
not expose fixed active arrays, construction-ring positions, controller slots,
raw name lookup, or process-global singleton ownership.

Vehicle construction completion must correlate the exact request, definition,
asset scope, world, feature, and target registry revision. Late completion
cannot
occupy a reused slot or publish a stale vehicle. Suspension and resume operate
on
scoped leases and native Actor lifecycle rather than manually iterating one
global list.

## Concurrency and update order

Vehicle commands and observations declare native tick groups, prerequisites,
physics-thread boundaries, timestamps, and expected revisions.

The authoritative order is conceptually:

1. accept application, input, AI, route, damage, reset, and lifecycle commands;
1. prepare native asynchronous physics input;
1. execute native Chaos simulation;
1. publish one immutable physics output revision;
1. evaluate gameplay observations and accepted transactions;
1. update animation, render, audio, VFX, camera, HUD, haptics, and diagnostics;
1. process cancellation and teardown barriers.

Project code cannot manually call every vehicle manager in one frame loop or
read partially updated native state from another thread.

## Diagnostics

Read-only diagnostics may show:

- definition, instance, simulation, controller, occupant, world, and feature
  revisions;
- native physics-state readiness and asynchronous step information;
- transform, velocity, acceleration, speed, engine, gear, and control values;
- per-wheel contact, load, suspension, slip, steering, rotation, brake, drive,
  and surface values;
- AI intent, road projection, lane, route, intersection, traffic, pursuit, and
  parking state;
- collisions, damage zones, health, stages, detachable presentation,
  destruction,
  husk, reset, and recovery;
- input contexts, device assignment, haptics, camera, HUD, audio, VFX, and
  presentation leases;
- capacity, pooling, retained handles, streaming, network, and teardown;
- stale command, callback, snapshot, and ownership findings; and
- parity-test measurements.

Diagnostics cannot possess, move, accelerate, brake, damage, repair, reset,
destroy, spawn, park, pursue, retrieve, or grant a vehicle in shipping gameplay.

## Failure behavior

Closed failures include:

- missing or invalid vehicle definition;
- missing Skeletal Mesh, Physics Asset, Animation Blueprint, wheel, material, or
  movement configuration;
- unsupported native vehicle class or target;
- physics-state creation or destruction failure;
- invalid mass, center of mass, torque curve, gear, differential, steering,
  brake, suspension, tire, or wheel setup;
- invalid controller, input, occupant, route, parking, pursuit, or mission
  ownership;
- stale simulation snapshot or physics-step revision;
- non-finite transform, velocity, force, or tuning value;
- unresolved penetration, support, rollover, or recovery candidate;
- collision, damage, destruction, repair, or persistence conflict;
- missing capacity for a required vehicle;
- invalid husk or source mapping;
- streaming or feature replacement;
- network authority or correction failure;
- timeout or cancellation; and
- teardown leak.

Failure never falls through to a raw pointer, partially constructed vehicle,
unbounded retry, default model-name lookup, first-free slot, stale controller,
or hidden gameplay mutation.

## Validation

Cook and startup validation prove:

- unique vehicle, movement, wheel, damage, parking, pursuit, and presentation
  identities;
- native class, plugin, Skeletal Mesh, Physics Asset, Animation Blueprint,
  wheel,
  material, curve, collision, and asset compatibility;
- unit-labelled and finite tuning values;
- valid engine, transmission, differential, steering, brake, suspension, tire,
  and wheel configurations;
- compatible wheel bones, sockets, Physics Asset bodies, and animation bindings;
- valid damage zones, stages, materials, detachable presentation, repair, and
  destruction policy;
- valid seats, entry, exit, camera, and payload hardpoints;
- valid human input, AI intent, haptics, route, traffic, parking, pursuit,
  retrieval, race, and mission bindings;
- valid target quality, streaming, networking, and fallback policy;
- bounded capacities and complete reset requirements; and
- no packaged dependence on source player arrays, custom solver classes, raw
  event pointers, custom render passes, or mutable singleton managers.

Invalid required content fails before shipping. Optional content remains
inactive
with a typed diagnostic rather than constructing a partial vehicle.

## Tests

Required automated and integration tests include:

- construction success and rollback;
- native physics-state create, destroy, and replacement;
- deterministic input-command projection;
- Enhanced Input context acquisition and release;
- human and AI controller parity at the semantic command boundary;
- throttle, brake, handbrake, steering, reverse, horn, turbo, and special
  action;
- engine torque, gears, shifts, differential, and top-speed behavior;
- wheel contact, suspension, slip, steering, rotation, and surface response;
- traffic projection and dynamic-physics handoff;
- lane change, intersection, route, and obstacle integration;
- collision classification and damage commit;
- zone damage, material state, detachable presentation, smoke, sparks, and
  audio;
- destruction exactly once;
- husk acquire, mapping, capacity fallback, reset, and release;
- parked placement construction, clearance, occupancy, streaming, and removal;
- pursuit spawn, active budget, destruction, withdrawal, out-of-sight policy,
  and
  notoriety resolution;
- occupant enter, exit, destruction, reset, and camera handoff;
- manual reset, out-of-bounds, rollover, penetration, and mission recovery;
- pause, focus loss, suspension, and resume;
- split-screen input, camera, audio, haptics, and HUD isolation;
- feature overlay activation and removal;
- network authority, prediction, correction, and stale client rejection when
  enabled;
- quality invariance of gameplay outcomes;
- capacity exhaustion and required versus optional fallbacks;
- late callback and stale snapshot rejection;
- world unload and zero-resource teardown; and
- behavioral parity for handling, damage, traffic, pursuit, reset, and
  presentation.

## Invariants

The runtime maintains these invariants:

1. Unreal's native vehicle and Chaos systems own ordinary vehicle physics.
1. Project code does not implement a second tire, suspension, drivetrain,
   collision, or rigid-body solver.
1. Standard vehicles use fixed native topology unless a separate accepted
   modular-vehicle decision applies.
1. Stable semantic identities never derive from pointers, array positions,
   model names, joint indexes, or pool slots.
1. Human and artificial-intelligence controllers produce the same semantic
   command shape.
1. AI publishes intent and cannot write native wheel forces or solver state.
1. Road projection and dynamic physics transition through one verified handoff.
1. Native collision resolution is not replaced by a project collision solver.
1. Damage, destruction, repair, reset, notoriety, currency, mission, and roster
   changes are typed transactions.
1. Presentation cannot commit gameplay or persistence.
1. A husk is presentation associated with one destruction transaction, not a
   second vehicle.
1. Parked, traffic, pursuit, mission, race, retrieved, and player-controlled
   vehicles retain one canonical definition identity.
1. Controller, occupant, route, parking, pursuit, and feature ownership are
   revisioned leases.
1. Required vehicle capacity fails safely and visibly rather than silently
   dropping gameplay content.
1. Pooling never becomes identity and every reused object is completely reset.
1. Quality may scale cost and optional presentation but cannot alter gameplay
   semantics.
1. Network mods use explicit authority, prediction, correction, and validation.
1. Teardown rejects late commands, callbacks, snapshots, and native outputs.
1. No vehicle resource survives its owning world, feature, or transaction.

# Vehicle AI and route runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Road-network geometry and traffic runtime](road-network-geometry-and-traffic-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle audio and avatar-sound runtime](vehicle-audio-and-avatar-sound-runtime.md)

## Purpose

This specification defines native road-vehicle artificial intelligence for
traffic, waypoint routes, races, pursuit, evasion, target following, catch-up,
local obstacle avoidance, lane decisions, recovery, and presentation. It
replaces
fixed route arrays, hand-managed path segments, global debug registration, and
frame-dependent steering with validated data and bounded native controllers.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Gameplay catalog | Stable vehicle, route, lane, checkpoint, waypoint, and policy identities. |
| Mission and race services | Objective state, opponent membership, completion, failure, and catch-up permission. |
| Road-network subsystem | Immutable road, segment, lane, intersection, traffic-control, connectivity, spline, and query snapshots. |
| Vehicle AI controller | Target observation, route progress, driving state, steering requests, and recovery. |
| Vehicle movement port | Throttle, brake, steering, handbrake, reverse, turbo, and physical read-back. |
| Traffic subsystem | Ambient lane occupancy, intersection admission, density, and lifecycle. |
| UI projection | HUD and radar icon presentation from typed vehicle observations. |

<!-- markdownlint-enable MD013 -->

The controller does not own vehicle physics, mission completion, traffic
population identity, rewards, or HUD widgets.

## Runtime topology

The runtime module owns these C++ types:

<!-- markdownlint-disable MD013 -->

| Type | Responsibility |
| :--- | :--- |
| `USharVehicleAIDefinition` | Immutable controller, skill, recovery, and driving policy. |
| `USharVehicleRouteDefinition` | Ordered road, lane, waypoint, checkpoint, shortcut, and destination identity. |
| `ASharVehicleAIController` | Per-vehicle decision authority and StateTree context. |
| `USharVehicleRouteFollowingComponent` | Road-graph projection, route progress, look-ahead, and destination queries. |
| `USharVehicleLocalAvoidanceComponent` | Bounded obstacle sampling and safe steering candidate selection. |
| `USharTrafficCoordinationSubsystem` | Lane occupancy, intersection reservation, impedance events, and ambient lifecycle. |
| `FSharVehicleDriveRequest` | Desired speed, steering, brake, reverse, turbo, and validity interval. |
| `FSharVehicleAIObservation` | Immutable target, route, traffic, physics, and mission snapshot for one decision step. |

<!-- markdownlint-enable MD013 -->

Each active AI vehicle has one controller identity and one movement-command
lease. Traffic coordination may constrain the request but cannot directly mutate
mission or race state.

## Definition contract

Every `USharVehicleAIDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ControllerId` | Globally unique canonical identity. |
| `Mode` | Traffic, waypoint, race, chase, evade, target, or mission-specific adapter. |
| `StateTree` | Compatible native StateTree template. |
| `DrivingProfileId` | Speed, acceleration, braking, cornering, and reverse policy. |
| `RoutePolicyId` | Road projection, look-ahead, destination, and shortcut policy. |
| `AvoidancePolicyId` | Obstacle classes, sample bounds, clearance, and fallback. |
| `CatchUpPolicyId` | Optional bounded speed, skill, route, and distance correction. |
| `RecoveryPolicyId` | Stuck, overturned, invalid route, limbo, and reset behavior. |
| `PresentationPolicyId` | Radar, HUD, effects, and debug-view policy. |
| `DefinitionRevision` | Immutable revision used to reject stale controllers. |

<!-- markdownlint-enable MD013 -->

Every route definition contains:

- stable route and world identities;
- ordered path elements and lane choices;
- ordered authored waypoints or checkpoints when required;
- terminal destination and arrival tolerance;
- optional shortcut branches with eligibility and skill thresholds;
- expected traversal direction and lap behavior;
- road-network and overlay revisions;
- route-repopulation and streaming policy; and
- verification scenarios.

Road, segment, lane, intersection, legal-movement, speed, density, shortcut,
traffic-control, spatial-index, and path-query semantics follow
<!-- markdownlint-disable-next-line MD013 -->
[Road-network geometry and traffic runtime](road-network-geometry-and-traffic-runtime.md).
The vehicle controller consumes immutable graph results and never owns the base
road graph.

Route order is canonical. World discovery cannot silently reorder waypoints,
lanes, or shortcuts.

## Controller modes

### Traffic

Traffic mode follows a lane graph and cooperates with the traffic subsystem.
Its StateTree states include:

- `driving`;
- `waiting_at_intersection`;
- `waiting_for_free_lane`;
- `lane_changing`;
- `spline_transition`;
- `swerving`;
- `stopped`;
- `disabled`; and
- `recovering`.

Intersection entry requires an explicit reservation or admission result. A lane
change validates target-lane occupancy, remaining lane distance, merge
clearance,
and route continuity before steering begins.

### Waypoint and race

Waypoint mode follows ordered target identities and emits typed observations
when
it reaches a waypoint, the final waypoint, or the destination. The observation
contains route identity, waypoint identity, ordinal, lap, vehicle identity, and
verified world position.

A waypoint is not complete from distance alone when its definition also requires
direction, lane, trigger crossing, collectible state, or mission ownership.

### Chase

Chase mode observes a stable target identity, projects both vehicles onto the
road graph, and selects road following or bounded direct pursuit according to
the
approved beeline policy. Direct pursuit requires collision and reachability
validation and cannot ignore required road, mission, or world boundaries.

Moving targets invalidate stale path projections. Recalculation is bounded and
uses the latest complete observation snapshot.

### Evade and target

Evade mode maximizes declared separation while preserving valid road progress
and
world bounds. Target mode follows an objective vehicle, collectible, or authored
moving target according to its mission policy. Both remain route-controller
modes rather than separate physics implementations.

## Driving state

The canonical driving states are:

<!-- markdownlint-disable MD013 -->

| State | Contract |
| :--- | :--- |
| `waiting` | No drive request is emitted. |
| `waiting_for_player` | Mission policy pauses progress until the player satisfies the gate. |
| `accelerating` | Desired speed exceeds current speed within the drive profile. |
| `braking` | Planned speed or obstacle clearance requires deceleration. |
| `corner_preparation` | Look-ahead curvature requires a bounded entry speed. |
| `reversing` | Recovery or route policy explicitly permits reverse control. |
| `stopped` | A valid stop condition is active. |
| `evading` | A bounded avoidance candidate owns steering temporarily. |
| `limbo` | Presentation and collision are unavailable during a controlled transition. |
| `stunned` | External gameplay policy temporarily suspends control. |
| `out_of_control` | Physics read-back rejects normal drive authority. |
| `recovering` | The recovery transaction is evaluating or applying a safe reset. |

<!-- markdownlint-enable MD013 -->

State transitions are driven by typed observations and policy. Animation,
collision, or sound callbacks cannot select a driving state directly.

## Road projection and look-ahead

The route component projects the vehicle onto one canonical road or lane segment
and maintains a bounded look-ahead window. Projection records segment identity,
normalized progress, direction, lateral offset, and confidence.

A path window is rebuilt when:

- the target moves to another path element;
- the current segment unloads or becomes invalid;
- the route branch changes;
- the controller completes a waypoint or lap;
- recovery resets the vehicle; or
- accumulated projection error exceeds policy.

Rebuilding never searches the entire world without bounds. Candidate roads,
segments, and lanes come from one exact
[road-network snapshot](road-network-geometry-and-traffic-runtime.md), accepted
route definition, loaded regions, overlay revision, and bounded spatial query.

Closest-road projection, normalized road and segment progress, directed
traversal
distance, legal lane movements, shortcut eligibility, intersection reservations,
and deterministic path results carry graph and query revisions. A late path or
projection result cannot mutate a controller after graph replacement, region
unload, route change, or vehicle recovery.

## Local obstacle avoidance

The local avoidance component evaluates a finite set of steering candidates in
vehicle space. Each candidate receives typed costs for:

- static collision clearance;
- dynamic vehicle clearance;
- pedestrian and player safety;
- lane deviation;
- route progress;
- steering change;
- target alignment;
- reverse or stop requirement; and
- invalid or unloaded space.

Candidate order and tie-breaking are deterministic. Collision sweeps validate
the
selected candidate before it becomes a drive request. If no candidate is safe,
the controller brakes or stops according to policy instead of selecting the
least-invalid position.

This replaces fixed potential grids as gameplay authority. Debug visualizations
may render samples but cannot influence selection.

## Traffic obstacles and impedance

Traffic look-ahead distinguishes at minimum:

- ambient vehicle;
- mission or opponent vehicle;
- non-player character;
- player character;
- player vehicle;
- road end;
- blocked intersection; and
- invalid world state.

An impedance observation is rate-limited and identifies the blocked vehicle,
obstacle class, duration, route identity, and current state. It may inform
audio,
notoriety, traffic recovery, or mission diagnostics, but listener order cannot
change the current steering result.

## Catch-up policy

Catch-up is optional and definition-owned. It may adjust only declared bounded
values such as:

- target speed percentage;
- maximum speed modifier;
- shortcut eligibility or skill;
- route branch preference;
- turbo permission; and
- reset eligibility.

It cannot teleport a healthy visible opponent, ignore collision, skip required
checkpoints, change lap count, or alter player physics. The policy consumes
separation, route progress, visibility, mission state, and difficulty inputs and
produces an auditable correction result.

Race, chase, evade, and target modes may use different catch-up definitions.
Ambient traffic does not inherit race catch-up behavior.

## Shortcuts

Shortcut selection uses one deterministic skill sample derived from controller,
route, event, and session seed identities. Minimum and maximum skill bounds are
validated. A shortcut remains eligible only when its entrance, exit, loaded
world, collision, and mission conditions are valid.

The controller cannot discover arbitrary geometry and label it a shortcut at
runtime.

## Turbo

Turbo is an explicit application-port request. A waypoint or race policy may
request it only when cooldown, inventory, route, safety, and mission conditions
are satisfied. AI cannot grant itself inventory or bypass the vehicle's native
turbo authority.

## Recovery

Recovery begins only after a typed stuck or invalid-state observation. Inputs
may
include speed, wheel contact, route progress, displacement, orientation,
collision,
visibility, and elapsed simulation time.

Recovery options are ordered:

1. stop and reacquire route projection;
1. reverse within validated clearance;
1. select another safe local candidate;
1. request a nearby authored reset transform; and
1. fail the controller or mission according to policy.

A reset transform requires collision, floor, navigation, streaming, and mission
validation. Visible teleportation is forbidden unless the accepted gameplay
policy explicitly permits it.

## Physics and update order

AI decisions consume one immutable observation snapshot. The controller emits a
semantic drive intent before the vehicle physics step. Command projection,
standard Chaos vehicle simulation, wheels, suspension, powertrain, steering,
brakes, damage, reset, and immutable read-back follow
<!-- markdownlint-disable-next-line MD013 -->
[Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md).

The AI controller cannot write native wheel forces, suspension offsets, engine,
transmission, tire, rigid-body, or render-transform state. Human and artificial-
intelligence control converge on the same semantic vehicle command boundary.

Render frame rate cannot change route progress, timers, catch-up, steering
candidate order, or recovery thresholds. Presentation interpolation is separate
from simulation authority.

## UI and presentation

HUD and radar icons are projections of typed vehicle observations. The
controller
publishes target, route, destination, and state identities; it does not create
or
own widgets.

Engine, shift, reverse, in-air, skid, horn, damage, overlay, backup, and door
audio consume the same immutable vehicle observations through
<!-- markdownlint-disable-next-line MD013 -->
[Vehicle audio and avatar-sound runtime](vehicle-audio-and-avatar-sound-runtime.md).
The controller cannot select an audio clip, infer a gear from pitch, or accept
an
audio callback as movement, route, recovery, or mission evidence.

Debug route lines, path samples, potential values, target bounds, and controller
state are development-only overlays. Shipping behavior cannot depend on whether
debug rendering is registered.

## Streaming and lifecycle

A controller suspends safely when required road, lane, intersection, graph,
overlay, waypoint, target, or world state is unavailable. It may retain stable
identities and normalized route progress, but it cannot continue integrating
against unloaded pointers or a retired road-network revision.

Traffic pooling and representation changes reset controller-local target,
obstacle, route-window, timer, recovery, and presentation state. A recycled
vehicle cannot inherit the previous vehicle's mission, route, catch-up, or HUD
identity.

## Failure behavior

The controller fails closed when:

- its definition or route revision is missing or stale;
- the vehicle movement port is unavailable;
- required graph, overlay, road, intersection, lane, movement, waypoint, or
  segment identity does not resolve;
- route order, path cost, legal movement, or equal-cost tie-break is ambiguous;
- a road projection or path result is stale, truncated, or unavailable;
- no safe steering or stop response exists;
- catch-up exceeds declared bounds;
- recovery cannot prove a safe result; or
- native read-back contradicts the active state.

Failure returns a typed reason to the owning traffic, race, or mission service.
It does not silently switch controller mode or select an arbitrary destination.

## Verification

Automated verification proves:

- road-network, overlay, route, lane, waypoint, and checkpoint ordering is
  stable;
- equivalent graph and controller observations produce equivalent drive
  requests;
- equal-cost paths, closest-road candidates, legal movements, and intersection
  admissions resolve deterministically;
- traffic stops, intersection reservations, lane changes, and swerves remain
  bounded;
- waypoint, lap, final-target, and destination observations are exactly once;
- chase projection and direct pursuit obey collision and route policy;
- catch-up remains inside declared speed, shortcut, turbo, and reset bounds;
- obstacle avoidance never selects an invalid candidate;
- recovery produces a safe valid transform or a typed failure;
- graph revision, closure, region unload, and teleport trigger bounded
  replanning;
- stale, truncated, cancelled, and unavailable path results never mutate the
  controller;
- controller behavior is independent of debug rendering; and
- fixed-step replay produces the same state transitions, selected graph path,
  and
  route progress.

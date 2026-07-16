# Race route and opponent runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Gameplay census, presentation, and development-content boundary](gameplay-census-presentation-and-development-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Driving, traffic, and vehicle behavior parity](../../adr/gameplay/vehicles/driving-traffic-and-vehicle-ai.md)
- [Vehicle AI and route runtime](vehicle-ai-and-route-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission definition, stage, and objective runtime](mission-definition-stage-and-objective-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Local split-screen minigame session UI runtime](local-split-screen-minigame-session-ui-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [HUD, radar, camera, and navigation parity](../../adr/unreal/ui/hud-radar-camera-and-navigation.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Event-driven music and ambience](../../adr/unreal/runtime/event-driven-music-and-ambience.md)

## Purpose

This specification defines race identity, route topology, checkpoints, laps,
opponents, starting grids, position calculation, timers, failure conditions,
reset and retry, finish acceptance, street-race-set rewards, presentation,
mod overlays, validation, and verification.

The mission runtime owns mission-session state and completion transactions. This
specification owns the immutable race definition and the world-side observations
consumed by that mission state.

## Ownership

`USharRaceSubsystem` is a world subsystem. It owns the active race session for
the current world and publishes immutable race observations to the mission or
standalone race application port.

It consumes:

- canonical race, route, checkpoint, opponent, and catch-up definitions;
- campaign, progression, mission, and vehicle state;
- World Partition and Runtime Data Layer readiness;
- vehicle controller and fixed-step movement observations;
- timer-domain observations;
- HUD, radar, music, dialogue, and save application ports; and
- deterministic session and retry identities.

It does not grant vehicle ownership directly, calculate currency balances, infer
route membership from visible roads, or treat presentation completion as race
success.

## Race definition

`USharRaceDefinition` is a non-Blueprint primary data asset containing:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `RaceId` | Stable canonical race identity. |
| `LevelId` | Owning level. |
| `RaceClass` | Time trial, circuit, checkpoint, wager, or mission race. |
| `RouteId` | Canonical ordered route definition. |
| `StartGridId` | Ordered player and opponent start transforms. |
| `LapPolicy` | Exact lap count or no-lap checkpoint policy. |
| `TimePolicy` | Exact timer or untimed policy. |
| `OpponentIds` | Ordered opponent placements. |
| `FailurePolicyId` | Damage, out-of-vehicle, position, timeout, and mission rules. |
| `ResetPolicyId` | Player, opponent, route, timer, and world restoration. |
| `CatchUpPolicyId` | Explicit AI speed and shortcut policy. |
| `PresentationProfileId` | HUD, radar, dialogue, audio, and finish presentation. |
| `RewardPolicyId` | Optional wager or street-race-set contribution. |
| `RequiredLayers` | Exact Runtime Data Layers and streamed route cells. |
| `RevisionToken` | Deterministic generated-data revision. |

<!-- markdownlint-enable MD013 -->

A race definition references canonical vehicle and character identities. An
opponent placement never creates another vehicle or character definition.

## Route definition

`FSharRaceRouteRow` contains:

| Field | Contract |
| :--- | :--- |
| `RouteId` | Stable route identity. |
| `RaceId` | Owning race. |
| `Direction` | Forward, reverse, clockwise, counter-clockwise, or authored. |
| `CheckpointIds` | Ordered dense checkpoint identities. |
| `NavigationWaypointIds` | Ordered AI guidance points. |
| `LapClosureCheckpointId` | Checkpoint that advances the lap when applicable. |
| `FinishCheckpointId` | Exact terminal crossing. |
| `ShortcutPolicyId` | Declared legal shortcut and AI-use policy. |
| `ResetTransformIds` | Ordered safe reset transforms. |
| `RouteBounds` | Optional diagnostic and recovery corridor. |
| `RequiredLayers` | Route-specific world-layer dependencies. |

A route is accepted only when every checkpoint and waypoint resolves, ordering
is
unambiguous, the finish is reachable, and required world content is active.
Road splines, navigation meshes, road names, or visible arrows do not define
race order implicitly.

## Checkpoint observations

`FSharRaceCheckpointRow` records:

- canonical checkpoint identity;
- route ordinal;
- lap eligibility;
- swept crossing volume;
- accepted crossing direction;
- minimum forward progress;
- optional branch and merge identity;
- HUD and radar presentation;
- reset transforms associated with the checkpoint; and
- exact World Partition and Data Layer ownership.

The world adapter emits a checkpoint observation only after a fixed-step swept
crossing test proves the player vehicle crossed the accepted plane in the
allowed direction. Merely overlapping the volume, reversing through it, spawning
inside it, or unloading it is not success.

Each accepted checkpoint records race identity, session identity, route ordinal,
lap ordinal, vehicle identity, fixed-step timestamp, and observation identity.
Duplicate, stale, out-of-order, wrong-direction, or wrong-race observations are
rejected.

## Lap and checkpoint state machine

Each participant owns one race-progress state containing:

- race, route, and participant revisions;
- current lap ordinal;
- next required checkpoint ordinal;
- accepted checkpoint prefix;
- current path segment and normalized progress;
- finish state and accepted finish tick; and
- recovery snapshot identity.

A checkpoint observation advances progress only when it matches the exact next
required checkpoint for the active lap. Accepting the final route checkpoint may
activate the finish projection, but it does not finish the race until the finish
crossing itself is accepted.

A closed route advances from the closure checkpoint to the next lap only after
all required checkpoints in the current lap are accepted. The next lap begins
with a new lap revision and checkpoint prefix. The final lap closes into the
finish transaction instead of another rollover.

Opponent progress uses the same checkpoint and lap contract. AI route
controllers
may publish candidate progress, but the race runtime validates checkpoint order,
lap rollover, and finish before accepting it.

A participant that unloads, becomes inactive, or loses its controller retains
its last accepted progress or follows the declared withdrawal or recovery
policy.
It cannot be marked finished by disappearance.

## Race classes

### Time trial

A time trial has no required opponents. It requires the declared checkpoint
sequence and lap count before the exact timer expires. Position UI is omitted or
replaced by route and timer progress.

### Circuit race

A circuit race repeats one closed route for the declared lap count. Lap progress
advances only after every required checkpoint in the current lap and the lap
closure checkpoint are accepted in order.

### Checkpoint race

A checkpoint race uses one open ordered route. It has no lap rollover and
completes only when the finish checkpoint is accepted in first position when
first place is required.

### Mission race

A mission race may be a point-to-point competition against one mission target.
Its race result advances the owning mission stage but does not contribute to the
street-race set unless the race definition explicitly owns that progression row.

### Wager race

A wager race uses the same route and finish contracts with a separate reviewed
entry-fee and payout transaction. Economy settlement is atomic with race result
acceptance and follows the progression economy contract.

## Starting grid

`FSharRaceStartGridRow` declares one player transform and ordered opponent
slots.
Each transform includes location, rotation, initial gear and velocity policy,
collision readiness, required layers, and a safe reset identity.

Race activation:

1. validates the race, route, vehicle, world, progression, and mission
   revisions;
1. loads required definition, vehicle, character, audio, and presentation
   bundles;
1. reserves the start grid and checkpoint session;
1. places or validates the player vehicle;
1. spawns or reuses each declared opponent outside active traffic ownership;
1. applies opponent controller, tuning, driver, and catch-up profiles;
1. verifies every participant and route dependency;
1. publishes the immutable race projection;
1. begins the countdown and semantic race music state; and
1. starts fixed-step race evaluation only after countdown completion.

Opponent placements are mission or race instances. They never grant persistent
vehicle ownership.

## Opponent definition

`FSharRaceOpponentRow` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `OpponentPlacementId` | Stable race-scoped placement identity. |
| `VehicleId` | Canonical vehicle definition. |
| `DriverCharacterId` | Optional canonical driver presentation. |
| `StartSlotOrdinal` | Exact starting-grid position. |
| `RouteControllerId` | Circuit, waypoint, pursuit, or mission race controller. |
| `CatchUpProfileId` | Deterministic speed and recovery policy. |
| `ShortcutPolicyId` | Allowed shortcut decisions. |
| `FailureDisposition` | Recover, withdraw, or fail owning mission. |
| `PresentationProfileId` | HUD marker, radar icon, dialogue, and audio. |

<!-- markdownlint-enable MD013 -->

AI observes the same ordered route identities as the player. Navigation
waypoints
may guide steering but cannot mark checkpoints complete or change route order.
Catch-up policy may adjust bounded target speed or shortcut selection; it cannot
teleport, skip required checkpoints, or depend on frame rate.

## Position calculation

Race position uses a deterministic progress tuple:

1. completed lap count;
1. highest accepted route ordinal in the current lap;
1. projected forward distance from the last accepted checkpoint toward the next;
1. fixed-step crossing timestamp for a completed finish; and
1. canonical participant identity as the final stable tie breaker.

The tuple is recalculated from accepted route observations and fixed-step
vehicle
poses. Euclidean distance to the finish alone cannot rank participants across a
loop, branch, shortcut, or reversed segment.

HUD ordinal presentation is a projection of this tuple. A widget cannot become
position authority.

## Finish-position bonus evaluation

A finish-position bonus objective consumes the accepted ordered finish result
for
one race revision. Its definition declares the required position, eligible
participant, activation stage, reset policy, reward policy, and whether a tie is
accepted.

The bonus remains pending until the participant has an accepted finish result. A
live HUD position, temporary overtake, opponent deactivation, route-marker
state,
or predicted ordering cannot satisfy or fail it.

Checkpoint restore and full retry clear uncommitted finish-position evidence. An
accepted mission result records the optional objective once with the same race
and mission completion transaction.

## Timers and warnings

Timer policy declares the exact duration, start event, pause policy, warning
thresholds, expiration event, and retry restoration.

Timer values use the mission timer domain and fixed-step observations. Loading,
frontend transition, platform suspension, or approved pause behavior follows the
active policy. Frame rendering and audio duration never advance or expire a
race.

A timeout observation is accepted only when the timer domain reaches the exact
boundary for the active race revision. A finish crossing accepted at or before
that boundary wins according to the deterministic observation-order policy.

## Failure conditions

A race may declare:

- timeout;
- player vehicle health reaching the failure threshold;
- out-of-vehicle duration, normally ten seconds where declared;
- failure to finish in first place;
- target opponent reaching its finish first;
- leaving an authorized mission race state;
- losing a required payload or mission item; or
- invalid world or participant reconstruction.

An undeclared collision, temporary wrong turn, shortcut use, traffic contact, or
opponent damage is not automatically failure.

## Reset and retry

Retry restores the declared race checkpoint, not arbitrary current world state.
The reset snapshot contains:

- race and route revisions;
- player vehicle identity, health, and transform;
- every opponent identity, health, transform, and controller state;
- current lap and accepted checkpoint prefix;
- timer state;
- mission-owned payload or interaction state;
- traffic and population exclusions;
- HUD, radar, music, and camera state; and
- deterministic retry identity.

A full race retry normally restores the start grid and clears accepted route
observations. A mission checkpoint may restore a declared later race start only
when its definition explicitly owns that snapshot.

Reset transforms are evaluated in deterministic order and must pass swept
volume,
world readiness, and route-direction checks. Failure to find a valid transform
fails recovery without accepting race progress.

## Finish transaction

Finish is a two-stage transaction.

The world stage accepts the final crossing and freezes the participant result
ordering for the race revision. The application stage then validates the owning
mission, race-set, reward, economy, dialogue, music, and save revisions.

Commit may:

- advance a mission stage;
- mark one street race complete;
- complete the level race set when all three races are accepted;
- grant the declared race-set vehicle exactly once;
- settle a wager transaction;
- publish win or lose dialogue and music events; and
- update scrapbook and level-progress projections.

A finish animation, dialogue, audio stinger, HUD banner, or opponent despawn is
presentation and cannot substitute for the committed result.

## Street-race-set progression

Each base level has three street races. Individual completion is stored by
canonical race identity. The level street-race reward commits only when all
three
required identities are complete under the same accepted progression revision.

Replaying a completed race may produce presentation but cannot grant the vehicle
again. Wager races and mission races are excluded unless explicitly included by
the level race-set definition.

## HUD, radar, camera, and music

The race projection provides:

- current and total laps when applicable;
- accepted and remaining checkpoints;
- timer and warning state;
- deterministic position ordinal;
- opponent markers;
- next-checkpoint and finish markers;
- race-specific camera policy; and
- semantic race music state.

HUD and radar consume canonical route and participant identities. They cannot
scan world actors or infer race state from icon visibility.

Music transitions follow
[Music state and transition runtime](music-state-and-transition-runtime.md).
Race start, warning, win, lose, retry, and leave-vehicle events are distinct.

The built-in local split-screen race consumes the same canonical route, lap,
position, countdown, and result state through
<!-- markdownlint-disable-next-line MD013 -->
[Local split-screen minigame session UI runtime](local-split-screen-minigame-session-ui-runtime.md).
Its lobby and summary remain presentation adapters; the race service owns start,
finish, points, wins, times, and did-not-finish results.

## Verified route slice

<!-- markdownlint-disable MD013 -->

| Race identity | Verified contract |
| :--- | :--- |
| `rich_district_2_circuit_level_04` | Three laps; six AI route waypoints and five dense player checkpoints; opponents are Apu in the Longhorn, the Nuclear Waste Truck, and the Garbage Truck; first place required; player-vehicle destruction or ten seconds out of vehicle fails; no timer. |
| `squidport_checkpoint_level_03` | Five ordered checkpoints; opponents are Marge in the Canyonero, Sports Car A, and the road Pickup; first place required; player-vehicle destruction or ten seconds out of vehicle fails. |
| `squidport_tourist_resort_time_trial_level_06` | Two laps through eight ordered checkpoints within 115 seconds; player-vehicle destruction or ten seconds out of vehicle fails. |
| `squidport_2_checkpoint_level_06` | Six ordered checkpoints against Homer in the canonical Level 7 sports car placement; first place required; player-vehicle destruction or ten seconds out of vehicle fails. |

<!-- markdownlint-enable MD013 -->

The Level 3 and Level 6 Squidport checkpoint races are distinct route
identities.
Their similar display names cannot collapse checkpoints, opponents, rewards, or
save keys.

## Mission race integration

`S-M-R-T` uses a mission race against Skinner after the project pickup and
travel
stages. The route has three declared AI waypoints and one terminal race
crossing.
Winning advances the mission to the Springfield Elementary interior and Lisa
interaction. It does not count as a street race.

`Return of the Nearly-Dead` contains follow-and-collect and pursuit segments but
is not converted into one continuous street-race definition. Each stage retains
its own objective and timer policy.

A `race` objective in mission data references one canonical race definition. A
follow, follow-and-collect, avoid, or lose-tail stage remains its declared
objective even when another vehicle follows a route.

## Mod overlays

Validated overlays may add mod-owned races, routes, checkpoints, opponents,
presentation, and race sets. They cannot reorder immutable base checkpoints,
replace base race identity by display-name collision, or alter a base reward
without an authorized extensibility target.

A mod-owned route must pass the same closure, crossing-direction, reset,
determinism, World Partition, and reward tests before activation.

## Failure behavior

Race activation or execution fails closed on:

- an unknown or duplicate race, route, checkpoint, opponent, start, or reward
  identity;
- empty or disconnected checkpoint order;
- ambiguous lap closure or finish;
- a crossing volume without accepted direction or world ownership;
- missing vehicle, driver, controller, layer, bundle, HUD, radar, or music
  dependency;
- stale race, mission, progression, timer, or world revision;
- an opponent or player state whose read-back differs from the start plan;
- checkpoint acceptance out of order;
- a position result that depends on frame rate or container enumeration;
- a reset transform that is blocked or points against the accepted route;
- duplicate finish or reward acceptance; or
- a street-race set with missing, duplicate, or ambiguous membership.

## Verification

Automated evidence includes:

- route closure and reachability;
- dense checkpoint order and crossing direction;
- checkpoint deduplication and stale-observation rejection;
- circuit lap rollover and checkpoint-race terminal behavior;
- exact time-trial expiration and finish-boundary ordering;
- deterministic participant position on straights, loops, branches, and
  shortcuts;
- fixed-step opponent AI and bounded catch-up behavior;
- player and opponent reset, damage, and out-of-vehicle recovery;
- World Partition and Runtime Data Layer streaming during races;
- HUD, radar, camera, dialogue, and semantic music projection;
- mission races excluded from street-race-set counts;
- exactly-once street-race-set vehicle rewards;
- replay without duplicate progression;
- the four verified route definitions in this specification;
- keyboard, gamepad, and touch action parity; and
- equivalent result traces across supported platforms and graphics presets.

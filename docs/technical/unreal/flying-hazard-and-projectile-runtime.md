# Flying-hazard and projectile runtime

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Native flying-hazard actors and StateTree execution](../../adr/unreal/runtime/native-flying-hazard-actors-and-state-trees.md)
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission world-entity and respawn runtime](mission-world-entity-and-respawn-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Progression, collectibles, cheats, and credits](progression-collectibles-and-cheats.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)

## Purpose

This specification defines the native Unreal runtime for wasp cameras, UFOs,
other bespoke flying hazards, their projectiles, shields, tractor beams, spawn
points, rewards, persistence, and presentation. It replaces fixed global arrays,
scene-graph wrappers, behavior subclass arbitration, and ad hoc intersection
lists with bounded native systems.

## Ownership

The runtime has four authorities:

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Gameplay catalog | Stable identities and immutable hazard definitions. |
| Hazard subsystem | Validation, loaded-instance lookup, spawn coordination, pooling, and typed results. |
| Pawn and StateTree | Per-instance perception, movement, behavior, animation state, and task lifetime. |
| Domain services | Damage, currency, progression, mission observations, rewards, and save commits. |

<!-- markdownlint-enable MD013 -->

A collision callback, animation notify, projectile actor, visual effect, or
StateTree task may request a domain operation. It cannot write progression,
currency, mission, or save storage directly.

## Runtime topology

The runtime module owns these C++ types:

<!-- markdownlint-disable MD013 -->

| Type | Responsibility |
| :--- | :--- |
| `USharFlyingHazardDefinition` | Primary data asset containing one immutable archetype contract. |
| `ASharFlyingHazardPawn` | Loaded world representation with collision, movement, presentation, and StateTree components. |
| `ASharFlyingHazardAIController` | AI Perception configuration and StateTree context. |
| `USharFlyingHazardMovementComponent` | Swept three-dimensional movement and altitude policy. |
| `USharFlyingHazardSubsystem` | World-scoped registry, pooling, spawn coordination, and result publication. |
| `USharHazardDefenseComponent` | Shield and damage-phase state. |
| `USharTractorBeamComponent` | Beam activation, target reservations, pull transactions, and release. |
| `ASharHazardProjectile` | Pooled projectile representation with one impact authority. |
| `USharHazardSpawnDefinition` | Stable spawn identity, activation, persistence, and respawn policy. |
| `ASharHazardSpawnAnchor` | Authored world anchor that references one spawn definition. |

<!-- markdownlint-enable MD013 -->

`ASharFlyingHazardPawn` is not a progression object. The persistent identity is
owned by the catalog and save service and is rebound when presentation is
loaded.

## Definition contract

Every `USharFlyingHazardDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `HazardId` | Globally unique canonical identity. |
| `Archetype` | Closed enum such as wasp camera, attack UFO, beam UFO, or boss UFO. |
| `PawnClass` | Validated native Pawn class with required components. |
| `StateTree` | Canonical StateTree template compatible with the hazard schema. |
| `GameplayTags` | Archetype, team, target, damage, mission, and presentation tags. |
| `MovementPolicy` | Speed, acceleration, turn rate, ground clearance, altitude range, and sweep shape. |
| `PerceptionPolicy` | Sight, damage, touch, hearing, and custom stimulus filters. |
| `AttackPolicy` | Range, arc, charge, volley, cooldown, movement, and target rules. |
| `EvasionPolicy` | Trigger, distance, candidate query, speed, and recovery rules. |
| `ProjectileDefinition` | Optional projectile identity used by ranged attacks. |
| `DefensePolicy` | Health, shield phases, immunity tags, and damage reactions. |
| `TractorBeamPolicy` | Optional beam radius, height, pull rate, target filters, and completion result. |
| `RewardPolicy` | Typed reward and progression transaction identity. |
| `PresentationPolicy` | Mesh, animation, audio, effects, shadow, camera request, and accessibility cues. |
| `PoolingPolicy` | Preallocation count and reset contract. |
| `VerificationIds` | Golden scenarios required before activation. |

<!-- markdownlint-enable MD013 -->

Invalid definitions fail asset validation. Runtime fallback never guesses a
projectile, collision channel, reward, StateTree, target filter, or animation.

## Spawn definition

Every `USharHazardSpawnDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `SpawnId` | Globally unique persistent identity. |
| `HazardId` | Resolved hazard definition identity. |
| `LevelId` | Owning campaign level. |
| `Transform` | Authored world transform or anchor-relative transform. |
| `ActivationPolicy` | Always loaded, proximity, mission, data-layer, or explicit application request. |
| `PersistencePolicy` | Session, level, profile, or permanent completion state. |
| `RespawnPolicy` | Never, stream reload, cooldown, mission reset, or explicit reset. |
| `RespawnDelay` | Required only when the policy uses a cooldown. |
| `InitialStateTags` | Validated state supplied to the StateTree context. |
| `DefinitionRevision` | Immutable revision used to reject stale loaded instances. |

<!-- markdownlint-enable MD013 -->

A counted wasp-camera placement uses permanent level persistence and `never`
respawn after its destruction transaction commits. Streaming out an intact wasp
does not count as destruction and does not alter progress.

A spawn profile also declares its active-instance budget and optional
level-hazard reward-reserve identity. The base wasp profile permits at most one
active wasp in the owning world scope. Mods or later content may declare another
bounded value, but admission remains deterministic and cannot exceed the
profile.

A reserve-backed spawn atomically acquires one reward reservation before
construction. Insufficient reserve, an occupied active budget, stale world or
level revisions, proximity cooldown, or an ineligible trigger rejects the spawn
without creating a Pawn. Despawn before destruction releases the reservation;
destruction converts it to the declared coin batch under
<!-- markdownlint-disable-next-line MD013 -->
[Progression, collectibles, cheats, and credits](progression-collectibles-and-cheats.md).

## Loaded-instance registry

The subsystem maintains maps keyed by `SpawnId` and runtime instance handle. It
never uses array position as identity. Registration fails when:

- the spawn identity is already bound to another live Pawn;
- the hazard definition revision differs from the spawn request;
- required components or collision profiles are missing;
- the save state says the permanent object is already destroyed; or
- the owning world, level, or data layer is not eligible.

Unregistration records a typed reason: streamed out, pooled, destroyed, mission
reset, world teardown, or rejected. Only `destroyed` may initiate a destruction
transaction, and that transaction is idempotent by `SpawnId`.

## Pooling

Hazard and projectile pools are world-scoped. A pooled object is unavailable
until reset has cleared:

- canonical and owner identities;
- StateTree execution and external data;
- perception memories and registered stimuli;
- movement requests and velocity;
- collision responses and ignored actors;
- target, reservation, and tractor-beam handles;
- damage, shield, reward, and impact state;
- timers, delegates, audio, effects, and camera requests; and
- animation state, material parameters, and visibility.

The subsystem verifies the reset revision before reuse. A failed reset destroys
the instance and records a diagnostic instead of returning it to the pool.

## StateTree schema

The hazard StateTree schema exposes read-only definition data and typed runtime
ports. The canonical state vocabulary is:

<!-- markdownlint-disable MD013 -->

| State | Contract |
| :--- | :--- |
| `dormant` | Presentation may be loaded, but perception and movement are inactive. |
| `spawning` | Bind identity, restore persistence, reset components, and validate the initial transform. |
| `idle` | Maintain authored altitude and consume perception updates. |
| `observing` | Face or orbit a perceived stimulus without attacking. |
| `seeking_attack_position` | Run the bounded attack-position query and move to the accepted result. |
| `charging` | Lock the verified target, play charge presentation, and permit interruption. |
| `attacking` | Commit one projectile or beam activation according to the attack policy. |
| `evading` | Move to a deterministic safe result while preserving target memory policy. |
| `stunned` | Suspend attack and movement resources for the authored duration. |
| `damaged` | Apply damage reaction and evaluate shield or phase transitions. |
| `dying` | Disable attacks, commit destruction once, play terminal presentation, and release resources. |
| `despawned` | Unregister or pool after every terminal obligation is complete. |

<!-- markdownlint-enable MD013 -->

UFO templates may refine these states with `searching`, `approaching_target`,
`beam_active`, `pulling_target`, `releasing_target`, and boss damage phases. A
refinement cannot bypass the common damage, destruction, release, or persistence
contracts.

## Behavior priority

Transitions use this descending priority:

1. world teardown or permanent destruction;
1. invalid definition, identity, or persistence revision;
1. damage, shield break, stun, or mission-forced interruption;
1. active tractor-beam completion or release;
1. attack completion or cancellation;
1. evasion trigger;
1. attack opportunity;
1. observation stimulus; and
1. idle movement.

A state that owns an exclusive movement, attack, or beam resource cannot be
preempted by a lower-priority state. Conversation, cinematic, pause, and mission
policies apply explicit blocking tags rather than incrementing global counters.

## Perception and stimuli

AI Perception is the awareness adapter. Supported stimuli include:

- player sight and loss of sight;
- damage and shield damage;
- touch or collision;
- authored noise;
- nearby collectible or repair pickup;
- significant vehicle, object, jump, kick, or destruction events; and
- mission-owned attraction or suppression signals.

Gameplay stimuli carry a tag, stable source identity, location, strength,
timestamp, and expiration. The controller converts perception updates into a
sorted immutable snapshot for the next StateTree evaluation. Listener order is
never behavior priority.

A stimulus with an unloaded source may remain as a location until its
expiration,
but cannot become a target requiring a live Actor. Stale or unresolved targets
produce a typed rejection and return the StateTree to a safe state.

## Target selection

Target selection filters candidates by definition tags, mission policy, team,
liveness, collision eligibility, and range. Accepted targets are ordered by:

1. descending authored target priority;
1. ascending squared distance;
1. descending current stimulus strength; and
1. ascending stable target identity.

The selected target is revalidated before charge completion and before an attack
or beam side effect. The attack may preserve the previous target only when it
still wins the same ordering contract.

## EQS queries

EQS assets generate bounded candidate positions for observation, attack, and
evasion. Every query declares:

- a maximum result count;
- a search radius and altitude range;
- ground clearance and world bounds;
- line-of-sight and swept-path tests;
- distance from the target and player;
- mission, data-layer, and restricted-volume filters; and
- deterministic tie-breaking by quantized score and position.

The runtime accepts the highest-ranked valid result after a final collision
sweep. If no result survives, the task stops movement and returns a typed
`no_reachable_position` result. It never moves through geometry or invents a
fallback point behind the player camera.

## Movement

`USharFlyingHazardMovementComponent` owns movement simulation. Each update:

1. validates the current movement lease and destination revision;
1. computes the bounded desired velocity;
1. performs a swept move using the hazard collision shape;
1. resolves slide or stop according to the movement policy;
1. enforces minimum ground clearance and maximum authored altitude;
1. limits yaw and pitch change by the authored turn rate; and
1. publishes arrival, blocked, or invalidated status.

Long movement uses EQS-generated intermediate goals rather than a private path
planner. Short obstacle avoidance may use bounded sweeps. Movement never depends
on render visibility, camera direction, pointer order, or variable random seeds.

Randomized orbit or evasion choices use a deterministic stream seeded from the
campaign seed, `SpawnId` , definition revision, and action ordinal. Save data
does
not persist transient random-generator internals.

## Presentation and animation

Presentation consumes state; it never defines state. The standard components
support:

- wasp wing animation driven by an Animation Blueprint, Control Rig, or material
  motion with an authored rate;
- charge, attack, shield-hit, shield-break, damage, death, beam, and idle cues;
- UFO hover or bob motion that cannot change the authoritative collision
  transform unless included in movement policy;
- component-level shadows and lighting without gameplay visibility checks; and
- audio and effects with explicit start, stop, cancellation, and pooling reset.

Animation notifies may publish a presentation milestone. Attacks and rewards
still require the authoritative StateTree task and domain transaction.

## Projectile definition

Every projectile definition contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ProjectileId` | Stable canonical identity. |
| `ActorClass` | Validated pooled projectile class. |
| `CollisionProfile` | Dedicated query and object responses. |
| `Shape` | Sphere or capsule dimensions used for every sweep. |
| `InitialSpeed` | Positive speed in Unreal units per second. |
| `MaximumSpeed` | Not less than the initial speed. |
| `Lifetime` | Positive maximum lifetime. |
| `GravityScale` | Authored value, normally zero for wasp bolts. |
| `ImpactPolicy` | Typed player, vehicle, world, shield, and ignored-target effects. |
| `PresentationPolicy` | Mesh, trail, audio, impact effect, and accessibility cue. |
| `SubstepPolicy` | Maximum simulation step and iteration count. |

<!-- markdownlint-enable MD013 -->

## Projectile lifecycle

Projectile activation requires a valid definition, owner identity, source
transform, normalized direction, and action ordinal. The projectile:

1. resets its pooled state;
1. records owner and ignored-actor handles;
1. enables collision and presentation;
1. performs swept substepped movement;
1. resolves the first authoritative blocking impact;
1. requests the typed impact effect exactly once; and
1. disables and returns to the pool after impact, timeout, cancellation, or
   world teardown.

Overlap order cannot produce multiple penalties. An impact result contains the
projectile identity, owner identity, target identity, hit location, normal,
physical surface, action ordinal, and reason.

## Wasp-camera contract

A wasp-camera definition provides the level-specific projectile volley, shield,
evasion, and reward policy already declared by the gameplay census. Additional
runtime requirements are:

- observation may become hostile after a qualifying nearby stimulus;
- attack requires a live target, range, arc, line of sight, and visible charge
  presentation;
- the projectile penalty is requested once per accepted impact;
- shielded wasps consume shield damage before body destruction;
- vehicle, player, projectile, and other-hazard damage use one damage port;
- destruction commits the placement identity, progression, and reward once;
- streaming or pooling does not grant the reward; and
- a permanently destroyed placement is not recreated in the same save.

A wasp may damage another eligible wasp only when the projectile impact policy
explicitly permits friendly or neutral hazard targets.

## Shield contract

The defense component exposes `absent`, `active`, `hit_reaction`, `broken`, and
`disabled` states. Damage evaluation returns one of:

- rejected by immunity or mission policy;
- absorbed with shield remaining;
- shield broken with no body damage;
- shield broken with overflow body damage;
- body damaged; or
- terminal destruction.

Shield presentation follows the result. It cannot determine damage by itself.
Pooling, reset, and mission restart restore only the state declared by the spawn
and save policies.

## Tractor-beam contract

The tractor-beam component owns a bounded spatial query and target reservations.
A target must pass class, mission, team, mass, collision, and liveness filters.
Accepted targets are sorted by distance and stable identity before the
configured
maximum count is applied.

Beam activation follows these phases:

1. validate the owning UFO and beam definition;
1. query and reserve eligible targets;
1. publish beam-start presentation;
1. apply bounded pull acceleration toward the authored capture point;
1. revalidate each target on every authoritative update;
1. commit the configured capture or destruction result at the capture point;
1. release every target reservation; and
1. publish beam-stop presentation.

A damaged, destroyed, unloaded, cancelled, or mission-disabled UFO releases all
targets. A target that becomes invalid is released without a capture result.

## Tractor-beam target ownership

Each accepted beam target owns one reservation containing hazard, beam, target,
mission, world, and request revisions. A reserved dynamic object remains owned
by
its original domain until the capture transaction commits.

The beam may apply bounded pull acceleration and orientation presentation while
the reservation is valid. It cannot remove a traffic vehicle, mission prop,
payload, or persistent world object from its owning service before the accepted
capture or destruction result.

At the capture point, the beam submits one typed result. The target's owning
service decides whether to destroy, consume, detach, relocate, disable, or
reject
the result. Scene removal, render disappearance, and actor release are teardown
consequences rather than gameplay authority.

Cancelling the beam releases every reservation and restores normal simulation or
enters the target's declared recovery policy. A target cannot remain referenced
by a destroyed or unloaded hazard.

## UFO encounter contract

Standard UFO behavior uses target search, approach, beam activation, capture,
and release states. Boss UFO behavior additionally declares:

- ordered damage phases;
- phase-specific vulnerability tags;
- authored movement anchors or EQS query parameters;
- mission observations emitted by verified phase transitions;
- terminal destruction and cinematic handoff; and
- exactly-once boss reward or progression results.

The boss cannot infer a phase transition from animation state, mesh damage, or
camera completion.

## Damage, destruction, and rewards

All damage requests include source, target, damage type, magnitude, hit result,
mission context, and action ordinal. The domain service returns the
authoritative
result before presentation changes.

Destruction is idempotent by persistent spawn identity. The transaction order
is:

1. reject a duplicate or ineligible request;
1. commit damage and terminal state;
1. record persistent destruction when required;
1. record counted progression when required;
1. convert the accepted hazard-reward reservation into one declared coin batch,
   or grant another registered reward exactly once;
1. publish mission and presentation observations; and
1. permit pooling or unloading.

Partial failure before the commit leaves persistent state unchanged. Failure
after commit retries publication from the committed transaction record and never
grants the reward again.

## Camera presentation

A hazard may submit a typed camera-interest request containing subject identity,
reason, priority, minimum and maximum duration, interruption policy, and framing
hints. The camera subsystem accepts, rejects, or supersedes the request.

The hazard stores only its request handle. It never stores the previous camera,
switches cameras directly, or restores a camera after cancellation. Camera
rejection cannot block damage, destruction, mission, or reward completion.

## Streaming

World Partition and Runtime Data Layers control presentation lifetime. Before a
cell unloads, the subsystem records whether the instance is intact, temporarily
inactive, mission-owned, or permanently destroyed. Active beam, projectile,
camera, and target reservations are cancelled before unregistration.

Loading a cell resolves the current definition and persistent state before
spawning. A stale definition revision fails closed and records a validation
error; it does not silently reuse an incompatible pooled Pawn.

## Historical optimization translation

<!-- markdownlint-disable MD013 -->

| Historical technique | Original constraint | Unreal replacement |
| :--- | :--- | :--- |
| Fixed actor arrays and manual banks | Avoid dynamic allocation and old-console memory pressure. | World subsystem registries plus validated actor and projectile pools. |
| Distance-based actor removal | Keep a small active set without world streaming. | World Partition, data-layer activation, significance, and persistent spawn state. |
| Private static and dynamic intersection lists | Avoid repeated broad scene queries. | Collision channels, bounded sweeps, AI Perception, and EQS tests. |
| Hand-built flying waypoints | No native three-dimensional query workflow. | EQS candidate positions plus custom swept movement. |
| Visibility-gated attack checks | Avoid off-screen work and presentation anomalies. | Significance may reduce presentation cost; attack eligibility remains gameplay-driven. |
| Scene-graph shield and beam props | Couple rendering and gameplay state. | Dedicated defense and tractor-beam components with independent effects. |
| Procedural drawable mutation | Limited animation asset pipeline. | Animation Blueprint, Control Rig, component animation, and material parameters. |
| Global event listeners | Centralized low-cost awareness. | AI Perception stimuli and typed application events. |

<!-- markdownlint-enable MD013 -->

Native optimization cannot change attack timing, target ordering, damage,
rewards, persistence, or mission observations.

## Invariants

- Every loaded hazard resolves to one definition and, when persistent, one spawn
  identity.
- A persistent spawn identity has at most one live Pawn.
- The base wasp profile has at most one active wasp in its owning world scope.
- A reserve-backed live wasp owns exactly one accepted reward reservation.
- A pooled object has no previous owner, target, reward, collision, or task
  state.
- Every movement step is swept and bounded.
- Every projectile has one owner, one lifetime, and at most one impact result.
- Every beam target has at most one reservation from one beam instance.
- Damage and destruction are committed before presentation-only consequences.
- Counted destruction and rewards are exactly once.
- Streaming cannot convert unloading into destruction or respawn a permanently
  destroyed placement.
- Camera acceptance is never required for gameplay completion.

## Failure behavior

The runtime fails closed when a definition, StateTree, collision profile,
projectile, spawn identity, reward, or persistence revision is invalid. The
instance enters a non-attacking diagnostic state, releases resources, and is
unregistered or destroyed safely.

A blocked movement query stops the Pawn and returns a typed failure. A
projectile
activation failure produces no projectile and no cooldown commit unless the
attack policy explicitly consumes the attempt. A beam failure releases all
reservations. A reward or save failure prevents terminal cleanup until the
transaction is committed or safely abandoned according to persistence policy.

## Validation

Asset validation rejects:

- duplicate hazard, projectile, or spawn identities;
- incompatible Pawn, controller, component, or StateTree classes;
- missing collision profiles or invalid response combinations;
- non-positive speed, lifetime, range, timeout, or pool bounds;
- impossible altitude or clearance ranges;
- attack policies without a compatible projectile or beam;
- rewards without idempotency identity;
- non-positive active-hazard budgets;
- reserve-backed spawns without a valid reserve identity and reward amount;
- permanent counted placements with a respawn policy other than explicit reset;
- StateTree tasks that claim undeclared resources; and
- references outside canonical package identities.

## Verification

Automated tests must prove:

- deterministic target and EQS-result ordering under shuffled input;
- attack range, arc, charge, cooldown, and interruption behavior;
- no movement or projectile tunneling across supported frame rates;
- shield absorption, break, overflow, reset, and pooling behavior;
- one projectile impact and one penalty from overlapping collision callbacks;
- one wasp progression and reward commit under repeated destruction requests;
- active-budget rejection when the base world already has one accepted wasp;
- reserve acquisition, emitted batch, collected-unit removal, expired-unit
  return,
  cancellation release, and duplicate-callback rejection;
- persistent destruction across stream unload and reload;
- no reward from unload, pool return, or rejected spawn;
- beam reservation, pull, capture, invalidation, and cancellation cleanup;
- boss phase ordering and exactly-once terminal results;
- camera rejection and interruption without gameplay blockage;
- complete pooling reset after success, failure, cancellation, and world
  teardown;
  and
- parity scenarios for every wasp and UFO definition used by the campaign.

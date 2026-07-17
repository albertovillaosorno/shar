# Ambient population and named-character runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Mass Entity ambient population](../../adr/unreal/runtime/mass-entity-ambient-population.md)
- [Pedestrian path runtime](pedestrian-path-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)
- [Presentation playback runtime](presentation-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Dialogue selection, queue, and playback runtime](dialogue-selection-queue-and-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native flying-hazard actors and StateTree execution](../../adr/unreal/runtime/native-flying-hazard-actors-and-state-trees.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Flying-hazard and projectile runtime](flying-hazard-and-projectile-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)

## Purpose

This specification defines ambient pedestrian identity, zones, archetypes,
weighted groups, deterministic spawning, representation LOD, movement,
awareness,
threat response, impacts, conversations, named-character promotion, streaming,
mission ownership, platform budgets, failure, and verification.

Bespoke flying hazards, counted wasp cameras, UFO encounters, and their
projectiles require full Actor, collision, animation, and StateTree lifecycles.
They are excluded from Mass population ownership and follow the
[flying-hazard and projectile runtime](flying-hazard-and-projectile-runtime.md).

## Ownership

`USharAmbientPopulationSubsystem` is a world subsystem. It owns the active
population revision for the connected world, chapter unlocks, gameplay state,
clock phase, weather, and mission projection. It consumes:

- canonical character and population definitions;
- chapter, discovery, gameplay-state, Runtime Data Layer, and World Partition
  state;
- population-zone and pedestrian-path records;
- Mass entity templates and representation policies;
- player, traffic, mission, interaction, and notoriety observations;
- platform and quality presentation budgets; and
- deterministic session seed material.

It does not own character identity, mission progression, dialogue text, Smart
Object authority, or save transactions.

## Population definition

`USharPopulationDefinition` is a non-Blueprint primary data asset with:

| Field | Contract |
| :--- | :--- |
| `PopulationId` | Stable population-profile identity. |
| `ChapterPredicate` | Chapter and persistent unlock availability. |
| `GameplayStatePredicate` | Mission or non-mission eligibility. |
| `ClockAndWeatherPredicate` | World phase, Chapter 7 atmosphere, and hazards. |
| `RequiredLayerSetId` | Exact Runtime Data Layer composition. |
| `ZoneIds` | Ordered population zones. |
| `ArchetypeGroupIds` | Allowed weighted archetype groups. |
| `GlobalBudgetPolicyId` | Presentation budget and representation policy. |
| `SessionSeedPolicyId` | Stable seed derivation policy. |
| `RequiredNamedPlacements` | Named placements that must be present. |
| `RevisionToken` | Deterministic generated-data revision. |

A population definition is activated only after its chapter, gameplay-state,
clock, weather, discovery, and layer predicates are accepted. It never infers
population from visible sidewalks or loaded meshes.

## Zone definition

`FSharPopulationZoneRow` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ZoneId` | Stable zone identity. |
| `LevelId` | Owning level. |
| `PlacementBounds` | Authored world-space bounds or path membership. |
| `PedestrianPathIds` | Exact navigation and sidewalk paths. |
| `ArchetypeGroupId` | Weighted group eligible in the zone. |
| `TargetDensity` | Base disposable density before presentation scaling. |
| `MinPlayerDistance` | Minimum safe spawn distance. |
| `MaxActivationDistance` | Maximum activation distance. |
| `RemovalDistance` | Distance after which disposable entities may be removed. |
| `VisibilityGrace` | Off-screen duration before removal eligibility. |
| `ConversationPolicyId` | Optional ambient conversation policy. |
| `ThreatPolicyId` | Reaction and recovery policy. |
| `RequiredLayers` | Exact Runtime Data Layers. |
| `ExclusionVolumes` | Mission, traffic, interior, and unsafe spawn exclusions. |

<!-- markdownlint-enable MD013 -->

Activation and removal distances form hysteresis. The removal boundary must be
outside the activation boundary so entities do not churn at one threshold.

## Archetypes and weighted groups

`FSharAmbientArchetypeRow` records:

- canonical archetype identity;
- optional canonical named character identity;
- Mass entity template and traits;
- actor and instanced representation assets;
- locomotion and animation profile;
- voice and reaction-event profile;
- collision and impact profile;
- awareness, look-at, and threat policies;
- Smart Object capabilities;
- allowed levels, zones, and layer variants;
- maximum simultaneous count; and
- whether the row is disposable, promotable, or actor-required.

`FSharAmbientArchetypeGroupRow` contains a deterministic ordered set of
archetype
identities and positive integer weights. Zero, negative, duplicate, missing, or
platform-dependent weights are invalid.

A stable random choice is derived from population revision, level, zone,
spawn-slot identity, and session seed. Frame time, thread order, camera jitter,
and container enumeration are not seed material.

## Mass composition

Repository-owned Mass traits provide:

- canonical identity and placement fragments;
- level, zone, group, and policy fragments;
- path movement and target fragments;
- avoidance and obstacle fragments;
- look-at and awareness fragments;
- threat, panic, fall, and recovery fragments;
- conversation and Smart Object fragments;
- actor-promotion and mission-pin fragments;
- representation LOD fragments; and
- deterministic lifecycle and diagnostic fragments.

Mass processors operate on fragments and tags, never display names or object
paths. Processing order is declared through groups and dependencies and remains
stable for equivalent observations.

Mass StateTree owns disposable pedestrian behavior. Repository-owned evaluators,
tasks, and conditions consume typed observations and emit movement, look,
conversation, reaction, fall, recovery, interaction, or promotion intent.
StateTree never commits mission or save progress.

## Ambient start vignettes

`USharAmbientVignetteDefinition` selects safe non-mission presentation for a
playable character after new game, load, or declared free-roam return. It
includes location, character, animation, prop, audio, world-clock, chapter,
cooldown, weight, and cancellation predicates.

Homer's initial set may include eating a donut, performing a gag, idling at
home, or appearing at Moe's Tavern. Vignettes are presentation-only and release
cleanly when the player moves, switches character, opens a mission, or loads
another state.

## Chapter 7 zombies

Chapter 7 population profiles may spawn zombie archetypes with actor promotion,
melee attack, navigation, health damage, horror audio, and mission pinning.
Hostility consumes character and costume tags. The Devil Homer disguise
suppresses ordinary zombie target acquisition but does not affect radiation,
bosses, scripted hostility, or explosion damage.

Zombie density, representation, audio, and distant silhouettes respond to clock
phase, irradiated weather, visibility, and quality budgets without changing
combat rules.

## Representation LOD

Each archetype defines four presentation tiers:

1. actor representation for near, interactive, mission, or talkable entities;
1. low actor representation for nearby non-interactive entities;
1. instanced static mesh representation for distant disposable entities; and
1. no representation for valid far simulation or removal candidates.

Representation changes preserve Mass entity identity, path state, reaction
state,
and deterministic seed. They cannot create a new canonical character or replay a
conversation.

Required named, mission, interaction, driver, race-host, cinematic, or gag
placements are never reduced below the representation required by their active
contract.

## Actor-backed pedestrian lifecycle

A promoted or near-field pedestrian uses one stable character instance and an
explicit population lease. The lease contains population, archetype,
model-group,
character, path, zone, world, feature, representation, and controller revisions;
activation state; visibility and distance policy; and teardown.

Activation validates the character definition, model and animation readiness,
path projection, floor, capsule clearance, minimum player distance, nearby-
pedestrian spacing, and current population budget. Deactivation releases path,
controller, dialogue, reaction, presentation, and retained-asset ownership
before
the Actor is hidden, pooled, or destroyed.

Model groups are immutable weighted definitions selected by stable zone and
session seeds. Switching a group changes future eligible representations and may
replace disposable pedestrians through a revisioned transaction. It cannot swap
a model under a named, mission, interaction, conversation, driver, or otherwise
reserved character.

Walking, brisk walking, running, backing away, path following, waypoint arrival,
panic, avoidance, and recovery are typed movement intents consumed by the
playable-character and pedestrian-path contracts. Speed, spacing, panic radius,
out-of-sight lifetime, model capacity, and activation budgets are authored,
unit-labelled policy rather than fixed manager constants.

Vehicle contacts and threat observations consume immutable collision and vehicle
state from
<!-- markdownlint-disable-next-line MD013 -->
[Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md).
A pedestrian reaction cannot damage a vehicle, adjust notoriety, grant currency,
or commit mission state without the owning typed transaction.

Fixed seven-pedestrian arrays, first-free slots, generated names, mutable model-
in-use counters, raw character pointers, and camera-poll activation are not
target
identity or lifetime authority.

## Spawn planning

Each update builds a bounded candidate plan rather than spawning directly from
camera position.

A spawn candidate is eligible only when:

- its zone, path, World Partition cell, and required Data Layers are active;
- the zone is below its current disposable target;
- the selected archetype is below its group and archetype limit;
- the position is outside declared exclusion volumes;
- the swept capsule is clear of pedestrians, named actors, vehicles, Smart
  Objects, mission targets, and blocking world geometry;
- the navigation and path direction are valid;
- the position is outside the minimum player distance; and
- the candidate identity has not already been accepted.

Candidate ordering is deterministic by zone, path, spawn slot, and archetype.
Each frame admits only the bounded number allowed by the population budget.

## Pedestrian path integration

Population zones select only paths whose chapter, gameplay-state, world-layer,
and archetype predicates are active. Path assignment, segment progress,
capacity,
ZoneGraph projection, Smart Object handoff, promotion, and streaming follow the
[pedestrian path runtime](pedestrian-path-runtime.md).

A spawned ambient entity receives one accepted path reservation before movement.
Population density cannot exceed path capacity by creating untracked walkers.
When no eligible reservation exists, the planner waits, chooses another path, or
skips one disposable spawn according to policy.

Named-character promotion transfers the same path identity, segment progress,
and reservation to the actor representation. A mission or interaction takeover
may pause, reroute, or release the assignment, but it cannot strand path
capacity.

Cosmetic blink and facial-idle layers follow the
[presentation playback runtime](presentation-playback-runtime.md). They remain
non-authoritative and may scale with representation quality without changing
identity, awareness, conversation, or mission behavior.

## Removal and streaming

A disposable entity becomes removable only when:

- it is outside the removal boundary or its owning cell is leaving the active
  world;
- it has exceeded the visibility grace when visibility is material;
- it is not in an impact, fall, recovery, conversation, interaction, or
  promotion
  transaction;
- it is not a mission, driver, race-host, cinematic, gag, or named placement;
- it owns no Smart Object reservation; and
- no gameplay subsystem has pinned it.

Removal releases representation and Mass state after diagnostics are recorded.
Streaming out and back in may generate a new disposable entity for the same zone
slot, but cannot duplicate a stable named placement or save-relevant identity.

## Awareness and look-at

Ambient awareness uses bounded perception observations. A pedestrian may look at
the player when the player is within the declared range, visible, and not
superseded by threat, conversation, interaction, or mission intent.

Look-at has enter and exit thresholds, maximum duration, cooldown, and head or
body rotation limits. It does not run an unbounded world query per entity.

## Vehicle and violence reactions

Threat observations include:

- approaching vehicle trajectory and time to closest approach;
- vehicle horn within the declared response zone;
- accepted nearby pedestrian impact;
- player melee or destructive action;
- notoriety pursuit or environmental hazard; and
- scripted level-specific panic events.

The reaction state is one of observe, flinch, evade, panic, fall, recover, or
resume. Priority and interruption rules are explicit.

A vehicle-evade task chooses a validated lateral or path-safe destination. It
does
not teleport through geometry or into traffic. Failure to find a safe
destination
uses the declared brace or stop fallback.

An accepted kick or vehicle impact may transition the pedestrian to physical
fall
or ragdoll presentation. Recovery requires bounded linear and angular velocity,
a valid floor, a clear stand capsule, no continuing threat, and the declared
minimum rest time. Failure retains a safe fallen or recovery state rather than
snapping upright inside geometry.

Notoriety events are emitted through the notoriety port. The population system
does not apply fines or pursuit directly.

## Ambient conversations

Conversation candidates require:

- compatible conversation capabilities and voice profiles;
- both entities in normal ambient state;
- a declared distance and facing relationship;
- no active threat, mission, player interaction, or Smart Object reservation;
- a deterministic conversation event identity; and
- cooldown eligibility for both participants.

A short reservation prevents a third entity from taking either participant.
Eligibility publishes immutable participant and event context to
<!-- markdownlint-disable-next-line MD013 -->
[Dialogue selection, queue, and playback runtime](dialogue-selection-queue-and-playback-runtime.md),
which owns deterministic line matching, queueing, positional playback,
subtitles,
and completion observations.

The population service retains movement and reservation authority. Dialogue
selection, audio completion, subtitles, or mouth animation cannot release the
participants, create mission progress, or write permanent relationship state.
The reservation ends only through its accepted interaction or population result.

## Named-character placements

A named character has one `USharCharacterDefinition`. Each appearance is a
placement row declaring one or more capabilities:

- disposable ambient;
- persistent ambient;
- talkable;
- mission giver or target;
- race host or finish presenter;
- driver;
- cinematic;
- gag participant; or
- frontend idle presentation.

A placement requiring stable interaction, mission, dialogue, cinematic, save, or
driver ownership is actor-required. It is authored as an actor or promoted from
a
Mass entity before the owning transaction begins.

Actor construction, character definitions, movement, artificial-intelligence and
non-player-character controllers, collision, rendering, props, camera targets,
and footprint presentation follow
<!-- markdownlint-disable-next-line MD013 -->
[Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md).

Promotion performs:

1. reserve the Mass entity and placement identity;
1. load the actor presentation and interaction bundles;
1. spawn the candidate actor at the accepted transform;
1. transfer canonical identity, path, animation, reaction, and dialogue context;
1. register mission, interaction, or driver ownership;
1. verify the actor and Mass representations agree;
1. commit actor authority; and
1. remove or suspend the disposable representation.

Demotion occurs only when all actor-required capabilities and reservations have
ended. A failed promotion preserves the Mass entity and rejects the requesting
interaction or mission activation.

## Minor-character census boundary

Character census and non-story index pages are coverage evidence. They do not
create aggregate `minor_characters` or `non_story_characters` runtime entities.

Each named person receives a canonical character definition when evidence is
covered. Group membership is a tag or query projection. Background, driver,
race-host, gag, mission, cinematic, and talkable roles are placement
capabilities,
not separate characters.

## Platform and quality budgets

`FSharPopulationBudgetPolicy` declares per target and quality preset:

- disposable Mass entity target;
- actor and low-actor representation limits;
- instanced representation limit;
- spawn and removal work per frame;
- perception and reaction query budgets;
- animation and audio concurrency; and
- emergency degradation order.

Budgets may reduce disposable density, update rate, representation fidelity,
animation complexity, and optional voice concurrency. They cannot remove
required
named placements, alter mission behavior, disable player interaction, or change
notoriety semantics.

## Failure behavior

Population activation fails closed on:

- unknown level, zone, path, archetype, group, trait, or policy identity;
- invalid weights, counts, distances, bounds, or hysteresis;
- a required named placement without a character definition;
- duplicate placement or Mass identity;
- missing World Partition, Data Layer, navigation, representation, or StateTree
  dependency;
- a spawn intersecting an exclusion or occupied volume;
- actor promotion or read-back mismatch;
- nondeterministic candidate ordering or seed use; or
- a quality policy that removes required gameplay placements.

A malformed disposable row is rejected without removing valid population. A
missing required named placement fails the owning level or mission activation.

## Verification

Automated evidence includes:

- deterministic population plans from equivalent level, layer, zone, seed, and
  budget input;
- weighted-group and maximum-count distribution tests;
- path, navigation, occupied-volume, and exclusion rejection;
- activation and removal hysteresis;
- World Partition and Data Layer streaming cycles;
- actor, low actor, instanced, and no-representation transitions;
- look-at enter, exit, cooldown, and priority behavior;
- horn, violence, nearby impact, vehicle approach, evade, fall, and recovery;
- safe recovery floor and stand-capsule checks;
- two-party ambient conversation reservation and cancellation;
- named placement actor promotion and demotion;
- mission, race-host, driver, cinematic, gag, and Smart Object pinning;
- duplicate canonical character prevention;
- Low through Ultra disposable-density scaling with invariant required actors;
- Android lifecycle and low-memory representation reduction; and
- repeated runs producing equivalent placement and state traces.

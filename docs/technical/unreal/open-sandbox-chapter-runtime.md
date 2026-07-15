# Open sandbox chapter runtime

- Status: Active
- Last reviewed: 2026-07-15

<!-- markdownlint-disable MD013 -->

## Governing decisions and design

- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
- [Faithful seven-chapter open-world scope](../../adr/pipeline/unreal/faithful-seven-chapter-open-world-scope.md)
- [Unified open world and chapter projection](../../adr/pipeline/unreal/unified-open-world-and-chapter-projection.md)
- [Open sandbox campaign design](../gameplay/open-sandbox-campaign-design.md)
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)

## Purpose

This specification defines the native Unreal implementation of the connected
sandbox, seven chapters, mission and non-mission states, cumulative unlocks,
world clock, map discovery, character switching, checkpoints, economy, boss
areas, achievements, taxi work, traversal, interiors, Chapter 7 survival, and
cel-shaded presentation.

Historic source level identifiers remain conversion aliases only. Runtime and
player-facing authority uses chapter, world, mission, and unlock identities.

## Runtime topology

The runtime owns:

- `USharCampaignSubsystem`, chapter order, story completion, and chapter unlocks;
- `USharSandboxStateSubsystem`, the exclusive `mission` or `non_mission` state;
- `USharWorldClockSubsystem`, the 24-minute clock and sleep transactions;
- `USharMapDiscoverySubsystem`, fog, landmarks, routes, and terrain discovery;
- `USharCharacterEligibilitySubsystem`, unlock and story availability;
- `USharCollectibleActivationSubsystem`, cumulative chapter-set activation;
- `USharAchievementSubsystem`, pending base and mod-aware achievement projection;
- `USharSideActivitySubsystem`, taxi, race, wager, and other activity ownership;
- `USharWorldExpansionSubsystem`, bosses, structures, shortcuts, and permanent
  area unlocks; and
- the existing mission, save, progression, input, camera, audio, population,
  loading, and mod-overlay subsystems through typed ports.

No subsystem discovers authority from loaded actors, Data Layer names, map
filenames, widget state, or source level ordinals.

## Gameplay-state contract

`FSharGameplayStateSnapshot` contains:

- state kind, `mission` or `non_mission`;
- state revision;
- world and session identity;
- current chapter boundary;
- optional active mission or activity identity;
- controlled character and eligibility revision;
- active checkpoint when present;
- world-clock revision;
- map-discovery revision; and
- persistent-world revision.

The sandbox subsystem accepts one transition at a time. Entering `mission`
requires a validated mission or activity transaction. Returning to `non_mission`
removes transient mission projection and commits only accepted persistent results.

Application states such as loading, pause, frontend, and failure presentation do
not create additional gameplay-state values.

## Campaign and chapter assets

`USharCampaignDefinition` contains:

- `CampaignId`;
- exactly seven ordered `ChapterIds`;
- initial chapter and character;
- ending mission and credits identities;
- chapter-completion presentation policy;
- overall completion and achievement policy;
- default world-clock profile;
- initial terrain and discovery policy; and
- deterministic revision.

`USharChapterDefinition` contains:

- chapter identity and ordinal;
- source aliases such as historic level identity;
- ordered story and bonus missions;
- races, wagers, taxi milestones, and optional boss slot;
- forced and unlocked characters;
- terrain, route, interior, and shortcut unlocks;
- collectible activation set;
- ambient population and audio profile;
- chapter weather and hazard profile;
- completion rewards and presentation; and
- successor chapter.

The catalog validates exactly seven dense base chapters. A mod may add a separate
campaign but cannot insert into the base order or reuse base save identities.

## New-game transaction

New game:

1. creates the initial portable progression revision;
1. activates the persistent geographic world;
1. enters `non_mission`;
1. selects Chapter 1 as the narrative boundary;
1. unlocks Homer and terrain family 1;
1. activates Chapter 1 persistent collectible sets;
1. initializes the map with undiscovered cloud fog;
1. starts the world clock at the authored time;
1. initializes renewable coin-source eligibility;
1. requests a valid Homer ambient vignette; and
1. verifies that no mission projection exists.

The vignette uses a data-defined weighted set with conditions, cooldowns, world
locations, animation, props, and audio. It cannot mutate progression.

## Mission acceptance

`FSharMissionAcceptanceRequest` validates:

- mission identity and chapter availability;
- one active gameplay activity only;
- required character, costume, vehicle, and location;
- terrain, interior, boss, and shortcut gates;
- time-window policy;
- mission bundles and streaming readiness;
- checkpoint graph;
- save compatibility; and
- expected world and campaign revisions.

Mission projection is a move-only lease. It owns mission-specific actors,
vehicles, pickups, hazards, routes, Data Layers, dialogue, audio, camera, UI, and
objective state. Releasing it removes transient content in deterministic order.

Persistent mission results use typed transactions and are never inferred from
actors that happen to remain loaded.

## Checkpoint save and load

`FSharMissionCheckpointSnapshot` stores:

- mission and checkpoint identities;
- chapter and gameplay-state revision;
- objective graph state;
- controlled character, costume, and eligibility;
- vehicle identity, role, condition, and transform;
- health and stamina;
- timer and world-clock policy;
- required inventory and mission-local counters;
- accepted persistent transaction identities; and
- retry transform and streaming dependencies.

Manual save during a mission records the latest accepted checkpoint. Loading
reconstructs the mission lease and checkpoint state after all dependencies are
ready. Duplicate rewards, collectibles, achievement progress, boss unlocks, and
currency transactions are rejected by transaction identity.

## Chapter completion

Completing a final story mission commits one chapter-completion transaction. It:

- records completion exactly once;
- activates the next chapter's persistent collectible set;
- applies character, terrain, route, interior, shortcut, boss-area, and side
  activity unlocks;
- publishes chapter-completion presentation;
- updates achievement projections;
- exposes the next mission marker; and
- performs an optional forced protagonist and mission-start transition.

User-facing presentation says chapter, never level. Source level aliases may
appear only in diagnostics and conversion evidence.

## Character eligibility

`FSharCharacterEligibilitySnapshot` distinguishes:

- discovered identity;
- permanently unlocked identity;
- currently eligible identity;
- mission-forced identity; and
- temporary story lock reason.

Homer is initially unlocked. Bart unlocks after the final Chapter 1 mission and
may be automatically selected for the next accepted mission. Bart is locked after
Chapter 2 until Chapter 4 completes. Lisa missions force Lisa. Outside missions,
any unlocked and eligible character may be selected from the menu.

Character switching uses a safe-placement query and cannot cross terrain,
interior, mission, boss, or shortcut gates.

## Costumes

The frontend projects every costume from the start. Purchase is one atomic
currency and ownership transaction. Owned costumes are menu-equippable at safe
points.

Mission definitions may force or forbid a costume. The Devil Homer costume adds
a zombie-disguise tag consumed by Chapter 7 hostility rules. It does not grant
radiation or explosion immunity.

## Collectible activation

Each chapter owns one cumulative activation set. New game activates Chapter 1.
Chapter completion activates the successor set. Activated sets never deactivate.

`FSharCollectiblePlacementAvailability` combines:

- canonical placement identity;
- collectible family and chapter set;
- activation state;
- collected or destroyed state;
- mission-only flag;
- world and structure availability; and
- mod-overlay revision.

Mission-only objective pickups require the active mission lease and never enter
persistent chapter activation.

## Card-set abilities

The 49 cards resolve into seven chapter sets. `USharCardSetAbilityDefinition`
contains:

- chapter set identity;
- exact seven-card membership;
- passive ability identity;
- affected character or system;
- bounded modifiers;
- stacking and conflict policy;
- mission restrictions;
- presentation; and
- balance-test identities.

One card never grants an ability independently. Completing a set commits one
exactly-once unlock. Mod overlays may replace extensible tuning but cannot change
base set membership or fabricate completion.

## World clock

`USharWorldClockSubsystem` advances one full cycle in 1,440 real seconds. One
in-game hour therefore lasts 60 real seconds.

The clock defines sunrise, day, sunset, and night intervals plus continuous solar,
lighting, sky, audio, population, and material parameters. It uses simulation time
and remains deterministic under pause, save, load, and fixed-step tests.

A mission declares one policy:

- use continuous world time;
- require an acceptance window;
- pause at accepted time;
- clamp to an authored phase;
- advance from checkpoint; or
- apply a temporary presentation override.

Returning to `non_mission` reconciles through the declared policy. A mission
cannot permanently corrupt the world clock.

## Sleep transactions

`USharRestLocationDefinition` declares location, safety requirements, price,
permitted target phases, resulting spawn, chapter availability, and mission
restrictions.

A sleep transaction verifies currency, danger, notoriety, mission state,
checkpoint safety, and target time. It applies the debit and time advance
atomically. Free homes and paid motels use the same contract.

## Map discovery and fog

`USharMapDiscoverySubsystem` owns stable regions, routes, landmarks, structures,
interiors, connectors, and viewpoints. Portable state stores discovered semantic
identities, not texture pixels.

The map renders undiscovered regions with stylized cloud fog. Mission markers use
a separate projection and remain visible through fog. A marker cannot reveal
hidden road geometry, collectibles, or shortcuts.

Discovery transactions may unlock help cards, route hints, terrain gates, and map
presentation. Terrain family 1 is initial; later terrain families require chapter
or discovery transactions.

## Connected terrain and shortcuts

Connectors are semantic world assets with endpoints, traversal class, availability,
chapter gate, discovery effect, mission restrictions, navigation support, and mod
extension points.

Burns' mansion uses a persistent connector originating inside the nuclear plant.
Its unlock transaction occurs only after fairness tests prove it cannot bypass
earlier terrain-family-1 missions. The exact generated geometry remains pending.

Known progression-breaking shortcuts are rejected by mission-route and world-gate
tests even when historical speedruns used them.

## Structure and interior capability

`USharStructureDefinition` contains:

- structure identity and exterior component set;
- terrain and district identity;
- interior capability: none, linked, streamed, mission-only, or future slot;
- interior identity when present;
- door and window component identities;
- breakable-entry policies;
- navigation and streaming dependencies;
- chapter and mission availability; and
- persistence and mod-extension policy.

Terrain meshes do not own structures. Structures do not infer interiors from
visual windows. Bart window breaking requires a valid breakable-entry definition
and available interior.

## Bart traversal and melee

Bart zip lines use `USharZipLineDefinition` assets with endpoints, direction,
entry conditions, motion profile, camera, animation, cancellation, failure
recovery, discovery effect, and mission restrictions.

Melee uses character-specific ability definitions with attack phases, collision,
damage or reaction policy, stamina interaction, AI response, mission restrictions,
and accessibility presentation. Original missions may ignore melee.

## Stamina and world-detail presentation

`USharStaminaSubsystem` owns sprint drain, recovery, exhaustion, character
modifiers, card-set passives, mission overrides, and save policy.

Footprints, dirt, wetness, dust, splashes, and vehicle or costume soiling use
physical-material and presentation adapters. Quality presets may reduce density
or lifetime but cannot change traversal or detection semantics.

## Economy and renewable sources

Currency definitions distinguish one-time and renewable source placements.
Renewable sources reset eligibility only at a new world session and use bounded
per-session transactions.

The economy model validates expected chapter income, mandatory and optional
costs, failure recovery, repair, permanent costumes, vehicles, paid sleep, taxi,
wagers, and convenience services. It proves a recoverable story path at every
chapter and coin balance.

Instant vehicle repair or recovery is a typed currency sink. It cannot repair a
mission-forbidden vehicle or bypass a vehicle-destruction failure condition.

## Taxi activity

The purchasable taxi unlocks `USharTaxiActivityDefinition` content. Each unique
job declares pickup, passenger, destination, time, route, vehicle condition,
chapter availability, dialogue, fare, tip, failure, and milestone identity.

Taxi work runs under `mission` gameplay state as a side activity. Unique
milestones are persistent and achievement-visible. Repeatable jobs provide
bounded renewable income. Taxi completion never gates story chapters.

## Boss and world-expansion runtime

`USharBossEncounterDefinition` contains boss slot, chapter, arena, creature,
phases, hazards, checkpoint graph, reward, permanent area unlock, and generic
asset replacement slots.

Two definitions are confirmed: Chapter 2 mechanical dinosaur and the
Apu-associated museum skeleton encounter. The third slot remains invalid until a
complete reviewed definition exists.

Boss completion commits a permanent world-expansion transaction. The stadium or
museum then becomes available in `non_mission`, map discovery, streaming, and
side-content projections.

## Chapter 7 runtime

Chapter 7 activates an irradiated weather and survival profile while retaining
the world clock. The profile drives cloud cover, humidity, haze, visibility,
lighting, audio, population, radiation, zombie hostility, and horror presentation.

`USharHealthSubsystem` becomes visible and authoritative for damage. Radiation
uses bounded exposure rules. Vehicle explosion lethality uses validated radius,
line-of-effect, and mission policy. Death restores the accepted checkpoint during
missions or a safe recovery point outside missions.

Zombie affiliation consumes character and costume tags. Devil Homer suppresses
ordinary zombie aggression only.

## Achievement runtime

Achievement implementation is pending, but the schema is authoritative.
`USharAchievementDefinition` contains:

- stable identity and platform mapping;
- base or mod owner;
- exact predicate and counters;
- chapter and content dependencies;
- no-missable reachability policy;
- replay and post-game availability;
- mod compatibility family;
- progress migration; and
- presentation.

Base definitions cover chapter completion, collectibles, current coin milestones,
side missions, taxi milestones, 100 percent completion, shortcuts, no-death
mission records, world expansions, purchases, and cumulative humorous actions.

No-death progress is tracked per mission and may be retried. No base achievement
uses an irreversible single opportunity.

Mods declare base-compatible, base-incompatible, or custom-achievement-provider
policy. Mod-owned achievements use namespaced identities and cannot impersonate
base platform mappings.

## Cel-shaded rendering

The rendering module uses project-owned cel-shading materials, post-process or
material-domain outlines, lighting profiles, and quality variants inspired by the
dimensional cartoon presentation of *The Simpsons Game*.

The implementation does not copy assets or proprietary shaders. It supports the
world clock, Chapter 7 atmosphere, dirt, wetness, damage, accessibility, and mod
material replacement. Automated screenshots verify readability of characters,
roads, mission markers, hazards, and interiors across all phases and presets.

## Speedrun integrity

Regression fixtures cover invalid campaign completion, checkpoint corruption,
out-of-bounds objectives, stale mission leases, duplicate rewards, time arithmetic,
shortcut gate bypass, and platform-dependent computation behavior.

Intentional movement and route skill remain supported. The runtime does not keep a
known progression defect merely because an existing speedrun category uses it.

## Mod-facing server adapter

The base runtime remains a local single-player campaign. It exposes a
transport-neutral server-adapter port for validated community packages. The port
uses canonical player, entity, world, action, event, package, and revision
identities; bounded authority snapshots; deterministic join and leave lifecycle;
and explicit serialization schemas.

The adapter does not provide a built-in multiplayer mode, matchmaking, browser,
hosting, moderation, anti-cheat, remote account service, or official server
persistence. A community server mod supplies those systems and owns its own
session, save, achievement, and compatibility namespaces.

Network authority may replace local session authority only after the server mod,
package set, protocol revision, world definition, and player permissions are
validated. Disconnect or mod removal returns through a governed transition and
cannot merge server state into a base campaign save implicitly.

## Failure behavior

The runtime fails closed on:

- any third player-facing gameplay state;
- more than one active mission lease;
- a test-level identity in campaign or save data;
- chapter unlocks out of order;
- deactivation of an earlier collectible set;
- missing checkpoint dependencies;
- invalid character eligibility;
- a mission marker that reveals hidden map geometry;
- an interior inferred without a structure contract;
- a shortcut that bypasses an accepted gate;
- duplicate renewable or persistent currency transactions;
- an unreachable base achievement;
- a mod with ambiguous achievement policy;
- an undefined third boss; or
- visual settings that alter gameplay meaning.

## Validation

Generation and cook checks prove:

- exactly seven base chapters and no Level 11 identity;
- one persistent world and two gameplay states;
- valid mission projections and checkpoint graphs;
- cumulative collectible activation;
- character unlock and temporary lock rules;
- complete 24-minute time and sleep definitions;
- map fog and mission-marker separation;
- connector, structure, interior, window, and zip-line dependencies;
- economy solvency and renewable-source bounds;
- taxi and boss definitions;
- Chapter 7 health, radiation, zombie, and checkpoint behavior;
- achievement reachability and mod policy;
- cel-shading assets and screenshot coverage; and
- absence of obsolete level-state or test-state definitions.

## Tests

Required tests mirror every scenario in the implementation-neutral
[open sandbox campaign design](../gameplay/open-sandbox-campaign-design.md), plus
Unreal lifecycle, streaming, Asset Manager, World Partition, Data Layer, save,
input, camera, audio, rendering, and mod-overlay integration tests.

## Invariants

- One connected world remains canonical.
- Chapters are story and unlock boundaries, not world states.
- `mission` and `non_mission` are the only gameplay states.
- Dynamic time is mandatory in the base game.
- Persistent chapter content activates cumulatively.
- Mission projection is temporary and transaction-owned.
- Every structure declares interior capability.
- Base achievements are non-missable.
- Generic boss and connector assets remain mod-replaceable.
- Historic level names never become player-facing runtime authority.

<!-- markdownlint-enable MD013 -->

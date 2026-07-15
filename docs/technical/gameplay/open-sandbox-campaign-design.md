# Open sandbox campaign design

- Status: Active
- Last reviewed: 2026-07-15

<!-- markdownlint-disable MD013 -->

## Governing decision

- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)

## Purpose

This specification defines the player-facing campaign, sandbox, chapter,
exploration, time, progression, character, economy, boss, achievement, traversal,
horror, and visual-design behavior independently from any engine implementation.

It preserves the original story-mission sequences while changing the surrounding
structure from isolated numbered levels into one connected, persistent sandbox.

## Core state model

The player is always in one of two gameplay states:

| State | Meaning |
| :--- | :--- |
| `non_mission` | Persistent free-roam sandbox with exploration, side activities, collectibles, purchases, character switching, and world progression. |
| `mission` | One accepted story mission, side mission, taxi job, race, or boss encounter with explicit objectives and checkpoint state. |

Menus, loading screens, cinematics, pause, failure presentation, and frontend
screens are presentation or application states. They do not create a third
player-facing gameplay state.

There is no test level and no campaign-visible development state. Development
uses isolated fixtures that cannot enter saves, completion, map discovery,
achievements, or the content catalog.

## Chapter model

The campaign has seven ordered narrative chapters. Historic numeric level
identities remain import aliases and source provenance only. User-facing text,
progression, achievements, menus, and transitions use chapter terminology.

A chapter definition contains:

- stable chapter identity and ordinal;
- ordered story missions;
- optional side missions, races, taxi milestones, and boss encounter;
- default and forced playable characters;
- terrain and route discovery unlocks;
- collectible activation set;
- character, costume, vehicle, interior, and shortcut unlocks;
- ambient population and presentation profile;
- Chapter 7 weather and survival profile where applicable;
- chapter completion presentation; and
- next-chapter transition.

Chapter completion occurs only after the final required story mission commits.
Optional content contributes to completion and achievements but does not block the
next chapter unless its own definition explicitly says otherwise.

## New game

A new game commits these initial conditions:

- world state is `non_mission`;
- Chapter 1 is the current narrative boundary;
- Homer is the controlled and only eligible unlocked character;
- terrain family 1 is traversable;
- Chapter 1 collectible, wasp, gag, and persistent side-content placements are
  activated;
- later chapter sets remain inactive;
- the map exists but undiscovered terrain is cloud-covered;
- the 24-minute world clock begins at the authored start time;
- one bounded renewable-coin source set is eligible for the session; and
- no mission-specific actors or objectives are spawned.

The first world presentation chooses a contextual Homer vignette from valid
locations and conditions. Examples include eating a donut, performing a gag,
idling at home, or socializing at Moe's Tavern. Repeated starts use weighted
variation and cooldowns so the world feels alive without becoming random
progression authority.

## Mission acceptance and projection

A mission definition may be visible before its terrain is discovered. Its map
marker remains visible through cloud fog and communicates direction without
revealing undiscovered roads or landmarks.

Accepting a mission validates:

- chapter and prerequisite completion;
- required character and vehicle;
- current time window;
- required terrain, interior, shortcut, and boss-gate state;
- mission bundle and checkpoint support;
- conflicting active activity; and
- save and failure recovery policy.

Only after acceptance does the game project mission-specific actors, objective
pickups, hazards, routes, vehicles, dialogue, and scripted changes. Returning to
`non_mission` removes transient mission projection and preserves only accepted
persistent results.

## Mission checkpoints and saves

Every story mission and boss encounter declares checkpoints. Long side missions
and taxi chains declare checkpoints when losing all progress would be unfair.

A checkpoint captures:

- mission and checkpoint identity;
- chapter and world-state revision;
- controlled character and mission-forced eligibility;
- vehicle identity, condition, and mission role;
- objective and timer state;
- health, stamina, and required inventory;
- world clock and mission time policy;
- persistent changes already committed; and
- deterministic retry position and orientation.

Manual save during a mission stores the latest accepted checkpoint, not an
arbitrary half-committed frame. Loading resumes from that checkpoint and cannot
duplicate coins, rewards, collectibles, unlocks, boss completion, or achievement
progress.

## Open-world map and discovery

The map uses stylized cloud fog over undiscovered terrain. Discovery is permanent
per save and occurs through traversal, landmarks, chapter unlocks, viewpoints,
missions, and declared exploration events.

Map presentation distinguishes:

- undiscovered terrain under opaque decorative clouds;
- partially discovered terrain with approximate routes or hints;
- discovered terrain with roads, landmarks, interiors, activities, and shortcuts;
- available mission markers visible regardless of discovery;
- unavailable markers with a clear prerequisite; and
- temporary mission routes shown only during the active mission.

Contextual help may explain map controls, nearby activities, new terrain, newly
opened interiors, or chapter changes. It does not solve hidden collectibles or
objectives automatically.

## Connected world and terrain gates

The world is physically connected through original bridge, road, tunnel, path,
stair, zip-line, and transition assets. Connectors are original or appropriately
licensed and remain replaceable by mods.

Terrain family 1 is the only ordinary free-roam region at new game. Terrain
families 2 and 3 unlock through chapter and discovery transactions. A locked
connector presents an understandable world condition rather than an invisible
collision wall whenever practical.

Unlocked geography remains available unless a mission applies a temporary,
explicit route restriction. Story progression cannot silently unload an earlier
terrain family or erase its collectibles.

## Structure and interior model

Every structure record contains an interior capability:

| Capability | Meaning |
| :--- | :--- |
| `none` | Decorative or exterior-only structure; no implied entrance. |
| `linked` | Persistent interior connected to the exterior. |
| `streamed` | Interior loaded through an explicit transition or streaming boundary. |
| `mission_only` | Interior exists only for declared mission projection. |
| `future_slot` | Stable extension point with no base interior yet. |

Structures, exterior components, interiors, windows, doors, and terrain are
separate semantic records. A structure may expose breakable windows only when an
actual available interior and valid entry route exist.

Burns' mansion is a persistent interior and world area. It unlocks through a
later traversal route originating inside the nuclear plant. The unlock condition
must occur after every earlier mission for which mansion access would create an
unfair terrain-family-1 shortcut.

## Character unlock and switching

Unlocked-character state and current eligibility are separate.

The menu shows every character from the beginning with a visible lock reason.
Outside missions, the player may switch instantly to any unlocked and eligible
character. Character switching restores a safe nearby placement, preserves world
state, and cannot bypass a terrain, mission, boss, or interior gate.

Completing the final Chapter 1 story mission unlocks Bart and presents a chapter
completion message. The next Bart mission may automatically select Bart and place
him at its accepted start location.

Bart is unavailable before that transition. After the final Chapter 2 story
mission, Bart becomes temporarily ineligible until the final Chapter 4 mission
commits because the story treats him as missing and then ill. Lisa missions force
Lisa. Every other mission may force its canonical protagonist.

## Costumes

Every costume appears in the menu from the beginning. Locked or unowned costumes
show price and prerequisites.

Purchasing a costume is permanent. An owned costume may be equipped from the menu
at any safe point without returning to a shop. Mission definitions may force,
forbid, or temporarily replace a costume only when the story or mechanics require
it.

The Devil Homer costume prevents ordinary zombie aggression. It does not prevent
radiation damage, boss damage, vehicle explosions, mission failure, or other
explicit hazards.

## Collectible activation

Chapter sets activate cumulatively:

1. Chapter 1 sets activate at new game;
1. completing Chapter 1 activates Chapter 2 sets;
1. completing Chapter 2 activates Chapter 3 sets;
1. the pattern continues through Chapter 7; and
1. activated uncollected content remains active permanently.

A player who completes every story mission before collecting optional content can
then find every chapter's activated collectibles in the connected world.

Persistent activation applies to cards, wasps, gags, and declared chapter-scoped
collectible families. Mission-objective pickups remain mission-only. Renewable
coins follow the economy policy rather than collectible persistence.

## Collector-card abilities

There are exactly 49 counted collector cards: seven chapter sets of seven.
Individual cards do not grant abilities.

Completing a chapter's seven-card set grants one passive ability. Seven abilities
are the maximum base set because each must remain visible, useful, testable, and
compatible with mission balance.

Initial ability roles are:

| Chapter set | Intended passive role |
| :--- | :--- |
| Chapter 1 | Reduced stamina drain or modest sprint-endurance improvement. |
| Chapter 2 | Better Bart slingshot stability, aim assistance, or recovery. |
| Chapter 3 | Improved collectible and point-of-interest awareness. |
| Chapter 4 | Better non-mission recovery, repair, or defensive resilience. |
| Chapter 5 | Modest economy benefit without invalidating progression costs. |
| Chapter 6 | Improved vehicle recovery or control under pressure. |
| Chapter 7 | Bounded radiation, zombie, or horror-survival assistance. |

Final values require simulation and mission regression tests. A passive cannot
skip objectives, satisfy achievements automatically, remove stamina entirely,
create infinite money, or make the player invulnerable.

## Time cycle

The world has four imperative phases:

- sunrise;
- day;
- sunset; and
- night.

A full cycle lasts exactly 24 real minutes. One real minute represents one
in-game hour. Phase boundaries and lighting curves are data-driven but preserve
that total duration.

The clock runs continuously in `non_mission`. A mission declares whether it uses
world time, pauses time, clamps to a phase, advances from a checkpoint, or applies
a temporary authored override. Returning to `non_mission` reconciles the world
clock through an explicit policy.

## Sleeping

Rest locations declare availability, price, permitted time jumps, spawn point,
and safety requirements.

Homes and story-owned safe locations may be free. Motels and convenience rest
locations may charge coins. The player selects an available target phase or
mission time window rather than waiting through unnecessary real time.

Sleeping cannot bypass mission completion, danger, wanted or notoriety state,
boss locks, terrain discovery, or required narrative events.

## Movement, stamina, and world detail

Running consumes a bounded stamina meter. Stamina recovers while walking,
standing, resting, or through declared abilities. Exhaustion limits sprinting but
does not trap the player or prevent required mission completion.

Surface response may produce footprints, dirt, mud, dust, wetness, splashes,
skid evidence, and costume or vehicle soiling. Presentation scales by platform
quality while gameplay collision and traversal remain unchanged.

## Melee combat

Every eligible playable character has a simple melee action set with readable
startup, active, recovery, hit reaction, crowd response, and mission policy.
Original missions need not depend on melee, but mission definitions may permit,
restrict, or ignore it.

Combat cannot permanently remove required characters, break story triggers, or
create irreversible civilian consequences.

## Bart traversal

Bart can use declared zip lines. A zip line has stable endpoints, direction,
eligibility, traversal state, cancellation, failure recovery, and map-discovery
behavior.

Bart can break eligible windows. A window is eligible only when:

- its structure has an available interior;
- the window declares a breakable entry route;
- mission and chapter policy permit entry;
- collision and navigation support the transition; and
- breaking it cannot bypass a required boss, shortcut, or progression gate.

Broken-window state follows the structure's reset and persistence policy.

## Economy

The coin economy has permanent and renewable sources. Renewable sources are a
bounded safety and replay mechanism, not an unlimited instant grant.

Each new world session resets only the renewable source set. One-time mission,
chapter, achievement, collectible, boss, purchase, and discovery transactions do
not reset.

The mathematical economy curve balances expected income, optional exploration,
required purchases, chapter progression, recovery costs, and reasonable player
error. Difficulty rises gradually without requiring repetitive grinding.

Primary sinks are:

- permanent high-value costumes;
- vehicles;
- instant vehicle repair or recovery;
- paid motel sleep;
- taxi and wager participation where a fee applies; and
- optional convenience services.

The game guarantees a recoverable path after bankruptcy. Required story progress
cannot depend on an exhausted one-time currency source.

## Taxi missions

Purchasing the taxi unlocks taxi work. The activity is a respectful nod to
classic open-world taxi missions and driving-focused Simpsons games, not a copied
mission script.

Taxi definitions may include:

- passenger identity or archetype;
- pickup and destination;
- time target;
- route and discovery requirements;
- vehicle-condition threshold;
- optional dialogue or humorous event;
- fare, tip, streak, and failure rules; and
- unique milestone identity.

Completing every unique base taxi milestone grants an achievement. Repeatable taxi
work remains available for fair income. No taxi activity is missable or required
for the main story.

## Boss encounters

The campaign reserves three boss slots. Two are confirmed:

1. a mechanical dinosaur encounter associated with the stadium near the end of
   Chapter 2; and
1. an Apu-associated Tyrannosaurus-skeleton encounter associated with the museum.

The third remains pending design approval. It has no invented location, chapter,
mechanics, reward, or asset requirement yet.

Boss creature models and presentation are original, repository-owned or
appropriately licensed generic assets. The mechanical dinosaur and skeleton may
be replaced through validated mods.

Completing the respective encounters permanently opens the stadium and museum.
Those locations remain explorable in `non_mission`, gain map entries, and may host
later side content.

## Chapter 7 horror and survival

Chapter 7 uses sustained environmental horror rather than cheap jump scares.
Humor, recognizable characters, and original mission identity remain intact.

The day-night clock continues. Radiation clouds prevent a clear daytime sky.
Daytime is humid, overcast, slightly brighter, and mildly hazy at long distance.
Night is darker and more threatening, with readable long-range silhouettes,
monsters, unsettling ambience, and stronger contrast.

The player has a visible health bar. Radiation applies rate-based damage through
volumes, surfaces, weather exposure, or declared hazards. Zombies can attack.
Nearby vehicle explosions are lethal when the player is inside a validated lethal
radius. Mission death restarts from the accepted checkpoint.

The Devil Homer costume changes zombie affiliation so ordinary zombies do not
initiate attacks. Bosses, scripted enemies, radiation, explosions, and explicit
mission hostility ignore that disguise unless their own definitions say
otherwise.

## Achievements

Achievements are pending implementation but are required product scope.

The base achievement set is intentionally approachable and has no missable
achievements. Every condition remains obtainable through free roam, mission
replay, side-activity replay, post-game play, or cumulative tracking.

The tone target is the joke: "The platinum trophy for Simpsons Hit & Run gave me
Simpsonphobia." Completion may be broad and funny, but never dependent on a
single irreversible opportunity.

Planned families include:

- complete each chapter;
- complete all chapters;
- collect card sets, wasps, costumes, and other families;
- own a declared current coin total;
- complete all side missions;
- complete all taxi milestones;
- reach 100 percent completion;
- discover and traverse every authored shortcut;
- complete each original story mission without dying, tracked per mission and
  retryable;
- unlock the museum, stadium, Burns' mansion, and other major areas;
- purchase the taxi;
- kick 100 pedestrians; and
- additional character-specific or Simpsons-themed cumulative challenges.

Achievement rules use stable identities, explicit counters, and exact completion
predicates. They do not infer success from UI screens or platform callbacks.

## Mod achievement policy

Every active mod declares:

- whether it preserves base-achievement eligibility;
- which base achievement families it affects;
- whether it adds namespaced mod achievements;
- required package and revision identity;
- migration and removal behavior; and
- whether saved progress remains valid when the mod is disabled.

A base-compatible mod cannot change an achievement's semantic difficulty while
claiming compatibility. A base-incompatible mod suspends only affected base
progress. Mod achievements remain separately identified and cannot masquerade as
base or platform achievements.

## Burns' mansion

Burns' mansion is accessible in the final sandbox. Its primary unlock route begins
inside the nuclear plant and becomes available through a later traversal
transaction.

The unlock is deliberately delayed so it cannot bypass early mission routes in
terrain family 1, especially chapters that reuse that geography. Once unlocked,
the route and mansion remain available outside missions unless a temporary
mission rule closes them.

The exact semi-authored, mathematically generated traversal geometry is pending a
later specification. Until then, the route identity, endpoints, gate, fairness
constraints, and persistent unlock are authoritative.

## Speedrun integrity

Progression-breaking bugs are fixed even when existing speedrun categories use
them. This includes near-instant campaign completion, invalid checkpoint writes,
out-of-bounds objective acceptance, stale mission state, duplicate rewards,
incorrect time arithmetic, and platform-dependent computation exploits.

Intentional movement techniques, route optimization, combat execution, vehicle
skill, resource planning, and sequence choices remain valid speedrunning. The
design favors genuine mastery over corrupt or accidental state.

## Visual direction

The visual baseline uses cel shading inspired by the dimensional cartoon look of
*The Simpsons Game*. All shaders, outlines, materials, textures, lighting models,
and authored assets are original to this project or appropriately licensed.

The rendering model supports:

- stable character and vehicle outlines;
- stepped or artist-shaped diffuse response;
- controlled specular response;
- readable sunrise, day, sunset, and night palettes;
- Chapter 7 cloud, humidity, haze, radiation, and horror lighting;
- dirt, footprints, wetness, damage, and surface details;
- Low through Ultra quality presets; and
- mod-replaceable materials and presentation profiles.

Cel shading cannot flatten navigation readability, hide hazards, make nighttime
missions unfair, or change gameplay state.

## Invariants

- There are exactly two player-facing gameplay states.
- There is no Level 11 or test level.
- The player exists in one connected sandbox, not inside a numbered level.
- Seven chapters preserve original story order and unlock content cumulatively.
- Mission-only content appears only for the active mission.
- Chapter collectible sets activate in order and remain active.
- All 49 cards are grouped into seven ability-granting sets.
- Base achievements are never missable.
- Purchased costumes remain owned and menu-equippable.
- Character eligibility respects story locks and mission forcing.
- The world clock completes one full cycle in 24 real minutes.
- Chapter 7 remains cloudy while retaining the underlying clock.
- Mods declare achievement compatibility and use stable replacement identities.
- The base campaign remains single-player while community server mods may define
  separate namespaced multiplayer modes.
- Progression-breaking speedrun defects are not preserved as features.

## Community server mods

The repository supplies architecture support, not a first-party multiplayer
product. A server mod may define players, session rules, authority, replication,
world snapshots, chat, discovery, persistence, moderation, and custom
achievements under its own package identities.

Base campaign saves, chapter progression, achievements, mission checkpoints,
and economy remain local unless a server mod imports them through a separate
explicit contract. Server state never silently writes into a base save. The
community package and operator own hosting, security, anti-cheat, moderation,
compatibility, and support. Adapter, protocol, package, authority, persistence,
and teardown behavior follow the
[multiplayer adapter and community-server extension](../modding/multiplayer-adapter-and-community-server-extension.md).

## Failure behavior

The design fails closed when:

- more than one mission or boss becomes active;
- a mission projects content before acceptance;
- a later chapter set activates early;
- an earlier activated set disappears;
- a save references an unaccepted checkpoint;
- character switching violates story eligibility;
- a shortcut bypasses a required gate;
- a structure advertises an interior it does not have;
- a window transition lacks an interior or navigation path;
- renewable currency duplicates a one-time transaction;
- an achievement becomes permanently unobtainable;
- a mod misdeclares base-achievement compatibility; or
- a visual or performance setting changes gameplay meaning.

## Verification

Required evidence includes:

- new-game and chapter-transition golden scenarios;
- mission and non-mission state exclusivity;
- checkpoint save and load at every story mission boundary;
- cumulative collectible activation through all seven chapters;
- Bart unlock, story lock, forced Lisa missions, and free-roam switching;
- cloud-map discovery and hidden-terrain mission markers;
- terrain connector and Burns' mansion shortcut fairness tests;
- 24-minute cycle timing and sleep transitions;
- Chapter 7 day, night, radiation, zombie, disguise, death, and checkpoint tests;
- seven card-set passive simulations and mission regressions;
- economy solvency, renewable-source, repair, costume, sleep, and taxi models;
- museum and stadium permanent-open transactions;
- no-missable achievement reachability proofs;
- mod achievement compatibility and custom-achievement tests;
- taxi side-mission completion and repeatability;
- structure interior capability and Bart window-entry tests;
- cel-shading readability and quality-preset screenshots; and
- speedrun exploit regression fixtures.

## Known limits

The third boss encounter, exact chapter-set passive values, final achievement
catalog, Burns' mansion generated traversal geometry, detailed taxi route census,
and final economy coefficients remain pending explicit design and balancing.
They cannot be guessed by implementation code.

<!-- markdownlint-enable MD013 -->

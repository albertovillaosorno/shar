# Open sandbox chapters and world progression

- Status: Accepted
- Decision date: 2026-07-15
- Scope: Campaign structure, open-world state, chapter progression, and player
  freedom

## Context

The product is not seven isolated gameplay levels and does not have a
development `level_11_test` state. It is one connected open sandbox with seven
narrative chapters. The original mission sequences remain the campaign backbone,
but the player exists in the world before, between, and after missions.

The previous model assigned protagonist, collectibles, lighting, and world state
to an active level. That conflicts with an open-world game where unlocked
characters, discovered terrain, purchased costumes, side activities, time of
day, and persistent collectibles continue outside missions.

The world must also remain understandable and fair. Story progression may unlock
new terrain, collectibles, characters, interiors, bosses, and side activities,
but it cannot silently replace the entire world or expose every chapter's
content at once.

## Decision

The base game has exactly two player-facing gameplay states:

- `non_mission`, the persistent open sandbox; and
- `mission`, one explicitly accepted mission or boss encounter.

There is no test level, test campaign state, or `level_11_test` identity.
Editor, automation, and development validation use ordinary test fixtures, maps,
data layers, and test worlds that never appear in campaign identity or
player-facing progression.

The campaign contains seven ordered narrative chapters. A chapter is a story
and unlock boundary, not a separate world or player state. Existing mission
sequences remain associated with their corresponding chapter.

The game has one connected geographic world. Terrain families are joined by
original, repository-owned or appropriately licensed connectors such as bridges,
roads, tunnels, paths, and transitions. Shared geography, structures, interiors,
coordinates, and placement identities remain canonical across the complete game.

## Non-mission sandbox

Starting a new game enters the persistent world as Homer in `non_mission`
state. The player is not described as being "in Level 1." Chapter 1 story
content is the current narrative boundary, while the player remains in the game
world.

The initial spawn uses a contextual ambient vignette. Homer may perform a gag,
eat a donut, idle at home, or appear at an appropriate social location such as
Moe's Tavern. Vignettes are non-authoritative presentation and cannot change
progression, inventory, health, or mission state.

Outside missions, the player may explore, collect unlocked content, purchase and
change costumes, switch among eligible unlocked characters, use unlocked
vehicles, enter available interiors, complete side activities, fight, and use
the map.

## Mission state

A mission appears in the world only after its definition is available and the
player accepts or enters that mission. Mission-specific actors, vehicles,
objectives, pickups, hazards, routes, and scripted world changes exist only for
the active mission unless their definition explicitly commits a persistent world
result.

Only one base-campaign mission or boss encounter is active at a time. Missions
may force a character, vehicle, start location, time window, weather override,
or restricted route. Leaving, failing, restarting, saving, or loading follows
the mission's checkpoint policy.

The game supports saving and loading during missions through deterministic
checkpoints. A save records the active mission, accepted checkpoint, chapter,
world state, character, vehicle, time, health, and required mission transaction
state. Loading cannot duplicate rewards or replay already accepted mutations.

## Chapter progression

Completing the final story mission of a chapter commits one chapter-completion
transaction. Presentation uses chapter language, for example:

> Congratulations. You completed the final mission of Chapter 1.
>
> Characters unlocked: Bart.

The next chapter's first mission may then appear. When required by story parity,
the transition automatically selects the next protagonist and places the player
at the next mission's accepted start location.

Completing Homer's final Chapter 1 mission unlocks Bart. Bart cannot be selected
before the player has completed a Bart mission entry transition.

After the final Chapter 2 mission, Bart becomes unavailable until the final
Chapter 4 mission is completed because he is missing during Chapter 3 and ill
during Chapter 4. Lisa missions force Lisa. Other missions may force their
canonical protagonist, while `non_mission` state permits any currently eligible
unlocked character.

## Terrain discovery and map

The map is available from the beginning. Undiscovered geography is covered by
stylized cloud fog. Discovering routes, districts, landmarks, interiors, and
connectors reveals them permanently and may surface contextual help.

Mission markers remain visible even when their destination lies beneath map fog.
The marker communicates direction and mission availability without revealing the
hidden terrain layout.

Chapter 1 permits ordinary play only in terrain family 1. Terrain families 2 and
3 become traversable through explicit discovery or chapter unlock transactions.
Physical connectors may exist before unlock but remain blocked by diegetic,
validated gates.

A discovered shortcut cannot bypass a mission boundary, forced route, boss gate,
or chapter restriction. Burns' mansion becomes permanently accessible through a
later traversal route from inside the nuclear plant. The route remains locked
until it cannot create unfair shortcuts for earlier missions that use terrain
family 1.

## Collectible activation

Collectibles, wasps, gags, and other chapter-scoped world content do not all
appear at once.

At new-game commit, every Chapter 1 collectible placement is active. Completing
the final Chapter 1 story mission activates the Chapter 2 collectible set.
Completing each later chapter activates the next chapter's set. Therefore, a
player who finishes all story missions without collecting anything eventually
has all Chapter 1 through Chapter 7 collectible sets active simultaneously.

Activation is independent from collection. Previously activated and uncollected
content remains present after later chapters unlock. Mission-only pickups remain
scoped to their mission and are not part of this persistent activation rule.

All collectible categories and every costume are visible in the menu from the
beginning. Locked entries show their requirement. Purchased costumes are
permanent and may be equipped from the menu without visiting a shop.

There are 49 collector cards: seven cards in each of seven chapter sets.
Completing one full chapter set unlocks one balanced passive ability. The game
does not create one ability per card.

The seven passives must be meaningful but bounded. Candidate families include
reduced stamina drain, improved slingshot handling, increased environmental
awareness, better purchase terms, improved vehicle recovery, stronger defensive
resistance, and a Chapter 7 survival benefit. Exact values remain tuning data
and cannot invalidate missions, achievements, or speedrun integrity.

## Time, sleep, and mission availability

The world always runs an imperative cycle:

1. sunrise;
1. day;
1. sunset; and
1. night.

One complete cycle lasts 24 real minutes, so one real minute equals one in-game
hour. The clock continues in `non_mission` state and follows explicit mission
pause or override policy in `mission` state.

Some missions require a specific time window. Their markers may remain visible,
but acceptance communicates the required time. The player may wait naturally or
sleep at an eligible home, motel, or other declared rest location.

Free rest locations may advance time according to their definition. Commercial
rest locations charge a fair coin fee. Sleeping never skips required mission
state, chapter completion, boss gates, or scripted consequences.

## Chapter 7 atmosphere and survival

Chapter 7 preserves the day-night clock but applies a permanent irradiated cloud
and weather profile. There is no clear daytime sky.

During daytime, the world is humid, cloudy, slightly brighter, and mildly hazy
at long distance. Visibility remains playable. At night, long-distance
visibility may improve while lighting, silhouettes, monsters, sound, and
ambience create sustained horror. Cheap jump scares are not the primary
technique, and the design does not abandon the game's comedic identity.

Chapter 7 introduces a visible health bar. Radiation volumes and contaminated
surfaces apply bounded health damage. A nearby vehicle explosion may cause
immediate death when its lethal-radius rules are satisfied. Mission death
returns to the most recent accepted checkpoint.

Zombies may attack the player. The canonical Devil Homer costume suppresses
zombie hostility according to its declared disguise rule; it does not grant
universal invulnerability or disable radiation.

## Character movement and interaction

The game includes melee combat even where original missions do not require it.
Combat remains simple, readable, and compatible with the existing tone.

Running consumes stamina. Exhaustion reduces sprint capability and recovers over
time. Footprints, dirt, wetness, and related surface details make traversal feel
alive but remain scalable presentation.

Bart has a zip-line traversal ability. Bart may also break windows only on
structures whose catalog definition declares a real, available interior and a
breakable-window entry contract. Decorative windows on structures without
interiors cannot become false entrances.

Every structure definition declares whether it has no interior, a linked
interior, a streamed interior, a mission-only interior, or a future interior
slot. Structures and interiors remain separate from terrain assets.

## Economy

A bounded set of renewable coin sources resets when a world session begins.
Renewable sources are explicitly identified and cannot include one-time mission,
collectible, achievement, or chapter rewards.

Costs and income follow a mathematical progression curve that becomes gradually
more demanding without requiring excessive grinding. The economy provides
recurring sinks:

- permanent costume purchases;
- vehicle purchases;
- instant vehicle repair or recovery after destruction;
- paid sleep at commercial rest locations;
- taxi and wager participation where applicable; and
- optional convenience services that never sell required story completion.

The system protects a recoverable minimum economy so the player cannot become
permanently unable to continue.

## Taxi side missions

The taxi is purchasable. Owning it unlocks a taxi side-mission family as a nod
to classic open-world taxi gameplay and driving-focused Simpsons games.

Taxi work consists of bounded passenger pickups, destinations, timing, route,
vehicle-condition, and optional bonus constraints. Unique service milestones
produce progression and an achievement; repeatable work produces balanced coin
income. Taxi missions never become required for story completion.

## Boss encounters and world expansion

The product target contains three boss slots, but only two encounters are
currently confirmed. The third remains explicitly pending and must not be
invented merely to fill the count.

The confirmed encounters are:

- a Chapter 2 mechanical dinosaur encounter associated with the stadium; and
- an Apu-associated dinosaur-skeleton encounter associated with the museum.

Their creature assets use original, repository-owned or appropriately licensed
generic designs, such as a mechanical dinosaur and a Tyrannosaurus skeleton.
They cannot copy protected third-party models.

Completing each encounter permanently opens its associated stadium or museum and
adds the location to ordinary sandbox exploration. Boss completion cannot be
required again after the persistent unlock is accepted.

## Achievements

The achievement feature is required but implementation remains pending.
Completion is intentionally approachable, and the base campaign contains no
missable achievements. A player can return, replay, or use a post-game state to
satisfy every base requirement.

The intended tone is summarized by the joke: "The platinum trophy for
Simpsons Hit & Run gave me Simpsonphobia." Difficulty comes from breadth and
affection for the game, not irreversible traps.

Planned base categories include:

- chapter completion;
- collector-card sets, wasps, costumes, and other collectible families;
- current total coin milestones;
- all side missions and all taxi milestones;
- 100 percent completion;
- discovering and using every authored shortcut;
- completing every original story mission without dying, tracked per mission so
  failed attempts can be retried;
- opening the museum, stadium, Burns' mansion, and other major world expansions;
- purchasing and using the taxi;
- cumulative humorous actions such as kicking 100 pedestrians; and
- other characterful Simpsons-themed challenges.

Mods declare one achievement policy:

- base-compatible, preserving eligibility for declared base achievements;
- base-incompatible, suspending affected base achievement progress while active;
  or
- custom-achievement provider, registering namespaced mod-owned achievements.

Removing a mod cannot fabricate base achievement progress. Mod achievements
remain associated with their package identity and revision.

## Community multiplayer extension boundary

The base campaign is single-player and does not ship a multiplayer campaign,
matchmaking, server browser, hosted service, moderation service, or official
server network.

The architecture exposes stable mod-facing server adapters. A validated
community package may define its own multiplayer mode and independently operated
server using canonical identities, authority snapshots, transport-neutral
messages, join and leave lifecycle, package compatibility, and namespaced
persistence.

A community server mod owns networking transport, hosting, discovery, rules,
moderation, anti-cheat, security, persistence, compatibility, uptime, and
support. It cannot reinterpret a base save, fabricate base achievements, or
claim that its server mode is the first-party campaign. The complete boundary is
owned by
<!-- markdownlint-disable-next-line MD013 -->
[mod-owned multiplayer adapters and community servers](../modding/mod-owned-multiplayer-adapters-and-community-servers.md).

## Visual direction

The game uses cel-shaded rendering inspired by the dimensional cartoon
presentation of *The Simpsons Game*. The project implements original shaders,
materials, outlines, lighting, textures, and models; it does not copy that
game's assets or proprietary implementation.

Cel shading remains compatible with sunrise, day, sunset, night, irradiated
Chapter 7 weather, accessibility, performance presets, dirt, footprints, and
modded material replacements.

## Speedrun integrity

Known defects that permit impossible or near-instant campaign completion,
progression skips, invalid checkpoint commits, out-of-bounds objective
completion, or computation-dependent exploits are bugs and must be fixed even
when an existing speedrun route depends on them.

The product still supports legitimate speedrunning through movement skill, route
planning, vehicle control, resource management, mission execution, and other
intentional mechanics. Validation distinguishes creative play from corrupted
state or accidental computation behavior.

## Consequences

- The player is always in one world, not inside a numbered gameplay level.
- Chapters order story and unlocks without replacing sandbox state.
- Mission and non-mission are the only player-facing gameplay states.
- No test level or campaign-external test state exists.
- Dynamic time, exploration, terrain discovery, and map fog apply to the base
  game from the beginning.
- Original mission sequences remain intact while presentation, traversal,
  persistence, and side activities expand around them.
- New world connectors, boss assets, and generic fallbacks use original or
  appropriately licensed content and remain mod-replaceable.
- Achievement, mod, save, checkpoint, and speedrun policies become explicit and
  testable.

## Rejected alternatives

- Seven isolated player-facing levels.
- A hidden or visible `level_11_test` development state.
- Activating every chapter's collectibles and world content at new game.
- Fixed campaign lighting with a dynamic cycle reserved for testing.
- One ability for each of 49 cards.
- Missable base achievements.
- Burns' mansion access that bypasses early mission routes.
- Boss encounters using copied third-party creature models.
- Cel shading implemented through copied assets or proprietary shaders.
- Preserving progression-breaking bugs solely for historical speedrun routes.

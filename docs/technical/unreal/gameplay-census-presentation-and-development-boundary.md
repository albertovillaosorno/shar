# Gameplay census, presentation, and development-content boundary

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Contextual interaction query and transaction boundary](../../adr/unreal/runtime/contextual-interaction-query-and-transaction.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native flying-hazard actors and StateTree execution](../../adr/unreal/runtime/native-flying-hazard-actors-and-state-trees.md)

## Purpose

This specification closes the boundary between canonical shipping gameplay,
role-specific world variants, presentation assets, and development evidence. A
name found in source material does not automatically create a player-owned
asset,
a mission, or a shipping feature. Every record must resolve to one canonical
identity and one explicit availability class before import or runtime use.

Detailed execution remains in the owning specifications:

<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapter runtime](open-sandbox-chapter-runtime.md)
- [Gameplay content catalog](gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission, interaction, interior, and notoriety runtime](mission-interaction-and-notoriety-runtime.md)
- [Race route and opponent runtime](race-route-and-opponent-runtime.md)
- [Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md)
- [Contextual interaction runtime](contextual-interaction-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Flying-hazard and projectile runtime](flying-hazard-and-projectile-runtime.md)

## Canonicalization

Import normalizes evidence into stable identities before creating Unreal assets.
Case differences, punctuation differences, alternate display titles, and
duplicate
index entries never create additional gameplay definitions.

Each census record contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ContentId` | Stable canonical identity used by generated data and save rows. |
| `DisplayNameKey` | Localized presentation identity; never the canonical key. |
| `Aliases` | Reviewed alternate names that resolve only to `ContentId`. |
| `ContentFamily` | Mission, race, vehicle, character, location, collectible, presentation, or development evidence. |
| `Availability` | Shipping state from the closed enum below. |
| `LevelScope` | Ordered base levels or explicit global scope. |
| `RuntimeRoles` | Concrete roles such as ownership reward, traffic, mission target, or vendor. |
| `OwningDefinition` | Primary asset or generated row that owns runtime behavior. |
| `VerificationIds` | Golden tests and import checks required before activation. |

<!-- markdownlint-enable MD013 -->

The availability enum is:

<!-- markdownlint-disable MD013 -->

| Value | Meaning |
| :--- | :--- |
| `shipping` | Normal campaign or front-end content. |
| `mission_only` | Spawned or selectable only by declared mission state. |
| `traffic_only` | Ambient traffic access; never persistent ownership by implication. |
| `development_only` | Retained for diagnostics or parity research; absent from normal shipping queries. |
| `diagnostic_only` | Metadata or review evidence with no runtime asset. |
| `excluded` | Rejected evidence that cannot enter generated or runtime catalogs. |

<!-- markdownlint-enable MD013 -->

A cheat or mod overlay may query a broader catalog, but it never changes the
base availability value or silently grants ownership.

## Campaign identity

The runtime represents one connected sandbox with seven ordered narrative
chapters and exactly two player-facing gameplay states. General game summaries
and navigation indexes do not create aggregate `Game`, `Wiki`, or `AllContent`
assets. They resolve to campaign documentation and diagnostic coverage records.

The campaign story remains the ordered investigation of surveillance, cola,
alien control, and the final Chapter 7 response. Final shipping behavior is the
authority. Verified Unused Content ships through its dedicated surface;
prerelease, prototype, and rejected variants without an accepted identity remain
under the development-evidence contract below.

## Verified mission slice

<!-- markdownlint-disable MD013 -->

| Mission identity | Source chapter and class | Required contract |
| :--- | :--- | :--- |
| `the_cola_caper` | Level 1 tutorial | Drive to the Kwik-E-Mart, enter the interior, talk to Apu, collect the declared groceries, and unlock the main mission sequence. The tutorial is non-failing and not replayable in the same save after completion. |
| `the_fat_and_furious` | Chapter 1 story finale; source Level 1 | Start at the power plant, race Smithers to the manor, complete the Burns interaction, commit Chapter 1 completion, unlock Bart, activate Chapter 2 collectibles, and expose the next Bart mission. The title alias without the second article resolves to this identity. |
| `this_old_shanty` | Level 1 bonus | Complete Cletus's ordered collection chores and grant the declared vehicle reward once. |
| `weapons_of_mass_delinquency` | Level 2 story mission 2 | Collect the ordered fireworks from named contacts, then escape the police pursuit without losing the mission vehicle state. Capitalization variants resolve to one identity. |
| `vox_nerduli` | Level 2 story mission 3 | Race the declared opponent to the Java Server destination and publish one race result. |
| `the_old_pirate_and_the_sea` | Level 3 story finale | Destroy the black sedan target, complete Bart's rescue sequence, and unlock the Level 4 transition. |
| `wolves_stole_my_pills` | Level 4 story mission 5 | Reach Nelson, follow the sedan, collect ten medication payloads, lose the pursuer, return to Grampa, obtain the caffeine target, and complete the final crop-circle dialogue. |
| `the_cola_wars` | Level 4 story mission 6 | Require the police costume at activation, collect the authored cola-can set across the suburban route, and complete the Apu follow-up. The collection is primarily on foot. |
| `and_baby_makes_8` | Level 5 story mission 2 | Escape the mafia pursuit and complete the declared destination sequence without promoting the pursuer to an owned vehicle. |
| `this_little_piggy` | Level 5 story mission 4 | Require the American costume at activation, collect the donut trail, follow Wiggum, and complete the DMV destination sequence. |
| `theres_something_about_monty` | Level 7 story mission 4 | Reach the power plant while avoiding the alien probe, then complete the authored vertical ascent to the terminal interaction. |

<!-- markdownlint-enable MD013 -->

Mission display titles, transcript text, music cues, costume gates, rewards, and
ordered objective rows remain separate generated records. A summary page never
replaces those records.

## Verified race slice

The race runtime has three street-race classes and one wager class:

- time trial: finish the authored lap count within the limit;
- circuit race: finish first after the authored lap count;
- checkpoint race: traverse ordered checkpoints and finish first; and
- wager race: pay the quoted entry fee, complete the timed route, and settle the
  reward transaction from the same quote revision.

Levels 1 through 6 use Milhouse, Nelson, and Ralph for time trial, circuit, and
checkpoint hosting. Level 7 uses zombie hosts for all three classes. Completing
all three street races in a level grants that level's declared vehicle reward
and
contributes to level completion. Street races are optional for story
progression.

<!-- markdownlint-disable MD013 -->

| Race identity | Required route contract |
| :--- | :--- |
| `time_trial_level_01` | Level 1 time trial hosted near the trailer park; complete five laps within the authored time limit. |
| `town_square_circuit_level_02` | Level 2 circuit hosted near the town-square Krusty Burger; complete four laps and finish first. |
| `suburban_rich_checkpoint_level_04` | Start near the Evergreen Terrace stone sign, traverse the rich district and tunnel route, and finish at the power-plant bridge. |
| `suburban_countryside_2_checkpoint_level_07` | Start at the school, traverse the Halloween suburban route, and finish at the power-plant parking lot against the Hearse, Ghost Ship, and Coffin Cart. |

<!-- markdownlint-enable MD013 -->

Every base level has one wager race hosted through the declared mob interaction.
The entry fee is charged once at acceptance. Resetting or abandoning the race
does not refund it. Leaving the vehicle starts the standard ten-second recovery
window; entering a different vehicle fails the wager. Wager completion does not
count as a street-race win or story mission.

The runtime does not reproduce collision-loss, blocked-shortcut, or AI recovery
bugs. Route boundaries and shortcuts are authored explicitly and validated by
race-route tests.

## Verified vehicle slice

<!-- markdownlint-disable MD013 -->

| Vehicle identity | Availability and role | Required contract |
| :--- | :--- | :--- |
| `station_wagon` | `Unused Content` | Ships through the dedicated Unused Content surface with generic fallback presentation where required and stable mod-replacement slots. Campaign ownership remains opt-in. |
| `surveillance_van` | `shipping`, Chapter 1 purchase | Gil offer for 100 coins; ownership grants persistent sandbox retrieval access. Mission and cinematic placements remain separate roles. |
| `taxi` | `shipping`, purchasable after its chapter prerequisite | Ordinary traffic use never grants ownership. Purchasing the taxi grants permanent sandbox access and unlocks taxi side missions, repeatable fares, unique milestones, and the all-taxi achievement path. |
| `tractor` | `shipping`, Level 4 purchase | Willie offer for 400 coins; ownership grants cross-level phone-booth access. |
| `wwii_vehicle` | `shipping`, Level 2 bonus reward | Grant once from the declared bonus mission and expose through phone booths after ownership. |
| `wwii_vehicle_rocket` | `mission_only`, Level 7 | Distinct loadout variant required by the final delivery mission; never collapse into the ordinary WWII vehicle definition. |
| `zombie_car` | `shipping`, Level 7 purchase | Zombie-vendor offer for 500 coins; ownership grants cross-level phone-booth access. |
| `vote_quimby_truck` | `traffic_only`, Level 5 | Ambient truck with its declared campaign livery and horn event; no ownership implication. |
| `witch_broom` | `traffic_only`, Level 7 | Small traffic vehicle with passenger seating, cackle horn event, no wheel-skid presentation, and no ordinary ownership. |

<!-- markdownlint-enable MD013 -->

The persistent base roster contains exactly 42 owned vehicles. Ownership may
come
from a level start, street-race reward, bonus-mission reward, or purchase.
Mission
use, forced use, opponent use, traffic use, prop use, and driver presentation
are
orthogonal roles and never grant ownership.

The unusable-vehicle census is one diagnostic set, not two gameplay families.
Its entries remain `mission_only` or `development_only` unless another verified
contract grants a narrower shipping role. Completion-override or mod access is a
query policy over the same definition, never a rewrite of availability.

## Verified character and location slice

<!-- markdownlint-disable MD013 -->

| Identity | Required roles |
| :--- | :--- |
| `waylon_smithers` | Mission opponent, driver, bonus-mission participant, ambient placement, and cinematic placement under one character identity. |
| `chief_wiggum` | Mission contact, pursuer, driver presentation, ambient placement, doorbell voice event, and Level 7 world placement. |
| `groundskeeper_willie` | School-area ambient placement, gag presentation, and Level 4 tractor vendor. |
| `zombie_ambient` | Level 7 weighted ambient archetypes, street-race hosts, vendor presentation, and mission contacts; no named-character save identity. |

<!-- markdownlint-enable MD013 -->

The first suburban world includes separate canonical locations for the Simpsons
house, Flanders house, Wiggum house, and the Gold House. The third-world
location
slice includes Android's Dungeon, Wall E. Weasel's, Planet Hype, and their
mission, gag, card, and route roles. Display groupings do not merge these world
locations or their streaming identities.

## Talk objectives

A talk objective references a named interaction source and one required dialogue
result. It commonly activates a mission, but activation and completion are
separate objective policies. The interaction runtime reserves the speaker,
positions the player when required, completes dialogue, and publishes one typed
mission observation. Transport completion or prompt display is not objective
completion.

## Wasp cameras

Each base level contains exactly 20 wasp-camera targets, for 140 campaign
targets.
Each target has stable level membership, placement identity, AI profile, damage
state, currency reward, and level-progress row.

The authored difficulty progression is:

| Level range | Projectile and defense contract |
| :--- | :--- |
| Levels 1-3 | One projectile per attack and no shield phase. |
| Levels 4-6 | Three-projectile volley. |
| Levels 5-7 | One shield phase before the body can be destroyed. |
| Level 7 | Five-projectile volley and the most evasive movement profile. |

AI state is explicit: passive, neutral, or hostile. Nearby collector-card or
repair-pickup collection publishes an alert stimulus that may transition an
eligible camera to hostile. A projectile hit removes five coins through the
economy port. Destruction grants the configured coin reward and progress exactly
once.

Vehicle impacts, kicks, and declared character attacks use the same damage port.
The runtime must not reproduce any collision count after which cameras become
non-collidable; collision and damage remain valid until authoritative
destruction.

## Repair pickups

A repair pickup restores the current player vehicle. When the player is on foot,
it restores the last valid player-controlled vehicle retained for the current
level. It repairs driveability and all supported visible damage channels.

Repair pickups are collectable on foot or in a vehicle and respawn after the
approximately one-minute authored cooldown. Collection near a wasp camera emits
the standard alert stimulus. Respawn timing and vehicle selection follow
[Contextual interaction runtime](contextual-interaction-runtime.md).

## Typeface and text-presentation contract

Dynamic text uses role definitions rather than font-family names in gameplay
code. `FSharTypefaceRoleDefinition` contains:

- `RoleId`;
- primary font asset and typeface face;
- glyph-coverage requirement;
- size, letter spacing, line height, outline, and shadow policy;
- ordered fallback assets;
- platform overrides; and
- metric and screenshot golden-test identities.

Required dynamic roles are:

| Role | Use |
| :--- | :--- |
| `brand_logo` | Main title treatment and approved logo variants. |
| `license_body` | Startup legal and ownership text. |
| `loading_headline` | Newspaper-style level headline. |
| `loading_body` | Newspaper-style supporting copy. |
| `ui_display` | Menus, large HUD notices, and selection labels. |
| `ui_body` | Small HUD, settings, subtitles, and supporting text. |
| `bonus_counter` | Bonus-game numeric presentation. |
| `credits` | Ordered credits sequences. |

Text already rasterized into a converted texture remains part of that texture
and
does not create a runtime font dependency. Dynamic font assets must arrive
through the package index with glyph coverage and metrics. Import never guesses
a font file from a family name. Missing required roles fail cooking.

Source font-build scripts and generated byte-array headers are conversion
provenance only. Native packages use reviewed Unreal font, composite-font,
atlas, and material assets; runtime code never compiles an opaque source font
package into a C++ string literal.

## Unused Content

Verified unused content is required shipping content. Every accepted unused
identity appears in the canonical catalog and is reachable through a dedicated
`unused_content` frontend and gameplay surface unless a reviewed integration
places it directly in the campaign.

An unused-content definition declares:

- canonical content identity and source-evidence revision;
- gameplay class and required native definition;
- presentation, audio, animation, physics, and interaction dependencies;
- dedicated gallery, sandbox, selection, or world-placement availability;
- campaign integration state;
- save and progression isolation;
- fallback policy for every missing dependency; and
- fields and extension points available to validated mod overlays.

Missing source presentation never removes the accepted identity. The base game
uses clearly generic, repository-owned or appropriately licensed meshes,
materials, textures, audio, animation, icons, text, and tuning where required.
Generic fallbacks must be visibly classified as replacements and cannot claim to
reconstruct or reproduce unavailable protected material.

Every fallback is addressed by the same stable semantic identity used by the
final content. A validated mod may replace presentation, audio, animation,
tuning, placement, user-interface, or other declared extensible fields without
changing the canonical identity or save meaning. Removing a fallback therefore
does not require executable patching or loose-file path collisions.

Unused content does not silently grant campaign rewards, completion, purchases,
or achievements. Its dedicated surface may record only typed discovery,
selection, and local presentation state unless a separate reviewed campaign
integration explicitly declares progression behavior.

Cook and automation validation prove that every accepted unused identity is
reachable, has complete native gameplay dependencies, resolves every required
fallback, and survives activation and removal of a replacement mod overlay.

## Development evidence

Prerelease changes, prototype differences, discarded variants without an
accepted content identity, obsolete text, and research indexes do not override
final shipping behavior. They normalize into `FSharDevelopmentContentRecord`
with:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `EvidenceId` | Stable diagnostic identity. |
| `RelatedContentIds` | Existing canonical identities affected by the evidence. |
| `DifferenceClass` | Presentation, mission, vehicle, world, audio, text, bug, or unknown. |
| `ObservedDifference` | Public domain-level summary without implementation text. |
| `ShippingDecision` | Keep final behavior, restore verified behavior, diagnostic only, or reject. |
| `VerificationIds` | Tests proving the decision cannot silently drift. |

<!-- markdownlint-enable MD013 -->

Development records are editor and test data only. They are excluded from
shipping asset bundles, campaign queries, save schemas, rewards, traffic, and
mission activation. A reviewed mod may deliberately project one through the mod
overlay contract, but the base catalog remains unchanged.

Navigation indexes and duplicate overview pages create diagnostic coverage rows
only. They never create aggregate runtime assets or duplicate campaign data.

## Alias normalization

The following aliases resolve without creating additional definitions:

| Alias | Canonical identity |
| :--- | :--- |
| `The Fat and Furious` | `the_fat_and_furious` |
| `The Fat and the Furious` | `the_fat_and_furious` |
| `Weapons Of Mass Delinquency` | `weapons_of_mass_delinquency` |
| `Weapons of Mass Delinquency` | `weapons_of_mass_delinquency` |
| `Unusable Cars` | `unusable_vehicle_census` |
| `Unusable Vehicles` | `unusable_vehicle_census` |
| `Witch` | `witch_broom` |
| `Wiggum` | `chief_wiggum` |
| `Willie` | `groundskeeper_willie` |
| `Zombies` | `zombie_ambient` |

## Invariants

- One physical or logical content unit maps to one canonical identity.
- Duplicate titles and case variants never duplicate save or runtime rows.
- Availability is explicit and never inferred from file presence.
- Traffic, mission, reward, ownership, and driver roles remain independent.
- Development evidence cannot enter shipping bundles by default.
- Every level has exactly three street races, one wager race, and 20 wasp
  cameras.
- The persistent vehicle roster contains exactly 42 identities.
- Dynamic text resolves through a declared typeface role.

## Verification

Generation and automation must prove:

- duplicate aliases collapse to the declared canonical identities;
- all mission and race rows reference existing level definitions;
- all vehicle roles preserve their declared availability and ownership boundary;
- the 42-vehicle persistent roster has no duplicates or development-only rows;
- each level has three street races, one wager race, and 20 distinct wasp rows;
- all 140 wasp identities are unique and reward progress once;
- repair pickup cooldown and last-vehicle targeting are deterministic;
- every dynamic text widget resolves one complete typeface role;
- rasterized texture text creates no implicit runtime font dependency;
- development records are absent from shipping asset bundles; and
- repeated generation preserves identities, membership, ordering, and counts.

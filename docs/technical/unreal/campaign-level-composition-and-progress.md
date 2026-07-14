# Campaign level composition and progress

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

- [Canonical seven-level campaign and world variants](../../adr/unreal/runtime/canonical-seven-level-campaign-and-world-variants.md)
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
- [Native world partition and data layers](../../adr/pipeline/unreal/world-partition-and-data-layer-import.md)
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)

## Purpose

This specification fixes the campaign, level, world-family, mission-sequence,
progress, transition, and load contracts for the seven-level game.

## Native Unreal composition

The campaign uses native Unreal facilities behind repository-owned contracts:

- Asset Manager primary assets identify and load campaign and level definitions;
- World Partition streams spatial cells inside each base world;
- Runtime Data Layers compose each level variant;
- streaming sources pin destinations and active mission actors; and
- a `UGameInstanceSubsystem` coordinates campaign-level lifetime and travel.

Engine assets execute the contract. They do not infer or own campaign identity.

## Campaign definition

`USharCampaignDefinition` is a non-Blueprint primary data asset with:

| Field | Contract |
| :--- | :--- |
| `CampaignId` | Stable identity `base_campaign`. |
| `OrderedLevelIds` | Exactly `level_01` through `level_07`. |
| `InitialLevelId` | `level_01`. |
| `TutorialMissionId` | Optional onboarding mission before Level 1 story progress. |
| `CompletionRuleId` | Exact overall-completion policy. |
| `MovieRewardId` | All-card movie reward contributing one percent. |
| `EndingMissionId` | Final story mission that commits campaign completion. |
| `CreditsSequenceId` | Post-ending credits presentation. |
| `RevisionToken` | Deterministic generated-data revision. |

The ordered level list is dense, unique, and immutable for the base campaign.
A mod may add a separate campaign definition but cannot insert rows into the
base sequence or reuse its save keys.

## Level definition

`USharLevelDefinition` is a non-Blueprint primary data asset with:

| Field | Contract |
| :--- | :--- |
| `LevelId` | Stable identity from `level_01` through `level_07`. |
| `SequenceOrdinal` | Dense ordinal from one through seven. |
| `NarrativeDate` | Month and day in the campaign timeline. |
| `PlayableCharacterId` | Canonical protagonist identity. |
| `PreviousLevelId` | Optional predecessor. |
| `NextLevelId` | Optional successor. |
| `BaseWorldFamilyId` | One of the three canonical world families. |
| `WorldAsset` | Soft reference to the owning World Partition world. |
| `RuntimeLayerSetId` | Exact level-variant Data Layer composition. |
| `StartingVehicleId` | Canonical starting vehicle. |
| `StoryMissionIds` | Exactly seven ordered story missions. |
| `BonusMissionId` | Exactly one level bonus mission identity. |
| `StreetRaceIds` | Exactly three ordered street races. |
| `StreetRaceRewardId` | One reward granted after the race set. |
| `WagerRacePolicyId` | One repeatable economy challenge. |
| `CollectorSetId` | One seven-card set. |
| `CostumeOfferIds` | Exactly three counted costume offers. |
| `ProgressionVehicleIds` | Exactly five counted vehicle unlocks. |
| `WaspPlacementIds` | Exactly twenty counted placements. |
| `GagPlacementIds` | Exact level-specific counted placements. |
| `AudioProfileId` | Level music, ambience, dialogue, and event policy. |
| `CompletionTransition` | Successor level or campaign ending. |

Starting vehicles and secret vehicles remain cataloged and playable but are not
members of `ProgressionVehicleIds`.

## Canonical level census

| Level | Date | Protagonist | Base world | Gags |
| :--- | :--- | :--- | :--- | ---: |
| `level_01` | October 25 | `homer_simpson` | `suburban` | 15 |
| `level_02` | October 26 | `bart_simpson` | `downtown` | 11 |
| `level_03` | October 27 | `lisa_simpson` | `harbor` | 11 |
| `level_04` | October 28 | `marge_simpson` | `suburban` | 15 |
| `level_05` | October 29 | `apu_nahasapeemapetilon` | `downtown` | 6 |
| `level_06` | October 30 | `bart_simpson` | `harbor` | 11 |
| `level_07` | October 31 | `homer_simpson` | `suburban` | 15 |

Narrative dates are campaign data. They do not read the device calendar and do
not select the front-end calendar theme.

## Story and bonus mission sequence

### Level 1

The tutorial is `the_cola_caper`. The seven counted story missions are:

1. `s_m_r_t`;
1. `petty_theft_homer`;
1. `office_spaced`;
1. `blind_big_brother`;
1. `flowers_by_irene`;
1. `bonestorm_storm`; and
1. `the_fat_and_furious`.

The bonus mission is `this_old_shanty`.

### Level 2

1. `detention_deficit_disorder`;
1. `weapons_of_mass_delinquency`;
1. `vox_nerduli`;
1. `bart_n_frink`;
1. `better_than_beef`;
1. `monkey_see_monkey_doh`;
1. `cell_outs`; and
1. bonus mission `dial_b_for_blood`.

### Level 3

1. `nerd_race_queen`;
1. `clueless`;
1. `bonfire_of_the_manatees`;
1. `operation_hellfish`;
1. `slithery_sleuthing`;
1. `fishy_deals`;
1. `the_old_pirate_and_the_sea`; and
1. bonus mission `princi_pal`.

### Level 4

1. `for_a_few_donuts_more`;
1. `redneck_roundup`;
1. `ketchup_logic`;
1. `return_of_the_nearly_dead`;
1. `wolves_stole_my_pills`;
1. `the_cola_wars`;
1. `from_outer_space`; and
1. bonus mission `beached_love`.

### Level 5

1. `incriminating_caffeine`;
1. `and_baby_makes_8`;
1. `eight_is_too_much`;
1. `this_little_piggy`;
1. `never_trust_a_snake`;
1. `kwik_cash`;
1. `curious_curator`; and
1. bonus mission `kinky_frinky`.

### Level 6

1. `going_to_the_lu`;
1. `getting_down_with_the_clown`;
1. `lab_coat_caper`;
1. `duff_for_me_duff_for_you`;
1. `full_metal_jackass`;
1. `set_to_kill`;
1. `kang_and_kodos_strike_back`; and
1. bonus mission `milking_the_pigs`.

### Level 7

1. `rigor_motors`;
1. `long_black_probes`;
1. `pocket_protector`;
1. `theres_something_about_monty`;
1. `alien_autotopsy_part_1`;
1. `alien_autotopsy_part_2`;
1. `alien_autotopsy_part_3`; and
1. bonus mission `flaming_tires`.

A level cannot transition to its successor before its seventh story mission
commits. Bonus missions, street races, collectibles, purchases, and gags affect
completion but do not block the next story level.

## Street-race and wager membership

| Level | Street-race set | Reward | Wager fee | Time | Payouts |
| :--- | :--- | :--- | ---: | :--- | :--- |
| 1 | time trial, circuit, checkpoint | `electaurus` | 20 | 3:15 | 40/60/80 |
| 2 | commercial time trial, town circuit, motorway checkpoint | `sedan_level_02` | 25 | 2:10 | 50/75/100 |
| 3 | docks time trial, hillside circuit, Squidport checkpoint | `book_burning_van` | 30 | 2:20 | 60/90/120 |
| 4 | mansion time trial, rich circuit, suburban checkpoint | `clown_car` | 35 | 2:30 | 70/105/140 |
| 5 | entertainment time trial, commercial circuit, mixed checkpoint | `el_carro_loco` | 40 | 2:05 | 80/120/160 |
| 6 | resort time trial, casino circuit, Squidport checkpoint | `stutz_bearcat_36` | 45 | 1:55 | 90/135/180 |
| 7 | store time trial, haunted circuit, countryside checkpoint | `open_wheel_race_car` | 50 | 1:30 | 100/150/200 |

Payout values are easy, medium, and hard in that order. A wager-race attempt
stages its entry debit and result transaction independently from level progress.
Route topology, checkpoint order, opponents, position, timers, reset, finish, and
exactly-once race-set rewards follow
[Race route and opponent runtime](race-route-and-opponent-runtime.md).

The Level 7 store time trial has five laps and a seventy-second stage timer. A
stale summary claiming three laps is not authoritative.

The Level 4 mansion and power-plant time trial has three laps and a
one-hundred-thirty-one-second limit.

## Level-progress categories

The eight categories are:

| Category | Required count |
| :--- | ---: |
| Story missions | 7 |
| Bonus mission | 1 |
| Street races | 3 |
| Character costumes | 3 |
| Progression vehicles | 5 |
| Collector cards | 7 |
| Wasp cameras | 20 |
| Gags | Level-specific census |

Each category contributes exactly one eighth of level completion. Within a
category, progress is the accepted count divided by the required count and
clamped from zero through one.

The exact level fraction is:

`sum(category_completed / category_required) / 8`

The runtime stores the fraction as checked integer rational arithmetic. It does
not accumulate binary floating-point values. The UI multiplies the exact
fraction by one hundred and formats one decimal place.

A level reports one hundred percent only when all required identities in every
category are accepted. Duplicate events, inaccessible presentation, and stale
actors cannot increase a count.

## Overall completion

Overall completion is:

`0.99 * mean(level_01 through level_07 percentages) + movie_reward`

`movie_reward` is `1.0` percentage point when the all-card movie reward is
accepted and `0.0` otherwise. The runtime evaluates the equivalent exact rational
formula and formats the result for presentation.

The all-card reward remains a separate transaction. Merely opening the movie
screen does not grant the one-percent contribution.

## Base-world composition

The three base-world families are:

| Family | Levels | World responsibility |
| :--- | :--- | :--- |
| `suburban` | 1, 4, 7 | Evergreen Terrace, rich district, power plant, and countryside geography. |
| `downtown` | 2, 5 | Town square, entertainment, commercial, expressway, and stadium geography. |
| `harbor` | 3, 6 | Squidport, hillside, observatory, studio, brewery, and docks geography. |

Each base world is one persistent World Partition world. A level definition
selects one generated Runtime Data Layer set containing only that level's
variants and gameplay placements.

Level 7 is a substantial suburban variant, not a fourth geographic family.
Renamed locations such as `spook_e_mart` and `zombie_burger` are aliases or
presentation variants of canonical locations unless geometry or gameplay
identity proves a distinct location.

## World-transition transaction

A level transition follows this sequence:

1. verify the destination level and predecessor transition;
1. validate the accepted save revision and campaign state;
1. resolve the destination world, layer set, protagonist, and starting state;
1. load the destination definition and required Asset Manager bundles;
1. start a destination streaming source and wait for required cells;
1. open the destination base world when it differs from the active world;
1. activate only the destination Runtime Data Layer set;
1. bind level placements and world subsystems by stable identities;
1. verify the protagonist, spawn, mission, traffic, and progress projections;
1. commit the resume state; and
1. release prior bundles and streaming sources.

Failure before step ten leaves the prior accepted resume state intact. A failed
activation cannot produce a partially accepted level transition.

## Vehicle progression pattern

Every level exposes seven notable vehicle roles:

- one starting vehicle;
- one progression purchase or required seller vehicle;
- one bonus-mission reward;
- one street-race reward;
- two additional vendor offers; and
- one secret world vehicle.

Only the five purchase or reward roles count toward level completion. Starting
and secret roles do not.

A vehicle index page is a projection over catalog membership. It never creates a
second level inventory or omits a canonical vehicle from the level definition.

## Audio profile

Each level references one audio profile containing music states, ambience zones,
mission cues, character dialogue scopes, vehicle roles, and event transitions.
Music identity, cue bindings, semantic events, Quartz timing, graph parameters,
and transitions follow
[Music state and transition runtime](music-state-and-transition-runtime.md).

A page or index with no independently identified sound rows does not create a
second Level 7 sound catalog. Verified audio assets remain owned by their exact
audio roles and level profile.

## Validation

Generation rejects:

- a campaign with anything other than seven dense base levels;
- a level with the wrong predecessor, successor, date, protagonist, or world;
- a level without seven story missions, one bonus mission, or three street
  races;
- a level without three counted costumes, five counted vehicles, seven cards,
  twenty wasps, or its declared gag census;
- a story, bonus, race, reward, card, costume, vehicle, wasp, or gag identity
  assigned to more than one incompatible level role;
- a Data Layer set containing another level's gameplay placements;
- a world transition that accepts incomplete streaming or bindings;
- a progress value calculated from presentation state;
- a wager race counted toward completion; or
- an audio index that invents duplicate assets without role evidence.

## Verification

Automated evidence includes:

- exact campaign and level primary asset identifiers;
- the seven-level order, dates, protagonists, and world families;
- all story and bonus mission memberships;
- all street-race sets, rewards, wager policies, and payouts;
- exact per-category denominators and gag counts;
- one-decimal level progress golden cases from empty through complete;
- overall progress cases with and without the movie reward;
- repeated level transitions with stable save and placement identities;
- Runtime Data Layer read-back for every level variant;
- World Partition streaming completion before player activation; and
- failure injection before every transition commit boundary.

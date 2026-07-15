# Unreal gameplay content catalog

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Gameplay census, presentation, and development-content boundary](gameplay-census-presentation-and-development-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Legacy runtime identity normalization](legacy-runtime-identity-normalization.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Event-driven music and ambience](../../adr/unreal/runtime/event-driven-music-and-ambience.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mass Entity ambient population](../../adr/unreal/runtime/mass-entity-ambient-population.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Transactional phone-booth vehicle retrieval](../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Validated game-feature mod overlays](../../adr/unreal/runtime/validated-game-feature-mod-overlays.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Driving, traffic, and vehicle behavior parity](../../adr/gameplay/vehicles/driving-traffic-and-vehicle-ai.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Unreal manifest and package taxonomy](../../adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Unified open world and chapter projection](../../adr/pipeline/unreal/unified-open-world-and-chapter-projection.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native world partition and data layers](../../adr/pipeline/unreal/world-partition-and-data-layer-import.md)

## Purpose

This specification defines the canonical Unreal representation for gameplay
content. It fixes identity, asset placement, schemas, loading, progression,
validation, and verification for characters, vehicles, missions, locations,
rewards, costumes, dialogue events, races, and bonus modes.

The catalog is the runtime-facing composition layer above deterministic package
plans. It does not decode source formats, rediscover package membership, or let
mutable editor state redefine game identity.

## Catalog boundary

The catalog consumes approved native asset plans. A plan supplies stable package
identities, capabilities, dependencies, normalized artifacts, and provenance.
The catalog converts those inputs into game-domain definitions without changing
the plan's identity or classification.

A source census index and its manifest are coverage evidence, not runtime
assets. Their public contract is:

- every listed record has exactly one manifest entry;
- requested and exported record totals agree;
- export errors are zero before a catalog slice is accepted;
- duplicate pages and alternate names are normalized as aliases rather than
  duplicated gameplay entities; and
- descriptive prose, screenshots, external links, and historical trivia never
  become runtime authority.

## Canonical content layout

All authored runtime content lives below `/Game/SHAR`. No gameplay system may
scan another root or infer ownership from an arbitrary folder.

```text
/Game/SHAR
├── Data
│   ├── Catalog
│   ├── Campaigns
│   ├── Levels
│   ├── Characters
│   ├── Vehicles
│   ├── Missions
│   │   ├── Level_01
│   │   ├── Level_02
│   │   ├── Level_03
│   │   ├── Level_04
│   │   ├── Level_05
│   │   ├── Level_06
│   │   └── Level_07
│   ├── Locations
│   ├── Populations
│   ├── Music
│   ├── Rewards
│   ├── Costumes
│   ├── BonusModes
│   └── Tables
│       ├── Aliases
│       ├── Dialog
│       ├── MissionSteps
│       ├── RaceCheckpoints
│       ├── VehicleTuning
│       └── CostumeOffers
├── Art
│   ├── Characters
│   ├── Vehicles
│   ├── World
│   ├── Props
│   ├── UI
│   └── VFX
├── Audio
│   ├── Dialog
│   ├── Music
│   └── SFX
├── Media
└── Maps
    ├── Geography
    ├── LevelStates
    └── Tests
```

`Data` owns definitions and generated rows. `Art`, `Audio`, and `Media` own
secondary assets. `Maps` owns the persistent World Partition geography and
campaign or test-state projections. A secondary asset has one canonical location
even when several definitions reference it.

## Naming and identity

Canonical domain identifiers are lowercase `snake_case` names that remain
stable after publication. Primary asset names use those identifiers and never
include display punctuation, localization, level placement, source filenames,
or local routes.

| Asset family | Primary asset type | Object name |
| :--- | :--- | :--- |
| Root catalog | `SharCatalog` | `DA_SHAR_GameplayCatalog` |
| Campaign | `SharCampaign` | `DA_Campaign_<canonical_id>` |
| Level | `SharLevel` | `DA_Level_<canonical_id>` |
| Character | `SharCharacter` | `DA_Character_<canonical_id>` |
| Vehicle | `SharVehicle` | `DA_Vehicle_<canonical_id>` |
| Mission | `SharMission` | `DA_Mission_<canonical_id>` |
| Location | `SharLocation` | `DA_Location_<canonical_id>` |
| Population | `SharPopulation` | `DA_Population_<canonical_id>` |
| Music profile | `SharMusicProfile` | `DA_MusicProfile_<canonical_id>` |
| Music composition | `SharMusicComposition` | `DA_Music_<canonical_id>` |
| Reward | `SharReward` | `DA_Reward_<canonical_id>` |
| Costume set | `SharCostumeSet` | `DA_CostumeSet_<canonical_id>` |
| Bonus mode | `SharBonusMode` | `DA_BonusMode_<canonical_id>` |

Secondary asset prefixes are fixed:

- `SK_` for skeletal meshes;
- `SM_` for static meshes;
- `M_` for master materials;
- `MI_` for material instances;
- `T_` for textures;
- `ABP_` for animation Blueprints;
- `A_` for animation sequences and montages;
- `S_` for sounds;
- `W_` for worlds; and
- `DT_` for generated data tables.

A primary asset identifier is the pair of primary asset type and canonical
identifier. The Unreal object name is a reviewable presentation of that
identity, not its source. Renaming an object does not create a new domain
entity.

## Shared definition contract

Every top-level definition contains the following fields.

<!-- markdownlint-disable MD013 -->

| Field | Type | Contract |
| :--- | :--- | :--- |
| `CanonicalId` | `FName` | Stable domain identity; never localized. |
| `DisplayName` | `FText` | Localizable player-facing name. |
| `Aliases` | `TArray<FName>` | Alternate lookup names resolving to this identity. |
| `SourcePackageIds` | `TArray<FName>` | Approved deterministic package references. |
| `ContentTags` | `FGameplayTagContainer` | Capabilities and classifications, never identity. |
| `RequiredDefinitions` | soft primary-asset references | Definitions that must resolve before activation. |
| `RevisionToken` | `FString` | Deterministic generated-data revision. |
| `ValidationProfile` | `FName` | Exact validator contract for the asset family. |

<!-- markdownlint-enable MD013 -->

Aliases are normalized case-insensitively for lookup but stored in canonical
lowercase form. An alias may target only one canonical identifier. Alias chains,
cycles, and aliases that collide with a canonical identifier are invalid.

## Root catalog service

`USharGameplayCatalog` is the sole runtime registry. It is a non-Blueprint
primary data asset loaded through the Asset Manager. It contains soft primary
asset references grouped by family and a soft reference to the alias table.

`USharGameplayCatalogSubsystem` owns runtime resolution. It provides:

- canonical and alias lookup;
- bounded asynchronous definition loading;
- bundle selection;
- dependency closure validation;
- read-only enumeration by family or gameplay tag; and
- deterministic unload when a scope is no longer active.

The subsystem never discovers assets by directory scan. Asset Manager settings
register each fixed primary asset type and the exact `/Game/SHAR/Data` roots.
Cook rules include every catalog-reachable definition and reject orphaned
runtime definitions.

## Load bundles

Every definition uses the same bundle vocabulary.

<!-- markdownlint-disable MD013 -->

| Bundle | Includes | Allowed load point |
| :--- | :--- | :--- |
| `Definition` | Definition object and generated rows | Catalog validation and save migration |
| `Gameplay` | Collision, physics, objective, AI, and interaction assets | Available sandbox, chapter unlock, or active mission scope |
| `Presentation` | Meshes, materials, animation, UI, and icons | Visible or previewed content |
| `Audio` | Dialogue, music, vehicle, and interaction sounds | Audible content scope |
| `Cinematic` | Sequences, media, cameras, and cinematic-only assets | Active cinematic |
| `EditorReview` | Review-only references and conformance evidence | Editor and automated review only |

<!-- markdownlint-enable MD013 -->

`Definition` is always the first bundle loaded. Runtime code requests only the
additional bundles required by the current role. A mission must not preload all
presentation or audio assets for unrelated entities.

## Character definition

`USharCharacterDefinition` extends the shared definition with:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `CharacterRole` | Playable, non-playable, mission giver, ambient, or passenger. |
| `PlayableLevelIds` | Levels in which player control is permitted. |
| `PresenceLevelIds` | Levels in which the character may be placed. |
| `DefaultVehicleId` | Optional canonical vehicle identity. |
| `CostumeSetId` | Optional canonical costume-set identity. |
| `QuoteTable` | Soft reference to ordered quote-event rows. |
| `DefaultPresentationId` | Complete prepared base-model presentation identity. |
| `PresentationVariantTable` | Complete outfit, costume, or prop-bearing model variants. |
| `SemanticPreparationManifest` | FBX-owned UV, texture-region, eye-layer, rig-preservation, and variant evidence. |
| `EyeProfileId` | Prepared sclera, pupil, upper-eyelid, and lower-eyelid ownership. |
| `SkeletalMesh` | Soft default complete-model presentation reference. |
| `AnimationClass` | Existing animation Blueprint consumer; no retargeting change is implied. |
| `VoiceProfileId` | Canonical audio routing identity. |

<!-- markdownlint-enable MD013 -->

Character placement in a world is separate from character identity. The same
definition supports mission-giver, ambient, passenger, and playable placements
through role-specific components and data-layer composition.

Semantic UV, texture, eye, outfit, prop, and visual rig-display preparation is
owned by the FBX pipeline and follows the
[character semantic preparation](../fbx/character-semantic-preparation.md)
specification. Unreal consumes prepared evidence and does not perform the first
semantic split during UAsset import.

Each current outfit or integrated-prop presentation resolves to one complete
skeletal model. Equipping a costume selects that complete presentation at a safe
point; it does not assemble external garments, detach a prop, reconstruct a
hidden body, retarget animation, or alter hierarchy, bind state, skin weights,
or deformation behavior.

## Quote-event rows

`FSharQuoteEventRow` contains:

- canonical character identity;
- gameplay event tag;
- deterministic variant ordinal;
- soft sound reference;
- localization key;
- priority;
- cooldown duration;
- interruption policy; and
- optional context tags for vehicle, mission, location, or damage state.

Rows are ordered by character, event tag, and variant ordinal. Runtime selection
uses deterministic seeded choice when multiple variants are eligible. Missing
audio may suppress playback, but it must not remove the event or alter gameplay.

## Vehicle definition

`USharVehicleDefinition` extends the shared definition with:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `LifecycleState` | Active, inaccessible, or unused. |
| `NativeLevelIds` | Levels where the vehicle naturally exists. |
| `AcquisitionTable` | Soft reference to ordered acquisition rows. |
| `DriverCharacterId` | Optional canonical driver identity. |
| `TuningRowId` | Required vehicle-tuning row. |
| `Mesh` | Soft skeletal or static mesh reference. |
| `AnimationProfileId` | Doors, wheels, suspension, damage, and special effects. |
| `AudioProfileId` | Engine, horn, collision, and special audio. |
| `DamageProfileId` | Health, visual damage, destruction, and repair behavior. |
| `TrafficProfileId` | Optional traffic and pursuit behavior. |

<!-- markdownlint-enable MD013 -->

A vehicle definition has one identity and any number of acquisition contexts.
`FSharVehicleAcquisitionRow` contains vehicle identity, chapter acquisition
group, optional source-level alias, acquisition kind, seller or mission
identity, coin price, progression predicate,
phone-booth policy, and deterministic priority. Acquisition kinds are starting,
purchase, mission reward, street-race reward, native road access, secret world
access, mission-only, and completion override.

A road vehicle can be drivable in its native level without becoming a persistent
phone-booth reward. A vehicle may be both a reward in one level and a purchase
in
another without duplicating its definition. Inaccessible and unused lifecycle
states remain cataloged for completeness but cannot be activated by normal
progression.

## Vehicle-tuning rows

`FSharVehicleTuningRow` contains normalized speed, acceleration, toughness, and
handling ratings plus soft references to the native physics, tire, suspension,
damage, camera, and AI profiles. The four ratings are presentation metadata;
physics assets own simulation values. Validation rejects a visible rating that
has no corresponding native profile evidence.

## Mission definition

`USharMissionDefinition` extends the shared definition with:

| Field | Contract |
| :--- | :--- |
| `ChapterId` | Canonical narrative chapter identity. |
| `SourceLevelAlias` | Optional historic conversion alias only. |
| `SequenceOrdinal` | Stable sequence position within its mission family. |
<!-- markdownlint-disable-next-line MD013 -->
| `MissionClass` | Story, bonus, race, wager, taxi, boss, or another registered side activity. |
| `GiverCharacterId` | Optional mission-giver identity. |
| `PlayableCharacterId` | Required or default controlled-character identity. |
<!-- markdownlint-disable-next-line MD013 -->
| `OfferDefinitionId` | Optional world offer, marker, dialogue, and availability definition. |
| `PreviousMissionId` | Optional progression predecessor. |
| `NextMissionId` | Optional progression successor. |
| `StageTable` | Required ordered mission-stage table. |
<!-- markdownlint-disable-next-line MD013 -->
| `ConditionDefinitionIds` | Ordered required, optional, failure, and recovery condition definitions. |
<!-- markdownlint-disable-next-line MD013 -->
| `PresentationProfileId` | Mission offer, conversation, marker, HUD, camera, and transition presentation. |
<!-- markdownlint-disable-next-line MD013 -->
| `BonusObjectiveIds` | Optional independently evaluated objective definitions. |
| `BossEncounterId` | Optional typed encounter definition for a boss mission. |
| `RewardId` | Optional completion reward. |
<!-- markdownlint-disable-next-line MD013 -->
| `CompletionTransition` | Unlock, chapter transition, world expansion, ending, or none. |
| `WorldLayerSetId` | Required world and data-layer composition. |

Mission identity is independent of the world actor that starts it. A mission
may move or gain additional entry points without changing its save key.

## Mission-stage rows

`FSharMissionStageRow` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `MissionId` | Owning mission identity. |
| `StageId` | Stable mission-scoped stage identity. |
| `SequenceOrdinal` | Dense zero-based order within the mission. |
| `ObjectiveKind` | One value from the controlled objective taxonomy. |
| `ObjectivePolicyId` | Required objective-specific runtime policy identity. |
| `ConditionIds` | Ordered required, failure, optional, and recovery conditions. |
| `ParticipantBindingIds` | Characters, vehicles, AI, payloads, and world actors. |
| `RouteAndWaypointIds` | Ordered route, checkpoint, destination, and recovery identities. |
| `TargetIds` | Canonical entities, actors, zones, or items. |
| `RequiredCount` | Non-negative completion count. |
| `TimePolicyId` | Countdown, count-up, inherited, added, paused, or untimed policy. |
| `ForcedVehicleId` | Optional vehicle required for this stage. |
| `OpponentIds` | Ordered race, chase, or avoid participants. |
| `LocationId` | Canonical location or route identity. |
| `LockRequirementIds` | Explicit vehicle, costume, reward, or progression requirements. |
| `LoadPlanId` | Stage-specific asset and world-composition plan. |
| `CheckpointPolicyId` | Checkpoint creation and restore behavior. |
| `SuccessTransition` | Declared successor or mission completion. |
| `FailureTransition` | Stage retry, checkpoint restore, mission retry, abort, or failure. |
| `PresentationProfileId` | HUD, camera, dialogue, countdown, music, and transition requests. |
| `WorldPolicyId` | Traffic, population, notoriety, safe-zone, and control policy. |
| `FinalPolicy` | Whether accepted success may terminate the mission. |
| `BonusObjectiveStartIds` | Optional objectives activated by this stage revision. |

<!-- markdownlint-enable MD013 -->

The controlled objective taxonomy includes:

- `talk`;
- `enter_vehicle` and `exit_vehicle`;
- `travel`;
- `collect`;
- `deliver`;
- `destroy`;
- `hit_and_collect`;
- `follow`;
- `follow_and_collect`;
- `race`;
- `time_trial`;
- `avoid`;
- `load_vehicle`;
- `buy_vehicle`;
- `buy_costume`;
- `play_cinematic`; and
- `complete`.

A compound mission is an ordered composition of these objective contracts. It
is not represented as one opaque script. Every stage exposes preconditions,
observable progress, success, failure, and deterministic recovery. The source
concept commonly described as go-to maps to `travel`; it does not create a
second objective kind. Definition compilation, stage execution, objective
adapters, participant bindings, loading, checkpoint, abort, and progression
behavior follow the
<!-- markdownlint-disable-next-line MD013 -->
[mission definition, stage, and objective runtime](mission-definition-stage-and-objective-runtime.md).
Interaction, interior, notoriety, and world-safety behavior follow the
<!-- markdownlint-disable-next-line MD013 -->
[mission, interaction, interior, and notoriety runtime](mission-interaction-and-notoriety-runtime.md).

## Avoid objective contract

An `avoid` stage declares one or more pursuer identities, an escape condition,
and a reset policy. Completion requires all pursuers to remain outside the
configured detection or pursuit threshold for the configured duration. Merely
reaching a destination does not complete an avoid stage unless the stage
declares
that destination as its escape condition.

Pursuer destruction, despawn, world streaming, or mission restart must not
silently complete the objective. Each case follows the row's explicit failure or
recovery policy.

## Race checkpoint rows

`FSharRaceCheckpointRow` contains race identity, lap ordinal, checkpoint
ordinal, world-space route anchor identity, allowed travel direction, optional
time split, and respawn transform identity. Checkpoint order is dense and
stable. Circuit, checkpoint, and time-trial races use the same row type with
different completion policies.

## Location definition

`USharLocationDefinition` contains canonical geographic identity, world
coordinates, bounds, parent district or route, chapter and discovery
availability, World Partition Data Layers, structure and interior ownership,
mission entry points, interactive-object references, collectible placements,
connectors, shortcuts, and streaming bounds.

One persistent geographic world owns terrain and component placement. Seven
chapters contribute cumulative unlocks while the active `mission` or
`non_mission` state selects temporary gameplay projection. There is no test
level projection. Location definitions never collapse chapter progression,
mission state, collectibles, map discovery, structures, interiors, or save
identity.

## Reward definition

`USharRewardDefinition` contains reward kind, granted canonical identities,
progression predicate, repeatability, presentation references, and save-state
key. Vehicle rewards grant access to an existing vehicle definition; they never
create a second vehicle asset.

## Costume-set definition

`USharCostumeSetDefinition` contains the owning character, chapter availability,
menu visibility, eligibility, and a soft costume-offer table.
`FSharCostumeOfferRow` contains costume identity, display name, permanent coin
price, chapter prerequisite, complete prepared presentation identity, preview
references, and optional purchase-location identity.

Every costume is visible from the start. Buying a costume commits permanent
ownership, and an owned costume may be equipped from the menu at a safe point.
A costume changes presentation by selecting one complete prepared model unless a
separate explicit gameplay definition owns a bounded effect, such as Devil
Homer's zombie-disguise rule. It must not otherwise change collision, movement,
mission eligibility, save identity, voice identity, animation behavior, or
undeclared gameplay tags.

The catalog rejects a costume that requires returning to a shop after ownership,
loses ownership at a chapter transition, or grants undeclared gameplay behavior.

## Bonus-mode definition

`USharBonusModeDefinition` contains mode rules, eligible characters, eligible
vehicles, map unlock predicates, route definitions, scoring policy, and result
persistence. Bonus modes use separate maps and progression keys but reference
the same canonical character and vehicle definitions as the main game.

## Verified initial character slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Aliases | Required contract |
| :--- | :--- | :--- |
| `abraham_simpson` | `abe_simpson`, `grampa` | Non-playable mission giver; present across several levels; mission roles remain level-scoped. |
| `agnes_skinner` | none | Ambient and passenger-capable non-playable character. |
| `apu_nahasapeemapetilon` | `apu` | Playable in Level 5; present in all seven levels; owns the Longhorn; has event-tagged dialogue. |
| `barney_gumble` | `barney` | Non-playable character; car-dealer and mission roles are placement-specific. |
| `bart_simpson` | `bart` | Playable protagonist with level-scoped missions, costumes, owned vehicles, and event-tagged dialogue. |
| `carl_carlson` | `carl` | Non-playable mission giver with a Level 1 mission role. |

<!-- markdownlint-enable MD013 -->

Alias records for these names resolve to the listed canonical identity. They do
not create duplicate character definitions, quote tables, progression keys, or
world actors.

## Verified initial vehicle slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Verified context | Required rule |
| :--- | :--- | :--- |
| `stutz_bearcat_1936` | Reward | Level 6 street-race prize; phone-booth access after unlock. |
| `sports_car_1970s` | Starting | Level 7 starting vehicle; character-driver presentation is level-scoped. |
| `atv` | Secret | Native to Level 4; normal progression does not grant global access. |
| `ambulance` | Road | Native to Level 5; completion override may expose it outside normal progression. |
| `armored_truck` | Purchasable | Persistent unlockable vehicle with a separate reward and phone-booth rule. |
| `audi_tt` | Unused Content | Ships as a reachable unused-content vehicle; incomplete presentation or support uses declared generic fallbacks and remains replaceable by validated mod overlays. |
| `bandit` | Reward | Level 6 bonus-mission reward and a forced vehicle in a later mission. |
| `bonestorm_truck` | Inaccessible | Alias `cbone`; mission target in Level 1; completion override does not change its canonical identity. |
| `book_burning_van` | Reward | Level 3 street-race prize; phone-booth access after unlock. |
| `brick_car` | Unused Content | Ships through the unused-content selection surface with isolated progression and mod-replaceable presentation, tuning, and placement fields. |
| `burns_armored_truck` | Road | Distinct Level 6 road variant; never aliases the purchasable armored truck. |
| `cpolice` | Inaccessible | Police vehicle present in Levels 1 through 6; excluded from normal progression. |
| `canyonero` | Purchasable | Player vehicle and forced transport for the Level 1 hit-and-collect mission. |
| `car_built_for_homer` | Reward and purchase | Bonus-mission reward in one context and a 500-coin Level 5 purchase in another; alias `custom_built_car`; one canonical vehicle and phone-booth identity. |
| `cell_phone_car` | Inaccessible | Level 2 mission target; excluded from normal progression. |

<!-- markdownlint-enable MD013 -->

The vehicle-family census additionally establishes these invariants:

- every drivable vehicle has speed, acceleration, toughness, and handling
  presentation ratings;
- every active vehicle can be damaged and destroyed according to a typed damage
  profile;
- horn, engine, collision, camera, wheel, and special effects are explicit
  profile references;
- road, reward, secret, inaccessible, and Unused Content are distinct
  availability states;
- every verified Unused Content identity is reachable in the shipping product
  through its dedicated surface even when campaign progression remains isolated;
- missing presentation or support resolves through clearly generic,
  repository-owned or appropriately licensed fallback definitions;
- every generic fallback exposes schema-declared replacement fields to validated
  mod overlays; and
- a completion override never changes a vehicle's canonical identity or native
  level placement.

## Unused Content catalog projection

`USharUnusedContentDefinition` projects any accepted unused character, vehicle,
mission, world object, audio event, animation, costume, effect, or presentation
identity into the shipping product. It contains:

- canonical identity and content class;
- source-evidence and catalog revisions;
- native gameplay definition and dependency closure;
- dedicated frontend, sandbox, gallery, selection, or placement route;
- campaign integration and progression-isolation policy;
- generic fallback identities for every unavailable dependency;
- mod-extensible field declarations; and
- validation and replacement-overlay test identities.

A missing original asset does not delete the content definition. The catalog
resolves an original generic fallback that is clearly identified as replacement
material and does not imitate unavailable protected expression. The fallback
uses stable semantic slots so a validated mod can replace individual meshes,
materials, textures, audio, animation, user-interface, tuning, dialogue,
placement, or complete presentation bundles without changing the canonical
content identity.

The dedicated Unused Content surface is part of the base package. It cannot be
implemented only as a development command, editor browser, or optional external
mod. Campaign use remains opt-in per definition so inclusion cannot accidentally
grant rewards, unlocks, purchases, completion, or achievements.

## Verified initial mission and race slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Level and class | Ordered contract |
| :--- | :--- | :--- |
| `alien_autotopsy_part_1` | Level 7 main mission 5 | Collect map, enter vehicle, collect waste, travel to playground, deliver vehicle into the target zone, then exit. |
| `alien_autotopsy_part_2` | Level 7 main mission 6 | Force `bandit`, deliver the payload, and satisfy an avoid objective before completion. |
| `alien_autotopsy_part_3` | Level 7 main mission 7 | Force the rocket-equipped wartime vehicle, race an opponent, collect and deliver the payload, satisfy avoidance, then trigger the ending transition. |
| `bart_and_frink` | Level 2 main mission 4 | Follow the delivery vehicle, talk to the police contact, locate the criminal, talk to the opponent, race to the stadium, and collect the radio. |
| `beached_love` | Level 4 bonus mission | Timed collect mission with a one-time vehicle reward. |
| `better_than_beef` | Level 2 main mission 5 | Force the pickup, collect all road items, return, avoid the pursuer, then return again. |
| `blind_big_brother` | Level 1 main mission 4 | On-foot travel, enter the office, exit, destroy nine control boxes, and return. |
| `bonestorm_storm` | Level 1 main mission 6 | Force `canyonero`, travel, talk, hit the target truck, collect ten dropped boxes, and return home. |
| `bonfire_of_the_manatees` | Level 3 main mission 3 | Force `longhorn`, travel and talk, hit the target vehicle and collect dropped items, travel to the observatory, then talk. |
| `caravan_park_time_trial` | Level 1 street race | Five clockwise laps through the trailer-park route within ninety seconds. |
| `casino_circuit_race` | Level 6 street race | Five counter-clockwise laps with one ordered opponent and a fixed circuit. |
| `cell_outs` | Level 2 final main mission | Destroy four cell-phone cars, complete the mission, and unlock Level 3. |

<!-- markdownlint-enable MD013 -->

Mission rows preserve predecessors, successors, giver identities, controlled
character, forced vehicle, timers, counts, opponents, routes, and completion
transitions independently. Narrative text and dialogue do not substitute for
those structured fields.

## Verified initial location, reward, costume, and bonus-mode slice

### Android's Dungeon

`androids_dungeon` is an interior-capable location available in Levels 3 and 6.
It can host mission starts, interactive gags, and a collector-card placement. A
special completion-gated ticket interaction is active only in its declared
level context. The Level 3 and Level 6 placements reference one location
definition but retain separate data-layer and interaction rows.

### Buzz Cola

`buzz_cola` is a collectible and world-prop identity. Collection state,
presentation, collision, placement, reward contribution, and respawn policy are
separate fields. A decorative prop instance cannot accidentally grant
collection progress.

### Bonus missions

Each historic chapter group retains one canonical bonus-mission slot. A bonus
mission:

- has a specific mission giver distinct from the main-story continuation;
- projects one typed offer and marker into `non_mission` when eligible;
- can complete only once per save unless a separate replay policy is declared;
- grants one declared vehicle reward or another typed reward exactly once;
- may contain independently evaluated optional bonus objectives; and
- remains optional for main-story progression unless another explicit predicate
  requires its reward.

Mission offer, marker, acceptance, optional-objective, and completion behavior
follow the
<!-- markdownlint-disable-next-line MD013 -->
[mission, interaction, interior, and notoriety runtime](mission-interaction-and-notoriety-runtime.md).

### Bonus game

`bonus_game` is a top-down racing mode. It references the five playable
character definitions and the catalog's eligible vehicle set. Each bonus map has
an explicit chapter-card-set completion predicate. Map unlocks are independent,
and completing one chapter set does not unlock another chapter's map.

### Character costumes

Each historic chapter acquisition group provides three ordinary costume offers
for its playable character. Offers are purchased with coins through a declared
offer. A purchased costume persists, is visible from the beginning, and may be
equipped from the menu at a safe point for its owning character.

## Progression and save-state contract

Save data stores canonical identities and explicit state, never object paths or
display names. The minimum state is:

- current chapter identity;
- `mission` or `non_mission` gameplay state;
- completed mission identities;
- active mission and checkpoint identity when resumable;
- unlocked and currently eligible character identities;
- unlocked vehicle identities;
- terrain, map discovery, connector, interior, and world-expansion state;
- completed bonus-mission and taxi-milestone identities;
- purchased and equipped costume identities;
- collected cards and completed chapter-card sets;
- unlocked passive ability and bonus-map identities;
- world clock, health, stamina, and Chapter 7 survival state where required;
- achievement progress and active mod-achievement policy; and
- migration revision.

Alias resolution occurs before save lookup. Save migration may redirect a
retired canonical identity only through an explicit versioned migration map.
Missing definitions fail the load with a recoverable diagnostic; they are never
silently dropped from progression.

## Import and generation flow

1. Validate the native asset plan and package dependency graph.
1. Resolve each package to one catalog family and canonical identity.
1. Normalize aliases before creating any object.
1. Generate or update primary data assets and typed tables in deterministic
   identity order.
1. Attach soft secondary-asset references and bundle metadata.
1. Validate every definition and dependency closure without loading unrelated
   presentation bundles.
1. Apply bounded editor mutations.
1. Read back primary asset identifiers, rows, tags, bundles, and references.
1. Compare read-back state with the approved plan.
1. Reject and roll back incomplete catalog slices.

Generation is idempotent. Repeating it with equivalent validated input preserves
primary asset identifiers, row names, row order, aliases, tags, and references.

## World integration

One persistent geographic map is the World Partition world. Seven chapters add
cumulative unlocks over that world. The exclusive `mission` or `non_mission`
state selects transient projection for mission actors, traffic overrides,
objective pickups, hazards, routes, dialogue, and interactions. There is no test
level or campaign-visible development projection.

The base world always owns dynamic sunrise, day, sunset, and night. Chapter 7
adds its irradiated cloud, humidity, haze, hazard, and horror profile without
creating another map or disabling the world clock.

Catalog definitions reference geographic location, structure, interior,
connector, component, placement, discovery, and Data Layer identities. They
never store mutable actor pointers as authority.
Runtime placement resolves actors from stable coordinate and transform records
after the required World Partition cells and data layers are active.

Streaming out a cell suspends eligible ambient presentation but does not reset
mission progress, vehicle damage, collected rewards, or save state. Mission
actors required by an active step remain pinned through an explicit gameplay
streaming source or the step fails before activation.

## Invariants

- One canonical gameplay entity has one primary asset identifier.
- Every alias resolves directly to one canonical identity.
- Every catalog-reachable primary asset is included in cook rules.
- Canonical identities, aliases, progression keys, table rows, and gameplay
  bundles remain logically identical across platforms, architectures, and
  graphics presets.
- Platform cooking may select native presentation implementations, but it cannot
  remove or redefine a gameplay definition required by the shared catalog.
- Every mission has a dense ordered step sequence.
- Every step references existing canonical entities and locations.
- Every forced vehicle is available to the mission even when normal progression
  would not unlock it.
- Every reward grants an existing definition.
- Every costume belongs to one character and one purchase rule.
- Every quote row has a unique character, event, and variant key.
- Every race has a dense checkpoint order and explicit direction.
- Gameplay tags classify content but never determine identity.
- Soft references and bundles prevent unrelated content from being loaded
  eagerly.
- Equivalent validated input generates equivalent catalog state.

## Failure behavior

Catalog generation fails closed on:

- duplicate canonical identities;
- alias collisions, chains, or cycles;
- unsupported asset families;
- missing package provenance;
- unresolved required definitions;
- invalid soft references;
- missing Asset Manager registration or cook rules;
- invalid gameplay tags;
- nondeterministic table order;
- gaps or duplicates in mission-stage or checkpoint ordinals;
- negative counts or non-positive configured timers;
- forced vehicles without required gameplay assets;
- rewards that reference inaccessible or missing definitions;
- a level placement without a valid geographic-world, coordinate, placement,
  and data-layer composition;
- platform or preset cooking that removes, duplicates, or rekeys a required
  gameplay definition; or
- read-back state that differs from the approved plan.

A failed batch leaves no success marker. Newly created incomplete assets are
removed, and previously valid assets retain their last accepted revision.
Runtime lookup returns a typed missing, invalid, or unavailable result rather
than a null dereference or guessed fallback.

## Verification

Engine-independent tests verify:

- canonical identifier normalization;
- alias uniqueness and cycle rejection;
- deterministic generation order;
- schema validation;
- mission-stage and race-checkpoint topology;
- progression predicates;
- save migration; and
- package-to-definition membership.

Editor integration tests verify:

- every primary asset type is registered;
- primary asset identifiers survive save, reload, and cook discovery;
- bundle metadata loads only declared secondary assets;
- generated tables use the expected C++ row structure;
- soft references resolve after import;
- aliases resolve to the same loaded object as canonical identities;
- Windows, Linux, macOS, and Android cooks preserve the same canonical
  identities, aliases, progression keys, and required gameplay bundles;
- Low through Ultra desktop cooks preserve the same gameplay definitions, while
  Android Low preserves the same definitions through its mobile presentation
  implementations;
- World Partition and data-layer activation produces the expected placements;
- read-back state matches the approved native asset plan; and
- a second generation produces no semantic diff.

Runtime parity tests execute representative contracts from this slice:

- a character alias and canonical name load the same character definition;
- a street-race reward becomes available through the phone booth only after
  completion;
- a road vehicle remains native-level-only before its completion override;
- a forced mission vehicle loads even when it is not normally unlocked;
- an avoid objective cannot complete through streaming or despawn;
- the Level 1 on-foot destroy mission completes exactly after nine targets;
- the Level 2 final mission unlocks Level 3 only after four targets;
- a bonus mission cannot grant its vehicle twice;
- collector-card completion unlocks only the matching bonus map; and
- a costume changes presentation without changing gameplay identity.

## Verified second character slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Aliases | Required contract |
| :--- | :--- | :--- |
| `charles_montgomery_burns` | `mr_burns`, `burns` | Non-playable mission character with distinct Level 1 and Level 7 placements. |
| `clancy_wiggum` | `chief_wiggum`, `wiggum` | Non-playable police character, passenger, and mission participant; present across all seven levels; owns event-tagged dialogue. |
| `cletus_spuckler` | `cletus` | Non-playable mission giver with level-scoped main and bonus mission roles. |
| `comic_book_guy` | `jeffrey_albertson` | Non-playable mission giver; owns the Kremlin vehicle reference; cutscene-only and interactive placements remain distinct. |
| `julius_hibbert` | `dr_hibbert` | Non-playable Level 5 mission giver. |
| `nick_riviera` | `dr_nick` | Non-playable mission character with Level 2, Level 3, and Level 6 placements. |

<!-- markdownlint-enable MD013 -->

The Chief Wiggum quote page contributes rows to `clancy_wiggum`'s quote table.
It does not create a second character, dialogue owner, or voice identity.
Likewise, alternate pages for Cletus resolve to `cletus_spuckler`.

## Verified second vehicle slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Verified contexts | Required rule |
| :--- | :--- | :--- |
| `chase_sedan` | Level 6 purchase for 500 coins; mission and opponent placements in Levels 3, 4, and 6 | Purchase ownership, police presentation, and alien-controlled mission behavior are separate acquisition and placement rows. |
| `clown_car` | Level 4 street-race reward | Phone-booth access begins after the reward transaction. |
| `coffin_cart` | Level 7 road vehicle | Native road access does not grant persistent retrieval before the completion override. |
| `cola_truck` | Level 5 purchase for 350 coins; mission target | The player-owned offer and alien-controlled mission placement share one vehicle definition. |
| `compact_car` | Road vehicle in Levels 3, 4, and 6 | Native traffic access remains distinct from completion-override retrieval. |
| `cube_van` | Unused and inaccessible | Cataloged for completeness; no normal world placement or progression activation. |
| `curator` | Level 4 purchase for 300 coins; Level 5 mission target | Player ownership and target behavior use separate acquisition and placement rows. |
| `car_built_for_homer` | Alias `custom_built_car`; Level 5 purchase for 500 coins; reward context | Every acquisition grants the same canonical vehicle and save identity. |
| `donut_truck` | Level 3 purchase for 250 coins | Persistent retrieval begins only after purchase. |
| `duff_truck` | Level 1 purchase for 125 coins; Level 6 mission target | Ordinary tuning and mission-specific target tuning remain explicit profiles. |
| `el_carro_loco` | Level 5 street-race reward | Phone-booth access begins after all three level races complete. |
| `electaurus` | Level 1 street-race reward | Driver presentation in later levels does not change ownership identity. |
| `family_sedan` | Level 1 starting vehicle | Available from the retrieval interface from the start; Homer is the canonical driver presentation. |
| `ferrini_black` | Inaccessible Level 7 hostile vehicle | Alias `alien_car`; mission pursuit and race roles do not grant ownership. |
| `ferrini_red` | Level 6 starting vehicle; Level 5 forced mission vehicle | Bart driver presentation and cross-level mission use retain one identity. |
| `fire_truck` | Level 2 purchase for 250 coins | Persistent retrieval begins only after purchase. |
| `fish_delivery_truck` | Level 3 road vehicle | Alias `fish_van`; completion override does not change its native traffic role. |

<!-- markdownlint-enable MD013 -->

Mission-specific tuning never mutates the shared vehicle definition. A placement
row may select a mission tuning profile, driver, artificial-intelligence role,
damage policy, or objective marker while preserving the canonical vehicle,
acquisition, and save identity.

## Verified second mission slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Level and class | Ordered contract |
| :--- | :--- | :--- |
| `clueless` | Level 3 main mission 2 | Alternate timed travel and talk steps across Wall E. Weasel's, Planet Hype, and the Springfield Sign. |
| `curious_curator` | Level 5 final main mission | Force `ferrini_red`, pursue and destroy `curator`, collect the museum key, complete the transition, and unlock Level 6. |
| `detention_deficit_disorder` | Level 2 main mission 1 | Travel toward the store, satisfy the Skinner avoid objective, then complete the destination step. |
| `dial_b_for_blood` | Level 2 bonus mission | Collect the plasma-center blood, travel and talk at Moe's, collect the second blood, travel and talk at the construction-site restaurant, collect the third blood, return, talk, and grant the wartime vehicle reward once. |
| `duff_for_me_duff_for_you` | Level 6 main mission 4 | Travel to the brewery, hit the target Duff Truck, collect six dropped laser crates, return to the brewery, and collect the final proof item. |
| `eight_is_too_much` | Level 5 main mission 3 | Talk to Hibbert, require `car_built_for_homer` or an explicitly permitted substitute, enter the vehicle, hit the van, collect ten diapers, return to the hospital, and talk. |
| `fishy_deals` | Level 3 main mission 6 | Talk to the sea-captain contact, collect the ordered moving fish targets with the declared miss allowance, and complete the save objective. |
| `flaming_tires` | Level 7 bonus mission | Talk to Smithers, collect the three ordered personal-item targets under their timers, return after each required segment, and grant the Burns limousine once. |

<!-- markdownlint-enable MD013 -->

A required vehicle and a forced vehicle are distinct. A forced vehicle is
selected by the mission. A required-vehicle step validates that the player has
entered an allowed definition and may permit declared substitutes. The mission
cannot silently replace an invalid vehicle with an arbitrary current car.

## Verified second street-race slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Level and policy | Route contract |
| :--- | :--- | :--- |
| `checkpoint_race_level_01` | Level 1 checkpoint race | Start at the church, traverse the ordered residential and poor-district checkpoints, and finish at the power-plant parking area against three ordered opponents. |
| `circuit_race_level_01` | Level 1 circuit race | Complete three laps around the rich-district loop before three ordered opponents. |
| `commercial_district_time_trial_level_02` | Level 2 time trial | Complete three laps of the commercial and monorail loop within 81 seconds. |
| `docks_time_trial_level_03` | Level 3 time trial | Complete four laps of the docks, studio road, alley, ramp, and ship-jump loop within 111 seconds. |
| `commercial_district_circuit_level_05` | Level 5 circuit race | Complete three ordered commercial-to-town-square laps against `ferrini_red`, a campaign truck, and an ambulance. |
| `entertainment_district_time_trial_level_05` | Level 5 time trial | Complete five clockwise laps of the two-block entertainment loop within 81 seconds. |
| `entertainment_commercial_checkpoint_level_05` | Level 5 checkpoint race | Traverse the courthouse, train-yard, expressway, and commercial-district checkpoint chain against `ferrini_red`. |

<!-- markdownlint-enable MD013 -->

Race definitions preserve route direction, lap count, time limit, opponents,
closed shortcuts, checkpoint order, respawn transforms, and finish transition.
Artificial-intelligence catch-up policy is an explicit race profile and cannot
silently vary by frame rate or graphics preset.

## Verified second location slice

`duff_brewery` is an open location available in Levels 3 and 6. One location
definition owns shared geometry and interaction identity. Level-specific world
layers own mission targets, traffic, collectibles, dialogue, and progression
state. The Level 6 mission route references the brewery, the target truck, six
dropped mission items, and the final proof-item placement through canonical
identities.

## Progression and meta-game integration

The currency, collector-card, destructible-source, cheat, credits, and calendar
entries in this coverage slice are governed by
<!-- markdownlint-disable-next-line MD013 -->
[Progression, collectibles, cheats, and credits](progression-collectibles-and-cheats.md).
The gameplay catalog references their primary assets and tables but does not
collapse their persistence or mutation semantics into generic pickups.

Additional parity tests from this slice verify:

- alternate character names resolve to one canonical character and quote table;
- one vehicle can expose multiple acquisition rows without duplicate ownership;
- a purchase and a mission target can reference the same vehicle with different
  placement profiles;
- a required-vehicle mission accepts only declared vehicle definitions;
- a forced-vehicle mission loads the exact declared definition;
- a destroy step completes on validated destruction rather than despawn;
- a hit-and-collect step accepts each dropped target once;
- card, coin, and cheat state follow their distinct save policies;
- every street-race route has dense checkpoints and deterministic opponents; and
- the Level 5 final mission unlocks Level 6 only after its key collection and
  completion transition.

## Verified third character slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Aliases | Required contract |
| :--- | :--- | :--- |
| `professor_frink` | `frink` | Non-playable scientist and mission giver with level-scoped story, bonus-mission, ambient, and cinematic placements from Levels 2 through 7. |
| `gil_gunderson` | `gil` | Non-playable vehicle vendor whose level inventories are separate offer rows owned by one character identity. |
| `abraham_simpson` | `abe_simpson`, `grampa_simpson`, `grampa`, `grandpa_simpson`, `grandpa` | Non-playable mission giver and ambient character; every spelling resolves to one dialogue, placement, and save identity. |
| `groundskeeper_willie` | `willie` | Non-playable school-area character with level-scoped ambient placements and an explicit tractor association. |
| `hans_moleman` | `ralph_melish` | Non-playable mission giver and ambient gag participant; mission placement and gag presentation remain separate rows. |
| `homer_simpson` | `homer` | Playable character in Levels 1 and 7 with additional level-scoped presentation roles; all quote rows bind to this identity. |
| `horatio_mccallister` | `sea_captain` | Non-playable Level 3 mission giver and ambient Squidport placement. |
| `comic_book_guy` | `jeffrey_albertson` | The existing canonical identity is reaffirmed; cutscene, mission-giver, store, vehicle-owner, and ambient placements do not create another character. |
| `jimbo_jones` | `jimbo` | Non-playable Level 2 mission character with declared ambient placements in later entertainment-district variants. |
| `kang` | none | Individual cinematic antagonist identity with no ordinary world placement. |
| `kodos` | none | Individual cinematic antagonist identity with no ordinary world placement. |

<!-- markdownlint-enable MD013 -->

Kang and Kodos may share a cinematic cast group, dialogue scene, spacecraft, and
plot-state presentation. The pair is not a third character identity and cannot
own a duplicate dialogue, progression, or save record.

A character quote collection always references its canonical character. The
Homer quote collection therefore extends `homer_simpson`; it does not create a
quote-only character. No separate gag quote owner is defined by this slice.

## Verified third vehicle slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Verified context | Required rule |
| :--- | :--- | :--- |
| `garbage_truck` | Level 4 road vehicle | Drivable from native traffic only; no normal retrieval ownership before the completion override. |
| `ghost_ship` | Level 7 road vehicle and race opponent | Drivable from native traffic only; race placement and completion-override retrieval do not create a second vehicle. |
| `glass_truck` | Level 1 road vehicle | Drivable from native traffic only; no normal retrieval ownership before the completion override. |
| `globex_super_villain_car` | Level 6 purchase for 600 coins | The accepted purchase grants persistent retrieval for this canonical identity. |
| `hallo_hearse` | Level 7 road vehicle | Distinct from `hearse`; native traffic access does not grant persistent retrieval. |
| `hearse` | Level 7 purchase for 750 coins and race opponent | Purchase ownership and race placement share one vehicle definition. |
| `honor_roller` | Level 2 starting vehicle | Persistent retrieval is available from level start without a purchase transaction. |
| `hover_bike` | Level 7 purchase for 1,000 coins | Persistent retrieval begins only after the accepted purchase. |
| `hover_car` | Level 5 bonus-mission reward | The reward transaction grants persistent retrieval exactly once. |
| `ice_cream_truck` | Unused and inaccessible | Cataloged for completeness; no ordinary traffic, mission, purchase, reward, or retrieval activation. |
| `itchy_and_scratchy_movie_truck` | Level 6 road vehicle | Drivable from native traffic only; presentation audio belongs to its vehicle profile and does not imply ownership. |

<!-- markdownlint-enable MD013 -->

`hallo_hearse` and `hearse` are separate canonical definitions despite their
similar display names. Validation rejects an alias, redirect, purchase, traffic
placement, or race row that collapses them.

## Verified third mission slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Level and class | Ordered contract |
| :--- | :--- | :--- |
| `flowers_by_irene` | Level 1 main mission 5 | Enter the house, activate the television interaction, leave the interior, enter a vehicle, and follow the surveillance vehicle to the declared destination without violating the separation policy. |
| `for_a_few_donuts_more` | Level 4 main mission 1 | Complete the opening follow segment, hit the donut truck, collect ten emitted donuts exactly once, return, and deliver the accepted set. |
| `from_outer_space` | Level 4 final main mission | Destroy three declared trucks, return toward home, satisfy the final avoid policy, complete the transition, and unlock Level 5. |
| `full_metal_jackass` | Level 6 main mission 5 | Pursue and destroy the declared sedan, accept the dropped laser item once, and complete only after collection. |
| `getting_down_with_the_clown` | Level 6 main mission 2 | Trigger the opponent vehicle and win the declared race to the Squidport finish against the limousine. |
| `going_to_the_lu` | Level 6 main mission 1 | Force the school bus, collect the declared child targets, deliver them to the studio destination, and retain no ownership change from the forced vehicle. |
| `incriminating_caffeine` | Level 5 main mission 1 | Follow the target truck, collect eleven ordered drops without violating the follow policy, and finish at the declared club destination. |
| `kang_and_kodos_strike_back` | Level 6 final main mission | Force the 1970s sports car, race the chase sedan to the brewery, complete the transition, and unlock Level 7. |

<!-- markdownlint-enable MD013 -->

The three target-following forms use different objective policies:

- `follow` enforces separation and normal target-contact notoriety;
- `follow_and_collect` enforces separation plus ordered dropped-item acceptance
  and retains normal target-contact notoriety; and
- `hit_and_collect` emits declared items from accepted impacts and exempts only
  contact with the declared objective target.

A retry may select a declared retry start profile, including a target that
begins
moving immediately rather than waiting for proximity. Catch-up, lead failure,
separation failure, drop acceptance, and target-contact policy remain explicit
runtime data.

## Verified third street-race slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Level and policy | Route contract |
| :--- | :--- | :--- |
| `hillside_area_circuit_level_03` | Level 3 circuit race | Complete five laps of the declared figure-eight hillside loop against the Canyonero, one sports car, and one compact car. |
| `haunted_suburbia_circuit_level_07` | Level 7 circuit race | Complete three school-to-residential-and-return laps against the Hearse, Ghost Ship, and Coffin Cart. |

<!-- markdownlint-enable MD013 -->

Both routes require dense ordered checkpoints, declared direction at every
ambiguous crossing, deterministic reset transforms, exact opponent identities,
and a finish transition that cannot be reached by approaching the finish from an
undeclared route segment.

## Verified third location and interaction slice

The Level 1 location set adds `simpson_house`, `flanders_house`, `wiggum_house`,
and `gold_house`. The Gold House location record and the Level 1 location index
resolve to the same `gold_house` identity; they do not create duplicate world
anchors, collectibles, or secret-vehicle placements.

The Level 3 set adds `androids_dungeon`, `wall_e_weasels`, and `planet_hype`.
`planet_springfield` is a display alias for `planet_hype` , not a second
location.
The Level 2 location census in this slice declares only the role of notable
locations and contributes no new canonical location identity.

The canonical indoor set, portal transactions, world-layer composition, movement
restrictions, vehicle-state preservation, gag interactions, and notoriety
behavior follow the
<!-- markdownlint-disable-next-line MD013 -->
[mission, interaction, interior, and notoriety runtime](mission-interaction-and-notoriety-runtime.md).

Gag placements, rewards, level-scoped completion, and the verified level totals
follow
<!-- markdownlint-disable-next-line MD013 -->
[Progression, collectibles, cheats, and credits](progression-collectibles-and-cheats.md).

## Verified fourth character slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Aliases | Required contract |
| :--- | :--- | :--- |
| `kearney_zzyzwicz` | `kearney` | Non-playable character with Level 2, Level 4, and Level 6 placements; the Level 6 vendor role references the existing vehicle offers. |
| `kent_brockman` | none | Cutscene and broadcast character; television-gag audio is a presentation placement, not an ambient world character. |
| `krusty_the_clown` | `krusty` | Non-playable mission giver and story character with level-scoped ambient, cinematic, and mission placements. |
| `lenny_leonard` | `lenny` | Non-playable Level 1 mission giver with declared ambient placements in Levels 2 and 5. |
| `lisa_simpson` | `lisa` | Playable Level 3 protagonist; quote rows and every other level placement retain one character identity. |
| `marge_simpson` | `marge` | Playable Level 4 protagonist; quote rows and every other level placement retain one character identity. |
| `louie` | none | Non-playable wager-race host in all seven levels and a separate Level 5 story placement. |

<!-- markdownlint-enable MD013 -->

A quote page extends the canonical character's quote-event table. It never
creates a quote-only character, voice owner, progression key, or placement.

A vendor, race host, mission giver, ambient pedestrian, cinematic role, and
broadcast role are placement capabilities. They do not create parallel
character definitions.

## Verified fourth vehicle slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Aliases | Verified context | Required rule |
| :--- | :--- | :--- | :--- |
| `knight_boat` | `knightboat` | Level 3 secret vehicle | One world placement grants temporary access; it does not count toward the five progression vehicles. |
| `kremlin` | none | Level 4 bonus-mission reward | The accepted reward grants persistent retrieval exactly once. |
| `krustys_limo` | none | Level 4 purchase for 350 coins and mission presentation | Purchase ownership and opponent or ambient placements share one definition. |
| `limo` | none | Level 2 purchase for 150 coins | Distinct from `krustys_limo`; purchase grants persistent retrieval. |
| `longhorn` | none | Level 5 starting vehicle | Available from level start and excluded from counted progression vehicles. |
| `malibu_stacy_car` | none | Level 3 starting vehicle | Available from level start and bound to Lisa's default driver presentation. |

<!-- markdownlint-enable MD013 -->

`knight_boat` and `knightboat` are aliases. `limo` and `krustys_limo` are
separate canonical vehicle definitions. Validation rejects an alias or redirect
that collapses the two limousine definitions.

## Verified fourth mission slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Level and class | Ordered contract |
| :--- | :--- | :--- |
| `ketchup_logic` | Level 4 main mission 3 | Force the pickup truck and required costume, collect eighteen ordered packets within 120 seconds, reach the pursuit trigger within 45 seconds, evade the sedan within 60 seconds, and return to the declared destination. |
| `kinky_frinky` | Level 5 bonus mission | Destroy the Hover Car within 180 seconds, return within 30 seconds, complete the conversation, and grant the Hover Car reward once. |
| `kwik_cash` | Level 5 main mission 6 | Force the Bandit, reach and evade the first police pursuit, locate and destroy the Armored Truck without a destroy timer, return, evade the second pursuit within 45 seconds, and complete the final return and conversation. |
| `lab_coat_caper` | Level 6 main mission 3 | Follow Frink's Hover Car through the declared repeated route to the observatory while satisfying the separation policy. |
| `long_black_probes` | Level 7 main mission 2 | Require the owned Zombie Car, enter it, travel to the playground, and follow the alien probe to the power plant without violating separation or vehicle-health policy. |

<!-- markdownlint-enable MD013 -->

The required costume in `ketchup_logic` is a precondition, not a second player
character. The forced pickup and Bandit are mission placements and do not grant
ownership. The required Zombie Car checks persistent ownership before mission
activation and cannot be replaced by the current arbitrary vehicle.

Inactive or commented source-stage rows are not imported as mission steps. Only
active objective, condition, timer, target, and transition evidence becomes the
public ordered contract.

## Verified fourth street-race slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Level and policy | Route contract |
| :--- | :--- | :--- |
| `mansion_power_plant_time_trial_level_04` | Level 4 time trial | Complete three laps through the mansion grounds, power-plant passage, and Stonecutters route within 131 seconds. |
| `kwik_e_mart_time_trial_level_07` | Level 7 time trial | Complete five counter-clockwise laps of the store, gas-station, and donut-shop block within the seventy-second stage timer. |

<!-- markdownlint-enable MD013 -->

The Level 7 race uses five laps. A stale descriptive summary that lists three
laps cannot override executable route evidence. Both races require dense
checkpoint order, declared direction, exact timer, reset transforms, vehicle
failure policy, and deterministic finish transition.

## Verified fourth location slice

`kwik_e_mart` is one canonical indoor location available in Levels 1, 4, and 7.
`spook_e_mart` is the Level 7 presentation alias and variant. Interior portals,
gags, mission entry, costume interaction, and progression remain level-scoped.

`krusty_burger` is one canonical exterior location family used across all seven
levels. Multiple physical restaurants are placement identities referencing the
same location definition and level-specific site rows. `zombie_burger` is a
Level 7 presentation alias, not a new location identity.

A location family and a physical site are distinct. Validation rejects a mission
that references an ambiguous family when an exact site placement is required.

## Verified fourth campaign and index slice

The seven historic level pages, their aggregate page, the Level 6 vehicle page,
and the source main page are census or navigation evidence. Runtime campaign,
chapter, vehicle, mission, race, collectible, and location identities are owned
by the catalog and the
[open sandbox chapter runtime](open-sandbox-chapter-runtime.md) specification.
Source indexes never become duplicate primary assets or player-facing level
states.

The Level 7 sound page in this slice contains no independently identified sound
rows. It therefore creates no audio definition. Level audio remains owned by the
level audio profile and exact role records.

## Verified fifth character slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Aliases | Required contract |
| :--- | :--- | :--- |
| `mayor_quimby` | `quimby` | Non-playable civic character with cinematic, billboard, vehicle-presentation, and level-scoped ambient roles. |
| `milhouse_van_houten` | `milhouse` | Non-playable mission character and Levels 1 through 6 time-trial host; race-host and story placements share one character identity. |
| `moe_szyslak` | `moe` | Non-playable talkable and mission character with house, tavern, ambient, and story placements. |
| `charles_montgomery_burns` | `mr_burns`, `monty_burns`, `burns` | The existing canonical identity is reaffirmed for intercom, mission, cinematic, and Level 7 interaction roles. |
| `waylon_smithers` | `mr_smithers`, `smithers` | Non-playable mission, ambient, driver, cinematic, and Level 7 bonus-mission character. |
| `ned_flanders` | `ned` | Non-playable mission, talkable, house-interaction, gag, and ambient character. |
| `nerd` | none | Non-playable mission and race-driver archetype with exact Level 2 and Level 3 story placements. |
| `otto_mann` | `otto` | Non-playable mission character, bus driver, and level-scoped ambient placement. |

<!-- markdownlint-enable MD013 -->

The minor-character and non-story-character indexes are query projections over
canonical definitions and placement capabilities. They do not create aggregate
characters or duplicate dialogue owners. The full placement rules follow
<!-- markdownlint-disable-next-line MD013 -->
[Ambient population and named-character runtime](ambient-population-and-named-character-runtime.md).

## Verified fifth vehicle slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Aliases | Verified context | Required rule |
| :--- | :--- | :--- | :--- |
| `milk_truck` | none | Level 6 mission target and completion override | Mission destruction does not grant ownership; completion-only retrieval uses its explicit override. |
| `mini_school_bus` | none | Level 1 traffic and completion override | Traffic access is temporary and does not grant ordinary retrieval ownership. |
| `minivan` | none | Level 1 traffic and completion override | Native traffic and completion-override retrieval reference one definition. |
| `monorail_car` | none | Level 2 secret vehicle | World access is temporary and excluded from the five counted progression vehicles. |
| `obliteratatron_big_wheel_truck` | `obliteration_big_wheel_truck`, `monster_truck` | Level 5 secret vehicle | All three names resolve to one secret-vehicle definition and placement family. |
| `mr_burns_limo` | `burns_limo` | Level 7 bonus-mission reward | The accepted reward grants persistent retrieval exactly once. |
| `mr_plow` | none | Level 2 purchase for 200 coins | Purchase ownership gates the declared required-vehicle mission and normal retrieval. |
| `nerd_car` | none | Level 3 purchase for 250 coins and race opponent | Purchase and opponent placements share one definition. |
| `nonuplets_minivan` | `shelbyville_nonuplets_van` | Completion-override vehicle | No ordinary traffic, purchase, reward, or secret placement grants ownership. |
| `nuclear_waste_truck` | none | Level 4 traffic and completion override | The traffic vehicle is distinct from the nuclear-waste mission payload. |
| `open_wheel_race_car` | none | Level 7 street-race reward | Completing the declared race set grants persistent retrieval. |
| `pickup_road_vehicle` | `pickup` | Traffic in Levels 1, 3, and 6 plus completion override | Traffic access and static prop placements do not grant ownership. |
| `cletus_pickup_truck` | `pickup_truck` | Level 1 bonus reward and mission vehicle | Distinct from `pickup_road_vehicle`; reward ownership and forced mission use share one definition. |
| `pizza_van` | none | Level 2 traffic and mission target plus completion override | Distinct from the purchasable surveillance vehicle despite related presentation. |

<!-- markdownlint-enable MD013 -->

The vehicle browser, locked rows, health, repair, completion override, delivery,
and mission restrictions follow
<!-- markdownlint-disable-next-line MD013 -->
[Vehicle retrieval and phone-booth runtime](vehicle-retrieval-and-phone-booth-runtime.md).

## Verified fifth mission slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Level and class | Ordered contract |
| :--- | :--- | :--- |
| `milking_the_pigs` | Level 6 bonus mission | Hit Chief Wiggum's vehicle and accept the evidence folder within 120 seconds, complete the Snake conversation, locate and destroy the Milk Truck within 180 seconds, return, and grant the Bandit once. |
| `monkey_see_monkey_doh` | Level 2 main mission 6 | Require the owned Mr. Plow, travel to the research center, collect thirty declared monkeys within 240 seconds, return, complete the Dr. Nick interaction, and reach the final blender target. |
| `nerd_race_queen` | Level 3 main mission 1 | Force Comic Book Guy's vehicle, win the declared race against the Nerd Car, reach the comic target, return within 90 seconds, and complete the final interaction. |
| `never_trust_a_snake` | Level 5 main mission 5 | Hit the garbage truck and accept five emitted targets within 255 seconds, collect twenty-five declared garbage targets without a timer, reach the DMV, complete the Snake interaction and interior transition, and accept the folder target. |
| `office_spaced` | Level 1 main mission 3 | Require the Plow King, reach Lenny, reach the Smithers pursuit start within 90 seconds, and destroy Smithers' vehicle before its race-condition destination. |
| `operation_hellfish` | Level 3 main mission 4 | Require the School Bus, reach the observatory and first target, then destroy three declared sedans in successive 120-second, 90-second, and 75-second stages. |
| `petty_theft_homer` | Level 1 main mission 2 | Collect the ordered personal-item targets under their declared 40-second or untimed policies, complete the Barney interaction, return to Ned, and complete the final conversation. |

<!-- markdownlint-enable MD013 -->

A zero timer declaration in this verified slice means untimed. It is not a
zero-second timeout. Required and forced vehicles remain separate activation
policies and never grant ownership.

## Verified fifth street-race slice

`motorway_checkpoint_level_02` is the Level 2 checkpoint race. It has twelve
dense ordered checkpoints, starts near the town-hall district, ends at the east
motorway exit, requires first place against Lisa's vehicle, a sports car, and a
taxi, and fails on declared player-vehicle destruction or out-of-vehicle
timeout.
The finish conversation is presentation after race acceptance.

## Verified fifth location and payload slice

`moes_tavern` is one canonical Level 2 and Level 5 indoor location. Exterior
portal placements, interior interactions, mission targets, gags, ambience, and
music state reference the same location identity.

`nuclear_waste` is a mission payload item, not a vehicle or generic collectible.
Its definition owns attachment, collision sensitivity, detachment, destruction,
delivery-zone acceptance, retry, and presentation policy. A vehicle carrying the
payload remains a separate canonical vehicle instance.

The detailed payload lifecycle follows the
<!-- markdownlint-disable-next-line MD013 -->
[mission, interaction, interior, and notoriety runtime](mission-interaction-and-notoriety-runtime.md).

## Verified fifth system and index slice

The mission index, minor-character index, non-story-character index, mod index,
and modification index are coverage or navigation evidence. They do not create
aggregate runtime assets. Accepted mod packages project through
[Mod package overlay runtime](mod-package-overlay-runtime.md).

The music census resolves through
[Music state and transition runtime](music-state-and-transition-runtime.md).
The pedestrian census resolves through
<!-- markdownlint-disable-next-line MD013 -->
[Ambient population and named-character runtime](ambient-population-and-named-character-runtime.md).
The phone-booth census resolves through
<!-- markdownlint-disable-next-line MD013 -->
[Vehicle retrieval and phone-booth runtime](vehicle-retrieval-and-phone-booth-runtime.md).

The newspaper page contributes no independently identified gameplay definition
in
this slice. Historical oddity and unused-behavior lists are negative
compatibility
or review evidence; they do not become successful gameplay features unless an
intentional behavior has its own verified contract.

## Verified sixth character and archetype slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Aliases | Required contract |
| :--- | :--- | :--- |
| `principal_seymour_skinner` | `principal_skinner`, `seymour_skinner`, `skinner` | One non-playable character owns mission-opponent, bonus-mission-giver, school, ambient, driver, and cinematic placements. |
| `professor_frink` | `frink` | The existing scientist identity is reaffirmed for mission-giver, driver, observatory, bonus-reward, ambient, and cinematic placements. |
| `reverend_lovejoy` | `lovejoy` | Non-playable named character with level-scoped ambient and presentation placements. |
| `horatio_mccallister` | `sea_captain` | The existing canonical identity is reaffirmed for Squidport ambience and the `princi_pal` interaction. |
| `snake_jailbird` | `snake` | One non-playable character owns mission-giver, target, driver, ambient, and dialogue placements across Levels 2, 3, 5, 6, and 7. |
| `mayor_quimby` | `quimby` | The existing civic character identity owns cutscene, billboard, vehicle-presentation, and ambient references. |
| `waylon_smithers` | `mr_smithers`, `smithers` | The existing character identity owns mission, driver, bonus-mission, ambient, and cinematic placements. |

<!-- markdownlint-enable MD013 -->

`Skeleton` identifies a generic Level 7 ambient archetype. It uses a population
archetype and placement rows, not a named character, dialogue owner, or save
identity. Named-character and ambient-archetype behavior follows
<!-- markdownlint-disable-next-line MD013 -->
[Ambient population and named-character runtime](ambient-population-and-named-character-runtime.md).

## Verified sixth vehicle slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Aliases | Verified context | Required rule |
| :--- | :--- | :--- | :--- |
| `planet_hype_50s_car` | none | Level 6 secret vehicle | Temporary world access only; completion override does not create ordinary ownership. |
| `plow_king` | none | Level 1 purchase for 150 coins | Purchase ownership is required by the declared mission gate. |
| `police_car` | none | Level 5 purchase for 425 coins and mission-forced placement | Ownership, forced use, driver presentation, and pursuit placement remain separate rows. |
| `hover_car` | `professor_frinks_hover_car`, `frinks_hover_car` | Level 5 bonus reward and later forced or target placements | The bonus reward grants ownership once; other placements do not replay it. |
| `quad_bike` | `atv` | Level 4 secret vehicle | One trailer-park placement grants temporary access. |
| `rc_buggy` | `r_c_buggy` | Level 7 secret vehicle | One roof placement grants temporary access. |
| `red_brick_car` | `brick_car` | Development-only vehicle | Excluded from shipping ownership, traffic, secret, mission, race, and completion-override queries. |
| `suv` | none | Traffic in Levels 4 and 5 | Native traffic access does not grant persistent ownership. |
| `school_bus` | none | Level 3 purchase for 300 coins and mission vehicle | Purchase, forced use, required use, and Otto driver presentation share one definition. |
| `sedan_level_02` | none | Level 2 street-race reward | Distinct persistent reward identity. |
| `sedan_level_03` | `skinners_sedan` | Level 3 bonus-mission reward and Skinner driver placement | Distinct from every other sedan definition. |
| `sedan_a` | none | Unused development traffic definition | Excluded from normal shipping access. |
| `sedan_b` | none | Level 2 traffic | Traffic and completion override remain separate from ownership. |
| `speed_rocket` | none | Level 1 secret vehicle | Temporary world access only. |
| `sports_car_a` | none | Traffic in Levels 2 and 3 plus race placements | Traffic, opponent, prop, and completion-override rows share one definition. |
| `sports_car_b` | none | Level 5 traffic | Traffic access does not grant persistent ownership. |

<!-- markdownlint-enable MD013 -->

The complete 42-vehicle persistent roster, seven secret placements, seven
traffic
rosters, completion override, sedan identity boundary, drivers, and development
exclusions follow
[Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md).

## Verified sixth mission slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Level and class | Ordered contract |
| :--- | :--- | :--- |
| `s_m_r_t` | Level 1 main mission 1 | Accept the science-project target, reach Skinner's start, win the untimed route race, enter Springfield Elementary, talk to Lisa, and complete the final dialogue. |
| `princi_pal` | Level 3 bonus mission | Force Skinner's Sedan; complete the 40-second opening errand, untimed laundry target, 30-second restaurant travel, meal interaction, 45-second casino travel, cream interaction, and 35-second arcade return; then grant Skinner's Sedan once. |
| `slithery_sleuthing` | Level 3 main mission 5 | Force the Police Car, follow Snake over four route waypoints and accept three emitted targets, satisfy separation and vehicle conditions, reach the casino within 60 seconds, and complete Wiggum's final sequence. |
| `redneck_roundup` | Level 4 main mission 2 | Follow Cletus over eight route waypoints and accept seven emitted objects without violating the separation policy; no timer applies. |
| `return_of_the_nearly_dead` | Level 4 main mission 5 | Reach the school within 30 seconds, complete Nelson's interaction, follow the sedan and accept ten pills, reach the false destination within 90 seconds, lose the tail within 90 seconds, reach Grampa within 150 seconds, collect the interior caffeine target, return, and complete the cinematic transition. |
| `set_to_kill` | Level 6 main mission 6 | Require purchase of the Globex Super Villain Car, reach Krustylu, destroy and accept twenty-five laser-stand targets within 100 seconds, return within 50 seconds, and complete the Krusty interaction. |
| `rigor_motors` | Level 7 main mission 1 | Talk to Ned within 30 seconds, collect the first-aid kit, reach and collect the boards within the declared 15-second travel stage, reach Moe within 15 seconds, collect the chainsaw, and return home within 40 seconds. |
| `pocket_protector` | Level 7 main mission 3 | Force the Hover Car, acquire the nuclear-waste payload within 120 seconds, reach the playground within 100 seconds while retaining vehicle and payload, and destroy the boss target within 10 seconds while preserving the payload policy. |

<!-- markdownlint-enable MD013 -->

A pre-mission purchase or ownership gate is activation policy, not a duplicate
mission objective. A zero timer declaration means untimed. Forced, required, and
owned vehicles never collapse into one acquisition state.

## Verified sixth race slice

<!-- markdownlint-disable MD013 -->

| Canonical identity | Verified route contract |
| :--- | :--- |
| `rich_district_2_circuit_level_04` | Three laps; six AI route waypoints and five dense player checkpoints; opponents are Apu in the Longhorn, the Nuclear Waste Truck, and the Garbage Truck; first place required; no timer. |
| `squidport_checkpoint_level_03` | Five ordered checkpoints against Marge in the Canyonero, Sports Car A, and the road Pickup; first place required. |
| `squidport_tourist_resort_time_trial_level_06` | Two laps through eight ordered checkpoints within 115 seconds. |
| `squidport_2_checkpoint_level_06` | Six ordered checkpoints against Homer in the canonical Level 7 sports-car placement; first place required. |

<!-- markdownlint-enable MD013 -->

The race-objective index contributes the race-class vocabulary but creates no
race asset. Exact route, crossing, opponent, position, failure, reset, finish,
and race-set reward semantics follow
[Race route and opponent runtime](race-route-and-opponent-runtime.md).

## Verified sixth location slice

`simpson_house` is one canonical location family with Levels 1, 4, and 7 world
variants. `simpsons_house` and punctuation variants are aliases. Interiors,
mission starts, gags, family placements, and exterior sites remain level-scoped.

`springfield_elementary` is one canonical school location family. Exact
exterior,
interior, mission-door, character, gag, and race-finish placements are separate
rows owned by their level and Runtime Data Layers.

`frink_observatory` is one canonical interior location available in Levels 3 and
6. Costume interaction, alarm gag, mission starts, story interactions, and level
presentation reference the same definition with level-scoped placements.

## Verified sixth frontend, index, and compatibility slice

The player-vehicle, road-vehicle, and secret-vehicle pages are census evidence
owned by
[Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md).
They do not create aggregate vehicle definitions or grant ownership.

The Scrap Book page resolves through
[Frontend shell and menu runtime](frontend-shell-and-menu-runtime.md). Its
`game_stats` mode aggregates accepted campaign progress and movies. Its
`open_book` mode presents level-separated missions, clothing, persistent
vehicles, and collector cards. Locked presentation never changes progression.

The sedan disambiguation page contributes aliases and collision tests only. It
creates no generic `sedan` primary asset. Prerelease material, the Red Brick
Car,
Sedan A, unused variants, prototype screenshots, and abandoned placements are
negative compatibility evidence under the
[runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md).
They are not successful shipping behavior.

## Known limits

This specification fixes the catalog architecture and the six verified coverage
slices. It does not claim that every remaining character, vehicle, mission,
location, reward, costume, quote, interaction, or bonus-mode record has already
been entered. New coverage extends these schemas and invariants; it does not
introduce a parallel catalog pattern.

# Unreal gameplay content catalog

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content
  catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Gameplay census, presentation, and development-content
  boundary](gameplay-census-presentation-and-development-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Legacy runtime identity
  normalization](legacy-runtime-identity-normalization.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Event-driven music and
  ambience](../../adr/unreal/runtime/event-driven-music-and-ambience.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mass Entity ambient
  population](../../adr/unreal/runtime/mass-entity-ambient-population.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Transactional phone-booth vehicle retrieval](../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD013 -->
- [Validated game-feature mod
  overlays](../../adr/unreal/runtime/validated-game-feature-mod-overlays.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Driving, traffic, and vehicle behavior
  parity](../../adr/gameplay/vehicles/driving-traffic-and-vehicle-ai.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Unreal manifest and package
  taxonomy](../../adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md)
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Unified open world and chapter projection](../../adr/pipeline/unreal/unified-open-world-and-chapter-projection.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD013 -->
- [Native world partition and data
  layers](../../adr/pipeline/unreal/world-partition-and-data-layer-import.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native art authoring, style, and asset validation
  contract](native-art-authoring-style-and-asset-validation-contract.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical core-design and dialogue evidence
  normalization](historical-core-design-and-dialogue-evidence-normalization.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Progression, collectibles, cheats, and
  credits](progression-collectibles-and-cheats.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Flying hazard and projectile
  runtime](flying-hazard-and-projectile-runtime.md)

## Purpose

This specification defines the canonical Unreal representation for gameplay
content. It fixes identity, asset placement, schemas, loading, progression,
validation, and verification for characters, vehicles, missions, locations,
rewards, costumes, dialogue events, races, bonus modes, billboards, collector
cards, ambient and interactive gags, interiors, pedestrians, and presentation
catalogs.

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
│   ├── Billboards
│   ├── CollectorCards
│   ├── Gags
│   ├── Interiors
│   ├── PresentationCatalogs
│   ├── BonusModes
│   └── Tables
│       ├── Aliases
│       ├── Dialog
│       ├── MissionSteps
│       ├── RaceCheckpoints
│       ├── VehicleTuning
│       ├── BillboardPlacements
│       ├── CollectorCardPlacements
│       ├── GagBindings
│       ├── InteriorPresentations
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

<!-- markdownlint-disable MD013 -->

- **Asset family:** Root catalog
  - **Primary asset type:** `SharCatalog`
  - **Object name:** `DA_SHAR_GameplayCatalog`
- **Asset family:** Campaign
  - **Primary asset type:** `SharCampaign`
  - **Object name:** `DA_Campaign_<canonical_id>`
- **Asset family:** Level
  - **Primary asset type:** `SharLevel`
  - **Object name:** `DA_Level_<canonical_id>`
- **Asset family:** Character
  - **Primary asset type:** `SharCharacter`
  - **Object name:** `DA_Character_<canonical_id>`
- **Asset family:** Vehicle
  - **Primary asset type:** `SharVehicle`
  - **Object name:** `DA_Vehicle_<canonical_id>`
- **Asset family:** Mission
  - **Primary asset type:** `SharMission`
  - **Object name:** `DA_Mission_<canonical_id>`
- **Asset family:** Location
  - **Primary asset type:** `SharLocation`
  - **Object name:** `DA_Location_<canonical_id>`
- **Asset family:** Population
  - **Primary asset type:** `SharPopulation`
  - **Object name:** `DA_Population_<canonical_id>`
- **Asset family:** Music profile
  - **Primary asset type:** `SharMusicProfile`
  - **Object name:** `DA_MusicProfile_<canonical_id>`
- **Asset family:** Music composition
  - **Primary asset type:** `SharMusicComposition`
  - **Object name:** `DA_Music_<canonical_id>`
- **Asset family:** Reward
  - **Primary asset type:** `SharReward`
  - **Object name:** `DA_Reward_<canonical_id>`
- **Asset family:** Costume set
  - **Primary asset type:** `SharCostumeSet`
  - **Object name:** `DA_CostumeSet_<canonical_id>`
- **Asset family:** Billboard
  - **Primary asset type:** `SharBillboard`
  - **Object name:** `DA_Billboard_<canonical_id>`
- **Asset family:** Collector card
  - **Primary asset type:** `SharCollectorCard`
  - **Object name:** `DA_CollectorCard_<canonical_id>`
- **Asset family:** Gag
  - **Primary asset type:** `SharGag`
  - **Object name:** `DA_Gag_<canonical_id>`
- **Asset family:** Interior presentation
  - **Primary asset type:** `SharInteriorPresentation`
  - **Object name:** `DA_InteriorPresentation_<canonical_id>`
- **Asset family:** Presentation catalog
  - **Primary asset type:** `SharPresentationCatalog`
  - **Object name:** `DA_PresentationCatalog_<canonical_id>`
- **Asset family:** Bonus mode
  - **Primary asset type:** `SharBonusMode`
  - **Object name:** `DA_BonusMode_<canonical_id>`

<!-- markdownlint-enable MD013 -->

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

- **Field:** `CanonicalId`
  - **Type:** `FName`
  - **Contract:** Stable domain identity; never localized.
- **Field:** `DisplayName`
  - **Type:** `FText`
  - **Contract:** Localizable player-facing name.
- **Field:** `Aliases`
  - **Type:** `TArray<FName>`
  - **Contract:** Alternate lookup names resolving to this identity.
- **Field:** `SourcePackageIds`
  - **Type:** `TArray<FName>`
  - **Contract:** Approved deterministic package references.
- **Field:** `ContentTags`
  - **Type:** `FGameplayTagContainer`
  - **Contract:** Capabilities and classifications, never identity.
- **Field:** `RequiredDefinitions`
  - **Type:** soft primary-asset references
  - **Contract:** Definitions that must resolve before activation.
- **Field:** `RevisionToken`
  - **Type:** `FString`
  - **Contract:** Deterministic generated-data revision.
- **Field:** `ValidationProfile`
  - **Type:** `FName`
  - **Contract:** Exact validator contract for the asset family.

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

- **Bundle:** `Definition`
  - **Includes:** Definition object and generated rows
  - **Allowed load point:** Catalog validation and save migration
- **Bundle:** `Gameplay`
  - **Includes:** Collision, physics, objective, AI, and interaction assets
  - **Allowed load point:** Available sandbox, chapter unlock, or active mission
    scope
- **Bundle:** `Presentation`
  - **Includes:** Meshes, materials, animation, UI, and icons
  - **Allowed load point:** Visible or previewed content
- **Bundle:** `Audio`
  - **Includes:** Dialogue, music, vehicle, and interaction sounds
  - **Allowed load point:** Audible content scope
- **Bundle:** `Cinematic`
  - **Includes:** Sequences, media, cameras, and cinematic-only assets
  - **Allowed load point:** Active cinematic
- **Bundle:** `EditorReview`
  - **Includes:** Review-only references and conformance evidence
  - **Allowed load point:** Editor and automated review only

<!-- markdownlint-enable MD013 -->

`Definition` is always the first bundle loaded. Runtime code requests only the
additional bundles required by the current role. A mission must not preload all
presentation or audio assets for unrelated entities.

## Frontend screen and flow definitions

`USharFrontendScreenDefinition` is a catalog-reachable primary asset containing
canonical screen identity, Common UI layer, widget class, view-model schema,
semantic action set, entry and exit policy, required bundles, focus,
accessibility, failure, and feature ownership.

`USharFrontendFlowDefinition` contains validated navigation edges, boot-task
graph, loading-presentation choices, modal policies, local bonus-mode setup
routes, and feature-overlay boundaries. Screen and flow identity never derive
from widget names, integer message values, package order, or platform-specific
source files.

Gallery, options, save-browser, language, legal, loading, media, and replay
screens reference canonical catalog and domain identities. Presentation assets
may vary by platform, locale, accessibility, theme, and quality without changing
screen or command identity.

The detailed runtime contract follows the
<!-- markdownlint-disable-next-line MD013 -->
[frontend screen flow and settings
runtime](frontend-screen-flow-and-settings-runtime.md).

## Character definition

`USharCharacterDefinition` extends the shared definition with:

<!-- markdownlint-disable MD013 -->

- **Field:** `CharacterRole`
  - **Contract:** Playable, non-playable, mission giver, ambient, or passenger.
- **Field:** `PlayableLevelIds`
  - **Contract:** Levels in which player control is permitted.
- **Field:** `PresenceLevelIds`
  - **Contract:** Levels in which the character may be placed.
- **Field:** `DefaultVehicleId`
  - **Contract:** Optional canonical vehicle identity.
- **Field:** `CostumeSetId`
  - **Contract:** Optional canonical costume-set identity.
- **Field:** `QuoteTable`
  - **Contract:** Soft reference to ordered quote-event rows.
- **Field:** `DefaultPresentationId`
  - **Contract:** Complete prepared base-model presentation identity.
- **Field:** `PresentationVariantTable`
  - **Contract:** Complete outfit, costume, or prop-bearing model variants.
- **Field:** `SemanticPreparationManifest`
  - **Contract:** FBX-owned UV, texture-region, eye-layer, rig-preservation, and
    variant evidence.
- **Field:** `EyeProfileId`
  - **Contract:** Prepared sclera, pupil, upper-eyelid, and lower-eyelid
    ownership.
- **Field:** `SkeletalMesh`
  - **Contract:** Soft default complete-model presentation reference.
- **Field:** `AnimationClass`
  - **Contract:** Existing animation Blueprint consumer; no retargeting change
    is implied.
- **Field:** `VoiceProfileId`
  - **Contract:** Canonical audio routing identity.

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

- **Field:** `LifecycleState`
  - **Contract:** Active, inaccessible, or unused.
- **Field:** `NativeLevelIds`
  - **Contract:** Levels where the vehicle naturally exists.
- **Field:** `AcquisitionTable`
  - **Contract:** Soft reference to ordered acquisition rows.
- **Field:** `DriverCharacterId`
  - **Contract:** Optional canonical driver identity.
- **Field:** `TuningRowId`
  - **Contract:** Required vehicle-tuning row.
- **Field:** `Mesh`
  - **Contract:** Soft skeletal or static mesh reference.
- **Field:** `AnimationProfileId`
  - **Contract:** Doors, wheels, suspension, damage, and special effects.
- **Field:** `AudioProfileId`
  - **Contract:** Engine, horn, collision, and special audio.
- **Field:** `DamageProfileId`
  - **Contract:** Health, visual damage, destruction, and repair behavior.
- **Field:** `TrafficProfileId`
  - **Contract:** Optional traffic and pursuit behavior.

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

- **Field:** `ChapterId`
  - **Contract:** Canonical narrative chapter identity.
- **Field:** `SourceLevelAlias`
  - **Contract:** Optional historic conversion alias only.
- **Field:** `SequenceOrdinal`
  - **Contract:** Stable sequence position within its mission family.
- **Field:** `MissionClass`
  - **Contract:** Story, bonus, race, wager, taxi, boss, or another registered
    side activity.
- **Field:** `GiverCharacterId`
  - **Contract:** Optional mission-giver identity.
- **Field:** `PlayableCharacterId`
  - **Contract:** Required or default controlled-character identity.
- **Field:** `OfferDefinitionId`
  - **Contract:** Optional world offer, marker, dialogue, and availability
    definition.
- **Field:** `PreviousMissionId`
  - **Contract:** Optional progression predecessor.
- **Field:** `NextMissionId`
  - **Contract:** Optional progression successor.
- **Field:** `StageTable`
  - **Contract:** Required ordered mission-stage table.
- **Field:** `ConditionDefinitionIds`
  - **Contract:** Ordered required, optional, failure, and recovery condition
    definitions.
- **Field:** `PresentationProfileId`
  - **Contract:** Mission offer, conversation, marker, HUD, camera, and
    transition presentation.
- **Field:** `BonusObjectiveIds`
  - **Contract:** Optional independently evaluated objective definitions.
- **Field:** `BossEncounterId`
  - **Contract:** Optional typed encounter definition for a boss mission.
- **Field:** `RewardId`
  - **Contract:** Optional completion reward.
- **Field:** `CompletionTransition`
  - **Contract:** Unlock, chapter transition, world expansion, ending, or none.
- **Field:** `WorldLayerSetId`
  - **Contract:** Required world and data-layer composition.

Mission identity is independent of the world actor that starts it. A mission
may move or gain additional entry points without changing its save key.

## Mission-stage rows

`FSharMissionStageRow` contains:

<!-- markdownlint-disable MD013 -->

- **Field:** `MissionId`
  - **Contract:** Owning mission identity.
- **Field:** `StageId`
  - **Contract:** Stable mission-scoped stage identity.
- **Field:** `SequenceOrdinal`
  - **Contract:** Dense zero-based order within the mission.
- **Field:** `ObjectiveKind`
  - **Contract:** One value from the controlled objective taxonomy.
- **Field:** `ObjectivePolicyId`
  - **Contract:** Required objective-specific runtime policy identity.
- **Field:** `ConditionIds`
  - **Contract:** Ordered required, failure, optional, and recovery conditions.
- **Field:** `ParticipantBindingIds`
  - **Contract:** Characters, vehicles, AI, payloads, and world actors.
- **Field:** `RouteAndWaypointIds`
  - **Contract:** Ordered route, checkpoint, destination, and recovery
    identities.
- **Field:** `TargetIds`
  - **Contract:** Canonical entities, actors, zones, or items.
- **Field:** `RequiredCount`
  - **Contract:** Non-negative completion count.
- **Field:** `TimePolicyId`
  - **Contract:** Countdown, count-up, inherited, added, paused, or untimed
    policy.
- **Field:** `ForcedVehicleId`
  - **Contract:** Optional vehicle required for this stage.
- **Field:** `OpponentIds`
  - **Contract:** Ordered race, chase, or avoid participants.
- **Field:** `LocationId`
  - **Contract:** Canonical location or route identity.
- **Field:** `LockRequirementIds`
  - **Contract:** Explicit vehicle, costume, reward, or progression
    requirements.
- **Field:** `LoadPlanId`
  - **Contract:** Stage-specific asset and world-composition plan.
- **Field:** `CheckpointPolicyId`
  - **Contract:** Checkpoint creation and restore behavior.
- **Field:** `SuccessTransition`
  - **Contract:** Declared successor or mission completion.
- **Field:** `FailureTransition`
  - **Contract:** Stage retry, checkpoint restore, mission retry, abort, or
    failure.
- **Field:** `PresentationProfileId`
  - **Contract:** HUD, camera, dialogue, countdown, music, and transition
    requests.
- **Field:** `WorldPolicyId`
  - **Contract:** Traffic, population, notoriety, safe-zone, and control policy.
- **Field:** `FinalPolicy`
  - **Contract:** Whether accepted success may terminate the mission.
- **Field:** `BonusObjectiveStartIds`
  - **Contract:** Optional objectives activated by this stage revision.

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
[mission definition, stage, and objective
runtime](mission-definition-stage-and-objective-runtime.md).
Interaction, interior, notoriety, and world-safety behavior follow the
<!-- markdownlint-disable-next-line MD013 -->
[mission, interaction, interior, and notoriety
runtime](mission-interaction-and-notoriety-runtime.md).

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

`USharRewardDefinition` contains:

<!-- markdownlint-disable MD013 -->

- **Field:** `RewardId`
  - **Contract:** Canonical reward identity.
- **Field:** `RewardKind`
  - **Contract:** Vehicle ownership, costume ownership, currency, media,
    feature, service, or another registered kind.
- **Field:** `GrantedDefinitionIds`
  - **Contract:** Ordered canonical identities granted by the reward.
- **Field:** `ProgressionPredicateId`
  - **Contract:** Mission, race, collectible, achievement, purchase, or system
    eligibility.
- **Field:** `RepeatabilityPolicy`
  - **Contract:** Permanent once, repeatable service, per-attempt, or another
    registered policy.
- **Field:** `TransactionKeyPolicy`
  - **Contract:** Deterministic exactly-once key construction.
- **Field:** `PresentationProfileId`
  - **Contract:** Unlock, purchase, preview, audio, and accessibility
    presentation.
- **Field:** `SaveSchemaId`
  - **Contract:** Durable ownership or completion projection.
- **Field:** `SupersessionPolicyId`
  - **Contract:** Replacement and mod-overlay behavior.

<!-- markdownlint-enable MD013 -->

Vehicle and costume rewards grant access to existing canonical definitions; they
never create duplicate assets or alternate save identities.

`FSharMerchandiseOfferRow` contains offer identity, reward identity, seller
role,
seller placement, chapter and progression predicates, currency kind, exact
price, stock and repeatability policy, preview presentation, and offer revision.
Seller roles are extensible catalog identities rather than a closed runtime type
switch.

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

## Billboard definition

`USharBillboardDefinition` contains:

<!-- markdownlint-disable MD013 -->

- **Field:** `BillboardId`
  - **Contract:** Canonical sign or environmental-graphic identity.
- **Field:** `PresentationAssetId`
  - **Contract:** Approved mesh, material, texture, decal, or animated
    presentation identity.
- **Field:** `LocalizationPolicyId`
  - **Contract:** Fixed graphic, localized material, text overlay, or another
    declared policy.
- **Field:** `PlacementPolicyId`
  - **Contract:** Allowed location, zone, structure, route, surface, and
    orientation rules.
- **Field:** `VariantIds`
  - **Contract:** Approved visual, chapter, damage, daypart, or rotating
    variants.
- **Field:** `InteractionPolicyId`
  - **Contract:** None, mission target, breakable, collectible-adjacent, camera
    target, or another registered policy.
- **Field:** `StreamingProfileId`
  - **Contract:** Bundle, Data Layer, HLOD, instancing, and residency policy.
- **Field:** `RightsAndApprovalState`
  - **Contract:** Import-review state that must be accepted before publication.

<!-- markdownlint-enable MD013 -->

`FSharBillboardPlacementRow` identifies billboard, world, location, zone, stable
placement, transform, surface or structure owner, chapter availability, variant,
Data Layer, and expected revisions.

Production columns such as approved, completed, placed, assigned, date, or
review
comment are not runtime booleans. They are consumed by the private import review
and only accepted semantic definitions and placements reach this catalog.

A billboard identity is independent of the number of placements. Rotating or
animated presentation uses an explicit material, state-prop, or presentation
profile; it is not inferred from a spreadsheet column or texture filename.

## Collector-card definition

`USharCollectorCardDefinition` contains:

<!-- markdownlint-disable MD013 -->

- **Field:** `CollectorCardId`
  - **Contract:** Canonical card identity.
- **Field:** `SetId`
  - **Contract:** Owning chapter or registered card-set identity.
- **Field:** `Ordinal`
  - **Contract:** Dense stable ordinal within the set.
- **Field:** `TitleTextKey`
  - **Contract:** Localized title key.
- **Field:** `DescriptionTextKey`
  - **Contract:** Localized description or trivia key.
- **Field:** `FrontPresentationId`
  - **Contract:** Card-front material, texture, mesh, or widget presentation.
- **Field:** `DetailPresentationId`
  - **Contract:** Full-detail gallery presentation.
- **Field:** `AudioProfileId`
  - **Contract:** Optional collection and gallery audio.
- **Field:** `UnlockPolicyId`
  - **Contract:** Collection and persistence rule.
- **Field:** `ProgressionContributionId`
  - **Contract:** Chapter and game-completion contribution.
- **Field:** `ReplacementPolicyId`
  - **Contract:** Overlay and supersession behavior.

<!-- markdownlint-enable MD013 -->

`FSharCollectorCardPlacementRow` contains card, world, chapter, location,
placement, transform, visibility, accessibility, collision, pickup, radar,
streaming, and save revisions.

Cards are not identified by display title, source row order, chapter text, or a
historical numeric code. Alternate titles and punctuation normalize to aliases.
One card has one durable collection key even when several descriptions or review
rows refer to it.

Placement validation requires:

- the card belongs to the declared set and chapter;
- its ordinal is unique and dense where the set requires it;
- the location and placement exist;
- the pickup is reachable through accepted gameplay;
- radar and presentation references resolve;
- collection state persists exactly once;
- gallery detail and localized text exist; and
- the card contributes to completion through one declared rule.

Card runtime, collection, save state, reward contribution, and gallery
projection
follow
<!-- markdownlint-disable-next-line MD013 -->
[Progression, collectibles, cheats, and
credits](progression-collectibles-and-cheats.md).

## Gag definition

`USharGagDefinition` contains:

- canonical gag identity;
- ambient, interactive, mission, cinematic, or another registered gag class;
- eligible chapters, levels, locations, interiors, zones, and placements;
- participant character and prop roles;
- animation catalog and choreography references;
- dialogue, audio, VFX, camera, and subtitle bindings;
- trigger, interaction, repeatability, cooldown, and completion policy;
- collision and inaccessible-presentation rules;
- progression, reward, persistence, and save behavior;
- required asset bundles;
- quality and platform policy; and
- replacement and teardown behavior.

Gags use stable semantic identities. A scene filename, animation filename,
source
line color, prose description, or historical interior table position cannot
become runtime identity.

### Ambient-gag binding

`FSharAmbientGagBindingRow` contains gag, interior or world presentation zone,
chapter, participant set, optional prop set, load bundle, playback policy, delay
or cooldown policy, weight, exclusion group, camera-visibility rule, and
expected
revisions.

An ambient gag may be looping or one-shot. Timing is expressed through typed
animation and scheduling policy rather than inserted empty source frames.
Temporary chairs, handheld props, or other set dressing are explicitly owned by
the gag presentation lease and cannot duplicate authoritative interior geometry.

Ambient presentation may occur outside player-reachable space, but it still
requires valid bounds, collision separation, streaming, audio, animation, and
teardown.

### Interactive-gag binding

`FSharInteractiveGagBindingRow` contains gag, interaction definition, world or
interior placement, activation requirements, camera and input policy, animation,
audio, VFX, participant roles, repeatability, cooldown, completion, progression,
reward, persistence, and cancellation behavior.

An interactive gag commits completion only through the progression transaction.
Animation, dialogue, VFX, or a source completion column cannot grant completion.

Gag completion and persistence follow
<!-- markdownlint-disable-next-line MD013 -->
[Progression, collectibles, cheats, and
credits](progression-collectibles-and-cheats.md).
Character animation follows
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
[Character animation clip catalog and vehicle-handoff choreography runtime](character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md). <!-- markdownlint-disable-line MD013 -->

## Interior-presentation definition

`USharInteriorPresentationDefinition` contains:

- canonical interior and owning structure identities;
- portal, door, transition, entry, exit, and safe-spawn identities;
- chapter and progression availability;
- world, Level Instance, Data Layer, streaming, and bundle policy;
- ambient and interactive character placements;
- ambient and interactive gag bindings;
- temporary presentation props;
- camera-cut, loading, audio, reverb, lighting, and material profiles;
- player-reachable and presentation-only zones;
- collision and navigation policy;
- mission and collectible references; and
- teardown and failure behavior.

`FSharInteriorCharacterPlacementRow` identifies interior, stable placement,
character definition, role, ambient animation or behavior, dialogue profile,
chapter availability, mission exclusions, interaction policy, and expected
revisions.

A character model, gag, dialogue line, or prop may appear in several interiors
through separate placement rows without creating duplicate character or asset
identities.

Interior transitions and interaction authority follow
<!-- markdownlint-disable-next-line MD013 -->
[Mission, interaction, interior, and notoriety
runtime](mission-interaction-and-notoriety-runtime.md).

## Presentation-catalog definition

`USharPresentationCatalogDefinition` groups content for one bounded user-facing
surface such as a chapter scrapbook, reward gallery, card browser, vehicle
preview, costume browser, or completion summary.

It declares:

- catalog and surface identities;
- ordered registered content families;
- chapter and progression scope;
- visible locked, unlocked, purchased, collected, and completed states;
- preview and detail presentation references;
- localized title and description keys;
- sorting and grouping policy;
- completion-weight policy;
- input, camera, audio, and accessibility profiles;
- platform and local-player scope; and
- replacement and teardown behavior.

Presentation catalogs are projections of authoritative progression. Selecting,
previewing, rotating, animating, damaging, opening, or playing an item cannot
change ownership or completion.

Scrapbook, card-gallery, reward-browser, costume, vehicle, media, and statistics
surfaces consume these definitions through their dedicated UI runtimes.

## Semantic content-source normalization

Normalized Markdown, CSV, text, and JSON design evidence may establish content
facts after review. Intake follows one deterministic transaction:

1. classify the source as semantic design evidence rather than raw art or
   production administration;
1. identify the intended content family and schema;
1. normalize encoding, line endings, headings, columns, and empty cells;
1. separate product semantics from production approval, completion, assignment,
   date, and review metadata;
1. normalize canonical identities and aliases;
1. resolve chapters, levels, locations, interiors, participants, assets,
   rewards,
   placements, dialogue, animation, and progression references;
1. reject ambiguous, duplicate, contradictory, or incomplete rows;
1. generate typed definitions and rows in deterministic order;
1. validate the complete reference graph and required native asset bundles;
1. compare counts and identities with accepted normalized manifests; and
1. publish one catalog revision atomically.

The runtime never opens historical CSV, Markdown, text, or office documents.
Generated Data Assets and Data Tables are the only catalog inputs.

## Content-family reconciliation

A broad content list and a family-specific table may refer to the same entity.
Reconciliation uses stable semantic identity and records:

- source fact identities;
- canonical definition identity;
- accepted aliases;
- family and role;
- chapter and location scope;
- expected native assets;
- placement and progression references;
- conflicts and resolution;
- accepted omissions; and
- terminal result.

One source row does not automatically create one runtime object. Repeated rows,
review variants, chapter placements, and completion-tracking copies may collapse
into one definition plus several typed placement or availability rows.

Exact duplicate documents and old/new folder copies collapse to one fact set by
content digest and semantic identity. Folder labels, apparent revision age, file
length, and repeated placement do not determine authority. Changed revisions are
compared field by field and record accepted, adapted, superseded, rejected, or
unresolved results before one catalog revision can publish.

Legacy master content lists may contribute candidate character, vehicle, reward,
costume, billboard, gag, placement, role, and availability facts. Approval,
completion, in-game, assignment, reference-review, and placement-status columns
remain production metadata. Historical counts, row membership, ordering, and
status flags cannot create visibility, ownership, unlock, placement, or runtime
availability.

### Authored copy and environmental text

Historical loading headlines and bylines, interior text, generic signage,
billboard copy, and similar authored rows normalize into repository-owned
localization and presentation data. Every accepted row declares:

- one stable text identity and content family;
- one loading, interior, sign, billboard, UI, or world-presentation owner;
- locale, accessibility, content-filter, and fallback policy;
- optional chapter, location, interior, placement, or presentation references;
- source-review and rights state in private provenance only; and
- one accepted revision with native localization and presentation read-back.

Raw historical wording, source row order, episode or tape references, approval,
completion, review, and replacement fields do not become public specification
text or runtime identity. A loading-text row cannot define chapter progression;
an
interior-text row cannot create an interior; and a signage row cannot create a
world placement. Those relationships require existing canonical owners.

External mission and story proposal sets follow
<!-- markdownlint-disable-next-line MD013 -->
[Historical core-design and dialogue evidence
normalization](historical-core-design-and-dialogue-evidence-normalization.md).
Candidate chapter, character, vehicle, landmark, location, interior, mission,
boss, race, collectible, dialogue, camera, audio, and presentation facts are
reconciled individually against current canonical definitions. Conflicting draft
counts, fixed per-level ownership, proposal ordering, questions, comments, legal
requests, and stakeholder workflow state cannot create or replace catalog rows.
Unresolved proposals publish no partial content family or placement graph.

## Production metadata boundary

The following fields remain import-review metadata unless a separate public
product contract explicitly owns a safe semantic subset:

- approval state;
- completion state;
- assignment or owner;
- milestone;
- date;
- review comment;
- source-workstation or source-folder state;
- source episode or reference-review state;
- art-production status;
- placement-complete status; and
- free-form production notes.

They do not become runtime progression, visibility, ownership, unlock, purchase,
collection, interaction, or placement state.

## Bonus-mode definition

`USharBonusModeDefinition` contains mode rules, eligible characters, eligible
vehicles, map unlock predicates, route definitions, scoring policy, and result
persistence. Bonus modes use separate maps and progression keys but reference
the same canonical character and vehicle definitions as the main game.

## Verified initial character slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `abraham_simpson`
  - **Aliases:** `abe_simpson`, `grampa`
  - **Required contract:** Non-playable mission giver; present across several
    levels; mission roles remain level-scoped.
- **Canonical identity:** `agnes_skinner`
  - **Aliases:** none
  - **Required contract:** Ambient and passenger-capable non-playable character.
- **Canonical identity:** `apu_nahasapeemapetilon`
  - **Aliases:** `apu`
  - **Required contract:** Playable in Level 5; present in all seven levels;
    owns the Longhorn; has event-tagged dialogue.
- **Canonical identity:** `barney_gumble`
  - **Aliases:** `barney`
  - **Required contract:** Non-playable character; car-dealer and mission roles
    are placement-specific.
- **Canonical identity:** `bart_simpson`
  - **Aliases:** `bart`
  - **Required contract:** Playable protagonist with level-scoped missions,
    costumes, owned vehicles, and event-tagged dialogue.
- **Canonical identity:** `carl_carlson`
  - **Aliases:** `carl`
  - **Required contract:** Non-playable mission giver with a Level 1 mission
    role.

<!-- markdownlint-enable MD013 -->

Alias records for these names resolve to the listed canonical identity. They do
not create duplicate character definitions, quote tables, progression keys, or
world actors.

## Verified initial vehicle slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `stutz_bearcat_1936`
  - **Verified context:** Reward
  - **Required rule:** Level 6 street-race prize; phone-booth access after
    unlock.
- **Canonical identity:** `sports_car_1970s`
  - **Verified context:** Starting
  - **Required rule:** Level 7 starting vehicle; character-driver presentation
    is level-scoped.
- **Canonical identity:** `atv`
  - **Verified context:** Secret
  - **Required rule:** Native to Level 4; normal progression does not grant
    global access.
- **Canonical identity:** `ambulance`
  - **Verified context:** Road
  - **Required rule:** Native to Level 5; completion override may expose it
    outside normal progression.
- **Canonical identity:** `armored_truck`
  - **Verified context:** Purchasable
  - **Required rule:** Persistent unlockable vehicle with a separate reward and
    phone-booth rule.
- **Canonical identity:** `audi_tt`
  - **Verified context:** Unused Content
  - **Required rule:** Ships as a reachable unused-content vehicle; incomplete
    presentation or support uses declared generic fallbacks and remains
    replaceable by validated mod overlays.
- **Canonical identity:** `bandit`
  - **Verified context:** Reward
  - **Required rule:** Level 6 bonus-mission reward and a forced vehicle in a
    later mission.
- **Canonical identity:** `bonestorm_truck`
  - **Verified context:** Inaccessible
  - **Required rule:** Alias `cbone`; mission target in Level 1; completion
    override does not change its canonical identity.
- **Canonical identity:** `book_burning_van`
  - **Verified context:** Reward
  - **Required rule:** Level 3 street-race prize; phone-booth access after
    unlock.
- **Canonical identity:** `brick_car`
  - **Verified context:** Unused Content
  - **Required rule:** Ships through the unused-content selection surface with
    isolated progression and mod-replaceable presentation, tuning, and placement
    fields.
- **Canonical identity:** `burns_armored_truck`
  - **Verified context:** Road
  - **Required rule:** Distinct Level 6 road variant; never aliases the
    purchasable armored truck.
- **Canonical identity:** `cpolice`
  - **Verified context:** Inaccessible
  - **Required rule:** Police vehicle present in Levels 1 through 6; excluded
    from normal progression.
- **Canonical identity:** `canyonero`
  - **Verified context:** Purchasable
  - **Required rule:** Player vehicle and forced transport for the Level 1
    hit-and-collect mission.
- **Canonical identity:** `car_built_for_homer`
  - **Verified context:** Reward and purchase
  - **Required rule:** Bonus-mission reward in one context and a 500-coin Level
    5 purchase in another; alias `custom_built_car`; one canonical vehicle and
    phone-booth identity.
- **Canonical identity:** `cell_phone_car`
  - **Verified context:** Inaccessible
  - **Required rule:** Level 2 mission target; excluded from normal progression.

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

- **Canonical identity:** `alien_autotopsy_part_1`
  - **Level and class:** Level 7 main mission 5
  - **Ordered contract:** Collect map, enter vehicle, collect waste, travel to
    playground, deliver vehicle into the target zone, then exit.
- **Canonical identity:** `alien_autotopsy_part_2`
  - **Level and class:** Level 7 main mission 6
  - **Ordered contract:** Force `bandit`, deliver the payload, and satisfy an
    avoid objective before completion.
- **Canonical identity:** `alien_autotopsy_part_3`
  - **Level and class:** Level 7 main mission 7
  - **Ordered contract:** Force the rocket-equipped wartime vehicle, race an
    opponent, collect and deliver the payload, satisfy avoidance, then trigger
    the ending transition.
- **Canonical identity:** `bart_and_frink`
  - **Level and class:** Level 2 main mission 4
  - **Ordered contract:** Follow the delivery vehicle, talk to the police
    contact, locate the criminal, talk to the opponent, race to the stadium, and
    collect the radio.
- **Canonical identity:** `beached_love`
  - **Level and class:** Level 4 bonus mission
  - **Ordered contract:** Timed collect mission with a one-time vehicle reward.
- **Canonical identity:** `better_than_beef`
  - **Level and class:** Level 2 main mission 5
  - **Ordered contract:** Force the pickup, collect all road items, return,
    avoid the pursuer, then return again.
- **Canonical identity:** `blind_big_brother`
  - **Level and class:** Level 1 main mission 4
  - **Ordered contract:** On-foot travel, enter the office, exit, destroy nine
    control boxes, and return.
- **Canonical identity:** `bonestorm_storm`
  - **Level and class:** Level 1 main mission 6
  - **Ordered contract:** Force `canyonero`, travel, talk, hit the target truck,
    collect ten dropped boxes, and return home.
- **Canonical identity:** `bonfire_of_the_manatees`
  - **Level and class:** Level 3 main mission 3
  - **Ordered contract:** Force `longhorn`, travel and talk, hit the target
    vehicle and collect dropped items, travel to the observatory, then talk.
- **Canonical identity:** `caravan_park_time_trial`
  - **Level and class:** Level 1 street race
  - **Ordered contract:** Five clockwise laps through the trailer-park route
    within ninety seconds.
- **Canonical identity:** `casino_circuit_race`
  - **Level and class:** Level 6 street race
  - **Ordered contract:** Five counter-clockwise laps with one ordered opponent
    and a fixed circuit.
- **Canonical identity:** `cell_outs`
  - **Level and class:** Level 2 final main mission
  - **Ordered contract:** Destroy four cell-phone cars, complete the mission,
    and unlock Level 3.

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
[mission, interaction, interior, and notoriety
runtime](mission-interaction-and-notoriety-runtime.md).

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
- Exact duplicate source documents collapse to one fact set and cannot multiply
  definitions, placements, aliases, or availability rows.
- Old/new folder labels, apparent revision age, historical counts, and
  production
  status flags never determine catalog authority.
- Authored copy uses stable localization identities and registered presentation
  owners; raw historical wording and review metadata never define runtime
  identity.
- Loading copy cannot define chapter progression, interior copy cannot define an
  interior, and signage copy cannot define a placement.
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
- Every billboard placement resolves one accepted billboard, location, world,
  transform, presentation, and streaming profile.
- Every collector card belongs to one set, has one durable collection key, and
  resolves one reachable placement and gallery presentation.
- Every gag has one typed class, participant and prop roles, trigger or
  scheduling
  policy, presentation bindings, and completion authority.
- Every interior presentation resolves portals, safe placements, streaming,
  characters, gags, collision, and teardown.
- Production approval, completion, assignment, milestone, and review metadata
  never become runtime state.
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
- billboard, card, gag, interior, or presentation-catalog rows with unresolved
  semantic identities, placements, assets, progression, or teardown;
- card-set ordinal gaps, duplicate durable collection keys, or unreachable card
  placements;
- production approval, completion, assignment, milestone, or review metadata
  entering runtime definitions;
- authored loading, interior, signage, billboard, or reference copy without one
  stable localization identity, presentation owner, locale, accessibility,
  content-filter, fallback, and rights-review result;
- raw historical wording, episode or tape references, or replacement notes
  entering public specifications or runtime identity;
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
- billboard, collector-card, gag, interior-presentation, and
  presentation-catalog
  schema normalization;
- card-set ordinals, durable keys, placement reachability, and gallery
  references;
- production-metadata exclusion and family reconciliation;
- authored loading, interior, signage, billboard, and reference-copy
  localization,
  presentation ownership, rights-review isolation, and raw-source-text
  exclusion;
- exact duplicate collapse and fact-level old/new revision reconciliation;
- legacy character, vehicle, reward, costume, billboard, gag, placement, role,
  and availability-list conversion without status-field authority;
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
- billboard, card, gag, and interior bundles load only their declared native
  presentation dependencies;
- production-only columns are absent from generated Data Assets and Data Tables;
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

- **Canonical identity:** `charles_montgomery_burns`
  - **Aliases:** `mr_burns`, `burns`
  - **Required contract:** Non-playable mission character with distinct Level 1
    and Level 7 placements.
- **Canonical identity:** `clancy_wiggum`
  - **Aliases:** `chief_wiggum`, `wiggum`
  - **Required contract:** Non-playable police character, passenger, and mission
    participant; present across all seven levels; owns event-tagged dialogue.
- **Canonical identity:** `cletus_spuckler`
  - **Aliases:** `cletus`
  - **Required contract:** Non-playable mission giver with level-scoped main and
    bonus mission roles.
- **Canonical identity:** `comic_book_guy`
  - **Aliases:** `jeffrey_albertson`
  - **Required contract:** Non-playable mission giver; owns the Kremlin vehicle
    reference; cutscene-only and interactive placements remain distinct.
- **Canonical identity:** `julius_hibbert`
  - **Aliases:** `dr_hibbert`
  - **Required contract:** Non-playable Level 5 mission giver.
- **Canonical identity:** `nick_riviera`
  - **Aliases:** `dr_nick`
  - **Required contract:** Non-playable mission character with Level 2, Level 3,
    and Level 6 placements.

<!-- markdownlint-enable MD013 -->

The Chief Wiggum quote page contributes rows to `clancy_wiggum`'s quote table.
It does not create a second character, dialogue owner, or voice identity.
Likewise, alternate pages for Cletus resolve to `cletus_spuckler`.

## Verified second vehicle slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `chase_sedan`
  - **Verified contexts:** Level 6 purchase for 500 coins; mission and opponent
    placements in Levels 3, 4, and 6
  - **Required rule:** Purchase ownership, police presentation, and
    alien-controlled mission behavior are separate acquisition and placement
    rows.
- **Canonical identity:** `clown_car`
  - **Verified contexts:** Level 4 street-race reward
  - **Required rule:** Phone-booth access begins after the reward transaction.
- **Canonical identity:** `coffin_cart`
  - **Verified contexts:** Level 7 road vehicle
  - **Required rule:** Native road access does not grant persistent retrieval
    before the completion override.
- **Canonical identity:** `cola_truck`
  - **Verified contexts:** Level 5 purchase for 350 coins; mission target
  - **Required rule:** The player-owned offer and alien-controlled mission
    placement share one vehicle definition.
- **Canonical identity:** `compact_car`
  - **Verified contexts:** Road vehicle in Levels 3, 4, and 6
  - **Required rule:** Native traffic access remains distinct from
    completion-override retrieval.
- **Canonical identity:** `cube_van`
  - **Verified contexts:** Unused and inaccessible
  - **Required rule:** Cataloged for completeness; no normal world placement or
    progression activation.
- **Canonical identity:** `curator`
  - **Verified contexts:** Level 4 purchase for 300 coins; Level 5 mission
    target
  - **Required rule:** Player ownership and target behavior use separate
    acquisition and placement rows.
- **Canonical identity:** `car_built_for_homer`
  - **Verified contexts:** Alias `custom_built_car`; Level 5 purchase for 500
    coins; reward context
  - **Required rule:** Every acquisition grants the same canonical vehicle and
    save identity.
- **Canonical identity:** `donut_truck`
  - **Verified contexts:** Level 3 purchase for 250 coins
  - **Required rule:** Persistent retrieval begins only after purchase.
- **Canonical identity:** `duff_truck`
  - **Verified contexts:** Level 1 purchase for 125 coins; Level 6 mission
    target
  - **Required rule:** Ordinary tuning and mission-specific target tuning remain
    explicit profiles.
- **Canonical identity:** `el_carro_loco`
  - **Verified contexts:** Level 5 street-race reward
  - **Required rule:** Phone-booth access begins after all three level races
    complete.
- **Canonical identity:** `electaurus`
  - **Verified contexts:** Level 1 street-race reward
  - **Required rule:** Driver presentation in later levels does not change
    ownership identity.
- **Canonical identity:** `family_sedan`
  - **Verified contexts:** Level 1 starting vehicle
  - **Required rule:** Available from the retrieval interface from the start;
    Homer is the canonical driver presentation.
- **Canonical identity:** `ferrini_black`
  - **Verified contexts:** Inaccessible Level 7 hostile vehicle
  - **Required rule:** Alias `alien_car`; mission pursuit and race roles do not
    grant ownership.
- **Canonical identity:** `ferrini_red`
  - **Verified contexts:** Level 6 starting vehicle; Level 5 forced mission
    vehicle
  - **Required rule:** Bart driver presentation and cross-level mission use
    retain one identity.
- **Canonical identity:** `fire_truck`
  - **Verified contexts:** Level 2 purchase for 250 coins
  - **Required rule:** Persistent retrieval begins only after purchase.
- **Canonical identity:** `fish_delivery_truck`
  - **Verified contexts:** Level 3 road vehicle
  - **Required rule:** Alias `fish_van`; completion override does not change its
    native traffic role.

<!-- markdownlint-enable MD013 -->

Mission-specific tuning never mutates the shared vehicle definition. A placement
row may select a mission tuning profile, driver, artificial-intelligence role,
damage policy, or objective marker while preserving the canonical vehicle,
acquisition, and save identity.

## Verified second mission slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `clueless`
  - **Level and class:** Level 3 main mission 2
  - **Ordered contract:** Alternate timed travel and talk steps across Wall E.
    Weasel's, Planet Hype, and the Springfield Sign.
- **Canonical identity:** `curious_curator`
  - **Level and class:** Level 5 final main mission
  - **Ordered contract:** Force `ferrini_red`, pursue and destroy `curator`,
    collect the museum key, complete the transition, and unlock Level 6.
- **Canonical identity:** `detention_deficit_disorder`
  - **Level and class:** Level 2 main mission 1
  - **Ordered contract:** Travel toward the store, satisfy the Skinner avoid
    objective, then complete the destination step.
- **Canonical identity:** `dial_b_for_blood`
  - **Level and class:** Level 2 bonus mission
  - **Ordered contract:** Collect the plasma-center blood, travel and talk at
    Moe's, collect the second blood, travel and talk at the construction-site
    restaurant, collect the third blood, return, talk, and grant the wartime
    vehicle reward once.
- **Canonical identity:** `duff_for_me_duff_for_you`
  - **Level and class:** Level 6 main mission 4
  - **Ordered contract:** Travel to the brewery, hit the target Duff Truck,
    collect six dropped laser crates, return to the brewery, and collect the
    final proof item.
- **Canonical identity:** `eight_is_too_much`
  - **Level and class:** Level 5 main mission 3
  - **Ordered contract:** Talk to Hibbert, require `car_built_for_homer` or an
    explicitly permitted substitute, enter the vehicle, hit the van, collect ten
    diapers, return to the hospital, and talk.
- **Canonical identity:** `fishy_deals`
  - **Level and class:** Level 3 main mission 6
  - **Ordered contract:** Talk to the sea-captain contact, collect the ordered
    moving fish targets with the declared miss allowance, and complete the save
    objective.
- **Canonical identity:** `flaming_tires`
  - **Level and class:** Level 7 bonus mission
  - **Ordered contract:** Talk to Smithers, collect the three ordered
    personal-item targets under their timers, return after each required
    segment, and grant the Burns limousine once.

<!-- markdownlint-enable MD013 -->

A required vehicle and a forced vehicle are distinct. A forced vehicle is
selected by the mission. A required-vehicle step validates that the player has
entered an allowed definition and may permit declared substitutes. The mission
cannot silently replace an invalid vehicle with an arbitrary current car.

## Verified second street-race slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `checkpoint_race_level_01`
  - **Level and policy:** Level 1 checkpoint race
  - **Route contract:** Start at the church, traverse the ordered residential
    and poor-district checkpoints, and finish at the power-plant parking area
    against three ordered opponents.
- **Canonical identity:** `circuit_race_level_01`
  - **Level and policy:** Level 1 circuit race
  - **Route contract:** Complete three laps around the rich-district loop before
    three ordered opponents.
- **Canonical identity:** `commercial_district_time_trial_level_02`
  - **Level and policy:** Level 2 time trial
  - **Route contract:** Complete three laps of the commercial and monorail loop
    within 81 seconds.
- **Canonical identity:** `docks_time_trial_level_03`
  - **Level and policy:** Level 3 time trial
  - **Route contract:** Complete four laps of the docks, studio road, alley,
    ramp, and ship-jump loop within 111 seconds.
- **Canonical identity:** `commercial_district_circuit_level_05`
  - **Level and policy:** Level 5 circuit race
  - **Route contract:** Complete three ordered commercial-to-town-square laps
    against `ferrini_red`, a campaign truck, and an ambulance.
- **Canonical identity:** `entertainment_district_time_trial_level_05`
  - **Level and policy:** Level 5 time trial
  - **Route contract:** Complete five clockwise laps of the two-block
    entertainment loop within 81 seconds.
- **Canonical identity:** `entertainment_commercial_checkpoint_level_05`
  - **Level and policy:** Level 5 checkpoint race
  - **Route contract:** Traverse the courthouse, train-yard, expressway, and
    commercial-district checkpoint chain against `ferrini_red`.

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
[Progression, collectibles, cheats, and
credits](progression-collectibles-and-cheats.md).
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

- **Canonical identity:** `professor_frink`
  - **Aliases:** `frink`
  - **Required contract:** Non-playable scientist and mission giver with
    level-scoped story, bonus-mission, ambient, and cinematic placements from
    Levels 2 through 7.
- **Canonical identity:** `gil_gunderson`
  - **Aliases:** `gil`
  - **Required contract:** Non-playable vehicle vendor whose level inventories
    are separate offer rows owned by one character identity.
- **Canonical identity:** `abraham_simpson`
  - **Aliases:** `abe_simpson`, `grampa_simpson`, `grampa`, `grandpa_simpson`,
    `grandpa`
  - **Required contract:** Non-playable mission giver and ambient character;
    every spelling resolves to one dialogue, placement, and save identity.
- **Canonical identity:** `groundskeeper_willie`
  - **Aliases:** `willie`
  - **Required contract:** Non-playable school-area character with level-scoped
    ambient placements and an explicit tractor association.
- **Canonical identity:** `hans_moleman`
  - **Aliases:** `ralph_melish`
  - **Required contract:** Non-playable mission giver and ambient gag
    participant; mission placement and gag presentation remain separate rows.
- **Canonical identity:** `homer_simpson`
  - **Aliases:** `homer`
  - **Required contract:** Playable character in Levels 1 and 7 with additional
    level-scoped presentation roles; all quote rows bind to this identity.
- **Canonical identity:** `horatio_mccallister`
  - **Aliases:** `sea_captain`
  - **Required contract:** Non-playable Level 3 mission giver and ambient
    Squidport placement.
- **Canonical identity:** `comic_book_guy`
  - **Aliases:** `jeffrey_albertson`
  - **Required contract:** The existing canonical identity is reaffirmed;
    cutscene, mission-giver, store, vehicle-owner, and ambient placements do not
    create another character.
- **Canonical identity:** `jimbo_jones`
  - **Aliases:** `jimbo`
  - **Required contract:** Non-playable Level 2 mission character with declared
    ambient placements in later entertainment-district variants.
- **Canonical identity:** `kang`
  - **Aliases:** none
  - **Required contract:** Individual cinematic antagonist identity with no
    ordinary world placement.
- **Canonical identity:** `kodos`
  - **Aliases:** none
  - **Required contract:** Individual cinematic antagonist identity with no
    ordinary world placement.

<!-- markdownlint-enable MD013 -->

Kang and Kodos may share a cinematic cast group, dialogue scene, spacecraft, and
plot-state presentation. The pair is not a third character identity and cannot
own a duplicate dialogue, progression, or save record.

A character quote collection always references its canonical character. The
Homer quote collection therefore extends `homer_simpson`; it does not create a
quote-only character. No separate gag quote owner is defined by this slice.

## Verified third vehicle slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `garbage_truck`
  - **Verified context:** Level 4 road vehicle
  - **Required rule:** Drivable from native traffic only; no normal retrieval
    ownership before the completion override.
- **Canonical identity:** `ghost_ship`
  - **Verified context:** Level 7 road vehicle and race opponent
  - **Required rule:** Drivable from native traffic only; race placement and
    completion-override retrieval do not create a second vehicle.
- **Canonical identity:** `glass_truck`
  - **Verified context:** Level 1 road vehicle
  - **Required rule:** Drivable from native traffic only; no normal retrieval
    ownership before the completion override.
- **Canonical identity:** `globex_super_villain_car`
  - **Verified context:** Level 6 purchase for 600 coins
  - **Required rule:** The accepted purchase grants persistent retrieval for
    this canonical identity.
- **Canonical identity:** `hallo_hearse`
  - **Verified context:** Level 7 road vehicle
  - **Required rule:** Distinct from `hearse`; native traffic access does not
    grant persistent retrieval.
- **Canonical identity:** `hearse`
  - **Verified context:** Level 7 purchase for 750 coins and race opponent
  - **Required rule:** Purchase ownership and race placement share one vehicle
    definition.
- **Canonical identity:** `honor_roller`
  - **Verified context:** Level 2 starting vehicle
  - **Required rule:** Persistent retrieval is available from level start
    without a purchase transaction.
- **Canonical identity:** `hover_bike`
  - **Verified context:** Level 7 purchase for 1,000 coins
  - **Required rule:** Persistent retrieval begins only after the accepted
    purchase.
- **Canonical identity:** `hover_car`
  - **Verified context:** Level 5 bonus-mission reward
  - **Required rule:** The reward transaction grants persistent retrieval
    exactly once.
- **Canonical identity:** `ice_cream_truck`
  - **Verified context:** Unused and inaccessible
  - **Required rule:** Cataloged for completeness; no ordinary traffic, mission,
    purchase, reward, or retrieval activation.
- **Canonical identity:** `itchy_and_scratchy_movie_truck`
  - **Verified context:** Level 6 road vehicle
  - **Required rule:** Drivable from native traffic only; presentation audio
    belongs to its vehicle profile and does not imply ownership.

<!-- markdownlint-enable MD013 -->

`hallo_hearse` and `hearse` are separate canonical definitions despite their
similar display names. Validation rejects an alias, redirect, purchase, traffic
placement, or race row that collapses them.

## Verified third mission slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `flowers_by_irene`
  - **Level and class:** Level 1 main mission 5
  - **Ordered contract:** Enter the house, activate the television interaction,
    leave the interior, enter a vehicle, and follow the surveillance vehicle to
    the declared destination without violating the separation policy.
- **Canonical identity:** `for_a_few_donuts_more`
  - **Level and class:** Level 4 main mission 1
  - **Ordered contract:** Complete the opening follow segment, hit the donut
    truck, collect ten emitted donuts exactly once, return, and deliver the
    accepted set.
- **Canonical identity:** `from_outer_space`
  - **Level and class:** Level 4 final main mission
  - **Ordered contract:** Destroy three declared trucks, return toward home,
    satisfy the final avoid policy, complete the transition, and unlock Level 5.
- **Canonical identity:** `full_metal_jackass`
  - **Level and class:** Level 6 main mission 5
  - **Ordered contract:** Pursue and destroy the declared sedan, accept the
    dropped laser item once, and complete only after collection.
- **Canonical identity:** `getting_down_with_the_clown`
  - **Level and class:** Level 6 main mission 2
  - **Ordered contract:** Trigger the opponent vehicle and win the declared race
    to the Squidport finish against the limousine.
- **Canonical identity:** `going_to_the_lu`
  - **Level and class:** Level 6 main mission 1
  - **Ordered contract:** Force the school bus, collect the declared child
    targets, deliver them to the studio destination, and retain no ownership
    change from the forced vehicle.
- **Canonical identity:** `incriminating_caffeine`
  - **Level and class:** Level 5 main mission 1
  - **Ordered contract:** Follow the target truck, collect eleven ordered drops
    without violating the follow policy, and finish at the declared club
    destination.
- **Canonical identity:** `kang_and_kodos_strike_back`
  - **Level and class:** Level 6 final main mission
  - **Ordered contract:** Force the 1970s sports car, race the chase sedan to
    the brewery, complete the transition, and unlock Level 7.

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

- **Canonical identity:** `hillside_area_circuit_level_03`
  - **Level and policy:** Level 3 circuit race
  - **Route contract:** Complete five laps of the declared figure-eight hillside
    loop against the Canyonero, one sports car, and one compact car.
- **Canonical identity:** `haunted_suburbia_circuit_level_07`
  - **Level and policy:** Level 7 circuit race
  - **Route contract:** Complete three school-to-residential-and-return laps
    against the Hearse, Ghost Ship, and Coffin Cart.

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
[mission, interaction, interior, and notoriety
runtime](mission-interaction-and-notoriety-runtime.md).

Gag placements, rewards, level-scoped completion, and the verified level totals
follow
<!-- markdownlint-disable-next-line MD013 -->
[Progression, collectibles, cheats, and
credits](progression-collectibles-and-cheats.md).

## Verified fourth character slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `kearney_zzyzwicz`
  - **Aliases:** `kearney`
  - **Required contract:** Non-playable character with Level 2, Level 4, and
    Level 6 placements; the Level 6 vendor role references the existing vehicle
    offers.
- **Canonical identity:** `kent_brockman`
  - **Aliases:** none
  - **Required contract:** Cutscene and broadcast character; television-gag
    audio is a presentation placement, not an ambient world character.
- **Canonical identity:** `krusty_the_clown`
  - **Aliases:** `krusty`
  - **Required contract:** Non-playable mission giver and story character with
    level-scoped ambient, cinematic, and mission placements.
- **Canonical identity:** `lenny_leonard`
  - **Aliases:** `lenny`
  - **Required contract:** Non-playable Level 1 mission giver with declared
    ambient placements in Levels 2 and 5.
- **Canonical identity:** `lisa_simpson`
  - **Aliases:** `lisa`
  - **Required contract:** Playable Level 3 protagonist; quote rows and every
    other level placement retain one character identity.
- **Canonical identity:** `marge_simpson`
  - **Aliases:** `marge`
  - **Required contract:** Playable Level 4 protagonist; quote rows and every
    other level placement retain one character identity.
- **Canonical identity:** `louie`
  - **Aliases:** none
  - **Required contract:** Non-playable wager-race host in all seven levels and
    a separate Level 5 story placement.

<!-- markdownlint-enable MD013 -->

A quote page extends the canonical character's quote-event table. It never
creates a quote-only character, voice owner, progression key, or placement.

A vendor, race host, mission giver, ambient pedestrian, cinematic role, and
broadcast role are placement capabilities. They do not create parallel
character definitions.

## Verified fourth vehicle slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `knight_boat`
  - **Aliases:** `knightboat`
  - **Verified context:** Level 3 secret vehicle
  - **Required rule:** One world placement grants temporary access; it does not
    count toward the five progression vehicles.
- **Canonical identity:** `kremlin`
  - **Aliases:** none
  - **Verified context:** Level 4 bonus-mission reward
  - **Required rule:** The accepted reward grants persistent retrieval exactly
    once.
- **Canonical identity:** `krustys_limo`
  - **Aliases:** none
  - **Verified context:** Level 4 purchase for 350 coins and mission
    presentation
  - **Required rule:** Purchase ownership and opponent or ambient placements
    share one definition.
- **Canonical identity:** `limo`
  - **Aliases:** none
  - **Verified context:** Level 2 purchase for 150 coins
  - **Required rule:** Distinct from `krustys_limo`; purchase grants persistent
    retrieval.
- **Canonical identity:** `longhorn`
  - **Aliases:** none
  - **Verified context:** Level 5 starting vehicle
  - **Required rule:** Available from level start and excluded from counted
    progression vehicles.
- **Canonical identity:** `malibu_stacy_car`
  - **Aliases:** none
  - **Verified context:** Level 3 starting vehicle
  - **Required rule:** Available from level start and bound to Lisa's default
    driver presentation.

<!-- markdownlint-enable MD013 -->

`knight_boat` and `knightboat` are aliases. `limo` and `krustys_limo` are
separate canonical vehicle definitions. Validation rejects an alias or redirect
that collapses the two limousine definitions.

## Verified fourth mission slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `ketchup_logic`
  - **Level and class:** Level 4 main mission 3
  - **Ordered contract:** Force the pickup truck and required costume, collect
    eighteen ordered packets within 120 seconds, reach the pursuit trigger
    within 45 seconds, evade the sedan within 60 seconds, and return to the
    declared destination.
- **Canonical identity:** `kinky_frinky`
  - **Level and class:** Level 5 bonus mission
  - **Ordered contract:** Destroy the Hover Car within 180 seconds, return
    within 30 seconds, complete the conversation, and grant the Hover Car reward
    once.
- **Canonical identity:** `kwik_cash`
  - **Level and class:** Level 5 main mission 6
  - **Ordered contract:** Force the Bandit, reach and evade the first police
    pursuit, locate and destroy the Armored Truck without a destroy timer,
    return, evade the second pursuit within 45 seconds, and complete the final
    return and conversation.
- **Canonical identity:** `lab_coat_caper`
  - **Level and class:** Level 6 main mission 3
  - **Ordered contract:** Follow Frink's Hover Car through the declared repeated
    route to the observatory while satisfying the separation policy.
- **Canonical identity:** `long_black_probes`
  - **Level and class:** Level 7 main mission 2
  - **Ordered contract:** Require the owned Zombie Car, enter it, travel to the
    playground, and follow the alien probe to the power plant without violating
    separation or vehicle-health policy.

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

- **Canonical identity:** `mansion_power_plant_time_trial_level_04`
  - **Level and policy:** Level 4 time trial
  - **Route contract:** Complete three laps through the mansion grounds,
    power-plant passage, and Stonecutters route within 131 seconds.
- **Canonical identity:** `kwik_e_mart_time_trial_level_07`
  - **Level and policy:** Level 7 time trial
  - **Route contract:** Complete five counter-clockwise laps of the store,
    gas-station, and donut-shop block within the seventy-second stage timer.

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

- **Canonical identity:** `mayor_quimby`
  - **Aliases:** `quimby`
  - **Required contract:** Non-playable civic character with cinematic,
    billboard, vehicle-presentation, and level-scoped ambient roles.
- **Canonical identity:** `milhouse_van_houten`
  - **Aliases:** `milhouse`
  - **Required contract:** Non-playable mission character and Levels 1 through 6
    time-trial host; race-host and story placements share one character
    identity.
- **Canonical identity:** `moe_szyslak`
  - **Aliases:** `moe`
  - **Required contract:** Non-playable talkable and mission character with
    house, tavern, ambient, and story placements.
- **Canonical identity:** `charles_montgomery_burns`
  - **Aliases:** `mr_burns`, `monty_burns`, `burns`
  - **Required contract:** The existing canonical identity is reaffirmed for
    intercom, mission, cinematic, and Level 7 interaction roles.
- **Canonical identity:** `waylon_smithers`
  - **Aliases:** `mr_smithers`, `smithers`
  - **Required contract:** Non-playable mission, ambient, driver, cinematic, and
    Level 7 bonus-mission character.
- **Canonical identity:** `ned_flanders`
  - **Aliases:** `ned`
  - **Required contract:** Non-playable mission, talkable, house-interaction,
    gag, and ambient character.
- **Canonical identity:** `nerd`
  - **Aliases:** none
  - **Required contract:** Non-playable mission and race-driver archetype with
    exact Level 2 and Level 3 story placements.
- **Canonical identity:** `otto_mann`
  - **Aliases:** `otto`
  - **Required contract:** Non-playable mission character, bus driver, and
    level-scoped ambient placement.

<!-- markdownlint-enable MD013 -->

The minor-character and non-story-character indexes are query projections over
canonical definitions and placement capabilities. They do not create aggregate
characters or duplicate dialogue owners. The full placement rules follow
<!-- markdownlint-disable-next-line MD013 -->
[Ambient population and named-character
runtime](ambient-population-and-named-character-runtime.md).

## Verified fifth vehicle slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `milk_truck`
  - **Aliases:** none
  - **Verified context:** Level 6 mission target and completion override
  - **Required rule:** Mission destruction does not grant ownership;
    completion-only retrieval uses its explicit override.
- **Canonical identity:** `mini_school_bus`
  - **Aliases:** none
  - **Verified context:** Level 1 traffic and completion override
  - **Required rule:** Traffic access is temporary and does not grant ordinary
    retrieval ownership.
- **Canonical identity:** `minivan`
  - **Aliases:** none
  - **Verified context:** Level 1 traffic and completion override
  - **Required rule:** Native traffic and completion-override retrieval
    reference one definition.
- **Canonical identity:** `monorail_car`
  - **Aliases:** none
  - **Verified context:** Level 2 secret vehicle
  - **Required rule:** World access is temporary and excluded from the five
    counted progression vehicles.
- **Canonical identity:** `obliteratatron_big_wheel_truck`
  - **Aliases:** `obliteration_big_wheel_truck`, `monster_truck`
  - **Verified context:** Level 5 secret vehicle
  - **Required rule:** All three names resolve to one secret-vehicle definition
    and placement family.
- **Canonical identity:** `mr_burns_limo`
  - **Aliases:** `burns_limo`
  - **Verified context:** Level 7 bonus-mission reward
  - **Required rule:** The accepted reward grants persistent retrieval exactly
    once.
- **Canonical identity:** `mr_plow`
  - **Aliases:** none
  - **Verified context:** Level 2 purchase for 200 coins
  - **Required rule:** Purchase ownership gates the declared required-vehicle
    mission and normal retrieval.
- **Canonical identity:** `nerd_car`
  - **Aliases:** none
  - **Verified context:** Level 3 purchase for 250 coins and race opponent
  - **Required rule:** Purchase and opponent placements share one definition.
- **Canonical identity:** `nonuplets_minivan`
  - **Aliases:** `shelbyville_nonuplets_van`
  - **Verified context:** Completion-override vehicle
  - **Required rule:** No ordinary traffic, purchase, reward, or secret
    placement grants ownership.
- **Canonical identity:** `nuclear_waste_truck`
  - **Aliases:** none
  - **Verified context:** Level 4 traffic and completion override
  - **Required rule:** The traffic vehicle is distinct from the nuclear-waste
    mission payload.
- **Canonical identity:** `open_wheel_race_car`
  - **Aliases:** none
  - **Verified context:** Level 7 street-race reward
  - **Required rule:** Completing the declared race set grants persistent
    retrieval.
- **Canonical identity:** `pickup_road_vehicle`
  - **Aliases:** `pickup`
  - **Verified context:** Traffic in Levels 1, 3, and 6 plus completion override
  - **Required rule:** Traffic access and static prop placements do not grant
    ownership.
- **Canonical identity:** `cletus_pickup_truck`
  - **Aliases:** `pickup_truck`
  - **Verified context:** Level 1 bonus reward and mission vehicle
  - **Required rule:** Distinct from `pickup_road_vehicle`; reward ownership and
    forced mission use share one definition.
- **Canonical identity:** `pizza_van`
  - **Aliases:** none
  - **Verified context:** Level 2 traffic and mission target plus completion
    override
  - **Required rule:** Distinct from the purchasable surveillance vehicle
    despite related presentation.

<!-- markdownlint-enable MD013 -->

The vehicle browser, locked rows, health, repair, completion override, delivery,
and mission restrictions follow
<!-- markdownlint-disable-next-line MD013 -->
[Vehicle retrieval and phone-booth
runtime](vehicle-retrieval-and-phone-booth-runtime.md).

## Verified fifth mission slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `milking_the_pigs`
  - **Level and class:** Level 6 bonus mission
  - **Ordered contract:** Hit Chief Wiggum's vehicle and accept the evidence
    folder within 120 seconds, complete the Snake conversation, locate and
    destroy the Milk Truck within 180 seconds, return, and grant the Bandit
    once.
- **Canonical identity:** `monkey_see_monkey_doh`
  - **Level and class:** Level 2 main mission 6
  - **Ordered contract:** Require the owned Mr. Plow, travel to the research
    center, collect thirty declared monkeys within 240 seconds, return, complete
    the Dr. Nick interaction, and reach the final blender target.
- **Canonical identity:** `nerd_race_queen`
  - **Level and class:** Level 3 main mission 1
  - **Ordered contract:** Force Comic Book Guy's vehicle, win the declared race
    against the Nerd Car, reach the comic target, return within 90 seconds, and
    complete the final interaction.
- **Canonical identity:** `never_trust_a_snake`
  - **Level and class:** Level 5 main mission 5
  - **Ordered contract:** Hit the garbage truck and accept five emitted targets
    within 255 seconds, collect twenty-five declared garbage targets without a
    timer, reach the DMV, complete the Snake interaction and interior
    transition, and accept the folder target.
- **Canonical identity:** `office_spaced`
  - **Level and class:** Level 1 main mission 3
  - **Ordered contract:** Require the Plow King, reach Lenny, reach the Smithers
    pursuit start within 90 seconds, and destroy Smithers' vehicle before its
    race-condition destination.
- **Canonical identity:** `operation_hellfish`
  - **Level and class:** Level 3 main mission 4
  - **Ordered contract:** Require the School Bus, reach the observatory and
    first target, then destroy three declared sedans in successive 120-second,
    90-second, and 75-second stages.
- **Canonical identity:** `petty_theft_homer`
  - **Level and class:** Level 1 main mission 2
  - **Ordered contract:** Collect the ordered personal-item targets under their
    declared 40-second or untimed policies, complete the Barney interaction,
    return to Ned, and complete the final conversation.

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
[mission, interaction, interior, and notoriety
runtime](mission-interaction-and-notoriety-runtime.md).

## Verified fifth system and index slice

The mission index, minor-character index, non-story-character index, mod index,
and modification index are coverage or navigation evidence. They do not create
aggregate runtime assets. Accepted mod packages project through
[Mod package overlay runtime](mod-package-overlay-runtime.md).

The music census resolves through
[Music state and transition runtime](music-state-and-transition-runtime.md).
The pedestrian census resolves through
<!-- markdownlint-disable-next-line MD013 -->
[Ambient population and named-character
runtime](ambient-population-and-named-character-runtime.md).
Historical pedestrian, ambient-gag, walker, driver, passenger, character-role,
and vehicle-availability tables follow
<!-- markdownlint-disable-next-line MD013 -->
[Historical core-design and dialogue evidence
normalization](historical-core-design-and-dialogue-evidence-normalization.md).
They create canonical definitions and typed availability or placement rows only
after character, vehicle, mission, seat, dialogue, animation, collision, native
asset, and loading dependencies resolve. Model-ready, animator, status,
approval,
voice-production, and matrix-position fields remain import-review metadata.
The phone-booth census resolves through
<!-- markdownlint-disable-next-line MD013 -->
[Vehicle retrieval and phone-booth
runtime](vehicle-retrieval-and-phone-booth-runtime.md).

The newspaper page contributes no independently identified gameplay definition
in
this slice. Historical oddity and unused-behavior lists are negative
compatibility
or review evidence; they do not become successful gameplay features unless an
intentional behavior has its own verified contract.

## Verified sixth character and archetype slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `principal_seymour_skinner`
  - **Aliases:** `principal_skinner`, `seymour_skinner`, `skinner`
  - **Required contract:** One non-playable character owns mission-opponent,
    bonus-mission-giver, school, ambient, driver, and cinematic placements.
- **Canonical identity:** `professor_frink`
  - **Aliases:** `frink`
  - **Required contract:** The existing scientist identity is reaffirmed for
    mission-giver, driver, observatory, bonus-reward, ambient, and cinematic
    placements.
- **Canonical identity:** `reverend_lovejoy`
  - **Aliases:** `lovejoy`
  - **Required contract:** Non-playable named character with level-scoped
    ambient and presentation placements.
- **Canonical identity:** `horatio_mccallister`
  - **Aliases:** `sea_captain`
  - **Required contract:** The existing canonical identity is reaffirmed for
    Squidport ambience and the `princi_pal` interaction.
- **Canonical identity:** `snake_jailbird`
  - **Aliases:** `snake`
  - **Required contract:** One non-playable character owns mission-giver,
    target, driver, ambient, and dialogue placements across Levels 2, 3, 5, 6,
    and 7.
- **Canonical identity:** `mayor_quimby`
  - **Aliases:** `quimby`
  - **Required contract:** The existing civic character identity owns cutscene,
    billboard, vehicle-presentation, and ambient references.
- **Canonical identity:** `waylon_smithers`
  - **Aliases:** `mr_smithers`, `smithers`
  - **Required contract:** The existing character identity owns mission, driver,
    bonus-mission, ambient, and cinematic placements.

<!-- markdownlint-enable MD013 -->

`Skeleton` identifies a generic Level 7 ambient archetype. It uses a population
archetype and placement rows, not a named character, dialogue owner, or save
identity. Named-character and ambient-archetype behavior follows
<!-- markdownlint-disable-next-line MD013 -->
[Ambient population and named-character
runtime](ambient-population-and-named-character-runtime.md).

## Verified sixth vehicle slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `planet_hype_50s_car`
  - **Aliases:** none
  - **Verified context:** Level 6 secret vehicle
  - **Required rule:** Temporary world access only; completion override does not
    create ordinary ownership.
- **Canonical identity:** `plow_king`
  - **Aliases:** none
  - **Verified context:** Level 1 purchase for 150 coins
  - **Required rule:** Purchase ownership is required by the declared mission
    gate.
- **Canonical identity:** `police_car`
  - **Aliases:** none
  - **Verified context:** Level 5 purchase for 425 coins and mission-forced
    placement
  - **Required rule:** Ownership, forced use, driver presentation, and pursuit
    placement remain separate rows.
- **Canonical identity:** `hover_car`
  - **Aliases:** `professor_frinks_hover_car`, `frinks_hover_car`
  - **Verified context:** Level 5 bonus reward and later forced or target
    placements
  - **Required rule:** The bonus reward grants ownership once; other placements
    do not replay it.
- **Canonical identity:** `quad_bike`
  - **Aliases:** `atv`
  - **Verified context:** Level 4 secret vehicle
  - **Required rule:** One trailer-park placement grants temporary access.
- **Canonical identity:** `rc_buggy`
  - **Aliases:** `r_c_buggy`
  - **Verified context:** Level 7 secret vehicle
  - **Required rule:** One roof placement grants temporary access.
- **Canonical identity:** `red_brick_car`
  - **Aliases:** `brick_car`
  - **Verified context:** Development-only vehicle
  - **Required rule:** Excluded from shipping ownership, traffic, secret,
    mission, race, and completion-override queries.
- **Canonical identity:** `suv`
  - **Aliases:** none
  - **Verified context:** Traffic in Levels 4 and 5
  - **Required rule:** Native traffic access does not grant persistent
    ownership.
- **Canonical identity:** `school_bus`
  - **Aliases:** none
  - **Verified context:** Level 3 purchase for 300 coins and mission vehicle
  - **Required rule:** Purchase, forced use, required use, and Otto driver
    presentation share one definition.
- **Canonical identity:** `sedan_level_02`
  - **Aliases:** none
  - **Verified context:** Level 2 street-race reward
  - **Required rule:** Distinct persistent reward identity.
- **Canonical identity:** `sedan_level_03`
  - **Aliases:** `skinners_sedan`
  - **Verified context:** Level 3 bonus-mission reward and Skinner driver
    placement
  - **Required rule:** Distinct from every other sedan definition.
- **Canonical identity:** `sedan_a`
  - **Aliases:** none
  - **Verified context:** Unused development traffic definition
  - **Required rule:** Excluded from normal shipping access.
- **Canonical identity:** `sedan_b`
  - **Aliases:** none
  - **Verified context:** Level 2 traffic
  - **Required rule:** Traffic and completion override remain separate from
    ownership.
- **Canonical identity:** `speed_rocket`
  - **Aliases:** none
  - **Verified context:** Level 1 secret vehicle
  - **Required rule:** Temporary world access only.
- **Canonical identity:** `sports_car_a`
  - **Aliases:** none
  - **Verified context:** Traffic in Levels 2 and 3 plus race placements
  - **Required rule:** Traffic, opponent, prop, and completion-override rows
    share one definition.
- **Canonical identity:** `sports_car_b`
  - **Aliases:** none
  - **Verified context:** Level 5 traffic
  - **Required rule:** Traffic access does not grant persistent ownership.

<!-- markdownlint-enable MD013 -->

The complete 42-vehicle persistent roster, seven secret placements, seven
traffic
rosters, completion override, sedan identity boundary, drivers, and development
exclusions follow
[Vehicle access and roster runtime](vehicle-access-and-roster-runtime.md).

## Verified sixth mission slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `s_m_r_t`
  - **Level and class:** Level 1 main mission 1
  - **Ordered contract:** Accept the science-project target, reach Skinner's
    start, win the untimed route race, enter Springfield Elementary, talk to
    Lisa, and complete the final dialogue.
- **Canonical identity:** `princi_pal`
  - **Level and class:** Level 3 bonus mission
  - **Ordered contract:** Force Skinner's Sedan; complete the 40-second opening
    errand, untimed laundry target, 30-second restaurant travel, meal
    interaction, 45-second casino travel, cream interaction, and 35-second
    arcade return; then grant Skinner's Sedan once.
- **Canonical identity:** `slithery_sleuthing`
  - **Level and class:** Level 3 main mission 5
  - **Ordered contract:** Force the Police Car, follow Snake over four route
    waypoints and accept three emitted targets, satisfy separation and vehicle
    conditions, reach the casino within 60 seconds, and complete Wiggum's final
    sequence.
- **Canonical identity:** `redneck_roundup`
  - **Level and class:** Level 4 main mission 2
  - **Ordered contract:** Follow Cletus over eight route waypoints and accept
    seven emitted objects without violating the separation policy; no timer
    applies.
- **Canonical identity:** `return_of_the_nearly_dead`
  - **Level and class:** Level 4 main mission 5
  - **Ordered contract:** Reach the school within 30 seconds, complete Nelson's
    interaction, follow the sedan and accept ten pills, reach the false
    destination within 90 seconds, lose the tail within 90 seconds, reach Grampa
    within 150 seconds, collect the interior caffeine target, return, and
    complete the cinematic transition.
- **Canonical identity:** `set_to_kill`
  - **Level and class:** Level 6 main mission 6
  - **Ordered contract:** Require purchase of the Globex Super Villain Car,
    reach Krustylu, destroy and accept twenty-five laser-stand targets within
    100 seconds, return within 50 seconds, and complete the Krusty interaction.
- **Canonical identity:** `rigor_motors`
  - **Level and class:** Level 7 main mission 1
  - **Ordered contract:** Talk to Ned within 30 seconds, collect the first-aid
    kit, reach and collect the boards within the declared 15-second travel
    stage, reach Moe within 15 seconds, collect the chainsaw, and return home
    within 40 seconds.
- **Canonical identity:** `pocket_protector`
  - **Level and class:** Level 7 main mission 3
  - **Ordered contract:** Force the Hover Car, acquire the nuclear-waste payload
    within 120 seconds, reach the playground within 100 seconds while retaining
    vehicle and payload, and destroy the boss target within 10 seconds while
    preserving the payload policy.

<!-- markdownlint-enable MD013 -->

A pre-mission purchase or ownership gate is activation policy, not a duplicate
mission objective. A zero timer declaration means untimed. Forced, required, and
owned vehicles never collapse into one acquisition state.

## Verified sixth race slice

<!-- markdownlint-disable MD013 -->

- **Canonical identity:** `rich_district_2_circuit_level_04`
  - **Verified route contract:** Three laps; six AI route waypoints and five
    dense player checkpoints; opponents are Apu in the Longhorn, the Nuclear
    Waste Truck, and the Garbage Truck; first place required; no timer.
- **Canonical identity:** `squidport_checkpoint_level_03`
  - **Verified route contract:** Five ordered checkpoints against Marge in the
    Canyonero, Sports Car A, and the road Pickup; first place required.
- **Canonical identity:** `squidport_tourist_resort_time_trial_level_06`
  - **Verified route contract:** Two laps through eight ordered checkpoints
    within 115 seconds.
- **Canonical identity:** `squidport_2_checkpoint_level_06`
  - **Verified route contract:** Six ordered checkpoints against Homer in the
    canonical Level 7 sports-car placement; first place required.

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

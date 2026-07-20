# Content roots, modules, and package layout

- Status: Active
- Last reviewed: 2026-07-18

## Root ownership

Base-game content uses only `/Game/SHAR`. Validated mod content uses only
`/Game/Mods/<namespace>`. Engine, plugin, developer, transient, staging, and
test roots are never referenced by shipping base definitions.

## Canonical base layout

```text
/Game/SHAR
├── Data
│   ├── Catalog
│   ├── PlatformProfiles/<profile_id>
│   ├── Characters/<character_id>
│   ├── Characters/AnimationLibraries
│   ├── Vehicles/<vehicle_id>
│   ├── Worlds/<world_id>
│   ├── Actions/<action_id>
│   ├── ActionSequences/<sequence_id>
│   ├── Locations/<location_id>
│   ├── Missions/<chapter_id>/<mission_id>
│   ├── Interactions/<interaction_id>
│   ├── ApplicationModes/<mode_id>
│   ├── SaveSchemas/<schema_id>
│   ├── Progression/<catalog_id>
│   ├── Presentations/<presentation_id>
│   ├── Rewards/<reward_id>
│   ├── Abilities/<ability_id>
│   ├── Cameras/<camera_profile_id>
│   ├── Audio/<audio_profile_id>
│   ├── UI/<screen_or_flow_id>
│   ├── Populations/<population_id>
│   ├── GameModes/<game_mode_id>
│   └── Tables/<table_family>
├── Art
│   ├── Characters/<character_id>
│   │   ├── Meshes
│   │   ├── Materials
│   │   ├── Textures
│   │   └── Physics
│   ├── Characters/Rigs/<rig_family>
│   ├── Characters/Animations/<rig_family>/<clip_group>
│   ├── Characters/AnimationBlueprints/<rig_family>
│   ├── Vehicles/<vehicle_id>
│   │   ├── Meshes
│   │   ├── Materials
│   │   ├── Textures
│   │   ├── Animations
│   │   ├── Physics
│   │   └── Damage
│   ├── World/<world_id>/<region_id>/<asset_id>
│   ├── Props/<prop_id>
│   ├── UI/<screen_or_feature_id>
│   └── VFX/<effect_id>
├── Audio
│   ├── Dialogue/<locale>/<character_id>
│   ├── Music/<music_id>
│   ├── SFX/<domain>/<asset_id>
│   └── Mix
├── Media/<media_id>
└── Maps
    ├── OpenWorld
    ├── Interiors
    ├── Experiences
    └── Tests
```

`Maps/Tests` and any asset reachable only from it use development-only cook
rules and cannot appear in shipping chunks. Production runtime content never
lives in `Developers`, `Collections`, `Staging`, `Temp`, or arbitrary migration
folders.

## Canonical package examples

<!-- markdownlint-disable MD013 -->
```text
/Game/SHAR/Data/PlatformProfiles/DA_PlatformProfile_windows_x8664
/Game/SHAR/Data/Characters/homer/DA_Character_homer
/Game/SHAR/Data/Characters/homer/DA_CharacterPresentation_homer_default
/Game/SHAR/Art/Characters/homer/Meshes/SK_Character_homer_default
/Game/SHAR/Art/Characters/homer/Materials/MI_Character_homer_body
/Game/SHAR/Art/Characters/homer/Textures/T_Character_homer_body_BC
/Game/SHAR/Data/Characters/AnimationLibraries/DA_CharacterAnimationLibrary_humanoid_common_v1
/Game/SHAR/Art/Characters/Animations/humanoid_common_v1/Locomotion/A_humanoid_common_v1_locomotion_walk_forward
/Game/SHAR/Data/Vehicles/family_sedan/DA_Vehicle_family_sedan
/Game/SHAR/Art/Vehicles/family_sedan/Meshes/SK_Vehicle_family_sedan
/Game/SHAR/Data/Missions/chapter_01/mission_01/DA_Mission_chapter_01_mission_01
/Game/SHAR/Data/Actions/enter_vehicle_position/DA_Action_enter_vehicle_position
/Game/SHAR/Data/ActionSequences/enter_vehicle/DA_ActionSequence_enter_vehicle
/Game/SHAR/Data/Interactions/enter_family_sedan/DA_Interaction_enter_family_sedan
/Game/SHAR/Data/ApplicationModes/front_end/DA_ApplicationMode_front_end
/Game/SHAR/Data/SaveSchemas/portable_v1/DA_SaveSchema_portable_v1
/Game/SHAR/Data/Progression/base/DA_ProgressionCatalog_base
/Game/SHAR/Data/Presentations/kwik_e_mart_intro/DA_Presentation_kwik_e_mart_intro
/Game/SHAR/Maps/OpenWorld/W_SHAR_OpenWorld
```
<!-- markdownlint-enable MD013 -->

Folder names use canonical identifiers. Object prefixes and role suffixes are
fixed by the naming contract. A definition and its secondary assets may share a
canonical identifier without sharing a package.

## Package granularity

One package contains one independently replaceable top-level asset unless Unreal
requires generated companions. A character definition, character presentation,
skeletal mesh, skeleton, Physics Asset, Animation Blueprint, animation sequence,
material instance, texture, vehicle definition, world definition, mission
definition, and UI screen each have independent packages.

Do not place all content for a character, vehicle, level, or chapter into one
monolithic package. Do not create one package per source file when several
source files jointly define one native asset. Package boundaries follow native
lifecycle and mod replacement granularity.

## Bundle vocabulary

Every Primary Asset uses only these bundle names:

<!-- markdownlint-disable MD013 -->
| Bundle | Content | Typical load scope |
| :--- | :--- | :--- |
| `Definition` | Identity, metadata, dependencies, generated rows | Catalog and save migration |
| `Gameplay` | Collision, abilities, AI, mission, interaction, physics | Active simulation |
| `Presentation` | Meshes, materials, animation, UI, icons | Visible or previewed content |
| `Audio` | Dialogue, music, SFX, MetaSounds | Audible scope |
| `Cinematic` | Sequences, cameras, media | Active cinematic |
| `EditorReview` | Review images and conformance evidence | Editor only |
<!-- markdownlint-enable MD013 -->

A new bundle requires an architecture decision. Importers may not invent bundle
names per asset.

## Module-to-content ownership

Modules own schemas and services, not hardcoded asset lists. `SharCharacters`
may declare character asset types and validation but does not scan character
folders. `SharMissions` may resolve a character identity through `SharContent`
but does not include character implementation headers merely to load a mesh.

Editor-only importer modules may depend on runtime definition modules so they
can construct valid assets. Runtime modules never depend on editor modules.

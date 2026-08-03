# Identity, naming, revisions, and import plans

- Status: Active
- Last reviewed: 2026-07-20

## Canonical identifiers

Canonical identifiers are non-empty lowercase ASCII `snake_case` values
matching:

```text
^[a-z0-9]+(?:_[a-z0-9]+)*$
```

They are stable domain identity. They do not contain localization, punctuation,
display capitalization, chapter placement, source extension, local path, hash,
platform, quality level, or temporary revision.

Examples:

- valid: `homer`, `family_sedan`, `chapter_01`, `mission_01`, `kwik_e_mart`;
- invalid: `Homer`, `homer-m`, `l1_homer_final.fbx`, `C:\assets\homer`, or an
  import ordinal presented as identity.

## Primary Asset types and object names

- **Family:** Gameplay catalog
  - **Primary Asset type:** `SharCatalog`
  - **Object name:** `DA_SHAR_GameplayCatalog`
- **Family:** Platform profile
  - **Primary Asset type:** `SharPlatformProfile`
  - **Object name:** `DA_PlatformProfile_<id>`
- **Family:** Character
  - **Primary Asset type:** `SharCharacter`
  - **Object name:** `DA_Character_<id>`
- **Family:** Character presentation
  - **Primary Asset type:** `SharCharacterPresentation`
  - **Object name:** `DA_CharacterPresentation_<id>_<variant>`
- **Family:** Rig profile
  - **Primary Asset type:** `SharRigProfile`
  - **Object name:** `DA_Rig_<id>`
- **Family:** Character animation library
  - **Primary Asset type:** `SharCharacterAnimationLibrary`
  - **Object name:** `DA_CharacterAnimationLibrary_<rig_family>`
- **Family:** Vehicle
  - **Primary Asset type:** `SharVehicle`
  - **Object name:** `DA_Vehicle_<id>`
- **Family:** Vehicle presentation
  - **Primary Asset type:** `SharVehiclePresentation`
  - **Object name:** `DA_VehiclePresentation_<id>_<variant>`
- **Family:** World
  - **Primary Asset type:** `SharWorld`
  - **Object name:** `DA_World_<id>`
- **Family:** Action
  - **Primary Asset type:** `SharAction`
  - **Object name:** `DA_Action_<id>`
- **Family:** Action sequence
  - **Primary Asset type:** `SharActionSequence`
  - **Object name:** `DA_ActionSequence_<id>`
- **Family:** Interaction
  - **Primary Asset type:** `SharInteraction`
  - **Object name:** `DA_Interaction_<id>`
- **Family:** Application mode
  - **Primary Asset type:** `SharApplicationMode`
  - **Object name:** `DA_ApplicationMode_<id>`
- **Family:** Save schema
  - **Primary Asset type:** `SharSaveSchema`
  - **Object name:** `DA_SaveSchema_<id>`
- **Family:** Progression catalog
  - **Primary Asset type:** `SharProgressionCatalog`
  - **Object name:** `DA_ProgressionCatalog_<id>`
- **Family:** Meta catalog
  - **Primary Asset type:** `SharMetaCatalog`
  - **Object name:** `DA_MetaCatalog_<id>`
- **Family:** Frontend catalog
  - **Primary Asset type:** `SharFrontendCatalog`
  - **Object name:** `DA_FrontendCatalog_<id>`
- **Family:** Presentation
  - **Primary Asset type:** `SharPresentation`
  - **Object name:** `DA_Presentation_<id>`
- **Family:** Location
  - **Primary Asset type:** `SharLocation`
  - **Object name:** `DA_Location_<id>`
- **Family:** Mission
  - **Primary Asset type:** `SharMission`
  - **Object name:** `DA_Mission_<chapter>_<id>`
- **Family:** Reward
  - **Primary Asset type:** `SharReward`
  - **Object name:** `DA_Reward_<id>`
- **Family:** Ability
  - **Primary Asset type:** `SharAbility`
  - **Object name:** `DA_Ability_<id>`
- **Family:** Camera profile
  - **Primary Asset type:** `SharCameraProfile`
  - **Object name:** `DA_Camera_<id>`
- **Family:** Audio profile
  - **Primary Asset type:** `SharAudioProfile`
  - **Object name:** `DA_Audio_<id>`
- **Family:** UI flow
  - **Primary Asset type:** `SharUIFlow`
  - **Object name:** `DA_UIFlow_<id>`
- **Family:** Game mode
  - **Primary Asset type:** `SharGameMode`
  - **Object name:** `DA_GameMode_<id>`
- **Family:** Mod descriptor
  - **Primary Asset type:** `SharModDescriptor`
  - **Object name:** `DA_Mod_<namespace>`

Primary Asset identity is `<type>:<canonical_id>`. Runtime identity never
depends on the object name, although validation requires the object name to
match the canonical presentation shown above.

## Secondary prefixes

| Asset | Prefix |
| :--- | :--- |
| Skeletal Mesh | `SK_` |
| Skeleton | `SKEL_` |
| Static Mesh | `SM_` |
| Physics Asset | `PHYS_` |
| Animation Blueprint | `ABP_` |
| Animation Sequence | `A_` |
| Animation Montage | `AM_` |
| Control Rig | `CR_` |
| Material | `M_` |
| Material Instance | `MI_` |
| Texture | `T_` |
| Niagara System | `NS_` |
| MetaSound Source | `MS_` |
| Sound Wave | `SW_` |
| Data Table | `DT_` |
| StateTree | `ST_` |
| Widget Blueprint | `WBP_` |
| World | `W_` |
| Level Instance | `LI_` |
| Data Layer | `DL_` |

Role suffixes are semantic: textures use `_BC`, `_N`, `_ORM`, `_E`, `_M`, or
`_LUT`; presentations use registered variant identifiers such as `default` or a
costume identity. Source suffixes and platform names are not native identity.

## Revision fields

Every generated definition contains:

- `DefinitionSchemaVersion`: positive integer for C++ schema compatibility;
- `RevisionToken`: deterministic digest or immutable revision of generated data;
- `SourcePackageIds`: ordered unique public-safe package identities;
- `ValidationProfile`: canonical identifier for the exact required checks;
- `OwningFeature`: `base` or a validated mod namespace.

A data revision does not change canonical identity. A schema-breaking semantic
change requires a new schema version and an explicit migrator. Save data stores
canonical identities and schema-aware state, never object paths.

## Aliases and redirects

Aliases use the canonical identifier syntax and resolve directly to exactly one
canonical identity. Alias chains and cycles are forbidden. Asset Manager
redirects are generated only for published object or type renames; they are not
a substitute for the alias table.

Deleting or reusing a published identity is forbidden. Removed content becomes a
tombstone definition with migration and fallback policy until every supported
save, server package set, and mod compatibility window no longer references it.

## Import plan file

Each package submitted to native import contains one UTF-8 JSON file named
`unreal-import-plan.json`. It uses LF line endings, no byte-order mark, sorted
object keys, deterministic array ordering, and this schema identity:

```text
shar.unreal.import-plan.v1
```

The plan contains no absolute path, username, drive letter, editor session id,
temporary directory, source-game route, credential, or proprietary source prose.
Artifact references are normalized forward-slash paths relative to the plan.

## Required envelope

```json
{
  "schema": "shar.unreal.import-plan.v1",
  "transaction_id": "sha256:<64 lowercase hex characters>",
  "package_id": "character.homer.default",
  "package_revision": "sha256:<64 lowercase hex characters>",
  "asset_family": "character",
  "canonical_id": "homer",
  "owning_feature": "base",
  "source_coordinate_system": {
    "distance_unit": "centimeter",
    "forward_axis": "+y",
    "up_axis": "+z",
    "handedness": "right"
  },
  "target_coordinate_system": {
    "distance_unit": "centimeter",
    "forward_axis": "+x",
    "up_axis": "+z",
    "handedness": "unreal"
  },
  "artifacts": [],
  "targets": [],
  "dependencies": [],
  "validation_profile": "character_presentation_v1"
}
```

The source coordinate object records normalized evidence; the target object is
fixed. The importer applies the declared conversion and requires identity-valued
transforms in final native assets.

## Artifact records

Each artifact record contains:

- `artifact_id`: deterministic package-local identity;
- `role`: registered semantic role such as `skeletal_mesh`, `base_color`,
  `animation_sequence`, `dialogue_wave`, or `mission_rows`;
- `relative_path`: normalized plan-relative path;
- `media_type`: registered media type;
- `sha256`: lowercase content digest;
- `byte_length`: exact non-negative length;
- `required`: explicit Boolean;
- `metadata`: role-specific typed object.

The importer rejects undeclared files, missing required artifacts, duplicate
artifact identities, digest mismatch, path traversal, unsupported media types,
ambiguous roles, and fields not accepted by the selected schema revision.

## Target records

Each target record contains:

- `primary_asset_id` when the target is a Primary Asset;
- `object_path` using the exact canonical package and object name;
- `native_class` as a class-restricted Unreal path;
- `source_artifact_ids` in deterministic order;
- `asset_bundles` from the fixed bundle vocabulary;
- `properties` using the family-specific schema;
- `verification` containing required read-back checks;
- `promotion_group` for atomic publication.

## Publication transaction

The importer implements these states:

1. `planned`: schema, identity, paths, hashes, and dependencies are valid;
1. `staged`: native assets exist only under a transaction-specific staging root;
1. `read_back`: native classes and properties are inspected independently;
1. `validated`: structural, semantic, visual, and dependency checks pass;
1. `published`: final packages are atomically promoted and registered;
1. `verified`: Asset Manager and catalog resolve the final identities;
1. `rolled_back`: staged or partially promoted state is removed or restored.

No shipping definition may reference a staging package. A failed transaction
publishes nothing. Re-running the same accepted plan from a clean project
produces the same logical object paths, Primary Asset identities, dependency
graph, and read-back values.

The pipeline decides taxonomy, identity, target class, intended properties, and
required evidence. The editor adapter applies the plan and reports native state.
The adapter must not classify packages, infer missing roles, select a different
folder, invent a material, repair an invalid rig silently, or accept an editor
default that conflicts with the plan.

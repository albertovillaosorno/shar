# Characters, rigs, animation, and selection

- Status: Active
- Last reviewed: 2026-07-18

## Native asset set

Each character has one `USharCharacterDefinition` and one or more
`USharCharacterPresentationDefinition` assets. A presentation is one complete
appearance: Skeletal Mesh, Skeleton, Physics Asset, material instances, rig
profile, shared animation library, eye profile, and optional attached-prop
profile.

A costume or mod replacement selects a complete presentation. Runtime does not
assemble hidden body pieces from source files, infer outfit parts from names, or
retarget an arbitrary skeleton during spawn.

## Canonical placement

<!-- markdownlint-disable MD013 -->
```text
/Game/SHAR/Data/Characters/<id>/DA_Character_<id>
/Game/SHAR/Data/Characters/<id>/DA_CharacterPresentation_<id>_<variant>
/Game/SHAR/Art/Characters/<id>/Meshes/SK_Character_<id>_<variant>
/Game/SHAR/Art/Characters/Rigs/<rig_family>/SKEL_<rig_family>
/Game/SHAR/Art/Characters/Rigs/<rig_family>/DA_Rig_<rig_family>
/Game/SHAR/Art/Characters/<id>/Physics/PHYS_Character_<id>_<variant>
/Game/SHAR/Data/Characters/AnimationLibraries/DA_CharacterAnimationLibrary_<rig_family>
/Game/SHAR/Art/Characters/AnimationBlueprints/<rig_family>/ABP_Character_<rig_family>
/Game/SHAR/Art/Characters/Animations/<rig_family>/<clip_group>/<animation_asset>
/Game/SHAR/Art/Characters/<id>/Materials/MI_Character_<id>_<surface_role>
/Game/SHAR/Art/Characters/<id>/Textures/T_Character_<id>_<surface_role>_<texture_role>
```
<!-- markdownlint-enable MD013 -->

Shared Skeletons, Animation Blueprints, and every Animation Sequence, Montage,
Blend Space, Pose Asset, and animation-support asset live under the central rig-
family roots. No shipping animation asset is duplicated under a character
folder. The binding and deduplication contract is defined by [Shared character
animation library](shared-character-animation-library.md).

## Normalized source package

A character presentation package contains:

- one canonical binary FBX 7.7 skeletal scene;
- external texture files defined by the material contract;
- one semantic preparation manifest describing mesh sections, surface roles, UV
  channels, eye layers, presentation variant, and texture bindings;
- one rig manifest mapping semantic bone and socket roles;
- one typed reference to the compatible shared animation library;
- no copied common animation payloads inside the character package; and
- one `unreal-import-plan.json` with final native targets and verification.

The production FBX contains no embedded texture payload. Source coordinate
policy is recorded in the plan. Final native assets always use centimeters,
positive X forward, positive Z up, and applied transforms.

## Geometry and transform

The skeletal root is at the ground projection beneath the pelvis in reference
pose. Mesh and skeleton import transforms are identity-valued in the final
asset. Negative scale, non-uniform skeleton scale, unapplied scene rotation,
hidden correction parents, NaN values, degenerate rendered triangles, and
zero-area UV triangles outside declared exceptions are rejected.

A presentation declares expected height, bounds, capsule radius and half-height,
mesh relative transform, floor offset, LOD profile, shadow profile, and quality
policy. The importer validates these against native read-back.

## Rig profiles

Runtime code never hardcodes source bone names. A `SharRigProfile` maps semantic
roles to actual bones and sockets. Humanoid profile revision one requires:

- root, pelvis, spine lower, spine upper, neck, and head;
- left and right clavicle, upper arm, lower arm, hand;
- left and right thigh, calf, foot, and toe or declared toe fallback;
- optional jaw, eye, eyelid, finger, twist, facial, prop, and cloth roles;
- sockets for camera target, voice origin, left hand, right hand, back prop,
  vehicle seat alignment, and interaction origin.

Different skeleton topologies are valid when they satisfy a registered profile.
Characters are not forced into one historical skeleton. Animation sharing occurs
only within an explicitly compatible rig family or through an offline
deterministic retarget recipe whose result is a native Animation Sequence.

## Skinning

- all rendered vertices have normalized non-negative weights;
- no vertex references an absent bone;
- desktop maximum is eight influences per vertex;
- mobile cooked variants use four influences when the target profile requires
  it;
- zero-weight clusters, duplicate clusters, invalid bind matrices, and NaN
  values are rejected;
- mesh, Skeleton, reference pose, Physics Asset, and animation track
  compatibility must agree before publication.

## LOD contract

Every shipping character declares one LOD profile. `character_standard_v1` uses:

<!-- markdownlint-disable MD013 -->
| LOD | Screen-size intent | Triangle target relative to LOD0 | Notes |
| :--- | :--- | :--- | :--- |
| 0 | Close gameplay and cinematics | 100 percent | Full silhouette and deformation |
| 1 | Ordinary third-person play | at most 65 percent | Preserve face, hands, and major curves |
| 2 | Medium distance | at most 35 percent | Reduce small accessories and hidden loops |
| 3 | Far actor representation | at most 15 percent | Preserve silhouette and material identity |
<!-- markdownlint-enable MD013 -->

A tiny source character may share geometry between adjacent LODs only when the
plan records that reduction would create worse deformation or no measurable
gain. Nanite is not assumed for Skeletal Meshes; the profile follows supported
native skeletal rendering capabilities.

## Animation assets

Animation Sequences use semantic clip identities, explicit sample rate, frame
range, looping policy, root-motion policy, curves, notifies, sync markers, and
additive settings. Runtime never constructs sequence names from character
prefixes.

All compatible clips belong to one shared library per rig family. The first
common humanoid library imports the best validated universal clips once, even
when their source evidence was historically associated with one character.
Character-specific performances remain centrally stored and use explicit
eligibility predicates; they are not duplicated into character packages.

The shared library groups clips by roles such as locomotion, idle, interaction,
dialogue, impact, fall, recovery, vehicle entry, vehicle exit, and mission
action. Animation Blueprints consume movement and gameplay state through typed
interfaces, Gameplay Tags, and animation-instance data; they do not own mission
or progression rules.

Animation sampling defaults to 30 frames per second when normalized evidence is
30 Hz. A different rate is retained only when declared and verified. Unreal
import must preserve clip duration and key timing; editor preview speed is never
accepted as timing evidence.

## Physics and collision

Each presentation has a Physics Asset with named semantic bodies and
constraints. Gameplay collision uses the Character capsule and Character
Movement Component. Ragdoll, hit reaction, traces, and cosmetic overlap use
declared bodies and channels. Bone names are resolved through the rig profile.

## Character definition fields

The definition contains canonical identity, display name, tags, roles, default
presentation, allowed presentation variants, movement profile, ability set,
interaction capabilities, voice profile, camera profile, AI eligibility,
footprint profile, unlock policy, fallback policy, and soft references to
optional content.

The first C++ batch implements shared identity, character definition,
presentation, and load-free validation. Additional runtime fields require tests
and a versioned schema change.

## Character selector and progression

The main menu and in-world character selection flow enumerate the catalog, not a
hardcoded array. Every character is visible from the beginning with one state:
locked, temporarily unavailable, selectable, mission-forced, or mod-owned.

Mission completion may grant a permanent `unlock_character` reward. The default
campaign progressively unlocks the complete eligible character roster. Outside
missions, the player may select any currently available unlocked character.
Missions may force or restrict a character through explicit mission policy and
restore the prior free-roam selection afterward.

Selection is transactional:

1. validate progression and current mission policy;
1. load the target definition and required bundles asynchronously;
1. validate presentation, abilities, camera, audio, and world compatibility;
1. transfer player state through declared persistent attributes;
1. replace or reconfigure the pawn at a safe location;
1. publish the new selected identity; and
1. unload obsolete bundles when no other scope retains them.

Failure leaves the existing character active and reports the missing invariant.

## Mod extension

A mod may add a namespaced character, presentation, costume, rig family,
animation catalog, ability set, or selector category. Replacing a base
presentation does not change the base character identity or save meaning. A mod
that changes gameplay behavior declares compatibility and authority effects
separately from visual replacement.

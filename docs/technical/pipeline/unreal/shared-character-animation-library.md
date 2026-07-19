# Shared character animation library

- Status: Active
- Last reviewed: 2026-07-18

## Purpose

This contract defines the only native destination for character animation
assets. The pipeline imports each compatible clip once into a shared library.
Character packages contain models, materials, rig bindings, physics, and
presentation data; they do not contain copied common animation sequences.

## Canonical roots

```text
/Game/SHAR/Data/Characters/AnimationLibraries/
/Game/SHAR/Art/Characters/Animations/<rig_family>/<clip_group>/
/Game/SHAR/Art/Characters/AnimationBlueprints/<rig_family>/
/Game/SHAR/Art/Characters/Rigs/<rig_family>/
```

No shipping animation asset may live below
`/Game/SHAR/Art/Characters/<character_id>/`. Character identity is not an
animation storage boundary.

## Primary Asset

Each compatible rig family has one `USharCharacterAnimationLibraryDefinition`
with Primary Asset type `SharCharacterAnimationLibrary` and object name:

```text
DA_CharacterAnimationLibrary_<rig_family>
```

The first common humanoid family is `humanoid_common_v1`. It owns the validated
locomotion and ordinary gameplay bank used by all compatible characters. The
pipeline may derive that bank from any accepted source package, including a
package historically associated with one character. Native ownership remains the
shared rig family.

## Rig-family compatibility

A character presentation may reference a library only when all of these match:

- required semantic bone roles;
- Skeleton identity and revision, or an accepted offline retarget result;
- reference-pose compatibility profile;
- scale and root-orientation policy;
- curve, notify, marker, and slot schemas;
- root-motion policy;
- Animation Blueprint family;
- required interaction and vehicle-handoff sockets.

Visual body proportions may differ. Compatibility is proved by profile and
native read-back, not by character name or assumed bone count.

An incompatible topology receives a new rig family. Examples may include a non-
humanoid creature, an exceptional boss, or a future modded skeleton. It still
uses the same central root under a different rig-family directory.

## Directory taxonomy

Each rig-family directory contains semantic groups only:

```text
<rig_family>/
├── Locomotion
├── Idles
├── Traversal
├── Interaction
├── VehicleHandoff
├── Dialogue
├── Reactions
├── Combat
├── MissionActions
├── CinematicBridge
├── Additive
├── Poses
└── Tests
```

`Tests` is development-only and excluded from shipping cook rules. A new
semantic group requires a documented consumer and validation profile.

## Native names

Animation asset names do not contain a character prefix unless the clip is truly
restricted to that character and the restriction is part of the semantic clip
identity.

```text
A_<rig_family>_<clip_id>
AM_<rig_family>_<action_id>
BS_<rig_family>_<blend_id>
PA_<rig_family>_<pose_id>
CR_<rig_family>_<control_rig_id>
ABP_Character_<rig_family>
```

Examples:

```text
A_humanoid_common_v1_locomotion_walk_forward
A_humanoid_common_v1_idle_relaxed_01
AM_humanoid_common_v1_vehicle_enter_left
A_humanoid_common_v1_dialogue_homer_angry_01
```

The final example is centrally stored but explicitly eligible only for
Homer-like identity or a declared variant predicate. It is not copied into a
Homer directory.

## Library schema

The library definition contains:

- canonical rig-family identity and revision;
- compatible rig profile and Skeleton Primary Asset identities;
- Animation Blueprint class;
- default locomotion, idle, traversal, reaction, dialogue, interaction, and
  vehicle-handoff role maps;
- clip definitions with soft native references;
- clip-group and load-bundle membership;
- eligibility predicates and variant axes;
- sync groups and markers;
- montage slots, sections, and interruption policy;
- root-motion and in-place policy;
- curve and notify schemas;
- additive and pose dependencies;
- fallback chains;
- platform compression and streaming profile;
- deterministic source package identities;
- validation profile and revision token.

A clip definition contains:

- canonical clip identity;
- semantic role and Gameplay Tags;
- one native Animation Sequence, Montage, Blend Space, or Pose Asset reference;
- duration, sample rate, frame range, and looping policy;
- root-motion mode;
- compatible rig revision range;
- eligibility predicate;
- sync group, markers, notifies, curves, and slots;
- mirroring and additive policy;
- interruptibility and terminal observation policy;
- quality and platform policy;
- source artifact and generated revision evidence.

## Import package contract

An animation import package contains:

- one normalized binary FBX 7.7 animation scene or a deterministic equivalent
  normalized animation payload;
- explicit source and target rig identities;
- clip identity, semantic role, sample rate, frame range, and duration;
- root-motion, looping, additive, and mirroring declarations;
- curve, notify, marker, and event records;
- offline retarget recipe and result evidence when required;
- target object path below the shared rig-family root;
- library mutation plan that adds or updates exactly one clip definition;
- native read-back expectations.

The importer rejects a character-owned target path, undeclared Skeleton, runtime
retarget expectation, ambiguous clip role, duplicate native clip identity, or a
library mutation that changes unrelated entries.

## Common-bank deduplication

The pipeline computes a semantic and structural fingerprint for each candidate
clip. The fingerprint includes compatible rig family, normalized track set,
duration, sample rate, root-motion policy, curve schema, marker schema, and
normalized key data digest.

- byte-identical or semantically identical clips become one native asset;
- aliases point to one canonical clip identity;
- a clip with different timing, root motion, markers, curves, or eligibility is
  a distinct revision or variant;
- source-package duplication never creates duplicate UAssets;
- an improved source replaces or revisions the canonical clip only after all
  compatibility tests pass.

## Character binding

`USharCharacterPresentationDefinition` references one shared animation library
as a typed soft Primary Asset reference. It may also declare a bounded list of
eligible variant sets. It never references a private per-character animation
folder.

The character definition does not own animation assets. Character selection,
costume changes, and mod visual replacement preserve the library when rig-family
compatibility remains valid.

## Runtime selection

Runtime requests a semantic animation role. The library resolves it using:

1. exact required gameplay and presentation tags;
1. rig-family compatibility;
1. character, costume, vehicle, interaction, and mission eligibility predicates;
1. requested variant axes;
1. deterministic weighted choice where allowed;
1. fallback chain.

The resolver returns a typed clip identity and retained soft-asset load request.
It never concatenates object paths or scans folders.

## Mods

A mod may:

- add a namespaced rig-family library;
- add namespaced clips to a compatible library through a validated overlay;
- replace one clip identity with declared priority and compatibility;
- add character-specific or mode-specific eligibility variants;
- provide offline-retargeted results for its own rig family.

A mod may not duplicate the complete base bank merely to replace one clip.
Overlay activation and teardown are transactional.

## Validation

Publication proves:

- every required semantic role resolves exactly once or through a declared
  fallback;
- no native sequence is duplicated under character folders;
- all soft references remain within approved base or mod roots;
- native Skeleton, duration, sample rate, root motion, curves, markers, and
  notifies match the plan;
- shared clips deform every declared representative character correctly;
- vehicle-handoff clips satisfy seat and socket choreography profiles;
- repeated generation preserves object paths, library ordering, and
  fingerprints;
- loading one character does not force the complete animation corpus resident;
- bundles load only the required clip groups;
- a failed update leaves the prior library revision intact.

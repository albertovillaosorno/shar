# Missions, gameplay, rewards, and saves

- Status: Active
- Last reviewed: 2026-07-18

## Mission representation

Every mission is a `SharMission` Primary Asset. The pipeline produces typed
mission data and bindings. It does not generate arbitrary executable code and
does not create one bespoke Blueprint or StateTree graph per mission.

Mission execution uses one bounded library of native C++ StateTree tasks,
evaluators, conditions, and domain services. Mission definitions select
templates, parameters, participant identities, objective policies, routes,
camera intents, audio, rewards, checkpoint policy, and transitions.

## Canonical placement

<!-- markdownlint-disable MD013 -->
```text
/Game/SHAR/Data/Missions/<chapter_id>/<mission_id>/DA_Mission_<chapter_id>_<mission_id>
/Game/SHAR/Data/Missions/<chapter_id>/<mission_id>/DT_MissionStages_<chapter_id>_<mission_id>
/Game/SHAR/Data/Missions/Templates/ST_Mission_<template_id>
/Game/SHAR/Data/Rewards/<reward_id>/DA_Reward_<reward_id>
```
<!-- markdownlint-enable MD013 -->

## Mission input format

Normalized mission data is UTF-8 JSON matching
`shar.unreal.mission-definition.v1`. Arrays are ordered explicitly and maps use
canonical keys. Free-form executable script, local file path, editor object
path, source-language callback name, integer event code, and arbitrary console
command are forbidden.

## Required mission fields

A mission definition contains:

- canonical mission and chapter identities;
- display text and localization keys;
- sequence ordinal and mission class;
- availability, prerequisite, and lock conditions;
- offered-by and playable-character policies;
- forced, allowed, or prohibited vehicle policies;
- world and Data Layer composition;
- ordered stage records;
- participant and spawn bindings;
- route, checkpoint, destination, and recovery bindings;
- presentation, camera, dialogue, music, and HUD profiles;
- success, failure, abort, retry, and checkpoint transitions;
- reward transaction;
- save and compatibility revision;
- required asset load plans and fallback policy.

## Stage records

Each stage has stable mission-scoped identity, dense zero-based order, objective
kind, parameter schema, success and failure conditions, optional time policy,
target and participant identities, world policy, checkpoint policy, presentation
requests, and explicit transitions.

Registered objective kinds include:

- `talk`;
- `enter_vehicle`;
- `exit_vehicle`;
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
- `protect`;
- `interact`;
- `boss_phase`;
- `action_sequence`.

A new objective kind requires a native policy implementation, schema, tests, and
versioned registration. Free-form script text is rejected.

## Participant bindings

Characters, vehicles, props, zones, routes, cameras, dialogue lines, effects,
and world actors are referenced by canonical identities and semantic roles.
Mission data does not bind an actor by label, source filename, fixed pool index,
or package path. A world placement record resolves the identity for the active
world revision.

## Rewards and unlocks

Rewards are independent Primary Assets and apply atomically. Registered
operations include:

- `grant_currency`;
- `unlock_character`;
- `unlock_vehicle`;
- `unlock_costume`;
- `unlock_ability`;
- `unlock_world_region`;
- `unlock_activity`;
- `grant_collectible`;
- `set_progression_flag`;
- `grant_achievement_progress`.

Completing selected missions progressively unlocks characters for the character
selector. Vehicle rewards or purchases grant phone-booth availability. Mission-
forced characters and vehicles do not imply permanent ownership.

The reward transaction is idempotent. Loading a checkpoint, reconnecting to a
self-hosted server mode, replaying a completed mission, or recovering after a
crash cannot duplicate permanent rewards.

## Gameplay definition model

Attributes, abilities, damage, stamina, combat, traversal, interaction,
notoriety, status effects, pickups, collectibles, gags, races, taxi work,
bosses, and world state use versioned definitions and semantic tags. Gameplay
Ability System may provide execution and replication, but SHAR definitions own
identity, save meaning, permission, and mod compatibility.

One mission or character cannot subclass an unrelated concrete gameplay class to
borrow behavior. Reuse occurs through composition, abilities, policies,
interfaces, and registered StateTree tasks.

## Save contract

Save data stores canonical identities, compact domain state, schema versions,
transaction revisions, and namespaced mod state. It never stores raw UObject
pointers, object package paths as canonical identity, editor actor labels,
source filenames, or transient World Partition package names.

Save migration is explicit and deterministic. Missing optional mod content is
quarantined or replaced according to declared fallback policy. Missing required
base definitions fail with actionable diagnostics rather than silent reset.

The save model separates:

- account-independent local profile and settings;
- campaign progression and world state;
- mission checkpoint state;
- current character and vehicle selection;
- inventory, currency, collectibles, purchases, abilities, and achievements;
- namespaced mod state;
- community-server state, which remains owned by that server and is never merged
  automatically into the local campaign.

## Camera modernization

Mission data requests camera intents and authored profiles. It never reproduces
historical camera transforms or writes camera state directly. The camera service
may improve collision, framing, look-ahead, speed response, transitions,
accessibility, and input behavior while preserving scene intention.

## Mod extension

Mods may add mission definitions, objective policies from an approved extension
registry, rewards, activities, abilities, and world content. Native-code mods
are a separate trust tier. Data-only mods cannot execute arbitrary code.

## Validation

Publication rejects duplicate stage identities, non-dense order, unreachable
transitions, missing terminal outcomes, unresolved participants, impossible
reward operations, circular prerequisites, missing world layers, unsupported
objective kinds, non-idempotent permanent rewards, unversioned save fields, or
runtime paths embedded in domain identity.

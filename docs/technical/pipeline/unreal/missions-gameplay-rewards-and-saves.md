# Missions, gameplay, rewards, and saves

- Status: Active
- Last reviewed: 2026-08-08

## Mission representation

Every mission is a `SharMission` Primary Asset. The pipeline produces typed
mission data and bindings. It does not generate arbitrary executable code and
does not create one bespoke Blueprint or StateTree graph per mission.

Mission execution uses one bounded library of native C++ StateTree tasks,
evaluators, conditions, and domain services. Mission definitions select
templates, parameters, participant identities, objective policies, routes,
camera intents, audio, rewards, checkpoint policy, and transitions.

## Canonical placement

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Unreal asset path is indivisible -->
- `/Game/SHAR/Data/Missions/<chapter_id>/<mission_id>/DA_Mission_<chapter_id>_<mission_id>` <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Unreal asset path is indivisible -->
- `/Game/SHAR/Data/Missions/<chapter_id>/<mission_id>/DT_MissionStages_<chapter_id>_<mission_id>` <!-- markdownlint-disable-line MD013 -->
- `/Game/SHAR/Data/Missions/Templates/ST_Mission_<template_id>`
- `/Game/SHAR/Data/Rewards/<reward_id>/DA_Reward_<reward_id>`

## Legacy mission-script intake

Source MFK scripts are normalization evidence, not runtime mission data. The
straggler normalizer emits
`shar-schoenwald.straggler.mission-script.v3` with canonical command
invocations plus explicit mission, stage, objective, and condition context
evidence. Parsing finds the matching call parenthesis outside quoted text, so
trailing comments never enter `args_raw` or the final positional argument, and
nested argument groups remain intact. Version 3 also carries reviewed structural
compatibility adaptations. Each adaptation is bound to one logical source path,
command ordinal, command identity, and surrounding invocation fingerprint; the
semantic consumer independently revalidates that fingerprint before accepting
it. The current corpus has exactly two such adaptations and no unresolved
context findings after they are applied.

A closed preflight registry then validates every observed `AddObjective` and
`AddCondition` alias, exact observed arity, and every command that occurs inside
those scopes. Unknown aliases, cross-scope commands, and unobserved argument
counts fail closed. Direct mission- and stage-scope commands are also closed by
an observed scope-and-arity registry. A command observed both outside a selected
mission and inside one requires an explicit general-scope registration for that
form; unrelated utility commands remain raw unscoped evidence. Positional values
remain raw source evidence until a reviewed typed parameter compiler owns them.
These registries are conversion evidence only: they do not claim that legacy
parameters have already been compiled into final runtime policies.

Semantic intake independently replays the complete mission, stage, objective,
and condition stack instead of trusting an empty producer finding list. It also
recomputes the command histogram, mission-flow and vehicle-physics summaries,
`loadp3d` reference inventory, and each invocation's semantic role from the
normalized source statements. Any disagreement between those derived fields and
the v3 document fails before scope projection.

After those gates pass, a lossless source-scope graph binds reviewed objective
and condition aliases and their registered modifiers to their exact owning
stage. It retains uninterpreted `AddStage` arguments plus every direct command at
unscoped, mission, or stage scope in source order. Each observed stage must own
exactly one root objective. A modifier visible through both an objective and a
nested condition is retained only by the most-specific condition scope rather
than duplicated. The graph still publishes no Unreal asset and assigns no new
gameplay meaning to positional arguments.

The authoritative source corpus currently projects 154 mission graphs containing
611 stages and 611 root objectives, with 408 conditions: 402 declared directly
at stage scope and 6 inside an objective. Two `dummy` objectives remain explicit
unavailable semantic results. The same corpus freezes 7,705 commands outside a
selected mission, 811 direct mission commands, 2,454 direct stage commands,
3,605 reviewed objective commands, and 375 reviewed condition commands. A change
in those inventories must be reviewed rather than silently accepted as a new
compiler rule.

Parameter compilation is alias-specific rather than positional across all
objectives or conditions. All 611 direct objective parameter shapes and all 408
direct condition parameter shapes now have closed typed representations.
`buycar`, `buyskin`, and the reviewed `getin` form publish exact source
references; race wager and road-arrow tokens are typed independently. The one
source spelling `niether` is retained as explicitly unrecognized legacy evidence
rather than repaired to `neither`. The ten `keepbarrel` numeric values and one
`damage` token are likewise typed as closed legacy shapes without assigning
undocumented gameplay meaning.

Stage compilation then types all 611 source stage headers. Numeric `AddStage`
values remain opaque reviewed flags because no authoritative meaning for their
bits has been established; the explicit final marker and locked vehicle/costume
forms remain distinct. The compiler now closes 1,832 reviewed stage directives:
timers and added time, checkpoints, message indices, stage vehicles, vehicle
activation, waypoints, HUD icons, traffic maxima, fade and iris compatibility
arguments. It retains documented unused arguments exactly and preserves the
legacy `AddStageTime(0)` source value together with its reviewed effective
one-second addition. The current `SetMaxTraffic` corpus is closed to the
observed documented car-count domain 1 through 5.

Mission-scope initialization compilation covers 374 current directives: restart
locators, initial walk locators, initial player vehicles, forced-car markers,
dynamic-load data, and 22 street-race prop load plus 22 unload records.
Dynamic-load strings publish their exact `.p3d` references while an observed
second argument remains named only as legacy evidence until its semantics are
independently established. Street-race prop records retain their exact source
Dyna Load Data string and independently validate the documented terminal `;`
load or `:` unload form.

After source-scope projection, selected objective-command compilation now emits
2,873 typed source directives from the 3,605 reviewed objective-scoped commands.
It covers NPC placement and waypoints, drivers and vehicle targets, talk targets,
destinations, collectibles and effects, collectible-to-waypoint bindings,
durations, race laps, fees, dialogue participants and position locators, ambient
NPC/player animation identities, presentation bitmap P3D references, and FMV RMV
references. Authored optionality is preserved: missing talk-target values or
collectible effects do not become fabricated defaults, legacy collectible tails
remain opaque, the fourth dialogue-info value remains a closed legacy zero, and
the optional dialogue-position flag remains the reviewed legacy one. All 375
condition-scoped commands are also typed across all 408 conditions while health,
distance, time, and position source values retain only semantics established by
the reviewed command boundary; no undocumented units are invented.

These objective, condition, initialization, and stage compilers are mandatory
semantic gates for `prepare-unreal`. Camera bindings, AI tuning, pickup/reward
references, remaining presentation and transition commands, and final catalog
resolution are still pending, so no mission asset is emitted yet.

The v3 evidence records `context_command_count`,
`context_adaptation_count`, `context_finding_count`, ordered adaptations, and
ordered findings. Findings describe source structure without repairing it.
Semantic intake accepts only exact v3 JSON with a reproducible command
histogram, strictly increasing ordinals, matching context counts, independently
verified adaptations, and zero unresolved findings. An original zero-byte MFK
is preserved as an inert v3 source with zero statements, invocations, summaries,
adaptations, and findings; that exact self-consistent empty state is valid
source evidence rather than a fabricated mission. Source bytes and statement
emptiness must agree in both directions: zero bytes with statements or nonzero
bytes with zero statements is contradictory and fails closed. `prepare-unreal`
applies the preflight only to normalized `mission-script` sources before they
can contribute Unreal source evidence.

The current source audit contains two reviewed structural defects: one orphan
condition close and one missing condition close before stage completion. Both
are handled only by their exact path-and-command-window adaptation fingerprints;
any path, command, argument, or ordinal drift restores the finding and blocks
semantic conversion instead of guessing intent.

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

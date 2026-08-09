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
forms remain distinct. The stage report now emits 2,549 typed directives: every
one of the 2,454 direct-stage commands plus all 95 objective-scoped commands
explicitly delegated to stage semantics. This includes timers, checkpoints,
messages, vehicles, characters, waypoints, countdowns, traffic, AI source tuples,
safe zones, HUD and presentation controls, music, swap locators, completion
signals, and transition markers. AI and race catch-up values remain exact source
integers or decimal lexemes where gameplay units and ordering are unresolved.

Mission-scope initialization compilation now covers all 811 direct mission
commands with no raw fallback. Restart locators, initial walk and vehicle state,
forced-car state, dynamic-load and street-race P3D references, player-car
placement, state props, animated/start cameras and multi-controllers, failure
hint counts, presentation bitmaps, HUD visibility, and pedestrian-group indices
all pass closed value or reference validators. Dynamic-load compatibility fields
whose meaning remains unresolved stay explicit legacy evidence.

Objective-command ownership is exhaustive across all 3,605 reviewed
objective-scoped commands. 3,498 emit objective directives directly, 95 are
required to emit stage directives through one shared delegation registry, and 12
are the structural `AddCondition`/`CloseCondition` pairs. A registered objective
command with no owner now fails semantic preflight instead of disappearing. All
375 condition-scoped commands are likewise typed across all 408 conditions.
Authored optionality and source spelling are preserved, including missing talk
target options, collectible extension tails, dialogue compatibility fields,
`niether`, source AI tuples, and other values whose gameplay meaning is not yet
established.

These objective, condition, mission-scope, and stage compilers are mandatory
semantic gates for `prepare-unreal`. Command-level semantic ownership is now
closed for the reviewed mission corpus, but canonical catalog resolution,
participant/route/camera binding, reward and transition policy construction, and
final topology validation remain pending. No mission asset is emitted yet.

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

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
semantic gates for `prepare-unreal`. The next gates resolve every reviewed
character and vehicle source identity and every explicit `LoadP3DFile` first
argument against the already validated phase-three package index before a
mission source can contribute Unreal evidence. Character skeleton source names
resolve to one canonical participant while retaining the exact base-model,
costume, or crowd package subcategory; vehicle source names resolve to one exact
`cars` package. The runtime `current` vehicle token remains symbolic, and the
reviewed `AddStageVehicle` driver token `none` emits no false character package
reference. Missing or ambiguous referenced participant names and unindexed P3D
loads fail closed. The source loader accepts an optional heap name and an
optional inventory-section override; migration validates and preserves both only
as source provenance, never as target allocation authority or additional asset
references. The base mission corpus has 950 one-argument calls and 16
two-argument calls using `GMA_LEVEL_OTHER`, with no authored third argument.
Producer and replay summaries count only the first path argument as P3D evidence.

The same portable P3D path authority now backs typed presentation references.
All 61 `SetPresentationBitmap` directives across initialization, stage, and
objective semantics bind to canonical phase-three packages through one shared
catalog built once per `prepare-unreal` run. The current corpus contains 56
unique presentation paths, all under the `ui-images/mission-briefing` taxonomy,
with zero missing bindings or normalized-root collisions. This resolves package
identity only; presentation timing and drawable selection remain separate
runtime semantics.

Reviewed initialization camera identities are also package-backed now. Exact
embedded names from decoded `camera` and `multi_controller` components are
cataloged with their package/member provenance and source level. Across 194
`SetMissionStartCameraName`, `SetMissionStartMulticontName`, and animated-camera
multi-controller references, global name lookup would be ambiguous for 190
references; qualifying by the exact `levelNN` of the source mission script makes
all 194 references unique with zero missing candidates. Four unreferenced
level-local component keys still have multiple candidates; those candidates are
preserved so a future reference remains explicitly ambiguous. The binding uses
`(source level, component kind, exact embedded name)` and never falls back to a
global-name winner. Camera timing, blending, playback, and presentation behavior
remain separate runtime concerns.

Mission locator intake and typed command binding are package-backed now. The
local adapter validates both observed `p3d-locator` member families but adds
only decoded `srr_locator` records to the mission catalog. It reads the embedded
JSON `name`, trims only trailing NUL padding, preserves the exact decoded source
type, and binds the row to its package id and member id. Package-local duplicate
names fail closed; cross-package duplicates remain an explicit `Ambiguous`
result. This is required because locator names are level/load-contextual and can
occur in more than one package, while extracted filenames sanitize trailing NUL
padding to underscores and the source corpus also contains genuinely authored
trailing underscores. Filename trimming and global-name lookup are therefore not
canonical identity rules.

For each source that selects exactly one mission, `prepare-unreal` now pairs the
exact `<mission-id>i.mfk.json` source with `<mission-id>l.mfk.json` and the
longest matching sibling `*level.mfk.json` family. The pairing is keyed by full
source path rather than mission id, so repeated ids such as `m1` cannot leak
package context across levels. Before this cross-source preflight, the mission
source is re-read through the stable-source guard and its size and SHA-256 must
match the already verified Unreal source evidence. Typed initialization, stage,
and objective locator fields then resolve against the explicit level-load and
mission-load packages plus indexed P3Ds loaded at mission start by typed
`SetDynaLoadData` and `StreetRacePropsLoad` evidence. A shared Dyna Load Data
parser now preserves ordered region load/unload, interior load/unload, and World
Sphere enable/disable postfix operations. Base mission initialization
remains fail-closed to its observed load-only subset, including the one corpus
form whose final region P3D omits its terminal postfix. Dyna P3D paths use the
source
format's implicit `art/` root unless that prefix is already explicit, and every
resulting package root must exist in the phase-three index. Unload
evidence does not become an active package. Post-start base-game evidence now
also includes decoded type-5 `DynamicZone` locators. Their Dyna strings compile
to the same ordered package-transition model, and `prepare-unreal` requires every
P3D load effect to bind to an indexed package while treating an absent unload as
a deterministic remove-if-present effect. The current corpus contains 109 such
zones, 372 indexed P3D loads, and 728 P3D unloads; 30 unload targets are absent
from the extracted package index. No observed base-game Dyna string both loads
and unloads the same P3D target; an order-sensitive conflict remains unresolved
rather than assuming textual order. Child-trigger occupancy is explicit: the
first child-volume entry of an episode executes the zone transition, overlapping
child volumes do not retrigger it, and final exit rearms a later entry without
undoing the prior transition. No player traversal path is inferred from trigger
geometry.

Documented Event and CarStart roles receive exact type constraints, and
`ActivateVehicle(..., "NULL", ...)` emits no false locator reference. Generic
missing or duplicate candidates remain `Missing` or `Ambiguous`. Camera
best-side is narrower: its locator lookup is deferred until mission reset, when
Pure3D starts with its Default inventory section selected and searches the
current section before remaining sections in creation order. This load path
creates Level before Mission, so reviewed duplicate `bm1_bestside` cases choose
the Level candidate. The 18 references face 10 Type-3 `CarStart` candidates,
while none of the 1,100
DynamicZone P3D effects targets those mission packages, confirming streaming
does not decide the lookup. Other locator roles remain fail-closed until their
runtime lookup paths are traced separately.

Stage completion markers are also separated by authority before final graph
emission. The reviewed corpus has 6 iris and 14 fade requests, 5 stay-black
markers, 108 stage-complete presentation markers, 3 level-over terminals, and 1
game-over terminal. Iris is the effective visual transition in the one stage
that authors both iris and fade. Stay-black and stage-complete remain
presentation-only; they cannot select a mission successor. Level-over maps to a
chapter terminal override and game-over to game completion. Successor, retry,
rollback, and recovery topology still requires a complete graph compiler.

`BindReward` binds all 90 reviewed P3D references through the shared canonical
package catalog while preserving its source policy tokens without assigning
unlock or economy behavior. Route topology, remaining reward behavior,
transitions between non-terminal stages, remaining catalog binding, and final
topology validation remain pending. No mission asset is emitted yet.

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

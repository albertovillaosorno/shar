# Mission definition, stage, and objective runtime

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed StateTree action sequences](../../adr/unreal/runtime/typed-state-tree-action-sequences.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission, interaction, interior, and notoriety runtime](mission-interaction-and-notoriety-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission briefing, result, and replay UI runtime](mission-briefing-result-and-replay-ui-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored spatial placement and trigger runtime](authored-spatial-placement-and-trigger-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission world-entity and respawn runtime](mission-world-entity-and-respawn-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Presentation playback runtime](presentation-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
- [Race route and opponent runtime](race-route-and-opponent-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical core-design and dialogue evidence normalization](historical-core-design-and-dialogue-evidence-normalization.md)

## Purpose

This specification defines how authored mission evidence becomes immutable
mission definitions and how the native runtime loads, starts, updates,
transitions, recovers, completes, cancels, and tears down missions, stages,
objectives, conditions, participants, and presentation requests.

It replaces runtime command interpretation, mutable parser context, mission
and stage singletons, fixed-capacity arrays, pointer-owned identity,
frame-order dependence, direct user-interface mutation, and loading
callbacks that can advance stale mission state.

A mission is a validated data graph executed by repository-owned C++
services through native Unreal facilities. It is not a script file, console
command sequence, Blueprint graph, actor list, package name,
or presentation event.

## Authority and ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Import pipeline | Parse source mission evidence and emit deterministic definitions. |
| Mission catalog | Own mission, stage, objective, condition, offer, reward, and presentation identities. |
| Mission session subsystem | Own one accepted mission session and its state transitions. |
| Mission load coordinator | Resolve bundles, Data Layers, participants, and readiness barriers. |
| StateTree adapter | Execute the shared hierarchical control graph from immutable session state. |
| Objective adapters | Evaluate one registered objective schema from typed observations. |
| Condition evaluators | Evaluate required, optional, failure, and recovery predicates. |
| Progression service | Commit attempts, skips, results, rewards, completion, and best evidence. |
| World adapters | Project actors, vehicles, characters, routes, placements, and effects. |
| Presentation services | Project camera, HUD, dialogue, music, animation, and transition requests. |

<!-- markdownlint-enable MD013 -->

No world actor, objective adapter, condition, loading callback, or
presentation callback may mutate progression or choose an
undeclared successor directly.

## Runtime topology

The mission runtime owns:

- `FSharMissionSessionId`, a unique accepted-session identity;
- `FSharMissionRevision`, the catalog and definition revision;
- `FSharMissionStageId`, a stable mission-scoped stage identity;
- `FSharMissionStageRevision`, one activation of one stage;
- `FSharMissionObjectiveId`, a stable objective binding identity;
- `FSharMissionObjectiveRevision`, one objective activation;
- `FSharMissionTransitionId`, a unique requested transition;
- `FSharMissionCheckpointId`, an accepted recovery boundary;
- `FSharMissionLoadPlanId`, a deterministic readiness plan;
- `FSharMissionParticipantBindingId`, a stable role binding;
- `USharMissionDefinition`, immutable mission data;
- `FSharMissionStageRow`, immutable stage data;
- `FSharObjectivePolicyRow`, immutable objective policy;
- `FSharMissionConditionDefinition`, immutable condition policy;
- `USharMissionPresentationProfile`, non-authoritative presentation data;
- `USharMissionSessionSubsystem`, one world-scoped mission authority;
- `USharMissionDefinitionCompiler`, an editor and commandlet compiler; and
- registered C++ objective, condition, transition, and recovery adapters.

Runtime projections use weak engine handles correlated with stable
identities and revisions. Actor names, object pointers, array indexes,
source mission numbers, console function names, and package load
order are not authority.

## Build-time definition compiler

Mission source evidence is converted before cooking. The compiler consumes
the validated extraction contract and emits one deterministic mission graph.

The compiler:

1. opens one explicit conversion session;
1. resolves source aliases to canonical catalog identities;
1. validates command schema, argument count, units, and enum values;
1. maintains an explicit mission, stage, objective, and
   condition context stack;
1. rejects commands outside their declared context;
1. resolves every character, vehicle, locator, route, interior, reward,
   media, camera, audio, presentation, and bundle reference;
1. emits mission, stage, objective, condition, participant, loading, and
   presentation definitions;
1. validates transition topology and recovery reachability;
1. records provenance and conversion findings;
1. serializes definitions in canonical identity order; and
1. closes only after the complete graph passes validation.

Runtime packages never execute source mission scripts, register a source
command console, parse positional argument arrays, or create gameplay
objects while interpreting source text.

## Conversion command registry

Every recognized source command maps to one registered conversion
schema. A schema declares:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `CommandAlias` | Source-only name accepted by the converter. |
| `Scope` | Level, mission, stage, objective, condition, or presentation. |
| `MinimumArguments` | Exact lower bound. |
| `MaximumArguments` | Exact upper bound. |
| `ArgumentSchemas` | Ordered typed argument definitions. |
| `OutputKind` | Definition, row, binding, transition, or finding. |
| `RequiredContext` | Exact open conversion context. |
| `ReferenceKinds` | Canonical identities that must resolve. |
| `ConflictPolicy` | Duplicate, override, merge, or rejection rule. |
| `DeprecationPolicy` | Replacement or explicit unsupported finding. |

<!-- markdownlint-enable MD013 -->

Aliases that historically selected the wrong handler, depended on a global
current object, accepted contradictory forms, or had no complete behavior do
not become runtime compatibility APIs. Conversion either maps them to the
intended stable contract with evidence or reports them as unavailable.

Unknown commands, extra positional arguments, missing close operations,
out-of-order commands, unresolved references, and context leaks fail
conversion. The compiler never guesses the active mission, stage,
objective, or condition.

## External proposal conversion boundary

External partner, licensor, publisher, or writer-facing mission frameworks
follow
<!-- markdownlint-disable-next-line MD013 -->
[Historical core-design and dialogue evidence normalization](historical-core-design-and-dialogue-evidence-normalization.md).
They are proposal sets, not source mission scripts and not runtime definitions.

A proposal fact can contribute only after conversion resolves:

- one canonical chapter, mission, stage, objective, participant, vehicle, route,
  location, interior, reward, dialogue, camera, audio, or presentation identity;
- one accepted mission family and typed objective adapter;
- explicit prerequisites, readiness dependencies, failure, recovery, completion,
  persistence, and teardown behavior;
- conflicts with other proposal drafts and current public contracts; and
- one terminal accepted, adapted, superseded, rejected, or unresolved result.

Draft mission-type lists do not create new objective adapters automatically.
Prose mission pitches, questions, brainstorming, fixed level ownership,
one-boss-
per-level assumptions, source ordering, and stakeholder notes cannot enter the
mission graph. An unresolved proposal publishes no partial mission, stage,
participant, placement, or progression definition.

## Historical mission-bundle reconciliation

Per-level breakdowns, level summaries, mission outlines, functionality records,
presentation records, race designs, and source mission scripts may describe the
same candidate mission graph. The compiler groups them by canonical chapter,
mission, stage, objective, participant, route, and presentation identities
before
selecting facts.

Reconciliation records:

- source fact and revision identities;
- accepted aliases for historical level, mission, and stage labels;
- candidate objective adapter and condition schemas;
- participant, vehicle, route, placement, reward, dialogue, and presentation
  references;
- conflicts, duplicates, omissions, and supersession decisions; and
- one complete accepted graph or an explicit unresolved result.

Source mission numbers, document order, folder labels, headings, prose step
numbers, script commands, and apparent revision age are not authority. Exact
content duplicates collapse to one fact set. A changed copy is compared fact by
fact and cannot partially overwrite an accepted mission.

Mission-editing instructions that mention source DCC scenes, locator files,
checkout operations, source-control paths, or workstation procedures generate no
runtime or target-authoring contract. They remain historical implementation
provenance only.

Mission title, hint, and icon evidence is referenced through stable presentation
identities. The mission graph never selects a proposal column, parses display
text,
or uses source color keys as gameplay state.

## Mission definition envelope

Each mission definition contains:

- canonical mission and chapter identities;
- mission family and sequence identity;
- availability and offer definitions;
- controlled-character and default-vehicle policies;
- ordered stage rows;
- optional mission-wide conditions and bonus objectives;
- mission load and release plans;
- restart, abort, skip, replay, and checkpoint policies;
- mission presentation profile;
- reward and progression transaction policies;
- final transition and world-expansion policy;
- mod overlay and compatibility declarations; and
- catalog, schema, and conversion revisions.

A mission definition is complete only when every stage, transition,
condition, objective policy, participant binding, bundle, and
terminal path resolves.

## Mission session states

One accepted mission session has exactly one state:

- `idle`;
- `accepted`;
- `loading`;
- `ready`;
- `active`;
- `transitioning`;
- `succeeded`;
- `failed`;
- `recovering`;
- `cancelled`; or
- `completed`.

`accepted` reserves the mission identity and gameplay-state lease without
projecting mission actors.

`loading` executes the correlated mission load plan. It cannot
update mission simulation.

`ready` means every required bundle, Data Layer, participant, route,
placement, and adapter has passed its readiness barrier. The session has not
started the first stage until the start transaction commits.

`active` owns exactly one active stage revision and one
root objective revision.

`transitioning` finalizes the current stage, executes its declared
transition, verifies postconditions, and activates the successor.

`succeeded` means the final stage result is accepted but durable
completion has not committed.

`failed` records one typed terminal cause and the
applicable recovery policy.

`recovering` restores an accepted checkpoint and validates the reconstructed
world before reactivation.

`cancelled` is a terminal non-completion result for an accepted abort,
declined offer, revoked dependency, or explicit user cancellation.

`completed` is entered only after the progression transaction
commits exactly once.

## Mission load transaction

A mission load plan contains:

- mission and session revisions;
- required primary-asset bundles;
- Runtime Data Layer and world-composition requests;
- stage-zero participant, route, placement, and interaction dependencies;
- forced-character, costume, and vehicle requirements;
- mission-only actor and payload definitions;
- presentation, audio, camera, and user-interface bundles;
- prefetch and residency priorities;
- timeout and cancellation policy;
- rollback operations; and
- readiness verification probes.

Loading is asynchronous and correlated. Completion from an older plan,
mission, stage, world, feature, or package revision is ignored. Required
failures roll back all acquired handles and leave the session before start.

A loading callback cannot start a stage. It only publishes one typed load
result to the session, which revalidates the full readiness barrier.

Optional presentation may degrade according to its policy. Missing gameplay
assets, participant bindings, routes, collision, conditions, or progression
schemas reject readiness.

## Mission stage row

`FSharMissionStageRow` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `MissionId` | Owning canonical mission identity. |
| `StageId` | Stable mission-scoped stage identity. |
| `SequenceOrdinal` | Dense zero-based stage order. |
| `RootObjectiveId` | Exactly one registered objective binding. |
| `ConditionIds` | Ordered required, failure, optional, and recovery conditions. |
| `ParticipantBindingIds` | Characters, vehicles, AI, payloads, and world actors. |
| `RouteAndWaypointIds` | Ordered route, checkpoint, destination, and recovery identities. |
| `ActivationPredicate` | Mission, chapter, participant, inventory, and feature requirements. |
| `LockRequirementIds` | Explicit vehicle, costume, reward, or progression requirements. |
| `TimePolicyId` | Countdown, count-up, inherited, added, paused, or untimed policy. |
| `LoadPlanId` | Stage-specific acquire and release plan. |
| `CheckpointPolicyId` | Checkpoint creation and restore behavior. |
| `SuccessTransitionId` | Declared successor or mission success. |
| `FailureTransitionId` | Stage retry, mission retry, rollback, abort, or terminal failure. |
| `PresentationProfileId` | HUD, camera, dialogue, countdown, music, and transition requests. |
| `WorldPolicyId` | Traffic, population, notoriety, safe-zone, and control policy. |
| `FinalPolicy` | Whether accepted success may terminate the mission. |
| `BonusObjectiveStartIds` | Optional objectives activated at this stage revision. |

<!-- markdownlint-enable MD013 -->

One stage owns one root objective. A compound gameplay sequence is
represented by ordered stages or by one explicitly registered composite
objective policy. It is never hidden in mutable script state.

## Stage lifecycle

A stage activation follows this sequence:

1. validate the stage identity and predecessor result;
1. evaluate lock and activation requirements;
1. execute the stage load plan;
1. resolve participant bindings and exact weak engine handles;
1. initialize objective and condition adapters;
1. capture the accepted recovery snapshot;
1. submit world and presentation requests;
1. commit the stage revision as active;
1. start objective and condition evaluation;
1. process fixed-step observations until one terminal
   transition is requested;
1. stop simulation-facing adapters;
1. finalize objective and conditions exactly once;
1. release or transfer stage-owned handles;
1. execute and verify the declared transition; and
1. activate the successor or terminal mission state.

Initialization, start, update, terminal result, finalization, and release
are separate phases. Repeating any phase for the same revision is idempotent
or rejected with a typed duplicate finding.

A stage cannot become active while its objective, required condition,
participant binding, or gameplay bundle is unresolved.

## Stage transition model

Initial transition kinds are:

- `next_stage`;
- `previous_declared_stage`;
- `rollback_by_policy`;
- `restart_stage`;
- `restart_checkpoint`;
- `restart_mission`;
- `return_to_free_roam`;
- `mission_success`;
- `mission_failure`;
- `chapter_transition`;
- `game_completion`; and
- `cancelled`.

Transitions reference identities, not arithmetic on an array index. A
rollback policy resolves one declared predecessor and validates that its
checkpoint is restorable. Negative indexes, wrapping, random mission
selection, and implicit fallback to stage zero are forbidden.

A stage marked final does not complete the mission merely because it is the
last loaded row. Final success requires the declared root objective, all
required conditions, the final policy, and progression
preconditions to agree.

## Timing and suspension

Mission and stage time use integer simulation ticks or another exact
monotonic representation. Policies declare:

- countdown, count-up, inherited, added, or untimed behavior;
- start boundary;
- pause and suspension reasons;
- warning thresholds;
- expiry result;
- checkpoint serialization;
- display rounding; and
- whether optional objectives observe the same clock.

Interior transitions, loading barriers, accepted cinematics, arrest
resolution, pause menus, and other controlled states suspend time only when
the policy says so. Frame count, rendering rate, wall-clock time, and
presentation completion are not mission time.

A warning observation is presentation only. Expiry becomes authoritative
only after the session validates the timer revision and active stage.

## Participant bindings

A participant binding contains:

- canonical role identity;
- expected character, vehicle, actor, payload, or AI definition;
- spawn, recovery, and optional alternate placement identities;
- required controller and behavior profile;
- driver, passenger, costume, and vehicle-role policy;
- ownership and persistence policy;
- damage and notoriety exemptions;
- stage acquire, transfer, hide, limbo, and release behavior; and
- world, mission, stage, and participant revisions.

Bindings distinguish the controlled player vehicle, default vehicle,
temporary forced vehicle, opponent, target, mission-only vehicle, traffic
vehicle, and presentation-only vehicle. A vehicle name or mutable slot
cannot satisfy several roles implicitly.

Destroyed presentation and replacement presentation preserve the canonical
vehicle identity according to the vehicle runtime. Stage reset never guesses
an original vehicle from a visual wreck actor.

## Artificial-intelligence bindings

Vehicle and character AI bindings reference native behavior profiles, route
and waypoint identities, traffic and population policy, catch-up policy,
activation, control handoff, and teardown.

AI activation occurs after the exact vehicle, route, and controller are
ready. Changing a catch-up profile or route creates a new binding revision.
A missing controller, duplicated vehicle binding, invalid route, or
incompatible behavior profile rejects stage readiness.

AI limbo, traffic suppression, chase suppression, and safe-zone behavior are
explicit world requests with handles. They are not global booleans restored
by assuming the previous state.

## Condition integration

Mission conditions follow the typed contract in the mission, interaction,
interior, and notoriety specification.

A stage declares the exact conditions it owns and the order-independent
terminal policy that combines them. Conditions can request:

- objective satisfaction;
- objective violation;
- stage failure;
- stage rollback;
- recovery;
- non-terminal telemetry; or
- a typed interior or world transition signal.

An objective cannot allocate or insert a hidden condition at runtime.
Conversion must emit the condition definition and stage binding explicitly.

## Objective adapter contract

Every objective kind is implemented by one registered C++ adapter with:

- canonical objective and policy identities;
- one active mission and stage revision;
- exact participant and subject bindings;
- accepted observation schemas;
- immutable configuration parameters;
- explicit initialization and finalization;
- `pending`, `satisfied`, `violated`, and `invalid` state;
- progress projection;
- cancellation and teardown behavior;
- checkpoint serialization when required; and
- typed terminal results.

An objective adapter consumes observations and emits intent. It cannot
advance the stage, change mission state, commit progression, invoke an
undeclared load, or infer completion from actor
destruction or disappearance.

Optional navigation, icons, damage meters, proximity meters, prompts,
dialogue, collection effects, and music are projections of the objective
snapshot. Their failure cannot change the authoritative result.

## Purchase objectives

`buy_vehicle` and `buy_costume` complete from an accepted permanent purchase
or ownership transaction for the exact catalog identity.

Merely entering a vehicle, changing a current vehicle pointer, changing a
visible model, equipping a costume through development tooling, or loading
an owned asset does not prove purchase.

The objective records the purchase transaction revision and succeeds once. A
refund, migration, or mod override follows the economy and progression
policies; it cannot replay the mission reward through a stale objective.

## Wager-entry fee objective

A wager-entry objective requests one atomic economy transaction containing:

- mission and wager identities;
- required fee;
- current balance revision;
- acceptance or rejection result;
- refund policy;
- attempt identity; and
- resulting balance revision.

Insufficient funds rejects mission entry without partially deducting
currency. Successful deduction does not complete the race; it only satisfies
the entry stage. Cancellation and load failure follow the
declared refund policy.

## Collectible objective base

The collectible objective family contains an ordered target set. Every
target binding declares:

- canonical collectible and placement identities;
- optional moving-parent or vehicle binding;
- collection eligibility and participant scope;
- activation, visibility, collision, and reservation policy;
- reset, respawn, and checkpoint behavior;
- optional dialogue and presentation identities; and
- terminal count policy.

Collection is an accepted transaction for one target revision. Duplicate
overlap, action, animation, or event delivery cannot collect
the same target twice.

A collectible that unloads, is destroyed, becomes hidden, changes parent, or
loses presentation remains pending or follows its explicit failure policy.
Those events never count as collection.

## Dumped-collectible objective

A dumped-collectible objective binds one exact source vehicle and a
deterministic set of payload identities. It declares:

- accepted collision and route observations that may release payloads;
- release force and distance policy;
- maximum active payload count;
- payload lifetime and timeout;
- player collection eligibility;
- source-vehicle damage and destruction policy;
- route and AI binding;
- progress and focus presentation; and
- cleanup behavior.

Payload release is a typed transaction. A payload cannot occupy two active
slots, be collected before release, or be recreated through callback order.

The objective does not create a hidden condition dynamically. Source-vehicle
failure is a declared stage condition or objective failure policy.

## Delivery objective

A delivery objective extends the collectible contract with destination and
submission policy. It supports carried world items and
vehicle-attached payloads.

Completion requires all declared payload transactions to be accepted at the
correct destination with the correct participant and stage revision. Contact
alone is insufficient when an explicit interaction or
submission action is required.

Delivered state, collision state, destination marker state, and
source-object state are restored or released through the checkpoint policy.
Delivery presentation never owns the submitted result.

## Destroy objectives

A `destroy` objective binds exact target identities and authoritative health
or destruction observations. It may project target markers, damage meters,
immunity exceptions, and encounter presentation.

Despawn, World Partition unload, actor replacement, garbage collection,
visual wreck creation, or missing presentation does not
count as destruction.

A boss-destroy objective consumes the registered boss encounter result.
Player vehicle destruction, boss damage presentation, and boss defeat are
different typed observations. Only the encounter's terminal defeat result
can satisfy the objective.

## Dialogue objective

A dialogue objective binds exact speaker, listener, conversation, placement,
camera, and presentation identities.

Starting a conversation, initializing dialogue presentation, and completing
the conversation are separate observations. The objective succeeds only from
the accepted terminal conversation result.

Temporary relocation, vehicle hiding, traffic clearing, camera cuts, and
participant control use the mission presentation transaction. Original state
is captured by revision and restoration is idempotent on success,
cancellation, timeout, mission replacement, or teardown.

A late conversation callback cannot complete a replacement stage or restore
over a newer character or vehicle state.

## Cinematic objective

A cinematic objective requests one validated media presentation and waits
for a terminal played, skipped, unavailable, cancelled, or failed result
according to policy.

Media playback is non-authoritative. The objective definition declares which
terminal presentation results satisfy the stage. Missing media cannot
silently become success unless an explicit accessibility or platform
fallback grants the same accepted result.

Durable media unlock and viewing state commit through progression, not
through player callbacks or a media component pointer.

## Follow and escape objectives

A `follow` objective binds one target and a maximum allowed distance policy.
It consumes bounded distance observations and may publish proximity
telemetry. Target unload, replacement, destruction, or route invalidation
follows the objective's explicit failure or recovery rule.

An `avoid` or `escape` objective succeeds only when every declared pursuer
meets the escape predicate for the required duration. A target being far
away in one frame, disappearing, or being destroyed is not sufficient unless
the policy explicitly accepts it.

Road distance, route progress, and Euclidean distance are separate schemas.
The policy states which measurement is authoritative.

## Enter-vehicle objective

An `enter_vehicle` objective binds the controlled participant and exact
required vehicle identity. It distinguishes:

- starting outside the vehicle;
- starting inside the correct vehicle;
- entering the correct vehicle;
- entering a wrong vehicle;
- exiting during a grace period;
- required strict behavior;
- forced-vehicle handoff; and
- cancellation or vehicle invalidation.

Success follows the accepted possession and seating transaction, not a
generic enter event. The objective cannot redirect the
current vehicle globally.

## Exterior and interior objectives

`exit_interior` and `enter_interior` objectives bind canonical interior,
portal, participant, and transition identities.

They consume the interior subsystem's accepted transition result. Starting
already in the requested composition may satisfy the objective only when the
policy explicitly permits it.

A portal overlap, filename, loading callback, or camera transition does not
prove entry or exit. Failure to activate the required composition enters
recovery and leaves the prior composition authoritative.

## Travel objective

A travel objective binds one destination placement, participant policy,
interaction requirement, route policy, and arrival predicate.

Arrival may require proximity, crossing direction, dwell, vehicle state, or
an explicit contextual interaction. The spatial subsystem reports evidence;
the objective validates it against the active mission and stage revisions.

Navigation arrows, lit routes, animated icons, collection effects, and
arrival dialogue are optional presentation. A missing route projection
cannot complete or fail the objective unless navigation is itself a
declared gameplay rule.

## Contextual talk objective

A `talk` objective binds one exact character placement, contextual interaction,
conversation offer, and mission-stage revision. The target may be promoted from
ambient representation before the interaction becomes eligible.

Availability requires:

- the exact character and placement revisions;
- an accepted interaction source and reservation policy;
- target readiness and compatible busy state;
- participant, distance, approach, and input eligibility;
- active mission and stage revisions; and
- required dialogue and presentation readiness.

A busy, unloaded, replaced, or reserved target makes the interaction unavailable
without failing the objective unless the policy says otherwise. Availability
changes update the candidate and prompt projection; they do not recreate the
objective or mutate input globally.

Success requires the contextual interaction subsystem's accepted conversation
handoff. A trigger overlap, button press, prompt display, marker removal, or
letterbox transition is not success.

The objective owns no character, trigger, prompt, input, traffic, or
presentation
state directly. Finalization releases the exact reservations and projections and
cannot restore over a newer interaction or input owner.

## Timer objective

A `timer` objective declares an exact duration, time source, start boundary,
suspension policy, and terminal result. It uses the mission timing contract and
records elapsed simulation ticks for the active objective revision.

The timer may represent a bounded pause, wait, survival interval, presentation
barrier, or another registered duration policy. It cannot use frame count,
wall-clock time, animation duration, or an uncorrelated callback.

Completion occurs once at the exact accepted boundary. Pause, interior
transition, cinematic, platform suspension, mission recovery, and frontend state
follow the declared suspension policy. Reset creates a new objective revision
and
restores the authored duration rather than retaining stale elapsed time.

A timer has no implicit reward, currency, mission-success, or presentation side
effect. The owning stage declares what follows the accepted result.

## Race objective binding

A `race` objective references one canonical race definition, route revision,
participant set, start grid, timer policy, and accepted result policy.
Checkpoint,
lap, position, finish, retry, wager, and race-set progression follow the
[race route and opponent runtime](race-route-and-opponent-runtime.md).

The mission adapter translates one accepted race result into objective intent.
It
cannot calculate position from actor pointers, inspect HUD state, advance laps
from trigger visibility, or complete when an opponent unloads.

Race presentation, including finish markers, lap counters, position, damage,
proximity, and route guidance, observes the race snapshot. It does not own route
progress or objective completion.

## Asynchronous vehicle-load objective

A vehicle-load objective requests a validated vehicle primary asset and one
participant-role replacement transaction. It declares:

- vehicle identity and configuration profile;
- destination role and ownership policy;
- spawn and recovery placement;
- old-role release policy;
- driver and control policy;
- timeout, cancellation, and rollback; and
- post-load verification.

The objective succeeds only after the new vehicle is ready, spawned, bound
to the correct role, positioned safely, and verified. A loading callback
from an older request cannot replace the current vehicle or
finish the objective.

Failure restores the previous accepted role binding and
releases partial assets.

## Item-pickup objective

An item-pickup objective binds one exact item identity and pickup
transaction. It distinguishes collection, destruction, explosion
presentation, removal, streaming removal, and actor replacement.

Success requires the accepted pickup result. If policy allows destruction to
expose or transform the item, the objective waits for the declared
transformation result rather than an arbitrary visual effect callback.

The item is released or persisted according to ownership and checkpoint
policy. Objective teardown cannot delete a persistent item
owned by another system.

## Optional bonus objectives

The reviewed optional objective families are:

- `no_damage`, evaluated against exact participant or
  vehicle damage channels;
- `no_chase_collisions`, evaluated from accepted
  pursuit-collision observations;
- `time_remaining`, evaluated from the declared mission clock; and
- `finish_position`, evaluated from the accepted ordered race result.

Each optional objective has its own identity, activation stage, observation
schemas, terminal state, checkpoint policy, reward policy, and telemetry.

Optional failure does not fail the mission unless the mission definition
promotes it to a required condition. Optional success and reward claims
commit once with the mission result transaction.

## Stage presentation requests

A stage presentation profile may submit:

- objective text and localized message identity;
- HUD visibility and objective icon policy;
- world, radar, map, damage, proximity, collectible, and
  directional markers;
- countdown sequence and temporary control restriction;
- conversation and completion dialogue;
- camera rig, preset, cut, and transition requests;
- fade, iris, loading-screen, and special-screen requests;
- music event, state, tension, and persistence requests;
- traffic, population, and chase presentation policy; and
- accessibility alternatives.

Every request has an owner handle and stage revision. Finalization releases
or transfers it explicitly. Presentation callbacks cannot choose
mission or stage state.

## Checkpoint and restart

A checkpoint contains:

- mission, stage, objective, and condition revisions;
- mission and stage clock state;
- controlled participant and vehicle-role bindings;
- mission-owned actor, payload, collectible, and AI state;
- accepted world-composition and Data Layer state;
- progression transactions already committed;
- optional-objective state;
- deterministic selection revisions; and
- release and reconstruction plans.

Restart follows one declared checkpoint policy. It releases the current
stage, rolls back uncommitted transactions, restores world composition,
reconstructs participants, verifies the snapshot, and creates
a new stage revision.

Restart cannot dump every dynamic object globally, infer an earlier stage
from an index, repair state through a visual fade, or overwrite
persistent world state.

## Mission abort and cancellation

Abort eligibility is explicit per mission and stage. An accepted abort:

1. records the cancellation cause;
1. stops objective and condition evaluation;
1. cancels correlated loads and presentation;
1. rolls back uncommitted economy and mission transactions;
1. restores the accepted free-roam participant and vehicle state;
1. releases mission-only actors and layers;
1. releases the gameplay-state lease; and
1. publishes one terminal cancellation result.

Forced-vehicle, wager, interior, dialogue, and loading stages declare
additional rollback policy. Returning to free roam is not represented by
decrementing the mission index.

## Completion and progression

Mission success commits one atomic progression transaction containing:

- mission and attempt identities;
- completed mission identity;
- optional-objective results;
- best time, position, damage, score, or other registered evidence;
- reward and unlock identities;
- chapter or world-expansion result;
- replay and skip state changes;
- achievement and statistic effects; and
- resulting save revision.

The session enters `completed` only after the transaction commits and
verifies. A save failure leaves the session in typed recovery and cannot
grant presentation without durable state.

Repeated success callbacks, stage replay, loading completion, presentation
replay, or mod deactivation cannot duplicate the transaction.

## World streaming and teardown

Mission and stage load plans use the native asset-loading and
world-composition contracts. Each acquired asset, Data Layer, actor
projection, placement, participant, objective, condition, and presentation
request has one owner handle.

Stage transition transfers only explicitly declared handles. Mission
completion, failure, abort, world teardown, feature removal, and mod
deactivation release all remaining handles in dependency order.

Unloading an actor or cell is not an objective result. Late callbacks are
rejected by mission, stage, world, and request revisions.

## Mods and game features

A validated package may add or override namespaced mission
definitions when it declares:

- mission, stage, objective, condition, participant, and
  transition identities;
- required registered schemas and capabilities;
- chapter, world, feature, and dependency predicates;
- loading, checkpoint, save, reward, and teardown policies;
- conflicts and override priority;
- multiplayer compatibility where applicable; and
- deterministic tests.

An overlay cannot introduce runtime script execution, arbitrary Blueprint
completion logic, incompatible objective payloads, mutable global context,
or unbounded references.

Removing a feature cancels or migrates its active session according to the
declared policy and releases all owned handles.

## Diagnostics

Development diagnostics expose immutable snapshots of:

- mission, session, stage, objective, condition, and checkpoint identities;
- catalog, world, stage, and request revisions;
- current state and last accepted transition;
- readiness barriers and outstanding load requests;
- participant and vehicle-role bindings;
- objective progress and accepted observations;
- condition states and telemetry;
- uncommitted and committed progression transactions;
- presentation request ownership; and
- rejection, rollback, and teardown findings.

Diagnostics can request a validated restart, abort, or observation injection
in a dedicated test world. They cannot mutate production state or
execute source commands.

## Failure behavior

Compilation or runtime fails closed on:

- unknown, duplicate, or ambiguous identity;
- invalid command scope, arguments, units, or ordering;
- unclosed mission, stage, objective, or condition context;
- missing objective, condition, participant, route, bundle, or transition;
- stage gaps, duplicate ordinals, cycles without a registered loop policy,
  or unreachable terminal states;
- incompatible participant or vehicle-role binding;
- stale mission, stage, world, load, objective, or presentation revision;
- duplicate initialization, terminal result, finalization,
  or progression commit;
- actor destruction or unload presented as success without policy;
- partial load, checkpoint, restore, abort, or teardown;
- presentation attempting an authoritative transition;
- runtime source-script execution; or
- save and catalog revisions that cannot be reconciled.

Failure returns typed evidence and preserves the last accepted mission,
world, and progression state. It never guesses a successor, clamps an
ordinal, drops an owned handle silently, or converts a missing
dependency into success.

## Validation

Definition validation proves:

- every source command maps uniquely or is explicitly unavailable;
- every external proposal fact has one terminal accepted, adapted, superseded,
  rejected, or unresolved result;
- unresolved proposal facts generate no mission, stage, objective, participant,
  placement, or progression definition;
- every historical mission bundle has one reconciled fact graph or an explicit
  unresolved result;
- exact duplicate sources collapse and changed revisions cannot partially
  overwrite an accepted mission;
- title, hint, and icon references resolve through stable presentation
  identities;
- source editing paths, DCC scenes, checkout instructions, and script ordering
  create no runtime or target-authoring authority;
- every mission and stage identity is unique;
- stage ordinals are dense and transitions resolve;
- every stage owns exactly one root objective;
- all objective, condition, participant, route, placement, load, and
  presentation schemas resolve;
- all terminal and recovery paths are reachable and deterministic;
- required assets and Data Layers belong to a load plan;
- purchase, wager, reward, and completion operations are transactional;
- no pointer, filename, source ordinal, array slot, or callback order
  is runtime authority;
- every handle has a release or transfer path; and
- overlays cannot weaken the base schema.

## Tests

Required automated tests include:

- deterministic conversion from equivalent mission evidence;
- external proposal-set merge, conflict, accepted adaptation, supersession,
  rejection, unresolved-state isolation, and zero partial publication;
- mission-bundle grouping across breakdowns, level summaries, outlines,
  functionality, presentation, race, and script evidence;
- exact duplicate collapse, changed-revision comparison, conflict findings, and
  zero partial overwrite;
- source DCC, locator, checkout, path, and editing-process rejection;
- stable presentation references for mission titles, hints, and icons;
- unknown, duplicate, malformed, out-of-order, and
  unclosed command rejection;
- invalid argument count, units, enums, and references;
- stage topology, cycle, terminal, and rollback validation;
- load success, optional degradation, required failure, cancellation, and
  late callback rejection;
- objective and condition initialization and finalization exactly once;
- duplicate, stale, wrong-stage, wrong-participant, and wrong-target
  observation rejection;
- purchase and wager transaction atomicity;
- collectible duplication, unload, destruction, moving-parent, and
  checkpoint recovery;
- dumped-payload slot, lifetime, collision, and collection determinism;
- delivery destination and explicit-interaction policy;
- destroy versus unload and replacement behavior;
- dialogue relocation and restoration after success,
  cancellation, and timeout;
- cinematic played, skipped, unavailable, cancelled, and failed policies;
- follow and escape distance-schema behavior;
- correct and incorrect vehicle entry;
- interior and exterior transition rollback;
- travel arrival, dwell, direction, and interaction requirements;
- asynchronous vehicle load replacement and rollback;
- item pickup versus destruction and streaming removal;
- optional-objective independence and exactly-once reward claims;
- checkpoint reconstruction equivalence;
- abort from every stage phase;
- mission completion transaction deduplication;
- world, feature, and mod teardown with zero owned handles; and
- identical terminal snapshots across supported frame rates
  and graphics presets.

## Invariants

- One world has at most one accepted base-game mission session.
- One active session has exactly one active stage revision.
- One active stage has exactly one root objective revision.
- Every objective and condition belongs to one mission and stage revision.
- Every stage transition is declared and revision-correlated.
- Every required load dependency is ready before stage activation.
- Every participant role resolves to at most one authoritative binding.
- Presentation never owns mission, objective, condition,
  reward, or save state.
- Actor unload, destruction, and presentation completion
  are distinct results.
- Restart restores one accepted checkpoint without rewriting
  durable progression.
- Abort releases mission state without decrementing a mission ordinal.
- Completion commits once before the session becomes completed.
- Runtime packages never execute source mission commands.
- External proposal documents never become runtime mission graphs directly.
- Unresolved proposal facts publish no partial mission state.

# Mission, interaction, interior, and notoriety runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)

## Purpose

This specification defines the C++ runtime contracts for mission execution,
objective evaluation, world interactions, indoor transitions, notoriety,
recovery, and their native Unreal projections.

The runtime consumes validated catalog identities and rows. It does not infer
mission meaning from actors, map names, content paths, Blueprint graphs, or
loaded packages.

## Ownership boundary

Four services own the runtime state described here:

| Service | Lifetime | Authority |
| :--- | :--- | :--- |
| `USharMissionSessionSubsystem` | World | Active mission, objective state, ordered transitions, failure, recovery, and completion. |
| `USharInteractionSubsystem` | World | Interaction eligibility, reservation, execution, cancellation, and exactly-once results. |
| `USharInteriorSubsystem` | World | Interior transition transactions, world composition, movement restrictions, and exterior restoration. |
| `USharNotorietySubsystem` | World | Notoriety value, warning, pursuit, arrest, decay, and resolution. |

The progression service owns durable mission, gag, reward, and level completion.
The save service persists only accepted checkpoints and completed transactions.
World actors publish typed observations and execute presentation. They never own
a progression key or authoritative state transition.

## Mission session state

A mission session has one canonical mission identity and one state:

- `idle`;
- `preparing`;
- `active`;
- `succeeded`;
- `failed`;
- `recovering`; or
- `completed`.

`preparing` resolves the mission definition, step table, objective policies,
required bundles, world layers, actors, and restart checkpoint before gameplay
begins. A missing dependency rejects the start without changing progression.

`active` owns exactly one current step. Compound objectives are represented by
ordered child steps rather than hidden script state. A step transition is
accepted only after the current objective reports a terminal result and the
session validates the declared successor.

`succeeded` means the final objective completed but its progression transaction
has not yet been accepted. `completed` is entered only after rewards, unlocks,
and the mission completion key are committed exactly once.

`failed` records a typed cause. `recovering` applies the declared restart policy,
restores the accepted checkpoint, and verifies the restored world before the
session can return to `active`.

## StateTree projection

Mission execution uses one repository-owned StateTree schema and a bounded C++
library of tasks, evaluators, conditions, and transition handlers.

The StateTree receives an immutable mission-session view and emits typed intent:
start an objective adapter, observe progress, request success, request failure,
or request recovery. It cannot grant rewards, mutate save state, change the
canonical mission identity, or choose an undeclared successor.

Mission rows parameterize the shared StateTree behavior. A mission-specific
StateTree asset may contain presentation-only composition when a native editor
workflow requires it, but it cannot contain unique completion or progression
logic. Validation rejects mission definitions that require an unregistered task,
condition, evaluator, transition, or external data binding.

A task failure propagates through the hierarchy to the nearest declared recovery
or failure transition. Unhandled failure terminates the step with a typed error;
it never becomes success through actor destruction, unloading, or missing data.

## Objective policy row

`FSharObjectivePolicyRow` defines objective-specific behavior:

| Field | Contract |
| :--- | :--- |
| `PolicyId` | Stable identity referenced by one or more mission steps. |
| `ObjectiveKind` | Exact controlled objective kind. |
| `RouteId` | Optional ordered route or destination identity. |
| `TargetIds` | Ordered canonical target identities. |
| `StartTrigger` | Immediate, proximity, interaction, dialogue completion, or explicit signal. |
| `CompletionRule` | Typed predicate evaluated from accepted observations. |
| `FailureRule` | Typed predicate with a declared grace period where applicable. |
| `RecoveryRule` | Restart step, restart mission, restore checkpoint, or return to free roam. |
| `NotorietyPolicyId` | Required policy for target contact and objective exemptions. |
| `CatchUpProfileId` | Optional artificial-intelligence catch-up profile. |
| `DropSequenceId` | Optional ordered dropped-item sequence. |
| `PresentationProfileId` | Objective marker, radar, gauge, prompt, and briefing presentation. |

Thresholds, counts, durations, and distances use explicit units and validated
ranges. A policy cannot rely on a frame count, graphics preset, current refresh
rate, or an implementation-defined floating-point comparison.

## Travel objective

A `travel` objective completes when the controlled pawn or required vehicle
enters the declared destination volume through an allowed approach and satisfies
any dwell or vehicle condition.

The source concept commonly described as go-to maps to `travel`; it does not
create a second objective kind. Reaching an actor near the destination does not
complete the objective unless that actor or its interaction is the declared
completion target.

World streaming cannot complete a travel objective. If its destination becomes
unavailable, the objective pauses within its declared recovery window or fails
with `destination_unavailable`.

## Follow objective

A `follow` objective declares:

- one target vehicle or convoy leader;
- a start trigger;
- minimum and maximum valid separation;
- a failure threshold and grace duration;
- an optional destination or terminal signal;
- a deterministic catch-up profile; and
- a target-contact notoriety policy.

The objective begins only from its declared trigger. A target that moves on
proximity during the first attempt and immediately during a retry expresses
those behaviors through separate start profiles rather than hidden mission
state.

Progress derives from route and target state, not from rendered gauge pixels.
The user interface receives normalized separation and warning state from the
objective. Falling outside the failure boundary for the full grace duration
fails the step. Overtaking the target does not complete the step and can fail
when the policy defines a maximum lead.

Catch-up adjusts the target's bounded speed or route behavior from the declared
profile. It cannot teleport the target, change mission time, depend on frame
rate, or silently vary by graphics preset.

## Follow-and-collect objective

A `follow_and_collect` objective composes the follow contract with one ordered
drop sequence. Each drop has a stable item identity, sequence ordinal, spawn
anchor, acceptance volume, and miss policy.

The target must remain valid until the objective reaches its terminal route
state. Collecting the required count does not bypass a declared destination, and
reaching the destination does not excuse missing required drops.

Each dropped item is accepted once. Streaming, respawn, target replacement, or
mission restart cannot duplicate an accepted item. A retry restores only the
items and target state owned by the accepted checkpoint.

Target collisions use the policy's normal notoriety behavior unless an explicit
exemption is declared. Follow-and-collect does not inherit the collision
exemption used by hit-and-collect.

## Hit-and-collect objective

A `hit_and_collect` objective declares one target, an impact acceptance policy,
a drop sequence, a required count, and whether destruction emits a terminal
item.

Only validated impacts from an allowed instigator can advance target damage or
emit an objective drop. Each threshold crossing and destruction event emits at
most one item identity. Repeated overlap, physics substeps, network-style event
replay, or actor replacement cannot duplicate a drop.

Contact with the declared objective target is notoriety-exempt only when the
objective policy says so. Damage to pedestrians, world props, traffic, or any
other vehicle remains governed by the normal notoriety policy.

Destroying the target before required intermediate drops are emitted is a typed
failure unless the policy explicitly defines a destruction-only terminal item.
A gameplay modifier that forces one-hit destruction cannot convert an invalid
sequence into completion.

## Collect and deliver objectives

A `collect` objective accepts only declared item identities from their allowed
source or placement. A `deliver` objective requires the accepted item set and the
declared destination or interaction.

Collecting an item and delivering it are separate transaction boundaries when a
mission can fail between them. A forced vehicle may constrain collection without
changing the item identity. Pedestrian, prop, and traffic contacts remain normal
notoriety events while collecting or delivering.

## Vehicle-attached payload objective

A `vehicle_payload` objective transports one declared mission item on a separate
canonical vehicle. The payload definition records:

- stable payload and attachment identities;
- eligible carrier vehicles and attachment sockets;
- attach, detach, and reattach policies;
- collision impulse and stability thresholds;
- intact, unstable, detached, destroyed, and delivered states;
- damage, effect, and audio presentation;
- accepted delivery volume and orientation policy;
- retry and checkpoint restoration; and
- whether incidental disturbance emits a separately declared economy event.

Attachment does not merge payload and vehicle identity. Vehicle replacement,
retrieval, destruction, World Partition streaming, or actor reconstruction cannot
silently deliver, duplicate, or delete the payload.

Delivery is accepted only when the declared payload identity remains in an
eligible state and enters the accepted delivery volume through the objective
transaction. Carrier entry without the payload, a detached payload outside the
volume, destruction effects, despawn, and presentation completion are not
success.

A fragile payload uses deterministic collision observations and validated
thresholds rather than frame-rate-dependent contact callbacks. Retry restores the
exact declared carrier, payload state, attachment, route context, and accepted
prior steps. It never preserves a partially destroyed candidate as success.

## Destroy and avoid objectives

A `destroy` objective completes only from validated destruction of every required
target. Despawn, garbage collection, World Partition deactivation, target
replacement, or load failure never counts as destruction.

An `avoid` objective follows the catalog contract: pursuers must remain outside
the declared pursuit boundary for the declared duration or satisfy an explicit
escape condition. A destroyed pursuer follows the objective's declared recovery
or success policy; destruction alone cannot silently complete the objective.

A compound destroy-then-avoid mission records destruction and escape as separate
steps so the level transition occurs only after both have been accepted.

## Race objective

A `race` objective consumes the canonical race definition, route rows, lap or
checkpoint policy, ordered opponents, time limit, finish transition, and catch-up
profile. Route topology, checkpoint crossing, position, opponent state, timer,
reset, finish, and street-race-set behavior follow
[Race route and opponent runtime](race-route-and-opponent-runtime.md).

A mission race may use a forced vehicle without granting ownership. Opponent
identity, route order, shortcuts, reset transforms, and finish direction remain
stable across retries, platforms, frame rates, and graphics presets. A vehicle
following a route for a follow, dump, avoid, or lose-tail objective does not
silently become a race.

## Objective observations

World adapters publish immutable observations with:

| Field | Contract |
| :--- | :--- |
| `ObservationId` | Unique session-scoped identity for deduplication. |
| `MissionId` | Active mission identity. |
| `StepOrdinal` | Active step ordinal. |
| `SourceId` | Canonical actor, item, zone, or interaction identity. |
| `Kind` | Typed collision, overlap, destruction, collection, route, timer, or interaction event. |
| `SimulationTime` | Monotonic fixed-step simulation timestamp. |
| `Payload` | Schema-defined values for the observation kind. |

The mission session rejects stale, duplicate, future-step, wrong-mission, or
unrecognized observations. Presentation events cannot be replayed as gameplay
observations.

## Interaction definition and reservation

`USharInteractionDefinition` contains the canonical interaction identity,
eligibility tags, action identity, duration policy, cancellation policy,
presentation profile, and typed result policy.

`FSharInteractionPlacementRow` binds that definition to a level, location,
Runtime Data Layer, Smart Object definition, slot, transform identity, and
availability predicate.

The interaction sequence is:

1. query eligible Smart Object slots;
1. select deterministically from the filtered result set;
1. reserve one slot for the requesting interactor;
1. revalidate level, progression, mission, and input eligibility;
1. begin the C++ interaction action;
1. commit or cancel the typed result;
1. release the reservation; and
1. publish presentation after the authoritative result is known.

Reservation prevents concurrent activation but does not grant progression. An
unloaded, invalidated, or lost slot cancels safely and cannot leave a partial
reward or consumed save key.

## Gag definition and execution

`USharGagDefinition` extends the interaction definition with:

| Field | Contract |
| :--- | :--- |
| `GagId` | Canonical gag concept identity. |
| `ActivationProfileId` | Input, proximity, cue, and animation policy. |
| `RewardPolicyId` | Optional currency or non-currency reward transaction. |
| `ReplayPolicy` | Presentation replay allowed or denied after completion. |
| `LocalePresentationId` | Optional localized media or presentation selection. |

A placement owns a level-scoped completion key. Reused world geometry may place
the same gag concept in several level variants, but each declared level placement
has one distinct progression identity.

Activation requires the interaction action, not contact alone. The visible cue,
prompt, animation, sound, and reward are presentation of one accepted result.
Reloading, changing missions, crossing a streaming boundary, or reactivating a
replayable animation cannot grant the reward or completion key twice.

A source page, quote stub, unused prototype, or unreachable actor does not create
a runtime gag or dialogue asset without validated placement evidence.

## Interior definition

`USharInteriorDefinition` contains:

| Field | Contract |
| :--- | :--- |
| `InteriorId` | Canonical location identity. |
| `LevelIds` | Level variants in which the interior is available. |
| `ExteriorPortalId` | Required entry interaction placement. |
| `InteriorPortalId` | Required exit interaction placement. |
| `InteriorDataLayers` | Runtime layers activated for the interior. |
| `ExteriorDataLayers` | Runtime layers whose state is restored on exit. |
| `SpawnTransformId` | Interior player arrival transform. |
| `ReturnTransformId` | Exterior player return transform. |
| `RestrictionProfileId` | Movement, combat, and action restrictions. |
| `NotorietyTransitionId` | Interior-specific pursuit and decay policy. |
| `InteractionPlacementIds` | Gags, characters, mission anchors, and costume stations. |

The canonical interior set is:

- Simpson House;
- Kwik-E-Mart;
- Springfield Elementary;
- Moe's Tavern;
- Springfield DMV;
- Android's Dungeon;
- Frink Observatory; and
- Bart's Bedroom.

Each interior supports at least one level-scoped gag and at least one declared
mission use. Costume and character interactions are optional and explicit.

## Interior transition transaction

Entry is an atomic world-composition transaction:

1. validate the portal, mission state, and interior definition;
1. capture the player, selected vehicle, vehicle transform, and vehicle damage;
1. stop unsafe transient player actions;
1. activate the interior Runtime Data Layers;
1. verify required interior actors and interactions;
1. move the player to the interior spawn transform;
1. apply the restriction and notoriety policies; and
1. hide or deactivate exterior presentation only after the interior is ready.

Exit performs the inverse transaction and restores the captured vehicle state.
The selected vehicle remains in the exterior world at its accepted transform and
damage state unless a mission transition explicitly owns a different outcome.

If any activation or verification step fails, the transaction restores the
previous composition and player state. It never leaves both compositions active,
loses the vehicle snapshot, or commits a partial transition.

Indoors, the standard restriction profile disables sprint, jump, aerial attack,
and successful attacks against non-player characters. The interaction subsystem
remains active. A mission may narrow actions further but cannot weaken the
standard restrictions without a separate accepted policy.

## Notoriety scale and event model

Notoriety uses a fixed integer scale from `0` through `10,000`. It never stores
an authoritative floating-point percentage.

`FSharNotorietyEvent` contains a unique event identity, source identity,
instigator identity, event kind, signed integer delta, objective policy identity,
and simulation timestamp.

The verified base policy defines:

| Event or threshold | Value |
| :--- | ---: |
| Minor accepted offense | `500` |
| Coin-bearing vehicle destruction | `3,300` |
| Warning threshold | `8,000` |
| Pursuit threshold | `10,000` |
| Arrest proximity hold | `1.0` second |
| Arrest fine | `50` coins, clamped to the current balance |
| Pursuers in Levels 1 through 3 | `1` active vehicle per wave |
| Pursuers in Levels 4 through 7 | `2` active vehicles per wave |

Pedestrian impact or attack, destructible-world damage, non-exempt vehicle
contact, and qualifying vehicle destruction publish typed events. A source that
cannot prove its event kind or identity is rejected rather than assigned a
default delta.

## Notoriety state machine

The subsystem has five states:

- `dormant`;
- `cooling`;
- `warning`;
- `pursuit`; and
- `resolving`.

`dormant` has no value and no pursuers. An accepted offense enters `cooling` or
`warning` according to the resulting value and resets the policy-defined decay
delay. `warning` begins at the warning threshold and publishes the warning
presentation once per entry.

Reaching the pursuit threshold enters `pursuit`, selects the level policy, and
requests the configured wave. Replacement waves may spawn while pursuit remains
active, but spawn limits and placement validation prevent unbounded simultaneous
pursuers.

During pursuit, decay follows the pursuit policy even when accepted offenses
occur. Contact with active police vehicles is exempt from further notoriety.
Other offenses remain governed by their normal policy.

A pursuit enters `resolving` when the value reaches zero. No replacement wave may
spawn after that transition. Resolution completes only after all remaining
pursuers are destroyed, evaded, or explicitly withdrawn. New ordinary offenses
are ignored during the bounded resolution window defined by the policy; the
subsystem then returns to `dormant`.

All decay delays and rates are policy data expressed in simulation-time units.
They do not depend on frame rate, animation duration, audio duration, or graphics
preset.

## Arrest

An arrest candidate begins when the player remains within an eligible pursuer's
capture volume while in a capturable state. Vehicle neutral state and on-foot
capture use separate eligibility predicates but share the configured hold time.

Leaving the volume, moving into an ineligible state, destroying the pursuer, or
invalidating the candidate resets the hold. One accepted candidate emits one
arrest transaction.

The transaction:

1. suspends player control and pursuit mutations;
1. applies the clamped 50-coin fine through the currency ledger;
1. records presentation and reset intent;
1. restores the player at the declared safe checkpoint;
1. removes pursuit actors and transient pursuit state;
1. preserves unrelated world damage and mission state; and
1. returns notoriety to `dormant`.

Arrest does not fail an active mission unless that mission step explicitly
states an arrest failure condition.

## Interior notoriety behavior

A successful interior entry withdraws active pursuit actors, prevents arrest in
the interior, and applies the interior decay profile. The value decays more
rapidly than the exterior base profile.

Interior entry does not erase the notoriety value as an unrecorded side effect.
The transition and subsequent decay are explicit events. Leaving before decay
completes restores the exterior policy from the remaining value without
recreating stale pursuers.

## World safety and recovery

Every level defines safe player and vehicle recovery transforms for each streamed
region, mission checkpoint, race reset, and interior portal.

Out-of-bounds, invalid floor, unrecoverable penetration, missing collision, and
non-finite transform detection request typed recovery. Recovery chooses the most
recent valid transform compatible with the active mission and vehicle state. It
never continues an infinite fall, exposes unloaded world, or crashes the native
package.

Duplicate actor identities, duplicate interaction placements, impossible route
links, unreachable required targets, invalid Data Layer sets, and unsupported
objective-policy combinations fail validation before cooking.

Historical defects, unused objects, malformed actors, accidental collision
holes, and unreachable prototype content remain excluded unless an accepted
record separately declares an intentional behavior.

## Determinism and save boundaries

The mission session serializes only accepted checkpoint state. It does not save
an in-flight physics contact, partially held arrest candidate, Smart Object
reservation, uncommitted drop, or incomplete interior transition.

On load, the session resolves catalog identities, restores the checkpoint,
reconstructs world composition, verifies actors and interactions, and only then
resumes simulation. Missing required content rejects the resume and offers the
declared recovery path.

Event ordering uses simulation timestamps followed by event identity. Repeated
input with the same catalog revision and accepted checkpoint produces the same
objective, notoriety, reward, and recovery transitions.

## Invariants

- One world has at most one authoritative mission session.
- One mission session has exactly one active step.
- A terminal step result is accepted once.
- A mission completion and each reward are committed once.
- Every objective uses a registered policy compatible with its kind.
- Every observation belongs to the current mission and step.
- A target unload never substitutes for completion or destruction.
- A drop, interaction result, gag key, and notoriety event are deduplicated by
  stable identity.
- A Smart Object reservation cannot outlive its interaction action.
- An interior transition is fully committed or fully restored.
- Notoriety remains within the fixed scale.
- Arrest cannot produce a negative currency balance.
- Platform, architecture, input adapter, and graphics preset cannot change any
  rule in this specification.

## Failure behavior

Configuration errors fail validation and block cooking. Runtime absence caused by
unexpected streaming or corruption produces a typed mission or transition error,
restores the last accepted state when possible, and records bounded diagnostics.

The runtime never guesses a replacement target, route, destination, vehicle,
interaction, reward, or Data Layer. It never marks an objective complete to
escape a blocked state.

A failed reward or save transaction leaves the mission in `succeeded` and retries
the transaction safely; it does not replay the final objective. A failed
interior transaction restores the exterior. A failed arrest transaction restores
control without applying an uncommitted fine.

## Verification

Repository logic tests prove:

- every mission step resolves one compatible objective policy;
- travel completes only from the declared destination;
- follow distance, lead, grace, retry start, and catch-up are deterministic;
- follow-and-collect preserves ordered drops and normal target notoriety;
- hit-and-collect deduplicates impact drops and honors only declared exemptions;
- destroy and avoid reject unload or despawn as success;
- forced race vehicles do not grant ownership;
- stale and duplicate observations are rejected;
- gag completion and rewards commit once per level placement;
- interior entry and exit preserve vehicle transform and damage;
- failed Data Layer activation restores the prior composition;
- notoriety thresholds, warning, waves, decay, resolution, arrest, and fines use
  the fixed policy;
- police contact during pursuit is exempt while unrelated offenses are not;
- arrest preserves the active mission unless its step declares failure;
- out-of-bounds recovery selects the declared valid transform; and
- accidental defect fixtures never become parity requirements.

Native automation verifies StateTree task registration, Smart Object reservation,
Runtime Data Layer activation, actor bindings, user-interface projections,
artificial-intelligence profiles, collision event classification, and cook-time
validation.

Full runtime parity evidence executes representative compound missions, gag and
interior interactions, warning and pursuit transitions, arrest, mission retry,
level transition, save and reload, and deterministic replay on every supported
platform family.

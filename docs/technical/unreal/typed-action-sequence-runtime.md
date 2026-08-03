# Typed action-sequence runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Typed StateTree action
  sequences](../../adr/unreal/runtime/typed-state-tree-action-sequences.md)
- [Presentation playback runtime](presentation-playback-runtime.md)
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Contextual interaction query and transaction boundary](../../adr/unreal/runtime/contextual-interaction-query-and-transaction.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Transactional phone-booth vehicle retrieval](../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md) <!-- markdownlint-disable-line MD013 -->

## Purpose

This specification defines the reusable native action vocabulary for characters,
vehicles, props, interactions, and mission presentation. It replaces positional
string registries, fixed handler arrays, custom task scheduling, and raw event
payloads with validated definitions and typed StateTree tasks.

## Ownership

The action runtime coordinates presentation and movement. It does not own
mission,
progression, reward, economy, save, vehicle-ownership, or collectible state.
Those effects remain behind application ports.

<!-- markdownlint-disable MD013 -->

- **Authority:** Generated action catalog
  - **Responsibility:** Stable identities, execution kinds, parameters,
    resources, and verification policy.
- **Authority:** StateTree
  - **Responsibility:** Ordered states, transitions, task lifetime, failure, and
    cancellation.
- **Authority:** Resource arbiter
  - **Responsibility:** Exclusive and shared runtime leases.
- **Authority:** Native task library
  - **Responsibility:** Typed movement, animation, vehicle, delay, event, and
    state operations.
- **Authority:** Domain services
  - **Responsibility:** Authoritative gameplay effects and persistent results.

<!-- markdownlint-enable MD013 -->

## Runtime topology

The runtime module owns these C++ types:

<!-- markdownlint-disable MD013 -->

- **Type:** `USharActionDefinition`
  - **Responsibility:** Primary data asset for one reusable action contract.
- **Type:** `USharActionSequenceDefinition`
  - **Responsibility:** Immutable ordered plan using registered action
    identities.
- **Type:** `USharActionCatalogSubsystem`
  - **Responsibility:** Definition lookup, revision validation, and executor
    registration.
- **Type:** `USharActionResourceArbiter`
  - **Responsibility:** Grants and releases typed resource leases.
- **Type:** `FSharActionContext`
  - **Responsibility:** Immutable actor, interaction, mission, vehicle, and
    world context.
- **Type:** `FSharActionRequest`
  - **Responsibility:** Definition identity, typed parameters, expected
    revision, and action ordinal.
- **Type:** `FSharActionResult`
  - **Responsibility:** Closed result with reason, observations, and
    verification evidence.
- **Type:** `FSharActionLease`
  - **Responsibility:** Move-only lease for one declared resource.
- **Type:** `FSharActionSequenceHandle`
  - **Responsibility:** Cancellation-safe handle for one active sequence.

<!-- markdownlint-enable MD013 -->

The StateTree schema exposes these types as external data. It never receives raw
pointers whose lifetime is not represented by a weak object handle or stable
identity.

## Definition contract

Every `USharActionDefinition` contains:

<!-- markdownlint-disable MD013 -->

- **Field:** `ActionId`
  - **Contract:** Globally unique canonical identity.
- **Field:** `ExecutionKind`
  - **Contract:** Closed enum selecting one native task implementation.
- **Field:** `GameplayTags`
  - **Contract:** Family, capability, blocking, mission, and presentation tags.
- **Field:** `ParameterSchema`
  - **Contract:** Closed typed payload for the selected execution kind.
- **Field:** `RequiredResources`
  - **Contract:** Ordered resource claims with exclusive or shared access.
- **Field:** `Preconditions`
  - **Contract:** Required actor, world, mission, vehicle, and interaction
    state.
- **Field:** `TimeoutPolicy`
  - **Contract:** Positive timeout or explicit no-timeout permission.
- **Field:** `CancellationPolicy`
  - **Contract:** Allowed cancellation points and required compensation.
- **Field:** `VerificationPolicy`
  - **Contract:** Observable postcondition required for success.
- **Field:** `PresentationPolicy`
  - **Contract:** Optional animation, sound, effects, prompt, and camera data.
- **Field:** `DefinitionRevision`
  - **Contract:** Immutable revision used to reject stale requests.

<!-- markdownlint-enable MD013 -->

Every `USharActionSequenceDefinition` contains:

<!-- markdownlint-disable MD013 -->

- **Field:** `SequenceId`
  - **Contract:** Globally unique canonical identity.
- **Field:** `StateTreeTemplate`
  - **Contract:** Canonical template compatible with the action schema.
- **Field:** `Steps`
  - **Contract:** Ordered action identities and typed parameter bindings.
- **Field:** `FailurePolicy`
  - **Contract:** Abort, compensate, fallback, retry, or continue for each
    declared failure class.
- **Field:** `SequenceTimeout`
  - **Contract:** Optional total bound not shorter than required step bounds.
- **Field:** `RequiredContext`
  - **Contract:** Actor, vehicle, interaction, mission, or world context
    requirements.
- **Field:** `VerificationPolicy`
  - **Contract:** Final observable sequence postcondition.
- **Field:** `DefinitionRevision`
  - **Contract:** Immutable revision used to reject stale execution.

<!-- markdownlint-enable MD013 -->

Definitions cannot contain source-language callbacks, free-form script
fragments,
unregistered event strings, or machine-specific object paths.

## Registry

The catalog subsystem builds a map from `ActionId` to validated definition and
registered executor. Registration order is irrelevant. Catalog activation fails
when:

- an identity is empty or duplicated;
- an execution kind has no native task implementation;
- the parameter schema does not match the execution kind;
- a sequence references a missing or incompatible action;
- required resources are undeclared or conflict within a parallel state;
- an asset, montage, socket, vehicle door, state, or event identity is
  unresolved;
- a timeout or retry policy is invalid; or
- a definition revision is inconsistent with generated data.

The runtime never aligns two arrays by index and never hashes display text to
recover gameplay identity.

## StateTree sequence model

Each ordered step is represented by one StateTree state using the `Next`
transition on success. A state may contain:

- one resource-owning action task;
- read-only evaluators;
- non-conflicting presentation tasks; and
- explicit success, failure, timeout, and cancellation transitions.

Tasks in one state execute concurrently, so validation rejects two tasks that
claim conflicting resources. Sequence templates provide common parent states for
resource acquisition, cancellation, compensation, and cleanup.

A sequence completes only after the final verification condition succeeds. Task
transport completion or montage start is not sequence completion.

## Sequence handoff and task state

An actor may have one active sequence and one validated pending sequence. A new
request never mutates the active StateTree instance in place. The sequence
coordinator validates and prepares the pending definition, then performs one
atomic handoff after the active sequence reaches a permitted cancellation or
completion point.

The coordinator exposes `idle`, `preparing`, `running`, `cancelling`, and
`completed` states. It cannot swap two hidden sequencers based on update order.
Pending work is rejected or superseded through typed policy when another request
arrives before handoff.

Every task uses the closed lifecycle `sleeping` , `running` , `succeeded` ,
`failed` ,
`timed_out`, or `cancelled`. StateTree task status and `FSharActionResult` must
agree. A task cannot report completion solely because its object was cleared or
its owner changed state.

## Character state projection

Character gameplay state is a typed projection over authoritative movement,
vehicle, collision, and action-sequence observations. The canonical high-level
states are:

<!-- markdownlint-disable MD013 -->

- **State:** `locomotion`
  - **Contract:** Character Movement owns walking, running, jumping, and
    grounded recovery.
- **State:** `in_vehicle`
  - **Contract:** A verified seat and vehicle relationship owns locomotion
    presentation.
- **State:** `entering_vehicle`
  - **Contract:** A typed vehicle-entry sequence owns approach, door, seat, and
    control handoff.
- **State:** `exiting_vehicle`
  - **Contract:** A typed exit sequence owns door, placement, collision, and
    control restoration.
- **State:** `simulation_reaction`
  - **Contract:** Ragdoll or other physics-owned reaction temporarily supersedes
    normal movement.
- **State:** `disabled`
  - **Contract:** No ordinary locomotion or vehicle sequence may start.

<!-- markdownlint-enable MD013 -->

State changes are requested through one character-state port. Enter and exit
hooks acquire or release resources, but they cannot contain hidden mission or
vehicle-ownership mutations. Vehicle entry and exit publish typed start and end
observations only after their corresponding sequence postconditions are
verified.

An invalid door side, blocked exit, missing floor, destroyed vehicle, streaming
change, or interrupted transition follows the sequence failure and compensation
policy. The character cannot be left simultaneously in vehicle and locomotion
ownership.

## Result model

`FSharActionResult` has one status:

<!-- markdownlint-disable MD013 -->

- **Status:** `success`
  - **Meaning:** The declared postcondition was observed.
- **Status:** `rejected`
  - **Meaning:** Preconditions were not met and no side effect began.
- **Status:** `failed`
  - **Meaning:** Execution began but could not reach the postcondition.
- **Status:** `timed_out`
  - **Meaning:** The authored bound elapsed and cleanup completed.
- **Status:** `cancelled`
  - **Meaning:** A permitted external cancellation completed cleanup.
- **Status:** `compensated`
  - **Meaning:** A failure occurred and the declared compensation restored a
    valid state.

<!-- markdownlint-enable MD013 -->

Every non-success result contains a typed reason. Free-form log text is
diagnostic
only and cannot drive a transition.

## Resource model

The resource arbiter supports these canonical resources:

<!-- markdownlint-disable MD013 -->

- **Resource:** `character_movement`
  - **Typical access:** Exclusive for arrive, position, jump, dodge, or forced
    locomotion.
- **Resource:** `character_facing`
  - **Typical access:** Exclusive while an action owns orientation.
- **Resource:** `character_controller`
  - **Typical access:** Exclusive while input or NPC control is overridden.
- **Resource:** `root_motion`
  - **Typical access:** Exclusive for a montage or movement action that applies
    root motion.
- **Resource:** `animation_slot.<name>`
  - **Typical access:** Exclusive within one montage slot group.
- **Resource:** `vehicle_control`
  - **Typical access:** Exclusive for driver or forced vehicle actions.
- **Resource:** `vehicle_door.<door>`
  - **Typical access:** Exclusive for an individual door.
- **Resource:** `interaction_reservation`
  - **Typical access:** Exclusive for the selected interaction source and slot.
- **Resource:** `camera_interest`
  - **Typical access:** Shared request handle; final selection belongs to the
    camera subsystem.
- **Resource:** `audio_emitter.<name>`
  - **Typical access:** Exclusive only when stop and replacement semantics
    require it.
- **Resource:** `domain_transaction.<kind>`
  - **Typical access:** Exclusive for one idempotent commit identity.

<!-- markdownlint-enable MD013 -->

Claims are sorted by canonical resource identity before acquisition to prevent
order-dependent deadlock. Partial acquisition releases all earlier leases before
returning `resource_unavailable`.

## Task lifecycle

Every native action task follows this lifecycle:

1. resolve the action and sequence revisions;
1. validate context and preconditions without side effects;
1. acquire all declared resource leases;
1. capture the minimal compensation snapshot;
1. start native movement, animation, vehicle, or presentation work;
1. observe progress and timeout through StateTree ticks or callbacks;
1. verify the declared postcondition;
1. publish typed observations or request the domain transaction;
1. release all leases and transient handles; and
1. return the closed result.

Cancellation enters the same cleanup path. Destruction, streaming, world
teardown,
controller replacement, StateTree stop, and interaction invalidation are
explicit
cancellation reasons.

## Canonical action vocabulary

The registered execution kinds are:

<!-- markdownlint-disable MD013 -->

- **Kind:** `delay`
  - **Required behavior:** Complete after the authored simulation duration;
    pause policy is explicit.
- **Kind:** `publish_event`
  - **Required behavior:** Publish one schema-registered typed event and action
    ordinal.
- **Kind:** `arrive`
  - **Required behavior:** Move through Character Movement or navigation to a
    destination and verify tolerance.
- **Kind:** `orient`
  - **Required behavior:** Rotate toward an authored direction or target within
    angular tolerance.
- **Kind:** `position`
  - **Required behavior:** Move through a bounded interpolation or authored
    slot; never hide an invalid path with a teleport.
- **Kind:** `ground_snap`
  - **Required behavior:** Resolve a valid walkable floor and apply a bounded
    correction.
- **Kind:** `change_locomotion`
  - **Required behavior:** Request the declared walking, vehicle, disabled, or
    contextual locomotion mode.
- **Kind:** `change_controller_state`
  - **Required behavior:** Request a registered NPC or player-controller state
    and verify ownership.
- **Kind:** `change_character_state`
  - **Required behavior:** Request a registered character state through the
    character application port.
- **Kind:** `play_montage`
  - **Required behavior:** Play a montage or section and complete on required
    notify or montage result.
- **Kind:** `play_idle_montage`
  - **Required behavior:** Play an interruptible idle montage and restore the
    prior idle policy.
- **Kind:** `hold_montage_frame`
  - **Required behavior:** Hold a validated frame or section until a typed
    release condition.
- **Kind:** `vehicle_idle`
  - **Required behavior:** Play the driver or passenger presentation while
    vehicle and seat state remain valid.
- **Kind:** `vehicle_door`
  - **Required behavior:** Open, close, lock, unlock, or release one validated
    door.
- **Kind:** `jump`
  - **Required behavior:** Delegate launch, airborne movement, landing, and
    recovery to Character Movement.
- **Kind:** `dodge`
  - **Required behavior:** Execute the authored dodge with collision and
    recovery verification.
- **Kind:** `cringe`
  - **Required behavior:** Play the bounded reaction while preserving movement
    policy.
- **Kind:** `flail`
  - **Required behavior:** Enter and leave the airborne or impact reaction
    through explicit states.
- **Kind:** `get_up`
  - **Required behavior:** Recover from a valid prone or impact state and verify
    locomotion restoration.
- **Kind:** `kick`
  - **Required behavior:** Emit the contact window through an animation notify
    and typed hit query.
- **Kind:** `surf`
  - **Required behavior:** Maintain the authored vehicle-relative presentation
    while attachment remains valid.
- **Kind:** `assign_parameter`
  - **Required behavior:** Write one sequence-local typed value; it cannot
    mutate domain storage.
- **Kind:** `commit_domain_effect`
  - **Required behavior:** Request one registered idempotent application
    transaction and verify its result.

<!-- markdownlint-enable MD013 -->

A new kind requires a schema change, native implementation, validation, and
contract tests. It cannot be added only through a display name.

## Movement actions

### Arrive

`arrive` requires a destination, acceptance radius, path policy, maximum speed,
and timeout. Strict arrival also requires final floor, orientation, and slot
validity. The action succeeds only when Character Movement reports a valid final
state inside tolerance.

Path invalidation, moving destinations, blocked navigation, changed interaction
slots, and world streaming return typed results. A fragile or optional movement
step is represented by failure policy, not a subclass that changes scheduler
semantics.

### Orient

`orient` resolves a world direction or target identity, ignores unsupported
vertical rotation when required by the character policy, and applies bounded
rotation. It succeeds only within authored angular tolerance for the required
stable duration.

### Position

`position` is reserved for short authored presentation alignment after a valid
approach. It declares local or world space, duration, collision policy, maximum
distance, and interruption behavior. Distances outside the validated bound are
rejected rather than teleported.

### Ground snap

`ground_snap` performs a bounded floor query using the character collision
shape.
It rejects missing floors, non-walkable normals, penetration, or a correction
larger than the authored maximum. It never becomes general out-of-bounds
recovery.

## Character locomotion and reactions

Walking and vehicle-idle tasks consume desired speed, direction, seat, and
locomotion data from authoritative components. They do not integrate their own
parallel movement simulation.

Jump uses Character Movement launch and falling state. Pre-jump, airborne,
optional repeated jump, slam, landing, and recovery are explicit StateTree
states
or montage sections. Gravity, launch velocity, target, and boost policy are
typed
parameters. Landing success requires a valid floor and restored movement mode.

Dodge, cringe, flail, get-up, kick, and surf tasks each declare entry state,
montage, movement ownership, collision policy, interruptibility, and terminal
state. They cannot infer gameplay contact from animation time alone.

## Animation actions

Animation actions use montages, sections, slot groups, root motion, and
notifies.
Every definition declares:

- montage and optional section identity;
- slot and resource claim;
- play rate and blend policy;
- looping policy;
- root-motion policy;
- required notifies;
- movement-abort policy;
- timeout; and
- completion and cancellation restoration.

A hold action may pause at a validated section boundary or normalized position.
Release conditions are typed vehicle, input, mission, or actor states. Directly
changing the animation asset's frame count is forbidden.

An idle montage may be superseded by a higher-priority locomotion or interaction
action. Its cancellation restores the prior idle policy without reporting
failure unless the sequence explicitly requires uninterrupted completion.

## Presentation playback tasks

A presentation task submits one typed request to the presentation playback
subsystem and waits for one revision-correlated result. The task definition
contains:

- presentation definition identity;
- owner and sequence revisions;
- participant and target bindings;
- required terminal result kinds;
- skip and cancellation policy;
- timeout and fallback policy; and
- compensation behavior.

The task cannot load arbitrary content, freeze world state, select a camera,
inspect a render flag, or infer completion from animation time. Animation,
camera, cosmetic, and media lifecycle follows the
[presentation playback runtime](presentation-playback-runtime.md).

`completed` or an explicitly accepted `skipped` result may satisfy the task.
`cancelled`, `failed`, unavailable presentation, target loss, and owner
replacement follow the task's declared transition. A stale result cannot advance
a replacement sequence.

Cancellation requests playback teardown and waits for compensation to complete
before releasing the task's resources. The action sequence never restores input,
camera, HUD, or world presentation through cached global state.

## Vehicle actions

Vehicle tasks resolve a stable vehicle identity and current seat state before
acquiring control. Door tasks additionally resolve one canonical door identity.

A door action declares operation, delay, duration or required animation notify,
collision behavior, and character relationship. It succeeds only when the
vehicle port reports the requested terminal door state. Cancellation releases
the
door lease and requests the declared safe state.

`release_vehicle_doors` clears only temporary action ownership. It does not
force
all doors open or closed and cannot override damage, lock, or mission policy.

## Typed events

`publish_event` uses a registered event identity and a reflected payload struct.
Channel, schema, scope, subscription, delivery, and tracing behavior follow the
<!-- markdownlint-disable-next-line MD013 -->
[typed event and observation routing
runtime](typed-event-and-observation-routing-runtime.md).
The event envelope contains sequence identity, action identity, action ordinal,
source identity, optional target identity, world time, and definition revision.

Subscribers cannot change the emitting task's result by listener order. A domain
transition that must influence success is a typed application-port call with a
returned result, not a broadcast event.

## Contextual interaction integration

The interaction subsystem starts a sequence only after candidate selection,
reservation, and final eligibility validation. The sequence receives the
move-only
interaction reservation lease and must return it on every terminal path.

Typical phases are:

1. approach the Smart Object or authored interaction slot;
1. orient and align the character;
1. play character and prop presentation;
1. request the typed domain effect;
1. verify world and domain postconditions; and
1. release or retain the interaction according to its cooldown policy.

Input presses while a non-repeatable sequence is active return
`already_executing` and do not create another sequence.

## One-shot interactions

A one-shot definition has an idempotent completion identity and persistence
policy. Pressing input, acquiring a reservation, or starting animation does not
consume it.

The interaction becomes unavailable only after:

1. the sequence reaches its domain-effect phase;
1. the domain effect commits or the declared presentation-only postcondition is
   verified;
1. the completion record is persisted when required; and
1. the interaction subsystem publishes the updated availability revision.

Failure or cancellation before completion restores availability. A retry after a
committed result reads the existing idempotent transaction and cannot replay the
effect.

## Automatic doors

An automatic door is a contextual world action with no manual input. It uses an
authored trigger or Smart Object occupancy query and these states:

<!-- markdownlint-disable MD013 -->

- **State:** `closed`
  - **Contract:** No eligible occupant is present and collision is in the closed
    state.
- **State:** `opening`
  - **Contract:** The first eligible occupant triggered the open sequence.
- **State:** `open`
  - **Contract:** The required open notify or transform is verified.
- **State:** `closing`
  - **Contract:** The final eligible occupant left and the close sequence is
    active.
- **State:** `blocked`
  - **Contract:** A sweep or overlap prevents safe closing.
- **State:** `disabled`
  - **Contract:** Definition, world, mission, or damage state forbids operation.

<!-- markdownlint-enable MD013 -->

Occupancy is a set of stable actor handles, not a raw integer counter. Duplicate
enter and missing exit notifications cannot make occupancy negative. Destroyed,
unloaded, or ineligible occupants are removed during reconciliation.

The door opens on the transition from zero to one eligible occupant and closes
on
the transition from one to zero. A new occupant during closing returns the door
to opening. A blocked close remains open or retries according to authored
policy.
Sound starts and stops with the transition handle and is always cleared on
cancellation, destruction, pooling, or world teardown.

## Prop and animation interaction registry

Contextual action definitions such as toggle, reverse, play once, looping play,
automatic play, destroy prop, vending machine, phone, dialogue, collectible,
teleport, repair pickup, purchase, and nitro resolve through canonical action
and
interaction identities.

The action catalog records execution kind explicitly. Multiple display aliases
may resolve to one identity, but one alias cannot silently select a different
executor. Duplicate or positional mappings fail generation and asset validation.

## Timing

All durations use seconds in validated typed fields. Simulation-time and
real-time policies are distinct. Paused gameplay, time dilation, cinematic time,
and editor preview behavior are explicit.

A timeout starts after resource acquisition unless the definition explicitly
includes acquisition time. Timeouts use monotonic runtime time and never wall
clock or render frame count. An animation timeout is a safety bound, not normal
completion evidence.

## Cancellation and compensation

Cancellation requests contain a reason and priority. The active task may finish
an uninterruptible critical section only when its definition permits it and the
world is still valid.

Cleanup order is:

1. stop new side effects;
1. cancel native movement, montage, vehicle, camera, audio, and effect handles;
1. invoke typed compensation when required;
1. verify a valid actor and world state;
1. release resource and interaction leases;
1. publish the terminal result; and
1. clear sequence-local data.

Compensation cannot reverse a committed idempotent domain transaction. It
instead
reconciles presentation to the committed state.

## Streaming and destruction

A sequence may continue across streaming only when every required actor and
resource is explicitly persistent. Otherwise streaming invalidation cancels the
sequence before the actor or component is destroyed.

Actor destruction, controller replacement, vehicle destruction, interaction
source invalidation, and world teardown are terminal cancellation inputs. Tasks
must use weak object handles and stable identities so cleanup does not
dereference
destroyed objects.

## Historical optimization translation

<!-- markdownlint-disable MD013 -->

- **Historical technique:** Fixed action-handler array
  - **Original constraint:** Bounded memory and simple global lookup.
  - **Unreal replacement:** Validated subsystem map keyed by canonical identity.
- **Historical technique:** Parallel action-name and constructor arrays
  - **Original constraint:** Avoid reflection and dynamic registration.
  - **Unreal replacement:** Generated definitions plus closed native executor
    registration.
- **Historical technique:** Custom task scheduler and memory pool
  - **Original constraint:** Reduce allocation and coordinate character actions.
  - **Unreal replacement:** StateTree task instances, Unreal object lifetime,
    and bounded resource leases.
- **Historical technique:** Frame-based animation control
  - **Original constraint:** Limited montage and notify tooling.
  - **Unreal replacement:** Animation montages, sections, slots, root motion,
    and notifies.
- **Historical technique:** Direct character transform writes
  - **Original constraint:** Simple scripted alignment.
  - **Unreal replacement:** Character Movement, navigation, Smart Object slots,
    and bounded presentation alignment.
- **Historical technique:** Raw global event payloads
  - **Original constraint:** Low-overhead cross-system signaling.
  - **Unreal replacement:** Reflected typed events and application-port results.
- **Historical technique:** Entrant counters for automatic doors
  - **Original constraint:** Minimal trigger state.
  - **Unreal replacement:** Stable occupancy sets and explicit door states.

<!-- markdownlint-enable MD013 -->

Native optimization cannot change action order, timing policy, resource
exclusivity, domain results, or cancellation cleanup.

## Invariants

- Every active action resolves to one definition revision and native executor.
- Every sequence step has one stable action ordinal.
- Resource acquisition order is deterministic.
- No two active tasks hold conflicting exclusive leases.
- A task publishes one terminal result.
- Domain commits are idempotent and occur through application ports.
- Animation start is never treated as animation completion.
- One-shot availability changes only after verified completion.
- Automatic-door occupancy cannot be negative or contain stale actors.
- Cancellation releases every movement, animation, vehicle, interaction, camera,
  audio, and domain lease.
- Streaming and destruction cannot leave a task active against an invalid actor.

## Failure behavior

Missing definitions, executors, resources, actors, vehicles, montages, sockets,
doors, states, events, or application ports fail before side effects. The result
is `rejected` with a typed reason.

A runtime failure stops further steps, runs the declared compensation, and
verifies a valid state. It returns `failed`, `timed_out`, `cancelled`, or
`compensated`.
Unknown exceptions, stale callbacks, duplicate completion, and resource leaks
are
contract violations and fail automated tests.

## Validation

Asset and generated-data validation rejects:

- duplicate action or sequence identities;
- unknown execution kinds or missing native executors;
- parameter payloads that do not match their execution kind;
- conflicting resources in one parallel state;
- circular or unreachable sequence transitions;
- missing timeouts for tasks that can wait indefinitely without explicit
  permission;
- unresolved montages, sections, slots, notifies, sockets, doors, states,
  events,
  or interaction definitions;
- one-shot actions without idempotent completion identity;
- automatic doors without open, close, blocked, and cancellation policies; and
- domain-effect tasks without a registered application port and verification.

## Verification

Automated tests must prove:

- catalog results are identical under shuffled definition registration;
- duplicate identities and positional mapping drift fail validation;
- sequence ordering, parallel resource compatibility, and deterministic lease
  acquisition;
- success, rejection, failure, timeout, cancellation, and compensation paths for
  every execution kind;
- no resource leak after actor, controller, vehicle, source, or world
  destruction;
- montage completion by notify or montage result and safety timeout behavior;
- arrive, orient, position, and ground-snap tolerances and invalid paths;
- jump, dodge, cringe, flail, get-up, kick, and surf state restoration;
- vehicle-door ownership, cancellation, damage, and release behavior;
- one typed event with one action ordinal under repeated callbacks;
- one-shot retry before completion and idempotent retry after commit;
- automatic-door duplicate enter, missing exit, blocked close, reopen,
  streaming,
  and destruction reconciliation;
- interaction reservation release on every terminal path; and
- parity scenarios for every registered campaign action identity.

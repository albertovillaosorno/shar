# Typed action-sequence runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

- [Typed StateTree action sequences](../../adr/unreal/runtime/typed-state-tree-action-sequences.md)
- [Contextual interaction query and transaction boundary](../../adr/unreal/runtime/contextual-interaction-query-and-transaction.md)
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
- [Transactional phone-booth vehicle retrieval](../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md)

## Purpose

This specification defines the reusable native action vocabulary for characters,
vehicles, props, interactions, and mission presentation. It replaces positional
string registries, fixed handler arrays, custom task scheduling, and raw event
payloads with validated definitions and typed StateTree tasks.

## Ownership

The action runtime coordinates presentation and movement. It does not own mission,
progression, reward, economy, save, vehicle-ownership, or collectible state.
Those effects remain behind application ports.

| Authority | Responsibility |
| :--- | :--- |
| Generated action catalog | Stable identities, execution kinds, parameters, resources, and verification policy. |
| StateTree | Ordered states, transitions, task lifetime, failure, and cancellation. |
| Resource arbiter | Exclusive and shared runtime leases. |
| Native task library | Typed movement, animation, vehicle, delay, event, and state operations. |
| Domain services | Authoritative gameplay effects and persistent results. |

## Runtime topology

The runtime module owns these C++ types:

| Type | Responsibility |
| :--- | :--- |
| `USharActionDefinition` | Primary data asset for one reusable action contract. |
| `USharActionSequenceDefinition` | Immutable ordered plan using registered action identities. |
| `USharActionCatalogSubsystem` | Definition lookup, revision validation, and executor registration. |
| `USharActionResourceArbiter` | Grants and releases typed resource leases. |
| `FSharActionContext` | Immutable actor, interaction, mission, vehicle, and world context. |
| `FSharActionRequest` | Definition identity, typed parameters, expected revision, and action ordinal. |
| `FSharActionResult` | Closed result with reason, observations, and verification evidence. |
| `FSharActionLease` | Move-only lease for one declared resource. |
| `FSharActionSequenceHandle` | Cancellation-safe handle for one active sequence. |

The StateTree schema exposes these types as external data. It never receives raw
pointers whose lifetime is not represented by a weak object handle or stable
identity.

## Definition contract

Every `USharActionDefinition` contains:

| Field | Contract |
| :--- | :--- |
| `ActionId` | Globally unique canonical identity. |
| `ExecutionKind` | Closed enum selecting one native task implementation. |
| `GameplayTags` | Family, capability, blocking, mission, and presentation tags. |
| `ParameterSchema` | Closed typed payload for the selected execution kind. |
| `RequiredResources` | Ordered resource claims with exclusive or shared access. |
| `Preconditions` | Required actor, world, mission, vehicle, and interaction state. |
| `TimeoutPolicy` | Positive timeout or explicit no-timeout permission. |
| `CancellationPolicy` | Allowed cancellation points and required compensation. |
| `VerificationPolicy` | Observable postcondition required for success. |
| `PresentationPolicy` | Optional animation, sound, effects, prompt, and camera data. |
| `DefinitionRevision` | Immutable revision used to reject stale requests. |

Every `USharActionSequenceDefinition` contains:

| Field | Contract |
| :--- | :--- |
| `SequenceId` | Globally unique canonical identity. |
| `StateTreeTemplate` | Canonical template compatible with the action schema. |
| `Steps` | Ordered action identities and typed parameter bindings. |
| `FailurePolicy` | Abort, compensate, fallback, retry, or continue for each declared failure class. |
| `SequenceTimeout` | Optional total bound not shorter than required step bounds. |
| `RequiredContext` | Actor, vehicle, interaction, mission, or world context requirements. |
| `VerificationPolicy` | Final observable sequence postcondition. |
| `DefinitionRevision` | Immutable revision used to reject stale execution. |

Definitions cannot contain source-language callbacks, free-form script fragments,
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
- an asset, montage, socket, vehicle door, state, or event identity is unresolved;
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

Every task uses the closed lifecycle `sleeping`, `running`, `succeeded`, `failed`,
`timed_out`, or `cancelled`. StateTree task status and `FSharActionResult` must
agree. A task cannot report completion solely because its object was cleared or
its owner changed state.

## Character state projection

Character gameplay state is a typed projection over authoritative movement,
vehicle, collision, and action-sequence observations. The canonical high-level
states are:

| State | Contract |
| :--- | :--- |
| `locomotion` | Character Movement owns walking, running, jumping, and grounded recovery. |
| `in_vehicle` | A verified seat and vehicle relationship owns locomotion presentation. |
| `entering_vehicle` | A typed vehicle-entry sequence owns approach, door, seat, and control handoff. |
| `exiting_vehicle` | A typed exit sequence owns door, placement, collision, and control restoration. |
| `simulation_reaction` | Ragdoll or other physics-owned reaction temporarily supersedes normal movement. |
| `disabled` | No ordinary locomotion or vehicle sequence may start. |

State changes are requested through one character-state port. Enter and exit
hooks acquire or release resources, but they cannot contain hidden mission or
vehicle-ownership mutations. Vehicle entry and exit publish typed start and end
observations only after their corresponding sequence postconditions are verified.

An invalid door side, blocked exit, missing floor, destroyed vehicle, streaming
change, or interrupted transition follows the sequence failure and compensation
policy. The character cannot be left simultaneously in vehicle and locomotion
ownership.

## Result model

`FSharActionResult` has one status:

| Status | Meaning |
| :--- | :--- |
| `success` | The declared postcondition was observed. |
| `rejected` | Preconditions were not met and no side effect began. |
| `failed` | Execution began but could not reach the postcondition. |
| `timed_out` | The authored bound elapsed and cleanup completed. |
| `cancelled` | A permitted external cancellation completed cleanup. |
| `compensated` | A failure occurred and the declared compensation restored a valid state. |

Every non-success result contains a typed reason. Free-form log text is diagnostic
only and cannot drive a transition.

## Resource model

The resource arbiter supports these canonical resources:

| Resource | Typical access |
| :--- | :--- |
| `character_movement` | Exclusive for arrive, position, jump, dodge, or forced locomotion. |
| `character_facing` | Exclusive while an action owns orientation. |
| `character_controller` | Exclusive while input or NPC control is overridden. |
| `root_motion` | Exclusive for a montage or movement action that applies root motion. |
| `animation_slot.<name>` | Exclusive within one montage slot group. |
| `vehicle_control` | Exclusive for driver or forced vehicle actions. |
| `vehicle_door.<door>` | Exclusive for an individual door. |
| `interaction_reservation` | Exclusive for the selected interaction source and slot. |
| `camera_interest` | Shared request handle; final selection belongs to the camera subsystem. |
| `audio_emitter.<name>` | Exclusive only when stop and replacement semantics require it. |
| `domain_transaction.<kind>` | Exclusive for one idempotent commit identity. |

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

Cancellation enters the same cleanup path. Destruction, streaming, world teardown,
controller replacement, StateTree stop, and interaction invalidation are explicit
cancellation reasons.

## Canonical action vocabulary

The registered execution kinds are:

| Kind | Required behavior |
| :--- | :--- |
| `delay` | Complete after the authored simulation duration; pause policy is explicit. |
| `publish_event` | Publish one schema-registered typed event and action ordinal. |
| `arrive` | Move through Character Movement or navigation to a destination and verify tolerance. |
| `orient` | Rotate toward an authored direction or target within angular tolerance. |
| `position` | Move through a bounded interpolation or authored slot; never hide an invalid path with a teleport. |
| `ground_snap` | Resolve a valid walkable floor and apply a bounded correction. |
| `change_locomotion` | Request the declared walking, vehicle, disabled, or contextual locomotion mode. |
| `change_controller_state` | Request a registered NPC or player-controller state and verify ownership. |
| `change_character_state` | Request a registered character state through the character application port. |
| `play_montage` | Play a montage or section and complete on required notify or montage result. |
| `play_idle_montage` | Play an interruptible idle montage and restore the prior idle policy. |
| `hold_montage_frame` | Hold a validated frame or section until a typed release condition. |
| `vehicle_idle` | Play the driver or passenger presentation while vehicle and seat state remain valid. |
| `vehicle_door` | Open, close, lock, unlock, or release one validated door. |
| `jump` | Delegate launch, airborne movement, landing, and recovery to Character Movement. |
| `dodge` | Execute the authored dodge with collision and recovery verification. |
| `cringe` | Play the bounded reaction while preserving movement policy. |
| `flail` | Enter and leave the airborne or impact reaction through explicit states. |
| `get_up` | Recover from a valid prone or impact state and verify locomotion restoration. |
| `kick` | Emit the contact window through an animation notify and typed hit query. |
| `surf` | Maintain the authored vehicle-relative presentation while attachment remains valid. |
| `assign_parameter` | Write one sequence-local typed value; it cannot mutate domain storage. |
| `commit_domain_effect` | Request one registered idempotent application transaction and verify its result. |

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

`ground_snap` performs a bounded floor query using the character collision shape.
It rejects missing floors, non-walkable normals, penetration, or a correction
larger than the authored maximum. It never becomes general out-of-bounds recovery.

## Character locomotion and reactions

Walking and vehicle-idle tasks consume desired speed, direction, seat, and
locomotion data from authoritative components. They do not integrate their own
parallel movement simulation.

Jump uses Character Movement launch and falling state. Pre-jump, airborne,
optional repeated jump, slam, landing, and recovery are explicit StateTree states
or montage sections. Gravity, launch velocity, target, and boost policy are typed
parameters. Landing success requires a valid floor and restored movement mode.

Dodge, cringe, flail, get-up, kick, and surf tasks each declare entry state,
montage, movement ownership, collision policy, interruptibility, and terminal
state. They cannot infer gameplay contact from animation time alone.

## Animation actions

Animation actions use montages, sections, slot groups, root motion, and notifies.
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

## Vehicle actions

Vehicle tasks resolve a stable vehicle identity and current seat state before
acquiring control. Door tasks additionally resolve one canonical door identity.

A door action declares operation, delay, duration or required animation notify,
collision behavior, and character relationship. It succeeds only when the
vehicle port reports the requested terminal door state. Cancellation releases the
door lease and requests the declared safe state.

`release_vehicle_doors` clears only temporary action ownership. It does not force
all doors open or closed and cannot override damage, lock, or mission policy.

## Typed events

`publish_event` uses a registered event identity and a reflected payload struct.
Channel, schema, scope, subscription, delivery, and tracing behavior follow the
[typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md).
The event envelope contains sequence identity, action identity, action ordinal,
source identity, optional target identity, world time, and definition revision.

Subscribers cannot change the emitting task's result by listener order. A domain
transition that must influence success is a typed application-port call with a
returned result, not a broadcast event.

## Contextual interaction integration

The interaction subsystem starts a sequence only after candidate selection,
reservation, and final eligibility validation. The sequence receives the move-only
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

| State | Contract |
| :--- | :--- |
| `closed` | No eligible occupant is present and collision is in the closed state. |
| `opening` | The first eligible occupant triggered the open sequence. |
| `open` | The required open notify or transform is verified. |
| `closing` | The final eligible occupant left and the close sequence is active. |
| `blocked` | A sweep or overlap prevents safe closing. |
| `disabled` | Definition, world, mission, or damage state forbids operation. |

Occupancy is a set of stable actor handles, not a raw integer counter. Duplicate
enter and missing exit notifications cannot make occupancy negative. Destroyed,
unloaded, or ineligible occupants are removed during reconciliation.

The door opens on the transition from zero to one eligible occupant and closes on
the transition from one to zero. A new occupant during closing returns the door
to opening. A blocked close remains open or retries according to authored policy.
Sound starts and stops with the transition handle and is always cleared on
cancellation, destruction, pooling, or world teardown.

## Prop and animation interaction registry

Contextual action definitions such as toggle, reverse, play once, looping play,
automatic play, destroy prop, vending machine, phone, dialogue, collectible,
teleport, repair pickup, purchase, and nitro resolve through canonical action and
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

Compensation cannot reverse a committed idempotent domain transaction. It instead
reconciles presentation to the committed state.

## Streaming and destruction

A sequence may continue across streaming only when every required actor and
resource is explicitly persistent. Otherwise streaming invalidation cancels the
sequence before the actor or component is destroyed.

Actor destruction, controller replacement, vehicle destruction, interaction
source invalidation, and world teardown are terminal cancellation inputs. Tasks
must use weak object handles and stable identities so cleanup does not dereference
destroyed objects.

## Historical optimization translation

| Historical technique | Original constraint | Unreal replacement |
| :--- | :--- | :--- |
| Fixed action-handler array | Bounded memory and simple global lookup. | Validated subsystem map keyed by canonical identity. |
| Parallel action-name and constructor arrays | Avoid reflection and dynamic registration. | Generated definitions plus closed native executor registration. |
| Custom task scheduler and memory pool | Reduce allocation and coordinate character actions. | StateTree task instances, Unreal object lifetime, and bounded resource leases. |
| Frame-based animation control | Limited montage and notify tooling. | Animation montages, sections, slots, root motion, and notifies. |
| Direct character transform writes | Simple scripted alignment. | Character Movement, navigation, Smart Object slots, and bounded presentation alignment. |
| Raw global event payloads | Low-overhead cross-system signaling. | Reflected typed events and application-port results. |
| Entrant counters for automatic doors | Minimal trigger state. | Stable occupancy sets and explicit door states. |

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
Unknown exceptions, stale callbacks, duplicate completion, and resource leaks are
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
- unresolved montages, sections, slots, notifies, sockets, doors, states, events,
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
- no resource leak after actor, controller, vehicle, source, or world destruction;
- montage completion by notify or montage result and safety timeout behavior;
- arrive, orient, position, and ground-snap tolerances and invalid paths;
- jump, dodge, cringe, flail, get-up, kick, and surf state restoration;
- vehicle-door ownership, cancellation, damage, and release behavior;
- one typed event with one action ordinal under repeated callbacks;
- one-shot retry before completion and idempotent retry after commit;
- automatic-door duplicate enter, missing exit, blocked close, reopen, streaming,
  and destruction reconciliation;
- interaction reservation release on every terminal path; and
- parity scenarios for every registered campaign action identity.

# Contextual interaction runtime

## Governing decisions

- [Contextual interaction query and transaction boundary](../../adr/unreal/runtime/contextual-interaction-query-and-transaction.md)
- [Typed StateTree action sequences](../../adr/unreal/runtime/typed-state-tree-action-sequences.md)
- [Typed action-sequence runtime](typed-action-sequence-runtime.md)
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
- [Transactional phone-booth vehicle retrieval](../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md)
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)

## Purpose

This specification defines the one canonical Unreal runtime for discovering,
presenting, reserving, and executing context-sensitive world actions. It covers
manual interactions, automatic interactions, passive pickups, purchases, and
mission-owned action targets without allowing any world actor to poll player
input or mutate unrelated domain storage.

## Runtime topology

The runtime module owns these C++ types:

| Type | Responsibility |
| :--- | :--- |
| `USharInteractionDefinition` | Primary data asset containing the stable interaction contract. |
| `USharInteractionSourceComponent` | World-local source, authored slots, bounds, and current source revision. |
| `USharInteractorComponent` | Character-local candidate collection and prompt projection. |
| `USharInteractionSubsystem` | World authority for selection, reservations, transactions, and results. |
| `ISharInteractionExecutor` | Typed application port implemented once per interaction kind. |
| `FSharInteractionQuery` | Immutable query input for one interactor and one simulation frame. |
| `FSharInteractionCandidate` | Evaluated candidate with no committed side effects. |
| `FSharInteractionReservation` | Move-only reservation token with source and interactor revisions. |
| `FSharInteractionResult` | Typed success, rejection, cancellation, or compensated-failure result. |

Blueprints may configure definitions and source components. They may not own
candidate ordering, reward commits, save writes, economy mutations, vehicle
ownership, mission progression, or compensation.

## Definition contract

Every `USharInteractionDefinition` contains:

| Field | Contract |
| :--- | :--- |
| `InteractionId` | Globally unique canonical identifier. |
| `InteractionTags` | Gameplay Tags that classify family, role, and authored context. |
| `ExecutionKind` | Closed enum selecting one registered typed executor. |
| `InputPolicy` | Manual press, automatic enter, automatic exit, or passive pickup. |
| `Priority` | Signed authored priority used before distance and identity. |
| `Prompt` | Localized text identity, icon identity, and accessibility description. |
| `EligibilityPolicy` | Required and blocked tags, mission state, save state, and source state. |
| `SlotPolicy` | Required character slot, facing tolerance, occupancy, and reservation mode. |
| `PresentationPolicy` | Movement lock, camera, animation, audio, prop animation, and duration. |
| `EffectPolicy` | Typed executor payload; never an arbitrary object path or script fragment. |
| `PersistencePolicy` | None, session, level, profile, or permanent collection state. |
| `CooldownPolicy` | No cooldown, fixed cooldown, or respawn definition identity. |
| `CancellationPolicy` | Allowed phases and required compensation behavior. |
| `VerificationPolicy` | Observable state that must confirm a successful commit. |

Definitions with missing executor registration, unresolved references, invalid
Gameplay Tags, an empty canonical identity, or contradictory input and slot
policies fail asset validation and cannot enter the runtime catalog.

## Canonical interaction kinds

| Kind | Required behavior |
| :--- | :--- |
| `mission_dialogue` | Reserve the speaker, position the player when required, run dialogue, and publish the declared mission observation. |
| `enter_interior` | Delegate the complete transition to the interior transaction port. |
| `enter_vehicle` | Revalidate vehicle state, seat availability, mission restrictions, and current character state before entry. |
| `summon_vehicle` | Open the phone-booth selection flow and delegate retrieval to the vehicle-retrieval transaction. |
| `prop_attach` | Attach the declared prop to the validated character socket and publish the attachment result. |
| `prop_toggle` | Move an authored prop animation toward the opposite stable endpoint. |
| `prop_reverse` | Reverse the active authored prop animation without rebuilding its state. |
| `prop_play_once` | Play from the declared start state to the terminal state once. |
| `prop_play_loop` | Start or stop a cyclic animation through explicit state, not repeated input polling. |
| `prop_auto_play` | Begin when the first eligible occupant enters and stop when the last eligible occupant exits. |
| `prop_auto_in_out` | Animate toward the occupied state on enter and toward the idle state after the final exit. |
| `destroy_prop` | Apply the declared damage transaction, wait for the authoritative destruction result, then publish mission and reward observations once. |
| `vending_machine` | Play the authored character and prop sequence, commit the configured economy effect once, and enforce cooldown. |
| `prank_phone` | Play the authored phone sequence and event result without entering vehicle-retrieval UI. |
| `doorbell` | Play one doorbell event while respecting cooldown and source availability. |
| `open_door` | Reserve the doorway, position the character, animate the door, and release only after passage or cancellation. |
| `talk_food` | Run the declared conversation and food presentation without creating a collectible save row. |
| `talk_collectible` | Run dialogue and then delegate the collectible grant to its typed port. |
| `collectible` | Commit a one-time or respawnable pickup according to the definition. |
| `repair_pickup` | Repair the active vehicle context and schedule the authored respawn. |
| `nitro_pickup` | Delegate the charge grant to the vehicle capability port. |
| `teleport` | Reserve both ends, validate the destination, transition atomically, and recover to the source on failure. |
| `purchase_vehicle` | Quote the canonical offer, debit currency, grant ownership, and persist one atomic result. |
| `purchase_costume` | Quote the canonical offer, debit currency, grant the costume, and persist one atomic result. |
| `generic_event` | Publish only a schema-registered event payload with a declared consumer. |

No generic-event definition may substitute for a kind that has domain effects.

## Candidate discovery

Each interactor maintains a bounded overlap set from interaction-source collision
channels. Streaming registration and overlap notifications update the set;
there is no world-wide per-frame actor scan.

For each query, the interactor supplies:

- its stable actor identity and state revision;
- world position and forward vector;
- current input context and local player identity;
- owned, required, and blocked Gameplay Tags;
- mission, progression, vehicle, and interior query snapshots; and
- the previous selected interaction identity, when still present.

The subsystem evaluates each source without side effects. Rejected candidates
carry a typed reason for diagnostics but do not reach the ordinary prompt model.
Accepted candidates are sorted by:

1. descending authored priority;
1. ascending squared distance to the resolved use slot; and
1. ascending canonical interaction identity.

Physics overlap order, actor creation order, streaming order, pointer values, and
frame timing are never selection inputs. The previous candidate remains selected
only when it still wins the same ordering contract.

## Prompt projection

The prompt is a projection of the selected accepted candidate. The prompt and
execution therefore cannot use different eligibility paths.

The prompt model contains the interaction identity, localized text identity,
input glyph action, icon identity, availability state, optional rejection reason
for accessibility presentation, and source screen anchor. UI code never reads
world actors directly.

A prompt disappears immediately when its reservation is owned by another actor,
its source revision changes, its eligibility snapshot becomes stale, or the
source leaves the bounded candidate set.

## Reservation and execution

Manual interaction begins only on the Enhanced Input `Interact` trigger. The
subsystem performs these phases in order:

1. Resolve the currently selected candidate.
1. Re-evaluate eligibility using current domain snapshots.
1. Claim the Smart Object slot when the definition requires one.
1. Create a reservation token containing source, interactor, and domain
   revisions.
1. Revalidate the token immediately before presentation begins.
1. Prepare movement, facing, camera, animation, and audio without committing
   rewards or progression.
1. Invoke the typed executor.
1. Verify the declared postcondition.
1. Publish the result exactly once.
1. Release the reservation and restore presentation state.

A source revision change invalidates every outstanding token for that source.
An interactor may own at most one non-passive reservation. Repeated input during
a non-repeatable transaction returns `already_executing` without additional side
effects.

## Character placement and presentation

Authored use slots provide transform, facing tolerance, and optional approach
radius. Character movement uses the native movement component and animation
montages; interaction code must not teleport a character to mask an invalid
approach path unless the definition is explicitly a teleport interaction.

Presentation preparation may lock movement and camera input only for the declared
phase. Every success, cancellation, source unload, character destruction, and
executor failure restores those locks through one scoped presentation token.

Prop animation state is explicit: idle, moving forward, terminal, moving
backward, cyclic, destroyed, or unavailable. Direction and normalized progress
are saved only when the definition declares session or persistent continuity.

## Automatic occupancy interactions

Automatic interactions never simulate an input press. The first eligible
occupant starts the occupied transition. Additional occupants increment the
occupancy set without replaying the start effect. The final eligible occupant
leaving starts the exit transition.

Occupancy is keyed by stable actor identity. Duplicate overlap events, actor
unload, destroyed actors, and streaming removal must converge to the same set.

## Collectibles and respawnables

One-time collectibles reserve by canonical collectible identity and commit their
save row before presentation reports success. A repeated collection request
returns `already_collected` without replaying rewards.

Repair pickups target the vehicle the player currently occupies. When the player
is on foot, they target the last valid player-controlled vehicle retained by the
vehicle-context service for the current level. A successful repair restores the
complete driveable state and all visible damage channels supported by the vehicle
runtime. The base respawn interval is approximately one minute and is authored as
a duration, not encoded in the pickup actor.

Alien-camera collectibles are adversarial destructible targets rather than
passive overlaps. Destruction, currency reward, level-progress credit, visual
shutdown, and mission observations commit once from the authoritative destruction
result. Nearby repair or card pickups may publish an alert stimulus, but they do
not call camera behavior directly.

## Vehicle and interior delegation

Vehicle entry, phone-booth retrieval, and interior transitions remain application
ports. The interaction subsystem owns only candidate selection, reservation, and
presentation handoff. It does not own seat state, vehicle spawning, world travel,
or interior streaming.

If a delegated transaction times out or fails, the interaction result preserves
the typed downstream error. The source remains available only when its own state
and the downstream domain still permit retry.

## Purchases

A purchase interaction resolves a canonical offer from the gameplay catalog.
The displayed price, eligibility result, debit, ownership grant, and save write
use one offer revision. A changed price or ownership state invalidates the
reservation and requires a new quote.

Currency is never debited before the grant is prepared. Success is published only
after the economy and ownership transaction commits durably. Duplicate purchase
requests for an owned item return `already_owned` and never debit currency.

## Streaming and lifetime

Source registration is idempotent. A source may register only after its
definition and world identity are valid. Unregistering a source removes it from
all candidate sets and cancels or compensates active transactions according to
their current phase.

Definitions are addressed through primary asset identity and load bundles.
Runtime state never depends on editor package iteration or local filesystem
layout. Soft references are resolved before a source becomes eligible.

## Failure behavior

The runtime returns typed results including:

- `not_found`;
- `not_eligible` with a stable reason;
- `source_stale`;
- `interactor_stale`;
- `slot_unavailable`;
- `already_executing`;
- `cancelled`;
- `downstream_rejected`;
- `verification_failed`; and
- `compensation_failed`.

A verification or compensation failure disables the source instance for the
session, records diagnostics with canonical identities, and prevents silent
retries. It does not grant partial rewards or advance mission state.

## Invariants

- One selected prompt maps to one evaluated candidate.
- One non-passive reservation exists per interactor.
- One exclusive slot has at most one reservation owner.
- Rewards and progression publish at most once per transaction identity.
- World actors do not poll input or write save, mission, economy, or ownership
  storage.
- Candidate ordering is deterministic for identical snapshots.
- A source unload cannot leave movement, camera, animation, or slot locks active.
- Generic events cannot carry unregistered payloads or hidden domain effects.

## Verification

Automation must prove:

- deterministic winner selection for overlapping equal-distance sources;
- prompt and execution eligibility parity;
- stale source and interactor revision rejection;
- duplicate input suppression while a transaction is active;
- reservation release on success, cancellation, unload, and failure;
- automatic occupancy behavior with duplicate enter and exit notifications;
- one-time collectible idempotency and respawnable pickup timing;
- repair targeting while driving and while on foot;
- alien-camera destruction committing reward and progress once;
- purchase rollback when debit or grant preparation fails;
- interior, vehicle-entry, phone-booth, and teleport downstream failures;
- prop animation recovery at every cancellation phase; and
- save and reload preservation for every persistent interaction policy.

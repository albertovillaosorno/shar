# Contextual interaction runtime

## Governing decisions

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Contextual interaction query and transaction boundary](../../adr/unreal/runtime/contextual-interaction-query-and-transaction.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD013 -->
- [Typed StateTree action
  sequences](../../adr/unreal/runtime/typed-state-tree-action-sequences.md)
- [Typed action-sequence runtime](typed-action-sequence-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission world-entity and respawn
  runtime](mission-world-entity-and-respawn-runtime.md)
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Transactional phone-booth vehicle retrieval](../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content
  catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)

## Purpose

This specification defines the one canonical Unreal runtime for discovering,
presenting, reserving, and executing context-sensitive world actions. It covers
manual interactions, automatic interactions, passive pickups, purchases, and
mission-owned action targets without allowing any world actor to poll player
input or mutate unrelated domain storage.

## Runtime topology

The runtime module owns these C++ types:

<!-- markdownlint-disable MD013 -->

- **Type:** `USharInteractionDefinition`
  - **Responsibility:** Primary data asset containing the stable interaction
    contract.
- **Type:** `USharInteractionSourceComponent`
  - **Responsibility:** World-local source, authored slots, bounds, and current
    source revision.
- **Type:** `USharInteractorComponent`
  - **Responsibility:** Character-local candidate collection and prompt
    projection.
- **Type:** `USharInteractionSubsystem`
  - **Responsibility:** World authority for selection, reservations,
    transactions, and results.
- **Type:** `ISharInteractionExecutor`
  - **Responsibility:** Typed application port implemented once per interaction
    kind.
- **Type:** `FSharInteractionQuery`
  - **Responsibility:** Immutable query input for one interactor and one
    simulation frame.
- **Type:** `FSharInteractionCandidate`
  - **Responsibility:** Evaluated candidate with no committed side effects.
- **Type:** `FSharInteractionReservation`
  - **Responsibility:** Move-only reservation token with source and interactor
    revisions.
- **Type:** `FSharInteractionResult`
  - **Responsibility:** Typed success, rejection, cancellation, or
    compensated-failure result.

<!-- markdownlint-enable MD013 -->

Blueprints may configure definitions and source components. They may not own
candidate ordering, reward commits, save writes, economy mutations, vehicle
ownership, mission progression, or compensation.

## Definition contract

Every `USharInteractionDefinition` contains:

<!-- markdownlint-disable MD013 -->

- **Field:** `InteractionId`
  - **Contract:** Globally unique canonical identifier.
- **Field:** `InteractionTags`
  - **Contract:** Gameplay Tags that classify family, role, and authored
    context.
- **Field:** `ExecutionKind`
  - **Contract:** Closed enum selecting one registered typed executor.
- **Field:** `InputPolicy`
  - **Contract:** Manual press, automatic enter, automatic exit, or passive
    pickup.
- **Field:** `Priority`
  - **Contract:** Signed authored priority used before distance and identity.
- **Field:** `Prompt`
  - **Contract:** Localized text identity, icon identity, and accessibility
    description.
- **Field:** `EligibilityPolicy`
  - **Contract:** Required and blocked tags, mission state, save state, and
    source state.
- **Field:** `SlotPolicy`
  - **Contract:** Required character slot, facing tolerance, occupancy, and
    reservation mode.
- **Field:** `PresentationPolicy`
  - **Contract:** Movement lock, camera, animation, audio, prop animation, and
    duration.
- **Field:** `EffectPolicy`
  - **Contract:** Typed executor payload; never an arbitrary object path or
    script fragment.
- **Field:** `PersistencePolicy`
  - **Contract:** None, session, level, profile, or permanent collection state.
- **Field:** `CooldownPolicy`
  - **Contract:** No cooldown, fixed cooldown, or respawn definition identity.
- **Field:** `CancellationPolicy`
  - **Contract:** Allowed phases and required compensation behavior.
- **Field:** `VerificationPolicy`
  - **Contract:** Observable state that must confirm a successful commit.

<!-- markdownlint-enable MD013 -->

Definitions with missing executor registration, unresolved references, invalid
Gameplay Tags, an empty canonical identity, or contradictory input and slot
policies fail asset validation and cannot enter the runtime catalog.

## Canonical interaction kinds

<!-- markdownlint-disable MD013 -->

- **Kind:** `mission_dialogue`
  - **Required behavior:** Reserve the speaker, position the player when
    required, run dialogue, and publish the declared mission observation.
- **Kind:** `enter_interior`
  - **Required behavior:** Delegate the complete transition to the interior
    transaction port.
- **Kind:** `enter_vehicle`
  - **Required behavior:** Revalidate vehicle state, seat availability, mission
    restrictions, and current character state before entry.
- **Kind:** `summon_vehicle`
  - **Required behavior:** Open the phone-booth selection flow and delegate
    retrieval to the vehicle-retrieval transaction.
- **Kind:** `prop_attach`
  - **Required behavior:** Attach the declared prop to the validated character
    socket and publish the attachment result.
- **Kind:** `prop_toggle`
  - **Required behavior:** Move an authored prop animation toward the opposite
    stable endpoint.
- **Kind:** `prop_reverse`
  - **Required behavior:** Reverse the active authored prop animation without
    rebuilding its state.
- **Kind:** `prop_play_once`
  - **Required behavior:** Play from the declared start state to the terminal
    state once.
- **Kind:** `prop_play_loop`
  - **Required behavior:** Start or stop a cyclic animation through explicit
    state, not repeated input polling.
- **Kind:** `prop_auto_play`
  - **Required behavior:** Begin when the first eligible occupant enters and
    stop when the last eligible occupant exits.
- **Kind:** `prop_auto_in_out`
  - **Required behavior:** Animate toward the occupied state on enter and toward
    the idle state after the final exit.
- **Kind:** `destroy_prop`
  - **Required behavior:** Apply the declared damage transaction, wait for the
    authoritative destruction result, then publish mission and reward
    observations once.
- **Kind:** `vending_machine`
  - **Required behavior:** Play the authored character and prop sequence, commit
    the configured economy effect once, and enforce cooldown.
- **Kind:** `prank_phone`
  - **Required behavior:** Play the authored phone sequence and event result
    without entering vehicle-retrieval UI.
- **Kind:** `doorbell`
  - **Required behavior:** Play one doorbell event while respecting cooldown and
    source availability.
- **Kind:** `open_door`
  - **Required behavior:** Reserve the doorway, position the character, animate
    the door, and release only after passage or cancellation.
- **Kind:** `talk_food`
  - **Required behavior:** Run the declared conversation and food presentation
    without creating a collectible save row.
- **Kind:** `talk_collectible`
  - **Required behavior:** Run dialogue and then delegate the collectible grant
    to its typed port.
- **Kind:** `collectible`
  - **Required behavior:** Commit a one-time or respawnable pickup according to
    the definition.
- **Kind:** `repair_pickup`
  - **Required behavior:** Repair the active vehicle context and schedule the
    authored respawn.
- **Kind:** `nitro_pickup`
  - **Required behavior:** Delegate the charge grant to the vehicle capability
    port.
- **Kind:** `teleport`
  - **Required behavior:** Reserve both ends, validate the destination,
    transition atomically, and recover to the source on failure.
- **Kind:** `purchase_vehicle`
  - **Required behavior:** Quote the canonical offer, debit currency, grant
    ownership, and persist one atomic result.
- **Kind:** `purchase_costume`
  - **Required behavior:** Quote the canonical offer, debit currency, grant the
    costume, and persist one atomic result.
- **Kind:** `generic_event`
  - **Required behavior:** Publish only a schema-registered event payload with a
    declared consumer.

<!-- markdownlint-enable MD013 -->

No generic-event definition may substitute for a kind that has domain effects.

## Candidate discovery

Each interactor maintains a bounded overlap set from interaction-source
collision channels. Registration, participant identity, occupancy, enter/exit,
and streaming teardown follow the
<!-- markdownlint-disable-next-line MD013 -->
[authored spatial placement and trigger
runtime](authored-spatial-placement-and-trigger-runtime.md).
There is no world-wide per-frame actor scan.

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

Physics overlap order, actor creation order, streaming order, pointer values,
and
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

Presentation preparation may lock movement and camera input only for the
declared
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
vehicle-context service for the current chapter and sandbox state. A successful
repair restores the complete driveable state and all visible damage channels
supported by the vehicle runtime.

Respawnable pickup families declare independent typed policies and durations.
Repair, temporary boost, hazard, and mod-defined pickups do not share one global
interval or runtime type switch. Cooldown, streaming, checkpoint, and
restoration
behavior follows the
<!-- markdownlint-disable-next-line MD013 -->
[mission world-entity and respawn
runtime](mission-world-entity-and-respawn-runtime.md).

Alien-camera collectibles are adversarial destructible targets rather than
passive overlaps. Destruction, currency reward, level-progress credit, visual
shutdown, and mission observations commit once from the authoritative
destruction
result. Nearby repair or card pickups may publish an alert stimulus, but they do
not call camera behavior directly.

## Vehicle and interior delegation

Vehicle entry, phone-booth retrieval, and interior transitions remain
application
ports. The interaction subsystem owns only candidate selection, reservation, and
presentation handoff. It does not own seat state, vehicle spawning, world
travel,
or interior streaming.

If a delegated transaction times out or fails, the interaction result preserves
the typed downstream error. The source remains available only when its own state
and the downstream domain still permit retry.

## Purchases

A purchase interaction resolves a canonical offer from the gameplay catalog.
The displayed price, eligibility result, debit, ownership grant, and save write
use one offer revision. A changed price or ownership state invalidates the
reservation and requires a new quote.

Currency is never debited before the grant is prepared. Success is published
only
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
- A source unload cannot leave movement, camera, animation, or slot locks
  active.
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

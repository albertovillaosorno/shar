# Mission world-entity and respawn runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Contextual interaction query and transaction boundary](../../adr/unreal/runtime/contextual-interaction-query-and-transaction.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Collector cards, coins, rewards, gags, and wasps](../../adr/gameplay/collectibles/collectibles-rewards-gags-and-wasps.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission definition, stage, and objective runtime](mission-definition-stage-and-objective-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Persistent world-object state runtime](persistent-world-object-state-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored spatial placement and trigger runtime](authored-spatial-placement-and-trigger-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [World render-entity and physics runtime](world-render-entity-and-physics-runtime.md)

## Purpose

This specification defines mission-owned world entities that can be activated,
collected, attached, disabled, destroyed, restored, or restored. It also
defines
mission safe zones and the recovery boundary between transient mission state,
persistent world state, and presentation.

The runtime replaces mutable global timers, pointer-owned placement, implicit
scene membership, fixed type switches, and actor callbacks that directly grant
progress or choose mission state.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| Mission session subsystem | Owns mission and stage revisions, checkpoints, and stage transitions. |
| Mission world-entity subsystem | Owns transient entity activation, respawn delay, restoration, and teardown. |
| Persistent world-object service | Owns durable collected, destroyed, unlocked, and restored state. |
| Interaction subsystem | Owns pickup and action reservations and exactly-once interaction results. |
| Vehicle context service | Owns the controlled vehicle and vehicle-attached payload bindings. |
| Spatial subsystem | Owns placement resolution, safe-zone occupancy, and bounded overlap evidence. |
| Presentation services | Project icons, effects, audio, animation, camera shake, and visibility. |

<!-- markdownlint-enable MD013 -->

A world actor is an adapter. It cannot own the canonical entity identity,
respawn policy, mission result, reward transaction, or save key.

## Runtime topology

The runtime uses:

- `FSharMissionWorldEntityId`, a stable catalog identity;
- `FSharMissionWorldEntityRevision`, one active entity revision;
- `FSharMissionWorldEntityDefinition`, immutable behavior and ownership data;
- `FSharMissionWorldEntityPlacement`, one canonical spatial binding;
- `FSharRespawnPolicy`, immutable respawn-delay and restoration policy;
- `FSharRespawnTicket`, one accepted respawn-delay transaction;
- `FSharSafeZoneDefinition`, immutable mission-zone policy;
- `FSharSafeZoneOccupancyId`, one participant occupancy record;
- `FSharVehiclePayloadBinding`, one vehicle-attached entity binding;
- `USharMissionWorldEntitySubsystem`, the world-scoped authority; and
- repository-owned actor, component, interaction, physics, and presentation
  adapters.

Every runtime observation carries world, mission, stage, entity, placement, and
request revisions. A stale actor, overlap, collision, timer, animation, or load
callback cannot mutate a replacement entity.

## Definition contract

`FSharMissionWorldEntityDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `EntityId` | Canonical entity identity. |
| `EntityKind` | Pickup, mission prop, payload, destructible, safe-zone anchor, or another registered kind. |
| `PlacementId` | Required authored placement identity. |
| `PersistencePolicy` | Transient, checkpoint, chapter, or permanent state. |
| `ActivationPredicate` | Chapter, mission, stage, feature, and progression requirements. |
| `InteractionPolicyId` | Optional passive or explicit interaction definition. |
| `RespawnPolicyId` | Optional respawn-delay or reset policy. |
| `CollisionPolicyId` | Collision, attachment, destruction, and recovery behavior. |
| `PayloadPolicyId` | Optional vehicle or actor attachment contract. |
| `RewardTransactionId` | Optional exactly-once reward or progression transaction. |
| `PresentationProfileId` | Icon, effect, audio, animation, and accessibility projection. |
| `CheckpointPolicyId` | Snapshot and reconstruction behavior. |
| `LoadPlanId` | Required asset and world-composition dependencies. |
| `TeardownPolicyId` | Release, persist, transfer, or restore behavior. |

<!-- markdownlint-enable MD013 -->

Definitions reject missing placement, invalid persistence and respawn
combinations, unregistered entity kinds, contradictory ownership, and reward
transactions without a durable key.

## Entity states

One entity revision has exactly one state:

- `unavailable`;
- `loading`;
- `ready`;
- `active`;
- `reserved`;
- `consumed`;
- `attached`;
- `disabled`;
- `destroyed`;
- `waiting_to_respawn`;
- `restoring`;
- `released`; or
- `failed`.

`active` means the entity can participate in gameplay. `reserved` means one
accepted interaction owns it. `consumed` means the current activation produced
its accepted result. `attached` means it is owned by a declared vehicle or actor
binding. `waiting_to_respawn` uses authoritative domain time, not actor
visibility.

`released` is terminal for the current revision. A later respawn creates a new
revision with the same canonical identity.

## Activation transaction

Activation follows this sequence:

1. resolve the definition, placement, and owning gameplay state;
1. evaluate activation and persistence predicates;
1. resolve required bundles and world composition;
1. create one entity revision;
1. construct the actor or component projection;
1. verify transform, collision, interaction, and presentation bindings;
1. publish the active snapshot; and
1. release temporary load handles not required for residency.

Partial activation rolls back every actor, interaction source, collision shape,
presentation request, and load handle. It cannot leave a visible but inactive
entity or an active entity without its canonical placement.

## Respawn policy

`FSharRespawnPolicy` contains:

- policy identity;
- eligible entity kinds;
- duration in simulation ticks;
- start boundary;
- suspension policy;
- reset and checkpoint behavior;
- world-streaming behavior;
- visibility and collision restoration policy;
- retry and failure policy; and
- optional accessibility presentation.

Respawn timing starts only after the consume or destruction transaction commits.
It uses mission or world simulation time according to policy. Rendering rate,
wall-clock time, actor hidden state, and presentation completion are not respawn
time.

The initial policy families are:

- `never`;
- `waiting_to_respawn`;
- `mission_restart`;
- `checkpoint_restore`;
- `chapter_reload`;
- `stream_reload`; and
- `explicit_reset`.

A respawn ticket records the entity identity, consumed revision, start tick,
duration, suspension policy, and resulting activation revision. Duplicate
collection cannot start duplicate tickets.

## Collection and pickup transaction

A pickup request contains:

- participant and vehicle context;
- entity and placement identities;
- active entity revision;
- interaction reservation identity;
- mission and stage revisions;
- requested gameplay effect;
- reward and progression transaction identities; and
- observation identity for deduplication.

The transaction:

1. validates availability, reservation, participant, and context;
1. prepares the gameplay effect and any durable transaction;
1. commits the effect and durable state atomically;
1. marks the entity consumed;
1. releases collision and interaction eligibility;
1. starts the declared respawn ticket when applicable; and
1. publishes presentation after the authoritative result.

A repair pickup resolves the exact controlled or retained player-vehicle
context.
It cannot repair an arbitrary nearby vehicle. A boost or temporary resource uses
its own registered effect rather than sharing a generic respawn type switch.

## Respawn restoration

When a respawn delay becomes eligible, restoration:

1. revalidates chapter, mission, stage, feature, and world predicates;
1. verifies that no permanent state suppresses the entity;
1. resolves the current placement and required assets;
1. creates a new entity revision;
1. restores authored transform, collision, interaction, and presentation;
1. verifies the active projection; and
1. closes the respawn ticket.

Streaming out during a respawn delay does not lose the ticket. Streaming in
does not shorten it. A permanent collection or destruction record suppresses
restoration regardless of an older ticket.

## Safe-zone definition

`FSharSafeZoneDefinition` contains:

- canonical zone and placement identities;
- shape and dimensions;
- participant filters;
- active mission and stage range;
- occupancy aggregation policy;
- chase, traffic, AI, damage, and control effects;
- entry and exit hysteresis;
- checkpoint and recovery behavior;
- presentation policy; and
- teardown behavior.

Safe-zone occupancy follows the authored spatial trigger contract. The first
eligible entry acquires the declared suppression handles. Additional occupants
do not replay the start result. The final eligible exit releases those handles.

A safe zone never disables a global system directly. It acquires scoped handles
from chase, traffic, AI, damage, or input services. Teardown releases the exact
handles even when the zone unloads while occupied.

## Vehicle-attached payloads

A vehicle payload binding contains:

- payload and vehicle-role identities;
- attachment point or transform policy;
- collision and simulation policy while attached;
- detachment, collection, delivery, and destruction observations;
- vehicle replacement and wreck behavior;
- checkpoint serialization; and
- ownership and teardown policy.

Attachment is an accepted transaction. A payload cannot be attached to two
vehicles, remain active at its original placement, or retain an old vehicle
pointer after a role replacement.

Detachment creates a new world-entity revision at a validated transform. The
vehicle and payload services agree on the transfer before either projection
changes.

## Mission props and destructible entities

Mission props may expose collision, animation, state transitions, damage,
collection, payload, and destruction capabilities. Their definition declares
which observations are authoritative.

A prop can transition through registered states, but an animation frame or
presentation callback cannot grant collection or destruction. Terminal damage,
accepted interaction, and explicit transformation are distinct results.

Destruction commits once by entity revision. Visual explosion, debris, audio,
camera shake, and HUD removal follow the result. World unload, actor removal,
and render visibility are not destruction.

Actor/component composition, Chaos bodies, cooked query surfaces, collision
profiles, sleep and wake, force requests, and breakable replacement follow
<!-- markdownlint-disable-next-line MD013 -->
[World render-entity and physics runtime](world-render-entity-and-physics-runtime.md).
Physics and collision callbacks provide revisioned evidence; they cannot commit
the mission result or persistent destruction themselves.

## Collision recovery

A movable mission entity records its last accepted transform and authored
recovery transform. Collision recovery may:

- stop unsafe linear and angular velocity;
- restore a previous accepted transform;
- restore the authored transform;
- detach according to payload policy;
- enter typed recovery; or
- fail the owning objective.

Recovery is driven by bounded collision and world-query evidence. It cannot
teleport an entity because a frame missed collision or because presentation is
not visible.

## Camera shake and other feedback

Impact and destruction may submit a camera-shake request containing source,
participant, distance, direction, magnitude, attenuation, and stage revision.
The camera subsystem validates and arbitrates the request.

Camera shake, HUD icons, collection effects, explosions, and audio are
non-authoritative. Their failure is reported as presentation degradation unless
the definition explicitly requires a presentation result for accessibility or
interaction clarity.

## Checkpoint integration

A checkpoint stores, when applicable:

- active entity revisions;
- consumed and destroyed transient results;
- open respawn tickets;
- vehicle payload bindings;
- safe-zone occupancy and acquired handles;
- accepted transforms and recovery transforms;
- uncommitted mission effects; and
- persistent transaction revisions already committed.

Restore creates new runtime revisions from the checkpoint. It never rewinds a
permanent collection, reward, purchase, or destruction transaction that already
committed durably.

## Streaming and teardown

World Partition and Runtime Data Layer changes may remove projections while
retaining domain state. Streaming removal:

- cancels uncommitted interactions;
- releases collision and presentation adapters;
- preserves eligible respawn-delay and checkpoint state;
- releases safe-zone handles;
- detaches or transfers payloads according to policy; and
- rejects late callbacks by revision.

Mission transition, abort, completion, feature removal, and world teardown must
leave no mission-owned actor, interaction source, collision shape, safe-zone
handle, respawn timer, payload binding, or presentation request.

## Mod overlays

A validated package may add namespaced entity, respawn, safe-zone, payload, and
presentation definitions. It must declare dependencies, conflicts, persistence,
resource limits, and teardown behavior.

An overlay cannot reinterpret a durable base-game identity, shorten another
package's active respawn delay, attach an entity it does not own, or weaken
exactly-once
reward and destruction semantics.

## Diagnostics

Development diagnostics expose immutable snapshots of:

- entity, placement, mission, stage, and world revisions;
- current entity state;
- interaction reservation;
- respawn ticket and remaining simulation ticks;
- payload binding;
- safe-zone occupancy and acquired handles;
- accepted and recovery transforms;
- persistent transaction revision;
- active projection handles; and
- last rejection, recovery, or teardown finding.

Diagnostics may request a validated reset in a test world. They cannot edit
production timers, mark an entity collected, or grant a reward.

## Failure behavior

The runtime fails closed on:

- missing or duplicate entity identity;
- invalid placement or world composition;
- incompatible persistence and respawn policies;
- stale mission, stage, entity, placement, or request revision;
- duplicate collection, destruction, attachment, or respawn;
- missing participant or vehicle context;
- payload attachment to multiple owners;
- safe-zone teardown with unreleased handles;
- actor unload presented as consumption or destruction;
- partial activation, restore, checkpoint, or teardown;
- permanent state contradicted by a transient ticket; or
- presentation attempting an authoritative result.

Failure preserves the last accepted world and progression state and returns
typed
evidence. It never guesses a placement, clamps a timer, or silently recreates an
entity.

## Validation

Definition validation proves:

- every identity and placement is unique;
- every entity kind has a registered adapter;
- persistence and respawn policies are compatible;
- every reward transaction has a durable key;
- every safe-zone effect has an acquire and release path;
- every payload binding has transfer and teardown behavior;
- every checkpoint field has a deterministic reconstruction rule;
- every load handle and projection has an owner; and
- overlays cannot weaken base invariants.

## Tests

Required automated tests include:

- one-time collection and duplicate request rejection;
- respawn-delay timing across frame rates and pauses;
- streaming out and in during a respawn delay;
- permanent collection suppressing stale respawn tickets;
- repair pickup vehicle-context selection;
- safe-zone first-entry and final-exit aggregation;
- safe-zone unload while occupied;
- payload attachment, transfer, detachment, and vehicle replacement;
- prop destruction versus unload and presentation completion;
- collision recovery to accepted and authored transforms;
- checkpoint restore with active respawn tickets and payloads;
- mission abort and completion teardown;
- late callback rejection after replacement; and
- mod feature removal with zero leaked handles.

## Invariants

- One canonical entity has at most one active runtime revision per owning scope.
- One interaction reservation can commit one entity result.
- One consumed or destroyed revision starts at most one respawn ticket.
- Durable collection and destruction override transient respawn-delay state.
- One payload has at most one authoritative owner binding.
- Safe-zone effects exist only while the accepted occupancy policy requires
  them.
- Actor visibility and world streaming never define gameplay state.
- Presentation cannot grant collection, destruction, reward, or mission
  progress.
- Every activation, restoration, transfer, and teardown is revision-correlated.

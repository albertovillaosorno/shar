# Persistent world-object state runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native World Partition and Data Layers](../../adr/pipeline/unreal/world-partition-and-data-layer-import.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Collector cards, coins, rewards, gags, and wasps](../../adr/gameplay/collectibles/collectibles-rewards-gags-and-wasps.md)

## Purpose

This specification defines persistent state for destructible, removable,
consumable, and variant world placements across streaming, level transitions,
save and load, restart, and catalog migration.

The design replaces sector hashes, load-order counters, packed positional object
numbers, and mutable global bit arrays with stable placement identities and
idempotent state transactions.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| World-placement catalog | Stable world, layer, region, placement, and state-schema identities. |
| Persistent-world subsystem | Read models, mutation validation, streaming projection, and lifecycle. |
| Progression repository | Durable accepted world-state and reward transactions. |
| World actors and components | Present the accepted state and request typed mutations. |
| Save service | Serialize, migrate, validate, and commit portable world-state records. |
| Import pipeline | Convert source regions and object evidence into stable native placements. |

<!-- markdownlint-enable MD013 -->

A streamed actor never owns durable state. It projects one accepted record and
submits typed mutation requests to the persistent-world subsystem.

## Runtime topology

The runtime module owns these C++ types:

<!-- markdownlint-disable MD013 -->

| Type | Responsibility |
| :--- | :--- |
| `USharPersistentWorldDefinition` | Immutable state schema, reset, respawn, and migration policy. |
| `USharPersistentPlacementDefinition` | Stable world, layer, region, placement, actor, and reward binding. |
| `USharPersistentWorldSubsystem` | World-scoped projection, request validation, and streaming integration. |
| `ISharPersistentWorldRepository` | Portable accepted-state and revision port. |
| `FSharPersistentObjectState` | Canonical placement, state value, revision, and transaction evidence. |
| `FSharPersistentMutationRequest` | Placement, expected revision, transition, cause, and authority. |
| `FSharPersistentMutationResult` | Closed success or failure result with accepted state. |

<!-- markdownlint-enable MD013 -->

The repository stores no actor pointers, streaming-cell handles, package paths,
source ordinals, or load-order positions.

## Stable placement identity

Every persistent object has one canonical `PlacementId`. Its definition binds:

- base-world identity;
- level and Runtime Data Layer composition;
- logical region identity;
- placement identity within that region;
- expected actor or component class;
- persistent state schema;
- initial state;
- reset and respawn policy;
- optional progression or reward transaction definition; and
- definition revision.

Identity is generated from reviewed placement data. Actor discovery order,
package load order, streaming order, array index, truncated hash, or display
name
cannot select durable state.

Two definitions with the same canonical placement identity fail catalog
activation. One source alias may resolve to only one placement identity.

## Region and sector conversion

Source sector or zone values remain import provenance. Conversion binds each
recognized source region to one canonical base-world and logical region
identity.
The generated mapping records:

- exact source region value and optional name;
- canonical world and region identities;
- expected Runtime Data Layer composition;
- placement census and mapping revision; and
- evidence used to establish the mapping.

Runtime streaming does not search a table of partial region hashes. Unknown,
ambiguous, or colliding source region values remain conversion findings.

A level-global persistent placement uses an explicit level-global region
identity.
It is not appended to a synthetic final sector or assigned from current load
order.

## State schemas

Each definition selects one closed state schema. Initial schemas include:

<!-- markdownlint-disable MD013 -->

| Schema | States |
| :--- | :--- |
| `destructible` | `intact`, `damaged`, `destroyed` |
| `removable` | `present`, `removed` |
| `consumable` | `available`, `consumed` |
| `variant` | Definition-owned closed variant identities. |
| `staged_destructible` | Ordered authored damage or payout stages plus `destroyed`. |

<!-- markdownlint-enable MD013 -->

A state schema declares legal transitions. Arbitrary integers, cleared bits, and
actor visibility are not durable state authority.

Presentation may have additional transient animation, debris, particle, sound,
or physics state. Those values are rebuilt from the accepted persistent state
and
are not serialized unless a separate schema explicitly requires them.

## Mutation transaction

A world-state mutation follows this sequence:

1. resolve the canonical placement and active definition revision;
1. verify the requesting world, layer composition, actor, and authority;
1. read the accepted state and revision;
1. validate the requested transition and expected revision;
1. prepare any linked reward or progression transaction;
1. atomically commit the new persistent state and linked durable effect;
1. publish the accepted result; and
1. project the result to loaded actors and observers.

Repeated delivery of the same transaction identity is idempotent. Two concurrent
requests with the same expected revision cannot both commit incompatible states.

A disappearance, streaming unload, actor destruction during teardown, or missing
presentation asset cannot be interpreted as a persistent mutation.

## Destruction and removal

Damage and collision systems may submit a destruction request only after the
owning gameplay system verifies its terminal condition. The accepted state then
controls later actor projection.

When a destroyed or removed placement streams in again, the subsystem may:

- suppress spawning;
- spawn the actor in its terminal presentation state;
- replace it with an authored remnant actor; or
- apply another definition-owned projection.

The projection policy cannot grant rewards again. A reset or respawn requires an
explicit definition and accepted reset transaction.

## Persistent rewards

A placement may reference one typed reward transaction. State and reward commit
atomically when the definition requires both.

The transaction records:

- placement identity;
- persistent mutation identity;
- reward definition identity;
- accepted catalog revision; and
- resulting progression revision.

Reloading, revisiting, streaming, duplicated collision, or replacing the actor
cannot pay the same persistent source twice. Presentation-only destructibles do
not acquire reward behavior by convention.

## Streaming lifecycle

When a World Partition cell or Runtime Data Layer becomes ready, the subsystem:

1. receives the set of persistent placement identities in that composition;
1. validates definition revisions;
1. reads one immutable accepted-state snapshot;
1. applies projection before ordinary interaction becomes available;
1. registers weak runtime bindings for later accepted updates; and
1. releases those bindings when the actors or composition unload.

Streaming order cannot allocate identity. A late state read for an unloaded
actor
may update the repository but cannot mutate a replacement actor without matching
world, placement, and binding revisions.

## Chapter unlocks, gameplay-state transitions, and restart

A chapter unlock or transition between `mission` and `non_mission` reads
persistent state after the catalog and save revisions are validated but before
new interactions are enabled. Chapter progression adds state cumulatively and
cannot unload or reset earlier portable placements.

Mission restart follows each definition's reset policy:

- `portable` retains the accepted state;
- `session` resets at session recreation;
- `mission` resets through the mission restart transaction; or
- `authored` applies a declared predicate and transition.

A world reload, chapter unlock, or mission-state transition cannot reset
portable world state merely because actors are recreated.

## Save representation

Portable save data stores a deterministic map from canonical placement identity
to non-default accepted state and revision evidence. Default state may be
omitted
only when schema and catalog revisions make reconstruction unambiguous.

A compact generated representation may use ordinals or bitsets inside one exact
save-schema revision. Deserialization converts them to canonical identities
before domain use. Reordering a placement catalog cannot reinterpret an existing
bit.

The save record includes enough mapping revision evidence to migrate changed
placements without guessing from load order or source region numbers.

## Migration

Migration uses explicit redirects and state converters. It may:

- redirect a placement identity;
- split one old state into deterministic new states;
- merge duplicate old records under a declared conflict rule;
- preserve state for temporarily unavailable optional content; or
- reject an incompatible mapping while preserving the old save.

Deleting a definition cannot silently delete accepted portable state. Mod
overlays
must namespace new placements and declare behavior when the providing mod is
absent.

## Multiplayer and authority

One authoritative world-state service accepts mutations for the active session.
Local split-screen players observe the same world placement state.

A local player's interaction identity may be recorded as cause evidence, but it
does not create a separate copy of a shared physical object's persistent state.
The base campaign remains local and single-player. A validated community server
mod may replace the session authority through the declared server-adapter port,
but it must namespace server persistence and cannot reinterpret a base save. The
adapter contract follows the
<!-- markdownlint-disable-next-line MD013 -->
[multiplayer adapter and community-server extension](../modding/multiplayer-adapter-and-community-server-extension.md).

## Failure behavior

The subsystem fails closed on:

- unknown or duplicate placement identity;
- ambiguous source-to-native mapping;
- stale definition or state revision;
- illegal state transition;
- request from the wrong world or layer composition;
- actor class or placement mismatch;
- reward transaction that cannot commit atomically;
- incompatible save-schema mapping;
- late callback targeting an unloaded or replaced actor; or
- an attempt to infer persistence from actor disappearance or load order.

Failure leaves the accepted state and durable progression unchanged.

## Verification

Automated tests cover:

- stable identity under reordered actor and cell loading;
- sector-alias collision and unknown-region rejection;
- destruction, removal, consumption, and variant transitions;
- duplicate and competing mutation requests;
- reward and state atomicity;
- stream out, stream in, world reload, chapter unlock, mission restart, save,
  and load;
- default-state omission and bitset migration;
- missing optional mod content;
- late asynchronous results after actor replacement; and
- equivalent logical state on every supported architecture and platform.

Runtime assertions verify that a loaded persistent actor has exactly one
matching
placement definition and accepted state projection.

## Invariants

- Persistent identity never depends on load order.
- One accepted transaction changes one placement revision at most once.
- A persistent reward source pays at most once for its declared reset lifetime.
- Streaming cannot create, clear, or reorder durable state.
- Save state uses canonical identities before domain evaluation.
- Presentation failure cannot fabricate or erase persistent state.
- Unknown mapping evidence is reported rather than guessed.

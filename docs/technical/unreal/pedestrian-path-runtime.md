# Pedestrian path runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Mass Entity ambient population](../../adr/unreal/runtime/mass-entity-ambient-population.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Ambient population and named-character runtime](ambient-population-and-named-character-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored spatial placement and trigger runtime](authored-spatial-placement-and-trigger-runtime.md)

## Purpose

This specification defines deterministic pedestrian path data, graph assembly,
occupancy, streaming, and Mass Entity integration. It replaces fixed global path
arrays, allocation-order identity, hard-coded path and pedestrian limits, and
runtime path objects that double as world-scene entities.

Pedestrian paths are immutable navigation definitions. They guide disposable
ambient entities and promoted named-character movement without owning character
identity, mission state, or presentation.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| Content compiler | Converts validated path evidence into canonical path graph definitions. |
| Pedestrian path registry | Owns path, node, segment, lane, and connectivity identities. |
| Ambient population subsystem | Selects eligible paths and creates Mass movement assignments. |
| ZoneGraph adapter | Projects eligible path topology into native traversal data. |
| Named-character runtime | Reserves path use for promoted actor representations when required. |
| World Partition adapters | Activate and release path projections with owning cells and Data Layers. |
| Presentation and debug adapters | Render optional path, bounds, occupancy, and failure diagnostics. |

<!-- markdownlint-enable MD013 -->

A Mass entity, actor, spline component, ZoneGraph lane, or debug primitive is an
adapter. None is the canonical path identity.

## Runtime topology

The runtime uses:

- `FSharPedestrianPathId`, a stable path identity;
- `FSharPedestrianPathNodeId`, a stable node identity;
- `FSharPedestrianPathSegmentId`, a stable directed segment identity;
- `FSharPedestrianPathDefinition`, immutable path topology;
- `FSharPedestrianPathSegmentRow`, immutable segment geometry and policy;
- `FSharPedestrianPathReservationId`, one accepted capacity reservation;
- `FSharPedestrianPathAssignmentId`, one entity-to-path assignment;
- `USharPedestrianPathRegistry`, the world-scoped validated registry;
- Mass fragments and processors for movement and occupancy; and
- ZoneGraph, World Partition, navigation, and diagnostic adapters.

Every assignment carries world, chapter, path, segment, population, and entity
revisions. Streaming, LOD, representation change, or entity promotion cannot
silently transfer an assignment to another path.

## Path definition

`FSharPedestrianPathDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `PathId` | Canonical path identity. |
| `WorldRegionId` | Owning geographic region. |
| `SegmentRows` | Ordered directed path segments. |
| `ClosedLoop` | Whether the final node connects to the first node. |
| `TraversalPolicyId` | Direction, reversal, loop, endpoint, and reroute behavior. |
| `CapacityPolicyId` | Maximum accepted occupancy and reservation rules. |
| `PopulationTags` | Eligible ambient archetypes and population groups. |
| `ChapterPredicate` | Chapter and cumulative-world availability. |
| `GameplayStatePredicate` | Mission and non-mission availability. |
| `WorldLayerSetId` | Required cell and Runtime Data Layer composition. |
| `AvoidanceProfileId` | Native crowd and local-avoidance policy. |
| `SmartObjectBindings` | Optional stops, conversations, benches, doors, and activities. |
| `FailurePolicyId` | Reroute, wait, release, promote, or typed failure behavior. |

<!-- markdownlint-enable MD013 -->

A path contains at least one valid segment. Closed loops require compatible end
and start nodes. Open paths declare endpoint behavior explicitly.

## Segment row

`FSharPedestrianPathSegmentRow` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `SegmentId` | Stable path-scoped segment identity. |
| `SequenceOrdinal` | Dense zero-based order within the path. |
| `StartNodeId` | Required start node. |
| `EndNodeId` | Required end node. |
| `StartPosition` | Authored world position after deterministic conversion. |
| `EndPosition` | Authored world position after deterministic conversion. |
| `Length` | Derived validated segment length. |
| `Bounds` | Derived conservative bounds for queries and streaming. |
| `TraversalWidth` | Valid movement width. |
| `DirectionPolicy` | Forward, reverse, or bidirectional traversal. |
| `SpeedProfileId` | Walking and contextual speed policy. |
| `ConnectionIds` | Explicit adjacent segment identities. |
| `HazardAndCrossingPolicyId` | Traffic, road crossing, threat, and wait behavior. |

<!-- markdownlint-enable MD013 -->

Segment length, center, bounds, and direction are derived once by the compiler.
Runtime code does not recompute identity or topology from pointer order.

## Graph assembly

The compiler:

1. resolves path and node identities;
1. validates every segment endpoint and ordinal;
1. derives length, bounds, direction, and connectivity;
1. verifies closed-loop or open-end topology;
1. rejects zero-length and non-finite geometry;
1. validates capacity and population policy;
1. resolves world, chapter, and gameplay-state predicates;
1. emits deterministic ZoneGraph projection data; and
1. serializes rows in canonical identity order.

Duplicate segments, gaps, self-intersections that violate policy, missing nodes,
and disconnected topology fail conversion. The runtime never allocates a free
path from a global index.

## Native Unreal projection

Eligible path definitions project into ZoneGraph and Mass movement data after
the
owning world cells and Runtime Data Layers are ready. Projection preserves
stable
path and segment metadata so native lane handles can be correlated back to the
canonical definition.

Repository-owned Mass traits provide:

- path identity and revision;
- current segment and progress;
- traversal direction;
- reservation identity;
- endpoint and loop policy;
- avoidance and speed profile;
- mission and interaction overrides; and
- release and recovery state.

Native lane handles are transient. A rebuilt ZoneGraph projection creates new
adapter handles without changing path identity or save-relevant state.

## Capacity and reservations

Capacity is a policy, not a compile-time constant. A path or segment reservation
contains:

- path and segment identities;
- entity and population assignment identities;
- requested direction and time window;
- mission or interaction priority;
- accepted capacity revision; and
- release reason.

Reservations are deterministic for equivalent candidate sets. They sort by
priority, request tick, and stable identity. A path at capacity returns a typed
result so the population planner can wait, choose another eligible path, or skip
one disposable spawn.

Representation LOD does not release occupancy. Promotion from a Mass entity to a
named actor transfers the same reservation. Destruction, streaming removal,
mission takeover, or feature removal releases it exactly once.

## Assignment lifecycle

A pedestrian path assignment has these states:

- `requested`;
- `reserved`;
- `waiting_for_projection`;
- `active`;
- `paused`;
- `rerouting`;
- `completed`;
- `cancelled`;
- `failed`; or
- `released`.

Activation requires a valid path revision, a live ZoneGraph projection, an
accepted reservation, and an eligible entity. Movement starts only after all
four
conditions hold.

At an open endpoint, the traversal policy chooses completion, reversal, wait,
Smart Object handoff, or deterministic reroute. At a closed loop, the assignment
advances from the final segment to the first without changing path identity.

## Movement and progress

Authoritative path progress is represented by path identity, segment identity,
normalized segment progress, traversal direction, and simulation tick.

Movement processors consume that progress and native lane data. They cannot
infer
canonical progress from actor position alone. Position is an observation used to
validate or recover the assignment.

A segment transition is accepted once and only when:

- the active segment and direction match;
- the endpoint crossing is valid;
- the declared next segment resolves;
- the next projection is ready; and
- the assignment revision remains current.

A skipped, repeated, or stale transition is rejected without advancing the path.

## Traffic crossings and hazards

A path segment may declare a road crossing, vehicle-threat zone, or authored
wait
point. Crossing policy consumes typed traffic and hazard observations and may
request:

- continue;
- slow;
- wait;
- evade;
- reroute;
- promote to an actor representation; or
- release a disposable assignment.

Vehicle proximity, horns, accepted impacts, violence, and mission hazards remain
owned by their respective services. The path runtime supplies traversal context
and never grants damage, animation, dialogue, or mission results.

## Smart Object integration

A path may bind nodes or segments to Smart Objects such as benches,
conversations,
shop windows, doors, or idle activities. Arrival creates an interaction
candidate
or reservation request. The path assignment pauses only after the Smart Object
reservation is accepted.

Completion, cancellation, timeout, or feature removal releases the Smart Object
slot and resumes, reroutes, or completes the path according to policy. A failed
activity cannot strand path capacity.

## Named-character promotion

A disposable Mass entity may be promoted to a repository-owned actor when it
becomes talkable, mission-relevant, cinematic, a driver, or otherwise identity
bearing. Promotion transfers:

- canonical character and placement identity;
- path assignment and reservation;
- current segment, progress, and direction;
- movement and avoidance context;
- active Smart Object reservation;
- world and mission revisions; and
- teardown ownership.

Promotion is transactional. The Mass representation is not released until the
actor accepts the transferred state. Demotion follows the same rule and is
forbidden while mission, interaction, dialogue, or save-relevant state requires
an actor.

## Streaming

A path projection activates only when its owning cells and Data Layers are
ready.
Streaming out:

- pauses or reroutes assignments before projection removal;
- releases native lane handles;
- retains canonical path identity and eligible reservations;
- preserves promoted named actors according to their world policy; and
- rejects late movement callbacks by revision.

Streaming in reconstructs adapter handles from the immutable definition. It does
not allocate a new path identity or reset occupancy implicitly.

## Platform and quality policy

Quality settings may reduce disposable population count, representation detail,
update frequency, and debug projection. They cannot:

- change path topology;
- remove required named-character routes;
- alter mission or interaction reachability;
- change reservation ordering;
- skip authoritative segment transitions; or
- produce different terminal assignment results.

Android Low uses the same graph and assignment contract with bounded population
and presentation budgets.

## Mod overlays

A validated package may add namespaced paths, segments, population tags, Smart
Object bindings, and traversal policies. It must declare world ownership,
dependencies, conflicts, and teardown.

An overlay cannot mutate another package's segment identities in place, exceed
validated geometry or capacity limits, or create unresolved cross-package graph
edges. Replacing a path creates a new definition revision and migrates or
cancels
active assignments according to policy.

## Diagnostics

Development diagnostics expose immutable snapshots of:

- path, node, segment, world, and projection revisions;
- topology and derived bounds;
- active assignments and reservations;
- current segment progress and direction;
- capacity and waiting candidates;
- ZoneGraph adapter handles;
- Smart Object handoffs;
- streaming and promotion state; and
- last rejection, reroute, or recovery finding.

Debug path rendering is optional presentation. It cannot create, modify, or
reserve topology.

## Failure behavior

The runtime fails closed on:

- duplicate or missing path, node, or segment identity;
- zero-length, non-finite, or invalid segment geometry;
- ordinal gaps or unresolved connections;
- invalid open or closed topology;
- incompatible world and chapter predicates;
- capacity underflow or duplicate reservation;
- stale assignment, projection, entity, or segment revision;
- transition to an undeclared segment;
- streaming removal with unreleased native handles;
- promotion or demotion that loses identity or reservation; or
- overlays that create unresolved or cyclic ownership.

Failure preserves the last accepted assignment or releases it according to the
registered policy. It never advances to a guessed segment or returns arbitrary
position data.

## Validation

Definition validation proves:

- stable unique identities;
- dense segment ordinals;
- finite geometry and conservative bounds;
- valid open or closed topology;
- resolved world, chapter, population, and Smart Object references;
- compatible capacity and traversal policies;
- deterministic ZoneGraph projection;
- complete streaming and teardown paths; and
- overlay isolation.

## Tests

Required automated tests include:

- deterministic graph conversion and serialization;
- open and closed path traversal;
- forward, reverse, and bidirectional assignments;
- endpoint completion, reversal, wait, and reroute;
- capacity acceptance and rejection ordering;
- duplicate reservation and release rejection;
- segment transition deduplication;
- streaming out and in with active assignments;
- Smart Object pause and resume;
- Mass-to-actor promotion and actor-to-Mass demotion;
- traffic crossing and hazard response;
- invalid geometry and disconnected topology rejection;
- mod replacement migration and cancellation; and
- identical assignment results across frame rates and quality presets.

## Invariants

- Path identity is independent of allocation order and native lane handles.
- Every segment belongs to one canonical path and has one dense ordinal.
- Every active assignment owns one accepted reservation.
- Capacity never depends on representation LOD or actor visibility.
- Segment progress advances only through a validated transition.
- Streaming changes adapter handles, not canonical path state.
- Promotion and demotion preserve identity, progress, and reservation.
- Disposable population scaling cannot remove gameplay-relevant traversal.

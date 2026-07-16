# State-driven missions, interactions, interiors, and notoriety

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Runtime mission execution and open-world interaction state

## Context

The gameplay catalog defines stable identities for missions, steps, characters,
vehicles, locations, and rewards. Runtime execution also needs deterministic
rules for compound objectives, interactable gags, indoor transitions, and the
crime-response system. These behaviors cross world streaming, vehicle state,
progression, user interface, artificial intelligence, and save boundaries.

A mission cannot be an opaque Blueprint graph, and an interactable actor cannot
become the authority for progression merely because it is placed in a map.
Likewise, an engine state machine, Smart Object, or Data Layer is a native
execution mechanism rather than a second source of domain identity.

The runtime must preserve intentional observable behavior while rejecting
accidental crashes, out-of-bounds states, duplicate actors, collision leaks,
and other historical defects as parity requirements.

## Decision

C++ domain and application services own mission, interaction, interior, and
notoriety state. Generated catalog rows provide immutable definitions. World
actors, StateTree assets, Smart Objects, Runtime Data Layers, user interface,
and artificial-intelligence controllers are adapters that observe or execute
those definitions.

Mission execution uses one C++ StateTree schema and a bounded library of native
C++ tasks, evaluators, and conditions. Mission rows select objective policies
and bind parameters. A mission does not own a hand-authored StateTree containing
unique progression logic, and Blueprint cannot define completion, failure,
recovery, reward, or save semantics.

Mission briefing, loading, result, statistics, and replay screens consume
immutable mission and progression projections. Failure categories and hint sets
are typed catalog data, and hint selection is deterministic from mission,
failure, attempt, and content revisions. User-interface animation or widget
state cannot start, complete, fail, retry, abort, skip, or replay a mission.

The base campaign skip policy becomes eligible after seven accepted failed
attempts for a skippable mission, while terminal missions in the final campaign
sequence remain non-skippable. Retry, abort, skip, and replay are explicit
application transactions with idempotency, loading, rollback, progression, and
save behavior; a screen never reloads a mission directly.

Smart Objects represent reservable interaction anchors for gags, conversations,
costume stations, interior portals, and other authored activities. Smart Object
definitions expose slots, eligibility tags, and presentation data. The
interactor service owns validation, execution, cancellation, exactly-once
completion, and progression effects.

World render entities use validated native Actor and component composition.
Primitive-component hits, Chaos contacts, sleep or wake, animation, visibility,
and renderer callbacks are revisioned observations only. They may propose
impact,
damage, interaction, recovery, or breakage work, but mission and application
services commit destruction, collection, reward, persistence, respawn, and
objective results through typed transactions.

A body sleeping, a primitive being culled, an Actor unloading, or a break
animation playing cannot complete or fail a mission. Stateful props project only
an accepted application-state revision; animation markers, collision enablement,
visibility, and local state enums cannot commit the transition themselves.

Native scene queries, closest-road or path lookups, terrain classification, and
line-of-sight checks are immutable evidence. They may inform a mission or
interaction decision but cannot activate objectives, move entities, or mutate
progression directly. Stale physics, collision, query, render, or teardown
callbacks cannot mutate a replacement entity revision.

The active world uses World Partition and Runtime Data Layers for level
variants,
mission overlays, interiors, and progression-driven presentation. Data Layers
never define canonical identity or save keys. Indoor transitions keep the same
world authority alive, preserve the selected vehicle snapshot, activate the
interior composition, apply indoor movement and combat restrictions, and restore
the exterior composition on exit.

Notoriety is a deterministic world subsystem driven by typed gameplay events and
a level policy. Every event declares an integer delta, cooldown behavior, and
whether the current objective exempts it. Pursuit activation, warning state,
wave composition, arrest, fine, decay, interior behavior, and conclusion are
explicit state transitions. Mission scripts and collision callbacks cannot
mutate the meter directly.

Accidental defects are not parity targets. A historical quirk is retained only
when an accepted decision identifies an intentional player-visible contract and
a regression test proves it. Otherwise the runtime fails safely, restores a
valid state, or rejects invalid content.

## Consequences

- Mission meaning remains data-driven and independently testable without a live
  world.
- StateTree supplies native hierarchical execution and recovery without owning
  mission identity or progression.
- Smart Objects provide native spatial queries and reservations without owning
  interaction results.
- Reused world geometry can expose different level-specific missions, gags,
  traffic, and progression through Runtime Data Layers.
- Entering an interior does not discard the active mission, vehicle damage, or
  progression transaction state.
- Objective-target collisions can be exempt from notoriety only through an
  explicit objective policy.
- Gag completion, rewards, and level completion remain exactly-once save events.
- Out-of-bounds, invalid streaming, duplicate interaction, and stale-target
  states have deterministic recovery paths instead of undefined behavior.
- Blueprint may author presentation and consume reflected read-only state, but
  it cannot become a parallel gameplay authority.

## Rejected alternatives

- One unique Blueprint or StateTree graph per mission.
- Mission completion inferred from actor destruction, unloading, or filesystem
  discovery.
- Smart Objects that directly grant rewards or mutate save data.
- Map travel for every interior when Runtime Data Layers can preserve world and
  vehicle state.
- A floating-point notoriety meter mutated from arbitrary collision callbacks.
- Hardcoded exceptions for individual mission names or target actors.
- Reproducing crashes, collision holes, duplicate actors, or out-of-bounds
  exploits as faithful behavior.

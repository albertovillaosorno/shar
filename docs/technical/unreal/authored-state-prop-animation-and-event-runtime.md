# Authored state-prop animation and event runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed StateTree action sequences](../../adr/unreal/runtime/typed-state-tree-action-sequences.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [World render-entity and physics runtime](world-render-entity-and-physics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Persistent world-object state runtime](persistent-world-object-state-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Gameplay audio source, residency, mix, and environment runtime](gameplay-audio-source-residency-mix-and-environment-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)

## Purpose

This specification defines authored stateful-prop definitions, instances,
transitions, animation ranges, visibility, collision projection, events,
callbacks, listener subscriptions, rendering updates, persistence, streaming,
networking, diagnostics, and teardown.

It replaces source-era state-prop objects that combine an animated object,
parallel arrays of state data, integer events, frame-number callbacks, fixed
listener arrays, mutable frame controllers, visibility toggles, and direct
render calls.

The target preserves authored behavior without importing source object
factories,
array positions, frame-controller pointers, callback IDs, event names, listener
slots, or per-render mutation as runtime authority.

## Native Unreal foundation

The implementation uses native Unreal facilities where applicable:

- Actor and Actor Component lifecycle;
- Skeletal Mesh, Static Mesh, Geometry Cache, Level Sequence, Animation
  Sequence,
  Anim Blueprint, Control Rig, and material parameters according to the asset;
- StateTree for authored hierarchical state logic when it is the simplest native
  representation;
- Gameplay Tags and typed event messages for semantic observations;
- animation notifies, montage events, sequence event tracks, or typed timing
  markers for presentation callbacks;
- collision profiles, body enablement, and Physics Assets;
- Asset Manager primary assets, bundles, soft references, and retained handles;
- World Partition, level instances, Game Features, and world subsystems; and
- replication or deterministic local observation according to authority policy.

StateTree is optional implementation machinery. The public contract remains a
closed typed state graph and cannot depend on editor node positions, transition
array order, or one particular Blueprint layout.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Owning domain service | Commits durable gameplay, interaction, mission, damage, reward, and persistence state. |
| State-prop definition | Declares valid states, projections, transitions, events, markers, fallbacks, and teardown. |
| State-prop runtime | Applies one accepted state revision to native components and emits typed observations. |
| Animation system | Owns native playback, evaluation, blending, notifies, root motion policy, and pose output. |
| Render and physics systems | Own visibility, materials, collision, simulation, transforms, bounds, and scene registration. |
| Audio and VFX services | Consume accepted state and marker observations through independent presentation leases. |
| Persistence service | Stores durable state identity and revision when the placement policy requires it. |
| Developer diagnostics | Observe definitions, instances, state, transitions, markers, callbacks, and failures. |

<!-- markdownlint-enable MD013 -->

A state-prop runtime may own a presentation-only autonomous cycle only when the
definition explicitly declares that no gameplay or persistent meaning depends on
its phase. Otherwise, the owning service commits every state revision.

## Runtime identities

Stable identities include:

- `FSharStatePropDefinitionId`;
- `FSharStatePropDefinitionRevision`;
- `FSharStatePropPlacementId`;
- `FSharStatePropInstanceId`;
- `FSharStatePropInstanceRevision`;
- `FSharStatePropStateId`;
- `FSharStatePropTransitionId`;
- `FSharStatePropTransitionRevision`;
- `FSharStatePropEventBindingId`;
- `FSharStatePropMarkerId`;
- `FSharStatePropCallbackId`;
- `FSharStatePropListenerLeaseId`;
- `FSharStatePropPlaybackId`;
- `FSharWorldRevision`;
- `FSharFeatureRevision`; and
- `FSharPersistenceRevision`.

Source state indexes, event integers, callback integers, object names, factory
names, drawable-element indexes, frame-controller indexes, memory addresses, and
listener-array positions are provenance only.

## Definition contract

`USharStatePropDefinition` contains:

- canonical definition identity and revision;
- compatible Actor or component class;
- native visual, animation, collision, audio, VFX, and interaction assets;
- initial-state policy;
- closed state definitions;
- closed transition definitions;
- event bindings;
- timing-marker definitions;
- listener observation schema;
- persistence and respawn policy;
- streaming and feature ownership;
- replication or local-only policy;
- accessibility and quality policy;
- fallback behavior; and
- validation and teardown requirements.

A definition is immutable while active instances reference its revision. Hot
reload in editor creates a replacement revision and migrates only through an
explicit compatibility path.

## State definition

Each `FSharStatePropStateDefinition` declares:

- stable state identity;
- optional semantic Gameplay Tags;
- visual representation and material parameters;
- visibility per named component role;
- animation or sequence binding;
- start and end markers or normalized range;
- playback rate;
- loop, finite-cycle, hold, reverse, and completion behavior;
- collision-body and query enablement;
- simulation, kinematic, and sleep policy;
- interaction availability;
- audio and VFX presentation bindings;
- automatic transition policy;
- accepted external events;
- timing markers and emitted observations;
- fallback state; and
- teardown behavior.

The definition uses named component roles and stable marker identities. It does
not use parallel visibility arrays, frame-controller indexes, raw minimum and
maximum frame numbers, or a hidden assumption that every animated object exposes
the same controller count.

## Visual and component projection

State application projects one accepted revision across all participating native
components:

- component visibility and hidden-in-game state;
- material parameters and optional material instance selection;
- skeletal, static, geometry-cache, or sequence playback;
- collision profile, body enablement, and overlap generation;
- simulated, kinematic, sleeping, or query-only policy;
- interaction prompts and targeting;
- audio and VFX leases; and
- navigation or obstruction projection when declared.

The projection is atomic from the gameplay contract's perspective. If one
required component cannot accept the target state, the transition fails or rolls
back according to policy. A half-applied state cannot remain authoritative.

## Animation range and cycle policy

Authored animation state uses stable section, notify, marker, or normalized-time
identities. Imported source frame ranges are converted once during asset
preparation and recorded as provenance.

A playback binding may declare:

- play once;
- loop continuously;
- repeat a finite number of cycles;
- hold the first or last pose;
- pause at a named marker;
- reverse from a declared marker;
- continue from accepted normalized progress; or
- remain static without an animation player.

Runtime frame rate, render cadence, and platform quality cannot alter logical
marker order, cycle count, or transition eligibility. Presentation interpolation
may vary without changing accepted state.

## Transition definition

Each `FSharStatePropTransitionDefinition` contains:

- stable transition identity;
- source and target state sets;
- semantic trigger identity;
- authority requirement;
- optional guard conditions;
- transition priority;
- interruption and supersession policy;
- animation, collision, audio, and VFX transition projection;
- completion barrier;
- persistence behavior;
- failure fallback; and
- diagnostic metadata.

Transition-array order is not priority. Priority and tie-breaking are explicit
and deterministic.

## Transition request

A transition request contains:

- instance and current-state identities;
- requested transition or target state;
- causation identity;
- requesting owner;
- world and feature revisions;
- expected instance and definition revisions;
- deterministic timestamp ordinal;
- optional typed parameters; and
- cancellation token.

The runtime validates that the request targets the current instance, the source
state is eligible, guards pass, required assets are ready, and no
higher-priority
transition already owns the instance.

## Transition lifecycle

The closed transition lifecycle is:

1. `requested`;
1. `validated`;
1. `preparing`;
1. `committing`;
1. `active`;
1. `completed`; or
1. `rejected`, `cancelled`, `superseded`, or `failed`.

A state revision is committed exactly once. Animation completion, notify order,
render visibility, audio completion, or VFX completion cannot independently
commit the transition.

## Automatic transitions

A state may request an automatic transition after:

- a named animation or sequence marker;
- a finite cycle count;
- a deterministic duration;
- a domain observation;
- an interaction result;
- a damage or breakage result; or
- another typed condition.

Automatic means definition-driven, not presentation-authoritative. The runtime
publishes a transition proposal, and the owning service commits it when the
transition has gameplay or persistent meaning.

Purely decorative state props may commit a presentation-only cycle locally when
the definition declares that policy and no external result depends on the phase.

## Event bindings

An event binding maps one typed semantic observation to one action:

- propose a transition;
- set a typed parameter;
- start, pause, resume, reverse, or stop presentation playback;
- emit a presentation cue;
- request an interaction operation;
- request damage or breakage processing; or
- ignore the event with a recorded reason.

Runtime does not compare raw event-name strings, integer event IDs, or callback
payload pointers. Unknown events are rejected or ignored according to explicit
forward-compatibility policy.

## Timing markers and callbacks

A timing marker is defined by stable identity and native timing evidence. It may
be backed by an animation notify, montage notify, sequence event, geometry-cache
marker, MetaSound trigger, or another validated source.

Marker observations contain:

- state-prop instance and playback identities;
- state and transition identities;
- marker identity;
- accepted normalized time or sequence time;
- cycle ordinal;
- world, feature, definition, and instance revisions;
- causation identity; and
- terminal or non-terminal classification.

A marker can request audio, VFX, interaction feedback, or a transition proposal.
It cannot grant rewards, apply damage, complete a mission, write persistence, or
replace the accepted state revision.

## Callback correlation

Every asynchronous callback is correlated to current instance, state, playback,
transition, world, feature, and definition revisions. A callback is accepted
only
when every required identity matches.

Late callbacks from an old state, replacement Actor, unloaded world, removed
Game
Feature, cancelled playback, or superseded transition are ignored and recorded.
Raw callback pointers, shared mutable callback objects, and untyped user data
are
prohibited.

## Listener subscriptions

Listeners subscribe through scoped `FSharStatePropListenerLease` records that
declare:

- subscriber identity;
- instance or definition filter;
- observation kinds;
- world and feature scope;
- delivery phase;
- lifetime and cancellation; and
- backpressure policy.

There is no fixed listener count. Subscription identity, not insertion order,
controls removal and delivery. A listener cannot mutate the prop during
delivery;
it may submit a new typed request after the current transaction completes.

## Rendering and update order

The state-prop runtime separates simulation, animation evaluation, component
projection, bounds synchronization, and rendering:

1. accept domain and application observations;
1. resolve and commit eligible state transitions;
1. update native animation or sequence state;
1. accept correlated markers;
1. update collision, transforms, and bounds through native prerequisites;
1. publish immutable presentation observations; and
1. let Unreal render registered components.

Repository code does not call a custom `Display` method or mutate frame
controllers immediately before drawing. Render cadence cannot become simulation
authority.

## Collision and physics

Per-state collision policy follows
<!-- markdownlint-disable-next-line MD013 -->
[World render-entity and physics runtime](world-render-entity-and-physics-runtime.md).
A state may change collision profile, body enablement, simulation mode, query
availability, or physical material only through an accepted transition.

Collision observations may propose transitions such as hit, opened, activated,
broken, or destroyed. Physics callbacks cannot change state directly or infer
rewards and persistence.

## Interaction and mission integration

Interaction and mission services reference stable state-prop placement and state
identities. They may require a state, propose a transition, wait for an accepted
state revision, or observe a marker.

An interaction transaction completes from domain evidence, not from visibility,
an animation callback, or a sound finishing. State-prop presentation remains a
projection of the accepted result.

## Audio and VFX

Audio and VFX bindings use typed source and effect definitions. State entry,
transition, marker, loop, and exit cues create bounded presentation leases.

State replacement, world unload, feature removal, transition cancellation, or
instance destruction cancels owned leases. Audio and VFX completion cannot
commit
state.

## Persistence and respawn

A placement declares one policy:

- presentation-only and reset on activation;
- world-session state;
- checkpoint-persistent;
- save-persistent;
- mission-owned; or
- externally reconstructed from another durable domain record.

Persistent records contain placement identity, canonical state identity,
accepted state revision, relevant domain result identity, and migration version.
Animation frame, raw callback state, component visibility arrays, and native
object pointers are never serialized.

## Streaming lifecycle

Activation requires:

- definition and native assets ready;
- owning world and placement revision active;
- initial or restored state validated;
- required components constructed;
- collision and interaction policy ready; and
- subscriptions established.

Deactivation freezes new requests, cancels pending transitions, tears down
listeners and presentation leases, unregisters components, releases retained
handles, and invalidates callbacks before the instance is destroyed.

## Feature and mod overlays

A validated Game Feature may add namespaced state-prop definitions, states,
transitions, markers, event bindings, and presentation assets. It cannot mutate
a
base definition in place, replace engine animation or rendering systems, or use
raw event and callback identifiers.

Feature removal cancels owned transitions and leases, unregisters definitions,
releases assets, removes owned instances, restores scoped base projections, and
rejects stale callbacks as one transaction.

## Local multiplayer and networking

Each instance has one authority policy. Local players may independently observe
or interact with the same prop, but accepted state is shared unless the
definition explicitly declares per-player presentation state.

Networked mods may replicate canonical state and transition revisions. They do
not replicate native component pointers, animation-player internals, callback
order, or raw source frame numbers.

## Accessibility and quality

Accessibility may add captions, reduced motion, stronger feedback, alternate
interaction cues, or longer presentation windows. Quality may reduce optional
particles, secondary audio, distant animation evaluation, or material cost.

Neither may change accepted state, transition order, collision, interaction
eligibility, marker semantics, persistence, or required feedback.

## Diagnostics

Read-only diagnostics expose:

- definition and instance identities;
- active state and revision;
- pending and active transitions;
- animation or sequence binding and normalized progress;
- component visibility and collision projection;
- recent events and markers;
- listener leases;
- persistence and streaming ownership;
- stale callback counts; and
- failure and fallback reasons.

Diagnostics cannot force state, advance animation, fire markers, retain assets,
or mutate listener subscriptions in shipping runtime.

## Failure behavior

Closed failure results include:

- `definition_missing`;
- `definition_invalid`;
- `initial_state_invalid`;
- `state_missing`;
- `transition_missing`;
- `transition_not_allowed`;
- `guard_rejected`;
- `asset_not_ready`;
- `component_missing`;
- `marker_missing`;
- `callback_stale`;
- `world_stale`;
- `feature_stale`;
- `instance_replaced`;
- `persistence_conflict`;
- `cancelled`;
- `superseded`; and
- `internal_failure`.

Required state props fail activation visibly and safely. Optional decorative
props
may use a declared inert fallback. Missing data never silently selects state
zero,
wraps an array index, or invokes an unverified callback.

## Validation

Validation proves:

- every definition, state, transition, event binding, marker, and component role
  has stable identity;
- initial and fallback states exist;
- transition graphs are deterministic and free of accidental dead ends;
- automatic cycles cannot produce unbounded same-frame transitions;
- required native assets and component roles exist;
- marker identities resolve to native timing evidence;
- collision and visibility projections are complete;
- persistent states have migrations;
- callback and listener lifetimes are bounded;
- Game Feature teardown is complete; and
- no source array index, raw event string, callback pointer, listener slot, or
  frame-controller pointer remains runtime authority.

## Tests

Required tests cover:

- activation in every initial-state policy;
- explicit, event-driven, marker-driven, and automatic transitions;
- finite cycles, loops, holds, reverse playback, and zero-duration rejection;
- atomic visibility, collision, and animation projection;
- cancellation and supersession;
- late marker and callback rejection;
- listener subscribe, delivery, removal, and teardown;
- persistence save, load, migration, respawn, and replacement;
- world unload during transition;
- Game Feature removal during playback;
- local multiplayer interaction contention;
- deterministic replay of accepted state revisions;
- accessibility and quality invariance; and
- headless execution without rendering, audio, or animation evaluation.

## Invariants

- Canonical state identity is never an array position.
- State transition authority is never animation completion alone.
- Rendering never mutates simulation state.
- Marker and callback acceptance always checks current revisions.
- Listener capacity is never a fixed source-era array.
- Visibility, collision, animation, and interaction projections cannot diverge
  after a committed transition.
- Presentation cannot grant rewards, apply damage, complete missions, or write
  persistence.
- World unload and feature removal leave no owned callbacks, listeners, assets,
  components, audio, or VFX leases.
- Headless gameplay can preserve semantic state without native presentation.

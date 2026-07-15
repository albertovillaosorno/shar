# Typed event and observation routing runtime

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed StateTree action sequences](../../adr/unreal/runtime/typed-state-tree-action-sequences.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Event-driven music and ambience](../../adr/unreal/runtime/event-driven-music-and-ambience.md)

## Purpose

This specification defines typed domain events, lifecycle observations,
presentation cues, subscription ownership, delivery phases, tracing, and failure
behavior for the native Unreal runtime.

It replaces one process-wide event enum, raw `void*` payloads, singleton
listener
maps, inheritance-based listeners, destructor-driven deregistration, and
behavior
that depends on listener-array order.

The event runtime is not a command bus, query service, or substitute for direct
application ports. It distributes accepted facts and observations after the
owning subsystem has validated them.

## Message kinds

Every message belongs to exactly one kind:

| Kind | Contract |
| :--- | :--- |
| Domain event | Immutable fact accepted by a domain transaction. |
| Lifecycle observation | Readiness, activation, teardown, or failure fact. |
| Presentation cue | Non-authoritative request for audiovisual presentation. |
| Diagnostic observation | Development-only trace or validation evidence. |
| External adapter observation | Normalized platform or engine callback. |

Commands request mutation and return typed results through application ports.
Queries return immutable projections. Neither is published as an event merely to
avoid naming an owner.

## Ownership

The owning domain or subsystem defines the event schema and publishes the fact.
The routing layer validates scope and delivery but never invents domain state.

The runtime module owns:

- `FSharMessageChannelId`, a stable namespaced channel identity;
- `FSharMessageSchemaId`, a stable payload-schema identity;
- `FSharMessageEnvelope`, immutable routing and correlation metadata;
- `FSharSubscriptionHandle`, move-only subscription ownership;
- `USharMessageSchemaCatalog`, generated schema and policy definitions;
- `USharWorldMessageRouterSubsystem`, world-scoped routing;
- `USharGameInstanceMessageRouterSubsystem`, game-instance routing; and
- local-player routing through the owning local-player subsystem.

Domain services may use typed native delegates when one owner and one observer
are sufficient. Cross-feature broadcasts use a schema-registered message
channel, normally through Unreal's gameplay-message facilities.

## Stable channel identity

Channels use namespaced semantic identities such as:

- `shar.vehicle.destroyed`;
- `shar.mission.step.completed`;
- `shar.progression.card.collected`;
- `shar.presentation.camera.shake`;
- `shar.lifecycle.world.ready`; and
- `shar.audio.music.state_changed`.

A source ordinal, array position, comment grouping, or integer range is import
provenance only. Catalog generation maps each retained source signal to one
canonical channel and schema.

The catalog rejects:

- duplicate canonical channels;
- one channel with multiple incompatible schemas;
- aliases that resolve ambiguously;
- missing scope or delivery policy;
- unbounded or pointer-only payloads; and
- channels whose declared owner cannot be resolved.

## Payload schema

Every payload is one reflected immutable `UScriptStruct` or equivalent native
value type. A schema declares:

- schema identity and revision;
- channel identity;
- owning module and publisher family;
- required scope;
- delivery phase;
- replay and persistence policy;
- bounded field sizes;
- canonical identity fields;
- optional transient object-reference rules; and
- redaction and diagnostic policy.

Durable or replayable events contain canonical identities and values only. They
do not contain actor pointers, vehicle pointers, component addresses, C strings,
compiler-layout structs, or platform handles.

A same-world transient observation may contain `TWeakObjectPtr` , `FObjectKey` ,
or
an engine object handle when the schema explicitly permits it. Consumers must
still tolerate invalidation before handling. A weak object reference cannot be
the only identity for a durable fact.

All scalar values have declared bounds. Vectors, forces, normalized magnitudes,
player identities, content identities, and revision tokens use repository-owned
native types.

## Envelope

Every routed message carries an envelope with:

- channel and schema identities;
- schema revision;
- scope identity;
- publisher identity;
- monotonic publication sequence;
- frame or simulation observation when applicable;
- world, session, transition, and local-player correlation identities;
- causation and transaction identity when available; and
- availability and diagnostic classification.

Wall-clock time may be included for diagnostics but cannot select gameplay
behavior or event ordering.

## Scope

A channel declares exactly one primary scope:

| Scope | Use |
| :--- | :--- |
| Process | Platform shutdown or fatal process observations only. |
| Game instance | Profile, catalog, application mode, and save lifecycle. |
| World | Gameplay, mission, vehicle, actor, physics, and streaming facts. |
| Session | Campaign, demo, race, or transient gameplay-session facts. |
| Local player | HUD, camera, input, and player-specific presentation. |
| Entity | Explicit actor, vehicle, mission, or placement identity. |

A world event cannot leak into another world or a later play-in-editor instance.
A local-player cue cannot be broadcast to every player unless a separate
multi-player presentation policy declares that behavior.

## Delivery phases

Each channel selects one delivery phase:

- immediate read-only notification;
- end of current domain transaction;
- end of current engine tick group;
- next world frame;
- after application-mode commit; or
- asynchronous adapter completion.

Immediate delivery is allowed only when every listener is read-only and
reentrancy-safe. A listener cannot mutate the subscription set or invoke another
domain transaction through an immediate callback unless the schema explicitly
routes that work into a later command phase.

Queued delivery preserves publication sequence within one publisher and phase.
Cross-publisher ordering is defined only when a coordinator emits a shared
sequence or dependency barrier. Listener registration order never selects
behavior.

## Publication transaction

A domain event is published after its owning mutation reaches an accepted state:

1. validate the command and expected revision;
1. apply the domain transition;
1. persist or stage required durable state;
1. construct the typed payload and envelope;
1. commit the mutation;
1. enqueue or deliver the accepted event; and
1. record terminal publication evidence.

A rejected or rolled-back mutation does not publish a success event. It returns
one typed command result. Failure observations use a separate schema when
consumers legitimately need them.

## Subscription lifecycle

Subscriptions return move-only handles owned by a subsystem, component, view
model, or other explicit lifetime. The handle records:

- router and channel identity;
- subscriber identity;
- scope and world revision;
- optional filter definition;
- availability profile; and
- active or released state.

Releasing the handle is idempotent. World teardown, local-player removal, game
feature deactivation, application-mode exit, and play-in-editor shutdown release
all affected handles.

A base-class destructor does not scan every channel to remove raw pointers.
Destroyed or garbage-collected subscribers cannot remain callable.

## Mutation during dispatch

Dispatch uses stable snapshot semantics. Subscriptions added during delivery do
not receive the current message. A released subscription receives no later
message and does not alter the iteration position of other subscribers.

A subscriber may release its own handle safely. The router never indexes into a
mutable raw-pointer vector after invoking user code.

Recursive publication is bounded by channel policy. Cycles are rejected by
catalog validation when statically visible and by runtime depth or causation
guards when dynamic.

## Domain channel families

The generated catalog groups channels by owner rather than one global enum.
Initial families include:

- application and world lifecycle;
- mission, objective, race, and progression;
- character, vehicle, traffic, and actor state;
- collision, damage, destruction, and physical response;
- collectibles, rewards, economy, and persistent placements;
- interiors, streaming, locations, and transitions;
- conversation, dialogue, mouth animation, audio, and music;
- HUD, menu, camera, navigation, and presentation;
- tutorials, cheats, demonstrations, and development diagnostics; and
- platform input, storage, media, and suspension observations.

Each family owns its payload types. A vehicle collision payload is not reused as
a camera-shake command merely because both contain a force value.

## Collision and force observations

Collision events contain stable participant identities, validated weak runtime
handles when useful, contact point, normal, relative velocity, impulse or
normalized force, physical-material identities, world revision, and simulation
step identity.

Derived presentation such as rumble, camera shake, particles, decals, and sound
subscribes through typed adapters. Those subscribers cannot change the accepted
collision or damage result.

Duplicate physics callbacks for the same simulation contact are normalized by
the owning physics adapter before domain publication.

## Dialogue and audio observations

Dialogue and conversation messages identify participants, dialogue or quote
event, conversation session, role, playback policy, and accepted state. They do
not pass mutable character pointers or raw sound-name strings as authority.

Animation-sound and music-state observations resolve canonical sound, animation,
positional policy, and music-state identities through the content catalog.
Presentation failure cannot roll back an accepted gameplay fact.

## Lifecycle integration

Application-mode, loading, streaming, save, input, and platform adapters publish
revisioned readiness and completion observations. Coordinators accept an
observation only when its transition, world, session, service, and request
identities match the current transaction.

Late completion from a cancelled or superseded request is ignored and recorded.
It cannot activate a replacement world, dismiss a newer loading screen, restore
old input mappings, or complete a newer save.

## Presentation cues

A presentation cue is non-authoritative and may be dropped, combined, limited,
or replaced according to accessibility and performance policy. It contains an
accepted cause identity and enough data to reproduce the intended presentation.

Examples include:

- camera shake;
- rumble or haptic patterns;
- particles, decals, and transient sounds;
- HUD notices and menu feedback;
- dialogue playback; and
- cinematic or animation cues.

A cue cannot grant rewards, complete missions, change damage, or mutate portable
state.

## Queries and read models

A consumer that needs current state uses a typed query or immutable projection.
It does not subscribe to all historical events and reconstruct authority from
listener arrival order.

Events may invalidate or refresh a read model. The read model remains owned by
its domain or projection service.

## Save and replay

Only schemas explicitly marked durable or replayable enter save, replay, or test
fixtures. Most presentation and lifecycle messages are transient.

Durable records use canonical identities, schema revision, transaction identity,
and deterministic field ordering. Migration follows explicit schema converters.
Unknown future required schemas block replay or migration instead of being cast
to the nearest event number.

## Mods and game features

A game feature or mod overlay may register namespaced channels and schemas when
its manifest declares:

- canonical identity and owner;
- payload type and revision;
- scope and delivery phase;
- dependencies and conflicts;
- availability and package policy;
- teardown behavior; and
- tests.

An overlay cannot replace a first-party schema with an incompatible payload or
broaden a local channel to process scope without an accepted override policy.

## Diagnostics

Development tracing records publication, scope, schema, publisher, subscriber
count, delivery phase, duration, correlation identities, drops, and failures.
Payload logging follows field-level redaction.

Development tools may simulate only schemas explicitly marked simulatable and
must construct a valid typed payload. Selecting an arbitrary integer event and
broadcasting a null payload is forbidden.

## Failure behavior

The router fails closed on:

- unknown channel or schema;
- schema and payload mismatch;
- invalid scope or world revision;
- unbounded or invalid payload data;
- forbidden object references;
- duplicate terminal publication;
- stale asynchronous completion;
- subscription use after release;
- recursive cycle or depth violation; and
- unauthorized shipping or mod channel registration.

A routing failure cannot convert a failed command into success or silently drop
one required domain event. Required publication failure returns to the owning
transaction before terminal success when atomicity demands it.

## Validation

Catalog validation proves:

- every channel has one owner and one compatible schema;
- source aliases map uniquely;
- scope and delivery phase are declared;
- durable schemas contain no transient object authority;
- required payload bounds are finite;
- game-feature teardown releases every subscription;
- shipping packages exclude diagnostic-only schemas; and
- event dependencies contain no forbidden cycle.

## Tests

Required tests include:

- deterministic channel and schema lookup;
- payload type and bound rejection;
- world, session, and local-player isolation;
- immediate and queued delivery phases;
- stable publication order within one publisher;
- listener registration-order independence;
- add and remove during dispatch;
- self-unsubscription and world teardown;
- stale asynchronous completion rejection;
- duplicate transaction publication rejection;
- weak-object invalidation;
- collision adapter normalization;
- presentation cue non-authority;
- durable schema serialization and migration;
- mod registration and teardown; and
- diagnostic simulation restrictions.

## Invariants

- Commands, queries, events, and presentation cues remain distinct.
- Every routed payload has a registered schema and bounded fields.
- Listener order never selects domain behavior.
- Raw pointers and source enum ordinals are not durable event authority.
- World and local-player messages remain isolated to their scope.
- One accepted transaction publishes each required terminal event at most once.
- Released subscriptions receive no later messages.
- Late callbacks cannot complete a replacement request.
- Presentation failure cannot reverse accepted gameplay state.
- Diagnostic-only channels cannot ship accidentally.

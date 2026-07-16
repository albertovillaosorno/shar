# Transient VFX and breakable-presentation runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native render-frame, view, and layer runtime](native-render-frame-view-and-layer-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Physical material and impact-response runtime](physical-material-and-impact-response-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [World render-entity and physics runtime](world-render-entity-and-physics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Persistent world-object state runtime](persistent-world-object-state-runtime.md)

## Purpose

This specification defines transient visual effects, reusable Niagara systems,
one-shot and continuous emitters, vehicle-surface effects, breakable replacement
presentation, pooling, quality budgets, streaming, diagnostics, and teardown.

It replaces one process-wide particle singleton, manually updated effect
controllers, fixed per-type instance arrays, integer player handles, per-frame
keepalive calls, custom scene-graph particle objects, manually reoriented
gravity
matrices, source-loaded particle factories, fixed breakable queues, global
breakable inventories, and renderer callbacks that remove gameplay objects.

Transient presentation consumes accepted results. It cannot become damage,
breakage, mission, reward, persistence, physics, or vehicle-state authority.

## Native Unreal foundation

The boundary uses native Unreal facilities:

- Niagara Systems, Emitters, Modules, Parameters, and Data Interfaces;
- `UNiagaraComponent` and supported system-instance lifecycle;
- Niagara Effect Types and scalability policy;
- Niagara pooling or project-owned bounded component reuse where measured;
- Actor and component attachment with explicit coordinate space;
- native decals, audio, camera effects, and material parameter collections where
  required by the presentation definition;
- Geometry Collections, replacement Actors, skeletal or static mesh fragments,
  and native physics when accepted breakage presentation requires them;
- Asset Manager bundles and retained handles; and
- world, feature, and local-player subsystem lifetimes.

Repository code provides typed requests, stable identities, deterministic
parameter binding, bounded leases, lifecycle validation, and immutable results.
It does not implement a second particle engine.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Domain and application services | Own damage, destruction, rewards, mission state, persistence, and vehicle state. |
| Impact-response service | Selects typed sound, Niagara, decal, camera, animation, and damage proposals. |
| VFX definition catalog | Owns stable effect identities, parameter schemas, lifetime classes, and quality policy. |
| Transient VFX subsystem | Validates requests, acquires assets, activates native components, and releases presentation leases. |
| Niagara | Owns particle simulation, emitter execution, modules, parameters, renderers, and native scalability behavior. |
| World render-entity runtime | Owns source Actor and component identity, transform, bounds, and streaming lifetime. |
| Breakage service | Commits breakage, replacement, persistence, and exactly-once result identity. |
| Presentation service | Projects accepted breakage through effects, debris, audio, camera feedback, and optional animation. |
| Display and quality policy | Owns target-specific budgets and visual fallback. |

<!-- markdownlint-enable MD013 -->

An effect being visible, complete, pooled, culled, or destroyed cannot commit a
gameplay result.

## Runtime identities

The boundary uses stable identities for:

- `FSharVfxDefinitionId`;
- `FSharVfxDefinitionRevision`;
- `FSharVfxRequestId`;
- `FSharVfxLeaseId`;
- `FSharVfxInstanceRevision`;
- `FSharVfxParameterSchemaId`;
- `FSharEmitterBindingId`;
- `FSharSourceEntityId`;
- `FSharSourceEntityRevision`;
- `FSharBreakageResultId`;
- `FSharBreakableDefinitionId`;
- `FSharBreakablePresentationRevision`;
- `FSharWorldCompositionRevision`;
- `FSharFeatureRevision`; and
- `FSharVfxResultId`.

Array slots, pool indices, raw component pointers, source factory addresses,
particle enum ordinals, integer player handles, source inventory sections, and
render-layer numbers are not durable identity.

## VFX definition

`FSharVfxDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `VfxId` | Canonical effect identity. |
| `NiagaraSystem` | Class-validated cooked Niagara System reference. |
| `ParameterSchemaId` | Typed user and system parameter contract. |
| `LifetimeClass` | One-shot, finite loop, leased continuous, attached continuous, or definition-owned persistent presentation. |
| `TransformPolicy` | World, actor, component, socket, bone, wheel, surface, or camera-relative placement. |
| `TimePolicy` | Game time, real time, presentation time, pause behavior, and time dilation. |
| `CompletionPolicy` | Native completion, declared duration, owner release, or accepted result. |
| `PoolingPolicy` | Native pooling or measured project reuse policy. |
| `ScalabilityPolicy` | Effect Type, distance, visibility, instance, significance, and quality behavior. |
| `FallbackPolicy` | Reduced system, material, decal, audio-only, or no-visual fallback. |
| `StreamingPolicy` | Bundle, prefetch, retention, and teardown behavior. |
| `DiagnosticsPolicy` | Development capture and inspection permissions. |

<!-- markdownlint-enable MD013 -->

Definitions reject missing cooked systems, untyped parameter names, indefinite
one-shot lifetimes, gameplay callbacks, unsupported coordinate spaces, and pools
without complete reset behavior.

## Request model

`FSharVfxRequest` contains:

- request and owner identities;
- effect definition and expected revision;
- world and feature revisions;
- source entity, component, socket, or surface identity;
- transform and coordinate space;
- source linear and angular velocity when relevant;
- physical surface and impact severity when relevant;
- typed parameter values;
- lifetime and priority policy;
- local-player or view eligibility when camera-relative;
- deduplication and rate-limit key;
- cancellation token; and
- diagnostics context.

Requests are immutable after acceptance. Parameter changes use a typed update
command correlated to the active lease.

## Terminal result

Every accepted request reaches exactly one terminal status:

- `completed`;
- `released`;
- `cancelled`;
- `superseded`;
- `rejected`;
- `timed_out`;
- `failed`; or
- `owner_gone`.

`FSharVfxResult` records request, definition, lease, world, feature, and source
revisions plus fallback and cleanup evidence.

VFX completion is presentation evidence only. It cannot prove a hit, kill,
breakage, collection, mission objective, or save transaction.

## Parameter schema

Every effect exposes a closed parameter schema with:

- canonical parameter identity;
- Niagara variable name and expected type;
- required or optional status;
- units and coordinate space;
- finite range and clamping policy;
- default and fallback;
- update frequency;
- replication or local-only classification; and
- quality applicability.

Typical parameters include transform, velocity, color, scale, emission
multiplier,
surface, severity, wheel slip, vehicle speed, lifetime, and owner visibility.

A raw string parameter, numeric slot, or arbitrary source bias index is not a
runtime contract.

## Coordinate space and transforms

A definition explicitly selects world, actor-local, component-local, socket,
bone, wheel, surface-frame, or camera-relative space.

The runtime does not rotate every emitter transform to force one historical up
axis. Import and construction normalize asset coordinates, while the request
provides the accepted runtime transform and source velocity.

Attached effects track a checked weak owner and attachment revision. Detachment,
owner replacement, teleport, world transfer, or teardown either updates through
a
declared policy or releases the lease.

## One-shot effects

One-shot effects start once from one accepted request and stop through native
completion, a declared duration, or an explicit owner cancellation.

The subsystem:

1. validates the request and assets;
1. selects native pooled or fresh component allocation;
1. applies the complete parameter snapshot;
1. binds transform and ownership;
1. activates the system;
1. observes native completion or timeout;
1. resets reusable state; and
1. publishes one terminal result.

A one-shot request cannot be restarted by an old completion callback.

## Leased continuous effects

Continuous effects use explicit leases. The owner acquires a lease, submits
typed
parameter updates while the lease is active, and releases it when the continuous
condition ends.

A lease declares:

- owner and source revisions;
- maximum lifetime or renewal deadline;
- parameter-update policy;
- pause and visibility behavior;
- asset-retention policy;
- cancellation and teardown; and
- fallback.

The runtime does not require an owner to call a play function every frame merely
to prevent emission from silently becoming zero. Missing required updates follow
a declared stale-update policy and produce diagnostics.

## Cyclic and looping systems

Finite loops declare loop count, duration, or terminal owner result. Indefinite
loops require an active bounded lease.

Looping system time follows the definition's presentation-time policy. Frame
count, callback frequency, and load completion order cannot select loop phase or
termination.

Reset returns the native system to a validated baseline and clears all
owner-specific parameters, bindings, event state, and completion delegates.

## Pooling and reuse

Pooling is an optimization, not identity or authority. The runtime prefers
Niagara's supported pooling behavior. A project-owned reuse layer is allowed
only
when profiling demonstrates a benefit and complete reset is tested.

A reusable instance must reset:

- transform and attachment;
- system age and execution state;
- all user parameters;
- source velocity and surface data;
- visibility and local-player policy;
- completion and event delegates;
- pause, time-dilation, and scalability state;
- feature and world ownership; and
- diagnostics tags.

Pool capacity is target- and effect-policy data. A fixed source capacity cannot
silently become a gameplay or content limit.

When capacity is exhausted, the definition chooses reject, reuse an eligible
oldest cosmetic lease, select a fallback, or allocate within a bounded budget.
The outcome is typed and observable.

## Native scalability

Every effect references a validated scalability policy covering:

- maximum active system instances;
- distance and visibility behavior;
- significance and culling response;
- CPU or GPU simulation requirements;
- spawn-count scaling;
- quality and platform tiers;
- fixed-bounds requirements;
- local-player and split-screen cost; and
- fallback behavior.

Culling an effect changes presentation only. Required collision, physics,
missions, interactions, route state, and persistence continue unchanged.

## Vehicle particle and surface effects

Vehicle effects consume immutable vehicle, wheel, movement, contact, and
physical
surface observations.

`FSharVehicleVfxObservation` contains:

- vehicle and movement revisions;
- wheel or part identity;
- world transform and local attachment transform;
- linear velocity and wheel-ground relative velocity;
- slip, skid, acceleration, braking, and impact evidence;
- physical surface and weather state;
- source visibility and quality policy; and
- simulation timestamp.

The vehicle VFX policy selects dust, smoke, water, sparks, exhaust, tire marks,
damage, or other cosmetic systems from typed thresholds.

An emitter cannot change traction, wheel contact, vehicle damage, speed, or
physical surface. It consumes those accepted observations.

## Wheel and part bindings

Wheel, exhaust, engine, body, and damage emitter bindings use canonical part or
socket identities. Array positions and assumed wheel order are not durable.

A binding validates:

- compatible vehicle definition;
- required socket, bone, or scene component;
- coordinate space;
- owner and attachment revision;
- eligible effect definitions;
- fallback transform; and
- teardown behavior.

A missing optional binding suppresses or falls back according to policy. It does
not attach to an arbitrary part.

## Impact effects

Impact VFX consume the immutable impact observation from the physical-material
runtime. The selected response may include Niagara, decal, sound, camera, and
animation requests.

Deduplication uses contact pair, response definition, simulation step, and
severity policy. It cannot suppress required domain damage or breakage results.

Visual placement may perform one bounded confirmation trace without replacing
the
accepted contact classification.

## Breakage authority

Breakage commits through the world-object, mission, damage, or interaction
owner.
`FSharBreakageResult` contains:

- breakage result identity;
- entity and placement revisions;
- breakable definition revision;
- accepted cause and evidence identity;
- intact and replacement representation identities;
- persistence and respawn result;
- reward and progression owner result where applicable;
- world and feature revisions; and
- presentation policy.

The presentation subsystem consumes this committed result. A particle, debris
animation, fragment timeout, or missing visual asset cannot decide whether the
object broke.

## Breakable presentation definition

`FSharBreakablePresentationDefinition` declares:

- accepted breakable identity;
- intact and broken visual representations;
- Geometry Collection, replacement Actor, skeletal, static mesh, or Niagara
  presentation;
- fragment collision and physics policy;
- Niagara, decal, sound, camera, and material requests;
- lifetime and cleanup policy;
- pooling or reuse eligibility;
- visibility and quality policy;
- streaming and prefetch policy;
- persistence and respawn projection; and
- deterministic fallback.

Every presentation path preserves the committed breakage result and canonical
placement identity.

## Breakage presentation transaction

After accepted breakage:

1. validate result and current entity revision;
1. acquire required presentation assets;
1. prepare replacement and effects without exposing partial state;
1. disable or replace the intact visual and collision projection according to
   the
   committed result;
1. activate broken presentation and transient effects;
1. publish the accepted presentation revision;
1. release transient fragments and effects according to policy; and
1. retain or reconstruct durable remnants from persistence state.

Failure uses the declared fallback and cannot roll back the committed domain
result unless the owning domain transaction explicitly supports it.

## Breakable queues and arbitration

Breakable presentation requests use a bounded priority queue keyed by stable
request identity, world revision, importance, distance, and policy.

A fixed circular queue cannot silently overwrite pending breakage. Capacity
pressure returns typed fallback evidence and preserves required durable
projection.

Presentation priority cannot alter reward, mission, persistence, or collision
authority. A low-priority cosmetic effect may be reduced while the accepted
broken
state remains correct.

## Zone, region, and streaming ownership

Transient VFX and broken presentation belong to exact world, region, Data Layer,
feature, and owner revisions.

Region unload:

- stops accepting new owned requests;
- cancels or migrates eligible continuous leases;
- releases transient components and pooled bindings;
- unregisters replacement presentation;
- retains only persistence-owned durable state;
- releases asset handles; and
- rejects late callbacks.

A global inventory section or source zone list is not runtime ownership.

## Construction integration

Cooked Niagara Systems, Geometry Collections, fragment meshes, materials, audio,
and breakable definitions are loaded and prepared through
<!-- markdownlint-disable-next-line MD013 -->
[Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md).

Import publishes:

- stable VFX and breakable identities;
- parameter schemas;
- primary-asset and bundle metadata;
- class-restricted soft references;
- platform variants;
- Effect Type and scalability policy;
- replacement and fallback plans; and
- provenance and validation evidence.

Shipping runtime cannot load source particle factories or breakable inventories.

## Time and pause behavior

Each effect selects game, real, presentation, or sequence time. Pause, slow
motion, loading, media, camera transitions, and platform suspension follow the
selected policy.

A frame delta passed through one global manager cannot redefine system time.
Effects that depend on final camera or physics state use native tick groups or
explicit prerequisites.

## Split-screen and multiple views

World-space effects are one world presentation projected independently into each
eligible view. Camera-relative effects are explicitly per view.

Scalability considers combined local-view cost without sharing camera-relative
parameters, visibility decisions, or screen-space transforms across players.

One player's effect culling cannot suppress another player's required feedback
when the definition marks it required for that view.

## Networking boundary

Servers commit gameplay and breakage results without requiring VFX. Clients may
project replicated or predicted accepted observations through local effects.

A VFX request declares authoritative, predicted, replayed, or local-only origin.
Deduplication and reconciliation use stable result identities, not component
pointers or arrival order.

Effect completion cannot acknowledge a networked gameplay transaction.

## Accessibility

Definitions declare alternatives or reductions for flashes, rapid contrast,
camera shake, persistent screen obstruction, color-only information, and motion.

Accessibility changes presentation parameters or selects a validated fallback.
They cannot remove required semantic feedback without an equivalent alternative.

## Platform and quality policy

Low through Ultra may vary:

- spawn counts;
- CPU or GPU emitter selection;
- renderer count and complexity;
- materials, lighting, distortion, and translucency;
- collision or depth sampling used only for presentation;
- decal and fragment density;
- effect distance and significance policy; and
- optional camera effects.

Required state feedback remains semantically equivalent. Android uses the
accepted Low-only policy and effects validated for the mobile renderer.

## Feature and mod overlays

A validated feature may add namespaced VFX, parameter schemas, Effect Types,
vehicle bindings, and breakable presentation definitions.

Activation validates assets, namespaces, platform support, budgets, constructor
availability, replacement compatibility, and teardown.

Removal cancels owned leases, releases native components and handles, removes
namespaced definitions, and rejects stale callbacks. It cannot leave a base
object
hidden, broken, or visually replaced.

## Concurrency

Game-thread request validation, native Niagara execution, render-thread
submission, physics observations, and asynchronous loading communicate through
supported engine boundaries.

Callbacks carry request, lease, definition, source, world, feature, and breakage
revisions. No callback reads a raw manager-owned object after release.

## Diagnostics

Development diagnostics may expose:

- active requests and leases by effect identity;
- native system and Effect Type;
- parameter snapshot and coordinate space;
- source entity, wheel, part, surface, and view revisions;
- allocation, pool reuse, rejection, and fallback counts;
- active system, emitter, and particle counts exposed by supported tools;
- scalability and significance decisions;
- breakage result and presentation revisions;
- region and feature ownership;
- stale callback and incomplete-reset findings; and
- CPU, GPU, memory, and overdraw evidence.

Diagnostics are read-only and cannot spawn, stop, break, reward, or persist an
object.

## Failure behavior

The boundary fails closed on:

- unknown VFX or breakable identity;
- missing or wrong-class cooked asset;
- invalid parameter type, range, units, or coordinate space;
- stale source, world, feature, view, lease, or breakage revision;
- unbounded continuous effect without an owner lease;
- pool reuse without complete reset;
- fixed capacity silently dropping required feedback;
- frame-call keepalive as the only continuous-lifetime authority;
- raw source particle factory or breakable inventory in shipping runtime;
- particle completion attempting gameplay mutation;
- breakage presentation without a committed breakage result;
- region unload with active unowned effects or handles;
- feature removal with owned systems or definitions; and
- server gameplay depending on renderer or VFX completion.

Failure returns typed evidence, releases prepared native objects, and preserves
the accepted domain and persistence state.

## Validation

Definition and cooked-asset validation prove:

- every VFX, parameter, vehicle binding, breakable, and fallback identity
  resolves;
- Niagara systems and parameters match the declared schema;
- lifetime and completion policies are bounded;
- pooled instances reset every mutable field;
- Effect Types and scalability cover all supported targets;
- coordinate-space and attachment bindings are valid;
- vehicle effects consume observations without changing vehicle state;
- breakage presentation requires a committed result;
- required semantic feedback survives every quality preset;
- streaming and feature teardown release all owned systems and handles;
- no source particle or breakable loader remains in packaged runtime; and
- no presentation callback has domain authority.

## Tests

Required automated and visual tests include:

- one-shot activation, completion, timeout, cancellation, and stale callback;
- continuous lease acquisition, update, expiry, and release;
- looping behavior across frame rates, pause, and time dilation;
- parameter-schema type, unit, range, and fallback validation;
- world, actor, socket, bone, wheel, surface, and camera-relative transforms;
- native pool reuse with complete reset;
- capacity pressure and typed fallback;
- Effect Type distance, significance, and quality behavior;
- vehicle dust, smoke, water, sparks, exhaust, and damage observations;
- missing vehicle binding and deterministic fallback;
- impact deduplication without domain-result suppression;
- breakage success, duplicate result, stale result, fallback, and teardown;
- fragment and Geometry Collection lifetime;
- persistent broken remnant reconstruction;
- region unload and feature removal with zero retained components and handles;
- split-screen camera-relative isolation;
- network result deduplication without VFX authority;
- accessibility alternatives;
- Low through Ultra semantic feedback;
- Android Low performance and fallback; and
- diagnostics with no behavior change.

## Invariants

- Niagara owns particle simulation and renderer execution.
- Transient VFX consume accepted observations and results.
- Every continuous effect has one bounded owner lease.
- Pool slots and component pointers are never public identity.
- All reusable instances reset completely before reassignment.
- Vehicle effects cannot change vehicle physics or movement state.
- Breakage commits before breakable presentation begins.
- VFX completion never commits gameplay, mission, reward, or persistence state.
- Quality may reduce cosmetic cost but not required semantic feedback.
- Servers never depend on VFX or renderer completion.
- Every retired world or feature releases all owned effects and handles.

# Native asset load request and streaming runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
- [Runtime parity test boundary](../../adr/unreal/runtime/runtime-parity-test-boundary.md)
- [Lossless extraction contract](../../adr/pipeline/extraction/lossless-extraction-contract.md)
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
- [Native import, material rebuild, and world assembly](native-import-material-and-world-assembly.md)
- [Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md)
- [Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md)
- [World render-entity and physics runtime](world-render-entity-and-physics-runtime.md)
- [Authored state-prop animation and event runtime](authored-state-prop-animation-and-event-runtime.md)
- [Local supersprint race session runtime](local-supersprint-race-session-runtime.md)
- [Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md)
- [Platform audio cooking and streaming](platform-audio-cooking-and-streaming.md)
- [Platform cinematic media packaging](platform-cinematic-media-packaging.md)
<!-- markdownlint-enable MD013 -->

## Purpose

This specification defines cooked-asset requests, package mounting, dependency
planning, progress, cancellation, timeouts, result handling, world streaming,
and source-format conversion boundaries for the native Unreal runtime.

It replaces one process-wide loading singleton, a fixed filename ring buffer,
handler enums, raw callback pointers, untyped user data, manual memory heaps,
inventory-section strings, source archive registration, and runtime parsing of
legacy camera, locator, road, path, UI, choreography, icon, and sound files.

Runtime loading consumes validated Unreal assets and package identities. Source
formats are decoded only by the deterministic import pipeline.

## Ownership

<!-- markdownlint-disable MD013 -->
| Authority | Responsibility |
| :--- | :--- |
| Asset Manager | Primary-asset identity, bundles, dependency discovery, and policy. |
| Streamable Manager | Asynchronous soft-object and bundle handles. |
| Package adapter | Installed package, chunk, IoStore, and game-feature mounting. |
| World runtime | World Partition, Runtime Data Layers, level instances, and actors. |
| Feature runtime | Game-feature activation and teardown. |
| Specialized subsystems | Audio, UI, media, camera, mission, and gameplay readiness. |
| Import pipeline | Source parsing, conversion, provenance, and native asset creation. |
| Loading coordinator | Typed request graph, barriers, results, progress, and recovery. |
<!-- markdownlint-enable MD013 -->

The loading coordinator does not parse source files, own gameplay state, or
select mission and progression outcomes.

## Runtime topology

The loading module owns these C++ types:

- `FSharLoadRequestId`, a stable request identity;
- `FSharLoadPlanId`, a stable dependency-plan identity;
- `FSharLoadScopeId`, a mode, world, session, feature, or local-player scope;
- `FSharPackageId`, a stable installed-package identity;
- `FSharAssetBundleId`, a stable Asset Manager bundle identity;
- `FSharLoadRequest`, an immutable typed request;
- `FSharLoadPlan`, an immutable dependency graph;
- `FSharLoadHandle`, a move-only cancellation and observation handle;
- `FSharLoadProgress`, a bounded progress snapshot;
- `FSharLoadResult`, a closed terminal result;
- `USharLoadCoordinatorSubsystem`, game-instance request coordination;
- `USharWorldReadinessSubsystem`, world and Data Layer barriers; and
- package, audio, UI, and media adapters behind application ports.

Requests refer to canonical identities. Filesystem paths, source handler types,
heap names, section names, and raw object pointers remain outside the contract.

## Request model

Every request declares:

| Field | Contract |
| :--- | :--- |
| `RequestId` | Stable unique identity for one attempt. |
| `PlanId` | Immutable dependency plan revision. |
| `ScopeId` | Owning mode, world, session, feature, or local player. |
| `CallerId` | Stable requesting subsystem or transaction identity. |
| `Priority` | Bounded product priority. |
| `AssetIds` | Primary assets, bundles, worlds, or feature identities. |
| `ExpectedRevisions` | Catalog, package, world, save, and mod-set revisions. |
| `Deadline` | Positive timeout or explicit long-running permission. |
| `CancellationPolicy` | Reject, cancel, replace, or retain shared work. |
| `ReadinessBarrierId` | Required post-load verification contract. |
| `ResultPolicy` | Required, optional, degraded, or presentation-only. |

A request cannot contain an arbitrary filename, callback pointer, memory heap,
source archive handle, or unbounded user-data payload.

## Stable asset identity

Runtime identity uses Asset Manager primary-asset identities, stable package
identities, soft object paths generated by the repository, and typed world or
feature identities.

A source filename may remain in import provenance, but runtime behavior never
depends on source path spelling, extension, platform directory, or an inventory
section inferred from a filename.

Aliases resolve during catalog generation. Ambiguous or missing aliases fail
validation rather than selecting the first loaded object.

## Load plans

A load plan is a directed acyclic dependency graph. Nodes may represent:

- installed package availability;
- game-feature registration and activation;
- primary-asset bundles;
- world or level-instance preparation;
- World Partition cells and Runtime Data Layers;
- native audio banks and streaming assets;
- Common UI and front-end assets;
- media sources and synchronized audio variants;
- camera, mission, population, vehicle, and gameplay definitions; and
- verification barriers.

Edges declare required ordering. Independent nodes may execute concurrently
under budget. A callback sentinel inserted into a filename queue is not a
readiness barrier.

Catalog validation rejects dependency cycles, missing nodes, ambiguous owners,
and barriers that can never reach a terminal state.

## Request arbitration

The coordinator orders pending work by:

1. fatal recovery and process-exit preparation;
1. active application-mode transition;
1. required world and session readiness;
1. local-player and front-end readiness;
1. optional presentation prefetch;
1. development and automation prefetch.

Equal-priority requests use stable request identity. Ring-buffer position,
submission callback order, and frame arrival cannot select a winner.

Shared dependencies are coalesced by asset, package, revision, and scope policy.
Cancelling one consumer does not cancel work still required by another consumer.

## Execution phases

A request follows these phases:

1. validate identity, scope, revisions, availability, and permissions;
1. resolve the immutable dependency graph;
1. mount required installed packages or game features;
1. acquire Asset Manager and streamable handles;
1. prepare worlds, Data Layers, audio, UI, and media in dependency order;
1. verify every required postcondition;
1. commit the owning application or gameplay transition;
1. release superseded source-scope handles; and
1. publish one terminal result.

Loading data is not success. Success requires the declared readiness barrier and
accepted owning transition.

## Result model

Every request reaches one terminal status:

<!-- markdownlint-disable MD013 -->
| Status | Meaning |
| :--- | :--- |
| `success` | All required assets and postconditions are ready. |
| `unavailable` | Required package, asset, platform feature, or capability is absent. |
| `rejected` | Scope, revision, permission, graph, or parameter validation failed. |
| `failed` | Loading or verification began but could not complete safely. |
| `timed_out` | The declared deadline elapsed and cleanup completed. |
| `cancelled` | The owning request cancelled and released its private work. |
| `superseded` | A newer accepted request replaced the pending request. |
| `degraded` | Only explicitly optional presentation assets were unavailable. |
<!-- markdownlint-enable MD013 -->

Every result identifies the verified final scope, loaded and released handles,
failed node, typed reason, and diagnostic correlation identity.

## Progress

Progress is a read-only projection over the plan. It contains:

- request and plan identities;
- completed, active, pending, failed, and cancelled node counts;
- bounded byte progress when the platform exposes reliable values;
- semantic phase and current required barrier;
- optional display label identity; and
- monotonic revision.

Byte counts never replace semantic readiness. UI may smooth presentation but
cannot report completion before the coordinator reaches a terminal result.

## Cancellation

Cancellation is explicit and idempotent. It stops work that is private to the
request, releases private handles, and preserves shared dependencies still in
use.

A request cannot cancel only future queue entries while leaving the active
operation without an owner. Every active node has a cancellation or completion
policy.

Late completion from cancelled or superseded work is ignored after identity and
revision checks. It cannot commit a world, dismiss a loading screen, activate an
old UI, or restore released audio.

## Timeouts and retries

Every required asynchronous node has a positive timeout or an explicit product
exception. Retry policy declares maximum attempts, backoff, and retriable typed
reasons.

Retries preserve the parent request identity and use distinct attempt
identities. A successful late attempt from an older request is still stale.

Unknown errors, malformed assets, schema failures, and package-integrity
failures are not retried automatically.

## Synchronous loading

Shipping gameplay and application transitions do not perform unbounded
synchronous disk or source-file loading on the game thread.

Synchronous access is limited to:

- already resident assets verified by read-back;
- editor import and validation tools;
- deterministic commandlets;
- startup assets explicitly approved by platform budgets; and
- tests whose fixture contract requires synchronous execution.

A synchronous fallback cannot silently replace a failed asynchronous request.

## Package mounting

Installed content uses platform packaging, IoStore, chunks, and game-feature or
mod package adapters. A package mount request declares:

- canonical package identity and version;
- integrity and signature policy;
- dependency and conflict identities;
- platform and build compatibility;
- mount and unmount lifecycle;
- Asset Registry contribution;
- game-feature policy when applicable; and
- failure and rollback behavior.

The runtime never registers a source archive by raw filename or stores package
objects in fixed platform-sized arrays.

Unmount waits for dependent handles to release. A package cannot disappear while
an active world, object, audio stream, UI, or media source still depends on it.

## Source conversion boundary

The following source families are import evidence, not shipping runtime loaders:

<!-- markdownlint-disable MD013 -->
| Source family | Native target |
| :--- | :--- |
| Generic scene packages | Cooked meshes, materials, animations, worlds, and Data Assets. |
| Camera data chunks | Versioned camera preset assets and authored rig definitions. |
| Locator chunks | Typed placements, triggers, Smart Objects, portals, and camera rails. |
| Road, path, and intersection chunks | Native road graph, route, spline, and navigation assets. |
| Choreography text | Typed action, animation, cinematic, and presentation definitions. |
| Console script text | Validated developer-command definitions or rejected input. |
| Legacy UI projects and icons | Common UI, UMG, Slate, texture, and material assets. |
| Sound resource files | Native sound, dialogue, bank, cue, and streaming definitions. |
| Source archives | Imported provenance and installed cooked package output. |
<!-- markdownlint-enable MD013 -->

The native runtime does not instantiate a source chunk handler to construct
actors or gameplay services during play.

## Construction handoff

A successful asset-load result transfers retained handles and immutable loaded
object identities to
<!-- markdownlint-disable-next-line MD013 -->
[Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md).
The construction subsystem revalidates definition, bundle, world, placement, and
feature revisions before preparing native objects.

Load completion cannot:

- instantiate an arbitrary wrapper class;
- call a mutable process-global listener;
- encode cancellation as a null entity;
- add a drawable to a fixed global-entity array;
- override another constructor or engine handler;
- activate collision, simulation, interaction, or mission behavior; or
- publish gameplay readiness before construction commit.

The load subsystem owns packages and handles. The construction subsystem owns
prepared native objects. World and gameplay services own activation.

## Generic scene packages

Generic source scene packages are decoded by the extraction and import pipeline.
Import validates chunk structure, provenance, identity, references, geometry,
materials, animation, collision, physics, and supported gameplay metadata.

Cooked packages contain only native Unreal objects and validated repository Data
Assets. Runtime inventory sections and temporary source sections do not exist.

## Camera data

Imported follow, walker, rail, static, and other camera definitions become
versioned camera presets. Runtime loading requests those presets by canonical
identity through the camera asset bundle.

Duplicate source camera identities fail import. The runtime does not read camera
chunks into a global camera inventory.

## Locators and trigger volumes

Imported locators normalize into typed placement records such as:

- generic anchors and directional transforms;
- interaction and action placements;
- interior portals and return points;
- camera cuts, FOV regions, static shots, and rail splines;
- dynamic-zone and streaming boundaries;
- vehicle, pedestrian, race, and mission spawn points;
- occlusion and visibility regions;
- collectibles and persistent placements; and
- validated sphere or box trigger definitions.

Each placement has stable identity, owner, world or Data Layer, transform,
shape, activation policy, and verification evidence. Integer locator subtypes
and nested source chunks remain provenance only. Registration, occupancy,
enter/exit observations, domain adapters, and teardown follow the
<!-- markdownlint-disable-next-line MD013 -->
[authored spatial placement and trigger runtime](authored-spatial-placement-and-trigger-runtime.md).

## Roads, paths, and intersections

Road, path, segment, and intersection chunks convert into immutable native route
and navigation data. Conversion validates:

- stable road, segment, lane, path, and intersection identities;
- endpoints, adjacency, direction, lane count, and geometry;
- speed, traffic, pedestrian, race, and AI policy;
- spline and route continuity;
- world and Data Layer ownership;
- duplicate and disconnected graph behavior; and
- deterministic ordering.

Runtime systems consume one validated graph revision. They do not append objects
to a process-global road manager while a source file is loading.

## Choreography and runtime text

Choreography source is parsed during import into typed native definitions. Every
retained action resolves to a closed handler, asset identity, bounds, ownership,
and verification rule.

Runtime loading of arbitrary text followed by console or choreography evaluation
is forbidden. Development text scripts follow the restricted developer-command
contract and cannot ship as implicit gameplay code.

## User interface and icons

Legacy UI projects, pages, screens, resources, and icons convert into Common UI,
UMG, Slate, texture, material, localization, and view-model assets.

Runtime requests a declared front-end or HUD bundle. It does not read an opaque
icon file into manually allocated memory or load a source UI project by section
name.

## Audio

Audio requests use the native audio asset, bank, locale, streaming, and mix
contracts. Completion must match request, bank, locale, world, and audio
revision.

Source sound handlers and raw filename callbacks are conversion provenance only.
A late audio callback cannot activate a released mode or world.

## World streaming

World requests coordinate:

- destination world identity;
- World Partition readiness;
- Runtime Data Layer activation;
- required level instances;
- gameplay-feature activation;
- actor and subsystem readiness;
- player spawn and recovery locations; and
- final world revision.

Streaming visibility alone is not readiness. Required actors, placements,
collision, navigation, mission anchors, and subsystem snapshots must match the
same world revision.

World-entity construction and teardown follow
<!-- markdownlint-disable-next-line MD013 -->
[World render-entity and physics runtime](world-render-entity-and-physics-runtime.md).
A region becomes gameplay-ready only after required Actor/component composition,
cooked collision, query surfaces, physical profiles, and simulation policy are
validated. Unload disables new commands, records required state, unregisters
components and bodies, destroys Actors, and releases retained assets under one
correlated transaction.

## Interior loading

Interior entry uses the ordinary world-composition request model. The interior
transaction requests its Data Layers, actors, lighting, reflection, interaction,
audio, camera, and presentation bundles under one transition identity.

The exterior remains authoritative until the interior barrier succeeds. Failed
or cancelled entry releases staged handles and restores the exterior without
leaving mixed visibility or stale interaction state.

## Memory and residency

Memory ownership follows Unreal objects, packages, streamable handles, worlds,
subsystems, and platform caches. Requests declare residency class and release
policy rather than selecting a manual heap.

Residency classes include:

- process startup;
- game instance;
- application mode;
- world or session;
- local player;
- feature or mod;
- transient presentation; and
- editor or automation only.

Budget enforcement may delay, reject, reduce, or evict optional work. It cannot
evict a required asset still owned by an active scope.

## Prefetch

Prefetch is non-authoritative and cancellable. It may warm likely worlds,
characters, vehicles, UI, audio, or media after required work and within
budget.

A prefetched asset does not unlock content, select a mission, activate a
feature, or prove readiness. The owning transaction still validates current
revisions.

## Mods and game features

A validated mod or game feature may contribute packages, primary assets,
bundles, world layers, UI, audio, media, and typed definitions when its manifest
declares identity, version, dependencies, conflicts, package policy, teardown,
and tests.

An overlay cannot register an arbitrary source file handler, execute runtime
text, replace a first-party asset with an incompatible schema, or retain handles
after deactivation.

## Diagnostics

Development diagnostics expose:

- request, plan, scope, and caller identities;
- dependency graph and node states;
- priorities and coalesced consumers;
- package mounts and active streamable handles;
- progress and readiness barriers;
- durations, retries, cancellation, and stale completions;
- memory-residency observations; and
- terminal results.

Diagnostics redact private paths and platform credentials. Source paths may be
shown only in editor import evidence that is already authorized for that user.

## Failure behavior

The runtime fails closed on:

- unknown request, asset, bundle, package, world, or feature identity;
- invalid scope or stale revision;
- dependency cycle or unresolved required node;
- package integrity, version, signature, or compatibility failure;
- missing cooked asset or schema mismatch;
- source-format input presented to shipping runtime;
- invalid or non-finite progress data;
- timeout, cancellation, or stale completion;
- readiness-barrier failure;
- handle use after release; and
- unauthorized mod or development package contribution.

A loading failure cannot publish mission success, grant rewards, activate an
unverified world, or leave input and presentation bound to destroyed state.

## Validation

Cook and catalog validation prove:

- every required runtime identity resolves to a cooked native asset;
- no shipping request depends on a source filename or handler enum;
- dependency graphs are acyclic and bounded;
- every active scope has a release path;
- required barriers are satisfiable and revisioned;
- package dependencies and conflicts are consistent;
- source parsers are excluded from unauthorized runtime packages;
- optional degradation cannot remove gameplay authority; and
- mod and game-feature teardown releases all contributed handles.

## Tests

Required tests include:

- deterministic identity and bundle resolution;
- graph cycle, missing-node, and ambiguous-alias rejection;
- priority, coalescing, and equal-priority ordering;
- cancellation of private and shared dependencies;
- timeout, retry, stale completion, and supersession;
- progress monotonicity and semantic barrier behavior;
- package mount, integrity failure, dependency, and unmount;
- world, Data Layer, feature, audio, UI, and media readiness;
- interior entry rollback and exterior restoration;
- source-family import-to-native mapping;
- shipping rejection of source decoders and runtime text evaluation;
- memory residency and release by scope;
- mod activation and teardown; and
- repeated play-in-editor sessions without leaked handles.

## Invariants

- Shipping runtime loads cooked native assets, never source formats.
- Every request has stable identity, scope, revisions, and one terminal result.
- Readiness requires verified postconditions, not only file completion.
- Cancellation and teardown release every private handle.
- Shared work remains alive while another accepted consumer owns it.
- Late callbacks cannot commit replacement state.
- World and local-player scopes remain isolated.
- Manual heaps, inventory sections, and handler enums are not runtime authority.
- Package mounting is versioned, validated, and dependency-safe.
- Optional presentation degradation cannot change gameplay semantics.

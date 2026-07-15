# Memory ownership, budget, and diagnostics runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
- [Developer command and diagnostic runtime](developer-command-and-diagnostic-runtime.md)
<!-- markdownlint-enable MD013 -->

## Purpose

This specification defines memory ownership, lifetime scopes, native containers,
allocation policy, pools, budgets, pressure handling, out-of-memory behavior,
tracing, leak detection, instance statistics, and validation for the Unreal
runtime.

It replaces global `new` and `delete` overrides, mutable heap stacks, source-era
allocator enums, platform heap construction, mode-specific heap destruction,
manual free-space searches, raw-address leak maps, fixed object pools, custom
container allocators, and diagnostic behavior that changes allocation routing.

Memory behavior follows Unreal object, subsystem, world, feature, package,
streaming, task, and native C++ lifetimes. Application modes never become heap
owners.

## Ownership

<!-- markdownlint-disable MD013 -->
| Authority | Responsibility |
| :--- | :--- |
| Unreal allocator and platform memory layer | Native allocation, alignment, low-level failure, and platform reporting. |
| UObject and garbage collection | Reflected object reachability, collection, clusters, and weak references. |
| Native C++ owners | RAII, smart pointers, containers, resources, and deterministic destruction. |
| Asset Manager and streaming | Asset residency, bundle handles, package dependencies, and release. |
| World and game features | World, Data Layer, level-instance, feature, actor, and component lifetimes. |
| Memory budget subsystem | Product categories, target budgets, pressure, and evidence. |
| Diagnostic subsystem | Memory Trace, Insights, LLM, statistics, leak tests, and reports. |
| Domain and application services | Bounded data structures and explicit ownership contracts. |
<!-- markdownlint-enable MD013 -->

The memory subsystem does not decide mission, progression, streaming
destination, quality preset, or application mode. It reports budget and pressure
evidence to the owning policy.

## Runtime topology

The memory module owns these repository types:

- `FSharMemoryBudgetId`, a stable budget identity;
- `FSharMemoryScopeId`, a process, world, session, feature, or request identity;
- `FSharMemoryCategoryId`, a stable semantic category;
- `FSharMemoryBudgetDefinition`, immutable thresholds and policy;
- `FSharMemoryUsageSnapshot`, revisioned usage and residency evidence;
- `FSharMemoryPressureObservation`, normalized pressure state;
- `FSharMemoryRecoveryRequest`, one bounded pressure response;
- `FSharMemoryRecoveryResult`, one closed terminal result;
- `FSharObjectPoolId`, a stable optional pool definition;
- `FSharObjectPoolHandle`, generation-safe checked ownership;
- `USharMemoryBudgetCatalog`, immutable target and profile definitions; and
- `USharMemoryBudgetSubsystem`, game-instance observation and coordination.

These types describe product budgets and observations. They do not replace
Unreal's allocator, garbage collector, containers, or tracing systems.

## Lifetime scopes

Every long-lived allocation belongs to one declared scope:

<!-- markdownlint-disable MD013 -->
| Scope | Examples | Required release boundary |
| :--- | :--- | :--- |
| Process | Immutable catalogs, core modules, crash-safe metadata. | Engine shutdown. |
| Game instance | Profile, application services, root UI, device configuration. | Game-instance teardown. |
| World | Actors, components, physics, navigation, streaming projections. | World cleanup. |
| Session | Mission, race, demo, replay, transient progression projections. | Session terminal result. |
| Game feature | Feature assets, services, actions, and overlays. | Feature deactivation. |
| Local player | Input, camera, HUD, menu, and player presentation. | Local-player removal. |
| Load request | Shared or private asset handles and verification state. | Request or final dependent release. |
| Task | Native temporary buffers and work products. | Task completion or cancellation. |
| Frame | Scratch values valid only for one declared frame phase. | End of the owning frame phase. |
<!-- markdownlint-enable MD013 -->

An allocation cannot be described only as `temporary`, `level`, `special`,
`other`, or `anywhere`. Its owner and release boundary must be observable.

## Native ownership rules

Use the narrowest native owner that expresses the lifetime:

- reflected gameplay objects use `UObject` ownership and weak references;
- actors and components belong to worlds and explicit gameplay owners;
- exclusive native resources use RAII and `TUniquePtr` where appropriate;
- shared native resources use `TSharedPtr` only when shared ownership is real;
- non-owning references use checked object keys, weak pointers, or stable IDs;
- arrays, maps, sets, queues, and strings use native Unreal containers;
- render, audio, physics, and platform resources use their native release paths;
- asynchronous operations own cancellation-safe move-only handles; and
- asset residency is retained by streamable or bundle handles.

A raw pointer may be a short-lived observation but never the only durable owner.
Manual reference counting is prohibited unless required by a narrow external API
adapter and hidden behind RAII.

## UObject and garbage collection

Reflected object references use supported Unreal properties and collection
contracts. A gameplay object cannot be kept alive by an untracked raw pointer.

Subsystem and feature teardown must:

1. cancel callbacks that could recreate references;
1. release native resources and streamable handles;
1. detach delegates and message subscriptions;
1. clear strong reflected references;
1. invalidate generation or revision tokens; and
1. permit garbage collection at the engine-owned time.

Garbage collection is not a substitute for domain cleanup. Rewards, saves,
reservations, input leases, and event subscriptions reach terminal states before
object reclamation.

## Native containers

Runtime containers use `TArray`, `TMap`, `TSet`, bounded queues, sparse arrays,
or another reviewed Unreal container selected by access pattern.

Container policy declares:

- element identity and ownership;
- expected and maximum cardinality;
- ordering and determinism requirements;
- reserve and growth behavior;
- thread ownership;
- removal and compaction policy; and
- serialization or replication boundary.

A custom linked list, binary map, or source-compatible Standard Library
allocator is not retained merely to preserve implementation shape. Replacement
behavior is verified through domain results and performance evidence.

## Primitive width and alignment

Runtime values use fixed-width or repository-owned semantic types where width,
signedness, serialization, hashing, or network behavior matters.

Unaligned source values are decoded through bounded conversion helpers that
explicitly declare byte order and width. Runtime code does not reinterpret two
32-bit words as a native 64-bit integer or depend on compiler alignment.

Source platform typedefs and integer-size assumptions remain import provenance.
Canonical assets serialize versioned identities and fixed-width values.

## Allocation policy

Ordinary gameplay code does not select a global allocator. It selects an owner,
container, resource type, and bounded lifetime.

Allocation-sensitive code must declare:

- maximum live count or byte budget;
- expected allocation frequency;
- thread and execution phase;
- owner and release boundary;
- failure behavior;
- tracing category; and
- evidence that a custom strategy is necessary.

Per-frame allocation is avoided in measured hot paths through reserve, reuse,
value storage, native scratch facilities, or task-local buffers. The
optimization must not retain stale objects or hide growth.

## Scratch and temporary memory

Native scratch facilities may be used only when their lifetime is shorter than
and fully contained by the documented owner.

A scratch allocation cannot:

- escape to another frame or asynchronous callback;
- contain durable object ownership;
- be released by changing application mode;
- rely on a global push or pop stack; or
- select a different allocator through implicit thread state.

Task-local and frame-local scratch behavior is verified under cancellation,
nested execution, thread migration, and editor multi-world use.

## Memory budget catalog

Every supported target has a memory budget profile. Profiles may vary by target,
quality preset, device class, or package profile without changing gameplay
meaning.

A budget definition contains:

<!-- markdownlint-disable MD013 -->
| Field | Contract |
| :--- | :--- |
| `BudgetId` | Stable namespaced identity. |
| `CategoryId` | Semantic usage category. |
| `ScopePolicy` | Process, game instance, world, session, feature, player, or request. |
| `SoftLimitBytes` | Warning and pressure threshold. |
| `HardLimitBytes` | Maximum accepted product usage when enforceable. |
| `PeakWindow` | Bounded interval for transient peaks. |
| `Priority` | Required, important, optional, or diagnostic. |
| `RecoveryPolicyId` | Permitted pressure response. |
| `TargetPredicate` | Exact supported target and profile condition. |
| `EvidencePolicy` | Required trace, report, and test observations. |
<!-- markdownlint-enable MD013 -->

Budgets are not allocator IDs. Multiple owners and engine systems may contribute
to one semantic budget.

## Initial budget categories

Initial product categories include:

- engine and immutable core;
- game-instance services and root catalogs;
- active world and World Partition residency;
- geometry, textures, materials, shaders, and pipeline caches;
- characters, vehicles, population, animation, and physics;
- mission, progression, navigation, and AI data;
- audio, dialogue, music, media, and streaming buffers;
- UI, fonts, localization, camera, and local-player presentation;
- effects, decals, particles, skid marks, and transient actors;
- save, configuration, package, and network buffers;
- mods and game features; and
- editor and development diagnostics.

Categories may be subdivided by stable identities. They cannot be silently
merged into a miscellaneous heap to hide ownership.

## Target and quality policy

The platform and quality contract declares supported targets and presets. A
lower quality preset may reduce optional visual residency through explicit
native settings, but it cannot change mission, collision, navigation,
progression, or save behavior.

A target profile declares:

- total product memory objective;
- expected operating-system and engine reservation;
- streaming pool and residency policy;
- audio, media, render, and transient headroom;
- peak transition allowance;
- diagnostic-build overhead; and
- representative hardware evidence.

An editor observation is not packaged-target evidence. Native packages are
profiled on representative hardware.

## Pressure states

Memory pressure has closed states:

- `normal`;
- `elevated`;
- `soft_limit_exceeded`;
- `hard_limit_threatened`;
- `allocation_failed`; and
- `fatal`.

Each observation contains target, process, world, session, budget, category,
usage, peak, limit, active request, and trace correlation revisions.

Pressure does not directly change gameplay. The owning recovery policy decides
what optional residency or work may be reduced.

## Pressure response

Permitted recovery actions include:

- cancel speculative prefetch;
- release unreferenced optional bundles;
- reduce optional cache residency;
- flush approved transient presentation caches;
- reject or defer optional diagnostic capture;
- complete world or mode transition cleanup;
- reject a new optional load request;
- return to a safe application mode when required; or
- enter the fatal platform policy.

A response cannot free required active assets, destroy a live world heap, drop
portable state, disable collision, change mission content, or continue after an
unsafe allocation failure.

Each response is revisioned and terminal. It records released handles and the
postcondition read-back.

## Loading and streaming integration

The loading coordinator declares expected peak and steady-state budget effects
for each plan. It may start required work only when its recovery and failure
policy is valid for the target.

Shared load dependencies remain resident until the final dependent handle
releases. Cancellation does not unload an asset still owned by another request.

Transition completion verifies both readiness and accepted memory
postconditions. A source-era mode-specific heap reset is never required.

## Pools and reuse

Custom pooling is permitted only after profiling proves a material benefit and a
native engine facility does not satisfy the need.

A pool definition declares:

- pooled type and stable pool identity;
- maximum live and retained counts;
- construction, reset, and validation policy;
- owner and scope;
- thread policy;
- generation counter;
- overflow behavior;
- pressure and teardown behavior; and
- tests for stale-handle rejection.

A returned object is inaccessible through its prior handle. Generation mismatch,
double return, foreign object, capacity overflow, and use after teardown fail
closed.

Pools cannot retain worlds, players, features, assets, or delegates past their
owners merely to avoid allocation.

## Small allocations

Small-allocation optimization uses the engine allocator and measured container
or object layout first. A dedicated small-object pool requires evidence for size
classes, alignment, contention, fragmentation, peak count, and release
behavior.

A fixed block pool cannot serve arbitrary sizes or infer ownership from address
range alone. Alignment and constructor or destructor behavior remain correct for
every supported type.

## Threading

Memory ownership is thread-safe by construction rather than a global allocator
stack stored in implicit thread-local state.

A cross-thread transfer declares:

- source and destination execution contexts;
- unique or shared ownership semantics;
- synchronization and cancellation;
- object and world validity rules;
- trace identity; and
- terminal release owner.

UObjects follow engine thread restrictions. Native buffers used by render,
audio, physics, or worker tasks follow their subsystem's transfer and fence
contracts.

## Out-of-memory behavior

An allocation failure is not repaired by scanning unrelated heaps for free
space. The allocator and platform adapter report a typed bounded failure.

The out-of-memory path:

1. records allocation size, category, scope, and platform evidence when safe;
1. stops new optional allocations;
1. invokes only predeclared bounded recovery actions;
1. reads back the affected budget and owner state;
1. returns a typed failure to the requesting operation when safe; and
1. enters fatal recovery when runtime integrity cannot be proven.

The handler avoids dynamic formatting, unbounded logging, recursive allocation,
and arbitrary UI construction.

## Memory Trace and Insights

Development and test builds use native Unreal tracing and approved platform
observations. Memory evidence includes:

- allocation and free events where supported;
- low-level memory category and tag;
- call stack or symbol identity under approved capture policy;
- thread, task, world, session, feature, and load-request correlation;
- current, peak, retained, and transient usage;
- fragmentation and allocator evidence when available;
- asset residency and streaming observations; and
- capture build, target, and configuration revision.

Captures use bounded duration, approved destinations, and redaction. Shipping
packages include only explicitly approved low-overhead telemetry.

## Statistics and reports

Repository diagnostic commands may produce:

- target and process memory summary;
- budget usage and headroom;
- active worlds, sessions, features, players, and load requests;
- object and reflected-class counts;
- asset and package residency;
- pool occupancy and generation errors;
- peak transition usage;
- optional cache residency;
- leak-test findings; and
- comparison against a recorded baseline.

Reports use stable type and category identities. Human-readable names do not
become runtime keys.

## Class and instance accounting

Class and instance diagnostics use reflected class identities, native object
iteration, LLM tags, explicit native-type counters, or trace aggregation.

A tracker does not require every constructor and destructor to mutate a global
map keyed by source strings. Diagnostic counters cannot change allocator choice
or object lifetime.

For non-UObject native types, counters are scoped, atomic when required, and
registered through stable diagnostic identities. Underflow, duplicate creation,
and missing destruction produce findings.

## Leak detection

Leak verification runs at explicit boundaries such as:

- feature activation and deactivation;
- world creation and teardown;
- repeated front-end and gameplay transitions;
- local-player add and remove;
- loading cancellation;
- editor play-session teardown;
- automation scenario completion; and
- process shutdown in diagnostic builds.

The test compares owner-scoped object, handle, resource, asset, and allocation
snapshots. Raw addresses may appear inside protected trace evidence but are not
the public identity of a leak.

Expected persistent owners are declared, not excluded through hardcoded heap
numbers. A leak test cannot be disabled silently because tracking itself uses
memory.

## Prop and actor statistics

Presentation and prop diagnostics identify content definition, placement, class,
world, Data Layer, instance count, peak count, estimated native and asset usage,
and active scope.

Statistics are observations only. They do not select a different allocator,
prevent valid spawning, or grant an object a longer lifetime.

Cook and automation checks compare declared placement and population budgets
with runtime observations.

## Diagnostic availability

Heavy memory tracing, object census, allocation call stacks, leak tests, and
interactive reports are editor, automation, development, or authorized
diagnostic features.

Shipping builds exclude unapproved tracking maps, mutable watchers, random
allocation rerouting, raw-address output, and expensive periodic heap scans.

Low-overhead budget and fatal-pressure observations remain available where
needed for product safety.

## Mods and game features

A mod or game feature declares expected steady and peak budgets, package
residency, optional caches, platform predicates, teardown, and tests.

Activation may be rejected when required memory policy is unavailable or the
feature exceeds an accepted hard target. An overlay cannot hide usage in a
base category, retain handles after deactivation, or disable leak and teardown
checks.

## Failure behavior

The memory runtime fails closed on:

- unknown budget, category, scope, pool, or target identity;
- non-finite, negative, overflowed, or contradictory measurements;
- allocation without a valid owner in reviewed runtime code;
- stale object, pool, request, world, or feature handle;
- double free, double return, or ownership cycle;
- scratch memory escaping its lifetime;
- unsafe cross-thread transfer;
- required asset eviction;
- hard-limit violation without an accepted recovery policy;
- allocation failure without a safe terminal result;
- diagnostic capture outside its availability profile; and
- mod teardown that retains memory ownership.

A memory failure cannot silently remove gameplay content or continue after
integrity is uncertain.

## Validation

Build, cook, and catalog validation prove:

- every budget and category identity is unique;
- every target has a complete budget profile;
- required load plans declare peak and steady-state policy;
- every custom pool has measured justification and complete bounds;
- source allocator enums and heap names are absent from runtime authority;
- generated assets contain fixed-width canonical values;
- diagnostic-only tracing is excluded from unauthorized packages;
- feature and mod manifests declare budget and teardown policy;
- no application mode depends on heap creation or destruction; and
- approved platform memory tests have current baselines.

## Tests

Required tests include:

- game-instance, world, session, feature, player, request, task, and frame scope
  teardown;
- repeated boot, front-end, gameplay, pause, demo, race, and exit transitions;
- asset coalescing, cancellation, and final-dependent release;
- garbage collection with weak-reference invalidation;
- container growth, reserve, ordering, and maximum cardinality;
- unaligned and cross-endian source conversion;
- target and quality budget lookup;
- soft and hard pressure response;
- optional residency release without gameplay change;
- allocation failure and fatal recovery;
- pool exhaustion, generation mismatch, double return, and teardown;
- asynchronous cancellation and cross-thread ownership;
- memory trace and report redaction;
- leak detection at world and feature boundaries;
- class, actor, prop, asset, and package census;
- mod activation and deactivation; and
- packaged-target peak and steady-state baselines.

## Invariants

- Memory lifetime follows explicit native owners, not application-mode heaps.
- Global allocator switching is not gameplay architecture.
- Runtime containers use reviewed native ownership and bounded growth.
- Source width, alignment, and heap ordinals are conversion provenance only.
- Budgets are semantic observations, not allocator identities.
- Pressure response cannot change gameplay meaning or destroy required
  residency.
- Pools require measured justification, bounds, generations, and teardown.
- Allocation failure has one typed safe or fatal terminal result.
- Diagnostics never change allocation routing or runtime ownership.
- Every world, feature, player, request, and task releases its owned memory.

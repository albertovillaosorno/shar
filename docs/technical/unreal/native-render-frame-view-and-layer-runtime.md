# Native render-frame, view, and layer runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [World render-entity and physics runtime](world-render-entity-and-physics-runtime.md)

## Purpose

This specification defines native Unreal frame execution, local-player views,
frontend and world presentation scopes, render readiness, loading barriers,
pause and presentation freezes, camera binding, post processing, display policy,
frame telemetry, and teardown.

It replaces one process-wide render-flow singleton, manually ordered manager
updates, ordinal render layers, mutable arrays of raw drawables, custom camera
and
view ownership, explicit begin-view and end-view calls, manual world-render pass
lists, source-section deletion queues, frame-buffer effect callbacks, and
render-state transitions named after object mortality.

The project coordinates accepted application and presentation state. Unreal
Engine owns viewport production, local-player views, scene registration, view
families, renderer passes, resource lifetime, swap or presentation, and final
frame submission.

## Native Unreal foundation

The boundary uses native Unreal facilities:

- `UGameViewportClient` and platform viewport integration;
- `ULocalPlayer` and player-controller view ownership;
- `APlayerCameraManager` and registered camera components;
- Actor and component tick groups plus explicit tick prerequisites;
- native scene, primitive, material, shadow, translucency, and post-process
  submission;
- World Partition, Runtime Data Layers, level instances, and Game Features;
- `UWorldSubsystem`, `UGameInstanceSubsystem`, and local-player subsystems for
  bounded service lifetimes;
- renderer and platform frame timing tools;
- native display, gamma, HDR, resolution, and scalability settings; and
- Common UI and UMG for frontend and in-game interface presentation.

Repository code provides typed requests, readiness barriers, deterministic
arbitration, policy validation, and immutable diagnostics. It does not recreate
a
renderer or a second game loop.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Engine frame loop | Advances worlds, tick groups, cameras, streaming, rendering, and platform presentation. |
| Application-mode coordinator | Owns frontend, loading, gameplay, pause, media, recovery, and shutdown mode transitions. |
| View coordinator | Projects accepted local-player and presentation-camera state into native views. |
| World-composition service | Owns world, region, Data Layer, level-instance, and feature readiness. |
| Renderer | Owns view families, visibility, draw submission, passes, shadows, translucency, post processing, and final output. |
| Presentation service | Owns cosmetic overlays, fades, letterbox, media, world presentation, particles, and lens requests. |
| Display-settings service | Owns accepted gamma, HDR, resolution, window mode, frame pacing, and quality policy. |
| Telemetry service | Owns read-only CPU, GPU, frame, streaming, memory, and presentation measurements. |
| Domain and application services | Own gameplay, missions, progression, persistence, and simulation results. |

<!-- markdownlint-enable MD013 -->

A mode, world, view, primitive, post-process effect, and user-interface layer
can
participate in one frame without sharing authority.

## Runtime identities

The boundary uses stable identities for:

- `FSharFrameId`;
- `FSharFrameRevision`;
- `FSharRenderScopeId`;
- `FSharRenderScopeRevision`;
- `FSharViewId`;
- `FSharViewRevision`;
- `FSharViewportId`;
- `FSharLocalPlayerId`;
- `FSharCameraLeaseId`;
- `FSharWorldCompositionRevision`;
- `FSharPresentationRevision`;
- `FSharDisplayPolicyRevision`;
- `FSharRenderReadinessRevision`;
- `FSharFrameTelemetryId`; and
- `FSharRenderRequestId`.

Layer ordinals, array positions, raw camera or view pointers, drawable
addresses,
source section names, timer callback order, and platform-specific renderer
handles are not durable identity.

## Render-scope definition

`FSharRenderScopeDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ScopeId` | Canonical frontend, loading, gameplay, pause, media, capture, diagnostic, or recovery scope identity. |
| `ApplicationModes` | Accepted modes in which the scope may be active. |
| `WorldPolicy` | Required or prohibited world and composition state. |
| `ViewPolicy` | Eligible local-player, cinematic, capture, mirror, or diagnostic views. |
| `PresentationPolicy` | UI, fade, media, sky, lens, particle, and post-process participation. |
| `SimulationPolicy` | Whether gameplay simulation is active, paused, partially paused, or absent. |
| `InputPolicy` | Input and focus ownership required by the scope. |
| `AudioPolicy` | Mix, pause, ducking, and media behavior. |
| `ReadinessPolicy` | Required assets, cameras, worlds, widgets, and renderer state. |
| `QualityPolicy` | Platform and scalability applicability. |
| `TeardownPolicy` | Cancellation, restoration, and release behavior. |

<!-- markdownlint-enable MD013 -->

Render scopes describe accepted presentation composition. They do not create a
parallel scene, duplicate world entities, or select gameplay rules.

## Frame contract

One engine frame is observed through an immutable `FSharFrameSnapshot`
containing:

- frame and world identities;
- application mode and transition revision;
- real, game, audio, and presentation delta times;
- pause and time-dilation state;
- local-player and active-view revisions;
- world-composition and streaming revisions;
- display and quality revisions;
- required presentation leases;
- previous terminal frame result; and
- diagnostics sampling policy.

The snapshot is evidence, not a mutable global context. Systems consume the
latest compatible snapshot through their native owners.

## Tick groups and dependencies

Per-frame work uses native Actor and component tick groups or subsystem hooks
with explicit prerequisites. The project does not run every manager from one
custom timer callback.

Typical placement is:

- movement and authored pre-physics pose work before physics;
- physics-dependent traces and final collision projections after physics;
- camera-relative or final-transform effects after their required camera and
  target updates;
- user-interface projections from immutable observations without forcing physics
  serialization; and
- telemetry capture at supported engine or platform points.

A dependency is declared only when one task consumes another task's result.
Moving an entire subsystem into a later group merely to imitate historical call
order is rejected unless observable behavior requires it.

Tick order cannot become mission, save, package, or entity identity.

## Application-mode integration

The application-mode coordinator activates render scopes through typed leases.
Frontend, loading, active gameplay, pause, media, recovery, and shutdown each
publish one accepted composition revision.

A mode transition:

1. resolves the target render-scope set;
1. validates world, view, display, UI, media, and asset prerequisites;
1. prepares target scopes without granting active input or gameplay authority;
1. commits the target application mode;
1. activates native views and presentation leases;
1. releases source-only scopes; and
1. verifies the target postcondition.

A render callback, loading animation, completed frame, or visible widget cannot
commit the application transition.

## Native view ownership

Each local player owns an independent native gameplay view through accepted
controller, camera-manager, viewport, and local-player state.

A view snapshot contains:

- view and local-player identities;
- viewport rectangle and safe area;
- camera lease and camera revision;
- world and composition revision;
- view transform, projection, field of view, and aspect policy;
- post-process and exposure policy;
- hidden and visible primitive policy owned by an accepted lease;
- quality and platform policy; and
- diagnostics permissions.

A camera pointer or numeric view index is not identity. Split-screen layout
changes publish new viewport and view revisions.

## Multiple local players

The runtime supports the configured local-player count through independent view
snapshots. One player's camera, culling, lens, HUD, or post-process result
cannot
mutate another player's view.

Shared world rendering may reuse native renderer work where Unreal supports it,
but project policy cannot merge gameplay visibility or camera authority.

A platform or mode that supports fewer local views rejects the incompatible
session before activation. It does not silently discard a player or reuse
another
player's camera.

## Frontend rendering

Frontend rendering uses Common UI, UMG, Slate, native viewport composition, and
optional preview-world or scene-capture content.

The frontend scope declares:

- root screen and layer-stack revisions;
- focus and input ownership;
- background media, preview, or world requirements;
- preview camera and lighting policy;
- safe-area and aspect behavior;
- quality and accessibility policy;
- optional rotating or animated preview entities; and
- teardown behavior.

A decorative model, currency icon, gallery item, or preview vehicle is a
presentation object. It cannot mutate inventory, currency, unlocks, or save
state.

## Gameplay world rendering

Active gameplay renders the accepted world composition and local-player views.
The renderer owns:

- primitive visibility;
- opaque, masked, and translucent submission;
- depth, custom depth, decals, and reflections;
- shadow views and shadow submission;
- LOD, HLOD, Nanite, instance, and material selection;
- sky, atmosphere, cloud, lens, and post-process integration;
- scene captures and mirrors; and
- final view composition.

Repository code configures and validates these facilities. It does not maintain
manual lists for world spheres, shadows, translucent objects, particles, coins,
triggers, artificial-intelligence diagnostics, or lens flares.

## Render readiness

`FSharRenderReadinessSnapshot` contains:

- application mode and render-scope revisions;
- world and composition readiness;
- local-player and view readiness;
- camera lease readiness;
- required asset and construction results;
- UI and media readiness;
- display-policy readiness;
- renderer resource readiness exposed through supported engine state;
- optional presentation fallbacks; and
- blocking findings.

`visible` is not equivalent to `ready`. A black, empty, or transitional frame
may
be valid while a committed loading or fade scope owns presentation.

Gameplay activation requires its complete application readiness barrier, not
merely one rendered world frame.

## Static and dynamic content barriers

Static world admission, region streaming, Runtime Data Layer changes, and
feature
activation use typed barriers shared with the world-composition and construction
services.

A barrier validates:

- exact world and region revision;
- required cooked assets and retained handles;
- successful native object construction;
- scene, collision, navigation, and gameplay registration;
- required cameras and presentation assets;
- local-player bindings; and
- cancellation or replacement state.

Source sections and source load-zone names are conversion provenance only.
Runtime cannot delete or redirect arbitrary source inventories to satisfy a
render-layer transition.

## Dynamic unload and replacement

Dynamic unload follows one correlated transaction:

1. stop new owner requests;
1. mark the exact region or feature revision as retiring;
1. cancel owned construction and presentation work;
1. remove gameplay, collision, navigation, and query registrations;
1. unregister native components and Actors;
1. release retained handles and packages; and
1. publish the terminal composition revision.

Deferred deletion may use engine-supported object lifetime. A custom raw-pointer
delete list cannot outlive the revision that created it.

## Pause and presentation freeze

Pause, media playback, photo-like inspection, loading overlays, and transition
barriers may request a presentation freeze through a typed scoped lease.

A freeze definition declares:

- affected application and render scopes;
- simulation pause or partial-pause policy;
- camera and input behavior;
- world and UI presentation allowed to continue;
- audio behavior;
- particle and animation time policy;
- timeout and cancellation policy; and
- exact restoration owner.

Freezing presentation never serializes arbitrary renderer objects or changes
persistent gameplay state. Releasing the lease restores only state owned by that
lease and only when the retained world and mode revisions still match.

## Render-scope activation states

The closed scope states are:

- `inactive`;
- `preparing`;
- `ready`;
- `active`;
- `suspended`;
- `retiring`;
- `failed`; and
- `released`.

Terms such as dead, corpse, frozen, chilled, warm, or resurrected remain source
implementation evidence. They do not appear in public runtime state or
telemetry.

Every accepted scope transition has a request identity, expected revision,
terminal result, and rollback or compensation behavior.

## Camera and view binding

A render scope receives cameras through registered camera leases. It cannot own
a
raw camera object independently from the camera subsystem.

Binding validates:

- local-player or presentation owner;
- world and camera revisions;
- viewport compatibility;
- aspect, field-of-view, and projection policy;
- post-process compatibility;
- streaming and target readiness; and
- cancellation state.

Camera cuts, blends, and replacements publish new view revisions. A late render
or
post-process callback cannot restore a prior camera.

## User interface composition

Frontend and in-game UI use native layer stacks and per-local-player viewmodels.
The renderer presents accepted widget state but does not own navigation, focus,
mission results, currency, prompts, pause, or settings transactions.

World rendering and UI composition remain separable. A platform may composite UI
through supported renderer paths without making widgets scene primitives or
world entities.

## Sky, world, lens, and transient effects

Sky, atmosphere, clouds, world backgrounds, lens effects, particles, decals, and
breakable presentation follow their dedicated definitions and native engine
facilities.

They consume accepted world, camera, impact, vehicle, weather, and presentation
snapshots. They cannot:

- define application mode;
- commit a mission or breakage result;
- change world streaming;
- become gameplay visibility authority;
- share per-view state across unrelated cameras; or
- keep a retired world alive through an unowned reference.

## Mood and lighting presentation

A lighting presentation request contains:

- world and presentation revisions;
- target light, sky, exposure, color, and material policy;
- blend curve and duration in seconds;
- quality and platform applicability;
- owner and priority;
- cancellation and restoration behavior; and
- validation targets.

Lighting transitions use native light, material, post-process, sky, and exposure
facilities. Integer modulus tricks, platform-specific color accumulation, or
frame-count interpolation are not public behavior.

Lighting presentation cannot alter gameplay time, weather authority, mission
state, or physical visibility.

## Display, gamma, HDR, and resolution

Display policy is owned by the settings and platform boundary. A validated
`FSharDisplayPolicySnapshot` contains:

- display and device identity;
- window mode and resolution;
- refresh and frame-pacing policy;
- gamma or brightness setting;
- SDR or HDR policy;
- color-space and output-format policy;
- quality preset;
- accessibility projection; and
- platform capability evidence.

Settings apply through supported engine and platform facilities. Per-channel
platform gamma hacks, raw renderer state mutation, or hidden hardware-specific
values are not portable runtime contracts.

A settings transaction reads back the accepted result and exposes recovery when
a
display mode cannot be sustained.

## Motion blur and post processing

Motion blur, bloom, exposure, depth of field, color grading, outlines, and other
post-process features use native Unreal view and post-process policy.

Each feature declares:

- visual purpose;
- applicable cameras and modes;
- quality and platform availability;
- intensity and temporal behavior;
- accessibility restrictions;
- camera-cut and pause behavior;
- performance budget; and
- fallback.

Frame-rate compensation uses time-based engine behavior or validated native
parameters. Project code does not rewrite blur strength from ad hoc average
frame
samples to imitate one platform path.

## Frame timing and telemetry

Frame telemetry is read-only and revisioned. `FSharFrameTelemetrySnapshot` may
contain:

- game-thread, render-thread, RHI, and GPU timings exposed by supported tools;
- frame, simulation, presentation, and swap intervals;
- view count and viewport layout;
- primitive, instance, draw, shadow, and translucency statistics;
- streaming and construction activity;
- Niagara and transient-effect activity;
- memory and residency evidence;
- quality, display, and platform revisions; and
- sampling uncertainty.

Telemetry collection cannot change tick order, renderer state, simulation time,
quality, or content. Debug overlays are optional presentation consumers of the
snapshot.

## Frame pacing

Frame pacing and caps follow platform and project settings. A render service
does
not own a busy timer or invoke rendering from a timer-completion callback.

The policy declares:

- target and maximum rates;
- VSync and presentation behavior;
- foreground and background policy;
- loading and media exceptions;
- platform-specific constraints; and
- diagnostics thresholds.

Missed targets produce telemetry and quality findings. They do not skip required
simulation, mission, save, or streaming transactions.

## Animated presentation updates

Animated environmental entities and stateful props update through their native
components, Animation Blueprints, sequences, or registered tick functions.

A world subsystem may index weak accepted identities for diagnostics or bounded
coordination, but it cannot:

- use fixed process-wide capacities as content limits;
- own raw animation-controller pointers;
- advance all animations manually in one arbitrary list;
- select state from source object names; or
- retain retired world entities.

Birds, crowds, props, and other repeated animation families use definition-owned
behavior, native instancing where valid, and deterministic placement identities.

## Frontend-to-world transition

The frontend and gameplay world may overlap temporarily during an accepted
transition, but each keeps independent scope, view, asset, and input ownership.

The transition declares:

- source and target scopes;
- loading and fade barriers;
- preview-world or active-world lifetime;
- UI and media handoff;
- local-player and controller binding;
- memory peak budget;
- cancellation and recovery; and
- final target postcondition.

No drawable or camera is moved between unrelated worlds through a raw pointer.
The target constructs or binds its own native objects.

## Local multiplayer

Local multiplayer maintains independent camera, UI, lens, and view policy per
participant. Shared world, lighting, and streaming policy may use conservative
unions of required views where supported.

A presentation freeze, camera cut, debug view, or post-process override names
its
eligible players. An unscoped global override is rejected.

## Community networking boundary

The dedicated or listen-server gameplay authority does not need a renderer.
Headless execution uses no-render representations while preserving world,
collision, mission, traffic, route, and persistence semantics.

Clients render replicated or locally predicted accepted state through ordinary
world entities and presentation requests. Renderer completion, particle
playback,
or view visibility never confirms a networked gameplay transaction.

## Platform and quality policy

Low through Ultra may vary:

- material and shader quality;
- shadow methods and resolution;
- LOD, HLOD, Nanite, and instance policy;
- post processing;
- sky, cloud, lens, and particle complexity;
- reflection and capture quality;
- resolution scale; and
- optional diagnostics.

They cannot vary canonical world identity, collision, navigation, simulation,
mission logic, route connectivity, persistence, or required feedback semantics.

Android uses the accepted Low-only policy and validated mobile renderer path.
Desktop-specific renderer assumptions cannot leak into mobile gameplay.

## Game Features and mods

A validated feature may register namespaced render-scope overlays, presentation
definitions, post-process policy, diagnostic views, and native content.

Activation validates:

- namespace and priority;
- supported application modes;
- required assets and constructors;
- platform and quality support;
- compatibility with base view and display policy;
- teardown completeness; and
- absence of renderer or gameplay authority replacement.

Removal cancels owned requests, restores owned scoped overrides, releases
presentation objects and handles, and unregisters definitions atomically.

## Concurrency

Application, world, game-thread, render-thread, RHI, GPU, and
asynchronous-loading
state communicate only through supported engine boundaries and immutable project
snapshots.

A render-thread callback cannot call gameplay services. A game-thread subsystem
cannot read renderer-private mutable storage. Async work carries frame, world,
view, owner, and request revisions and is discarded when stale.

## Diagnostics

Development diagnostics may expose:

- application mode and render-scope graph;
- active local-player views and camera leases;
- viewport layout and safe areas;
- readiness barriers and blocking dependencies;
- world, Data Layer, feature, and construction revisions;
- pause and presentation-freeze leases;
- display, gamma, HDR, resolution, quality, and frame-pacing policy;
- frame and subsystem timings;
- renderer statistics exposed by supported engine tools;
- stale callback and rejected transition counts; and
- teardown ownership.

Diagnostics are read-only and cannot force a scope active, hide content, change
a
camera, or alter a frame result.

## Failure behavior

The boundary fails closed on:

- unknown or duplicate render-scope identity;
- unsupported application-mode and scope combination;
- missing world, view, camera, UI, media, or asset dependency;
- stale world, view, camera, display, feature, or request revision;
- unsupported local-player count or viewport layout;
- scope activation before readiness;
- manual manager-loop or timer-driven rendering authority;
- raw drawable, camera, view, or renderer-resource ownership;
- source-section mutation in packaged runtime;
- unscoped global pause, post-process, gamma, or visibility override;
- renderer callback attempting domain mutation;
- dynamic unload with registered objects or retained handles;
- feature removal with owned scopes or presentation objects; and
- headless mode requiring renderer completion.

Failure returns typed evidence, restores the last accepted scope set or declared
recovery scope, and releases prepared resources.

## Validation

Definition and configuration validation prove:

- every scope, view, display, and presentation identity resolves;
- application-mode compatibility is explicit;
- local-player layouts are supported by each target;
- native tick groups and prerequisites cover required update order;
- no custom timer or manager loop owns frame execution;
- world and frontend scopes have complete readiness and teardown;
- display and quality policies use supported engine and platform facilities;
- renderer passes remain engine-owned;
- pause and presentation freezes restore only owned state;
- headless execution preserves gameplay semantics without renderer dependencies;
- feature overlays are namespaced and removable; and
- diagnostics are read-only.

## Tests

Required automated and visual tests include:

- frontend activation and teardown;
- loading barrier success, cancellation, timeout, and recovery;
- gameplay scope activation only after complete readiness;
- split-screen view and camera isolation;
- camera replacement and stale-callback rejection;
- pause, partial pause, media freeze, and restoration;
- static and dynamic region activation and unload;
- feature activation and removal with zero retained scope state;
- frontend-to-world transition memory and cancellation behavior;
- native tick-order and prerequisite verification;
- no manual manager-loop frame authority;
- independent world, UI, lens, and particle presentation;
- display-setting apply, read-back, rejection, and recovery;
- motion-blur and post-process behavior across frame rates;
- Low through Ultra visual-policy behavior;
- Android Low viewport and frame-pacing behavior;
- headless runtime without renderer dependencies;
- telemetry with no observable behavior change; and
- world teardown with no stale camera, view, Actor, component, or handle.

## Invariants

- Unreal Engine owns frame execution and final rendering.
- Application modes own render-scope activation through typed transactions.
- Every local player has an independent accepted view and camera lease.
- Renderer passes and scene submission are never manual project lists.
- Native tick groups and prerequisites replace one custom manager-update loop.
- Visible does not mean ready, active, complete, or persisted.
- Pause and presentation freezes are scoped, revisioned, and reversible.
- Display and quality settings cannot change gameplay semantics.
- Renderer and presentation callbacks cannot mutate domain state.
- Headless gameplay never depends on renderer completion.
- Every retired scope releases all owned views, presentation state, and handles.

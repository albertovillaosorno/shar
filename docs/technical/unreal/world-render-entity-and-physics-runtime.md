# World render-entity and physics runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
- [Spatial visibility, bounds, and culling
  runtime](spatial-visibility-bounds-and-culling-runtime.md)
- [Physical material and impact-response
  runtime](physical-material-and-impact-response-runtime.md)
- [Persistent world-object state
  runtime](persistent-world-object-state-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission world-entity and respawn runtime](mission-world-entity-and-respawn-runtime.md)
- [Native asset load request and streaming
  runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native render-frame, view, and layer runtime](native-render-frame-view-and-layer-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md)

## Purpose

This specification defines the native Unreal boundary for world render entities,
static and dynamic collision, rigid-body simulation, animated physical props,
instancing, breakage, query-only surfaces, shadows, streaming registration, and
component lifecycle.

It replaces one drawable-scene-graph inheritance hierarchy, process-wide entity
factories, raw drawable and simulation pointers, manual reference counting,
custom simulation admission lists, pooled ground planes, render-owned collision
responses, source-triangle parsing at runtime, and entity identity derived from
a
class or memory address.

A render component presents accepted state. A physics body simulates accepted
physical state. Neither owns progression, mission completion, persistence,
streaming admission, rewards, interaction authority, or application mode.

## Native Unreal composition

The runtime uses native Unreal facilities:

- `AActor` for world lifetime and transform ownership where an actor boundary is
  required;
- `USceneComponent` for component hierarchy and transform attachment;
- `UPrimitiveComponent` for render bounds, collision, overlap, scene queries,
  materials, shadows, and render-state integration;
- `UStaticMeshComponent` for one static or movable mesh representation;
- `UInstancedStaticMeshComponent` or
  `UHierarchicalInstancedStaticMeshComponent` for measured repeated-mesh cases;
- `USkeletalMeshComponent`, Animation Blueprints, and optional Control Rig for
  articulated animated props;
- Chaos rigid-body simulation and physical materials;
- Geometry Collections or a registered project-owned breakable adapter when the
  authored destruction contract requires them;
- typed collision and physics observations;
- World Partition and Runtime Data Layers for world lifetime; and
- Asset Manager bundles and retained load handles for required assets.

Repository code composes and validates these facilities. It does not create a
parallel scene graph, renderer, collision solver, or rigid-body engine.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| World-entity application service | Validates spawn, activation, replacement, destruction, persistence, and teardown commands. |
| Actor | Owns world lifetime, transform root, replication policy when applicable, and component composition. |
| Primitive component | Owns renderer registration, conservative bounds, material slots, collision profile, query flags, and shadow flags. |
| Chaos physics | Owns accepted rigid-body state, contacts, constraints, forces, impulses, sleep, wake, and solver integration. |
| Physical-profile service | Owns mass, density, friction, restitution, impact classification, and response policy. |
| Spatial visibility runtime | Owns renderer-facing bounds policy, view visibility, and converted spatial diagnostics. |
| World-composition service | Owns streaming admission, World Partition readiness, Runtime Data Layers, and region teardown. |
| Persistence service | Owns durable destroyed, collected, moved, unlocked, and replaced state. |
| Mission and interaction services | Own gameplay eligibility and consume verified entity or collision results. |
| Presentation service | Owns cosmetic effects, audio, animation, and camera feedback requested from accepted results. |

<!-- markdownlint-enable MD013 -->

Rendering, collision, physics, persistence, streaming, and gameplay activation
are
separate authorities even when one Actor contains components for several of
them.

## Runtime identities

The boundary uses stable identities for:

- `FSharWorldEntityId`;
- `FSharWorldEntityRevision`;
- `FSharPlacementId`;
- `FSharRepresentationId`;
- `FSharRepresentationRevision`;
- `FSharPhysicsBodyId`;
- `FSharPhysicsBodyRevision`;
- `FSharCollisionProfileId`;
- `FSharPhysicalProfileId`;
- `FSharBreakableDefinitionId`;
- `FSharStatefulPropDefinitionId`;
- `FSharStatefulPropRevision`;
- `FSharInstanceGroupId`;
- `FSharInstanceId`;
- `FSharQuerySurfaceId`;
- `FSharSceneQueryId`;
- `FSharSceneQueryRevision`;
- `FSharLensPresentationId`;
- `FSharWorldPresentationId`;
- `FSharWorldCompositionRevision`; and
- `FSharEntityRequestId`.

Actor pointers, component pointers, primitive indices, simulation handles,
instance array positions, material pointers, source class names, and memory
addresses are not durable identity.

## Definition contract

`FSharWorldRenderEntityDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `EntityId` | Canonical entity definition identity. |
| `RepresentationKind` | Static mesh, instanced mesh, skeletal prop, collision-only surface, fence, registered composite, or no-render server representation. |
| `AssetSetId` | Required mesh, material, animation, physics, collision, shadow, and effects assets. |
| `MobilityPolicy` | Static, stationary where supported, movable, or simulated. |
| `CollisionProfileId` | Registered object type, response matrix, query, overlap, and simulation policy. |
| `PhysicalProfileId` | Mass, density, friction, restitution, damping, thresholds, and force policy. |
| `BoundsPolicyId` | Conservative bounds and optional validated extension policy. |
| `ShadowPolicyId` | Native dynamic, static, virtual, contact, capsule, or disabled policy. |
| `BreakableDefinitionId` | Optional destruction and replacement transaction. |
| `StatefulPropDefinitionId` | Optional accepted-state, animation, collision, visibility, and event projection. |
| `SceneQueryPolicyId` | Optional native trace, overlap, surface, road, path, and line-of-sight query policy. |
| `LensPresentationId` | Optional per-view lens, flare, bloom, and occlusion presentation policy. |
| `WorldPresentationId` | Optional sky, atmosphere, horizon, dome, cloud, and world-background policy. |
| `PersistencePolicyId` | Durable state ownership and migration behavior. |
| `StreamingPolicyId` | Region, bundle, readiness, retention, and teardown behavior. |
| `QualityPolicyId` | LOD, HLOD, Nanite, instancing, shadow, and effect policy by target. |
| `DiagnosticsPolicyId` | Development visualization and capture permissions. |

<!-- markdownlint-enable MD013 -->

Definitions reject missing assets, contradictory mobility and simulation,
query-only geometry configured to simulate, mutable per-instance behavior that
cannot be represented by the chosen instancing type, collision without a
profile,
and destruction without a complete replacement and persistence path.

## Representation classes

The closed representation classes are:

- `static_render`, for immutable world geometry without a distinct physics body;
- `static_collision`, for static render geometry with registered collision;
- `repeated_static`, for measured identical placements using native instancing;
- `movable_rigid`, for a movable Chaos rigid body with one primary visual;
- `animated_visual`, for an animated presentation without authoritative physics;
- `animated_collision`, for authored animation with synchronized cooked
  collision;
- `animated_rigid`, for an animated or articulated prop with bounded physics;
- `stateful_prop`, for a typed accepted-state projection across presentation,
  collision, and physical channels;
- `linear_blocker`, for authored fence or barrier geometry;
- `query_surface`, for collision or scene-query geometry without ordinary visual
  presentation;
- `breakable_composite`, for a registered destruction representation;
- `world_presentation`, for world-scoped sky, atmosphere, horizon, or dome
  presentation;
- `lens_presentation`, for per-view lens, bloom, flare, and occlusion
  presentation; and
- `registered_composite`, for a validated component assembly not covered by the
  simpler classes.

A class selects component composition. It does not select mission, reward,
interaction, or persistence behavior.

## Entity construction transaction

Construction follows one correlated transaction:

1. resolve the canonical definition and placement;
1. validate world, composition, feature, and owner revisions;
1. acquire required Asset Manager bundles;
1. validate mesh, material, collision, physics, animation, and breakable assets;
1. create the Actor and components in an inactive prepared state;
1. apply transforms, materials, collision profiles, physical profiles, and
   representation policy;
1. verify conservative bounds and component hierarchy;
1. register streaming, persistence, and gameplay adapters without activation;
1. atomically commit the accepted entity revision;
1. register components and optionally enable simulation; and
1. publish the immutable active snapshot.

Any failure before commit destroys the prepared Actor, unregisters adapters,
releases assets, and leaves the previous accepted entity unchanged.

A global factory singleton cannot allocate arbitrary subclasses from raw
drawables. Construction uses a closed definition registry and project-owned
application port.

## Component hierarchy

One Actor has one declared transform root. Child components use checked socket,
bone, or scene-component attachment identities.

The hierarchy records:

- root and child component identities;
- relative transforms and attachment rules;
- mobility compatibility;
- collision ownership;
- bounds contribution policy;
- visibility and shadow policy;
- animation or physics ownership; and
- teardown order.

Two components cannot both claim the same authoritative rigid body, root
transform, or interaction reservation. Changing the hierarchy creates a new
representation revision.

## Static render entities

A static render entity uses one or more static mesh components with static
mobility unless the definition requires a movable but non-simulated prop.

The accepted component state includes:

- mesh and material revisions;
- transform and placement identity;
- LOD, HLOD, Nanite, distance, and culling policy;
- collision enabled state and response profile;
- decal, lighting, reflection, and shadow flags;
- custom primitive data schema; and
- bounds and render-state revision.

Material translucency, shader identity, or draw cost never becomes entity type
or
world identity. Unreal's mesh drawing pipeline owns batching and final pass
submission.

## Procedural and diagnostic geometry

Mutable triangle strips, immediate-mode vertex arrays, and source primitive
groups
are not ordinary shipping world representations. Required world geometry is
cooked into native mesh assets during import.

A runtime-generated mesh is allowed only for a registered bounded use case such
as diagnostics, editor visualization, or genuinely dynamic geometry that cannot
be authored or cooked. Its definition declares:

- owner and lifetime;
- vertex, index, section, and material limits;
- collision policy;
- bounds update policy;
- game-thread and render-thread ownership;
- platform and quality support;
- deterministic generation inputs; and
- teardown behavior.

Development bounding boxes, query triangles, roads, paths, and collision volumes
use engine debug drawing or dedicated diagnostic components. They cannot become
shipping content accidentally or provide gameplay authority through rendering.

## Repeated static instances

Repeated identical meshes may use ISM or HISM only after profiling and
validation
prove that grouping improves the target platform without violating semantics.

An instance group declares:

- one mesh and component-level material set;
- component-level collision and shadow policy;
- stable instance identities mapped to current engine indices;
- per-instance transforms and bounded custom data;
- LOD and culling policy;
- mutation and removal frequency;
- persistence requirements; and
- fallback to individual Actors or components.

The stable identity-to-index map is revisioned after add, remove, compaction, or
rebuild. A saved instance index is invalid.

HISM is reserved for large predominantly static groups where its hierarchy and
LOD behavior are measured as beneficial. ISM is preferred for Nanite-only groups
or frequently changing groups when validated. Per-instance gameplay state that
requires independent collision, physics, networking, or persistence normally
uses individual entities.

## Static collision entities

Static collision entities use a static mesh component or registered simple
collision components. They declare whether collision uses authored simple
shapes, complex query geometry, or a validated combination.

Static collision cannot be moved by collision callbacks. A request to replace,
move, disable, or destroy it goes through the world-entity application service
and creates a new accepted revision.

Collision geometry and visual geometry may differ. Their source identities,
transforms, material bindings, and validation evidence remain correlated.

## Dynamic rigid bodies

A movable rigid entity uses Chaos simulation through a primitive component with
an accepted physics body setup.

Its definition includes:

- initial transform, linear velocity, and angular velocity;
- mass or density policy;
- center-of-mass and inertia policy;
- linear and angular damping;
- gravity, continuous-collision, and sub-stepping requirements;
- sleep family and wake thresholds;
- collision profile and physical material;
- maximum force, impulse, and speed policy;
- out-of-bounds recovery; and
- persistence and streaming behavior.

Application code does not maintain a parallel position matrix or physics object
pointer. The primitive component and Chaos body provide the current accepted
physical state.

## Animated and articulated physical props

An animated physical prop combines a skeletal mesh, Animation Blueprint,
optional
Control Rig, Physics Asset, and registered rigid-body or physical-animation
policy.

The definition declares:

- animation state and action identities;
- authored animation-to-physics handoff;
- kinematic, simulated, or blended body sets;
- root transform and pose authority;
- collision and query bodies;
- breakable or detach behavior;
- representation LOD support; and
- cancellation and restoration behavior.

Animation cannot write a second transform over an actively simulated body
without
a declared handoff. Physics cannot silently override an authoritative animation
state. A missing Physics Asset, bone, body, controller, or animation binding
fails
or uses an explicit non-simulated fallback.

## Animated visual entities

An animated visual entity uses a skeletal mesh, component animation, Level
Sequence binding, Animation Blueprint, or registered native controller without
an
authoritative rigid body.

Its definition declares:

- animation asset and controller identities;
- deterministic initial phase or accepted seed policy;
- loop, pause, resume, reset, and terminal behavior;
- transform and root-motion authority;
- optional particle, audio, and material channels;
- conservative animated bounds;
- visibility and quality policy; and
- streaming and teardown behavior.

Animation start phase derives from stable definition, placement, world, and seed
identities. Load timing, frame rate, and global random-call order cannot alter
the
selected phase.

An animated visual may request presentation effects from authored markers. It
cannot publish gameplay events, move an authoritative collision body, grant a
reward, or persist state merely because a frame or loop completed.

## Animated collision entities

An animated collision entity synchronizes authored animation with cooked native
collision under one accepted pose and physics revision. The definition declares:

- visual and collision component identities;
- authoritative pose source;
- collision-body or Physics Asset bindings;
- kinematic, query-only, overlap, or simulated policy;
- body and shape enablement by accepted animation state;
- transform and bounds update policy;
- contact observation policy; and
- fallback when a required bone or collision shape is unavailable.

Collision updates follow supported Unreal component or Physics Asset paths. A
runtime cannot hand-edit source collision volumes, retain a second pose tree, or
update one body from an uncorrelated animation callback.

## Stateful props

A stateful prop projects one accepted domain or application state across render,
animation, collision, physics, audio, effects, and interaction channels. It does
not own the state transition authority.

`FSharStatefulPropDefinition` contains:

- canonical prop and state identities;
- initial-state policy;
- allowed transition graph;
- per-state representation and material bindings;
- per-state animation, collision, simulation, and visibility policy;
- typed input observations that may propose a transition;
- typed terminal results expected from the owning service;
- breakage and replacement policy;
- persistence and respawn policy; and
- accessibility, quality, streaming, and teardown behavior.

The closed projection states may include idle, entering, exiting, moving,
charging, charged, attacking, hit, destroyed, and other definition-owned values.
Source enum positions are conversion evidence only.

A collision, animation marker, player action, or event observation proposes
work.
The owning mission, interaction, damage, or world-object service validates and
commits the transition. The prop then applies the accepted state revision.

Visibility, collision-body enablement, physical simulation, and animation are
committed together or compensated together. A late animation, collision, event,
or save callback cannot apply an older state to a replacement prop.

Stateful prop presentation cannot generate coins, rewards, progression, mission
completion, or persistent destruction directly. Those effects require typed
application transactions and exactly-once result identities.

## World sky and background presentation

World-scoped sky, atmosphere, cloud, horizon, dome, and background presentation
uses native Unreal world and lighting components selected by an accepted visual
profile.

A world-presentation definition declares:

- world and presentation identities;
- sky, atmosphere, cloud, light, material, and optional animation assets;
- time-of-day and weather input projections;
- transform and camera-relative policy;
- quality and platform variants;
- bounds or always-visible policy where native components require it;
- activation, blending, and replacement behavior; and
- teardown and restoration behavior.

The presentation consumes accepted world-clock, weather, chapter, and visual
state. It cannot become their authority. One camera-relative dome or composite
cannot define world position, streaming readiness, mission state, or save state.

## Lens and flare presentation

Lens presentation is per accepted view. It prefers Unreal camera and
post-process
lens facilities when they satisfy the authored contract. A custom flare adapter
requires a registered definition and explicit evidence that native facilities
are
insufficient.

`FSharLensPresentationDefinition` declares:

- source light or emitter identity;
- eligible camera and viewport classes;
- bloom, flare, dirt-mask, and color policy;
- screen-space and world-space placement policy;
- bounded occlusion or visibility evidence;
- temporal smoothing in seconds rather than per-frame increments;
- split-screen, scene-capture, cinematic, and editor behavior;
- accessibility and photosensitivity policy;
- quality and platform fallback; and
- cancellation and teardown behavior.

Occlusion results are view-local and revisioned. One process-global framebuffer
read-back queue, shared billboard intensity, or fixed flare array cannot control
multiple local players, shadow views, scene captures, or cinematic cameras.

A delayed occlusion result may affect presentation intensity only after view,
world, emitter, and definition revisions are revalidated. It cannot hide the
source entity, alter gameplay visibility, or publish a domain result.

## Linear blockers and fences

A fence or linear blocker converts authored endpoints, height, thickness,
normal,
collision profile, physical material, and optional visual representation into
native components.

A zero-thickness source line receives a declared conservative collision shape.
The generated shape is deterministic and validated against navigation, vehicle,
character, and camera requirements.

A fence collision callback may publish an impact observation. It cannot apply
hidden gameplay effects or use inheritance solely to borrow friction settings.

## Query and intersection surfaces

Query-only surfaces support line, box, sphere, capsule, and registered overlap
or
sweep queries through Unreal collision channels and object types.

Converted triangle evidence records:

- source mesh and primitive-group identity;
- source and normalized transforms;
- triangle and material-slot identity;
- collision cooking policy;
- physical surface binding;
- terrain and interior classification;
- bounds and provenance; and
- conversion findings.

Shipping runtime does not parse triangle strips, allocate custom flat-triangle
arrays, or rebuild collision fields from source geometry. Cooked collision
assets
are produced during import.

Queries return engine hit results normalized to typed project results. A query
surface does not render itself merely to prove collision and does not become
mission or interaction authority.

## Native scene-query service

`USharSceneQuerySubsystem`, a world subsystem, normalizes native Unreal traces,
overlaps, sweeps, navigation projections, and registered road or path lookups.
It
replaces one process-wide intersection singleton and mutable same-frame caches.

`FSharSceneQueryRequest` contains:

- query and owner identities;
- world and composition revisions;
- participant or source entity revision;
- line, shape, overlap, closest-surface, closest-road, or line-of-sight kind;
- start, end, center, rotation, extent, and radius as applicable;
- trace channel, object types, and collision profile;
- ignored canonical entity identities;
- complex-versus-simple policy;
- physical-surface and terrain filters;
- maximum result count and deterministic ordering;
- synchronous or approved asynchronous execution policy; and
- diagnostics context.

A raw pointer to an object to avoid is converted to a checked canonical identity
and weak runtime binding before execution.

## Scene-query result

`FSharSceneQueryResult` contains:

- query, owner, world, and scene-query revisions;
- terminal status;
- ordered hit, overlap, road, path, or surface results;
- canonical entity and component identities;
- impact point, normal, distance, time, and penetration evidence;
- physical material, surface, terrain, and interior classification;
- blocking and overlap classification;
- native face or item index only as transient diagnostics;
- truncation or fallback evidence; and
- execution cost and findings.

Results are immutable. Native component pointers, face indices, and hit-array
positions are not durable identity.

## Closest road and path queries

Closest-road and closest-path requests use the accepted road, path, and world
registries plus native spatial or navigation facilities. They declare eligible
road classes, direction, lane, mission, interior, and participant filters.

Equal-distance candidates resolve by declared priority, canonical road or path
identity, and segment identity. Mutable insertion order or first pointer found
in
a tree cannot determine the result.

A closest-road result is query evidence. It cannot move a vehicle, activate a
mission route, or change traffic state without a typed owner transaction.

## Line-of-sight queries

Line-of-sight requests declare whether height is respected, which collision
channels and object types participate, ignored identities, maximum range, and
whether transparent or query-only surfaces are eligible.

The result distinguishes clear, blocked, invalid, cancelled, and unavailable.
Two-dimensional horizontal tests are a separate explicit query kind; they are
not
a hidden flag that silently discards height.

Line-of-sight evidence may inform artificial intelligence, cameras, interaction,
or presentation. It cannot directly mutate those systems.

## Query caching

Query caching is optional, bounded, and keyed by the complete normalized
request,
world collision-scene revision, query-policy revision, and execution frame or
simulation step where appropriate.

A cache entry stores immutable result data, not raw triangle or entity pointers.
It is invalidated by relevant component movement, collision-profile changes,
world streaming, Data Layer transitions, feature removal, or scene-query
revision.

A same-position or same-radius heuristic without world and filter revisions is
not a valid cache key. Cache hits and misses cannot change result ordering or
terminal status.

## Query concurrency and budgets

Synchronous queries run only on supported engine threads and within declared
per-system budgets. Approved asynchronous queries carry copied request data and
revalidate the owner and world before publication.

Each caller declares maximum hits, maximum candidate count, timeout, and
fallback.
A fixed reserve array cannot silently truncate candidates without typed
evidence.

Development diagnostics may compare native results with converted spatial
evidence, but cannot publish a second collision authority.

## Collision profiles

Every collidable component uses a registered collision profile. The profile
specifies:

- object type;
- block, overlap, and ignore responses;
- query-only, physics-only, query-and-physics, or disabled mode;
- trace and object-channel participation;
- overlap generation policy;
- complex-as-simple restrictions;
- character, vehicle, projectile, camera, interaction, and world responses; and
- feature and platform availability.

Direct per-object response mutation is allowed only through a typed, scoped, and
revisioned policy change. Teardown or owner replacement restores the accepted
profile rather than a cached pointer state.

## Simulation admission

Simulation admission is explicit and idempotent. Enabling simulation validates:

- active world and entity revisions;
- movable mobility;
- valid body setup and collision geometry;
- accepted physical profile;
- streaming readiness;
- solver and scene availability; and
- absence of a conflicting authoritative transform owner.

Disabling simulation records the final accepted transform and velocities before
applying the definition's kinematic, persistence, or teardown policy.

No manual simulation list is authoritative. Chaos owns solver membership after
component registration and simulation enablement.

## Sleep, wake, and rest

Chaos sleep and wake state is the physical authority. Project policy may tune
supported sleep families and consume sleep or wake observations.

A gameplay service cannot declare rest by averaging arbitrary frame velocities
and removing the body from simulation. If a gameplay rule requires a stable-rest
condition, it consumes a typed projection containing:

- body and entity revisions;
- Chaos sleep state;
- bounded linear and angular velocity evidence;
- contact and support evidence;
- required stable duration; and
- timeout or fallback policy.

Sleep never means destroyed, collected, persisted, unloaded, or unavailable.
Wake events caused by collision, force, transform, streaming, or explicit
command
remain revision-correlated.

## Ground support

Ground support is derived from Chaos contacts, movement-floor results, or
bounded
scene queries appropriate to the entity kind. The runtime does not allocate one
synthetic ground plane from a global pool for each object.

A support snapshot contains:

- entity and body revisions;
- support component and physical surface identities;
- contact point and normal;
- separation or penetration evidence;
- relative velocity;
- query or contact revision; and
- confidence and fallback policy.

Losing support may affect a typed gameplay or presentation state only through
its
own owner. It does not directly force the body into or out of simulation.

## Forces and impulses

Force, torque, impulse, and radial-force requests are typed commands containing:

- body and entity revisions;
- source and owner identity;
- vector, location, and reference frame;
- force or impulse mode;
- duration when continuous;
- clamping and mass-scaling policy;
- wake policy; and
- result and diagnostics fields.

The physical-profile service validates the command before it reaches Chaos.
Collision callbacks and visual effects cannot apply an unbounded force or reuse
a
stale body handle.

## Transform and bounds synchronization

For a simulated primitive, Chaos drives the component transform. For a kinematic
or static primitive, the accepted application or animation owner drives it.

Transform changes publish one revision used by:

- conservative render bounds;
- broad-phase collision;
- world placement and streaming adapters;
- interaction and mission projections;
- persistence when policy requires it; and
- diagnostics.

A component move requests the spatial visibility and placement updates through
native engine registration. Repository code does not manually remove and insert
the same entity into pointer-owned tree leaves.

## Collision observations and reactions

Physics contact callbacks are normalized into immutable observations containing:

- entity, body, component, and world revisions;
- other participant identities;
- contact points and normals;
- relative linear and angular velocity;
- normal and tangential impulse evidence;
- physical surfaces and collision profiles;
- solver frame and ordering evidence; and
- duplication or aggregation identity.

The physical material and impact-response runtime selects sound, effects,
decals, animation, damage proposals, and breakage requests.

A pre-contact or post-contact callback cannot directly grant rewards, complete a
mission, delete an entity, change persistence, or mutate an unrelated body.

## Breakage and destruction

Breakage is a transaction, not a render callback. A break request validates:

- breakable definition and current entity revision;
- accepted impact, damage, interaction, or mission evidence;
- threshold and cooldown policy;
- replacement representation and collision policy;
- reward or progression owner;
- persistence and respawn policy;
- effects and audio requests; and
- streaming readiness.

After commit, the service may disable or remove the intact representation, spawn
or activate the declared broken representation, transfer or create physics
bodies, publish effects, and persist the accepted result.

Geometry Collections, replacement Actors, authored animation, and Niagara may
implement the presentation through
<!-- markdownlint-disable-next-line MD013 -->
[Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md).
None can decide that the break transaction succeeded merely because an
animation,
fragment sequence, or effect completed.

## Render-layer and content identity

World, frontend, local-player, cinematic, reflection, capture, diagnostic, and
shadow views are native renderer and application scopes. They are not selected
by
legacy level or mission enum ordinals.

A render-scope request carries canonical world, application mode, viewport,
participant, camera, feature, and presentation revisions. Content definitions
use
stable chapter, mission, region, and package identities from their owning
catalogs.

A numeric render layer, level number, mission number, or array position may
remain inside a versioned conversion artifact. It cannot become a runtime save
key, package identity, camera authority, visibility authority, or feature
namespace.

Application scopes, local-player views, loading barriers, frontend and world
composition, presentation freezes, frame execution, display policy, and renderer
handoff follow
<!-- markdownlint-disable-next-line MD013 -->
[Native render-frame, view, and layer runtime](native-render-frame-view-and-layer-runtime.md).
A world entity cannot add itself to an ordinal render layer or drive frame
submission.

## Render and shadow policy

Primitive components use Unreal's renderer for opaque, masked, translucent,
shadow, depth, decal, reflection, and custom-depth participation.

A component declares material slots, translucency, cast-shadow flags, visible
shadow classes, lighting channels, and quality policy. The renderer owns final
sorting, pass submission, and shadow projection.

Repository code does not rank drawables by camera distance, shader name, or a
manual translucent list. It does not create a camera-facing shadow matrix or
change depth-write state around a custom shadow drawable.

Simple blob or contact shadows are registered native decals, materials, virtual
shadow behavior, or another validated presentation adapter. They remain cosmetic
and cannot provide gameplay contact evidence.

## Visibility and culling integration

Every primitive follows
<!-- markdownlint-disable-next-line MD013 -->
[Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md).

The component owns accurate bounds and render-state invalidation. Unreal owns
per-view visibility, instance culling, occlusion, LOD, and final draw
submission.

A world entity being culled does not become absent from collision, physics,
interaction, mission, persistence, or streaming state.

## World registration and streaming

An entity registers only after its world region, Data Layer, required bundles,
and definition are ready. Registration records:

- entity, placement, world, and composition revisions;
- Actor and component representation identities;
- persistent and transient owner identities;
- collision and simulation readiness;
- visibility and quality policy; and
- retained asset handles.

World unload, Data Layer deactivation, feature removal, owner cancellation, and
replacement use one teardown transaction. It disables new commands, cancels
pending async work, records required state, releases gameplay and presentation
leases, unregisters components, destroys the Actor, and releases assets.

A late load, collision, physics, animation, or render callback cannot restore or
remove a replacement entity.

## Persistence and respawn

Durable state follows
<!-- markdownlint-disable-next-line MD013 -->
[Persistent world-object state runtime](persistent-world-object-state-runtime.md)
and mission-scoped restoration follows
<!-- markdownlint-disable-next-line MD013 -->
[Mission world-entity and respawn runtime](mission-world-entity-and-respawn-runtime.md).

The persistence snapshot stores canonical entity and placement identities plus
schema-owned state. It does not store Actor addresses, component names, solver
handles, instance indices, or scene-tree locations.

Physics transforms or velocities are persisted only when the entity definition
requires durable movement. Otherwise streaming or restart reconstructs the
entity from the canonical placement and accepted world state.

## Local multiplayer and networking boundary

Each local view may observe the same entity through independent visibility,
camera, HUD, and audio projections. The entity and Chaos body remain one world
identity unless the game mode explicitly creates separate simulation worlds.

The built-in local split-screen mode does not imply network replication.
Community network adapters must map stable entity and physics identities through
the multiplayer boundary and cannot serialize raw component or solver handles.

## Platform and quality policy

All supported quality presets preserve gameplay-required collision, physics,
interaction, query, and entity state.

Quality may change:

- LOD, HLOD, Nanite, and instance policy;
- shadow method and distance;
- optional effects and decals;
- material and animation detail;
- physics sub-stepping or solver settings only within validated deterministic
  and
  gameplay-equivalent bounds; and
- diagnostic availability.

Quality cannot remove a required blocker, query surface, mission prop,
breakable,
vehicle interaction, or physics result. Android Low uses the same domain state
and collision rules as desktop targets.

## Feature and mod overlays

A validated feature may add namespaced entity definitions, representations,
collision profiles, physical profiles, breakable definitions, and query
surfaces.
It declares dependencies, conflicts, target support, budgets, migration, and
teardown.

An overlay cannot replace an active entity in place, weaken collision or
persistence policy, register arbitrary component classes, inject solver
callbacks,
or retain Actors and bodies after feature removal.

## Concurrency

Game-thread application state, physics-thread solver state, async asset loading,
and render-thread scene state communicate through engine-supported boundaries
and
immutable project snapshots.

Async callbacks carry entity, body, representation, world, owner, and request
revisions. Stale callbacks are rejected before mutation.

Repository code does not pass mutable custom scene-tree nodes, triangle arrays,
or raw physics pointers between threads.

## Diagnostics

Development diagnostics expose immutable snapshots of:

- entity, placement, representation, body, world, and asset revisions;
- Actor and component composition;
- mesh, materials, LOD, Nanite, and instancing policy;
- bounds and render-state revision;
- collision profile and physical profile;
- simulation, sleep, wake, velocity, and support state;
- recent collision and force observations;
- stateful-prop identity, accepted state, transition, and projection revision;
- scene-query request, filters, cache key, candidates, hits, and result
  revision;
- world-sky and lens-presentation identity, view, occlusion, and intensity
  state;
- breakage and persistence state;
- streaming and retained asset handles;
- native shadow and visibility policy; and
- last construction, collision, query, presentation, simulation, breakage, or
  teardown finding.

Chaos Visual Debugger, collision visualization, bounds views, and
repository-owned
read-only inspectors may consume this evidence. Diagnostics cannot mutate domain
state or mark a transaction successful.

## Failure behavior

The runtime fails closed on:

- missing or duplicate canonical entity identity;
- unsupported representation class;
- invalid Actor or component composition;
- missing mesh, material, animation, collision, or physics asset;
- non-conservative or non-finite bounds;
- static mobility combined with simulation;
- query-only geometry requested as a rigid body;
- collision without a registered profile;
- simulation with a conflicting transform owner;
- malformed stateful-prop transition or mixed projection revision;
- animation, collision, or visibility applying an unaccepted prop state;
- scene query with invalid channel, filter, extent, world, or result limit;
- stale or incomplete query cache key;
- nondeterministic closest-road, closest-path, or hit ordering;
- lens occlusion or intensity state shared across unrelated views;
- world presentation attempting world-clock, weather, mission, or save mutation;
- stale entity, body, representation, state, query, view, world, or request
  revision;
- saved or replicated raw instance index;
- source triangle parsing attempted in shipping runtime;
- breakage without replacement, persistence, or teardown policy;
- renderer callback attempting gameplay mutation;
- collision or query callback attempting direct mission, reward, or save
  mutation;
- world unload with registered components or retained bodies; or
- feature removal with owned Actors, components, bodies, or asset handles.

Failure returns typed evidence, rolls back prepared state, and preserves the
last
accepted entity revision. It never leaves a hidden collidable object, visible
non-collidable replacement, active body without an owner, or dangling component.

## Validation

Definition and converted-asset validation prove:

- every entity, placement, representation, collision, physical, and breakable
  identity resolves;
- every representation class has one registered native component composition;
- all required assets and target variants exist;
- bounds contain the accepted rendered and collision representations;
- mobility, simulation, collision, and transform ownership are compatible;
- ISM or HISM groups preserve stable project identities through index changes;
- cooked collision replaces source-triangle runtime parsing;
- stateful-prop graphs, per-state projections, and compensation paths are valid;
- animated visual and collision entities use deterministic start and pose
  policy;
- scene-query channels, object types, filters, limits, ordering, and cache keys
  are
  complete and deterministic;
- closest-road, closest-path, terrain, interior, and line-of-sight mappings use
  canonical identities;
- lens and world-presentation definitions have independent per-view and
  world-scoped ownership plus platform fallbacks;
- physical profiles use supported Chaos settings;
- every breakable has a complete replacement and persistence path;
- every streaming and feature owner has complete teardown; and
- no render, animation, collision, physics, or query callback has domain
  mutation
  authority.

## Tests

Required automated tests include:

- construction commit and partial rollback;
- static render and static collision registration;
- ISM identity mapping through add, remove, and compaction;
- HISM fallback for changing instance groups;
- movable rigid-body start, force, impulse, collision, sleep, wake, and
  teardown;
- animated-to-physics and physics-to-animation handoff;
- deterministic animated-visual start phase and reset;
- animated-collision pose, body, bounds, and fallback synchronization;
- stateful-prop accepted transition, compensation, duplicate observation, and
  stale callback rejection;
- stateful-prop reward and persistence isolation;
- missing Physics Asset and non-simulated fallback;
- fence shape generation and collision profile binding;
- cooked query surface line, box, sphere, capsule, overlap, and sweep results;
- closest-road and closest-path deterministic tie-breaks;
- horizontal and full three-dimensional line-of-sight behavior;
- scene-query cache invalidation after movement, profile change, streaming, and
  feature removal;
- query truncation and budget evidence;
- collision observation normalization and duplicate suppression;
- independent lens occlusion and intensity for split-screen and scene capture;
- lens fallback and temporal smoothing across frame rates;
- world-sky activation, blend, replacement, and teardown;
- breakage success, rejection, duplicate request, and rollback;
- persistence and respawn reconstruction;
- streaming unload during load, simulation, query, collision, and breakage;
- stale async, physics, animation, query, lens, and render callback rejection;
- native shadow and translucency behavior across quality presets;
- local split-screen visibility without duplicate world entities;
- feature removal with zero retained Actors, components, bodies, and assets; and
- identical gameplay results across supported rendering presets.

## Invariants

- Every world render entity has one canonical identity and accepted revision.
- Actor and component composition is definition-driven and validated.
- Unreal owns renderer registration and final draw submission.
- Chaos owns accepted rigid-body simulation state.
- Render, animation, collision, physics, and query callbacks publish evidence,
  not
  domain mutations.
- Stateful props project one accepted state revision across all channels.
- Scene-query results are immutable, bounded, revisioned, and deterministically
  ordered.
- Query caches include world, policy, filter, and collision-scene revisions.
- Lens presentation is per view and cannot become gameplay visibility authority.
- World presentation consumes world state but never owns it.
- Stable instance identity never equals an engine array index.
- Legacy render-layer, level, and mission ordinals never become runtime
  identity.
- Source triangle geometry is cooked before shipping runtime.
- Culling never disables collision, physics, persistence, or gameplay state.
- Breakage commits before replacement presentation is accepted.
- Every terminal entity revision releases or transfers all owned resources.

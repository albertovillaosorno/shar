# World render-entity and physics runtime

- Status: Active
- Last reviewed: 2026-07-15

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
- `FSharInstanceGroupId`;
- `FSharInstanceId`;
- `FSharQuerySurfaceId`;
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
- `animated_rigid`, for an animated or articulated prop with bounded physics;
- `linear_blocker`, for authored fence or barrier geometry;
- `query_surface`, for collision or scene-query geometry without ordinary visual
  presentation;
- `breakable_composite`, for a registered destruction representation; and
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
- bounds and provenance; and
- conversion findings.

Shipping runtime does not parse triangle strips, allocate custom flat-triangle
arrays, or rebuild collision fields from source geometry. Cooked collision
assets
are produced during import.

Queries return engine hit results normalized to typed project results. A query
surface does not render itself merely to prove collision and does not become
mission or interaction authority.

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

Geometry Collections, replacement Actors, or authored animation may implement
the
presentation. None can decide that the break transaction succeeded merely
because
an animation played.

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
- breakage and persistence state;
- streaming and retained asset handles;
- native shadow and visibility policy; and
- last construction, collision, simulation, breakage, or teardown finding.

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
- stale entity, body, representation, world, or request revision;
- saved or replicated raw instance index;
- source triangle parsing attempted in shipping runtime;
- breakage without replacement, persistence, or teardown policy;
- renderer callback attempting gameplay mutation;
- collision callback attempting direct mission, reward, or save mutation;
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
- physical profiles use supported Chaos settings;
- every breakable has a complete replacement and persistence path;
- every streaming and feature owner has complete teardown; and
- no component or physics callback has domain mutation authority.

## Tests

Required automated tests include:

- construction commit and partial rollback;
- static render and static collision registration;
- ISM identity mapping through add, remove, and compaction;
- HISM fallback for changing instance groups;
- movable rigid-body start, force, impulse, collision, sleep, wake, and
  teardown;
- animated-to-physics and physics-to-animation handoff;
- missing Physics Asset and non-simulated fallback;
- fence shape generation and collision profile binding;
- cooked query surface line, box, sphere, capsule, overlap, and sweep results;
- collision observation normalization and duplicate suppression;
- breakage success, rejection, duplicate request, and rollback;
- persistence and respawn reconstruction;
- streaming unload during load, simulation, collision, and breakage;
- stale async, physics, animation, and render callback rejection;
- native shadow and translucency behavior across quality presets;
- local split-screen visibility without duplicate world entities;
- feature removal with zero retained Actors, components, bodies, and assets; and
- identical gameplay results across supported rendering presets.

## Invariants

- Every world render entity has one canonical identity and accepted revision.
- Actor and component composition is definition-driven and validated.
- Unreal owns renderer registration and final draw submission.
- Chaos owns accepted rigid-body simulation state.
- Collision and physics callbacks publish evidence, not domain mutations.
- Stable instance identity never equals an engine array index.
- Source triangle geometry is cooked before shipping runtime.
- Culling never disables collision, physics, persistence, or gameplay state.
- Breakage commits before replacement presentation is accepted.
- Every terminal entity revision releases or transfers all owned resources.

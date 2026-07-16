# Native cooked-asset construction and registration runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Validated game-feature mod overlays](../../adr/unreal/runtime/validated-game-feature-mod-overlays.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](native-import-material-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [World render-entity and physics runtime](world-render-entity-and-physics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native render-frame, view, and layer runtime](native-render-frame-view-and-layer-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md)

## Purpose

This specification defines how validated cooked Unreal assets become prepared
and
active runtime objects after an accepted load request. It covers primary-asset
resolution, bundles, soft references, retained streamable handles, construction
plans, component preparation, registration, deduplication, callback correlation,
cancellation, feature overlays, diagnostics, and teardown.

It replaces runtime parsing of source chunks, one process-wide wrapper registry,
loader overrides, mutable listener pointers, integer user-data callbacks, fixed
global-entity arrays, source-name conventions, manually paired sub-loaders, and
null-entity cancellation signals.

Source-format decoding and native asset generation happen before shipping
runtime.
The runtime loads cooked Unreal packages and constructs only registered native
representations.

## Native Unreal foundation

The boundary uses native Unreal facilities:

- `UAssetManager` for primary-asset discovery, policy, bundles, and auditing;
- `FPrimaryAssetId` for engine-visible primary-asset identity;
- `TSoftObjectPtr`, `TSoftClassPtr`, and `FSoftObjectPath` for cook-aware soft
  references;
- `FStreamableManager` and retained `FStreamableHandle` ownership for
  asynchronous
  loading;
- the Asset Registry for unloaded metadata and validation;
- `UPrimaryDataAsset` or another registered native data class for immutable
  construction definitions;
- `UWorld`, World Partition, Runtime Data Layers, and Game Features for
  lifetime;
- native Actor and component construction;
- weak object references and typed delegates where observation is required; and
- subsystem lifetimes instead of manually created process singletons.

Repository code adds validation, correlation, typed results, deterministic
catalogs, and application transactions. It does not create a second package
loader or source-chunk object system.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Asset Manager | Discovers primary assets, applies rules, resolves bundles, and audits cooked dependencies. |
| Streamable Manager | Performs asynchronous object loading and retains loaded objects while handles are owned. |
| Cook and import pipeline | Converts normalized evidence into native assets and validates read-back before publication. |
| Construction catalog | Maps canonical definition identities to closed native construction plans. |
| Construction subsystem | Prepares Actors, components, and adapters and commits one accepted runtime revision. |
| World-composition service | Owns world, Data Layer, region, and feature readiness and teardown. |
| World render-entity runtime | Owns native Actor and component composition after construction commit. |
| Persistence and mission services | Own durable state, gameplay activation, rewards, collection, and destruction. |
| Feature runtime | Owns namespaced overlay registration and removal. |
| Presentation services | Own optional lens, sky, animation, particle, audio, and feedback adapters. |

<!-- markdownlint-enable MD013 -->

Loading an asset, constructing an object, registering it with a world,
activating
its gameplay adapters, and committing durable state are separate transactions.

## Runtime identities

The boundary uses stable identities for:

- `FSharConstructionDefinitionId`;
- `FSharConstructionDefinitionRevision`;
- `FSharConstructionPlanId`;
- `FSharConstructionRequestId`;
- `FSharConstructionRevision`;
- `FSharAssetSetId`;
- `FSharAssetSetRevision`;
- `FSharBundleId`;
- `FSharBundleRevision`;
- `FSharPreparedObjectId`;
- `FSharWorldEntityId`;
- `FSharPlacementId`;
- `FSharFeatureRevision`;
- `FSharWorldCompositionRevision`; and
- `FSharConstructionResultId`.

Loader-array positions, chunk numbers, source filenames, raw callback pointers,
integer user data, object addresses, wrapper classes, and mutable inventory
names
are not durable identity.

## Construction definition

`FSharCookedConstructionDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `DefinitionId` | Canonical project identity. |
| `PrimaryAssetId` | Registered Unreal primary asset or definition asset. |
| `ConstructionKind` | Closed native construction strategy. |
| `AssetSetId` | Required and optional asset identities. |
| `BundleIds` | Named bundles required for preparation, activation, and optional presentation. |
| `RepresentationId` | World render-entity representation contract when applicable. |
| `ComponentPlan` | Immutable component types, attachments, transforms, and configuration. |
| `CollisionPlan` | Cooked collision, Physics Asset, query, and physical-material requirements. |
| `AnimationPlan` | Optional Animation Blueprint, sequence, montage, controller, and state binding. |
| `PresentationPlan` | Optional sky, lens, particle, shadow, and audio bindings. |
| `WorldPolicy` | World, region, Data Layer, placement, and teardown requirements. |
| `PersistencePolicy` | Durable-state projection and migration behavior. |
| `FeaturePolicy` | Base or namespaced overlay ownership. |
| `FallbackPolicy` | Declared optional representation fallback. |
| `DiagnosticsPolicy` | Development inspection and capture permissions. |

<!-- markdownlint-enable MD013 -->

Definitions reject unknown construction kinds, hard references that violate the
bundle plan, unresolved dependencies, mutable global callback state, runtime
source parsers, and construction plans without complete rollback.

## Closed construction kinds

The initial closed construction kinds are:

- `static_render_entity`;
- `static_collision_entity`;
- `instanced_static_entity`;
- `movable_rigid_entity`;
- `animated_visual_entity`;
- `animated_collision_entity`;
- `animated_rigid_entity`;
- `stateful_prop_entity`;
- `linear_blocker_entity`;
- `query_surface_entity`;
- `breakable_entity`;
- `world_sky_presentation`;
- `lens_presentation`;
- `billboard_presentation`;
- `particle_presentation`;
- `registered_composite`; and
- `data_only_definition`.

A construction kind selects native composition only. It does not assign mission,
reward, save, interaction, or progression behavior.

## Source conversion boundary

Shipping runtime consumes cooked packages only. It cannot:

- parse source scene or wrapper chunks;
- invoke source-format sub-loaders;
- discover component relationships from arbitrary chunk order;
- create collision or Physics Assets from source triangle data;
- infer shadows or animations from filename suffixes;
- generate random timing from load order;
- select mission behavior from source object names; or
- retain source decoder types in runtime packages.

The import pipeline converts source evidence into native assets, construction
definitions, dependency bundles, identity maps, and provenance. Runtime
read-back
validates the cooked result rather than reinterpreting the source.

## Primary assets and bundles

Each construction definition is discoverable by one registered primary-asset
policy or by a validated world-owned hard reference when ordinary Unreal
lifetime
is sufficient.

Bundles are semantic and bounded. Typical bundle families are:

- `definition`, for immutable construction data;
- `representation`, for required render assets;
- `physics`, for collision, Physics Assets, and physical materials;
- `animation`, for optional animated behavior;
- `presentation`, for lens, sky, particle, audio, and shadow assets;
- `activation`, for gameplay-facing adapters; and
- `diagnostics`, for development-only assets.

Bundle names, dependencies, cook rules, and platform support are validated
during
cook. Loading a bundle cannot silently load an unrelated source inventory.

## Soft references

Construction definitions use class-restricted soft references for optional or
streamed assets. A soft reference is validated for:

- expected native class;
- canonical package path;
- redirect resolution;
- cook inclusion;
- feature ownership;
- platform availability;
- bundle membership; and
- fallback compatibility.

A raw string that happens to resemble an asset path is not a construction
contract.

## Construction request

`FSharConstructionRequest` contains:

- request and owner identities;
- definition and expected definition revision;
- placement and transform snapshot;
- world, composition, Data Layer, and feature revisions;
- requested bundles;
- activation policy;
- priority and deadline policy;
- cancellation token;
- expected existing-entity revision when replacing; and
- diagnostics context.

Requests are immutable after acceptance. A caller submits a replacement request
rather than mutating an active request in place.

## Lifecycle states

The lifecycle uses the closed states:

1. `requested`;
1. `resolving`;
1. `loading`;
1. `loaded`;
1. `validating`;
1. `preparing`;
1. `prepared`;
1. `committing`;
1. `active`;
1. `cancelling`;
1. `failed`; and
1. `released`.

Every accepted request reaches exactly one terminal result. Asset load
completion
is not construction completion, and construction completion is not gameplay
activation.

## Dependency resolution

The construction subsystem resolves a deterministic dependency graph from the
accepted definition and bundle revisions. It validates:

- required and optional edges;
- cycles;
- class compatibility;
- feature ownership;
- platform availability;
- package and chunk installation;
- fallback paths;
- cook rules; and
- maximum dependency count and memory estimate.

Dependency order uses canonical identity ordering where Unreal does not impose
an
engine order. Mutable listener registration order cannot change the plan.

## Asynchronous loading

Asynchronous requests retain streamable handles until the owning construction
transaction commits, fails, or cancels. The subsystem records:

- requested soft paths and primary assets;
- bundle and dependency revisions;
- handle identity and owner;
- progress and terminal load result;
- retained-object count;
- cancellation state; and
- elapsed wall and simulation-independent time.

A completion delegate revalidates request, owner, world, feature, definition,
and
bundle revisions before dereferencing loaded objects.

## Handle ownership

A streamable handle is owned by one explicit scope:

- construction request;
- active world entity;
- world composition;
- application mode;
- feature or mod;
- local player presentation; or
- development diagnostics.

The owner either transfers the handle at commit or releases it exactly once.
Delegate completion alone is not sufficient retention evidence.

## Deduplication and shared assets

Immutable meshes, materials, animations, collision assets, data assets, and
other
UObjects may be shared through ordinary Unreal package identity and retained
handles.

The runtime does not hold one process-global drawable instance and move it among
world placements. Each placement has independent Actor or component state even
when it shares immutable assets.

Deduplication keys include asset identity, bundle revision, feature revision,
platform, and cook revision. A cached asset cannot satisfy an incompatible
construction definition merely because its short name matches.

## Prepared-object transaction

Preparation creates native objects in an inactive, non-authoritative state. It:

1. resolves the registered constructor;
1. validates loaded asset classes and revisions;
1. creates the Actor or owning UObject in the accepted world scope;
1. creates only the declared components;
1. applies attachments and transforms;
1. binds meshes, materials, animations, collision, and physical profiles;
1. disables gameplay interaction and simulation until commit;
1. verifies component hierarchy, bounds, and required interfaces;
1. registers rollback ownership; and
1. publishes one immutable prepared snapshot.

Prepared objects cannot emit gameplay results, rewards, save mutations, or
mission observations.

## Commit and activation

Commit revalidates the request against the latest world, placement, feature,
persistence, and gameplay snapshots. It then:

1. accepts the prepared construction revision;
1. replaces or registers the canonical runtime binding;
1. transfers retained asset handles;
1. registers native components with the world;
1. applies accepted persistent state;
1. enables declared collision, simulation, animation, and presentation;
1. enables gameplay adapters only after their own validation; and
1. publishes one `active` result.

A failure before commit destroys the prepared objects and leaves the prior
active
revision unchanged.

## Typed completion results

`FSharConstructionResult` contains:

- request and construction result identities;
- definition, asset-set, bundle, world, feature, and placement revisions;
- terminal status;
- constructed native object identities;
- retained-handle transfer evidence;
- fallback or replacement decision;
- validation findings; and
- teardown obligations.

The closed terminal statuses are:

- `active`;
- `cancelled`;
- `superseded`;
- `unavailable`;
- `invalid_definition`;
- `load_failed`;
- `prepare_failed`;
- `commit_failed`; and
- `world_released`.

Cancellation is never encoded as a successful callback containing a null object.

## Callback correlation

Callbacks use typed delegates or subsystem observations carrying request and
revision identities. Raw listener pointers, mutable integer user data, and
registration-cancellation callbacks are prohibited.

A late callback cannot:

- commit a superseded construction;
- mutate a replacement Actor;
- transfer old handles;
- restore an unloaded world;
- re-register a removed feature;
- invoke a destroyed owner; or
- publish a second terminal result.

## Constructor registry

Constructors are registered by closed construction kind and feature namespace.
Each registration declares:

- constructor identity and revision;
- accepted definition class;
- supported native output classes;
- required interfaces;
- platform and feature support;
- validation function;
- prepare and rollback functions; and
- teardown obligations.

The registry is immutable during one accepted construction snapshot. A loader
cannot replace another loader globally or override a native engine handler in
place.

## Feature and mod overlays

A validated Game Feature may add namespaced definitions, constructors, assets,
and bundles. Registration validates dependencies, conflicts, platform support,
resource limits, and teardown.

An overlay cannot:

- replace a base constructor in place;
- intercept unrelated load requests;
- alter a base asset bundle silently;
- gain access to raw package callbacks;
- weaken class or cook validation; or
- leave registrations after feature removal.

Feature removal cancels owned requests, tears down owned runtime objects,
releases
handles, unregisters constructors, and invalidates stale callbacks atomically.

## Static render and physics construction

Static render, static collision, and static physics definitions bind cooked
`UStaticMesh`, material, collision, physical-material, mobility, shadow, and
bounds policy.

The constructor does not clone source drawable wrappers or infer physics from a
class name. Physics-enabled definitions require a complete collision and
physical profile before preparation succeeds.

Static entity construction validates one closed composition of native mesh,
material, collision, mobility, shadow, bounds, instance, and world-placement
policy. A source wrapper, tree-node class, or static-physics loader name cannot
select behavior at runtime.

Converted spatial-tree data may be retained as deterministic import evidence and
diagnostics under
<!-- markdownlint-disable-next-line MD013 -->
[Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md).
It is not constructed as a second authoritative runtime renderer or visibility
tree. Unreal component registration and renderer facilities own active scene
membership.

## Animated construction

Animated visual, animated collision, animated rigid, and stateful prop
definitions
bind validated skeletal or component animation assets, Animation Blueprints,
state definitions, collision shapes, Physics Assets, and presentation adapters.

Random start offsets are derived from stable definition, placement, and accepted
seed identities. Load timing and global random-call order cannot alter animation
selection.

## Query-surface construction

Query surfaces bind cooked collision or geometry data to native primitive
components and registered query channels. They do not rebuild flat triangle
arrays from source strips at runtime.

Construction validates terrain or surface metadata, interior classification,
query-only versus blocking policy, transform, bounds, and world ownership.

## Lens and billboard construction

Lens presentation prefers Unreal's camera and post-process lens facilities where
they satisfy the authored visual contract. A custom source-specific flare
adapter
requires a registered definition, per-view ownership, bounded occlusion
evidence,
accessibility and quality policy, and deterministic fallback.

The runtime cannot use one process-global framebuffer read-back queue, mutate
billboard intensity through shared raw objects, or share visibility results
across
local players and scene captures.

## Sky and world presentation construction

Sky, atmosphere, cloud, dome, horizon, and world-background definitions use
native world and presentation components selected by the accepted visual policy.
They are world-scoped and revisioned.

A sky presentation cannot become world identity, time authority, weather
authority, streaming authority, or mission state merely because it spans the
visible world.

## Particle construction

Particle construction binds cooked Niagara Systems, Effect Types, parameter
schemas, attachment policy, coordinate space, lifetime class, scalability,
pooling eligibility, platform variants, and fallback according to
<!-- markdownlint-disable-next-line MD013 -->
[Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md).

The constructor prepares native assets and optional components only. It cannot
load source particle factories, allocate fixed source instance arrays, assign
integer player handles, require a per-frame keepalive call, or register a custom
scene-graph particle object.

Continuous effects become active only after a bounded owner lease is accepted.
One-shot completion and pool release publish typed presentation results rather
than gameplay events.

## Breakable construction

Breakable definitions bind intact representation, collision, optional Geometry
Collection or registered replacement representation, effects, audio, debris,
persistence, and teardown.

Construction prepares presentation capability only. Destruction still requires
an accepted gameplay or persistence transaction. An animation ending cannot
grant coins, rewards, or progression.

## Render-scope registration handoff

A prepared renderable object transfers its immutable Actor, component, asset,
world, placement, and presentation revisions to
<!-- markdownlint-disable-next-line MD013 -->
[Native render-frame, view, and layer runtime](native-render-frame-view-and-layer-runtime.md).
The render boundary validates the active application scope and native view
eligibility before presentation.

Construction does not add raw drawables to ordinal layers, own cameras or views,
submit shadows or translucent passes, begin or end a view, drive the engine
frame,
or interpret one rendered frame as gameplay readiness.

Renderer registration occurs through native Actor and component lifecycle after
construction commit. Scope suspension, world unload, feature removal, and object
replacement tear down the exact accepted registration and retained handles.

## Collision and physical-profile binding

Collision construction validates object type, channels, response matrix, query,
overlap, simulation, CCD, mass, density, friction, restitution, damping, and
physical-surface identity.

Source class IDs, numeric breakable lists, and string-based physics properties
do
not survive as runtime authority.

## World and Data Layer readiness

A constructed object becomes active only in the accepted world, region, and Data
Layer revisions. World composition owns readiness and teardown.

A loaded UObject in memory is not proof that its target world or placement is
ready. A late load callback for an unloaded region can complete with
`world_released` but cannot recreate the Actor.

## Cancellation and supersession

Cancellation is idempotent and revisioned. It:

- prevents new preparation work;
- cancels or detaches owned asynchronous loads where supported;
- destroys prepared native objects;
- unregisters temporary adapters;
- releases owned handles;
- records one terminal result; and
- rejects later callbacks.

Supersession creates a new request identity. It does not mutate callback user
data
on the old request.

## Synchronous loading boundary

Synchronous loading is allowed only for validated startup, editor, automation,
or
small bounded cases where the owning contract explicitly permits it. Gameplay
and
world streaming do not silently fall back to blocking loads after an
asynchronous
failure.

Every synchronous call records the reason, expected size, caller, and measured
stall in development diagnostics.

## Concurrency

Definition and registry snapshots are immutable. Request state transitions occur
through one construction authority. Asset loading may complete concurrently, but
object preparation and world registration follow Unreal thread-affinity rules.

No callback reads mutable raw listener state. Cross-thread messages carry copied
identities and immutable results.

## Diagnostics

Development diagnostics expose:

- request, definition, bundle, asset-set, world, placement, and feature
  revisions;
- lifecycle state and terminal status;
- unresolved, loading, loaded, retained, and released assets;
- dependency graph and bundle membership;
- constructor registration and selected construction kind;
- prepared Actor and component hierarchy;
- render-scope and native registration handoff;
- Niagara, Effect Type, parameter-schema, and VFX lease preparation;
- converted spatial-tree evidence and native-scene registration policy;
- handle owner and transfer state;
- fallback and replacement decisions;
- stale callback count;
- cancellation and rollback evidence;
- cook and platform revisions; and
- last validation or teardown finding.

Diagnostics are read-only. They cannot force activation, bypass cook rules, or
publish gameplay results.

## Failure behavior

The boundary fails closed on:

- unknown or duplicate construction identity;
- runtime source parsing or source decoder dependency;
- missing primary asset, bundle, package, or cooked dependency;
- invalid soft-reference class or redirect;
- cyclic or unbounded dependency graph;
- mutable constructor override;
- raw listener or integer user-data callback authority;
- null-object cancellation encoding;
- stale request, world, placement, feature, or bundle revision;
- prepared object with undeclared components;
- collision, physics, animation, Niagara, Effect Type, or presentation class
  mismatch;
- particle construction without a typed parameter schema or bounded lifetime;
- prepared renderable object with no compatible render-scope handoff;
- converted spatial tree requested as a second runtime renderer;
- missing rollback or teardown path;
- handle release before ownership transfer;
- duplicate terminal result; and
- feature removal with retained constructors, assets, or objects.

Failure produces typed evidence and leaves the last accepted world and entity
revision unchanged.

## Validation

Cook and content validation proves:

- every definition has a stable identity and closed construction kind;
- every soft reference resolves to an allowed class;
- every required dependency is cooked and belongs to the expected bundle;
- dependency graphs are bounded and acyclic;
- every constructor is registered exactly once per namespace and revision;
- every construction plan has prepare, commit, rollback, and teardown paths;
- no runtime source parser or wrapper loader is packaged;
- every collision and Physics Asset binding is valid;
- every stateful, animated, lens, sky, query, particle, and breakable definition
  has a declared fallback or required-platform proof;
- every particle definition binds a cooked Niagara System, Effect Type, typed
  parameter schema, lifetime, pooling, and scalability policy;
- static and spatial constructor output registers through native scene paths;
- every renderable constructor has a compatible render-scope handoff;
- handles remain retained for their required scopes;
- cancellation and stale-callback behavior are deterministic; and
- feature removal releases all owned registrations and assets.

## Tests

Required automated tests include:

- primary-asset and bundle discovery;
- cook inclusion and redirect validation;
- resident and asynchronous load equivalence;
- retained-handle lifetime before and after commit;
- dependency cycle and class-mismatch rejection;
- missing optional asset fallback;
- prepared-object rollback;
- replacement without active-state gaps;
- cancellation before load, during load, during preparation, and after prepare;
- stale callback rejection after supersession and world unload;
- duplicate terminal-result rejection;
- shared immutable assets with independent placements;
- static, instanced, animated, rigid, stateful, query, lens, sky, particle, and
  breakable construction;
- static spatial evidence with native scene registration and no second runtime
  renderer;
- Niagara parameter-schema, lifetime, scalability, and pool preparation;
- render-scope handoff, suspension, replacement, and teardown;
- feature activation and removal;
- synchronous-load budget enforcement;
- identical results across supported frame rates; and
- packaged-build proof that no source decoder is required.

## Invariants

- Shipping runtime consumes cooked Unreal assets only.
- Asset loading, object construction, world registration, gameplay activation,
  and persistence are separate transactions.
- Every request has one stable identity and one terminal result.
- Streamable handles remain owned until transfer or release.
- Raw listener pointers and integer user data are never completion authority.
- Cancellation is a typed terminal result, not a null-object callback.
- Constructor registries are immutable and namespaced per accepted revision.
- Shared assets never imply shared mutable placement state.
- Particle construction never owns effect lifetime without a bounded
  presentation
  lease or one-shot completion policy.
- Converted spatial trees never replace native renderer scene registration.
- Renderable construction cannot bypass the accepted render scope or native view
  boundary.
- Feature removal releases every owned registration, handle, and object.
- A loaded object cannot bypass world, placement, feature, or gameplay
  validation.

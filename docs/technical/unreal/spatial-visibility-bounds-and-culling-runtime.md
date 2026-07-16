# Spatial visibility, bounds, and culling runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Runtime parity boundary](../../adr/unreal/runtime/remake-parity-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [HUD, radar, camera, and navigation parity](../../adr/unreal/ui/hud-radar-camera-and-navigation.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](native-import-material-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored spatial placement and trigger runtime](authored-spatial-placement-and-trigger-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Camera rig, preset, and arbitration runtime](camera-rig-preset-and-arbitration-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md)

## Purpose

This specification defines the native Unreal boundary for render bounds, view
volumes, distance and frustum rejection, occlusion, deterministic spatial-cell
build evidence, visibility policy, and diagnostics.

It preserves the observable world while replacing custom runtime point arrays,
raw pointer trees, fixed-capacity containers, platform-specific culling paths,
render-weight heuristics as gameplay authority, and visibility decisions that
silently unload collision, navigation, missions, or required presentation.

Unreal Engine owns scene-proxy visibility and occlusion. Repository code owns
validated content policy, deterministic conversion evidence, explicit gameplay
visibility, diagnostics, and tests. It does not fork or replace the renderer.

## Native Unreal foundation

The runtime uses native Unreal facilities, including:

- primitive box and sphere bounds;
- per-view convex frusta;
- renderer distance and view-frustum culling;
- supported dynamic occlusion methods;
- optional precomputed visibility where platform policy permits it;
- Cull Distance Volumes and per-component draw-distance policy;
- World Partition, Runtime Data Layers, LOD, HLOD, and Nanite policy;
- immutable game-thread visibility snapshots; and
- render-thread scene proxies owned by Unreal Engine.

Repository-owned C++ may validate or project these facilities. It cannot copy
engine scene state into a second authoritative spatial tree merely to decide
what the renderer should draw.

## Ownership

<!-- markdownlint-disable MD013 -->

| Service | Authority |
| :--- | :--- |
| Unreal renderer | Per-view primitive visibility, frustum rejection, supported occlusion, render-thread scene state, and final draw submission. |
| Primitive or component | Accurate local bounds, world transform, mobility, LOD, draw distance, and render-state invalidation. |
| Camera subsystem | Accepted view, projection, near plane, field of view, viewport, and camera revision. |
| World-composition service | World Partition, Runtime Data Layers, streaming sources, HLOD membership, and region readiness. |
| Spatial build service | Deterministic converted cell evidence, weighted partition diagnostics, and versioned build artifacts. |
| Visibility-policy service | Explicit mission, interaction, interior, feature, or accessibility visibility commands. |
| Asset-streaming service | Admission, residency, cancellation, and release of streamable world content. |
| Diagnostics service | Bounds, cell, frustum, culling, occlusion, and overdraw inspection without gameplay mutation. |

<!-- markdownlint-enable MD013 -->

Culling is not streaming, collision, replication, navigation, or gameplay
activation. Those systems may consume related identities but remain separate
authorities.

## Runtime identities

The boundary uses stable identities for:

- `FSharWorldCompositionId`;
- `FSharWorldCompositionRevision`;
- `FSharRenderPrimitiveId`;
- `FSharVisibilityPolicyId`;
- `FSharVisibilityPolicyRevision`;
- `FSharSpatialBuildArtifactId`;
- `FSharSpatialCellId`;
- `FSharSpatialPartitionNodeId`;
- `FSharViewId`;
- `FSharViewRevision`; and
- `FSharVisibilityQueryId`.

Actor pointers, array offsets, cell order, tree memory addresses, package load
order, and render-thread proxy addresses are never durable identity.

## Bounds contract

Every renderable primitive exposes conservative finite world-space bounds. The
accepted projection contains:

- local box extent and sphere radius;
- world origin and transform revision;
- mobility and representation revision;
- optional positive and negative authored bounds extensions;
- active LOD or representation class;
- gameplay-relevance classification; and
- diagnostic source identity.

A conservative bound contains every rendered vertex for the accepted pose,
material displacement policy, attachment state, and representation. A bound may
be slightly larger than the geometry; it must not be smaller in a way that
causes visible popping or missing shadows.

Bounds never authorize collision, navigation, interaction, damage, streaming,
or mission availability.

## Bounds generation

Static-mesh, skeletal-mesh, procedural, instanced, particle, decal, and custom
primitive adapters generate bounds through their native Unreal contracts.

Import and build validation verifies:

- every component has finite bounds;
- minimum coordinates do not exceed maximum coordinates;
- sphere radius and box extents are non-negative;
- transformed bounds contain converted source geometry within tolerance;
- animated and attached content has an explicit update policy;
- material displacement is represented conservatively; and
- bounds extensions stay within approved limits.

Manually increasing bounds to hide a defect is not an accepted repair. The
underlying animation, attachment, transform, or import problem must be fixed
unless the extension is declared and justified by authored behavior.

## Dynamic bounds

Movable and animated primitives update their native render bounds when their
accepted pose, attachment, transform, or representation changes.

An update carries primitive, world, representation, and transform revisions.
Late work from an older pose or streamed representation cannot replace newer
bounds.

High-frequency updates are budgeted and batched through supported engine paths.
A custom component must document why native bounds calculation is insufficient
and must provide deterministic tests for every supported representation.

## Spatial build artifact

Converted spatial information is retained as a versioned build artifact and
verification surface, not as source identity or a mandatory custom runtime
renderer.

`FSharSpatialBuildArtifact` records:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `ArtifactId` | Stable build identity. |
| `WorldCompositionId` | World composition being analyzed. |
| `SourceRevision` | Exact normalized input revision. |
| `BuilderRevision` | Deterministic builder implementation revision. |
| `GranularityProfileId` | Named cell-size and weighting policy. |
| `PrimitiveSetRevision` | Canonical ordered primitive membership. |
| `Bounds` | Finite world-space build bounds. |
| `Cells` | Canonical non-empty cell records. |
| `Nodes` | Optional versioned partition diagnostics. |
| `Statistics` | Weight, depth, occupancy, overlap, and imbalance summaries. |
| `Digest` | Deterministic content digest. |

<!-- markdownlint-enable MD013 -->

The artifact may support conversion comparison, debugging, tooling, and tests.
Shipping runtime use requires an explicit demonstrated benefit over native
Unreal facilities and a separate accepted decision.

## Cell construction

A build profile divides accepted world bounds using positive finite
three-dimensional granularities. The builder:

1. validates world bounds and granularity;
1. computes integer extents with deterministic upper rounding;
1. clamps each dimension to at least one cell;
1. maps canonical primitive samples or bounds to candidate cells;
1. accumulates finite geometry evidence and declared render weight;
1. computes non-empty cell bounds, centroid, and weight;
1. removes empty cells from the published artifact; and
1. writes cells in canonical coordinate and identity order.

Cell membership uses stable primitive identities and explicit overlap policy. A
primitive spanning several cells is not arbitrarily assigned to one cell merely
because its centroid falls there.

## Cell record

`FSharSpatialCellRecord` contains:

- stable cell identity;
- integer block coordinate;
- finite world bounds;
- weighted centroid;
- primitive membership or canonical membership digest;
- aggregate render-cost evidence;
- gameplay-relevance summary;
- optional HLOD and World Partition correlation; and
- source and builder revisions.

Render weight is diagnostic evidence. It may estimate vertex, instance, section,
material, shadow, or measured cost according to a named profile. It cannot
change simulation or make required content disappear.

## Coordinate mapping

World-to-cell conversion uses a declared origin, finite granularity, and checked
integer arithmetic. It rejects:

- zero, negative, infinite, or not-a-number granularity;
- non-finite points or bounds;
- integer overflow;
- coordinates outside declared build bounds without overflow policy; and
- inconsistent coordinate-system or unit revisions.

Boundary points use one documented half-open interval rule. The maximum outer
boundary is included through an explicit terminal-cell rule rather than
floating-point accident.

## Spatial partition diagnostics

The build service may create a deterministic binary partition over non-empty
cells for analysis, serialization experiments, or tooling.

Each node records:

- stable node identity;
- finite node bounds;
- aggregate weight and membership count;
- split axis and split plane when internal;
- left and right child indices;
- depth and subtree size; and
- terminal reason when a leaf.

Published data uses checked indices in a versioned array. Pointer offsets,
recursive ownership, implicit sibling arithmetic, and in-memory object layout
are not a serialized contract.

## Split selection

A split profile declares:

- candidate axes;
- weighted-median or uniform strategy;
- bin count and deviation threshold;
- minimum cell count and weight per child;
- maximum depth;
- maximum imbalance;
- overlap handling; and
- deterministic tie-break order.

The builder prefers a split that creates two non-empty valid children and
reduces the declared cost. Equal candidates resolve by canonical axis, plane,
and child digest order.

A node becomes a leaf when no valid split improves the profile, not because a
raw fixed array is full.

## Weighted median

Weighted-median diagnostics sort cells by the selected axis using finite bounds,
centroid, and stable cell identity. The chosen plane seeks balanced aggregate
weight while respecting cell extents and avoiding invalid empty children.

Cells crossing the candidate plane follow a declared overlap or duplication
policy. The builder records resulting imbalance and overlap cost.

A median based only on mutable insertion order is invalid.

## Flattened partition artifact

An optional diagnostic tree is published as one immutable versioned array. Each
node stores checked child indices, parent index, subtree range, bounds, split
data, membership digest, and terminal reason.

Flattening uses canonical preorder and verifies:

- one root and no unreachable nodes;
- child ranges contained by the owning subtree;
- no cycles or duplicate ownership;
- finite bounds and split planes;
- parent and child bounds containment;
- deterministic subtree sizes; and
- identical digest across clean builds.

Runtime code cannot navigate the artifact through pointer subtraction, implicit
sibling offsets, mutable linked nodes, or recursive ownership. Scratch, reserve,
use, and swap arrays are implementation details replaced by bounded native
containers with explicit capacity and failure behavior.

## Traversal and marking diagnostics

Repository tooling may traverse a build artifact to compare native visibility or
to inspect converted world membership. A traversal request contains artifact,
world, view, query, filter, and revision identities.

The closed traversal results are:

- visited node set;
- accepted leaf set;
- rejected subtree set;
- intersecting subtree set;
- primitive membership projection; and
- validation or numerical findings.

Mark-all, sphere, frustum, subtree, union, intersection, and difference
operations
produce immutable bitsets or checked identity sets. They do not reuse mutable
per-node flags as cross-view state.

A traversal filter is a registered pure predicate over immutable node and
primitive metadata. It cannot call gameplay code, mutate streaming, or retain
raw entity pointers.

## Convex view volume

A visibility query may project the accepted camera into a finite convex volume.
The canonical runtime adapter uses Unreal's per-view convex-frustum facilities.
Repository diagnostics may record:

- normalized planes;
- near and optional far plane policy;
- finite corner points when derivable;
- view and projection revisions;
- viewport identity; and
- numerical tolerance profile.

Plane order and corner-array order are diagnostic schema details, not camera or
world identity.

## Convex intersection result

Bounds-versus-volume tests return one closed result:

- `outside`;
- `intersecting`;
- `inside`; or
- `invalid`.

Runtime rejection must be conservative. Uncertain numerical cases remain
visible or fall back to the engine path; they do not disappear.

A point, box, sphere, or convex proxy comparison cannot mutate visibility
policy, streaming state, or actor activation.

## Plane and point robustness

Plane normals are finite and normalized within tolerance. Triple-plane corner
construction rejects singular or nearly parallel combinations.

Authoring and conversion tools may use higher precision than the shipping
render representation. Conversion records tolerance, coordinate origin, and
precision revision so repeated builds remain comparable.

Not-a-number values, infinities, inverted bounds, and degenerate volumes fail
validation before publication.

## Runtime culling pipeline

The renderer applies supported culling methods according to native engine and
platform behavior. Repository policy configures and validates the result rather
than reproducing the renderer.

The conceptual order is:

1. explicit activation and visibility policy;
1. streaming and representation readiness;
1. distance eligibility;
1. per-view frustum eligibility;
1. supported precomputed visibility where enabled;
1. supported dynamic occlusion; and
1. final renderer submission.

The exact internal renderer implementation remains engine-owned. Tests assert
observable correctness and budgets, not private internal call order.

## Scene registration and membership

A primitive enters renderer visibility through native component registration.
Repository state records canonical entity, component, world, representation,
bounds, mobility, and registration revisions.

Registration requires:

- an accepted world and representation revision;
- finite conservative bounds;
- valid mobility, material, shadow, and visibility policy;
- ready assets and component render state; and
- no conflicting replacement or teardown transaction.

Moving, attaching, detaching, changing representation, or changing bounds uses
native transform and render-state invalidation. Repository code does not insert,
remove, or move a drawable through custom leaf-owned pointer lists.

World unload, Data Layer deactivation, component destruction, feature removal,
and representation replacement unregister the exact accepted component. A late
move or render callback cannot restore an older membership revision.

## Per-view marking and immutable visibility sets

Each accepted view produces independent renderer-owned visibility state.
Repository diagnostics may capture immutable comparison sets keyed by view,
world, frame, primitive, and representation revisions.

Diagnostic set operations may compute:

- conservative union for shared streaming evidence;
- intersection for multi-view optimization analysis;
- difference between native and converted diagnostics;
- subtree or region membership; and
- opaque, masked, translucent, shadow, or diagnostic classifications.

These sets are evidence only. Mutable marks stored inside shared tree nodes
cannot
be reused across local players, scene captures, cinematic cameras, shadow views,
or asynchronous diagnostics.

## Render-pass and shadow separation

Visibility admission does not manually submit opaque, translucent, or shadow
objects. Unreal's renderer owns mesh draw commands, material pass selection,
translucency sorting, depth behavior, shadow views, and final submission.

Repository policy may configure component and material participation in:

- opaque and masked base passes;
- translucent passes;
- depth and custom-depth passes;
- static, dynamic, virtual, contact, or capsule shadows;
- reflection and scene-capture views; and
- development-only visualization.

A camera-distance rank, shader name, linked-list order, or custom sort callback
is
not portable draw authority. A primitive can participate in several renderer
views without being duplicated as several world entities.

Shadow visibility is evaluated from its own accepted shadow view and component
policy. A main-view rejection does not by itself remove a valid shadow caster,
and a shadow pass cannot mutate gameplay or persistent visibility.

## Distance culling

Distance policy may come from component settings, Cull Distance Volumes, HLOD,
foliage or instance policy, or a validated feature overlay.

Every distance rule declares:

- affected content class;
- minimum and maximum range when applicable;
- platform and quality applicability;
- fallback representation;
- gameplay-relevance classification; and
- validation camera set.

Required route landmarks, mission indicators, collision, navigation, and
interaction cannot be removed merely to satisfy a rendering benchmark.

## View-frustum culling

Each accepted view has its own frustum. Split-screen, scene captures, mirrors,
cinematics, minimaps, and editor diagnostics do not share one process-global
visibility result.

The camera subsystem owns view construction. The renderer owns primitive tests.
Repository diagnostics may compare accepted primitive bounds against a captured
view volume to explain a result, but that comparison is not a second draw list.

## Occlusion

Dynamic occlusion remains a renderer facility. Policy may select supported
methods by platform and quality profile, but cannot reinterpret a delayed or
uncertain query as gameplay absence.

Potential one-frame query latency, rapid-camera exposure, and bounds quality are
covered by visual tests. Conservative visibility is preferred over a false
negative.

Precomputed visibility may be used only where its world, mobility, memory, and
platform assumptions are valid. It does not replace World Partition or dynamic
occlusion universally.

## LOD, HLOD, and Nanite

LOD, HLOD, and Nanite choose a representation for visible content. They do not
change canonical world-object identity, collision authority, mission state, or
save state.

A visibility profile records representation coverage, transition thresholds,
material and shadow policy, collision behavior, and required fallback.

HLOD and World Partition cells are generated artifacts. Their numeric indices
or package order do not become durable gameplay identities.

## World Partition and streaming

Frustum or occlusion rejection does not imply that an asset is unloaded.
Streaming admission uses world composition, streaming sources, distance,
priority, dependency, memory, and lifecycle policy.

A streaming source may use camera and participant projections, but its request
is revision-correlated and owned by the streaming coordinator. A renderer result
cannot directly unload a region or cancel a gameplay-critical dependency.

## Explicit gameplay visibility

Mission, interaction, interior, chapter, and feature systems may request
explicit visibility through `FSharVisibilityPolicyCommand`.

A command includes:

- target canonical identity;
- owner and owner revision;
- desired visible, hidden, activated, or deactivated state;
- collision and navigation policy when relevant;
- transition and presentation policy;
- priority and conflict rules; and
- cancellation and restoration behavior.

The visibility-policy service validates and commits the command. Widgets,
cinematic tracks, culling queries, and renderer callbacks cannot commit it.

## Cinematic and presentation views

Level Sequences, media overlays, presentation cameras, and snapshot cameras use
ordinary accepted views and scoped visibility leases.

A cinematic may intentionally hide or reveal registered content through an
explicit presentation policy. Camera cuts and temporary view changes do not
mutate durable world visibility or streaming identity.

Stopping, skipping, cancellation, world teardown, and sequence replacement
restore only the state owned by the exact presentation lease.

## Local multiplayer and multiple views

Each local player owns an independent view and frustum revision. A primitive may
be visible in one viewport and rejected in another.

Shared streaming and representation policy may use the conservative union of
required local views plus declared prefetch margins. One player's occlusion
result cannot hide content from another player's view.

## Platform and quality policy

All five graphics presets preserve gameplay and required visual semantics.
Culling quality and cost may vary through supported engine settings, but the
world, missions, collision, navigation, progression, and package identities do
not.

Android remains constrained to the accepted `Low` policy. Precomputed
visibility,
distance rules, HLOD, instance policy, and occlusion choices are profiled on
representative Android ARM64 hardware rather than copied blindly from desktop.

## Feature and mod overlays

A validated feature may add visibility profiles, cull-distance rules, bounds
extensions, diagnostics, or explicit visibility commands within its namespace.

It must declare:

- target identities and compatibility revisions;
- affected platforms and quality profiles;
- gameplay-relevance impact;
- teardown and restoration behavior;
- validation views and budgets; and
- ownership of generated build artifacts.

Feature removal unregisters owned policy and generated artifacts atomically. It
cannot leave hidden base content, stale bounds extensions, or renderer state
that refers to removed packages.

## Concurrency

Game-thread policy and world-composition state publish immutable snapshots.
Unreal Engine transfers supported render-state changes to its render thread.
Repository code does not read or mutate render-thread scene-proxy storage
directly.

Build tools may parallelize cell accumulation and statistics only when output
ordering and floating-point reduction are deterministic. Final publication uses
canonical identity order and one digest.

## Diagnostics

Diagnostics may expose:

- primitive and aggregate bounds;
- bounds-source and update revisions;
- active view and frustum visualization;
- distance and explicit visibility policy;
- World Partition and HLOD correlation;
- cell occupancy, weight, centroid, and overlap;
- partition depth, subtree ranges, terminal reasons, and imbalance;
- traversal visits, accepted leaves, and rejected subtrees;
- native-versus-converted diagnostic set differences;
- scene registration and transform revision;
- per-view opaque, translucent, shadow, and diagnostic classifications;
- rejected, intersecting, and accepted counts;
- occlusion and overdraw evidence exposed by supported engine tools;
- stale update and invalid-number counts; and
- platform and quality profile.

Diagnostics are read-only. Enabling them cannot alter culling, streaming,
collision, timing, or gameplay.

## Failure behavior

The boundary fails closed on invalid authored or generated policy and fails
conservatively visible on uncertain runtime rejection.

Failures include:

- non-finite, inverted, or insufficient bounds;
- unsupported bounds extension;
- zero or invalid granularity;
- coordinate or integer overflow;
- non-deterministic cell membership or partition output;
- malformed flattened tree, child range, parent link, or subtree size;
- traversal with a stale artifact or impure filter;
- empty or invalid child partitions;
- malformed plane or convex-volume data;
- duplicate or stale scene registration;
- shared mutable visibility marks across views;
- repository-owned manual draw submission or shadow sorting;
- stale view, primitive, world, or feature revision;
- culling policy that removes gameplay-required semantics;
- explicit visibility mutation without an owning transaction;
- platform policy with no measured validation; and
- feature removal with unreleased policy or generated artifacts.

A failure never silently changes gameplay. Invalid content is blocked during
validation; uncertain runtime visibility remains conservative and diagnostic.

## Validation

Content and build validation proves:

- bounds are finite, conservative, and representation-aware;
- world-to-cell mapping is deterministic at boundaries;
- non-empty cells and memberships have stable identities;
- weighted partition output is deterministic and bounded;
- flattened node indices, parent links, subtree ranges, sizes, and leaf reasons
  are valid;
- traversal and set operations are immutable, deterministic, and revisioned;
- scene registration and transform invalidation use native engine paths;
- no shared mutable mark state crosses accepted views;
- render-pass and shadow participation remains renderer-owned;
- convex-volume tests agree with native engine tests within tolerance;
- every gameplay-relevant primitive has a valid visibility policy;
- LOD and HLOD retain required silhouettes and landmarks;
- each supported platform and quality profile has measured evidence;
- split-screen and cinematic views remain isolated; and
- feature removal restores base policy and releases generated artifacts.

## Tests

Required automated and visual tests include:

- bounds accumulation and transform containment;
- sphere-versus-box boundary cases;
- cell-coordinate minimum, interior, boundary, and maximum cases;
- zero and invalid granularity rejection;
- deterministic non-empty cell extraction;
- weighted-median and uniform split tie-breaks;
- unsplittable and maximum-depth leaves;
- flattened node, parent, child, subtree, and digest validation;
- immutable traversal union, intersection, difference, and subtree marking;
- stale artifact, view, and filter rejection;
- native component registration, move, detach, replacement, and teardown;
- independent opaque, translucent, shadow, scene-capture, and local-player
  views;
- box, sphere, point, and convex-volume classification;
- degenerate and nearly parallel plane rejection;
- fast camera rotation and occlusion recovery;
- dynamic and skeletal bounds updates;
- split-screen disagreement between views;
- cinematic cut and cancellation restoration;
- Low through Ultra visibility parity for required content;
- Android Low performance and required-content visibility;
- stale update rejection; and
- feature activation and removal cleanup.

## Invariants

- Unreal Engine owns final per-view render visibility and occlusion.
- Bounds are finite, conservative, and revision-correlated.
- Culling never becomes gameplay, streaming, collision, or navigation authority.
- Converted cells and partition trees are versioned evidence, not pointer-based
  runtime identity.
- Flattened trees use checked indices and immutable traversal results.
- Native component registration owns runtime scene membership.
- Cell construction and partition output are deterministic.
- Runtime rejection is conservative when numerical certainty is unavailable.
- Every local, cinematic, capture, or shadow view has an independent visibility
  result.
- Unreal owns material pass selection, translucency sorting, and shadow views.
- Quality presets may change cost and representation, not required semantics.
- Explicit gameplay visibility requires a typed owning transaction.
- Teardown releases only policy and artifacts owned by the exact revision.

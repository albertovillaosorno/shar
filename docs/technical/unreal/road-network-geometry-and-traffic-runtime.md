# Road-network geometry and traffic runtime

- Status: Active
- Last reviewed: 2026-07-16

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal Unreal runtime](../../adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](native-import-material-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle AI and route runtime](vehicle-ai-and-route-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Pedestrian path runtime](pedestrian-path-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored spatial placement and trigger runtime](authored-spatial-placement-and-trigger-runtime.md)

## Purpose

This specification defines the cooked road graph, authored road geometry, lanes,
segments, intersections, traffic control, speed and density policy, route
connectivity, closest-point queries, deterministic traversal, streaming,
diagnostics, and teardown.

It replaces fixed-capacity road and intersection pools, raw linked-list
geometry,
source control-point arrays used directly at runtime, insertion-order adjacency,
process-wide road-manager state, mutable path-element arrays, source hashes as
identity, and path search that depends on allocation or pointer order.

The road graph is authored world data and query infrastructure. It does not own
vehicle physics, artificial-intelligence decisions, mission progression,
navigation-mesh authority, or traffic-vehicle lifetime.

## Native Unreal foundation

The boundary uses native Unreal facilities where they fit:

- cooked Data Assets or equivalent native immutable graph assets;
- spline components or cooked spline samples for authored centerlines and lane
  geometry;
- native transforms, vectors, bounds, and curve interpolation;
- World Partition, Runtime Data Layers, and level-instance ownership;
- native navigation projection where pedestrian or off-road behavior requires
  it;
- world subsystems for runtime graph access;
- Mass, StateTree, or project traffic controllers as selected by accepted
  traffic
  decisions; and
- Asset Manager bundles and retained handles.

The project owns canonical road semantics, graph validation, deterministic query
results, and traffic admission policy. It does not replace Unreal's renderer,
physics, or general navigation system.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Road catalog | Owns stable road, segment, lane, intersection, control, shortcut, and route identities. |
| Import pipeline | Converts normalized road evidence into deterministic cooked graph assets and spline data. |
| Road-network subsystem | Publishes immutable graph snapshots and bounded deterministic queries. |
| Traffic subsystem | Owns lane occupancy, intersection reservations, density, spawn admission, and ambient lifecycle. |
| Vehicle route follower | Consumes road projections, route paths, lane choices, and look-ahead results. |
| Pedestrian and navigation services | Consume eligible crossing, sidewalk, path, and navigation projections. |
| Mission and race services | Own route objectives, checkpoints, closures, completion, failure, and shortcut eligibility. |
| World-composition service | Owns region readiness, overlays, streaming, and teardown. |
| Scene-query service | Owns normalized physical traces and may correlate hits with road metadata. |

<!-- markdownlint-enable MD013 -->

No consumer mutates the cooked base graph directly.

## Runtime identities

The boundary uses stable identities for:

- `FSharRoadNetworkId`;
- `FSharRoadNetworkRevision`;
- `FSharRoadId`;
- `FSharRoadRevision`;
- `FSharRoadSegmentId`;
- `FSharLaneId`;
- `FSharLaneRevision`;
- `FSharIntersectionId`;
- `FSharIntersectionRevision`;
- `FSharTrafficControlId`;
- `FSharRoadConnectionId`;
- `FSharRoadPathId`;
- `FSharRoadQueryId`;
- `FSharRoadQueryRevision`;
- `FSharWorldCompositionRevision`; and
- `FSharFeatureRevision`.

Array positions, pool slots, source object pointers, source hashes,
control-point
indices, linked-list nodes, and mutable path-element addresses are not durable
identity.

## Road-network asset

`USharRoadNetworkDefinition` contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `NetworkId` | Canonical world road-network identity. |
| `WorldId` | Owning world identity. |
| `CoordinateRevision` | Accepted coordinate-system and unit revision. |
| `Roads` | Canonically ordered road definitions. |
| `Segments` | Canonically ordered geometric and query segments. |
| `Lanes` | Directional lane definitions and samples. |
| `Intersections` | Intersection geometry, connectivity, and control definitions. |
| `Connections` | Directed legal graph edges and costs. |
| `TrafficPolicies` | Density, speed, control, occupancy, and admission definitions. |
| `SpatialIndex` | Optional deterministic cooked acceleration data. |
| `StreamingPolicy` | Region, bundle, overlay, and teardown behavior. |
| `Digest` | Deterministic content digest. |

<!-- markdownlint-enable MD013 -->

The graph is immutable after publication. Runtime changes use namespaced
overlays
or typed temporary policy snapshots.

## Road definition

`FSharRoadDefinition` contains:

- canonical road identity and revision;
- source and destination intersection identities;
- ordered segment identities;
- road class and semantic tags;
- direction policy;
- lane count and lane identities;
- width and shoulder policy;
- speed limit and difficulty policy;
- traffic density profile;
- shortcut classification;
- length and bounds;
- physical-surface and visual references;
- streaming region; and
- validation evidence.

A road identity is stable across rebuilds when its canonical authored identity
and
semantics remain the same. Segment count and array order are not identity.

## Authored geometry

Road geometry is represented by finite validated curves and samples. Import may
consume authored control points, cubic curves, polylines, or normalized segment
evidence and produce:

- native spline data;
- deterministic arc-length samples;
- tangent and normal frames;
- lane offsets;
- bounds and spatial-index records;
- physical-surface and terrain metadata; and
- source provenance.

Shipping runtime does not maintain a custom doubly linked list of control points
or rebuild source curves from arbitrary insertion order.

## Curve conversion

Curve conversion validates:

- finite control points and tangents;
- supported coordinate space and units;
- minimum segment length;
- continuity and curvature limits;
- deterministic sample tolerance;
- tangent and normal stability;
- lane-width compatibility;
- intersection join constraints; and
- identical output digest across clean builds.

Cubic curve evaluation is an import or authoring concern unless runtime
deformation
is explicitly required by an accepted feature. Cooked route following consumes
validated spline or sample data.

## Road segments

A road segment is the minimum canonical geometric and query interval. It
records:

- road and segment identities;
- start and end distance along the road;
- centerline samples or native spline interval;
- lane sample references;
- width and bounds;
- direction and grade;
- physical surface and terrain classification;
- adjacent segment identities;
- source and destination intersection relationships; and
- streaming region.

Segment joins are validated for positional and directional continuity. A gap or
self-intersection beyond declared tolerance fails publication.

Every segment also publishes deterministic geometric query evidence:

- four canonical boundary corners or equivalent cooked boundary representation;
- left and right edge normals;
- segment plane or surface normal;
- road-facing and cross-road axes;
- segment length, width, lane width, and lane count;
- start and end height and grade;
- unit distance and unit height projection;
- lane-center position at normalized progress;
- join points and neighboring-segment evidence;
- local-to-world transform and accepted scale; and
- conservative box and sphere bounds.

Queries reject non-finite transforms, degenerate axes, invalid lane ordinals,
progress outside declared policy, and geometry from a retired graph revision.
Runtime interpolation uses the cooked representation; it does not reconstruct
source segment objects or expose mutable corner pointers.

## Lane definition

`FSharLaneDefinition` contains:

- stable lane identity;
- owning road identity;
- direction of travel;
- ordered geometric samples;
- start and end intersection connections;
- speed limit;
- desired density policy;
- vehicle-class eligibility;
- turn eligibility;
- merge and split relationships;
- shoulder and stopping policy; and
- streaming and overlay ownership.

Lane density is validated as finite and non-negative. Desired density is traffic
policy, not a guarantee that vehicles are spawned.

## Lane sampling

Lane samples provide position, tangent, normal, curvature, distance along lane,
and owning segment identity.

Queries interpolate between validated samples or use the native spline. They
return canonical lane and segment identities plus normalized progress.

A caller cannot retain a pointer to a lane sample or assume the sample count is
stable across cooked revisions.

## Intersection definition

`FSharIntersectionDefinition` contains:

- canonical intersection identity;
- finite center and bounds;
- connected incoming and outgoing road identities;
- legal lane-to-lane movements;
- turn classification;
- traffic-control identity;
- conflict groups;
- crossing and pedestrian policy;
- priority and right-of-way policy;
- streaming region; and
- validation evidence.

The number of connected roads is bounded by definition policy rather than a
source fixed array. Unsupported complexity fails authoring validation or is
represented by multiple canonical intersections.

## Intersection geometry

Intersection geometry defines an accepted region and join points for connected
roads and lanes. It validates:

- road-end proximity;
- tangent compatibility;
- non-degenerate area;
- lane movement continuity;
- physical-surface and terrain policy;
- camera and collision relevance; and
- world-coordinate precision.

Point-in-intersection tests use the accepted native or cooked geometry and
return a
typed result. They do not depend on source road-list order.

## Traffic control

Traffic-control definitions include:

- uncontrolled or priority-controlled admission;
- stop, yield, signal, timed, event-driven, or scripted policy;
- movement conflict groups;
- minimum and maximum waits;
- fairness and starvation policy;
- emergency or mission priority;
- participant eligibility;
- pause and world-time behavior; and
- fallback when the controller is unavailable.

Source enum values such as no-stop or N-way are conversion evidence. Runtime
uses
closed semantic identities and definitions.

A signal policy may use semantic phases such as `red`, `yellow`, `green`, and
`advance_green`. Each phase declares eligible movement groups, minimum and
maximum
duration, transition constraints, world-time and pause behavior, emergency or
mission override, synchronization group, and deterministic next-phase selection.

Signal phase changes publish revisioned observations. A vehicle still requires
an
accepted reservation or admission result; seeing a green presentation cannot
grant entry by itself. Fixed road ordinals and one magic turn duration are
import
evidence only.

## Intersection reservations

A vehicle requests intersection admission with:

- vehicle and controller revisions;
- source lane and requested outgoing lane;
- intersection and traffic-control revisions;
- expected arrival interval;
- vehicle dimensions and class;
- mission or emergency priority;
- current occupancy snapshot; and
- cancellation token.

The traffic subsystem returns granted, queued, rejected, expired, cancelled, or
superseded. A granted reservation has a bounded validity interval and stable
identity.

A render, proximity, or lane query cannot grant intersection admission directly.

## Legal movements

Each directed lane movement records:

- source lane;
- destination lane;
- left, straight, right, U-turn, merge, split, or continuation classification;
- geometric path or spline interval;
- conflicting movement groups;
- vehicle-class restrictions;
- signal or priority requirements;
- shortcut and mission policy; and
- traversal cost.

Turn selection is deterministic from route, eligibility, traffic state, and
stable
identity. Mutable road insertion order cannot select a turn.

## Connectivity graph

The cooked directed graph contains road, segment, lane, intersection, and
movement
edges as appropriate for the query family.

Every edge records:

- source and destination canonical identities;
- legal direction;
- base distance and traversal cost;
- speed and difficulty modifiers;
- shortcut and closure policy;
- vehicle-class eligibility;
- world and overlay ownership; and
- deterministic tie-break identity.

Graph publication proves there are no dangling references and that required
reachable pairs remain connected.

## Route queries

`FSharRoadPathRequest` contains:

- request, owner, world, and graph revisions;
- source and destination projections;
- eligible road, lane, and vehicle classes;
- direction policy;
- shortcut and difficulty policy;
- closures and mission overlays;
- cost profile;
- maximum visited nodes and path length;
- timeout and cancellation; and
- deterministic tie-break policy.

The result contains terminal status, ordered canonical path elements, total
distance, total cost, selected lane movements, truncation evidence, and graph
revision.

## Deterministic pathfinding

Path search uses one declared deterministic algorithm and stable ordering.
Equal-cost candidates resolve by:

1. lower declared secondary cost;
1. canonical intersection identity;
1. canonical road identity;
1. canonical lane or movement identity; and
1. stable path digest.

Heap insertion order, pointer value, source allocation order, and hash-table
iteration cannot change the selected route.

Search bounds and cancellation are explicit. An incomplete path never
masquerades
as success.

## Closest-road and closest-path queries

A closest-road request declares:

- world and graph revisions;
- finite world position and radius;
- eligible road and lane classes;
- direction and vehicle filters;
- interior, mission, and streaming filters;
- whether centerline, lane, or surface distance is required; and
- maximum candidates.

The result contains road, segment, lane, normalized progress, closest point,
tangent, signed lateral offset, distance, and classification.

Equal candidates resolve by distance tolerance and canonical identity. A raw
tree
or pool traversal order cannot choose the result.

## Road progress

Road, segment, and lane progress use normalized parameters plus physical
distance.
Conversion functions validate finite input and clamp only according to declared
query policy.

`DetermineRoadT`-style and segment-parameter operations become typed helpers
that
return success or failure. They cannot write invalid progress into a controller.

## Traversal distance

Traversal-distance queries compute directed distance through one accepted graph
revision. They declare source and destination path elements, direction,
shortcut policy, closures, and maximum traversal.

The result distinguishes reachable, unreachable, invalid, cancelled, and bounded
search exhaustion. Negative or not-a-number distance never becomes a valid
result.

## Segment lookup

Finding a segment at a point or distance uses validated bounds and geometry.
Queries return stable segment identity and progress rather than a raw segment
pointer.

Ahead and behind lookup follows legal road order and direction. A caller cannot
step beyond the road silently or wrap without an explicit loop policy.

## Speed limits

Speed limits use simulation units and named profiles. The definition records:

- base speed;
- vehicle-class overrides;
- quality-independent gameplay semantics;
- weather, mission, damage, or traffic overlays;
- temporary closure or hazard policy; and
- diagnostics display units.

Unit conversion such as kilometers per hour to meters per second occurs through
one tested unit boundary. Display units cannot alter simulation values.

## Density policy

Traffic density profiles declare desired vehicles per distance or another named
normalized measure, eligible times and chapters, platform budget, population
class, and fallback.

The traffic subsystem uses density as one input to spawn and retention
decisions.
It also considers visibility, streaming, road capacity, intersection load,
participant proximity, memory, and performance.

Density never allocates fixed lane arrays or guarantees a vehicle count.

## Shortcuts and difficulty

Shortcut and difficulty values are typed authored policy. A shortcut edge
declares
eligibility, discoverability, route class, mission restrictions, artificial-
intelligence skill threshold, and cost adjustment.

Difficulty may affect controller policy and route selection only through
accepted
definitions. It cannot change graph identity or silently make a required route
unreachable.

## Road closures and overlays

Missions, chapters, construction, hazards, interiors, or mods may submit a typed
road-policy overlay containing:

- owner and overlay revision;
- affected roads, lanes, movements, or intersections;
- closure, direction, cost, speed, density, or control changes;
- participant eligibility;
- activation and expiration conditions;
- priority and conflict policy; and
- teardown behavior.

The base graph remains immutable. Overlay composition is deterministic and
revisioned.

## Traffic integration

The traffic subsystem consumes the accepted graph and policy snapshot for:

- lane occupancy;
- spawn and despawn admission;
- intersection reservations;
- legal turn selection;
- speed and density targets;
- route continuation;
- impedance and blockage observations; and
- streaming lifecycle.

Traffic vehicles own their controllers and movement requests. The road graph
does
not tick vehicles or apply throttle, brake, or steering.

Traffic projection, lane-change curves, intersection entry, and the verified
handoff between lightweight road movement and dynamic Chaos simulation follow
<!-- markdownlint-disable-next-line MD013 -->
[Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md).
A traffic instance cannot become player-controlled or physically simulated by
swapping a raw locomotion pointer or retaining stale lane state.

## Vehicle-AI integration

Vehicle controllers project onto the road graph through typed queries and retain
canonical road, segment, lane, route, and graph revisions.

Route following consumes look-ahead samples and legal movement results. A graph
revision change, closure, teleport, or lost projection invalidates stale
progress
and requests bounded replanning.

The controller cannot modify the base graph or reserve an intersection by
writing
lane state directly.

## Pedestrian and crossing integration

Pedestrian services may consume sidewalks, crossings, intersection bounds, and
traffic-control observations when those semantics are authored.

General pedestrian pathfinding remains owned by the pedestrian and native
navigation boundary. Vehicle centerlines are not automatically pedestrian paths.

Crossing admission and signal observations are typed and participant-specific.

## Scene-query integration

Physical line and shape traces remain owned by the scene-query service. Road
metadata may correlate a hit or world point with cooked road and surface
identities.

A visual road mesh, collision triangle, or physical-surface hit is evidence, not
canonical road identity by itself.

## Import and cooking

Import converts normalized road evidence into:

- canonical road, segment, lane, intersection, control, and connection
  identities;
- deterministic spline and sample assets;
- directed graph edges and cost profiles;
- spatial-index artifacts;
- physical-surface and terrain mappings;
- World Partition and Runtime Data Layer ownership;
- primary-asset and bundle metadata;
- source provenance and content digests; and
- validation reports.

Cook read-back verifies every identity, reference, curve, connection, cost,
bounds, and platform variant.

## Spatial index

An optional cooked spatial index accelerates closest-road, radius, and region
queries. It is versioned, deterministic, and derived from canonical graph data.

The index may use native spatial structures or project-owned immutable build
data.
It cannot become graph identity or gameplay authority. Invalid or missing index
data falls back to a bounded canonical query or blocks publication according to
policy.

## Streaming

Road graph assets may be globally resident or region-partitioned according to
the
world design. Every path query declares whether all required regions are loaded,
may be prefetched, or must return unavailable.

Region activation validates cross-boundary connections before traffic and route
queries become ready. Region unload removes runtime bindings but not canonical
identity.

A path cannot contain a missing region unless the result explicitly represents a
prefetch or partial planning state.

## Memory and capacity

Runtime graph storage uses validated native containers and cooked counts. Source
maximum constants do not define the public capacity.

Budgets cover roads, segments, lanes, intersections, movements, spatial-index
entries, active queries, and overlays. Exceeding a budget fails cook, rejects an
overlay, or returns typed query exhaustion. It never overwrites graph data.

## Concurrency

The road-network subsystem publishes immutable graph and overlay snapshots.
Queries may execute concurrently over one exact snapshot when supported.

Results carry graph, overlay, world, and request revisions. A late result from a
retired graph or unloaded region is rejected before controller mutation.

Traffic reservations use one authority and deterministic conflict arbitration.

## Road visualization and diagnostic rendering

Road, lane, segment, terrain, spawn, bounds, tangent, normal, connection,
signal,
and reservation visualization is development-only presentation through native
debug drawing or registered diagnostic components.

Visualization requests carry graph, overlay, world, local-player, view, and
request revisions. They may select loaded regions, road identities, terrain
classes, spawn-eligible segments, boxes, spheres, or another bounded diagnostic
filter.

Diagnostic rendering cannot:

- create or mutate road geometry;
- change terrain or physical-surface identity;
- reserve or release an intersection;
- spawn traffic;
- affect path cost or closest-road results;
- become shipping renderer authority; or
- keep an unloaded graph revision alive.

Visual tests compare deterministic query and geometry evidence rather than
relying
on one process-global debug singleton or mutable display flags.

## Diagnostics

Development diagnostics may expose:

- graph, world, overlay, and cook revisions;
- roads, segments, lanes, intersections, movements, and bounds;
- spline samples, tangents, curvature, and continuity findings;
- speed, density, shortcut, and difficulty policy;
- traffic controls and active reservations;
- route-search visited nodes, costs, tie-breaks, and terminal status;
- closest-road candidates and selected result;
- spatial-index occupancy and fallback;
- region readiness and cross-boundary connections; and
- stale query, invalid geometry, and budget findings.

Diagnostics are read-only. They cannot add a road, reserve an intersection,
change traffic state, or complete a mission.

## Failure behavior

The boundary fails closed on:

- duplicate or missing canonical identity;
- non-finite control point, tangent, sample, length, width, speed, or density;
- degenerate curve or segment;
- discontinuous road or lane join beyond tolerance;
- dangling road, lane, segment, intersection, or movement reference;
- illegal direction or impossible movement;
- nondeterministic path or closest-road result;
- unknown traffic control or conflicting control policy;
- stale graph, overlay, world, region, or request revision;
- fixed pool exhaustion without typed failure;
- source pointer or hash used as runtime identity;
- query success with truncated or unreachable output;
- region unload with active uncorrelated reservations; and
- feature removal with retained overlay or graph bindings.

Failure returns typed evidence and preserves the last accepted graph snapshot.

## Validation

Cook and definition validation prove:

- every graph identity and reference resolves;
- curve and sample output is finite and deterministic;
- every segment has valid corners, axes, normals, height, lane width, transform,
  progress projection, and conservative bounds;
- road, lane, and intersection joins satisfy tolerance;
- lane directions and legal movements are complete;
- signal phases, transitions, timings, and movement groups are valid;
- required route pairs remain reachable;
- deterministic path and closest-road tie-breaks are stable;
- speed, density, shortcut, and difficulty units are valid;
- traffic-control conflict groups are consistent;
- spatial-index output matches canonical geometry;
- region boundaries preserve required connections;
- overlays are deterministic and removable;
- no source fixed pool or pointer identity survives; and
- every target cooks the required graph assets.

## Tests

Required automated and visual tests include:

- cubic curve and spline conversion;
- finite, degenerate, and discontinuous geometry rejection;
- lane offset, tangent, normal, and progress interpolation;
- segment corner, edge-normal, plane-normal, height, transform, lane-center,
  progress, box, and sphere queries;
- road-segment point and distance lookup;
- intersection point containment and join validation;
- legal left, straight, right, merge, split, and continuation movements;
- red, yellow, green, and advance-green phase transitions;
- traffic-control timing, pause, override, movement-group, reservation,
  fairness,
  expiry, and cancellation;
- deterministic equal-cost route selection;
- unreachable, bounded, cancelled, and stale path results;
- closest-road and closest-lane tie-breaks;
- directed traversal distance;
- speed-unit conversion and overlay behavior;
- density policy without fixed allocation assumptions;
- shortcut eligibility and mission closure overlays;
- graph revision change and vehicle replanning;
- cross-region path readiness and unload;
- feature overlay activation and complete removal;
- diagnostics with no behavior change; and
- identical graph digest across clean imports.

## Invariants

- The cooked road graph is immutable and canonically identified.
- Roads, segments, lanes, intersections, and movements use stable identities.
- Native splines and cooked samples replace source linked geometry.
- Path and closest-road results are bounded, revisioned, and deterministic.
- Traffic control owns intersection admission; render and proximity do not.
- The graph does not own vehicle physics, steering, missions, or persistence.
- Source pool slots, pointers, hashes, and insertion order are never identity.
- Overlays never mutate the base graph and are completely removable.
- Streaming cannot reinterpret canonical road identity.
- Every retired graph or overlay releases all runtime bindings and reservations.

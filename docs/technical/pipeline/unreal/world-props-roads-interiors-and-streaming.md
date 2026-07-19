# World, props, roads, interiors, and streaming

- Status: Active
- Last reviewed: 2026-07-18

## World model

The base game uses one connected World Partition open world:

```text
/Game/SHAR/Maps/OpenWorld/W_SHAR_OpenWorld
```

Narrative chapters are progression and content-activation states, not seven
isolated runtime levels. Geography, roads, landmarks, interiors, mission layers,
population layers, collectibles, and chapter presentation use stable definitions
and Runtime Data Layers.

## Coordinates

Final world geometry uses centimeters, positive X forward for local asset
fronts, positive Z up, and one documented global origin. Pipeline world
placement records contain double-precision source transforms and deterministic
origin rebasing where required. Native read-back verifies translation, rotation,
scale, bounds, and pivot.

## Asset decomposition

A world source assembly is placement evidence. Shipping assets are decomposed
into independently reusable native components:

- terrain and broad ground surfaces;
- road segments, intersections, sidewalks, curbs, and lane metadata;
- buildings and structural modules;
- doors, windows, roofs, signs, and architectural props;
- vegetation, rocks, street furniture, and decals;
- collision and query surfaces;
- interiors and reusable Level Instances;
- mission-only or chapter-specific variants;
- breakable, animated, interactive, or stateful props.

One giant mesh per historical area is rejected. Arbitrary fragmentation per
source node is also rejected. Boundaries follow streaming, HLOD, collision,
material, interaction, reuse, and mod replacement needs.

## Canonical placement

```text
/Game/SHAR/Data/Worlds/open_world/DA_World_open_world
/Game/SHAR/Data/Locations/<location_id>/DA_Location_<location_id>
/Game/SHAR/Art/World/open_world/<region_id>/<asset_id>/SM_World_<asset_id>
/Game/SHAR/Art/Props/<prop_id>/SM_Prop_<prop_id>
/Game/SHAR/Maps/Interiors/LI_Interior_<interior_id>
/Game/SHAR/Data/Tables/Roads/DT_RoadSegments
/Game/SHAR/Data/Tables/Roads/DT_RoadLanes
/Game/SHAR/Data/Tables/Roads/DT_RoadIntersections
```

## Static mesh contract

Static meshes declare semantic identity, pivot policy, material roles, collision
profile, Nanite policy, LOD or HLOD policy, lightmap policy, bounds, mobility,
physical material, placement ownership, and quality profile.

- opaque non-deforming geometry uses Nanite by default when platform support and
  material features permit it;
- masked, translucent, deforming, World Position Offset, or mobile-restricted
  assets use declared native fallback profiles;
- collision uses simple primitives, complex-as-simple only for approved static
  query surfaces, or generated convex decomposition according to profile;
- visual meshes never silently become gameplay navigation or trigger authority.

Pivots follow semantic use: doors pivot at hinges, wheels at axles, reusable
building modules at a documented snap origin, ordinary props at stable ground or
placement origins, and large geography at deterministic world-grid anchors.

## Roads and traffic

Road data is a typed graph, not inferred from rendered road meshes. It declares
segments, lanes, directions, widths, speed policy, intersections, legal
movements, traffic controls, pedestrian crossings, parking, pursuit access,
recovery points, and deterministic spline geometry. Rendered meshes and decals
reference this graph.

Traffic and pedestrian simulation may use ZoneGraph and Mass representations.
The pipeline produces explicit graph records and placement identities; runtime
does not recover topology from filenames or triangle adjacency.

## Interiors

Each structure declares one interior capability: none, linked Level Instance,
streamed interior, mission-only interior, or extension slot. Interior identity,
entry and exit portals, visibility, audio, lighting, navigation, save policy,
and world-state bindings are explicit. An exterior mesh never implies an
interior.

## Data Layers

Data Layer identities are semantic and stable, for example:

- `base_geography`;
- `chapter_01_content` through `chapter_07_content`;
- `mission_<id>`;
- `collectibles_<chapter>`;
- `interior_<id>`;
- `world_state_<id>`;
- `mod_<namespace>_<layer>`.

Runtime activation is transactional. Mission completion, save loading, and mod
activation cannot leave half-applied layer sets.

## HLOD and streaming

World Partition cell sizing, loading range, HLOD layer, and runtime grid are
platform profiles rather than asset-name conventions. The importer generates and
validates HLOD inputs, but final HLOD assets remain native generated output.

Streaming dependencies are one-way and bounded. A cell may reference shared
secondary assets and definitions; it cannot force-load an unrelated chapter,
mission, or complete catalog. Camera and high-speed vehicle profiles may request
predictive streaming through world services.

## Lighting and time of day

The world supports a continuous 24-real-minute day cycle. Lighting, sky,
weather, fog, emissive windows, street lights, exposure, audio, and population
consume world clock and chapter state. Assets declare day and night capabilities
and material parameters; they do not contain duplicated geometry unless a
registered presentation profile requires it.

Chapter-specific weather, including the irradiated Chapter 7 presentation, is a
world-state profile layered over the same geography. It does not duplicate the
entire map.

## Navigation and gameplay surfaces

Walkable, drivable, climbable, breakable, interactive, damaging, slippery, wet,
interior, and restricted surfaces are explicit semantic records and native
collision or navigation data. Render material names do not become gameplay
classification.

## Mod extension

Mods may add namespaced World Partition content, Data Layers, Level Instances,
locations, roads, interiors, props, and world-state rules through validated Game
Feature actions. They cannot mutate base geography implicitly or collide with
base identities without declaring an explicit replacement transaction.

## Validation

Publication rejects invalid transforms, non-finite bounds, unstable pivots,
undeclared collision, overlapping identity, unresolved materials, missing HLOD
or fallback policy, invalid road graph topology, broken portals, Data Layer
cycles, shipping references to test roots, or final world actors that depend on
staging packages.

# World, props, roads, interiors, and streaming

- Status: Active
- Last reviewed: 2026-08-03

## World model

The base game uses one connected Unreal world with World Partition for native
streaming. Narrative chapters are progression and content-activation states,
not alternate copies of the geography.

World Partition is a streaming boundary only. It must not move, rotate, scale,
recenter, or rebuild source geography.

## Canonical world source

The original decoded map is constructed directly from normalized geometry and
placement JSON. Native world plans preserve source positions, pivots,
transforms,
UVs, materials, textures, and package identities while applying the documented
source-to-Unreal basis exactly once.

Do not add actor mirrors, yaw adjustments, height offsets, UV mirrors, map
offsets, interior movements, or transport-format root compensation.

The base project does not use an Unreal Landscape replacement. Terrain, roads,
shorelines, seabed, buildings, props, and interiors come from original decoded
world geometry. Any later optional replacement belongs to a mod, not the
faithful base port.

## Structural guide

The optional structural guide visualizes normalized world geometry for editor
inspection. It preserves source positions and UVs without becoming coordinate
authority. It excludes review galleries and adds no placement, height, terrain,
collision, or guide-only geometry.

The guide is not runtime, gameplay, collision, navigation, or material
authority. Delete it without changing production regeneration.

## Coordinates

Source coordinates remain authoritative through normalized evidence. Unreal
read-back must verify finite transforms, bounds, pivots, package identity, and
the declared native basis conversion. Validation does not enforce a synthetic
map extent, sea-level translation, or common height datum.

## Asset decomposition

Shipping content is decomposed only where source package identity or native
runtime ownership requires it:

- terrain, road, sidewalk, curb, and structural meshes;
- buildings, doors, windows, signs, and architectural props;
- vegetation, rocks, street furniture, and decals;
- interiors and reusable streamed content;
- mission or chapter variants;
- breakable, animated, interactive, or stateful props.

Decomposition must not change world-space placement. One giant replacement mesh
and arbitrary fragmentation by filename are both rejected.

## Roads and traffic

Road and traffic behavior uses typed graph records decoded from source evidence.
Rendered road meshes are presentation, not topology authority. Runtime does not
infer lanes or legal movement from filenames or triangle adjacency.

## Interiors

Each interior has stable identity, entry and exit portals, visibility, audio,
lighting, navigation, save policy, and world-state bindings. Fused interior FBXs
retain source-space geometry. Level 7 Halloween output contains only source
triangles absent from the canonical base.

Nested interior evidence publishes its exact texture set so native
construction cannot degrade silently to missing materials.

## Data Layers and streaming

Data Layers activate semantic content such as chapters, missions, collectibles,
interiors, world state, and mods. Activation is transactional.

World Partition cell sizing, loading range, and HLOD configuration are platform
profiles. They may optimize streaming but cannot alter base geometry or source
placement.

## Validation

Publication rejects:

- missing or malformed source packages;
- non-finite transforms or bounds;
- unstable package, mesh, material, or texture identities;
- unresolved external textures;
- map, interior, height, UV, or actor-transform corrections not declared by
  source evidence;
- partial normalized evidence or native catalog publication; and
- a world-import path that depends on an Unreal Landscape replacement.

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

The original decoded map is imported through deterministic binary FBX 7.7.
World FBXs preserve source positions, pivots, transforms, UVs, materials,
textures, and package identities. One `SHAR_Export_Root` with `ReflectX` owns
the explicit source-to-FBX/Unreal basis conversion.

Import every world FBX with identity actor location, rotation, and scale.
Disable
`Force Front XAxis`. Do not add an actor mirror, yaw adjustment, height offset,
UV mirror, map offset, or interior movement.

The base project does not use an Unreal Landscape replacement. Terrain, roads,
shorelines, seabed, buildings, props, and interiors come from original decoded
world geometry. Any later optional replacement belongs to a mod, not the
faithful base port.

## Structural guide

The optional structural guide combines normal-import world FBX geometry for
editor inspection. It shares the same `ReflectX` root and preserves source
positions and UVs. It excludes review galleries and adds no placement, height,
terrain, collision, or guide-only geometry.

The guide is not runtime, gameplay, collision, navigation, or material
authority. Delete it without changing production regeneration.

## Coordinates

Source coordinates remain authoritative through export. Unreal read-back must
verify finite transforms, bounds, pivots, package identity, and the declared FBX
root basis. Validation does not enforce a synthetic map extent, sea-level
translation, or common height datum.

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

Nested interior FBXs publish their exact external texture set beside the FBX so
Unreal import cannot degrade silently to missing materials.

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
- partial FBX or catalog publication; and
- a world-import path that depends on an Unreal Landscape replacement.

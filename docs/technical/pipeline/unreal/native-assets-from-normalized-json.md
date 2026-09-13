# Native assets from normalized JSON

- Status: Active
- Last reviewed: 2026-09-12

## Purpose

This specification describes production construction of Unreal assets from
normalized typed JSON and versioned plans. FBX remains a deprecated diagnostic
and interoperability representation and is not production authority.

## Construction boundary

The pipeline verifies normalized identities, hashes, ordering, source units,
source basis, topology, rig relationships, animation samples, material evidence,
and collision records before native construction. Unreal toolsets receive only
typed plans derived from that verified evidence.

A native constructor creates the target object with stable identity, applies the
required target representation, reads the object back, and compares it with the
plan before publication. The caller owns explicit save, rollback, and
transaction ordering.

## Coordinates and units

Normalized geometry stays in its declared source basis and units. Each native
asset family owns one documented source-to-Unreal conversion and applies it once
to every related position, direction, transform, animation sample, collision
shape, and semantic attachment.

Outer actor transforms, importer front-axis switches, negative-scale roots, and
format-specific scene-unit compensation are not valid substitutes for that
conversion. Conversion tests must include asymmetric source landmarks so a
mirror or axis swap cannot pass through symmetric geometry unnoticed.

## Meshes and rigs

Vehicle model publication uses `shar.normalized-skeletal-model.v1`. The payload
is emitted before any deprecated FBX or Chaos target-basis conversion and pins
its coordinate contract as right-handed meters with `+X` right, `+Y` up, and
`+Z` forward. It carries complete mesh groups, material identities, skeleton
rest matrices, skin influences, source rig metadata, and skeletal animation
samples. Catalog records bind each payload by exact bytes and SHA-256.

Static and skeletal mesh constructors consume normalized vertices, indices,
normals, texture coordinates, colors, material ownership, skeleton hierarchy,
bind transforms, skin influences, and semantic component identities directly.
They do not rediscover wheels, doors, terrain, props, or character parts from a
transport format.

Skeletal construction must preserve authored hierarchy and deformation while
mapping the complete rig into the accepted Unreal basis. Animation construction
uses the same basis policy as the rest pose and never depends on an importer's
implicit axis conversion.

For vehicles, the Unreal-side decode boundary validates
`shar.normalized-skeletal-model.v1` before object creation. Its sole target
conversion is source `(X right, Y up, Z forward)` to Unreal
`(X forward, Y right, Z up)`: vectors map to `(Z, X, Y)`, positions additionally
convert meters to centimeters, and local rest matrices use the same proper basis
conjugation. Native Skeletal Mesh, Skeleton, and animation builders consume that
already-validated target recipe rather than repeating coordinate policy.

Vehicle skeletal construction first materializes a transient `USkeleton` and
`USkeletalMesh`, then commits LOD 0 source `MeshDescription` data containing the
validated target-basis geometry, material groups, bone hierarchy, and skin
weights. This source-data checkpoint is not itself proof of built render data or
persisted package parity; publication remains gated on those later read-backs.

## Physics

Physics Assets are built from normalized collision and physics plans after the
native Skeleton exists. Shapes, self-collision policy, body simulation mode,
joint semantics, and constraints remain source-backed and are verified by
read-back.

Temporary approximations may support bounded tests, but they stay explicit
blockers until their source semantics have a native representation. They cannot
silently become final vehicle or world physics policy.

## Materials and textures

Material and texture construction consumes normalized shader, texture,
parameter, color-space, blend, culling, and runtime-behavior evidence directly.
Missing source semantics remain blocked instead of being replaced with invented
PBR constants or generic fallback textures.

## Migration from FBX

Existing FBX paths remain available only while a native asset family lacks full
parity coverage. Migration proceeds asset family by asset family, with native
and deprecated outputs compared against the same normalized source evidence.

After parity, production plans stop generating or verifying the FBX prerequisite
for that family. Format-specific root rotations, reflections, unit workarounds,
and importer policies are then deleted from the active production path rather
than retained as compatibility behavior.

## Validation

Acceptance requires deterministic plans, source-evidence verification, native
construction tests, read-back equality, rollback coverage, clean regeneration,
and representative runtime tests. An FBX import success is no longer production
acceptance evidence.

# World assembly from normalized chunks

- Status: Accepted
- Decision date: 2026-07-12
- Scope: World reconstruction

## Context

World reconstruction must preserve observable structure without importing
proprietary editor projects or source-engine runtime organization. Normalized
world evidence therefore needs one deterministic native assembly boundary.

## Decision

Faithful world assembly consumes validated semantic FBX components, terrain,
placement records, geographic taxonomy, transforms, bounds, and assembly
relationships produced from normalized world packages. Those records reconstruct
the one native geographic world deterministically without copying proprietary
editor projects or preserving source-engine runtime structures.

The seven main levels form three recurring exterior families: Levels 1, 4, and 7
share Zone 1; Levels 2 and 5 share Zone 2; Levels 3 and 6 share Zone 3. Reviewed
horizontal affine movements connect Zones 2 and 3 to Zone 1. A final source-X
reflection applies to all three exterior families and every associated
coordinate record so the shared FBX export-root conversion does not reverse the
assembled world's left and right sides.

An exact `43.396` meter source-height translation applies after every exterior
and interior placement without exception. The complete movement boundary covers
render geometry, collision evidence, doors, object placement, character and
object spawns, mission placement, triggers, cameras, locators, and lights. In
source coordinates height is Y; generated Blender evidence projects that same
translation onto Blender Z. This translation is additive and does not normalize
the world's minimum elevation to zero. Measured Blender bounds increase by
`43.395996` on both Z limits after `f32` storage while still crossing the zero
plane; any later ground-to-zero operation is a distinct transformation.

The operator may use an ignored Blender scene to review placement, but that
scene is comparison evidence rather than production authority. The pipeline
records only solved source-dependent affine matrices and verifies regenerated
geometry against the reviewed scene. Bonus and auxiliary packages do not enter
the seven-level world stage.

Later manual FBX corrections use an ignored relative-path mirror and become
committed source-dependent Rust algorithms, never replacement FBX authority.
Selection prefers exact path, exact stem, and a unique prefix before accepting a
structural fingerprint whose weakest dimension is at or above 99 percent.
Exact identity matches still require the registered source fingerprint.
Ambiguous or stale matches block publication. Blender ordering and
serialization drift are not authored changes.

Interiors do not inherit the exterior-family reflection. Each of the 19 source
packages has one reviewed full-XYZ source-space movement, including height, and
is grouped into one of eight stable interior identities. The reviewed matrices
remain placement authority, but the source's artificial 8,192-meter Zone 2 and
16,384-meter Zone 3 family displacements are cancelled before the shared FBX
basis conversion because the connected native world already owns family
placement. Ordinary recurring copies are then transformed into the same reviewed
world space and fused into one canonical base FBX per identity. Source collision
remains excluded.

Elementary School (`i00`), Kwik-E-Mart (`i01`), Simpsons House (`i02`), and
Bart's Room (`i07`) additionally publish one Level 7 Halloween overlay. The
overlay contains only world-space triangles absent from the canonical base.
Triangle ownership uses spatial centroid, vertex, and surface buckets with a
bounded five-millimeter comparison derived from measured review-placement noise.
Exact triangles are duplicates. An alternate diagonal is a duplicate only when
all candidate vertices are already owned and its centroid plus all three edge
midpoints remain covered by owned coplanar triangles; an uncovered planar span
is retained as new geometry. Source names, materials, UVs, normals, vertex
indices,
and triangle ordering are not ownership authority; retained triangles preserve
their original presentation. This prevents a mixed Halloween mesh from repeating
ordinary walls, floors, furniture, or fixtures without collapsing genuinely
different geometry.
Buildings, houses, windows, doors, linked interiors, landmarks, roads, props,
and mission anchors retain stable identities and coordinates. Campaign levels
project state over the assembled geography rather than owning alternate copies
of the same physical location.

## Consequences

- Validated normalized world packages and their semantic FBX placement evidence
  are the sole production inputs to faithful native world assembly.
- The resulting world is independently authored and can be regenerated from one
  terrain and component assembly without proprietary editor projects or
  source-engine runtime structures.
- Three family-level exterior transforms replace artificial map spacing and
  apply one final global X reflection without flattening height or moving
  interiors.
- Interior identity, reviewed placement, tolerant duplicate collapse, additive
  Halloween ownership, exact global height, and collision exclusion remain
  independently testable from exterior world assembly.
- Ignored review scenes and derived editing FBXs may be deleted without changing
  production regeneration authority.
- Geographic identities support map-like mission and mod editing.
- Missing component, transform, coordinate, interior-link, or assembly evidence
  fails before an incomplete native world is accepted.

## Rejected alternatives

- Importing or copying a proprietary editor project.
- Preserving source-engine runtime structures as the native architecture.
- Completing faithful world assembly through undocumented manual placement.

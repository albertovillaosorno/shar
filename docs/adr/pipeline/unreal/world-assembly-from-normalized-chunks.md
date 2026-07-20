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
horizontal affine movements connect Zones 2 and 3 to Zone 1 while preserving
source height. A final source-X reflection applies to all three exterior
families and every associated coordinate record so the shared FBX export-root
conversion does not reverse the assembled world's left and right sides. The
reflection covers render geometry, collision evidence, doors, object placement,
character and object spawns, mission placement, triggers, cameras, locators, and
lights.

The operator may use an ignored Blender scene to review placement, but that
scene is comparison evidence rather than production authority. The pipeline
records only the solved source-dependent transform and must verify it against
unchanged source geometry. Bonus and auxiliary packages do not enter the
seven-level world stage.

Interiors remain independent from exterior family movement and do not receive
the global reflection. Repeated level copies are identified by stable interior
identity and compared as complete package-local variants. Only exact normalized
local package duplicates collapse to one representative; every non-identical
source-level variant remains separate for operator review. Source collision is
excluded, and each variant keeps the existing own-center horizontal reflection.
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
- Interior identity, exact-duplicate collapse, source-level variant
  preservation, own-center mirroring, and collision exclusion remain
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

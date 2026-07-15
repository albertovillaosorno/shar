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
editor projects, preserving source-engine runtime structures, or relying on
manual actor placement.

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
- Geographic identities support map-like mission and mod editing.
- Missing component, transform, coordinate, interior-link, or assembly evidence
  fails before an incomplete native world is accepted.

## Rejected alternatives

- Importing or copying a proprietary editor project.
- Preserving source-engine runtime structures as the native architecture.
- Completing faithful world assembly through undocumented manual placement.

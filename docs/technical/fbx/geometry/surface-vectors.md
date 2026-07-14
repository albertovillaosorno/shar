# Surface vectors and texture coordinates

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [First-principles FBX output contract](../../../adr/fbx/export/fbx-output-contract-boundary.md)

## Purpose

This specification explains how repository-owned conversion handles texture
coordinates, normals, tangents, and binormals.

## Repository model

Surface channels are canonical arrays associated with validated mesh vertices or
polygon corners. One coordinate-conversion boundary transforms orientation and
handedness consistently across positions and surface vectors.

## Invariants

- Required channels match the owning geometry cardinality.
- Texture-coordinate channel identity and order are stable.
- Surface vectors are finite and use one declared coordinate convention.

## Failure behavior

- Cardinality mismatches, non-finite values, unsupported channel ownership, and
  contradictory coordinate evidence reject the mesh capability.

## Verification

- Channel-cardinality tests cover vertex and polygon-corner ownership.
- Coordinate tests verify consistent orientation across all vector families.
- Read-back tests compare channel names and ordering.

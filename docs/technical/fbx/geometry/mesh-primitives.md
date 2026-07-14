# Mesh primitive model

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [First-principles FBX output contract](../../../adr/fbx/export/fbx-output-contract-boundary.md)

## Purpose

This specification explains how repository-owned conversion represents topology
and authored mesh partitions.

## Repository model

The canonical mesh contains deterministic partitions, vertex attributes, index
buffers, primitive ranges, material slots, and declared bounds. Input adapters
validate source counts before scene construction.

## Invariants

- Indices reference existing vertices.
- Primitive ranges do not overlap or exceed index storage.
- Authored mesh partitions remain distinct unless a package rule explicitly
  combines them.
- Material-slot ordering is deterministic.

## Failure behavior

- Out-of-range indices, invalid primitive widths, contradictory counts, and
  missing required attributes reject the mesh.
- Degenerate geometry is reported rather than silently repaired.

## Verification

- Topology tests cover boundaries, empty ranges, and invalid indices.
- Determinism tests reorder equivalent input evidence.
- Writer read-back tests compare partition and material-slot counts.

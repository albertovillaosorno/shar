# Skin attachment

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Hexagonal scene export](../../../adr/pipeline/fbx/hexagonal-scene-export.md)

## Purpose

This specification explains how canonical meshes bind vertex influences to
canonical joints.

## Repository model

A skin model contains canonical joint references, deterministic influence
ordering, per-vertex weights, and bind transforms. Application validation
completes before the binary writer creates clusters or deformers.

## Invariants

- Every influence references a known canonical joint.
- Each supported vertex satisfies the package weight contract.
- Equivalent input produces stable joint and cluster order.
- Numerical normalization stays within an explicit tolerance.

## Failure behavior

- Unknown joints, invalid weights, missing required influences, non-finite
  transforms, and inconsistent bind evidence reject the skin capability.
- Unweighted vertices are never assigned to a guessed joint.

## Verification

- Domain tests cover normalization, ordering, and invalid references.
- Writer tests read back clusters and influence counts.
- Character tests verify skeleton, mesh, and skin identity agreement.

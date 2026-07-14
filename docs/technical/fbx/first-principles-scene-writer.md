# First-principles scene writer

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [First-principles FBX output contract](../../adr/fbx/export/fbx-output-contract-boundary.md)

## Purpose

This specification explains the repository-owned model package conversion flow
without documenting an external file format.

## Repository model

Input adapters translate decoded package evidence into a canonical scene domain.
Application services validate identity, hierarchy, references, timing, geometry,
materials, textures, skeletons, skinning, animation, cameras, and capability
completeness. A repository-owned binary writer serializes only validated domain
values. An independent reader and native-engine ingestion tests verify the
result.

## Invariants

Object ordering, identifiers, connections, properties, embedded resources, and
reports are stable for equivalent input. Blender and Maya do not participate in
generation, conversion, repair, validation, or acceptance.

## Failure behavior

Unknown references, cyclic hierarchy, invalid topology, contradictory timing,
invalid skin weights, unsupported capabilities, serializer overflow, or
read-back mismatch fail the package.

## Verification

Domain tests prove scene invariants, writer tests prove deterministic
serialization, independent read-back proves structure, and clean native
ingestion proves interoperability.

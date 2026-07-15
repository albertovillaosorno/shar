# Material assignment

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

<!-- markdownlint-disable-next-line MD013 -->
- [First-principles FBX output contract](../../../adr/fbx/export/fbx-output-contract-boundary.md)

## Purpose

This specification explains how canonical mesh partitions bind to
repository-owned material identities.

## Repository model

Material adapters normalize supported evidence into canonical identities and
properties. Scene construction binds each mesh partition or polygon range to a
validated material slot, and serialization preserves canonical slot order.

## Invariants

- Every material reference resolves to one canonical material.
- Equivalent input produces the same slot order.
- A partition cannot silently inherit an unrelated neighboring material.

## Failure behavior

- Unknown, duplicated, or ambiguous material references reject the affected
  package.
- Unsupported properties remain explicit capability limits.

## Verification

- Assignment tests cover multiple partitions and repeated materials.
- Determinism tests reorder material evidence.
- Writer read-back verifies material objects and connections.

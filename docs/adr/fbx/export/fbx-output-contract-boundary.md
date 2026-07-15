# First-principles FBX output contract

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Canonical model interchange output

## Context

The project needs a deterministic interchange artifact for Unreal ingestion
without depending on manual repair or an external content-authoring application.

## Decision

Canonical model output is binary FBX 7.7 generated directly by the
repository-owned writer from the canonical scene domain. Blender and Maya are
not used for generation, conversion, repair, validation, or acceptance.

Output is deterministic, self-contained for supported package profiles, and
explicit about unsupported capabilities. ASCII FBX and authoring-file formats
are not canonical outputs.

Engine-independent semantic preparation belongs to the canonical scene and FBX
phase before serialization. Character UV and texture modernization, semantic eye
and outfit regions, non-destructive rig display metadata, detachable animation
props, vehicle moving parts, world components, pivots, transforms, and
geographic
placements must therefore be present in FBX evidence rather than discovered for
the first time during native Unreal import.

Legacy helpers that invoke external content-authoring applications are outside
the supported workflow and must be retired rather than used as evidence.

## Consequences

- The repository owns serialization correctness.
- Validation uses repository-owned checks and clean Unreal ingestion evidence.
- Manual scene repair cannot hide writer defects.
- External application availability is not a prerequisite.

## Rejected alternatives

- Exporting through Blender or Maya.
- Display in one review application as conformance proof.
- Multiple canonical model formats.

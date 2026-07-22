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
ownership, complete outfit and prop-bearing variants, non-destructive
rig-display metadata, vehicle moving parts, world components, pivots,
transforms, and geographic placements must therefore be present in FBX
evidence. Native Unreal import cannot discover those separations for the first
time. Character animation
behavior remains unchanged in this phase.

Legacy helpers that invoke external content-authoring applications are outside
the supported workflow and must be retired rather than used as evidence.

### Editor-only structural-guide profile

The structural guide is an opt-in FBX 7.7 profile of the same repository-owned
binary writer. It emits one identity-transform mesh, one material, one external
texture reference, and four named per-polygon-vertex UV layers. It must not
change the ordinary character, vehicle, prop, or separated-world byte path.

This profile optimizes Unreal editor inspection rather than shipping fidelity.
Geometry, reviewed placement, explicit normals, winding, source UV tiling,
atlas addressing, and artifact hashes remain strict. Alpha is flattened to
opaque RGB and dynamic shader behavior is omitted. Source vertex colors are
baked exactly when uniform for a material/wrap identity; otherwise one
deterministic source-texture-wide average is used and counted in the manifest.
The approximation is acceptable only because the guide is explicitly excluded
from runtime, collision, gameplay, and shipping-render authority.

## Consequences

- The repository owns serialization correctness.
- Validation uses repository-owned checks and clean Unreal ingestion evidence.
- Manual scene repair cannot hide writer defects.
- External application availability is not a prerequisite.

## Rejected alternatives

- Exporting through Blender or Maya.
- Display in one review application as conformance proof.
- Multiple canonical model formats.

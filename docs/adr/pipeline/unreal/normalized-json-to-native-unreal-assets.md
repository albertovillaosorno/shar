# Normalized JSON to native Unreal assets

- Status: Accepted
- Decision date: 2026-09-12
- Scope: Production Unreal asset construction

## Context

Normalized extraction already preserves source geometry, topology, transforms,
rigs, animation, materials, collision, and provenance as typed JSON evidence.
Using FBX as an additional production boundary forces that evidence through a
second coordinate, unit, material, skeleton, and importer interpretation before
Unreal can construct its native assets.

Vehicle work demonstrated the cost of that extra boundary. Correct results
required coordinating FBX export-root transforms, scene units, importer axis
policy, skeletal root-local coordinates, and Chaos physics semantics even though
the normalized source evidence already represented the required facts.

## Decision

Normalized extracted JSON and versioned normalized plans are the canonical
production handoff into Unreal. Project-owned native editor automation
constructs
Static Meshes, Skeletal Meshes, Skeletons, animation assets, Physics Assets,
materials, textures, data assets, and world content directly from that evidence.

Source-to-Unreal basis, unit, and representation conversion occurs exactly once
in typed native construction policy. Source JSON remains source-space evidence;
target conversion is explicit, deterministic, independently tested, and verified
through native read-back.

FBX is deprecated for production Unreal ingestion. The repository-owned FBX
writer may remain temporarily for interoperability, diagnostics, comparison, and
migration evidence, but no new production Unreal plan may require FBX or treat
FBX transforms, material slots, scene units, or importer behavior as authority.

Existing FBX consumers are retired incrementally. Each asset family moves only
after its native constructor proves equivalent source coverage and stable native
identity, after which the corresponding FBX-specific rotations, reflections,
scene-unit workarounds and importer compensation are removed rather than kept
as parallel policy.

## Consequences

- Normalized JSON becomes the durable source-to-engine boundary.
- Unreal assets are reproducible without an intermediate mesh interchange file.
- Coordinate and unit conversion has one typed owner per native asset family.
- FBX bugs cannot silently redefine source geometry, rigs, materials, or
  physics.
- Existing FBX tooling remains useful during migration without retaining
  production authority.
- Native read-back and deterministic plans remain required before publication.

## Rejected alternatives

- Keeping FBX as the canonical production Unreal ingestion boundary.
- Maintaining native JSON construction and FBX import as equal production paths.
- Parsing raw proprietary package bytes inside the shipping Unreal runtime.
- Replacing deterministic construction with manual editor repair.
- Removing FBX diagnostics before native parity is demonstrated.

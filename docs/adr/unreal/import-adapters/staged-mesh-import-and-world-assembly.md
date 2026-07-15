# Staged mesh import and world assembly

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Phase 6 native Unreal import from normalized FBX, texture, WAV, JSON,
  and MOV evidence

## Context

Canonical FBX generation and native Unreal import are separate phases. FBX is a
general-purpose deterministic interchange artifact and now owns all
engine-independent semantic preparation. Character UV and texture modernization,
eye and outfit regions, detachable props, vehicle moving parts, world component
boundaries, pivots, transforms, and geographic placement must therefore be
complete before native import begins.

Native Unreal import still needs a transaction that validates evidence, creates
destination assets, applies engine-specific material and streaming policy, and
reads the result back without letting mutable editor state become authority.

## Decision

Phase 6 uses a staged native-import transaction. The importer first validates the
canonical FBX, texture, semantic-region, component, placement, and provenance
manifests in a quarantined staging area. It then maps those approved identities
to native meshes, textures, materials, physics assets, animation assets,
World Partition actors, data layers, and primary assets. Final UAssets are not
published until planned identities and read-back evidence agree.

The importer consumes canonical destination UVs and modern texture evidence; it
does not create their first semantic organization. It may transcode textures,
create mip chains, select platform compression, construct material instances,
and bind approved maps. Base color is required for textured materials. Normal,
specular, roughness, metallic, glossiness, emissive, and ambient-occlusion maps
remain optional unless the approved FBX preparation recipe marks them required.
A detected map is bound only after semantic and color-space validation.

Character import preserves the FBX topology, skeleton hierarchy, bind state,
skin weights, animation timing, semantic regions, integrated outfit identity,
eye-animation mechanism, and detachable-prop attachments. It may generate
engine-specific physics, sockets, retargeting metadata, LODs, or derived runtime
assets only through declared deterministic recipes that do not redefine the
canonical character identity.

Vehicle import consumes already separated body, wheels, trunk, and other
supported moving components with their pivots and transforms. World import
consumes already separated terrain, structures, windows, doors, linked interiors,
landmarks, props, bounds, and geographic placements. It creates the one native
World Partition geography and its level-state data layers without rediscovering
component boundaries from raw geometry.

Geometry refinement may add vertices only through a later declared deterministic
native recipe that preserves silhouette, topology boundaries, skinning,
collision, semantic component identity, material assignment, and animation
compatibility. No unconditional subdivision or arbitrary vertex inflation is
allowed. The initial character path performs no polygon increase.

LOD and HLOD generation is destination-specific and remains owned by Phase 6.
Required distant geometry must transition to approved lower-detail
representations rather than disappear through arbitrary authored visibility
toggles. Native frustum, occlusion, streaming, and platform culling remain valid
runtime optimizations.

This decision applies only to Phase 6 native import. It does not redefine binary
FBX generation, semantic component preparation, normalized audio or media
evidence, package taxonomy, or runtime gameplay behavior.

## Consequences

- Canonical FBX remains engine-independent, semantically complete, and
  deterministic.
- Native import cannot hide missing semantic regions, component boundaries,
  pivots, coordinates, or attachments behind editor repair.
- Final UAssets are not published until imported state matches the approved FBX
  and native asset plans.
- Missing optional shading maps use explicit neutral inputs rather than guessed
  dependencies.
- Generated native maps, physics assets, LODs, HLODs, and platform texture
  variants remain reproducible and provenance linked.
- The one geographic world can be regenerated from canonical components and
  placement evidence without manual actor placement.

## Rejected alternatives

- Treating the first successful FBX transport as a final production UAsset.
- Performing the first UV, texture, eye, outfit, vehicle, or world separation
  during UAsset import.
- Guessing shading maps or semantic components from file names alone.
- Generating derived maps or extra vertices without a versioned recipe.
- Importing the complete world as one indivisible static mesh.
- Using manual editor repair or placement as native asset authority.

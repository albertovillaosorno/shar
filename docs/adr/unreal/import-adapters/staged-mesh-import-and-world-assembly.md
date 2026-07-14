# Staged mesh import and world assembly

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Phase 6 native Unreal import from normalized FBX, texture, WAV, JSON,
  and MOV evidence

## Context

Canonical FBX generation and native Unreal import are separate phases. FBX must
remain a general-purpose deterministic interchange artifact, while the Unreal
import phase may create destination UVs, rebake textures, construct native
materials, refine geometry, split world geometry, and generate LOD or HLOD
assets.

Publishing an Unreal mesh immediately after transport would preserve source UV
and material limitations as native production state. Conversely, changing the
canonical FBX writer to depend on Unreal-specific UV, material, or world
partition behavior would collapse the phase boundary and make the interchange
artifact engine-specific.

## Decision

Phase 6 uses a staged native-import transaction. The importer first validates
canonical FBX geometry and real texture evidence in a quarantined staging area.
It then applies a versioned destination-UV policy, rebakes or regenerates the
required native texture set, creates material instances, generates declared LODs,
and verifies the resulting mesh before publishing the final UAsset identity.

Base-color texture evidence is used when a material is textured. Normal,
specular, roughness, metallic, emissive, and ambient-occlusion maps are optional
unless the approved recipe marks them required. A detected map is bound only
after semantic and color-space validation. A missing optional map uses an
explicit neutral material value or no texture input. Deterministic derived-map
generation is permitted only when its recipe and verification are recorded.

Geometry refinement may add vertices only through a declared deterministic
recipe that preserves silhouette, topology boundaries, skinning, collision,
material assignment, and animation compatibility. No unconditional subdivision
or arbitrary vertex inflation is allowed. The initial implementation remains
pending until a validated refinement recipe exists.

World import begins from a natural assembled FBX representation used as source
and placement evidence. Phase 6 then separates houses and other world components
into stable native mesh identities, reconstructs their transforms in one native
map, and generates LOD and HLOD representations. Required distant geometry must
transition to lower-detail representations rather than disappear through
arbitrary authored visibility toggles. Native frustum, occlusion, streaming, and
platform culling remain valid runtime optimizations.

This decision applies only to Phase 6 native import. It does not redefine binary
FBX generation, normalized audio or media evidence, package taxonomy, or runtime
gameplay behavior.

## Consequences

- Canonical FBX remains engine-independent and deterministic.
- Final mesh UAssets are not published until destination UVs, native textures,
  materials, LOD policy, and read-back validation agree with the import plan.
- Missing optional normal or specular maps do not block import and do not create
  guessed texture dependencies.
- Generated maps and geometry refinement remain reproducible and provenance
  linked.
- Whole-world source evidence may be decomposed into independently streamable
  native components without losing one-map assembly identity.
- LOD and HLOD policy owns distance simplification; required geometry cannot be
  replaced by unexplained disappearance.

## Rejected alternatives

- Treating the first successful FBX transport as the final production UAsset.
- Mutating canonical FBX generation to depend on Unreal-only UV or map assembly.
- Guessing normal, specular, or other material maps from file names alone.
- Generating derived maps or extra vertices without a versioned recipe.
- Shipping the complete world as one indivisible static mesh.
- Hiding required world components at distance instead of supplying an approved
  lower-detail representation.

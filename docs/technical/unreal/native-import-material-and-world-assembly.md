# Native import, material rebuild, and world assembly

- Status: Planned
- Last reviewed: 2026-07-14
- Delivery phase: Phase 6 only

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Converted asset ingestion boundary](../../adr/unreal/import-adapters/converted-asset-ingestion-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Import review boundary](../../adr/unreal/import-adapters/import-review-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset translation without copy-paste](../../adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md)

## Purpose

This specification defines the planned conversion of normalized FBX, texture,
WAV, JSON, and MOV evidence into native Unreal assets. It applies only after the
canonical conversion phases have produced validated engine-independent evidence.
It does not change the binary FBX writer, package taxonomy, extraction behavior,
or runtime gameplay contracts. Cooked runtime requests, package mounting, and
source-decoder exclusion follow the
<!-- markdownlint-disable-next-line MD013 -->
[native asset load request and streaming runtime](native-asset-load-request-and-streaming-runtime.md).

## Import transaction

Each import uses one immutable plan and these phases:

1. validate source identities, revisions, hashes, dependencies, and target
   names;
1. import into a quarantined native staging area;
1. read back mesh, media, audio, and data-asset structure;
1. rebuild destination UV, texture, material, geometry, collision, and LOD
   state;
1. assemble world components when the plan is a world package;
1. verify every planned native property and dependency;
1. publish the final stable package identities atomically; and
1. remove or tombstone staging state.

A successful editor call is transport evidence only. Publication occurs after
native read-back matches the complete plan.

## FBX source contract

An accepted FBX source provides:

- stable scene, node, mesh, material-slot, texture, skeleton, animation, camera,
  and transform identities required by its package profile;
- real canonical texture evidence for every material slot that is not explicitly
  declared untextured;
- deterministic source UV channels and surface-vector metadata;
- deterministic normal, tangent, smoothing, skin, bind-pose, collision, and
  animation evidence where required; and
- one import-recipe revision and one final native target identity.

Embedded or relative texture references are source evidence. The final UAsset
material dependency set is rebuilt by the native import plan and is not inferred
from arbitrary editor discovery.

## Staging packages

Staging packages are not shipping content. Their names are derived from the
transaction identity and cannot be referenced by runtime assets, maps, saves, or
mods. A staging mesh may temporarily preserve source UV and source texture
bindings so the importer can verify and rebake them.

A staged asset is promoted only when:

- every required source member resolved;
- the intended mesh type, skeleton, material slots, and transforms match;
- destination UV and texture rebuild completed when required;
- collision and LOD data match the approved recipe;
- no staging dependency remains in the final package graph; and
- the final package passes structural and visual-review evidence.

Failed or interrupted staging work is quarantined and never appears under the
final package identity.

## Destination UV policy

Source UVs remain traceability evidence. They are not assumed to be the final
native layout. Each mesh recipe declares one of these policies:

<!-- markdownlint-disable MD013 -->

| Policy | Contract |
| :--- | :--- |
| `preserve_validated` | Use a validated source UV channel without modification. |
| `generate_destination` | Create a deterministic production UV channel and rebake textures. |
| `preserve_and_generate_lightmap` | Preserve the material UV and generate a separate lightmap channel. |
| `quarantine` | Reject the mesh because no valid deterministic destination exists. |

<!-- markdownlint-enable MD013 -->

A generated destination UV records chart boundaries, seams, orientation,
padding, resolution class, overlap tolerance, output channel, and generator
revision. Nondeterministic packing, zero-area charts, invalid bounds, unexpected
overlap, or missing texture-bake coverage fails the import.

The final material samples exactly the declared destination channel. It cannot
silently mix old and new UV layouts.

## Texture and material rebuild

Native textures are imported or generated as versioned Unreal texture assets.
Material instances use a repository-owned master-material family and explicit
parameters.

The material plan supports:

- base color;
- normal;
- specular;
- roughness;
- metallic;
- ambient occlusion;
- emissive;
- opacity or mask; and
- declared detail or lookup textures.

Base color is required for a textured material. Other maps are optional unless
the recipe marks them required.

### Optional map detection

A candidate normal, specular, or other map is accepted only when:

- its identity is linked by normalized metadata or a validated deterministic
  detection rule;
- dimensions, channels, bit depth, and color space match the semantic role;
- the map covers the destination UV layout after rebaking when required; and
- native read-back confirms the expected compression and sampler settings.

A name resemblance alone is not sufficient detection evidence.

### Missing optional maps

When an optional map is absent:

- the material may leave that texture input unbound;
- the material may use an explicit neutral scalar or vector constant; or
- a derived map may be generated by an approved deterministic recipe.

A neutral tangent-space normal is represented by the material's neutral normal
value rather than a guessed source texture. Missing specular uses the declared
material constant. No placeholder texture may masquerade as detected evidence.

### Derived maps

Normal, roughness, specular, metallic, or ambient-occlusion derivation remains
optional. A derived map records:

- source texture identities;
- generator and recipe revision;
- parameter values and random-seed policy;
- output dimensions and color space;
- validation metrics; and
- final native texture identity.

Derived output that fails semantic or visual-review thresholds is discarded and
the neutral fallback remains active.

## Geometry refinement

The initial Phase 6 importer does not promise automatic vertex enrichment.
Geometry refinement remains pending until a validated recipe is selected.

A future recipe may add vertices through controlled subdivision, remeshing, or
surface reconstruction only when it preserves:

- silhouette within declared tolerance;
- hard edges and material boundaries;
- UV seams and rebake coverage;
- skeleton influence limits and deformation quality;
- collision intent and socket transforms;
- animation compatibility; and
- deterministic vertex and triangle ordering.

Vertex count is never a quality metric by itself. An unchanged mesh remains
valid when additional geometry would not improve the approved presentation goal.

## Native audio, data, and cinematic import

WAV import creates native sound assets and preserves normalized duration, sample
rate, channel, loop, routing, subtitle, concurrency, and event identities. Cook
settings are selected later by the target policy and cannot alter logical
timing.

JSON import creates typed data assets, tables, registries, StateTree bindings,
or
purpose-built native records. Free-form JSON objects cannot become runtime
reflection bags without a registered schema.

MOV or normalized cinematic evidence creates validated media-source,
media-player,
texture, audio, subtitle, and synchronization assets according to the cinematic
packaging policy. Import never assumes one container or codec works on every
claimed target.

## World source and decomposition

World conversion begins with one natural assembled FBX scene for each declared
world package. That source preserves authored transforms, hierarchy, material
membership, and placement relationships. It is evidence, not the final native
streaming unit.

The decomposition plan assigns every shipping world element to one stable
component identity, including:

- houses and other buildings;
- road and sidewalk sections;
- terrain and retaining geometry;
- bridges, tunnels, fences, and walls;
- props and repeated modular pieces;
- collision-only structures;
- decals and presentation-only surfaces; and
- authored grouping or attachment roots.

No component may be lost merely because it was nested inside a larger FBX node.
Duplicate geometry is preserved only when the plan proves distinct placement or
presentation intent.

## One-map assembly

Native world components are assembled into one canonical map composition per
world variant. Placement records use stable component identity, transform,
layer, collision, material, lighting, navigation, and streaming metadata.

Assembly is deterministic:

- component order is canonical;
- transforms use normalized units and axes;
- repeated pieces use instances when the recipe permits;
- map actors reference final packages only;
- Runtime Data Layers describe level or mission variants; and
- World Partition cells are generated from the approved assembly rather than
  becoming source identity.

## LOD and visibility policy

Each world component declares an LOD policy. Related components may also declare
an HLOD grouping policy.

Required world geometry must retain an appropriate representation across its
approved visibility range. Distance simplification uses LOD or HLOD transitions
instead of arbitrary authored disappearance of complete houses or world parts.

This rule does not disable native frustum culling, occlusion culling, streaming,
platform budgets, or explicit mission-driven visibility. Those mechanisms remain
valid when their policy and fallback representation are declared.

LOD generation records reduction targets, preserved boundaries, material merge
policy, collision policy, screen-size thresholds, HLOD grouping, and generator
revision. A lower LOD that changes traversable collision or removes required
silhouette landmarks fails validation.

## Determinism and provenance

The final provenance row records:

- every normalized source identity and revision;
- the import-plan and tool revisions;
- staging and final package identities;
- UV, texture-bake, material, refinement, collision, LOD, HLOD, and assembly
  recipe identities;
- read-back hashes and normalized metadata; and
- acceptance or quarantine result.

Equivalent approved input produces the same logical native packages,
dependencies, component identities, map placements, and validation report.
Binary package bytes may contain engine-managed nondeterminism only when the
normalization policy explicitly excludes it from logical comparison.

## Failure behavior

The transaction fails before publication when:

- required real texture evidence is missing;
- optional-map detection is ambiguous;
- destination UV or texture rebake fails;
- final materials reference staging or source-only assets;
- geometry refinement violates topology, skin, collision, or determinism;
- world decomposition omits, duplicates, or misplaces a component;
- LOD or HLOD removes required geometry or changes collision behavior;
- MOV, WAV, or JSON output does not match its typed target; or
- native read-back differs from the approved plan.

No manual editor repair converts a failed import into success.

## Verification

Automated verification proves:

- every planned source member and final package identity resolves;
- real base-color textures bind to every textured material slot;
- optional maps are either validated, explicitly neutral, or provenance-linked
  derived output;
- destination UVs and rebaked textures match the recipe;
- skeletal deformation, static-mesh topology, collision, and LODs match their
  policies;
- world components reconstruct the source assembly within transform tolerance;
- required geometry remains represented through declared LOD and HLOD ranges;
- WAV, JSON, and MOV imports produce expected native asset classes; and
- deleting generated native assets and replaying the plan reproduces the same
  logical project state.

# Character semantic preparation

- Status: Required preparation contract; catalog rollout in progress
- Last reviewed: 2026-07-15

<!-- markdownlint-disable MD013 -->

## Governing decisions

- [Character semantic texture, rig, outfit, and prop contract](../../adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md)
- [First-principles FBX output contract](../../adr/fbx/export/fbx-output-contract-boundary.md)
- [Eleven-phase remake delivery roadmap](../../adr/pipeline/eleven-phase-remake-delivery-roadmap.md)

## Purpose

This specification defines deterministic semantic preparation for character FBX
artifacts before native Unreal import. It covers mesh partitions, UVs, textures,
eye ownership, complete-model outfit and prop variants, visual skeleton review,
manifest evidence, and representative validation.

This phase does not change character animation behavior. It does not implement
eye tracking, gaze control, blink changes, animation retargeting, skeleton
behavior changes, runtime clothing, runtime props, or a modular attachment system.

## Ownership boundary

The FBX preparation pipeline is the first owner of:

- semantic mesh and material-region identity;
- modernized deterministic UV placement;
- generated atlas and texture evidence;
- eye-layer ownership;
- complete outfit and prop-bearing variant identity;
- visual skeleton-display metadata where safely representable; and
- preparation manifests and conformance evidence.

Unreal import consumes and validates this prepared interchange contract. UAsset
import does not become the first place where eyes, clothing, props, or semantic
texture regions are separated.

## Current implemented behavior

The existing character writer preserves and serializes:

- source polygon and vertex counts;
- authored mesh partitions;
- materials and embedded texture evidence;
- skeleton hierarchy and transforms;
- skin clusters and weights;
- animation curves and source-rate timing; and
- one self-contained character presentation per FBX artifact.

These capabilities are transport behavior. A successful FBX import alone does
not prove semantic preparation conformance.

The writer does not currently implement modular garments, live prop attachment,
eye-tracking behavior, retargeting changes, or deformation changes.

## Required preparation architecture

A prepared character artifact contains:

| Field | Contract |
| :--- | :--- |
| `CharacterPresentationId` | Stable identity for one complete visual presentation. |
| `BaseCharacterId` | Canonical character identity shared by related variants. |
| `VariantKind` | Base, outfit, costume, or prop-bearing complete-model variant. |
| `MeshManifest` | Deterministic mesh, material, topology, and semantic ownership evidence. |
| `TextureManifest` | Atlas, region, source evidence, hashes, encoding, and padding rules. |
| `EyeProfile` | Sclera, pupil, upper-eyelid, and lower-eyelid ownership for both eyes. |
| `RigDisplayProfile` | Optional visual-only bone readability and support-bone visibility metadata. |
| `BehaviorPreservation` | Hierarchy, bind, skinning, animation, and deformation invariants. |
| `DeferredCapabilities` | Explicit future extension slots not implemented by this phase. |

Equivalent input evidence produces equivalent FBX structure, manifests, texture
bytes, semantic identities, and hashes.

## Representative validation set

Semantic character preparation must pass representative non-Homer conformance on:

- Krusty the Clown;
- Lisa Simpson;
- Principal Skinner; and
- Chief Wiggum, the canonical police character.

The validator resolves canonical base-model packages rather than display-name
searches or arbitrary costume variants.

This set tests varied head geometry, hair, clothing, eye presentation, body shape,
and rig-display complexity. Passing Homer alone is insufficient.

## Geometry boundary

Preparation preserves source polygon and vertex counts. It may:

- preserve or expose authored mesh partitions;
- assign stable semantic ownership to existing surfaces;
- reorganize materials;
- change UV coordinates;
- change texture resolution and atlas placement; and
- generate deterministic manifests and hashes.

It may not subdivide, remesh, decimate, inflate geometry, reconstruct hidden
bodies, or transfer weights merely to support a more complex outfit system.

## Semantic texture regions

At minimum, every prepared character distinguishes:

- skin;
- hair;
- shoes;
- legs; and
- torso.

Evidence may add mouth, teeth, headwear, accessories, garment details, or other
regions. A region identity describes semantic ownership, not an incidental source
texel island.

Patterns and details move into the atlas region owned by the corresponding body
or clothing surface. Generated base color preserves validated source pigment and
does not bake one world-lighting state into the texture.

Unused texels use deterministic neutral fill and sufficient edge padding or
dilation. Alpha exists only where the material contract requires it.

Additional authored maps are preserved only from evidence or a versioned
deterministic derivation recipe. The pipeline does not invent normal, roughness,
metallic, emissive, or ambient-occlusion content from comparison artifacts.

## Eye semantic split

Each eye exposes separate ownership for:

- sclera, the white of the eye;
- pupil, the black dot;
- upper eyelid; and
- lower eyelid.

The split is intentionally simple. It exists for easy future customization,
clean mod replacement, and stable semantic addressing. It must not introduce
unnecessary runtime mathematics, additional animation systems, or behavior
changes.

Semantic layers may be represented through material regions, texture regions,
mesh partitions, or a validated combination. They do not need to become four
separate mesh objects per eye.

Equivalent eye evidence may share a content-hashed eye profile. A character may
provide a local override for any layer without changing canonical eye behavior.

This phase does not add gaze, tracking, blinking, controller, bone, transform, or
texture-animation behavior.

## Skeleton visual review

Skeleton cleanup is optional and visual only. It may improve:

- bone display readability;
- visual continuity in review tools;
- support-bone labeling; and
- optional hiding of support bones when FBX metadata represents it consistently.

It must not change:

- hierarchy;
- bind pose;
- inverse bind;
- skin weights;
- animation transforms; or
- deformation behavior.

A visual change is accepted only when supported consumers interpret the same
canonical skeleton and sampled deformation remains identical. When visual-only
metadata cannot express the improvement safely, cleanup is deferred.

Importer-specific sidecars are review aids only and cannot become canonical
preparation output.

## Outfit variants

The current rule is one outfit equals one complete model.

A costume or outfit variant contains its full visible body, clothing, mesh,
materials, textures, skeleton, skinning, and preserved animation evidence. It is
not assembled at runtime from a hidden base body plus external garments.

This strategy is required now because modular clothing would add unnecessary
complexity, clipping risk, rig-transfer risk, hidden-body reconstruction,
weight-transfer risk, animation compatibility work, and lower immediate value
than completing stable character preparation.

Complete-model variants are the safest current representation and match the
available evidence.

## Prop-bearing variants

A special presentation prop integrated with a character follows the same rule.
Homer with a donut is a complete character-model variant, not base Homer plus a
live runtime donut attachment.

The pipeline does not infer sockets, activation clips, detachable prop state,
attachment timing, or hidden geometry from a prop-bearing legacy presentation.

A genuinely standalone prop package may remain a standalone asset, but its
existence does not convert integrated character presentations into a runtime
attachment system.

## Deferred future extensibility

The architecture reserves stable extension points for future authored:

- external clothing layers;
- modern skin or appearance systems;
- modular garments;
- cloth simulation;
- equipment and attachment layers;
- runtime sockets and prop activation;
- character-body reconstruction designed for modular outfits; and
- replacement eye presentation systems.

These capabilities are not implemented now. A future decision must define
clipping, body coverage, rig compatibility, weight transfer, animation behavior,
save identity, mod precedence, and runtime ownership before activation.

## Unreal import boundary

Native Unreal import consumes one prepared presentation identity and its complete
FBX, texture, eye-profile, semantic-region, skeleton-preservation, and variant
manifests.

The importer may create skeletal mesh, material, texture, and metadata assets from
that evidence. It may not:

- split an unprepared source eye for the first time;
- retarget or rewrite animation as part of semantic preparation;
- change hierarchy, bind state, skin weights, or deformation;
- detach integrated clothing or props;
- create a runtime garment or attachment system; or
- claim deferred extensibility is implemented.

## Modding contract

Mods target stable semantic character, presentation, texture-region, eye-layer,
and complete-variant identities.

A mod may replace a complete outfit or prop-bearing variant without changing the
base character identity. Eye and texture replacements may target their declared
semantic layer or region.

The current mod contract does not require modular clothing or live attachments.
A mod that supplies a future custom system must declare its own validated
capabilities and cannot reinterpret base variants silently.

## Validation

Required preparation validation proves:

- Krusty, Lisa, Principal Skinner, and Chief Wiggum all pass;
- topology counts remain unchanged;
- mesh and material ownership is deterministic;
- UV coverage is finite, in bounds, and non-ambiguous;
- atlas bytes, hashes, encoding, and padding are deterministic;
- sclera, pupil, upper eyelid, and lower eyelid resolve for both eyes;
- shared eye profiles and local overrides resolve deterministically;
- hierarchy, bind, inverse bind, skin weights, and animation transforms match;
- sampled deformation is unchanged;
- support-bone hiding is visual-only or deferred;
- every outfit and integrated-prop presentation is self-contained;
- no runtime attachment or modular garment claim appears in generated metadata;
- repeated preparation produces equivalent artifacts; and
- Unreal import reads the prepared contract without inventing new separation.

## Failure behavior

Preparation fails closed on:

- missing representative character evidence;
- unresolved semantic region ownership;
- absent required eye layers;
- changed topology count;
- hierarchy, bind, inverse-bind, weight, animation, or deformation drift;
- nondeterministic atlas or profile output;
- clipping or hidden-body assumptions required by a modular outfit attempt;
- integrated props represented as undeclared runtime attachments;
- importer-only separation with no FBX preparation evidence; or
- generated metadata claiming a deferred system is active.

## Invariants

- Character animation behavior is outside this phase and remains unchanged.
- Semantic preparation belongs to FBX, not first-time UAsset import.
- Every eye has sclera, pupil, upper-eyelid, and lower-eyelid ownership.
- Eye separation remains simple, customizable, and mod-friendly.
- Skeleton cleanup is visual-only and non-deforming.
- One outfit equals one complete character model.
- An integrated special prop produces another complete model variant.
- Modular clothing and runtime attachments remain deferred.
- Representative validation includes Krusty, Lisa, Principal Skinner, and Chief
  Wiggum.

<!-- markdownlint-enable MD013 -->

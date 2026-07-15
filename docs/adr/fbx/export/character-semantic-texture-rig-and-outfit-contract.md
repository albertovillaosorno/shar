# Character semantic texture, rig, outfit, and prop contract

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Canonical character FBX preparation

## Context

Character packages preserve useful topology, UV, material, skin, skeleton,
animation, and attachment evidence, but their source texture layout is optimized
for an older rendering budget rather than deterministic modern editing. Bulk FBX
publication before semantic preparation would make that legacy layout the new
interchange contract and force every later mod, material, and outfit operation
to rediscover the same boundaries.

Local comparison artifacts demonstrate that ordinary FBX can represent one or
more skinned character meshes, separately addressable eyes and accessories,
embedded base-color and shading maps, skin clusters, and animation stacks. Those
artifacts are comparison evidence only. Their topology, names, paths, material
layout, and texture content are not repository authority.

## Decision

Canonical character FBX preparation is deterministic, engine-independent, and
completed before bulk character export or native Unreal import. It preserves the
source polygon and vertex counts. Character texture modernization may change UV
coordinates, texture resolution, atlas placement, material organization, and
semantic mesh partitions, but it must not subdivide or inflate character
geometry.

Each prepared character exposes one integrated body, clothing, and presentation
representation. Clothing, handheld presentation props, and legacy costume pieces
are not reconstructed as detachable runtime garments or attachments during this
phase. One outfit or prop-bearing presentation equals one complete character FBX
model. Homer with a donut is therefore another complete model rather than a base
Homer plus a dynamically separated donut.

The architecture may later accept authored modular garments, cloth simulation,
chests, equipment, or attachment layers through stable interfaces comparable to
modern skin systems. That capability is deliberately not inferred from the
legacy characters. Reconstructing hidden bodies, transferring skin weights,
solving clipping, and proving bind and animation compatibility add substantial
risk while the complete-model representation is already faithful and stable.

Texture generation creates a deterministic modern atlas and a semantic-region
manifest. The minimum non-eye regions are skin, hair, shoes, legs, and torso.
Evidence may require additional regions such as teeth, mouth, accessories,
headwear, garment details, or integrated presentation props. Patterns and
overlays are moved into the atlas region owned by the corresponding body or
clothing surface rather than retained as unrelated overlapping texture debt.

Each eye exposes four independently addressable semantic layers: sclera, pupil,
upper eyelid, and lower eyelid. Both eyes therefore expose eight semantic layer
instances, but they need not become eight mesh objects. Layer bytes and semantic
metadata produce a deterministic eye-profile hash. Characters with identical
eye evidence may reference one shared profile, while any character can override
one or more layers through a character-local profile. This phase does not add or
change gaze, blink, controller, bone, transform, or texture-animation behavior.

Base-color regeneration samples the validated source color associated with each
covered surface and rasterizes the destination atlas from transformed UV
triangles. Flat-color regions are represented by stable neutral base colors.
Authored patterns and details are resampled into their owning regions. Color
normalization operates in linear light against a neutral daylight reference,
preserves intended pigment relationships, and avoids baking one campaign level's
illumination into the texture. Published base-color PNGs use the declared sRGB
encoding. Runtime lighting and time of day remain responsible for environmental
appearance.

Unused atlas texels use deterministic neutral fill plus edge padding or dilation
sufficient to prevent sampling bleed. Transparency is used only when the
material contract requires alpha; arbitrary white or transparent backgrounds are
not accepted as a universal atlas rule.

Normal, specular, roughness, metallic, glossiness, emissive, and
ambient-occlusion maps are preserved only when authored evidence exists or when
a versioned deterministic derivation recipe defines the output and its
verification. The existence of such maps in comparison artifacts does not by
itself authorize guessing them for source characters.

Rig cleanup is optional, visual, and non-deforming. Bone hierarchy, bind and
inverse-bind matrices, skin weights, animation local transforms, attachment
identities, and deformation results remain invariant. A display change is
accepted only when the same FBX metadata is interpreted consistently by the
supported consumers and mathematical deformation remains unchanged. Importer-
specific sidecars are not canonical. When visual continuity or support-bone
hiding cannot be represented safely in FBX metadata, cleanup is deferred rather
than approximated through rest-pose or hierarchy changes.

Geometry that appears with a character presentation, including handheld food or
costume props, remains integrated in that complete model during this phase. The
pipeline does not infer runtime sockets, activation clips, detachable props, or
hidden base bodies from legacy evidence. Future authored equipment systems may
use explicit attachments, but they do not change the current complete-model
contract.

## Consequences

- Complete character catalog export waits until semantic texture and eye-layer
  validation passes for Krusty, Lisa, Principal Skinner, and Chief Wiggum.
- Character polygon and vertex counts remain unchanged during this phase.
- Mods can address stable semantic regions instead of editing incidental source
  texel islands.
- Every outfit or prop-bearing presentation remains a self-contained complete
  model; no hidden body or modular garment reconstruction is required.
- Equivalent eye layers share a deterministic content-hashed profile, while
  character-local overrides remain possible.
- Animation behavior is outside this phase and remains unchanged.
- Validation compares topology counts, bind state, skin weights, sampled
  deformation, UV coverage, atlas and eye-layer hashes, semantic coverage, and
  texture bleed.
- Repeated preparation with equivalent evidence produces identical semantic
  manifests, atlas bytes, FBX structure, and capability reports.

## Rejected alternatives

- Publishing every character with the source texture layout and modernizing only
  the later UAsset.
- Increasing character polygon counts as part of texture modernization.
- Treating clothing as a detachable runtime layer for every character.
- Hand-painting each character or outfit as the production process.
- Guessing blink behavior, shading maps, support-bone removal, or prop
  attachment.
- Using Blender, Maya, or another authoring application as the generation,
  repair, validation, or acceptance authority.

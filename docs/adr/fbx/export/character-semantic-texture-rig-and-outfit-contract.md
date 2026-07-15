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

Each prepared character exposes one integrated body-and-clothing representation,
separate eye geometry where the source evidence supports it, and separate
animation props where geometry and attachment evidence identify an independently
activated object. Clothing is not converted into detachable runtime garments.
Each published outfit is a complete integrated character FBX variant derived
from one canonical topology, skeleton, skin-weight identity, and versioned outfit
recipe.

The outfit algorithm is character-generic. Any compatible character may receive
a validated outfit recipe, while the initial production catalog generates
multiple outfit variants only for playable characters that have source-supported
costumes. An outfit may change integrated clothing surfaces, colors, patterns,
and supported accessories. It must not silently change collision, skeleton
identity, skin weights, gameplay identity, or animation compatibility.

Texture generation creates a deterministic modern atlas and a semantic-region
manifest. The minimum non-eye regions are skin, hair, shoes, legs, and torso.
Evidence may require additional regions such as teeth, mouth, accessories,
headwear, garment details, or detachable props. Patterns and overlays are moved
into the atlas region owned by the corresponding body or clothing surface rather
than retained as unrelated overlapping texture debt.

Each eye exposes four independently addressable semantic regions: upper eyelid,
lower eyelid, eye surface, and pupil or iris. Both eyes therefore expose eight
eye regions. These are semantic UV and material regions, not a requirement to
create eight separate mesh objects. The actual blink and eye-animation mechanism
must be derived from source evidence, including bones, transforms, blend shapes,
texture animation, or controllers. The pipeline must not invent an animation
mechanism merely to satisfy the region taxonomy.

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

Rig cleanup is visual and non-deforming. Bone hierarchy, bind and inverse-bind
matrices, skin weights, animation local transforms, attachment identities, and
deformation results remain invariant. Display length, display orientation, and
non-deforming helper metadata may be normalized so bones are visually connected
and point toward their semantic children. Support bones may be hidden from the
default review display but remain present and functional. A visual cleanup that
changes deformation or animation is a failure.

Geometry that represents an independently activated animation prop, such as a
handheld food item, is separated from the integrated character mesh only when
connected-component, material, transform, attachment, animation, or controller
evidence supports that classification. The prop receives a stable identity,
attachment transform, owning socket or bone, and explicit clip activation. It is
not permanently baked into every character animation.

## Consequences

- Complete character catalog export waits until semantic texture, eye, rig, and
  representative outfit validation pass.
- Character polygon and vertex counts remain unchanged during this phase.
- Mods can address stable semantic regions instead of editing incidental source
  texel islands.
- Every outfit remains a self-contained FBX while sharing one auditable topology,
  rig, and skin source of truth.
- Eye animation and detachable props remain evidence-driven rather than inferred
  from appearance alone.
- Validation compares topology counts, bind state, skin weights, sampled
  deformation, animation timing, UV coverage, atlas hashes, semantic coverage,
  texture bleed, outfit compatibility, and prop attachment behavior.
- Repeated preparation with equivalent evidence produces identical semantic
  manifests, atlas bytes, FBX structure, and capability reports.

## Rejected alternatives

- Publishing every character with the source texture layout and modernizing only
  the later UAsset.
- Increasing character polygon counts as part of texture modernization.
- Treating clothing as a detachable runtime layer for every character.
- Hand-painting each character or outfit as the production process.
- Guessing blink behavior, shading maps, support-bone removal, or prop attachment.
- Using Blender, Maya, or another authoring application as the generation,
  repair, validation, or acceptance authority.

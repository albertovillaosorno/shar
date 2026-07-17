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

Phase 6 uses a staged native-import transaction. The importer first validates
the
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

Approved components map to a closed native representation catalog: static mesh,
native instance group, skeletal or articulated prop, rigid body, linear blocker,
query surface, breakable composite, or another registered composition. The
importer creates and reads back component hierarchy, mobility, bounds, collision
profiles, physical materials, Physics Assets, instance identity maps, and
breakable replacement policy before publication.

Collision and query geometry are cooked during import. Shipping runtime cannot
parse source triangle strips, construct custom flat-triangle stores, allocate a
parallel drawable scene graph, or infer physical behavior from a source wrapper
class. Actor and component composition is data-driven and validated.

The importer publishes immutable construction definitions, primary-asset and
bundle metadata, class-restricted soft references, dependency digests, target
variants, fallback policy, and complete rollback evidence. Shipping runtime uses
Asset Manager and retained streamable handles to load those native assets and a
closed constructor registry to prepare them.

Niagara import publishes cooked Systems, Emitters, Effect Types, typed parameter
schemas, lifetime, pooling, scalability, vehicle bindings, and breakable
presentation fallbacks. Road import publishes canonical roads, segments, lanes,
intersections, legal movements, traffic controls, deterministic splines, graph
edges, spatial-index evidence, overlays, and content digests.

Audio import publishes native sources, vehicle-audio profiles, typed parameters,
curves, attenuation, concurrency, Sound Classes, Sound Mixes, submixes, buses,
modulation, residency definitions, primary-asset bundles, environment and Audio
Volume bindings, reverb effects, collision-audio profiles, and streaming policy;
typed dialogue lines, conversations, event bindings, selection groups, locales,
subtitles, priorities, probabilities, positional and mouth policy; and explicit
listener and moving-source definitions.

Source filenames, short character or event codes, resource hashes, cluster
ordinals, fixed source slots, clip and stream player arrays, custom file
instances, sound-memory regions, namespace membership, script-created sound
objects, tuning wiring graphs, raw callbacks, and platform reverb classes remain
import provenance only and are not parsed or instantiated as packaged runtime
authority. Unreal's Audio Mixer, Audio Components, stream cache, Sound Classes,
mixes, modulation, submixes, and native device lifecycle remain engine
authority.

Prepared renderables register through native Actor and component lifecycle and
accepted render scopes. Import does not publish ordinal render layers, custom
manager loops, manual pass lists, or a second runtime renderer.

Source chunk handlers, wrapper singletons, mutable listener pointers, integer
callback user data, loader overrides, fixed global-entity or particle arrays,
road pools, source particle or breakable inventories, and null-object
cancellation
protocols are excluded from the packaged runtime.

Character import preserves the FBX topology, skeleton hierarchy, bind state,
skin weights, animation timing, semantic regions, integrated outfit identity,
eye-animation mechanism, and detachable-prop attachments. It may generate
engine-specific physics, sockets, retargeting metadata, LODs, or derived runtime
assets only through declared deterministic recipes that do not redefine the
canonical character identity.

Vehicle import consumes already separated body, wheels, trunk, and other
supported moving components with their pivots and transforms. World import
consumes already separated terrain, structures, windows, doors, linked
interiors,
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

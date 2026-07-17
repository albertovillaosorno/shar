# Native import, material rebuild, and world assembly

- Status: Planned
- Last reviewed: 2026-07-17
- Delivery phase: Phase 6 only

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Converted asset ingestion boundary](../../adr/unreal/import-adapters/converted-asset-ingestion-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Import review boundary](../../adr/unreal/import-adapters/import-review-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [World render-entity and physics runtime](world-render-entity-and-physics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native render-frame, view, and layer runtime](native-render-frame-view-and-layer-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Road-network geometry and traffic runtime](road-network-geometry-and-traffic-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle audio and avatar-sound runtime](vehicle-audio-and-avatar-sound-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Dialogue selection, queue, and playback runtime](dialogue-selection-queue-and-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Gameplay audio source, residency, mix, and environment runtime](gameplay-audio-source-residency-mix-and-environment-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native audio device, resource, player, and tuning adapter runtime](native-audio-device-resource-player-and-tuning-adapter-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored state-prop animation and event runtime](authored-state-prop-animation-and-event-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Local supersprint race session runtime](local-supersprint-race-session-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Character animation clip catalog and vehicle-handoff choreography runtime](character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical source-document evidence classification and publication boundary](historical-source-document-evidence-classification-and-publication-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native art authoring, style, and asset validation contract](native-art-authoring-style-and-asset-validation-contract.md)
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

The immutable plan is produced from
<!-- markdownlint-disable-next-line MD013 -->
[Native art authoring, style, and asset validation contract](native-art-authoring-style-and-asset-validation-contract.md).
It carries semantic asset identity, authoring revision, unit and axis policy,
geometry and topology expectations, pivot and hierarchy roles, material and
texture families, collision, LOD, Skeleton, animation, vehicle, world-kit,
platform, quality, and Data Validation requirements. Import does not reconstruct
those decisions from source filenames, folders, material slot order, or editor
defaults.

Normalized manifests own source-asset and animation counts. Raw DCC scenes,
textures, package files, office documents, screenshots, and other non-semantic
artifacts are conversion inputs and never become per-file runtime or coverage
authority.

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

## Render-entity and collision cooking

Import maps approved component and placement identities to the closed native
representation classes in
<!-- markdownlint-disable-next-line MD013 -->
[World render-entity and physics runtime](world-render-entity-and-physics-runtime.md).
The plan selects static mesh, native instancing, skeletal prop, rigid body,
linear blocker, query surface, breakable composite, or registered composite.

Collision import produces cooked simple shapes, Physics Assets, static query
meshes, physical-material bindings, and registered collision profiles. Shipping
runtime cannot parse source triangle strips or rebuild flat-triangle arrays.

Read-back verifies component hierarchy, mobility, bounds, collision responses,
query channels, mass or density, Physics Asset bindings, instance identity maps,
and breakable replacement policy. A mesh asset alone is not proof that its world
entity or physics representation is ready.

The import transaction also publishes immutable construction definitions,
primary
asset and bundle metadata, soft-reference class constraints, dependency digests,
platform variants, and fallback policy for
<!-- markdownlint-disable-next-line MD013 -->
[Native cooked-asset construction and registration runtime](native-cooked-asset-construction-and-registration-runtime.md).

No source wrapper, chunk handler, mutable listener callback, loader override,
fixed global-entity registry, or source sub-loader is included in shipping
runtime. Cook validation fails when a native construction path depends on one.

## Niagara and breakable-presentation cooking

Import converts normalized effect and breakable evidence into cooked Niagara
Systems, Emitters, Effect Types, typed parameter schemas, platform variants,
scalability policy, Geometry Collections or replacement representations,
fragment assets, and complete presentation fallbacks under
<!-- markdownlint-disable-next-line MD013 -->
[Transient VFX and breakable-presentation runtime](transient-vfx-and-breakable-presentation-runtime.md).

Read-back verifies every Niagara variable name and type, lifetime class,
coordinate-space binding, pool-reset contract, quality fallback, breakable
result
binding, replacement representation, and teardown path. Shipping runtime cannot
load source particle factories, particle inventories, breakable inventories, or
fixed source queues.

## Road-network cooking

Import converts normalized road, lane, segment, intersection, traffic-control,
and connectivity evidence into the immutable graph, spline, sample,
spatial-index,
overlay, and bundle assets defined by
<!-- markdownlint-disable-next-line MD013 -->
[Road-network geometry and traffic runtime](road-network-geometry-and-traffic-runtime.md).

Road cooking validates finite curves, joins, directions, legal lane movements,
intersection conflict groups, speed and density units, deterministic path
ordering, closest-road tie-breaks, World Partition ownership, and identical
graph
digests across clean builds. Shipping runtime does not reconstruct source linked
lists or allocate fixed road pools.

## Native audio, data, and cinematic import

WAV import creates native sound assets and preserves normalized duration, sample
rate, channel, loop, routing, subtitle, concurrency, and event identities. Cook
settings are selected later by the target policy and cannot alter logical
timing.

Vehicle-audio import publishes stable profiles, role layers, typed source
parameters, gear and pitch curves, shift envelopes, surface groups, damage,
horn,
door, overlay, attenuation, concurrency, mix, streaming, and platform policy for
<!-- markdownlint-disable-next-line MD013 -->
[Vehicle audio and avatar-sound runtime](vehicle-audio-and-avatar-sound-runtime.md).

Dialogue import converts source metadata into stable line, conversation,
selection-group, event-binding, participant-role, locale, subtitle, priority,
probability, lifetime, positional, mouth, ducking, and fallback definitions for
<!-- markdownlint-disable-next-line MD013 -->
[Dialogue selection, queue, and playback runtime](dialogue-selection-queue-and-playback-runtime.md).
Every parsed source field is recorded as provenance and validated during import;
shipping runtime never parses filenames, directory fragments, underscore counts,
short character codes, event text, level, mission, role, or conversation order.

Spatial-audio import publishes listener policies, attenuation and concurrency
assets, source definitions, attachment classes, split-screen policy, focus,
occlusion, reverb, virtualization, streaming, and platform fallback for
<!-- markdownlint-disable-next-line MD013 -->
[Spatial audio listener and positional-source runtime](spatial-audio-listener-and-positional-source-runtime.md).

Gameplay-audio import also publishes canonical generic source definitions, typed
parameters, deterministic optional-playback policy, collision-audio profiles,
Sound Classes, Sound Mixes, submixes, buses, modulation, concurrency, residency
scopes, primary-asset bundles, environment definitions, Audio Volume bindings,
reverb effects, platform fallbacks, and diagnostic metadata for
<!-- markdownlint-disable-next-line MD013 -->
[Gameplay audio source, residency, mix, and environment runtime](gameplay-audio-source-residency-mix-and-environment-runtime.md).

Fixed source arrays, clip and stream player slots, custom file instances, sound
memory regions, resource capture counts, cluster ordinals, maximum-resource
constants, raw resource keys, namespace membership, script-created sound
objects,
manual tuning wires, callback pointers, and platform reverb-controller classes
remain source provenance only.

Cooked source class, Sound Wave loading behavior, stream-cache policy, native
component lifecycle, Sound Class hierarchy, Sound Mix or modulation routing,
submix graph, output policy, device recovery, and typed callback correlation are
validated for
<!-- markdownlint-disable-next-line MD013 -->
[Native audio device, resource, player, and tuning adapter runtime](native-audio-device-resource-player-and-tuning-adapter-runtime.md).
Packaged runtime consumes native assets and immutable definitions rather than a
translated custom renderer.

State-prop import converts source states, transitions, visibility, animation
ranges, finite cycles, holds, event bindings, timing callbacks, component roles,
and initial-state policy into immutable definitions for
<!-- markdownlint-disable-next-line MD013 -->
[Authored state-prop animation and event runtime](authored-state-prop-animation-and-event-runtime.md).
Source parallel arrays, frame-controller positions, event integers, callback
integers, object factories, and listener slots remain provenance only.

Character import publishes canonical character definitions, native skeleton and
animation references, movement profiles, collision, materials, variants, vehicle
handoff policy, artificial-intelligence definitions, input contexts, attached
props, camera targets, and footprint definitions for
<!-- markdownlint-disable-next-line MD013 -->
[Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md).
Runtime does not construct model, animation, material, or texture paths from raw
character-name prefixes and suffixes.

Historical digital-content-creation animation scenes are private evidence
handled
through
<!-- markdownlint-disable-next-line MD013 -->
[Historical source-document evidence classification and publication boundary](historical-source-document-evidence-classification-and-publication-boundary.md).
Conversion records the exact character, Skeleton revision, source time unit,
frame interval, animated track profile, root policy, curves, markers, clip role,
pose purpose, variant predicates, choreography phases, and normalization recipe.
It exports one bounded animation payload and imports one validated Animation
Sequence against the declared native Skeleton. Catalog, Montage, Section, Slot,
Sync Group, Pose Asset, and vehicle-handoff definitions are created only through
<!-- markdownlint-disable-next-line MD013 -->
[Character animation clip catalog and vehicle-handoff choreography runtime](character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md).
Source scene text, backup labels, editor metadata, private paths, unused nodes,
raw curve dumps, historical batch commands, and animation-choice configuration
syntax never enter the public repository or packaged runtime.

Vehicle import publishes canonical fixed-topology Chaos vehicle definitions,
Skeletal Meshes, Physics Assets, Animation Blueprints, wheel definitions, engine
torque curves, transmission, differential, steering, brake, suspension, tire,
center-of-mass, collision, damage, seat, hardpoint, material, light, audio, VFX,
parked, traffic, pursuit, husk, reset, quality, and network policy for
<!-- markdownlint-disable-next-line MD013 -->
[Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md).

Hand-written tire and suspension force parameters, custom physics and traffic
locomotion classes, wheel-array positions, pose drivers, raw controller
mappings,
model and joint name searches, custom drawables, damage texture pointers, fixed
vehicle pools, and source event callbacks remain provenance only. Import does
not
translate them into a second packaged vehicle solver or renderer.

Supersprint import publishes canonical tracks, route and checkpoint references,
starting grids, directions, lap and turbo policy, eligible characters and
vehicles, artificial-intelligence policy, cameras, traps, HUD definitions,
audio,
high-score schemas, and retained bundles for
<!-- markdownlint-disable-next-line MD013 -->
[Local supersprint race session runtime](local-supersprint-race-session-runtime.md).
Static character, vehicle, color, waypoint, and high-score arrays are provenance
only.

JSON import creates typed data assets, tables, registries, StateTree bindings,
or
purpose-built native records. Free-form JSON objects cannot become runtime
reflection bags without a registered schema.

MOV or normalized cinematic evidence creates validated media-source,
media-player,
texture, audio, subtitle, and synchronization assets according to the cinematic
packaging policy. Import never assumes one container or codec works on every
claimed target.

Historical level-building, world-building, and state-prop guides contribute only
normalized technical facts under
<!-- markdownlint-disable-next-line MD013 -->
[Historical source-document evidence classification and publication boundary](historical-source-document-evidence-classification-and-publication-boundary.md).
Eligible facts include semantic component roles, world decomposition, tracks,
terrain, landmarks, static, dynamic, animated, breakable, and stateful props,
placement, pivots, collision, materials, animation intent, and naming evidence.
Source prose, screenshots, embedded resources, schedules, estimates, employee
assignments, obsolete exporters, source-control directions, and generated HTML,
XML, CSS, or header companions are not packaged or copied into public docs.

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

Converted bounds, cell occupancy, weighted partition, and convex-volume evidence
follow
<!-- markdownlint-disable-next-line MD013 -->
[Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md).
They support deterministic build validation and diagnostics; they do not replace
Unreal's runtime renderer or become world-object identity.

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

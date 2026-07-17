# Native art authoring, style, and asset validation contract

- Status: Active
- Last reviewed: 2026-07-17

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Converted asset ingestion boundary](../../adr/unreal/import-adapters/converted-asset-ingestion-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Import review boundary](../../adr/unreal/import-adapters/import-review-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Hexagonal scene export](../../adr/pipeline/fbx/hexagonal-scene-export.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Historical source-document evidence classification and publication boundary](historical-source-document-evidence-classification-and-publication-boundary.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](native-import-material-and-world-assembly.md)
- [Unreal configuration and asset validation](config-and-asset-validation.md)
- [Unreal gameplay content catalog](gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Character animation clip catalog and vehicle-handoff choreography runtime](character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Playable avatar, character controller, and footprint runtime](playable-avatar-character-controller-and-footprint-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native vehicle physics, control, damage, and presentation runtime](native-vehicle-physics-control-damage-and-presentation-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [World render-entity and physics runtime](world-render-entity-and-physics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Spatial visibility, bounds, and culling runtime](spatial-visibility-bounds-and-culling-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Unreal platform, quality, and optimization contract](platform-quality-and-optimization.md)

## Purpose

This specification defines the repository-owned art-authoring contract that
turns approved visual design and normalized source evidence into deterministic,
validated native Unreal assets.

It covers:

- art asset identity and taxonomy;
- source, intermediate, native, and cooked boundaries;
- character modeling, rig readiness, and animation readiness;
- props, vehicles, world kits, interiors, and environmental set dressing;
- geometry, topology, scale, pivots, hierarchy, collision, and LOD policy;
- materials, textures, UVs, palettes, cel-shaded style, and presentation
  variants;
- platform and quality-profile budgets;
- naming, folders, metadata, dependencies, and content ownership;
- deterministic export and import transactions;
- native read-back and Data Validation;
- review, rejection, recovery, diagnostics, and teardown; and
- the boundary between authoring guidance and runtime authority.

The contract replaces source-era workstation layouts, machine-specific paths,
obsolete exporters, editor preference files, manual copy steps, platform viewer
checklists, naming by file suffix, hidden directory conventions, ad hoc build
folders, hand-maintained completion sheets, and direct publication of digital
content creation scenes.

Historical instructions may establish visual intent, asset roles, topology,
rigging, hierarchy, style, or validation facts after review. They do not become
public source text, executable workflow, target folder authority, or packaged
runtime content.

## Native Unreal foundation

The target uses native Unreal facilities where applicable:

- Content Browser and native asset packages;
- Static Mesh, Skeletal Mesh, Skeleton, Physics Asset, Animation Sequence,
  Animation Blueprint, Montage, Pose Asset, Material, Material Instance,
  Texture, Niagara System, Sound, Data Asset, Data Table, and World assets;
- the FBX content pipeline for approved normalized geometry and animation;
- native collision, socket, LOD, Nanite, instancing, and component facilities;
- Asset Manager primary assets, bundles, soft references, and registry metadata;
- World Partition, Data Layers, Level Instances, HLOD, and streaming policy;
- native Data Validation and repository-owned validator extensions;
- commandlet validation in continuous integration;
- native import settings, reimport state, and dependency read-back; and
- platform cooking, derived data, compression, and scalability systems.

A second asset database, mesh renderer, skeleton evaluator, material graph,
texture streamer, collision engine, LOD switcher, or package loader requires a
separate accepted decision.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Visual direction | Defines approved style, silhouette, palette, readability, scene composition, and presentation intent. |
| Content catalog | Owns stable character, vehicle, prop, location, billboard, card, gag, interior, material, and variant identities. |
| Authoring definition | Declares asset class, source role, scale, axes, pivots, topology, UV, material, collision, LOD, rig, animation, socket, platform, and validation policy. |
| Deterministic conversion pipeline | Converts reviewed evidence into bounded normalized interchange and metadata. |
| Native import adapter | Creates or updates Unreal assets with explicit import profiles and accepted destinations. |
| Native asset | Owns the imported geometry, skeleton, animation, material, texture, collision, LOD, and editor-visible state. |
| Data Validation | Verifies naming, metadata, dependencies, budgets, compatibility, cycles, and project-specific rules. |
| Runtime systems | Consume stable native asset identities and immutable definitions; they do not parse source scenes or authoring documents. |
| Platform-quality policy | Owns platform compatibility, quality tiers, budgets, scalability, and acceptance evidence. |
| Diagnostics | Observe immutable source, conversion, import, validation, dependency, and read-back results. |

<!-- markdownlint-enable MD013 -->

Visual assets never grant progression, mission completion, ownership, currency,
seat control, collision results, or save state. They present accepted gameplay
state through native components and typed adapters.

## Stable identities

Every authoring record uses stable typed identities for:

- content definition;
- authoring definition and revision;
- visual-style profile;
- source evidence and normalized source role;
- conversion recipe and toolchain revision;
- import profile;
- target package and asset;
- mesh, skeleton, rig, animation catalog, and physics profile;
- material family, material instance, texture set, and palette;
- collision, socket, hardpoint, LOD, HLOD, and quality profile;
- world kit, module, structure, interior, prop, billboard, and set-dressing
  family;
- platform, architecture, device class, and graphics preset;
- validation rule, validation run, finding, and waiver decision; and
- build, cook, package, feature, world, and dependency revision.

A source filename, folder position, workstation drive, exporter preset name,
array index, material slot number without a semantic role, or first-loaded asset
is not durable identity.

## Repository model

Repository-owned definitions contain only public, normalized facts needed to
build and validate native assets. They may be represented as typed
configuration,
Data Assets, Data Tables, generated metadata, or deterministic manifests.

The model distinguishes:

1. **Evidence** — private historical or supplied inputs used for bounded review.
1. **Normalized source** — approved geometry, animation, texture, audio, or data
   prepared by a repository-owned deterministic pipeline.
1. **Native authoring output** — Unreal assets created from one accepted source
   and import profile.
1. **Cooked output** — platform-specific derived packages and data.
1. **Runtime definition** — immutable semantic records that reference native
   assets through stable identities and soft references.

No layer substitutes for another. Cooked output cannot become source authority,
and a source scene cannot be loaded by gameplay code.

## Asset taxonomy

The authoring taxonomy includes at least:

- playable characters;
- non-player characters and pedestrians;
- character variants and complete costumes;
- vehicle bodies, wheels, interiors, doors, damage parts, lights, and props;
- static, dynamic, animated, breakable, and stateful props;
- terrain, roads, sidewalks, structures, landmarks, interiors, and world kits;
- billboards, signs, decals, posters, and environmental graphics;
- collector cards, reward previews, icons, and presentation objects;
- ambient and interactive gag presentation assets;
- cameras, sockets, locators, triggers, hardpoints, and authored placement aids;
- VFX, materials, texture sets, palettes, and presentation variants;
- HUD and frontend presentation assets; and
- quality, collision, LOD, HLOD, and platform variants.

Every asset belongs to one primary semantic family. Shared resources use
explicit
ownership and dependency records rather than being duplicated into arbitrary
folders.

## Source and target boundaries

A source package may contain geometry, joints, animation, cameras, lights,
materials, textures, helpers, construction history, editor state, comments,
unused nodes, and machine paths. Conversion selects only the roles declared by
the authoring definition.

The target package contains only:

- accepted native assets;
- repository-owned metadata;
- stable dependency references;
- import and provenance summaries safe for publication;
- validation state; and
- required editor or runtime support assets.

The target excludes:

- source-scene text;
- absolute paths;
- workstation or user identities;
- obsolete plug-in settings;
- hidden editor state;
- unused construction nodes;
- manual export scripts;
- private review comments;
- production schedules or completion tracking;
- source screenshots and embedded documents; and
- raw asset inventories that have a normalized manifest owner.

## Naming and package layout

Canonical package layout is semantic and product-owned. It does not preserve a
historical disk hierarchy merely because that hierarchy existed.

Names encode stable role, not workflow history. A naming policy declares:

- asset type prefix when required by project conventions;
- canonical semantic identity;
- optional presentation or quality variant;
- optional LOD or platform suffix when native tooling requires it;
- character, vehicle, location, or feature ownership;
- material, texture, collision, or socket role; and
- generated-versus-authored classification.

Names must be unique after Unicode normalization and case folding where the
target platform requires it. Reserved words, ambiguous abbreviations, duplicate
normalized names, temporary labels, and machine-generated random suffixes fail
validation.

Folder layout cannot become gameplay identity. Runtime references stable primary
asset identities and soft object paths generated from accepted definitions.

## Authoring transaction

A new or revised visual asset follows one transaction:

1. resolve the semantic content identity and owning feature;
1. resolve the authoring definition and accepted visual-style profile;
1. identify the bounded source evidence and intended source roles;
1. verify source media, encoding, integrity, and review eligibility;
1. normalize units, axes, scale, pivots, hierarchy, naming, materials, and
   animation timing through a versioned recipe;
1. validate geometry, UVs, textures, rig, collision, LODs, and dependencies;
1. produce one bounded normalized interchange package and manifest;
1. import through an explicit native import profile;
1. construct or update dependent native assets;
1. read back native state and dependency graphs;
1. run project Data Validation and platform-quality checks;
1. compare native read-back with the accepted definition and tolerances;
1. publish the native asset revision and catalog binding atomically; and
1. retain or restore the previous accepted revision if any step fails.

Partial publication is forbidden. A new mesh cannot become active while its
required Skeleton, Physics Asset, material family, collision, or catalog row is
stale or missing.

## Units, axes, scale, pivots, and transforms

The normalized authoring definition declares:

- source unit and target unit;
- target axis and handedness conversion;
- canonical forward, up, and right directions;
- object, component, skeleton, and motion-root transforms;
- origin and pivot role;
- local versus world transform ownership;
- scale tolerance;
- negative-scale policy;
- transform baking policy; and
- expected native bounds.

Target scale is validated numerically and behaviorally. Camera distance,
collision shape, movement speed, door clearance, seat position, wheel radius,
foot contact, prop interaction, and world placement must remain consistent.

A visual correction cannot hide a wrong unit conversion or malformed hierarchy.
Non-uniform or negative scale is rejected unless the exact asset class and
import
profile explicitly allow and verify it.

## Geometry and topology

Geometry validation covers:

- finite positions and normals;
- deterministic vertex and index ordering where required;
- valid triangles and polygon winding;
- degenerate, zero-area, duplicate, and non-manifold geometry policy;
- smoothing and hard-edge policy;
- vertex color roles;
- tangent and normal generation or preservation;
- UV set count and channel role;
- lightmap UV policy when applicable;
- material-slot roles;
- part and component boundaries;
- bounds and origin;
- collision suitability;
- deformation readiness for skeletal meshes; and
- platform and quality budgets.

Triangle and vertex counts are budgets attached to asset class, screen role,
quality profile, and platform target. Historical fixed numbers may inform an
initial profile but cannot become universal immutable limits.

Geometry may be multipart when semantic parts need independent materials,
visibility, damage, LOD, animation, attachment, or replacement. Multipart
authoring
must preserve one declared assembly and cannot create accidental overlapping or
unowned pieces.

## Static meshes and props

A static-mesh or prop definition declares:

- semantic prop identity and class;
- static, movable, physics, animated, breakable, or stateful role;
- component and material roles;
- pivot and placement policy;
- collision and physical-material policy;
- socket or interaction points;
- LOD and optional Nanite policy;
- instancing eligibility;
- damage, breakage, and replacement presentation;
- world, location, feature, and Data Layer ownership;
- audio and VFX bindings; and
- teardown and pooling policy when applicable.

Props authored for gags or interiors must declare whether they are shared world
geometry, animation-owned temporary presentation, or stateful runtime Actors.
Animation cannot silently move static world collision or duplicate an
authoritative
prop.

A prop process is complete only when geometry, materials, collision, bounds,
placement, interaction, runtime ownership, quality, and native read-back all
pass.

## Character modeling

A character authoring definition declares:

- canonical character and body-family identity;
- playable, driver, passenger, pedestrian, mission, cinematic, or ambient roles;
- complete model and costume variants;
- mesh part and material roles;
- canonical Skeleton revision;
- skin weights and influence limits;
- bind and reference pose;
- facial, eye, mouth, and optional morph-target policy;
- collision and Physics Asset profile;
- sockets and attached-prop roles;
- LOD and quality variants;
- presentation camera requirements;
- shadow, outline, and cel-shading policy; and
- compatible animation catalogs.

Characters that share animation use an explicit compatible body and Skeleton
profile. Visual similarity or filename prefix is insufficient.

A costume is one prepared presentation variant of the same gameplay identity
unless an accepted design declares otherwise. Changing costume cannot silently
change movement, collision, mission eligibility, or save identity.

## Character skeleton and rig readiness

Rig readiness requires:

- one canonical root joint;
- deterministic hierarchy and joint order;
- normalized joint identities;
- accepted rest and bind transforms;
- finite, compatible skin weights;
- declared deformation and helper roles;
- root, motion-root, orientation, foot, hand, face, and attachment roles;
- compatible scale and coordinate conversion;
- exact Skeleton binding policy;
- Physics Asset construction or selection policy; and
- post-import hierarchy and transform read-back.

Controllers, constraints, inverse-kinematics handles, selection sets, and other
DCC helpers are conversion inputs only. They are not imported as a second
runtime
rig unless an accepted target asset explicitly requires an equivalent native
facility.

## Character animation authoring

Character animation authoring follows
<!-- markdownlint-disable-next-line MD013 -->
[Character animation clip catalog and vehicle-handoff choreography runtime](character-animation-clip-catalog-and-vehicle-handoff-choreography-runtime.md).

The authoring definition adds:

- source performance and intended clip role;
- exact time rate and range;
- required and optional track profile;
- locomotion, action, reaction, dialogue, gag, cinematic, or vehicle-handoff
  classification;
- looping, additive, root-motion, Montage, Slot, Section, marker, curve, and
  interruption policy;
- compatible character and body profiles;
- temporary prop ownership;
- camera and presentation requirements; and
- native read-back expectations.

A standardized skeleton does not imply identical motion for every body type.
Shared locomotion may have body-family variants, while character-specific clips
remain explicit catalog entries.

## Vehicle modeling and rigging

A vehicle authoring definition declares:

- canonical vehicle and presentation-variant identity;
- body, chassis, wheel, door, trunk, hood, light, glass, interior, damage, and
  detachable component roles;
- Skeletal Mesh or Static Mesh composition;
- canonical root and wheel-bone hierarchy;
- wheel centers, radii, widths, suspension axes, and steering roles;
- center-of-mass and Physics Asset profile;
- collision, seat, camera, door, exit, and hardpoint transforms;
- material, light, emissive, glass, and damage-state families;
- LOD, HLOD, quality, and platform policy;
- traffic, parked, pursuit, player, mission, preview, and husk roles; and
- native Chaos vehicle compatibility.

Wheel placement and hierarchy are validated against the native movement profile.
A visually centered wheel with an incorrect radius, axis, suspension relation,
or
bone transform fails.

Doors and occupant openings must be compatible with the character-handoff
choreography. The vehicle asset exposes stable seat, side, door, threshold, and
attachment identities; animation filenames or joint indexes cannot define them.

Project code does not reproduce a source-era vehicle solver through rigging.
Native Chaos owns vehicle physics.

## World and level authoring

World authoring uses modular semantic kits rather than one opaque export. A
world
kit may include:

- terrain and landscape inputs;
- roads, sidewalks, intersections, shortcuts, and route landmarks;
- structures, facades, roofs, windows, doors, and interiors;
- static, movable, animated, breakable, and stateful props;
- billboards, signs, decals, and environmental graphics;
- collision and physical-material surfaces;
- lights, reflection, fog, and atmosphere policy;
- cameras, triggers, locators, sockets, and interaction anchors;
- Data Layer, Level Instance, World Partition, HLOD, and streaming ownership;
- ambient population and traffic zones;
- mission, collectible, gag, and interior placement references; and
- platform and quality budgets.

Modules expose stable connection, placement, bounds, collision, and ownership
metadata. A folder or scene-group name cannot substitute for a location,
structure, road, zone, or module identity.

World import validates seams, overlaps, gaps, inaccessible regions, road and
path
connectivity, vertical clearance, collision coverage, origin policy, bounds,
streaming cells, Data Layer membership, and deterministic placement.

## Interiors

An interior authoring definition declares:

- canonical interior and owning structure;
- portal and transition identities;
- exterior and interior door presentation;
- camera-cut and loading policy;
- player spawn, exit, and safe-placement anchors;
- interactive and ambient character placements;
- ambient and interactive gag presentation zones;
- temporary animation-owned props;
- collision and inaccessible presentation areas;
- lighting, audio, reverb, and material profiles;
- level and progression availability; and
- streaming, memory, and teardown policy.

Interior art may stage presentation outside player reach, but inaccessible
presentation cannot substitute for collision, streaming, or runtime ownership.

## Billboards, signs, and environmental graphics

A billboard definition declares:

- stable billboard or sign identity;
- approved visual variant and localization policy;
- world, zone, location, and placement identity;
- geometry, material, texture, and optional rotating or animated behavior;
- emissive, lighting, readability, and distance policy;
- collision and interaction policy;
- LOD, HLOD, instancing, and streaming behavior;
- rights or approval state where required; and
- replacement and teardown behavior.

Production approval, completion, or placement columns are import-review
metadata.
They are not runtime booleans. Runtime receives only accepted definitions and
placements.

## Materials and shaders

The project uses native Material and Material Instance assets. A material-family
definition declares:

- semantic surface and visual-style role;
- parent material and instance policy;
- base color or palette behavior;
- cel-shading, outline, shadow, highlight, and emissive policy;
- opacity, masking, translucency, and blend mode;
- roughness, metallic, specular, and normal policy;
- texture parameter roles and defaults;
- vertex-color and UV-channel usage;
- physical-material binding;
- quality and platform permutations;
- static-switch budget; and
- validation and fallback behavior.

Source DCC materials are evidence and assignment carriers. Basic imported
materials may assist review, but final project materials are rebuilt or bound to
accepted native families. Unsupported or ambiguous source shading networks do
not
silently become runtime materials.

Characters, vehicles, props, worlds, UI, and VFX use separate material families
when their rendering and parameter needs differ.

## Cel-shaded visual style

The approved visual style preserves:

- readable silhouettes;
- controlled value bands;
- stable character and object palettes;
- intentional outlines and edge treatment;
- restrained specular response unless a material role requires it;
- readable eyes, mouths, facial regions, and costume separation;
- clear vehicle body, glass, light, wheel, and damage regions;
- consistent world and prop color relationships;
- stable low-quality presentation rather than arbitrary material replacement;
  and
- scene readability across day, sunset, night, interiors, and effects.

Real-time lighting, reflections, shadows, and high-quality rendering may enrich
the scene without erasing the authored graphic style.

A historical palette or style description becomes a normalized visual-style
profile with reviewed color roles, relationships, tolerances, and reference
evidence. It is not copied as prose or hard-coded into gameplay.

## Textures and UVs

A texture-set definition declares:

- semantic texture identity and owning material role;
- source and target color space;
- dimensions, aspect ratio, format, and compression policy;
- mip generation and streaming policy;
- alpha-channel role;
- normal, mask, emissive, detail, palette, decal, or UI classification;
- UV channel and addressing policy;
- platform and quality variants;
- memory and residency budget;
- fallback and missing-map behavior; and
- native read-back expectations.

Texture names encode semantic role rather than a workstation folder. Source
image organization may inform conversion, but absolute source-image paths never
enter native material authority.

UV validation covers:

- finite coordinates;
- expected channel count;
- overlap and out-of-range policy;
- mirrored and stacked regions when declared;
- texel-density policy;
- seam placement;
- lightmap channel requirements; and
- material-slot compatibility.

## Texture and material naming

Naming policy distinguishes:

- base color and palette inputs;
- normals;
- packed masks;
- roughness, metallic, specular, emissive, opacity, and detail roles;
- character body, eye, mouth, clothing, and prop regions;
- vehicle body, glass, light, wheel, interior, and damage regions;
- world, prop, billboard, decal, UI, and VFX families; and
- platform or quality variants.

A naming convention is validated through stable metadata and repository rules.
It is not used to guess gameplay identity or silently infer a missing material
family.

## Color palettes and variants

A palette definition declares:

- stable palette identity;
- named color roles;
- linear and display-space values;
- allowable tolerance or substitution rules;
- character, costume, vehicle, prop, world, billboard, UI, or VFX scope;
- day, sunset, night, interior, damage, or effect variants;
- accessibility and readability constraints; and
- target material parameter bindings.

Variants share one semantic asset identity when they change presentation only.
A palette swap cannot alter collision, Skeleton, mission behavior, ownership, or
save identity.

## LOD, HLOD, Nanite, and visibility

Each renderable asset declares:

- LOD count and generation or authored-source policy;
- screen-size or platform transition policy;
- geometry, material, bone, animation, and collision preservation rules;
- silhouette, UV, and shading tolerances;
- optional Nanite eligibility and fallback;
- HLOD participation for world content;
- instancing eligibility;
- culling bounds and significance policy;
- shadow and reflection policy per quality profile; and
- native read-back evidence.

LOD changes cannot remove authoritative collision, interaction, navigation,
mission, seat, door, socket, or attachment semantics.

Characters and vehicles may reduce bones, materials, effects, and geometry only
through accepted profiles that preserve gameplay and animation compatibility.

## Collision and physical materials

Collision authoring declares:

- collision owner and component role;
- simple, complex, convex, primitive, Physics Asset, or generated shape policy;
- query and simulation channels;
- overlap, block, and ignore responses;
- physical-material identity;
- walkable, drivable, damage, breakage, trigger, and interaction roles;
- per-quality invariants;
- bounds and penetration tolerances; and
- native test evidence.

Render geometry is not automatically valid collision. Collision must be
validated independently for movement, vehicles, camera, projectiles,
collectibles,
world interaction, and performance.

## Platform and quality profiles

Every asset class maps to the supported platform matrix and Low through Ultra
quality policy. Profiles may vary:

- geometry and LOD selection;
- Nanite and HLOD usage;
- texture resolution, format, mip bias, and residency;
- material features and shader permutations;
- shadow, reflection, lighting, post-process, and VFX complexity;
- animation update rate and bone reduction;
- audio and media quality where presentation assets depend on them;
- world streaming and instance density; and
- memory and storage budgets.

Profiles cannot change semantic content, progression, collision outcomes,
mission rules, vehicle handling, collectible placement, or interaction identity.

Android remains Low-only unless a later accepted decision changes the product
policy. Low preserves the authored cel-shaded identity while reducing expensive
features and content density through declared budgets.

## Desktop and PC authoring requirements

Desktop authoring verifies:

- supported Windows, Linux, and macOS target profiles;
- x64 and ARM64 compatibility where declared;
- native texture, mesh, shader, and package formats selected by Unreal cooking;
- scalable resolution and aspect-ratio presentation;
- keyboard, mouse, controller, and accessibility presentation where art is input
  dependent;
- deterministic content across renderer and platform adapters; and
- no dependency on source-era console viewers or workstation-specific tools.

A platform-specific visual workaround belongs in a typed platform profile and
must preserve the shared semantic asset identity.

## Export transaction

Repository-owned export is deterministic and bounded. It declares:

- source evidence digest and media type;
- accepted semantic roles;
- conversion recipe and tool versions;
- unit, axis, scale, hierarchy, naming, and material normalization;
- included and excluded objects;
- geometry, rig, animation, texture, and metadata outputs;
- output paths relative to an approved staging root;
- hashes and byte sizes;
- warnings, repairs, and terminal result; and
- reproducibility evidence.

Export never depends on current editor selection, current working directory,
filesystem enumeration order, locale, wall-clock time, or a manually copied
preset.

The same accepted input and toolchain must produce equivalent normalized output
within declared tolerances.

## Native import transaction

Native import follows
<!-- markdownlint-disable-next-line MD013 -->
[Native import, material rebuild, and world assembly](native-import-material-and-world-assembly.md).

The import profile records:

- target package and asset class;
- Static Mesh or Skeletal Mesh selection;
- Skeleton selection or creation policy;
- material and texture import policy;
- normal, tangent, smoothing, vertex-color, UV, LOD, morph-target, and animation
  options;
- collision generation or supplied-collision policy;
- unit and transform policy;
- asset metadata and primary-asset registration;
- dependent native assets to create or update;
- reimport and replacement policy; and
- read-back requirements.

Imported default materials and textures never bypass the accepted project
material and texture definitions.

## Reimport and revision replacement

Reimport is a revisioned transaction. It verifies that:

- semantic identity is unchanged or explicitly migrated;
- source and recipe revisions are expected;
- Skeleton and track compatibility remain valid;
- material-slot roles remain resolvable;
- collision, sockets, LODs, bounds, and dependencies remain compatible;
- catalog references can be updated atomically;
- user-authored native overrides are preserved or explicitly replaced; and
- failed reimport restores the previous accepted revision.

A changed filename cannot create a second canonical asset or silently orphan the
first.

## Asset registry and catalog binding

After native validation, each asset publishes a catalog binding containing:

- semantic definition identity;
- native asset identity;
- asset class and authoring revision;
- owner feature, world, location, or character;
- required bundles and dependencies;
- platform and quality availability;
- material, collision, Skeleton, animation, and presentation compatibility;
- validation status and accepted warnings;
- replacement and deprecation state; and
- safe public provenance summary.

The binding is immutable for one revision. Runtime loading never searches by
source filename, folder substring, or first matching asset.

## Data Validation

Repository validators extend Unreal Data Validation for project-specific rules.
Validation covers at least:

- package and asset naming;
- asset class and metadata;
- primary-asset identity and bundle membership;
- duplicate or missing semantic definitions;
- forbidden hard references and dependency cycles;
- geometry, topology, bounds, scale, pivots, and transforms;
- UVs, materials, textures, color space, compression, and memory;
- Skeleton, skinning, animation, Physics Asset, sockets, and track
  compatibility;
- collision, physical materials, and navigation readiness;
- LOD, HLOD, Nanite, culling, and platform profiles;
- world, Data Layer, Level Instance, and streaming ownership;
- catalog, mission, collectible, gag, billboard, interior, and placement links;
- source contamination and private metadata;
- deterministic import and read-back; and
- cooked target availability.

Validation can run on one asset, one folder, assets and dependencies, or the
project. Continuous integration uses native commandlet validation plus
repository-owned checks.

## Validation findings

Every finding records:

- stable rule identity and revision;
- asset and semantic definition identity;
- severity;
- expected and observed normalized values;
- dependency and platform scope;
- evidence safe for publication;
- remediation guidance;
- waiver eligibility; and
- terminal resolution.

Warnings cannot hide missing gameplay-critical semantics. A waiver must be
narrow, reasoned, expiring when appropriate, and rejected for confidentiality,
identity, corruption, or unsafe runtime ownership failures.

## Read-back verification

Post-import verification reads native state for:

- asset class and package;
- geometry counts and bounds;
- transforms and scale;
- material slots and material instances;
- texture dimensions, format, color space, mips, and compression;
- Skeleton, joints, skin weights, Physics Asset, sockets, and animation binding;
- collision and physical materials;
- LOD, HLOD, Nanite, and culling data;
- primary-asset metadata and bundles;
- dependencies and soft references;
- world and Data Layer ownership; and
- platform cook availability.

Read-back is compared with explicit tolerances. Import success alone is not
acceptance.

## Content review boundary

Art review evaluates visual quality and technical readiness separately.

Visual review covers:

- silhouette, proportion, expression, palette, style, readability, composition,
  and presentation intent.

Technical review covers:

- identity, scale, geometry, rig, animation, materials, textures, collision,
  LODs, dependencies, platform budgets, import, validation, and read-back.

Both must pass. A technically valid but visually incorrect asset is rejected, as
is a visually approved asset that violates runtime or platform requirements.

## Production metadata boundary

Completion, approval, assignment, milestone, owner, date, review comment, source
workstation, source folder, and tool-state columns may support private review.
They are not runtime fields and are not copied into public catalog definitions
unless a separately approved public provenance record requires a safe subset.

Native content definitions contain only product semantics and approved public
metadata.

## Failure behavior

The transaction fails closed when:

- source identity or media is ambiguous;
- required semantic roles are missing;
- units, axes, pivots, hierarchy, or scale are incompatible;
- geometry is malformed or exceeds an unwaived budget;
- UV, material, texture, color-space, or compression policy fails;
- Skeleton, skinning, animation, Physics Asset, socket, or vehicle rig is
  incompatible;
- collision or physical-material behavior is unsafe;
- LOD, HLOD, Nanite, culling, streaming, or platform policy fails;
- native import creates unexpected assets or dependencies;
- Data Validation returns a blocking finding;
- private metadata or prohibited source content appears in output;
- reimport would orphan or partially replace an accepted revision;
- read-back differs beyond tolerance; or
- deterministic reproduction cannot be proven.

Failure leaves the previous accepted asset, catalog binding, and runtime state
unchanged.

## Diagnostics

Diagnostics expose immutable views for:

- authoring definition and revision;
- source evidence classification;
- conversion recipe and output manifest;
- native import profile and result;
- geometry, rig, animation, material, texture, collision, LOD, and bounds
  summaries;
- platform and quality compatibility;
- dependency graph and bundle membership;
- validation findings and accepted waivers;
- read-back comparison;
- reimport and replacement history; and
- final publication state.

Diagnostics never expose source-scene text, private paths, user identities,
production schedules, or confidential review comments.

## Tests

Automated tests cover:

- eligibility and identity normalization;
- source-role selection and exclusion;
- deterministic unit, axis, pivot, hierarchy, and naming conversion;
- static and skeletal mesh import profiles;
- geometry, topology, UV, material, texture, and palette validation;
- character rig, skinning, animation, pose, and Skeleton compatibility;
- vehicle hierarchy, wheels, doors, seats, sockets, and Chaos compatibility;
- prop, billboard, interior, world-kit, Data Layer, and placement definitions;
- collision and physical-material behavior;
- LOD, HLOD, Nanite, culling, and quality profiles;
- platform budget projections;
- Asset Manager registration and bundles;
- Data Validation rules and commandlet execution;
- reimport compatibility and rollback;
- native read-back and dependency comparison;
- private-metadata rejection; and
- zero-resource teardown for temporary review assets.

## Invariants

- Semantic identity is independent of source filename and folder position.
- Raw DCC, image, model, package, audio, video, Office, and cache files are not
  tracked as individual semantic coverage rows.
- Normalized manifests own asset counts and conversion evidence.
- Source scenes and historical workflow text never ship or execute at runtime.
- Every native asset has one accepted authoring definition and catalog owner.
- Geometry, rigs, animation, materials, textures, collision, LODs, and platform
  profiles are validated independently.
- Visual approval and technical acceptance are both required.
- Native Unreal assets and components remain engine authority.
- Runtime code consumes stable definitions and soft references, not source
  paths.
- Reimport is revisioned, atomic, validated, and recoverable.
- Quality profiles may reduce cost but never alter gameplay semantics.
- Data Validation blocks ambiguous identity, unsafe dependencies, source
  contamination, and incompatible native state.
- Production administration and completion tracking never become runtime data.
- The previous accepted revision survives every failed conversion or import.

# Semantic component and geographic placement contract

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Canonical prop, vehicle, and world FBX preparation

## Context

Props, vehicles, and world packages contain geometry, transforms, locators,
materials, and hierarchy evidence that is meaningful before any destination
engine exists. Deferring semantic separation until native Unreal import would
make UAsset construction responsible for discovering wheels, doors, buildings,
windows, interiors, terrain, landmarks, and mission locations. That would make
canonical FBX incomplete, reduce interoperability, and duplicate classification
logic across importers and mod tools.

The world also needs one spatial source of truth. A mission editor cannot refer
to a school, church, road segment, interior, or landmark semantically when the
only authority is an incidental mesh name or mutable editor actor placement.

## Decision

Semantic component decomposition is completed in the canonical scene domain and
serialized into binary FBX before native Unreal import. UAsset construction may
map, validate, optimize, stream, and generate destination-specific derived
assets, but it must not perform the first discovery of a component that should
already be addressable in FBX.

Every exported semantic component has a stable canonical identity, family and
subcategory, source-package provenance, local pivot, parent-relative transform,
world transform, axis-aligned bounds, geographic placement identity, and
assembly-parent identity. Placement records use one declared coordinate system,
scale, origin, and handedness. Repeated extraction from equivalent evidence
produces the same component identities and transforms.

Standalone props and animated hazards, including wasps, are exported as their
own FBX artifacts or as explicitly identified components in an owning assembly.
A prop that is attached to a character, vehicle, building, or animation retains
its attachment identity and transform rather than being merged irreversibly into
unrelated geometry.

Vehicle FBX separates the rigid body from each independently moving wheel. Each
wheel has its own pivot, axle orientation, side and ordinal identity, steering
and suspension role, and body-relative transform. The trunk is a distinct
component when hinge or animation evidence supports it. Doors, hood panels,
steering elements, lights, damage pieces, and other moving parts are separated
only when package, hierarchy, pivot, controller, collision, or animation evidence
supports that contract. Visual adjacency alone is insufficient.

World FBX separates terrain from structures placed above it. Houses, buildings,
windows, doors, signs, street furniture, vegetation, interactive props,
landmarks, and other independently addressable things receive stable component
identities when evidence supports the boundary. Interiors remain separate
components linked to the canonical identity of their exterior building. The
link preserves entrances, transition transforms, streaming bounds, and semantic
ownership without fusing interior and exterior geometry into one mesh.

A geographic catalog describes roads, blocks, districts, buildings, interiors,
landmarks, mission anchors, collectible anchors, interaction anchors, and other
meaningful locations. Each location references stable component identities,
world coordinates, bounds, parent geography, aliases, and supported level-state
availability. Mission definitions may therefore target a semantic location or
route in a map-like manner instead of relying on source filenames or manually
placed editor actors.

The complete geographic world has one deterministic assembly identity. A terrain
representation, component FBX artifacts, placement records, and assembly
relationships can reconstruct the whole map without manual placement. Component
separation may use scene hierarchy, package membership, connected components,
material assignments, locators, pivots, collision, controllers, and animation
evidence. When evidence is insufficient, the pipeline fails closed or consumes
a reviewed deterministic classification annotation. A manual edit in a content
authoring application is not the production assembly process.

## Consequences

- FBX artifacts remain useful to non-Unreal tools and mods because semantic
  components, pivots, and placements exist before UAsset import.
- Wheels, trunks, detachable props, wasps, terrain, buildings, windows,
  interiors, and landmarks can be validated independently.
- One geographic catalog becomes the source of truth for world assembly and
  semantic mission editing.
- Native import consumes stable components and coordinates rather than
  rediscovering them from geometry.
- Validation checks complete component coverage, unique identities, transform
  round trips, pivot behavior, attachment closure, interior-to-exterior links,
  coordinate consistency, bounds, and deterministic reassembly.
- World decomposition is the final FBX preparation lane because it depends on
  completed component, transform, taxonomy, and placement contracts.

## Rejected alternatives

- Splitting semantic components only after FBX becomes a UAsset.
- Shipping the whole world as one indivisible mesh.
- Combining terrain, buildings, and interiors into one unaddressable artifact.
- Separating vehicle or world parts by appearance alone without transform or
  behavioral evidence.
- Treating manual editor placement as coordinate authority.
- Maintaining different geographic identities for each campaign level when the
  physical location is the same.

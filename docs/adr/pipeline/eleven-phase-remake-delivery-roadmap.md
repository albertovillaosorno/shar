# Eleven-phase remake delivery roadmap

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Primary project delivery sequence

## Context

The project needs one durable dependency order from lawful local input to a
playable native reimplementation. That order must prevent downstream runtime or
packaging work from being treated as complete before its evidence and conversion
preconditions are satisfied, while excluding unrelated online products.

FBX generation also needs an internal dependency order. Publishing all models as
soon as transport succeeds would preserve source texture debt, defer semantic
component discovery to Unreal, and make later repair compete with already
published interchange artifacts.

## Decision

The repository adopts the following eleven-phase dependency sequence. The term
`roadmap` denotes ordering only; it does not promise dates, release milestones,
or completion status.

1. decode required source evidence;
1. generate the minor-unit manifest;
1. classify deterministic packages;
1. generate semantically prepared first-principles binary FBX artifacts;
1. establish native Unreal MCP terminal control;
1. convert normalized evidence into native Unreal assets;
1. implement the complete native runtime;
1. verify the Low, Medium, High, Epic, and Ultra graphics presets;
1. add local drop-in mods and AI skills;
1. package the complete local path to validated native platform builds; and
1. optimize, verify, document, and close the bounded implementation sequence.

Phase 4 proceeds in the following fixed order:

1. validate reference and source evidence without adopting external artifacts as
   authority;
1. implement deterministic character semantic-region discovery, UV
   transformation, modern atlas generation, and neutral color normalization;
1. expose sclera, pupil, upper-eyelid, and lower-eyelid ownership for both eyes
   without changing gaze, blink, bone, transform, or texture-animation behavior;
1. normalize skeleton display without changing hierarchy, bind state, skin
   weights, animation transforms, or deformation;
1. validate complete outfit and integrated-prop character variants; do not
   create modular garments, detachable animation props, or runtime attachments;
1. generate and compare the complete deterministic character catalog;
1. export standalone props, animated hazards, and wasps;
1. decompose vehicles into body, wheels, trunk, and other evidence-supported
   moving parts with stable pivots and transforms;
1. decompose world evidence into terrain, structures, windows, doors, linked
   interiors, landmarks, props, and geographic placement records; and
1. prove deterministic reconstruction of the one geographic map from its FBX
   components and assembly manifest.

Character modernization in Phase 4 does not increase polygon or vertex counts.
Semantic UV, material, texture, eye, outfit, prop, vehicle, and world component
preparation belongs to canonical FBX. Phase 6 consumes and validates that
prepared interchange evidence; it does not become the first owner of those
separations.

The roadmap excludes a first-party multiplayer campaign, matchmaking, hosted
platform, marketplace, social layer, server browser, and general launcher. Phase
9 still requires stable mod-facing server adapters so community packages can
implement independently operated multiplayer modes without becoming base-product
scope.

## Consequences

- A later phase cannot claim completion while an earlier dependency remains
  unverified or failed.
- Complete character catalog export cannot begin before Krusty, Lisa, Principal
  Skinner, and Chief Wiggum pass semantic texture, eye, rig-display, outfit, and
  prop-bearing complete-model conformance.
- Props and vehicles depend on the shared component and transform contracts.
- World decomposition remains last within FBX work because it depends on stable
  component identity, geographic placement, and deterministic assembly.
- Phase labels describe dependency order, not publication, release, or schedule
  commitments.
- Completion is finite and does not imply perpetual feature development.
- Scope additions require a new decision.

## Rejected alternatives

- Runtime-first development without sound conversion evidence.
- Publishing transport-only character FBX before texture and rig preparation.
- Deferring semantic component separation to UAsset import.
- Processing the world before characters, props, and vehicles establish the
  reusable conversion contracts.
- Combining the remake with an online platform or general editor.

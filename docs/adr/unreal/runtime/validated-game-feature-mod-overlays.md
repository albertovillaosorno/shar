# Validated game-feature mod overlays

- Status: Accepted
- Decision date: 2026-07-14
- Scope: Native projection of accepted local mod packages

## Context

Local packages already have deterministic identity, validation, dependency,
conflict, trust, preview, and atomic active-set contracts. Unreal still needs
one
native projection for accepted content without reverting to loose file
replacement, editor-directory discovery, or package-specific gameplay branches.

Portable structured data, target-cooked Unreal assets, and native executable
code
have different compatibility and trust boundaries. Treating them as one load
path would either reject useful portable content or overstate the safety and
portability of executable extensions.

## Decision

Accepted local packages project into one repository-owned overlay model.

Portable data overlays register semantic rows and primary-asset metadata through
repository-owned catalog and registry adapters. They never replace base files or
mutate base assets.

Target-cooked asset overlays use mounted, revision-bound Game Feature content.
The overlay declares the exact Unreal version, target, architecture, cook
revision, primary assets, data-registry sources, World Partition content, and
Game Feature actions it needs. Activation remains subordinate to the validated
candidate active-set transaction.

A cooked overlay may add namespaced construction definitions and constructors
only through the immutable validated construction registry. It cannot replace a
base constructor in place, intercept unrelated asset loads, mutate base bundles,
register raw package callbacks, or leave constructors and retained handles after
feature removal.

A cooked overlay may also add namespaced render-scope policy, Niagara and
breakable-presentation definitions, road-network overlays, traffic-control
policy, vehicle-audio profiles, generic source definitions, audio-residency
bundles, approved Sound Class branches, mixes, Control Buses, modulation,
submix sends, source and submix effects, environment definitions, reverb
effects,
collision-audio profiles, dialogue lines, conversations, event bindings,
selection groups, listener and positional-source definitions, subtitles,
state-prop definitions, states, transitions and markers, character definitions,
movement profiles, input contexts, material variants, attached props, footprint
definitions, character animation catalogs, clips, poses, track profiles,
Montages, Slots, Sections, sync groups, marker and curve schemas, dialogue
gestures, vehicle-handoff choreographies, billboards and placements, collector
cards and placements, ambient and interactive gags, interior presentations,
presentation catalogs, supersprint tracks, routes, grids, rulesets,
artificial-intelligence policies, vehicle definitions and variants,
movement and wheel profiles,
powertrain, steering, brake, suspension, tire, damage, reset, parked, pursuit,
husk, input, haptics, material, light, audio, VFX, camera and HUD definitions,
high-score schemas, and diagnostic views.

It cannot replace the engine frame loop, renderer, Audio Mixer, native audio
device, master Sound Class or submix graph, platform backend, stream cache,
protected residency scopes, platform mix or output policy, base VFX or audio
definitions, base road graph, traffic authority, listener policy, dialogue usage
outside its namespace, local-player identity, native input globally, Character
Movement, Chaos, the native physics scene, protected base vehicles, base
character, character-animation catalog, rig, Skeleton, or state-prop
definitions,
persistent vehicle roster, notoriety,
parking, pursuit, mission, race clocks, checkpoint and result authority,
persistent currency, or unrelated route and event queries.

Feature removal cancels owned construction, render-scope, VFX, route, traffic,
vehicle-audio, generic-audio, residency, mix, modulation, environment, dialogue,
listener, positional-source, state-prop transition, character construction,
animation catalog, clip, pose, montage, marker, curve, playback, choreography,
vehicle handoff, vehicle construction, control, artificial-intelligence,
physics,
route, parking, pursuit, husk, damage, reset, input, haptics, camera, prop,
interaction, footprint, billboard, collector-card, gag, interior-presentation,
presentation-catalog, supersprint, checkpoint, result, coin-presentation, and
sparkle requests; tears down owned runtime objects, effects, Audio Components,
playback, subtitles, mouth, ducking, Actor, component, controller, vehicle,
native-physics, camera, HUD, route, and presentation leases; clears submix, bus,
environment, input, vehicle, and state projections; releases
retained handles; unregisters namespaced constructors, assets, policies,
definitions, bindings, dialogue content, listeners, tracks, and graph overlays;
rejects stale device, loading, animation, marker, fade, playback, controller,
movement, physics, wheel, damage, parking, pursuit, husk, race, and callback
results; restores scoped base state; and proves zero owned native and project
resources, including vehicle resources, as one transaction.

Native executable packages are not loaded by this decision. They remain inactive
unless a separate accepted native-extension trust, ABI, signing, loading, and
rollback implementation exists for the exact target.

Legacy archives are transport inputs only. They are normalized and validated
into the canonical package model before any Unreal projection is considered.

## Consequences

- The canonical package declaration remains authority over identity and load
  order.
- Game Feature state is a native execution detail, not package identity.
- Portable data overlays may work across targets when every semantic capability
  resolves.
- Cooked asset overlays are target- and engine-build-specific.
- Runtime activation never scans arbitrary content folders or editor paths.
- Base assets remain immutable; overrides are deterministic overlay rows and
  references.
- Game Feature activation, catalog registration, Data Registry sources, and
  world content commit or roll back as one candidate revision.
- Removing a package restores the prior accepted overlay graph without deleting
  saved canonical identities.
- Native code never inherits trust or portability from a content-only package.

## Rejected alternatives

- Activating loose replacement files directly from a `Mods` directory.
- Treating archive order or mount order as semantic priority.
- Loading uncooked editor assets in a packaged runtime.
- Registering package content before dependency and conflict validation.
- Allowing Game Feature activation to bypass the package active-set transaction.
- Loading arbitrary native binaries because their metadata parsed successfully.

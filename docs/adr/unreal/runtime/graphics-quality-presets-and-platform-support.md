# Graphics quality presets and platform support

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Runtime quality levels, platform targets, and optimization policy

## Context

The runtime needs one deterministic gameplay implementation that can be packaged
for desktop and mobile platform families without creating platform-specific game
rules. Rendering quality must scale through explicit presets, but optimization
must not become a pretext for removing required content, changing behavior, or
shipping defects.

The former two-profile naming model is ambiguous and is no longer part of the
public contract. The lowest visual tier uses the ordinary Unreal scalability
name `Low`, while higher tiers follow a complete and predictable progression.

## Decision

The canonical graphics presets are, in ascending order:

1. `Low`;
1. `Medium`;
1. `High`;
1. `Epic`; and
1. `Ultra`.

`Low` is the minimum supported visual configuration. It uses deliberately low
rendering settings and should preserve the original game's visual readability,
with a presentation broadly comparable to the original game or a seventh-
generation console title. It must not remove gameplay-relevant geometry,
collision, navigation cues, mission feedback, or required effects.

`Medium`, `High`, and `Epic` provide monotonic increases in native Unreal visual
quality. `Ultra` enables the maximum supported settings and visual features for
the active platform and hardware. No preset may change simulation, missions,
physics, timing, progression, saves, package identities, or mod behavior.

The required platform families are Windows, Linux, macOS, and Android. The
required native architecture coverage is:

- Windows x64;
- Linux x64;
- macOS ARM64;
- Android ARM64; and
- native ARM64 desktop targets exposed as supported production targets by the
  selected Unreal toolchain.

A platform or architecture is not described as available until a native package
has been built, launched, and validated on representative hardware. Emulation or
an editor-only session does not satisfy availability.

Desktop builds expose all five graphics presets. Android builds expose `Low`
only and enforce a maximum frame-rate cap of 144 frames per second. Android must
not silently expose, select, or emulate higher presets. Changing that mobile
policy requires a later accepted decision.

Optimization uses native Unreal facilities and profiled C++ repairs first. It
must preserve correctness and the selected visual contract. Removing quality,
content, determinism, or gameplay behavior to make performance numbers appear
better is limitation, not optimization.

Unreal Engine remains authoritative for primitive bounds, per-view frustum
culling, supported dynamic occlusion, precomputed visibility, distance culling,
LOD, HLOD, Nanite, World Partition, and final render submission. Repository code
may configure, validate, diagnose, and compare these facilities, but it does not
fork the renderer or maintain a second authoritative runtime visibility tree.

Converted bounds, cells, weighted partitions, and convex-volume tests are
versioned build evidence and diagnostics. Shipping runtime use beyond native
Unreal facilities requires measured benefit, deterministic output, conservative
failure behavior, and a separate accepted decision. Culling never becomes
streaming, collision, navigation, mission, interaction, or gameplay authority.

World render entities use validated native Actor and component composition.
Chaos owns rigid-body simulation; primitive components own collision and
renderer
registration; ISM or HISM is selected only for measured repeated-mesh cases with
stable project identity outside engine instance indices. Repository code does
not
recreate a drawable scene graph, rigid-body solver, manual draw lists, or custom
runtime triangle-collision store.

Quality may change LOD, HLOD, Nanite, instancing, materials, shadows, optional
effects, Niagara complexity, post processing, display resolution, frame pacing,
and validated solver cost settings. It cannot change collision profiles,
physical
surfaces, entity identity, breakage transactions, required physics, mission
results, or persistence.

Unreal's native frame loop, tick groups, local-player views, renderer passes,
and
platform presentation remain authoritative. Repository code does not drive
rendering from a timer callback, maintain ordinal drawable layers, submit manual
world passes, or infer readiness from a visible frame.

Niagara owns particle simulation and rendering. Effect pooling, spawn counts,
fragments, and cosmetic fallback may vary by preset, but VFX completion cannot
commit gameplay and every continuous effect remains bounded by a typed lease.

Road meshes, ambient traffic density, and diagnostics may scale, but canonical
road, lane, intersection, legal-movement, traffic-control, and
route-connectivity
semantics remain identical across presets and platforms.

Making the game usable on more hardware is a desirable consequence of correct,
efficient engineering, not the product objective. The objective remains a
faithful, high-quality implementation with no avoidable technical debt.

## Consequences

- Every platform shares one gameplay, world, mission, save, package, and mod
  contract.
- Quality changes are rendering and performance changes only.
- Scalability uses native Unreal quality groups, device profiles, streaming,
  shader, memory, and platform facilities instead of parallel gameplay branches.
- C++ optimization follows measurement and preserves domain invariants.
- `Low` is a first-class supported preset, not a legacy mode.
- `Ultra` means the maximum supported quality rather than an approximate high
  preset.
- Mobile validation includes the forced `Low` policy and the 144-frames-per-
  second ceiling.
- Unsupported vendor features remain optional and fail without changing the
  gameplay contract.

## Rejected alternatives

- Retaining obsolete public preset names or compatibility aliases.
- Maintaining separate gameplay runtimes, save formats, or mod contracts for
  platforms or presets.
- Advertising an unbuilt or editor-only target as available.
- Treating broad low-end compatibility as the primary design goal.
- Improving benchmark results by deleting content, reducing quality outside the
  selected preset, changing simulation, hiding defects, or introducing visual or
  gameplay regressions.
- Using custom engine replacements when a supported native Unreal facility
  provides the required behavior.

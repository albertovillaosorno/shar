# Unreal platform, quality, and optimization contract

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)

## Purpose

This specification defines the repository-owned platform matrix, graphics-preset
projection, mobile restrictions, optimization boundary, and acceptance evidence
for the native Unreal runtime.

## Repository model

One platform-neutral gameplay domain supplies missions, physics, progression,
saves, package identities, and mod semantics. Platform adapters supply only the
native operating-system, processor-architecture, rendering, input, storage, and
packaging behavior required by Unreal.

The required distribution matrix is:

<!-- markdownlint-disable MD013 -->

| Target identifier | Platform family | Native architecture | Product class | Presets |
| :--- | :--- | :--- | :--- | :--- |
| `windows-x64` | Windows | x64 | Desktop | Low through Ultra |
| `linux-x64` | Linux | x64 | Desktop | Low through Ultra |
| `macos-arm64` | macOS | ARM64 | Desktop | Low through Ultra |
| `windows-arm64` | Windows | ARM64 | Desktop compatibility target | Low through Ultra |
| `linux-arm64` | Linux | ARM64 | Desktop compatibility target | Low through Ultra |
| `android-arm64` | Android | ARM64 | Mobile | Low only |

<!-- markdownlint-enable MD013 -->

Target identifiers are exact lowercase command and report values. Unknown
identifiers, informal aliases, or a platform name without its architecture are
invalid.

A target becomes an available product target only after a verified build-host,
SDK, and selected Unreal toolchain combination produces a native package and
that package passes the launch, runtime, rendering, input, storage, save, and
shutdown checks on representative hardware. Support for one build host does not
imply that the same host can produce every target. Emulation, cross-compilation
alone, and editor play do not prove availability.

The five graphics presets are ordered and monotonic:

<!-- markdownlint-disable MD013 -->

| Preset | Contract |
| :--- | :--- |
| Low | Lowest supported native rendering configuration while preserving every gameplay-relevant visual and collision contract. Its art target remains comparable to the original game or a seventh-generation console presentation. |
| Medium | A measured increase over Low in native texture, shadow, effects, filtering, post-processing, and view-distance quality. |
| High | A measured increase over Medium with no simulation or content divergence. |
| Epic | Unreal's high-end native scalability baseline with every selected group at or above High. |
| Ultra | Maximum supported quality for the active platform and hardware, including every stable optional feature selected for that target. |

<!-- markdownlint-enable MD013 -->

The exact command and report identifiers are `low`, `medium`, `high`, `epic`,
and `ultra`. Player-facing labels use `Low`, `Medium`, `High`, `Epic`, and
`Ultra`. Unknown spellings or aliases are invalid.

`Ultra` is not a marketing alias for `Epic`. The runtime resolves every
supported quality group and selected optional feature to its maximum validated
setting. Features unavailable on the active platform fail as unavailable and do
not silently reduce another quality group or change gameplay.

Android uses an explicit mobile device profile that exposes only `Low`. The
settings UI, command-line configuration, saved preferences, and automatic device
detection must not select `Medium`, `High`, `Epic`, or `Ultra` on Android. The
Android maximum frame-rate value is 144 frames per second. This is a ceiling,
not a promise that every device sustains 144 frames per second.

No desktop frame-rate cap is established by this specification. Desktop frame
pacing and cap policy remain a separate decision and must not be inferred from
the Android ceiling.

Optimization starts from measured evidence. Native Unreal scalability groups,
device profiles, asset streaming, level streaming, shader and pipeline caches,
material quality, visibility systems, instancing, asynchronous work, memory
budgets, and platform renderers are used before custom replacements. C++ hot
paths are optimized through profiling, bounded allocation, appropriate data
layout, deterministic concurrency, and removal of redundant work.

A lower graphics preset may deliberately select lower visual settings. Outside
that explicit preset selection, a performance change must not delete content,
reduce authored quality, alter simulation, hide a failure, weaken validation, or
change player-visible behavior. A change that does so is a limitation or defect,
not an optimization.

## Cel-shaded visual baseline

Every preset preserves the project-owned cel-shaded visual identity. The style is
inspired by the dimensional cartoon presentation of *The Simpsons Game* but uses
original materials, outlines, lighting functions, textures, meshes, and shader
code.

The shared cel-shading profile controls stepped diffuse response, bounded
specular response, character and vehicle outlines, depth and normal sensitivity,
shadow integration, emissive treatment, material exceptions, and accessibility.

Quality presets may change outline sampling, shadow resolution, distant material
complexity, dirt and footprint density, haze quality, and optional post effects.
They cannot remove mission-marker readability, hide radiation or combat hazards,
change world-clock phases, or make Chapter 7 visibility unfair.

The same profile supports sunrise, day, sunset, night, Chapter 7 irradiated cloud,
humidity, haze, wetness, dirt, damage, and mod-replaceable material definitions.

## Platform lifecycle and memory evidence

<!-- markdownlint-disable MD013 -->
Process entry, capability snapshots, display recovery, suspension, restart, legal presentation, and terminal exit follow the [native platform bootstrap and error-recovery runtime](native-platform-bootstrap-and-error-recovery-runtime.md).

Target budgets, ownership scopes, pressure response, residency, leak verification, and packaged-memory evidence follow the [memory ownership, budget, and diagnostics runtime](memory-ownership-budget-and-diagnostics-runtime.md).
<!-- markdownlint-enable MD013 -->

A quality preset may alter declared optional residency, cache size, and visual
streaming policy. It cannot redefine ownership, hide a hard-limit violation, or
free required gameplay assets.

## Invariants

- Every platform and preset consumes the same gameplay and package identities.
- Preset selection cannot change missions, physics, timing, progression, saves,
  collision, navigation, or mod semantics.
- Quality increases are monotonic from Low through Ultra for every supported
  quality group.
- Low retains all gameplay-relevant geometry, visibility cues, effects, and UI.
- Ultra resolves to the maximum validated settings supported by the active
  platform and hardware.
- Android exposes Low only and never exceeds the 144-frames-per-second ceiling.
- A platform is not advertised as available before a native package passes the
  complete acceptance suite on representative hardware.
- Optional vendor or hardware features remain optional and have native Unreal
  fallbacks.
- Optimization preserves deterministic results and cannot introduce tolerated
  bugs, missing content, or visual regressions outside the selected preset.
- Broad hardware accessibility is an engineering benefit, not the product's
  governing objective or a reason to compromise fidelity.

## Failure behavior

- An unknown platform, unsupported architecture, unavailable native toolchain,
  invalid preset, non-monotonic quality mapping, or failed package verification
  rejects the target.
- Android rejects persisted or requested presets above Low and normalizes the
  runtime to Low before gameplay begins.
- Android rejects a configured frame-rate ceiling above 144.
- A missing Ultra capability is reported explicitly; the runtime does not claim
  Ultra conformance for that capability.
- A performance change that alters deterministic gameplay, removes required
  visuals, changes collision or navigation, or causes a regression fails review
  and is reverted or repaired.
- A platform-specific adapter cannot create alternate gameplay data to bypass a
  shared-contract failure.

## Verification

- Build and launch a native package for every claimed available platform and
  architecture on representative hardware.
- Verify rendering, input, storage, save/load, restart, and clean shutdown in
  each native package.
- Capture resolved quality-group values for all five desktop presets and prove
  monotonic ordering.
- Verify Low against representative original-identity scenes and assert that no
  gameplay-relevant geometry, effect, cue, collision, or UI is missing.
- Verify Ultra resolves every supported group and selected optional feature to
  its maximum validated value.
- On Android, verify that only Low is visible and selectable, persisted settings
  above Low are rejected or normalized, and the frame-rate ceiling is 144.
- Replay deterministic scenarios across platforms and presets and compare
  simulation, mission, progression, save, and package results.
- Profile CPU, GPU, memory, storage, streaming, shader compilation, and frame
  pacing before and after each optimization.
- Require visual comparisons and gameplay regression tests for every
  performance-sensitive change.
- Record unsupported native targets as explicit blockers rather than claiming
  availability through emulation or unverified packaging.

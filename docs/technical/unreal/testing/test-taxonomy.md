# Unreal test taxonomy

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

- [Canonical seven-level campaign and world variants](../../../adr/unreal/runtime/canonical-seven-level-campaign-and-world-variants.md)
- [Common UI front end and progress projection](../../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
- [Event-driven music and ambience](../../../adr/unreal/runtime/event-driven-music-and-ambience.md)
- [Mass Entity ambient population](../../../adr/unreal/runtime/mass-entity-ambient-population.md)
- [Runtime parity test boundary](../../../adr/unreal/runtime/runtime-parity-test-boundary.md)
- [State-driven missions, interactions, interiors, and notoriety](../../../adr/unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
- [Graphics quality presets and platform support](../../../adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
- [Portable save storage and lifecycle](../../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
- [Transactional phone-booth vehicle retrieval](../../../adr/unreal/runtime/transactional-phone-booth-vehicle-retrieval.md)
- [Validated game-feature mod overlays](../../../adr/unreal/runtime/validated-game-feature-mod-overlays.md)

## Purpose

This specification explains how repository-owned Unreal tests are classified by
the contract, native target, graphics preset, input adapter, and environment they
prove.

## Repository model

Tests are grouped as deterministic repository logic, native automation, editor
integration, asset ingestion, runtime parity, graphics-preset conformance,
input-adapter conformance, platform-package conformance, or full-playthrough
verification. Each test uses the narrowest environment capable of proving its
contract.

Platform-independent domain tests prove shared simulation and identity rules.
Native package tests then prove platform lifecycle, rendering, input, storage,
save, and shutdown behavior for each claimed operating-system and architecture
target. Preset tests prove resolved quality settings and visual invariants.

## Invariants

- A test names the boundary, platform, architecture, preset, and input adapter it
  proves when those dimensions are material.
- Pure repository logic does not require a live editor or native platform
  package.
- Integration tests use synthetic or repository-owned evidence.
- Runtime parity claims map to observable behavior.
- Mission-runtime tests name the objective kind, policy, transition, and recovery
  contract they prove.
- Interaction tests distinguish reservation, presentation, progression, and save
  acceptance.
- Defect-recovery tests prove restoration to a valid state without treating the
  accidental defect as parity.
- A passing Windows package does not prove Linux, macOS, Android, x64, or ARM64.
- A passing Epic preset does not prove Low, Medium, High, or Ultra.
- Keyboard and mouse, gamepad, and touch tests assert the same semantic actions.
- Android conformance includes forced Low quality, touch completion, safe areas,
  display cutouts, lifecycle transitions, storage, and the 144-frames-per-second
  ceiling.
- Save conformance uses one logical schema and equivalent accepted revisions
  across x64, ARM64, Windows, Linux, macOS, and Android.

## Failure behavior

- Tests that depend on hidden editor state, proprietary fixtures, network
  services, undefined ordering, emulation-only behavior, or unrecorded device
  defaults are invalid.
- A passing lower-level test never substitutes for required integration, native
  package, preset, input-adapter, or runtime evidence.
- A target remains unsupported when its native package cannot be produced or its
  required evidence is incomplete.
- A preset fails conformance when resolved settings are non-monotonic, required
  visuals disappear, or gameplay behavior changes.
- A performance result fails review when it was obtained by changing gameplay,
  hiding a defect, or reducing quality outside the selected preset.

## Verification

- Repository policy tests inspect taxonomy, matrix coverage, and fixture
  boundaries.
- Native automation verifies editor-owned behavior.
- Mission-runtime suites cover travel, follow, follow-and-collect,
  hit-and-collect, destroy, avoid, race, retry, recovery, and exactly-once
  completion with malformed and stale observations.
- Interaction suites cover Smart Object reservation, cancellation, gag replay,
  interior transition rollback, vehicle-state preservation, and level-scoped
  progression.
- Notoriety suites cover fixed-point deltas, warning, pursuit waves, objective
  exemptions, decay, resolution, arrest, clamped fines, and interior policy.
- World-safety suites cover out-of-bounds, invalid floor, collision penetration,
  missing streamed actors, duplicate identities, and deterministic safe-transform
  recovery.
- Campaign suites cover the seven-level order, three base worlds, Runtime Data
  Layer membership, story transitions, progress denominators, and rational
  percentage projections.
- Frontend suites cover Common UI stacks, focus, semantic input, logical slots,
  resume selection, new-game replacement, scrapbook parity, options rollback,
  calendar fallback, and idle-event cancellation.
- Vehicle-retrieval suites cover locked projections, traffic and forced-use
  boundaries, health persistence, insufficient currency, exactly-once repair,
  safe delivery, driver presentation, mission policy, and transaction rollback.
- Ambient-population suites cover deterministic Mass plans, representation LOD,
  streaming hysteresis, avoidance, look-at, horn and violence reactions, fall and
  recovery, conversations, named-character promotion, and mission pinning.
- Music suites cover level profiles, cue bindings, state priority, Quartz
  quantization, graph parameters, mission and race events, interior commit,
  pause, lifecycle, fallback, and target-cook read-back.
- Mod-overlay suites cover portable rows, cooked Game Feature compatibility,
  dependency order, registries, world content, stale revisions, missing content,
  atomic activation, deactivation, and native-extension rejection.
- Every claimed platform and architecture passes native package launch,
  rendering, input, storage, save/load, cinematic playback, restart, and clean-
  shutdown tests.
- Cross-architecture golden saves, migration fixtures, interrupted-write fault
  injection, and Android lifecycle termination prove transactional recovery.
- Every required audio role verifies canonical duration, locale, loop boundaries,
  event timing, loading policy, stream-cache behavior, concurrency, focus, and
  no-network playback through the native target audio route.
- Every required cinematic verifies first and final frame, canonical duration,
  audio synchronization, subtitles, event timing, skip, pause, resume, and
  exactly-once completion through the native target media route.
- Every desktop preset passes resolved-setting, visual-comparison, performance,
  and deterministic-replay tests.
- Android Low passes touch, safe-area, lifecycle, frame-cap, and complete-
  playthrough tests on representative hardware.
- Full-playthrough evidence verifies closure for each claimed platform family.

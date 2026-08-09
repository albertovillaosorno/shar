# SHAR TODO

Current task list. Project phases and dated progress are recorded in
[`ROADMAP.md`](ROADMAP.md).

## Base-port rules

- [ ] Preserve the original missions, mission order, world layout, progression,
  gameplay structure, models, textures, audio, cinematics, UI, and localization.
- [ ] Do not manually redesign terrain, missions, models, textures, or world
  content for the base project.
- [ ] Limit base-asset changes to deterministic conversion, Unreal
  compatibility, import correctness, and fixes for defects introduced by the
  port.
- [ ] Keep optional creative changes, replacements, and enhancements inside
  mods.

## Repository and validation

- [ ] Finish the canonical `src/<domain>/<function>/<kind>/<part>` migration.
- [ ] Update stale paths in documentation, tests, manifests, and commands.
- [ ] Remove obsolete files and workspace references.
- [ ] Adopt Jig as the canonical repository validator.
- [ ] Keep Jig source-linked from `.dependencies/jig/source`.
- [ ] Complete the tracked `.jig/` policy, taxonomy, adapters, and projections.
- [x] Run a clean exhaustive Jig validation.
- [ ] Document the local Jig installation and decide later whether CI is useful.

## Source extraction and conversion

- [ ] Validate a user-supplied lawful source installation.
- [ ] Preserve original asset identities, package relationships, and ordering.
- [ ] Complete deterministic conversion of original models to binary FBX 7.7.
  - [x] Require actual model or world geometry before a package can reserve an
    FBX-backed `StaticMesh`; keep scene, locator, camera, animation, and physics
    evidence as companions only when geometry exists.
  - [x] Emit explicit FBX polygon smoothing metadata whenever authored normals
    exist, and prove the representative static import no longer warns about
    missing smoothing groups.
  - [x] Version the FBX catalog verifier for exact external PNG provenance
    without treating sidecars as independently promotable model inputs.
  - [x] Publish the complete verified package-level FBX catalog for every
    manifest package currently classified `requires-fbx` before promoting any
    of those plans to ready.
- [ ] Preserve source topology, UVs, materials, textures, pivots, rigs,
  animations, placements, and transforms without artistic edits.
- [ ] Correct only conversion errors where generated output differs from the
  original source evidence.
- [ ] Import the original world through source-authored FBX instead of replacing
  it with an Unreal Landscape.
- [ ] Reject heuristic map offsets, interior movements, global height raises,
  UV mirrors, and other corrections not present in source evidence.
- [ ] Verify representative character, prop, vehicle, interior, and world
  imports.
- [ ] Preserve original audio, cinematics, localization, UI, mission, and tuning
  data in deterministic normalized forms.
  - [x] Keep movie package manifests and decode reports independent of local
    source, extraction, and transaction paths.
  - [x] Keep movie-tool diagnostics on logical identities and error classes.

## Unreal assets

- [ ] Generate a public-safe deterministic Unreal import manifest.
- [ ] Apply conversion plans through tested native Unreal MCP commands.
  - [x] Parse accepted bundles into immutable typed operations without changing
    their canonical revisions.
  - [x] Verify applicable source files, roots, links, sizes, and SHA-256 before
    native execution planning.
  - [x] Compile reviewed texture, SoundWave, FileMediaSource, and ready
    static-mesh import routes while reporting every readiness blocker.
  - [x] Register and test a project-owned PostEngineInit ToolsetRegistry module
    for synchronous WAV import without replacement or implicit save.
  - [x] Route reviewed static FBX through the project-owned import toolset with
    authored normals, one combined StaticMesh target, no auxiliary asset import,
    and no replacement or implicit save.
  - [x] Import HAP MOV payloads beneath `Content/Movies`, create native
    FileMediaSource assets, verify both effects, and compensate them together.
  - [x] Audit only required live Toolset schemas for import, explicit save, and
    independent asset read-back without invoking mutations.
  - [x] Apply complete reviewed direct-import plans with global destination
    absence checks, explicit save, independent read-back, and reverse rollback.
  - [ ] Implement and execute the complete serialized package transaction loop
    for every operation family.
- [ ] Import meshes, skeletons, physics assets, and animations correctly.
  - [x] Import package-level skeletal FBX through a companion-aware transaction
    that owns the SkeletalMesh and new Skeleton together, verifies both classes
    and dirty state, saves both packages, reads both back, rejects replacement,
    and rolls both back without orphaning a Skeleton.
- [ ] Recreate materials only as required to match the original presentation.
- [ ] Import original textures without repainting, upscaling, or redesigning
  them.
- [ ] Convert original camera, mission, vehicle, gameplay, UI, and tuning data
  into native Unreal assets.
  - [x] Stop treating unresolved normalized source as a generic `DataAsset` or
    one bespoke StateTree per source bundle; publish
    `requires-semantic-conversion` until a deterministic domain compiler emits
    a concrete target. Plan-bundle v2 now carries the aggregate semantic blocker
    count in its revision and execution completeness gate.
  - [ ] Compile normalized mission-script bundles into typed `SharMission`
    definitions and bindings for the shared mission StateTree contract.
    - [x] Version normalized MFK command evidence to v3, strip trailing source
      comments from arguments, preserve nested calls, and publish deterministic
      mission/stage/objective/condition context findings plus reviewed
      compatibility adaptations.
    - [x] Fail semantic intake before Unreal planning on stale mission evidence,
      inconsistent command summaries, noncanonical ordinals, or unresolved
      context findings.
    - [x] Require every reflected mission stage to bind one validated objective
      policy row with explicit start, completion, failure, recovery, route,
      target, notoriety, catch-up, drop, and presentation identities.
    - [x] Resolve the two observed legacy context defects through exact
      path-and-command-window adaptations and independently revalidate their
      fingerprints during semantic preflight.
    - [x] Close the observed objective and condition alias registries plus exact
      objective/condition-scoped command argument counts against the repository
      mission corpus, failing unknown aliases, scopes, or argument-count drift.
    - [x] Replay normalized mission context and producer-derived summaries, then
      project a lossless mission/stage/objective/condition source-scope graph
      with one root objective per stage, closed direct mission/stage command
      scope-and-arity registries, and explicit general-scope overlap where
      observed; retain all positional values uninterpreted.
    - [ ] Map every reviewed participant, route, timing, load, checkpoint,
      presentation, reward, transition, and typed objective/condition parameter
      reference.
      - [x] Compile all 611 direct objective and 408 direct condition parameter
        shapes into typed evidence; preserve the one noncanonical `niether`
        route token and undocumented condition values without repairing them.
      - [x] Compile all 611 stage headers plus 1,832 reviewed timer,
        checkpoint, message, vehicle, waypoint, HUD, traffic, and transition
        directives, preserving opaque numeric `AddStage` flags and documented
        unused compatibility arguments exactly.
      - [x] Compile 374 reviewed mission-scope initialization directives for
        restart locators, initial walk/vehicle state, forced-car state,
        dynamic-load P3D references, and street-race prop load/unload evidence.
      - [x] Compile 2,873 selected objective-scoped participant, collectible,
        route, dialogue, timing, fee, animation, bitmap, and FMV references plus
        all 375 condition-scoped command values without inventing unresolved
        units, defaults, or legacy extension meanings.
      - [ ] Resolve the remaining camera, AI tuning, pickup, reward,
        presentation, stage-transition, and other unmapped command references
        before asset emission.
    - [ ] Emit lossless `USharMissionDefinition` assets only after the complete
      mission graph passes reference and topology validation.
  - [ ] Compile remaining normalized UI, font, localization, tuning, and other
    structured evidence into concrete Unreal types before enabling their editor
    factories.
- [ ] Convert original audio, cinematics, and localization into native Unreal
  assets.
- [ ] Preserve source world placement through Unreal streaming and partitioning
  without changing the playable layout.
- [ ] Preserve provenance and deterministic Unreal object identities.
- [ ] Make the complete import repeatable from a clean project.

## Faithful runtime

- [ ] Complete startup, saves, profiles, settings, loading, and progression.
- [ ] Reproduce original player movement, cameras, interactions, vehicles,
  traffic, pedestrians, damage, and recovery.
- [ ] Reproduce original missions, objectives, triggers, dialogue, rewards,
  collectibles, races, and progression gates.
- [ ] Reproduce original HUD, menus, navigation, subtitles, audio, cinematics,
  and localization behavior.
- [ ] Reproduce original world streaming, placement, physics, animation,
  effects, and platform input behavior.
- [ ] Preserve original mission timing, gameplay rules, and progression unless a
  technical compatibility fix is required.
- [ ] Bind generated assets through stable contracts instead of direct paths.
- [ ] Add parity tests for gameplay behavior and state transitions.

## Mods and skills

- [ ] Define deterministic mod identity, dependencies, priority, compatibility,
  supersession, and conflict rules.
  - [x] Detect duplicate normalized output paths before conversion or writing.
  - [x] Reject ambiguous outputs and report both exact source entries.
  - [x] Require clean extraction when the optional package set changes.
- [ ] Support validated replacement and extension packages for assets and data.
- [ ] Keep the unmodified faithful port as the default base-game package.
- [ ] Use one normalized desktop and Android mod import contract.
- [ ] Keep native-code mods behind an explicit trust boundary.
- [ ] Validate schemas, paths, integrity, limits, references, and load order.
- [x] Add preview and dry-run commands that show exactly what a mod changes.
- [ ] Finish user-facing and AI-agent modding skills.
- [x] Require approval before replacing content or activating packages.

## Platforms and packaging

- [ ] Package and launch the selected native desktop and Android targets.
- [ ] Require packaged-build evidence instead of editor play or emulation.
- [ ] Keep gameplay, saves, package identities, and mod contracts consistent
  across supported targets.
- [ ] Provide graphics and performance settings without changing base content.
- [ ] Profile CPU, GPU, memory, storage, streaming, shaders, loading, and frame
  time.
- [ ] Optimize only from measured evidence without removing original behavior.
- [ ] Run the complete pipeline in dependency order.
- [x] Resume safely after interruption without accepting stale partial output.
- [ ] Report progress, failures, provenance, and final artifacts.

## Final verification

- [ ] Complete a start-to-finish playthrough without progression-blocking
  defects.
- [ ] Compare missions, vehicles, collectibles, saves, localization, cinematics,
  world layout, and the ending against the original game.
- [ ] Verify generated assets preserve the original appearance and placement.
- [ ] Rebuild from clean input and compare deterministic outputs.
- [ ] Verify representative mods without changing the default base game.
- [ ] Verify an AI agent can create and validate a mod using published skills.
- [ ] Record known compatibility limitations honestly.
- [ ] Run the canonical global validation without cache.

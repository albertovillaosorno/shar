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
- [ ] Audit reported map-wide LOD/geometry overlaps before removing any
  vertices; permit only deterministic source-backed conversion corrections.
- [ ] Audit distant-object transforms from source evidence instead of accepting
  manual editor placement as world-layout authority.
- [ ] Audit the reported vertical offset in imported vehicle FBX files, record
  its deterministic cause, and remove or preserve it only from source evidence.
- [ ] Recheck those conversion audits in-game after deterministic fixes land.
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
      - [x] Compile all 611 stage headers plus 2,549 typed stage directives,
        covering all 2,454 direct-stage commands and all 95 objective commands
        explicitly delegated to stage semantics; preserve opaque numeric
        `AddStage` flags, raw AI tuning values, and compatibility arguments.
      - [x] Compile all 811 direct mission-scope commands into typed restart,
        load, vehicle, camera, presentation, hint, HUD, pedestrian-group, and
        street-race evidence with no raw mission command fallback.
      - [x] Assign all 3,605 objective-scoped commands exactly one semantic
        owner: 3,498 objective directives, 95 stage-delegated directives, and 12
        structural condition commands; compile all 375 condition-scoped command
        values without inventing unresolved units, defaults, or legacy meaning.
      - [ ] Resolve typed source identities and intentionally opaque values to
        canonical participant, route, camera, reward, presentation, transition,
        and catalog definitions before asset emission.
        - [x] Resolve reviewed character and vehicle source identities against
          the validated phase-three package index, preserving exact character
          variants, symbolic `current` vehicles, and `none` driver sentinels.
        - [x] Bind every explicit `LoadP3DFile` first argument to one canonical
          phase-three package and keep only that path in P3D summaries. Preserve
          the source loader's optional heap and inventory-section parameters as
          validated provenance rather than target runtime allocation authority.
          The mission corpus has 966 calls: 950 one-argument calls and 16
          two-argument calls, all using `GMA_LEVEL_OTHER`; the source loader
          supports a third inventory-section argument, but no base mission uses
          that form.
        - [x] Bind all 61 typed `SetPresentationBitmap` references across
          mission initialization, stage, and objective scopes through one shared
          canonical P3D package catalog. The corpus has 56 unique presentation
          paths, with zero missing package bindings and zero normalized-root
          collisions; presentation timing and drawable semantics remain
          unresolved rather than inferred from the package path.
        - [x] Bind all 90 reviewed `BindReward` P3D references through the
          shared canonical package catalog. Preserve the observed five/seven
          argument shapes plus exact reward type, mode, level, optional cost,
          and vendor tokens as source evidence; do not assign unlock or
          progression behavior from those tokens alone.
        - [x] Type all 32 reviewed `AddPurchaseCarReward` storefronts from the
          source loader: preserve exact action, choreo, position locator,
          trigger-radius lexeme, and car-start locator; bind the reward NPC to a
          canonical character package and derive only the source-backed `gil`
          vendor versus `simpson` playable-character seller choice. Bind all 26
          `AddPurchaseCarNPCWaypoint` calls to a unique prior storefront NPC.
          Level-locator resolution, merchandise/price, ownership, save-state,
          and final reward transaction semantics remain unresolved.
        - [x] Bind all 194 reviewed mission-start and animated camera or
          multi-controller component references by exact embedded component
          name,
          component kind, and source-script `levelNN` provenance. Global lookup
          is ambiguous for 190 references, while level-scoped exact matching is
          unique for all 194 with zero missing references. Four unreferenced
          level-local keys have multiple candidates; the catalog preserves those
          ambiguities so any future reference fails closed instead of choosing a
          winner. Package/member source identity is preserved without inventing
          cross-level precedence, blending, timing, or playback semantics.
        - [x] Build a package-scoped mission locator catalog from decoded
          `srr_locator` JSON `name` and type evidence; reject package-local name
          collisions and preserve cross-package duplicates as ambiguity instead
          of applying filename or global-name precedence.
        - [x] Bind each typed locator reference against the exact
          selected-source package context formed by the matching mission-load
          script, its longest matching level-load family, and indexed P3Ds
          loaded at mission start by `SetDynaLoadData` or `StreetRacePropsLoad`,
          after re-verifying source size and SHA-256. Apply only documented
          locator-type constraints, preserve the `ActivateVehicle` `NULL`
          sentinel, and keep missing or duplicated active names as typed
          outcomes rather than guessing precedence.
        - [x] Type Dyna Load Data postfix syntax as ordered region load/unload,
          interior load/unload, and World Sphere enable/disable operations while
          preserving exact source evidence and the observed terminal-less Level
          7 mission-start region load as an explicit legacy adaptation.
        - [x] Compile decoded base-game type-5 `DynamicZone` Dyna Load Data into
          ordered package transitions and preflight every authored P3D load
          against the phase-three package index. The 109-zone corpus carries 372
          indexed P3D loads and 728 P3D unloads; 30 unload targets are absent from
          the extracted index and therefore remain valid remove-if-present
          effects rather than false load requirements. No observed base-game
          Dyna string both loads and unloads the same P3D target; the domain
          refuses to invent runtime ordering if such a conflict appears later.
        - [x] Model `DynamicZone` traversal as aggregate child-trigger
          occupancy. The first child-volume entry of an occupancy episode emits
          the Dyna transition, overlapping child volumes do not retrigger it,
          final exit rearms the zone, and exit does not invert the transition.
          Each emitted step retains exact locator and source-package identity;
          traversal order and geometry remain caller-observed evidence.
        - [x] Separate streaming lifetime from duplicate-locator precedence for
          `bm1_bestside`: 18 references face 10 Type-3 `CarStart` candidates,
          while all 1,100 DynamicZone P3D effects touch none of those mission
          packages. Camera best-side lookup is deferred until mission reset;
          Pure3D starts in its Default inventory section, searches the current
          section first, then remaining sections in creation order. This load
          path creates Level before Mission, so duplicated best-side names choose
          the Level candidate instead of an arbitrary package winner.
        - [x] Establish base-game DynamicZone trigger/retrigger semantics without
          importing extension-only stage/checkpoint commands as game authority.
        - [ ] Trace runtime lookup for locator roles other than camera best-side
          before applying package precedence to those references.
        - [x] Classify reviewed stage markers without conflating presentation
          with topology: 6 iris and 14 fade requests are visual transitions,
          5 stay-black and 108 stage-complete markers are presentation policy,
          while 3 level-over and 1 game-over markers are terminal overrides.
          Iris wins the one observed stage that also authors fade. Successor,
          retry, rollback, and recovery graph topology remains unresolved.
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

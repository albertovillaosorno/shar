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

## Unreal assets

- [ ] Generate a public-safe deterministic Unreal import manifest.
- [ ] Apply conversion plans through tested native Unreal MCP commands.
- [ ] Import meshes, skeletons, physics assets, and animations correctly.
- [ ] Recreate materials only as required to match the original presentation.
- [ ] Import original textures without repainting, upscaling, or redesigning
  them.
- [ ] Convert original camera, mission, vehicle, gameplay, UI, and tuning data
  into native Unreal assets.
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

# SHAR TODO

Only unfinished work appears here. P0 starts the internal game pipeline and P5
contains final verification, public-safety gating, and publication. Work moves
forward through the horizons unless a typed dependency explicitly blocks it;
release-only legal/public-safety work must not preempt building the game.

Full metadata, acceptance criteria, dependencies, evidence, and planning notes
remain in typed records under `docs/todo/open/`. Completed records remain under
`docs/todo/completed/`.

**Canonical TODO format:** one `### TODO - ...` title, one synthesis paragraph,
then one direct Markdown link to the complete typed record. No per-item field
labels belong here.

## P0 — Source extraction and deterministic conversion

### TODO - Preserve the original missions, mission order, world layout,…

Preserve the original missions, mission order, world layout, progression,
gameplay structure, models, textures, audio, cinematics, UI, and localization.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/governance/preserve-the-original-missions-mission-order-world-layout-progression-gameplay.mdc](docs/todo/open/governance/preserve-the-original-missions-mission-order-world-layout-progression-gameplay.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Preserve original asset identities, package relationships, and ordering

Preserve original asset identities, package relationships, and ordering.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/preserve-original-asset-identities-package-relationships-and-ordering.mdc](docs/todo/open/conversion/preserve-original-asset-identities-package-relationships-and-ordering.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Preserve source topology, UVs, materials, textures, pivots, rigs, animations,…

Preserve source topology, UVs, materials, textures, pivots, rigs, animations,
placements, and transforms without artistic edits.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/preserve-source-topology-uvs-materials-textures-pivots-rigs-animations.mdc](docs/todo/open/conversion/preserve-source-topology-uvs-materials-textures-pivots-rigs-animations.mdc)

### TODO - Correct only conversion errors where generated output differs from…

Correct only conversion errors where generated output differs from the
original source evidence.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/correct-only-conversion-errors-where-generated-output-differs-from-the-original-source.mdc](docs/todo/open/conversion/correct-only-conversion-errors-where-generated-output-differs-from-the-original-source.mdc)

### TODO - Import the original world through source-authored FBX instead of…

Import the original world through source-authored FBX instead of replacing it
with an Unreal Landscape.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/import-the-original-world-through-source-authored-fbx-instead-of-replacing-it-with-an.mdc](docs/todo/open/conversion/import-the-original-world-through-source-authored-fbx-instead-of-replacing-it-with-an.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Verify representative character, prop, vehicle, interior, and world imports

Verify representative character, prop, vehicle, interior, and world imports.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/verify-representative-character-prop-vehicle-interior-and-world-imports.mdc](docs/todo/open/conversion/verify-representative-character-prop-vehicle-interior-and-world-imports.mdc)

### TODO - Audit reported map-wide LOD/geometry overlaps before removing any…

Audit reported map-wide LOD/geometry overlaps before removing any vertices;
permit only deterministic source-backed conversion corrections.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/audit-reported-map-wide-lod-geometry-overlaps-before-removing-any-vertices-permit-only.mdc](docs/todo/open/conversion/audit-reported-map-wide-lod-geometry-overlaps-before-removing-any-vertices-permit-only.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Recheck those conversion audits in-game after deterministic fixes land

Recheck those conversion audits in-game after deterministic fixes land.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/recheck-those-conversion-audits-in-game-after-deterministic-fixes-land.mdc](docs/todo/open/conversion/recheck-those-conversion-audits-in-game-after-deterministic-fixes-land.mdc)

## P1 — Unreal asset and mission compilation

### TODO - Generate a public-safe deterministic Unreal import manifest

Generate a public-safe deterministic Unreal import manifest.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/generate-a-public-safe-deterministic-unreal-import-manifest.mdc](docs/todo/open/unreal/generate-a-public-safe-deterministic-unreal-import-manifest.mdc)

### TODO - Apply conversion plans through tested native Unreal MCP commands

Apply conversion plans through tested native Unreal MCP commands.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/apply-conversion-plans-through-tested-native-unreal-mcp-commands.mdc](docs/todo/open/unreal/apply-conversion-plans-through-tested-native-unreal-mcp-commands.mdc)

### TODO - Implement and execute the complete serialized package transaction…

Implement and execute the complete serialized package transaction loop for
every operation family.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/implement-and-execute-the-complete-serialized-package-transaction-loop-for-every.mdc](docs/todo/open/unreal/implement-and-execute-the-complete-serialized-package-transaction-loop-for-every.mdc)

### TODO - Import meshes, skeletons, physics assets, and animations correctly

Import meshes, skeletons, physics assets, and animations correctly.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/import-meshes-skeletons-physics-assets-and-animations-correctly.mdc](docs/todo/open/unreal/import-meshes-skeletons-physics-assets-and-animations-correctly.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Recreate materials only as required to match the original presentation

Recreate materials only as required to match the original presentation.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/recreate-materials-only-as-required-to-match-the-original-presentation.mdc](docs/todo/open/unreal/recreate-materials-only-as-required-to-match-the-original-presentation.mdc)

### TODO - Import original textures without repainting, upscaling, or…

Import original textures without repainting, upscaling, or redesigning them.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/import-original-textures-without-repainting-upscaling-or-redesigning-them.mdc](docs/todo/open/unreal/import-original-textures-without-repainting-upscaling-or-redesigning-them.mdc)

### TODO - Convert original camera, mission, vehicle, gameplay, UI, and tuning…

Convert original camera, mission, vehicle, gameplay, UI, and tuning data into
native Unreal assets.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/convert-original-camera-mission-vehicle-gameplay-ui-and-tuning-data-into-native.mdc](docs/todo/open/unreal/convert-original-camera-mission-vehicle-gameplay-ui-and-tuning-data-into-native.mdc)

### TODO - Compile normalized mission-script bundles into typed `SharMission`…

Compile normalized mission-script bundles into typed `SharMission` definitions
and bindings for the shared mission StateTree contract.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/compile-normalized-mission-script-bundles-into-typed-sharmission-definitions-and.mdc](docs/todo/open/unreal/compile-normalized-mission-script-bundles-into-typed-sharmission-definitions-and.mdc)

### TODO - Map every reviewed participant, route, timing, load, checkpoint,…

Map every reviewed participant, route, timing, load, checkpoint, presentation,
reward, transition, and typed objective/condition parameter reference.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/map-every-reviewed-participant-route-timing-load-checkpoint-presentation-reward.mdc](docs/todo/open/unreal/map-every-reviewed-participant-route-timing-load-checkpoint-presentation-reward.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Resolve typed source identities and intentionally opaque values to canonical…

Resolve typed source identities and intentionally opaque values to canonical
participant, route, camera, reward, presentation, transition, and catalog
definitions before asset emission.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/resolve-typed-source-identities-and-intentionally-opaque-values-to-canonical.mdc](docs/todo/open/unreal/resolve-typed-source-identities-and-intentionally-opaque-values-to-canonical.mdc)

### TODO - Emit lossless `USharMissionDefinition` assets only after the…

Emit lossless `USharMissionDefinition` assets only after the complete mission
graph passes reference and topology validation.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/emit-lossless-usharmissiondefinition-assets-only-after-the-complete-mission-graph.mdc](docs/todo/open/unreal/emit-lossless-usharmissiondefinition-assets-only-after-the-complete-mission-graph.mdc)

### TODO - Compile remaining normalized UI, font, localization, tuning, and…

Compile remaining normalized UI, font, localization, tuning, and other
structured evidence into concrete Unreal types before enabling their editor
factories.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/compile-remaining-normalized-ui-font-localization-tuning-and-other-structured.mdc](docs/todo/open/unreal/compile-remaining-normalized-ui-font-localization-tuning-and-other-structured.mdc)

### TODO - Convert original audio, cinematics, and localization into native…

Convert original audio, cinematics, and localization into native Unreal
assets.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/convert-original-audio-cinematics-and-localization-into-native-unreal-assets.mdc](docs/todo/open/unreal/convert-original-audio-cinematics-and-localization-into-native-unreal-assets.mdc)

### TODO - Preserve source world placement through Unreal streaming and…

Preserve source world placement through Unreal streaming and partitioning
without changing the playable layout.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/preserve-source-world-placement-through-unreal-streaming-and-partitioning-without.mdc](docs/todo/open/unreal/preserve-source-world-placement-through-unreal-streaming-and-partitioning-without.mdc)

### TODO - Preserve provenance and deterministic Unreal object identities

Preserve provenance and deterministic Unreal object identities.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/preserve-provenance-and-deterministic-unreal-object-identities.mdc](docs/todo/open/unreal/preserve-provenance-and-deterministic-unreal-object-identities.mdc)

### TODO - Make the complete import repeatable from a clean project

Make the complete import repeatable from a clean project.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/make-the-complete-import-repeatable-from-a-clean-project.mdc](docs/todo/open/unreal/make-the-complete-import-repeatable-from-a-clean-project.mdc)

## P2 — Faithful runtime

### TODO - Complete startup, saves, profiles, settings, loading, and progression

Complete startup, saves, profiles, settings, loading, and progression.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/complete-startup-saves-profiles-settings-loading-and-progression.mdc](docs/todo/open/runtime/complete-startup-saves-profiles-settings-loading-and-progression.mdc)

### TODO - Reproduce original player movement, cameras, interactions, vehicles,…

Reproduce original player movement, cameras, interactions, vehicles, traffic,
pedestrians, damage, and recovery.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/reproduce-original-player-movement-cameras-interactions-vehicles-traffic.mdc](docs/todo/open/runtime/reproduce-original-player-movement-cameras-interactions-vehicles-traffic.mdc)

### TODO - Reproduce original missions, objectives, triggers, dialogue,…

Reproduce original missions, objectives, triggers, dialogue, rewards,
collectibles, races, and progression gates.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/reproduce-original-missions-objectives-triggers-dialogue-rewards-collectibles.mdc](docs/todo/open/runtime/reproduce-original-missions-objectives-triggers-dialogue-rewards-collectibles.mdc)

### TODO - Reproduce original HUD, menus, navigation, subtitles, audio,…

Reproduce original HUD, menus, navigation, subtitles, audio, cinematics, and
localization behavior.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/reproduce-original-hud-menus-navigation-subtitles-audio-cinematics-and.mdc](docs/todo/open/runtime/reproduce-original-hud-menus-navigation-subtitles-audio-cinematics-and.mdc)

### TODO - Reproduce original world streaming, placement, physics, animation,…

Reproduce original world streaming, placement, physics, animation, effects,
and platform input behavior.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/reproduce-original-world-streaming-placement-physics-animation-effects-and-platform.mdc](docs/todo/open/runtime/reproduce-original-world-streaming-placement-physics-animation-effects-and-platform.mdc)

### TODO - Preserve original mission timing, gameplay rules, and progression…

Preserve original mission timing, gameplay rules, and progression unless a
technical compatibility fix is required.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/preserve-original-mission-timing-gameplay-rules-and-progression-unless-a-technical.mdc](docs/todo/open/runtime/preserve-original-mission-timing-gameplay-rules-and-progression-unless-a-technical.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Bind generated assets through stable contracts instead of direct paths

Bind generated assets through stable contracts instead of direct paths.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/bind-generated-assets-through-stable-contracts-instead-of-direct-paths.mdc](docs/todo/open/runtime/bind-generated-assets-through-stable-contracts-instead-of-direct-paths.mdc)

### TODO - Add parity tests for gameplay behavior and state transitions

Add parity tests for gameplay behavior and state transitions.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/add-parity-tests-for-gameplay-behavior-and-state-transitions.mdc](docs/todo/open/runtime/add-parity-tests-for-gameplay-behavior-and-state-transitions.mdc)

## P3 — Build and packaged-game pipeline

### TODO - Define the portable `dist/` layout and `game/manifest/dist.json`

Define a semantic per-platform package contract without freezing incidental
Unreal runtime filenames or copying Fortnite launcher/anti-cheat structure.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/define-the-portable-dist-layout-and-machine-readable-dist-manifest.mdc](docs/todo/open/packaging/define-the-portable-dist-layout-and-machine-readable-dist-manifest.mdc)

### TODO - Canonical multi-platform build runner

Implement `tools/build/adapter-inbound/run.py` to consume saved preflight and
architecture
decisions, revalidate them, build every selected target transactionally, and
publish only the minimal native deliverable under `dist/<ARCH>/` for each
successful architecture.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/canonical-multi-platform-build-runner.mdc](docs/todo/open/packaging/canonical-multi-platform-build-runner.mdc)

### TODO - Package Linux, macOS, Android, Windows x86-64, and any…

Package Linux, macOS, Android, Windows x86-64, and any Windows-on-ARM target
that current Unreal/toolchain support can validate.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/package-linux-macos-android-windows-x86-64-and-any-windows-on-arm-target-that.mdc](docs/todo/open/packaging/package-linux-macos-android-windows-x86-64-and-any-windows-on-arm-target-that.mdc)

### TODO - Produce a local iOS `.ipa` package for sideloading/testing without…

Produce a local iOS `.ipa` package for sideloading/testing without App Store
submission or an Xcode-dependent authoring workflow; document any unavoidable
Apple signing or build-host constraints explicitly.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/produce-a-local-ios-ipa-package-for-sideloading-testing-without-app-store-submission.mdc](docs/todo/open/packaging/produce-a-local-ios-ipa-package-for-sideloading-testing-without-app-store-submission.mdc)

### TODO - Package and launch the selected native desktop and Android targets

Package and launch the selected native desktop and Android targets.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/package-and-launch-the-selected-native-desktop-and-android-targets.mdc](docs/todo/open/packaging/package-and-launch-the-selected-native-desktop-and-android-targets.mdc)

### TODO - Require packaged-build evidence instead of editor play or emulation

Require packaged-build evidence instead of editor play or emulation.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/require-packaged-build-evidence-instead-of-editor-play-or-emulation.mdc](docs/todo/open/packaging/require-packaged-build-evidence-instead-of-editor-play-or-emulation.mdc)

### TODO - Keep gameplay, saves, package identities, and mod contracts…

Keep gameplay, saves, package identities, and mod contracts consistent across
supported targets.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/keep-gameplay-saves-package-identities-and-mod-contracts-consistent-across-supported.mdc](docs/todo/open/packaging/keep-gameplay-saves-package-identities-and-mod-contracts-consistent-across-supported.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Provide graphics and performance settings without changing base content

Provide graphics and performance settings without changing base content.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/provide-graphics-and-performance-settings-without-changing-base-content.mdc](docs/todo/open/packaging/provide-graphics-and-performance-settings-without-changing-base-content.mdc)

### TODO - Profile CPU, GPU, memory, storage, streaming, shaders, loading, and…

Profile CPU, GPU, memory, storage, streaming, shaders, loading, and frame
time.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/profile-cpu-gpu-memory-storage-streaming-shaders-loading-and-frame-time.mdc](docs/todo/open/packaging/profile-cpu-gpu-memory-storage-streaming-shaders-loading-and-frame-time.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Optimize only from measured evidence without removing original behavior

Optimize only from measured evidence without removing original behavior.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/optimize-only-from-measured-evidence-without-removing-original-behavior.mdc](docs/todo/open/packaging/optimize-only-from-measured-evidence-without-removing-original-behavior.mdc)

### TODO - Run the complete pipeline in dependency order

Run the complete pipeline in dependency order.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/run-the-complete-pipeline-in-dependency-order.mdc](docs/todo/open/packaging/run-the-complete-pipeline-in-dependency-order.mdc)

### TODO - Report progress, failures, provenance, and final artifacts

Report progress, failures, provenance, and final artifacts.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/report-progress-failures-provenance-and-final-artifacts.mdc](docs/todo/open/packaging/report-progress-failures-provenance-and-final-artifacts.mdc)

## P4 — Mods, product surface, and user tooling

### TODO - Build the lightweight `src/user` exporter and cross-platform GUI

Create the ordinary-player Python release surface that reads the original game
in place, selects targets, and writes only to its own workspace and `dist/`.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/build-the-lightweight-src-user-exporter-and-cross-platform-gui.mdc](docs/todo/open/packaging/build-the-lightweight-src-user-exporter-and-cross-platform-gui.mdc)

### TODO - Make `AGENTS.md` mod-first; validate C++ with Clang

Make mod authoring the default user-facing agent posture, preserve a separate
repository-engineering mode, and validate native mod C++ with strict Clang
gates.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/make-agents-md-default-to-shar-mod-authoring-and-validate-cpp-with-clang.mdc](docs/todo/open/mods/make-agents-md-default-to-shar-mod-authoring-and-validate-cpp-with-clang.mdc)

### TODO - Define deterministic mod identity, dependencies, priority,…

Define deterministic mod identity, dependencies, priority, compatibility,
supersession, and conflict rules.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/define-deterministic-mod-identity-dependencies-priority-compatibility-supersession.mdc](docs/todo/open/mods/define-deterministic-mod-identity-dependencies-priority-compatibility-supersession.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Support validated replacement and extension packages for assets and data

Support validated replacement and extension packages for assets and data.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/support-validated-replacement-and-extension-packages-for-assets-and-data.mdc](docs/todo/open/mods/support-validated-replacement-and-extension-packages-for-assets-and-data.mdc)

### TODO - Keep the unmodified faithful port as the default base-game package

Keep the unmodified faithful port as the default base-game package.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/keep-the-unmodified-faithful-port-as-the-default-base-game-package.mdc](docs/todo/open/mods/keep-the-unmodified-faithful-port-as-the-default-base-game-package.mdc)

### TODO - Use one normalized portable mod import contract

Use one inspectable SHAR mod-package contract across desktop and mobile.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/use-one-normalized-desktop-and-android-mod-import-contract.mdc](docs/todo/open/mods/use-one-normalized-desktop-and-android-mod-import-contract.mdc)

### TODO - Keep native-code mods behind an explicit trust boundary

Keep native-code mods behind an explicit trust boundary.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/keep-native-code-mods-behind-an-explicit-trust-boundary.mdc](docs/todo/open/mods/keep-native-code-mods-behind-an-explicit-trust-boundary.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Validate schemas, paths, integrity, limits, references, and load order

Validate schemas, paths, integrity, limits, references, and load order.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/validate-schemas-paths-integrity-limits-references-and-load-order.mdc](docs/todo/open/mods/validate-schemas-paths-integrity-limits-references-and-load-order.mdc)

### TODO - Finish user-facing and AI-agent modding skills

Finish user-facing and AI-agent modding skills.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/finish-user-facing-and-ai-agent-modding-skills.mdc](docs/todo/open/mods/finish-user-facing-and-ai-agent-modding-skills.mdc)

### TODO - Ship four independent base save slots; add autosave only after…

Ship four independent base save slots; add autosave only after faithful port
parity is stable.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/ship-four-independent-base-save-slots-add-autosave-only-after-faithful-port-parity-is.mdc](docs/todo/open/product/ship-four-independent-base-save-slots-add-autosave-only-after-faithful-port-parity-is.mdc)

### TODO - Support keyboard/mouse and controller input across gameplay and menus

Support keyboard/mouse and controller input across gameplay and menus.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/support-keyboard-mouse-and-controller-input-across-gameplay-and-menus.mdc](docs/todo/open/product/support-keyboard-mouse-and-controller-input-across-gameplay-and-menus.mdc)

### TODO - Keep loading screens optional once streaming can replace them safely

Keep loading screens optional once streaming can replace them safely.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/keep-loading-screens-optional-once-streaming-can-replace-them-safely.mdc](docs/todo/open/product/keep-loading-screens-optional-once-streaming-can-replace-them-safely.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Ship a default, fully obtainable base achievement set suitable for 100%/platinum-style…

Ship a default, fully obtainable base achievement set suitable for
100%/platinum-style completion, with mod achievements separately namespaced.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/ship-a-default-fully-obtainable-base-achievement-set-suitable-for-100-platinum-style.mdc](docs/todo/open/product/ship-a-default-fully-obtainable-base-achievement-set-suitable-for-100-platinum-style.mdc)

### TODO - Add an optional Discord boundary for display username, Rich…

Add an optional Discord boundary for display username, Rich Presence,
parties/invites, and achievement-facing presentation without making Discord
identity authoritative for saves or gameplay.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/add-an-optional-discord-boundary-for-display-username-rich-presence-parties-invites.mdc](docs/todo/open/product/add-an-optional-discord-boundary-for-display-username-rich-presence-parties-invites.mdc)

### TODO - Expose base C++ extension points for boss encounters, health meters,…

Expose base C++ extension points for boss encounters, health meters, combat,
multiplayer adapters, and future mod-owned gameplay systems without
implementing replacement gameplay during the faithful-port phase.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/expose-base-c-extension-points-for-boss-encounters-health-meters-combat-multiplayer.mdc](docs/todo/open/product/expose-base-c-extension-points-for-boss-encounters-health-meters-combat-multiplayer.mdc)

### TODO - Add a first-class `Mods` route when creating or loading a game

Add a first-class `Mods` route when creating or loading a game.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/add-a-first-class-mods-route-when-creating-or-loading-a-game.mdc](docs/todo/open/product/add-a-first-class-mods-route-when-creating-or-loading-a-game.mdc)

### TODO - Classify mods as visual-only, additive/story, gameplay-extension, or…

Classify mods as visual-only, additive/story, gameplay-extension, or
native-code, with explicit compatibility and save-impact declarations.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/classify-mods-as-visual-only-additive-story-gameplay-extension-or-native-code-with.mdc](docs/todo/open/product/classify-mods-as-visual-only-additive-story-gameplay-extension-or-native-code-with.mdc)

### TODO - Allow visual-only mods to be reordered, enabled, disabled, or…

Allow visual-only mods to be reordered, enabled, disabled, or replaced from
the main menu without invalidating the active save.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/allow-visual-only-mods-to-be-reordered-enabled-disabled-or-replaced-from-the-main.mdc](docs/todo/open/product/allow-visual-only-mods-to-be-reordered-enabled-disabled-or-replaced-from-the-main.mdc)

### TODO - Treat story/additive mods as mutually incompatible by default unless…

Treat story/additive mods as mutually incompatible by default unless their
manifests explicitly declare a compatible composition contract.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/treat-story-additive-mods-as-mutually-incompatible-by-default-unless-their-manifests.mdc](docs/todo/open/product/treat-story-additive-mods-as-mutually-incompatible-by-default-unless-their-manifests.mdc)

### TODO - Permit nonvisual mods that do not mutate save/progression state, but…

Permit nonvisual mods that do not mutate save/progression state, but show an
explicit compatibility warning before activation.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/permit-nonvisual-mods-that-do-not-mutate-save-progression-state-but-show-an-explicit.mdc](docs/todo/open/product/permit-nonvisual-mods-that-do-not-mutate-save-progression-state-but-show-an-explicit.mdc)

### TODO - Let mods declare deterministic hierarchy, load order, and override…

Let mods declare deterministic hierarchy, load order, and override priority;
unresolved conflicts fail closed.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/let-mods-declare-deterministic-hierarchy-load-order-and-override-priority-unresolved.mdc](docs/todo/open/product/let-mods-declare-deterministic-hierarchy-load-order-and-override-priority-unresolved.mdc)

### TODO - Keep native C++ mods behind a trust scanner that reports filesystem,…

Keep native C++ mods behind a trust scanner that reports filesystem, process,
network, platform, save, and engine-surface access before the user decides
whether to load them.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/keep-native-c-mods-behind-a-trust-scanner-that-reports-filesystem-process-network.mdc](docs/todo/open/product/keep-native-c-mods-behind-a-trust-scanner-that-reports-filesystem-process-network.mdc)

### TODO - Make mission, model, material, texture, skeleton, coordinate,…

Make mission, model, material, texture, skeleton, coordinate, gameplay,
achievement, and UI definitions data-addressable for nonexpert mod authors.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/make-mission-model-material-texture-skeleton-coordinate-gameplay-achievement-and.mdc](docs/todo/open/product/make-mission-model-material-texture-skeleton-coordinate-gameplay-achievement-and.mdc)

### TODO - Deduplicate identical generated skeletons, textures, models, and…

Deduplicate identical generated skeletons, textures, models, and other assets
only when deterministic evidence proves equivalence. Never assume all
characters or mods share one skeleton or asset layout.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/deduplicate-identical-generated-skeletons-textures-models-and-other-assets-only-when.mdc](docs/todo/open/product/deduplicate-identical-generated-skeletons-textures-models-and-other-assets-only-when.mdc)

### TODO - Let generated base characters opt into shared…

Let generated base characters opt into shared skeleton/material/model assets
as an optimization, with simple per-asset/per-mod overrides that can break
sharing without changing global contracts.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/let-generated-base-characters-opt-into-shared-skeleton-material-model-assets-as-an.mdc](docs/todo/open/product/let-generated-base-characters-opt-into-shared-skeleton-material-model-assets-as-an.mdc)

### TODO - Keep multiplayer as an extension-ready base capability rather than a…

Keep multiplayer as an extension-ready base capability rather than a fully
implemented first-party mode during the port; mods must be able to add
replicated modes, lobbies, servers, missions, and progression namespaces.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/keep-multiplayer-as-an-extension-ready-base-capability-rather-than-a-fully-implemented.mdc](docs/todo/open/product/keep-multiplayer-as-an-extension-ready-base-capability-rather-than-a-fully-implemented.mdc)

### TODO - Keep DLSS and hardware-ray-tracing/RTX integrations behind optional…

Keep DLSS and hardware-ray-tracing/RTX integrations behind optional capability
adapters so graphics mods can target them without making proprietary GPU
features mandatory for the base port.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/keep-dlss-and-hardware-ray-tracing-rtx-integrations-behind-optional-capability-adapters.mdc](docs/todo/open/product/keep-dlss-and-hardware-ray-tracing-rtx-integrations-behind-optional-capability-adapters.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Preserve the delivery order: faithful port first, compatibility and quality-of-life…

Preserve the delivery order: faithful port first, compatibility and
quality-of-life improvements second, richer community mod content third.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/preserve-the-delivery-order-faithful-port-first-compatibility-and-quality-of-life.mdc](docs/todo/open/product/preserve-the-delivery-order-faithful-port-first-compatibility-and-quality-of-life.mdc)

## P5 — Final verification, public-safety gate, and publication

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Complete a start-to-finish playthrough without progression-blocking defects

Complete a start-to-finish playthrough without progression-blocking defects.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/complete-a-start-to-finish-playthrough-without-progression-blocking-defects.mdc](docs/todo/open/verification/complete-a-start-to-finish-playthrough-without-progression-blocking-defects.mdc)

### TODO - Compare missions, vehicles, collectibles, saves, localization,…

Compare missions, vehicles, collectibles, saves, localization, cinematics,
world layout, and the ending against the original game.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/compare-missions-vehicles-collectibles-saves-localization-cinematics-world-layout.mdc](docs/todo/open/verification/compare-missions-vehicles-collectibles-saves-localization-cinematics-world-layout.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Verify generated assets preserve the original appearance and placement

Verify generated assets preserve the original appearance and placement.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/verify-generated-assets-preserve-the-original-appearance-and-placement.mdc](docs/todo/open/verification/verify-generated-assets-preserve-the-original-appearance-and-placement.mdc)

### TODO - Rebuild from clean input and compare deterministic outputs

Rebuild from clean input and compare deterministic outputs.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/rebuild-from-clean-input-and-compare-deterministic-outputs.mdc](docs/todo/open/verification/rebuild-from-clean-input-and-compare-deterministic-outputs.mdc)

### TODO - Verify representative mods without changing the default base game

Verify representative mods without changing the default base game.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/verify-representative-mods-without-changing-the-default-base-game.mdc](docs/todo/open/verification/verify-representative-mods-without-changing-the-default-base-game.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Verify an AI agent can create and validate a mod using published skills

Verify an AI agent can create and validate a mod using published skills.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/verify-an-ai-agent-can-create-and-validate-a-mod-using-published-skills.mdc](docs/todo/open/verification/verify-an-ai-agent-can-create-and-validate-a-mod-using-published-skills.mdc)

### TODO - Record known compatibility limitations honestly

Record known compatibility limitations honestly.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/record-known-compatibility-limitations-honestly.mdc](docs/todo/open/verification/record-known-compatibility-limitations-honestly.mdc)

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: TODO title is canonical -->
### TODO - Define a public-safe reconstruction algorithm gate with bounded similarity

Define and test the tentative 45–55% source-similarity window against a lawful
100% reference without publishing payloads or reversible full-install evidence.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/security/define-a-public-safe-reconstruction-algorithm-gate-with-a-bounded-source-similarity-window.mdc](docs/todo/open/security/define-a-public-safe-reconstruction-algorithm-gate-with-a-bounded-source-similarity-window.mdc)

### TODO - Run the canonical global validation without cache

Run the canonical global validation without cache.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/run-the-canonical-global-validation-without-cache.mdc](docs/todo/open/verification/run-the-canonical-global-validation-without-cache.mdc)

### TODO - Add final player-facing README screenshots

Add screenshots only after the player GUI and final package layout stabilize.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/add-final-player-facing-readme-screenshots.mdc](docs/todo/open/verification/add-final-player-facing-readme-screenshots.mdc)

### TODO - Publish `src/user` as the versioned SHAR release ZIP

Publish only the declared lightweight user tree as `shar-v<version>.zip`, with
integrity hashes and clean end-to-end platform evidence.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/publish-src-user-as-the-versioned-shar-release-zip.mdc](docs/todo/open/verification/publish-src-user-as-the-versioned-shar-release-zip.mdc)

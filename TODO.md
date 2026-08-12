# SHAR TODO

Only unfinished work appears here. P0 is the highest-priority horizon and P5
is the final verification and publication horizon. Within each section, active
work appears first and stable task identity breaks ties; typed lanes and
dependencies remain execution authority.

Full metadata, acceptance criteria, dependencies, evidence, and planning notes
remain in typed records under `docs/todo/open/`. Completed records remain under
`docs/todo/completed/`.

**Canonical TODO format:** one `### TODO - ...` title, one synthesis paragraph,
then one direct Markdown link to the complete typed record. No per-item field
labels belong here.

## P0 — Authority and repository governance

### TODO - Preserve the original missions, mission order, world layout, progression, gameplay…

Preserve the original missions, mission order, world layout, progression,
gameplay structure, models, textures, audio, cinematics, UI, and localization.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/governance/preserve-the-original-missions-mission-order-world-layout-progression-gameplay.mdc](docs/todo/open/governance/preserve-the-original-missions-mission-order-world-layout-progression-gameplay.mdc)

### TODO - Do not manually redesign terrain, missions, models, textures, or world content for the…

Do not manually redesign terrain, missions, models, textures, or world content
for the base project.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/governance/do-not-manually-redesign-terrain-missions-models-textures-or-world-content-for-the.mdc](docs/todo/open/governance/do-not-manually-redesign-terrain-missions-models-textures-or-world-content-for-the.mdc)

### TODO - Limit base-asset changes to deterministic conversion, Unreal compatibility, import…

Limit base-asset changes to deterministic conversion, Unreal compatibility,
import correctness, and fixes for defects introduced by the port.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/governance/limit-base-asset-changes-to-deterministic-conversion-unreal-compatibility-import.mdc](docs/todo/open/governance/limit-base-asset-changes-to-deterministic-conversion-unreal-compatibility-import.mdc)

### TODO - Keep optional creative changes, replacements, and enhancements inside mods

Keep optional creative changes, replacements, and enhancements inside mods.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/governance/keep-optional-creative-changes-replacements-and-enhancements-inside-mods.mdc](docs/todo/open/governance/keep-optional-creative-changes-replacements-and-enhancements-inside-mods.mdc)

### TODO - Finish the canonical `src/<domain>/<function>/<kind>/<part>` migration

Finish the canonical `src/<domain>/<function>/<kind>/<part>` migration.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/repository/finish-the-canonical-src-domain-function-kind-part-migration.mdc](docs/todo/open/repository/finish-the-canonical-src-domain-function-kind-part-migration.mdc)

### TODO - Update stale paths in documentation, tests, manifests, and commands

Update stale paths in documentation, tests, manifests, and commands.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/repository/update-stale-paths-in-documentation-tests-manifests-and-commands.mdc](docs/todo/open/repository/update-stale-paths-in-documentation-tests-manifests-and-commands.mdc)

### TODO - Remove obsolete files and workspace references

Remove obsolete files and workspace references.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/repository/remove-obsolete-files-and-workspace-references.mdc](docs/todo/open/repository/remove-obsolete-files-and-workspace-references.mdc)

### TODO - Adopt Jig as the canonical repository validator

Adopt Jig as the canonical repository validator.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/repository/adopt-jig-as-the-canonical-repository-validator.mdc](docs/todo/open/repository/adopt-jig-as-the-canonical-repository-validator.mdc)

### TODO - Keep Jig source-linked from `.dependencies/jig/source`

Keep Jig source-linked from `.dependencies/jig/source`.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/repository/keep-jig-source-linked-from-dependencies-jig-source.mdc](docs/todo/open/repository/keep-jig-source-linked-from-dependencies-jig-source.mdc)

### TODO - Complete the tracked `.jig/` policy, taxonomy, adapters, and projections

Complete the tracked `.jig/` policy, taxonomy, adapters, and projections.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/repository/complete-the-tracked-jig-policy-taxonomy-adapters-and-projections.mdc](docs/todo/open/repository/complete-the-tracked-jig-policy-taxonomy-adapters-and-projections.mdc)

### TODO - Document the local Jig installation and decide later whether CI is useful

Document the local Jig installation and decide later whether CI is useful.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/repository/document-the-local-jig-installation-and-decide-later-whether-ci-is-useful.mdc](docs/todo/open/repository/document-the-local-jig-installation-and-decide-later-whether-ci-is-useful.mdc)

## P1 — Source extraction and deterministic conversion

### TODO - Validate a user-supplied lawful source installation

Validate a user-supplied lawful source installation.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/validate-a-user-supplied-lawful-source-installation.mdc](docs/todo/open/conversion/validate-a-user-supplied-lawful-source-installation.mdc)

### TODO - Preserve original asset identities, package relationships, and ordering

Preserve original asset identities, package relationships, and ordering.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/preserve-original-asset-identities-package-relationships-and-ordering.mdc](docs/todo/open/conversion/preserve-original-asset-identities-package-relationships-and-ordering.mdc)

### TODO - Complete deterministic conversion of original models to binary FBX 7.7

Complete deterministic conversion of original models to binary FBX 7.7.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/complete-deterministic-conversion-of-original-models-to-binary-fbx-7-7.mdc](docs/todo/open/conversion/complete-deterministic-conversion-of-original-models-to-binary-fbx-7-7.mdc)

### TODO - Preserve source topology, UVs, materials, textures, pivots, rigs, animations,…

Preserve source topology, UVs, materials, textures, pivots, rigs, animations,
placements, and transforms without artistic edits.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/preserve-source-topology-uvs-materials-textures-pivots-rigs-animations.mdc](docs/todo/open/conversion/preserve-source-topology-uvs-materials-textures-pivots-rigs-animations.mdc)

### TODO - Correct only conversion errors where generated output differs from the original source…

Correct only conversion errors where generated output differs from the
original source evidence.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/correct-only-conversion-errors-where-generated-output-differs-from-the-original-source.mdc](docs/todo/open/conversion/correct-only-conversion-errors-where-generated-output-differs-from-the-original-source.mdc)

### TODO - Import the original world through source-authored FBX instead of replacing it with an…

Import the original world through source-authored FBX instead of replacing it
with an Unreal Landscape.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/import-the-original-world-through-source-authored-fbx-instead-of-replacing-it-with-an.mdc](docs/todo/open/conversion/import-the-original-world-through-source-authored-fbx-instead-of-replacing-it-with-an.mdc)

### TODO - Reject heuristic map offsets, interior movements, global height raises, UV mirrors, and…

Reject heuristic map offsets, interior movements, global height raises, UV
mirrors, and other corrections not present in source evidence.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/reject-heuristic-map-offsets-interior-movements-global-height-raises-uv-mirrors-and.mdc](docs/todo/open/conversion/reject-heuristic-map-offsets-interior-movements-global-height-raises-uv-mirrors-and.mdc)

### TODO - Verify representative character, prop, vehicle, interior, and world imports

Verify representative character, prop, vehicle, interior, and world imports.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/verify-representative-character-prop-vehicle-interior-and-world-imports.mdc](docs/todo/open/conversion/verify-representative-character-prop-vehicle-interior-and-world-imports.mdc)

### TODO - Audit reported map-wide LOD/geometry overlaps before removing any vertices; permit only…

Audit reported map-wide LOD/geometry overlaps before removing any vertices;
permit only deterministic source-backed conversion corrections.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/audit-reported-map-wide-lod-geometry-overlaps-before-removing-any-vertices-permit-only.mdc](docs/todo/open/conversion/audit-reported-map-wide-lod-geometry-overlaps-before-removing-any-vertices-permit-only.mdc)

### TODO - Audit distant-object transforms from source evidence instead of accepting manual editor…

Audit distant-object transforms from source evidence instead of accepting
manual editor placement as world-layout authority.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/audit-distant-object-transforms-from-source-evidence-instead-of-accepting-manual-editor.mdc](docs/todo/open/conversion/audit-distant-object-transforms-from-source-evidence-instead-of-accepting-manual-editor.mdc)

### TODO - Audit the reported vertical offset in imported vehicle FBX files, record its…

Audit the reported vertical offset in imported vehicle FBX files, record its
deterministic cause, and remove or preserve it only from source evidence.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/audit-the-reported-vertical-offset-in-imported-vehicle-fbx-files-record-its.mdc](docs/todo/open/conversion/audit-the-reported-vertical-offset-in-imported-vehicle-fbx-files-record-its.mdc)

### TODO - Recheck those conversion audits in-game after deterministic fixes land

Recheck those conversion audits in-game after deterministic fixes land.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/recheck-those-conversion-audits-in-game-after-deterministic-fixes-land.mdc](docs/todo/open/conversion/recheck-those-conversion-audits-in-game-after-deterministic-fixes-land.mdc)

### TODO - Preserve original audio, cinematics, localization, UI, mission, and tuning data in…

Preserve original audio, cinematics, localization, UI, mission, and tuning
data in deterministic normalized forms.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/conversion/preserve-original-audio-cinematics-localization-ui-mission-and-tuning-data-in.mdc](docs/todo/open/conversion/preserve-original-audio-cinematics-localization-ui-mission-and-tuning-data-in.mdc)

## P2 — Unreal asset and mission compilation

### TODO - Generate a public-safe deterministic Unreal import manifest

Generate a public-safe deterministic Unreal import manifest.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/generate-a-public-safe-deterministic-unreal-import-manifest.mdc](docs/todo/open/unreal/generate-a-public-safe-deterministic-unreal-import-manifest.mdc)

### TODO - Apply conversion plans through tested native Unreal MCP commands

Apply conversion plans through tested native Unreal MCP commands.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/apply-conversion-plans-through-tested-native-unreal-mcp-commands.mdc](docs/todo/open/unreal/apply-conversion-plans-through-tested-native-unreal-mcp-commands.mdc)

### TODO - Implement and execute the complete serialized package transaction loop for every…

Implement and execute the complete serialized package transaction loop for
every operation family.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/implement-and-execute-the-complete-serialized-package-transaction-loop-for-every.mdc](docs/todo/open/unreal/implement-and-execute-the-complete-serialized-package-transaction-loop-for-every.mdc)

### TODO - Import meshes, skeletons, physics assets, and animations correctly

Import meshes, skeletons, physics assets, and animations correctly.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/import-meshes-skeletons-physics-assets-and-animations-correctly.mdc](docs/todo/open/unreal/import-meshes-skeletons-physics-assets-and-animations-correctly.mdc)

### TODO - Recreate materials only as required to match the original presentation

Recreate materials only as required to match the original presentation.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/recreate-materials-only-as-required-to-match-the-original-presentation.mdc](docs/todo/open/unreal/recreate-materials-only-as-required-to-match-the-original-presentation.mdc)

### TODO - Import original textures without repainting, upscaling, or redesigning them

Import original textures without repainting, upscaling, or redesigning them.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/import-original-textures-without-repainting-upscaling-or-redesigning-them.mdc](docs/todo/open/unreal/import-original-textures-without-repainting-upscaling-or-redesigning-them.mdc)

### TODO - Convert original camera, mission, vehicle, gameplay, UI, and tuning data into native…

Convert original camera, mission, vehicle, gameplay, UI, and tuning data into
native Unreal assets.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/convert-original-camera-mission-vehicle-gameplay-ui-and-tuning-data-into-native.mdc](docs/todo/open/unreal/convert-original-camera-mission-vehicle-gameplay-ui-and-tuning-data-into-native.mdc)

### TODO - Compile normalized mission-script bundles into typed `SharMission` definitions and…

Compile normalized mission-script bundles into typed `SharMission` definitions
and bindings for the shared mission StateTree contract.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/compile-normalized-mission-script-bundles-into-typed-sharmission-definitions-and.mdc](docs/todo/open/unreal/compile-normalized-mission-script-bundles-into-typed-sharmission-definitions-and.mdc)

### TODO - Map every reviewed participant, route, timing, load, checkpoint, presentation, reward,…

Map every reviewed participant, route, timing, load, checkpoint, presentation,
reward, transition, and typed objective/condition parameter reference.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/map-every-reviewed-participant-route-timing-load-checkpoint-presentation-reward.mdc](docs/todo/open/unreal/map-every-reviewed-participant-route-timing-load-checkpoint-presentation-reward.mdc)

### TODO - Resolve typed source identities and intentionally opaque values to canonical…

Resolve typed source identities and intentionally opaque values to canonical
participant, route, camera, reward, presentation, transition, and catalog
definitions before asset emission.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/resolve-typed-source-identities-and-intentionally-opaque-values-to-canonical.mdc](docs/todo/open/unreal/resolve-typed-source-identities-and-intentionally-opaque-values-to-canonical.mdc)

### TODO - Emit lossless `USharMissionDefinition` assets only after the complete mission graph…

Emit lossless `USharMissionDefinition` assets only after the complete mission
graph passes reference and topology validation.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/emit-lossless-usharmissiondefinition-assets-only-after-the-complete-mission-graph.mdc](docs/todo/open/unreal/emit-lossless-usharmissiondefinition-assets-only-after-the-complete-mission-graph.mdc)

### TODO - Compile remaining normalized UI, font, localization, tuning, and other structured…

Compile remaining normalized UI, font, localization, tuning, and other
structured evidence into concrete Unreal types before enabling their editor
factories.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/compile-remaining-normalized-ui-font-localization-tuning-and-other-structured.mdc](docs/todo/open/unreal/compile-remaining-normalized-ui-font-localization-tuning-and-other-structured.mdc)

### TODO - Convert original audio, cinematics, and localization into native Unreal assets

Convert original audio, cinematics, and localization into native Unreal
assets.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/unreal/convert-original-audio-cinematics-and-localization-into-native-unreal-assets.mdc](docs/todo/open/unreal/convert-original-audio-cinematics-and-localization-into-native-unreal-assets.mdc)

### TODO - Preserve source world placement through Unreal streaming and partitioning without…

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

## P3 — Faithful runtime

### TODO - Complete startup, saves, profiles, settings, loading, and progression

Complete startup, saves, profiles, settings, loading, and progression.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/complete-startup-saves-profiles-settings-loading-and-progression.mdc](docs/todo/open/runtime/complete-startup-saves-profiles-settings-loading-and-progression.mdc)

### TODO - Reproduce original player movement, cameras, interactions, vehicles, traffic,…

Reproduce original player movement, cameras, interactions, vehicles, traffic,
pedestrians, damage, and recovery.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/reproduce-original-player-movement-cameras-interactions-vehicles-traffic.mdc](docs/todo/open/runtime/reproduce-original-player-movement-cameras-interactions-vehicles-traffic.mdc)

### TODO - Reproduce original missions, objectives, triggers, dialogue, rewards, collectibles,…

Reproduce original missions, objectives, triggers, dialogue, rewards,
collectibles, races, and progression gates.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/reproduce-original-missions-objectives-triggers-dialogue-rewards-collectibles.mdc](docs/todo/open/runtime/reproduce-original-missions-objectives-triggers-dialogue-rewards-collectibles.mdc)

### TODO - Reproduce original HUD, menus, navigation, subtitles, audio, cinematics, and…

Reproduce original HUD, menus, navigation, subtitles, audio, cinematics, and
localization behavior.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/reproduce-original-hud-menus-navigation-subtitles-audio-cinematics-and.mdc](docs/todo/open/runtime/reproduce-original-hud-menus-navigation-subtitles-audio-cinematics-and.mdc)

### TODO - Reproduce original world streaming, placement, physics, animation, effects, and platform…

Reproduce original world streaming, placement, physics, animation, effects,
and platform input behavior.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/reproduce-original-world-streaming-placement-physics-animation-effects-and-platform.mdc](docs/todo/open/runtime/reproduce-original-world-streaming-placement-physics-animation-effects-and-platform.mdc)

### TODO - Preserve original mission timing, gameplay rules, and progression unless a technical…

Preserve original mission timing, gameplay rules, and progression unless a
technical compatibility fix is required.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/preserve-original-mission-timing-gameplay-rules-and-progression-unless-a-technical.mdc](docs/todo/open/runtime/preserve-original-mission-timing-gameplay-rules-and-progression-unless-a-technical.mdc)

### TODO - Bind generated assets through stable contracts instead of direct paths

Bind generated assets through stable contracts instead of direct paths.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/bind-generated-assets-through-stable-contracts-instead-of-direct-paths.mdc](docs/todo/open/runtime/bind-generated-assets-through-stable-contracts-instead-of-direct-paths.mdc)

### TODO - Add parity tests for gameplay behavior and state transitions

Add parity tests for gameplay behavior and state transitions.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/runtime/add-parity-tests-for-gameplay-behavior-and-state-transitions.mdc](docs/todo/open/runtime/add-parity-tests-for-gameplay-behavior-and-state-transitions.mdc)

## P4 — Build, packaging, mods, and product surface

### TODO - Hermetic build dependency bootstrap

Implement `tools/build/dependencies.py` with CPython 3.14.6 as the declared
bootstrap and install public project dependencies into repository-owned pinned
locations without mutating global packages; validate proprietary or platform
toolchains as explicit external prerequisites instead of silently using
arbitrary host state.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/hermetic-build-dependency-bootstrap.mdc](docs/todo/open/packaging/hermetic-build-dependency-bootstrap.mdc)

### TODO - Build preflight and saved check evidence

Implement `tools/build/check.py` as the supported preflight: validate the
lawful game installation and manifest, require `game/Simpsons.exe` directly
under the repository game root, verify Python 3.14.6, Unreal Engine 5.8.1, and
host prerequisites, then write `.cache/build/data/check.json` for later build
steps.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/build-preflight-and-saved-check-evidence.mdc](docs/todo/open/packaging/build-preflight-and-saved-check-evidence.mdc)

### TODO - Canonical multi-platform build runner

Implement `tools/build/run.py` to consume saved preflight and architecture
decisions, revalidate them, build every selected target transactionally, and
publish only the minimal native deliverable under `dist/<ARCH>/` for each
successful architecture.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/canonical-multi-platform-build-runner.mdc](docs/todo/open/packaging/canonical-multi-platform-build-runner.mdc)

### TODO - One-command build orchestration

Implement `tools/build/auto.py` as an optional one-command user flow that runs
dependency bootstrap, preflight, architecture selection, and build in order
while persisting each decision JSON so repeated or interrupted runs use
explicit evidence instead of hidden process state.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/one-command-build-orchestration.mdc](docs/todo/open/packaging/one-command-build-orchestration.mdc)

### TODO - Canonical generated workspace under .cache

Move regenerable extraction, FBX conversion, Unreal staging, build
intermediates, logs, and decision data out of the repository root into one
documented `.cache/` hierarchy; keep `game/` as user-supplied source evidence
and `dist/` as final copied output instead of introducing an ambiguous
generated `assets/` root.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/canonical-generated-workspace-under-cache.mdc](docs/todo/open/packaging/canonical-generated-workspace-under-cache.mdc)

### TODO - Package Linux, macOS, Android, Windows x86-64, and any Windows-on-ARM target that…

Package Linux, macOS, Android, Windows x86-64, and any Windows-on-ARM target
that current Unreal/toolchain support can validate.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/package-linux-macos-android-windows-x86-64-and-any-windows-on-arm-target-that.mdc](docs/todo/open/packaging/package-linux-macos-android-windows-x86-64-and-any-windows-on-arm-target-that.mdc)

### TODO - Produce a local iOS `.ipa` package for sideloading/testing without App Store submission…

Produce a local iOS `.ipa` package for sideloading/testing without App Store
submission or an Xcode-dependent authoring workflow; document any unavoidable
Apple signing or build-host constraints explicitly.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/produce-a-local-ios-ipa-package-for-sideloading-testing-without-app-store-submission.mdc](docs/todo/open/packaging/produce-a-local-ios-ipa-package-for-sideloading-testing-without-app-store-submission.mdc)

### TODO - Require at least one canonical `.ico` under `game/` in the game manifest; zero icons is…

Require at least one canonical `.ico` under `game/` in the game manifest; zero
icons is invalid. Unreal staging must consume the generated icon outputs after
the current icon producer is moved to its final repository-owned location,
expected under `src/unreal/icon`.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/require-at-least-one-canonical-ico-under-game-in-the-game-manifest-zero-icons-is.mdc](docs/todo/open/packaging/require-at-least-one-canonical-ico-under-game-in-the-game-manifest-zero-icons-is.mdc)

### TODO - Define deterministic mod identity, dependencies, priority, compatibility, supersession,…

Define deterministic mod identity, dependencies, priority, compatibility,
supersession, and conflict rules.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/define-deterministic-mod-identity-dependencies-priority-compatibility-supersession.mdc](docs/todo/open/mods/define-deterministic-mod-identity-dependencies-priority-compatibility-supersession.mdc)

### TODO - Support validated replacement and extension packages for assets and data

Support validated replacement and extension packages for assets and data.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/support-validated-replacement-and-extension-packages-for-assets-and-data.mdc](docs/todo/open/mods/support-validated-replacement-and-extension-packages-for-assets-and-data.mdc)

### TODO - Keep the unmodified faithful port as the default base-game package

Keep the unmodified faithful port as the default base-game package.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/keep-the-unmodified-faithful-port-as-the-default-base-game-package.mdc](docs/todo/open/mods/keep-the-unmodified-faithful-port-as-the-default-base-game-package.mdc)

### TODO - Use one normalized desktop and Android mod import contract

Use one normalized desktop and Android mod import contract.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/use-one-normalized-desktop-and-android-mod-import-contract.mdc](docs/todo/open/mods/use-one-normalized-desktop-and-android-mod-import-contract.mdc)

### TODO - Keep native-code mods behind an explicit trust boundary

Keep native-code mods behind an explicit trust boundary.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/keep-native-code-mods-behind-an-explicit-trust-boundary.mdc](docs/todo/open/mods/keep-native-code-mods-behind-an-explicit-trust-boundary.mdc)

### TODO - Validate schemas, paths, integrity, limits, references, and load order

Validate schemas, paths, integrity, limits, references, and load order.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/validate-schemas-paths-integrity-limits-references-and-load-order.mdc](docs/todo/open/mods/validate-schemas-paths-integrity-limits-references-and-load-order.mdc)

### TODO - Finish user-facing and AI-agent modding skills

Finish user-facing and AI-agent modding skills.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/mods/finish-user-facing-and-ai-agent-modding-skills.mdc](docs/todo/open/mods/finish-user-facing-and-ai-agent-modding-skills.mdc)

### TODO - Package and launch the selected native desktop and Android targets

Package and launch the selected native desktop and Android targets.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/package-and-launch-the-selected-native-desktop-and-android-targets.mdc](docs/todo/open/packaging/package-and-launch-the-selected-native-desktop-and-android-targets.mdc)

### TODO - Require packaged-build evidence instead of editor play or emulation

Require packaged-build evidence instead of editor play or emulation.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/require-packaged-build-evidence-instead-of-editor-play-or-emulation.mdc](docs/todo/open/packaging/require-packaged-build-evidence-instead-of-editor-play-or-emulation.mdc)

### TODO - Keep gameplay, saves, package identities, and mod contracts consistent across supported…

Keep gameplay, saves, package identities, and mod contracts consistent across
supported targets.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/keep-gameplay-saves-package-identities-and-mod-contracts-consistent-across-supported.mdc](docs/todo/open/packaging/keep-gameplay-saves-package-identities-and-mod-contracts-consistent-across-supported.mdc)

### TODO - Provide graphics and performance settings without changing base content

Provide graphics and performance settings without changing base content.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/provide-graphics-and-performance-settings-without-changing-base-content.mdc](docs/todo/open/packaging/provide-graphics-and-performance-settings-without-changing-base-content.mdc)

### TODO - Profile CPU, GPU, memory, storage, streaming, shaders, loading, and frame time

Profile CPU, GPU, memory, storage, streaming, shaders, loading, and frame
time.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/packaging/profile-cpu-gpu-memory-storage-streaming-shaders-loading-and-frame-time.mdc](docs/todo/open/packaging/profile-cpu-gpu-memory-storage-streaming-shaders-loading-and-frame-time.mdc)

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

### TODO - Ship four independent base save slots; add autosave only after faithful port parity is…

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

### TODO - Ship a default, fully obtainable base achievement set suitable for 100%/platinum-style…

Ship a default, fully obtainable base achievement set suitable for
100%/platinum-style completion, with mod achievements separately namespaced.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/ship-a-default-fully-obtainable-base-achievement-set-suitable-for-100-platinum-style.mdc](docs/todo/open/product/ship-a-default-fully-obtainable-base-achievement-set-suitable-for-100-platinum-style.mdc)

### TODO - Add an optional Discord boundary for display username, Rich Presence, parties/invites,…

Add an optional Discord boundary for display username, Rich Presence,
parties/invites, and achievement-facing presentation without making Discord
identity authoritative for saves or gameplay.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/add-an-optional-discord-boundary-for-display-username-rich-presence-parties-invites.mdc](docs/todo/open/product/add-an-optional-discord-boundary-for-display-username-rich-presence-parties-invites.mdc)

### TODO - Expose base C++ extension points for boss encounters, health meters, combat, multiplayer…

Expose base C++ extension points for boss encounters, health meters, combat,
multiplayer adapters, and future mod-owned gameplay systems without
implementing replacement gameplay during the faithful-port phase.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/expose-base-c-extension-points-for-boss-encounters-health-meters-combat-multiplayer.mdc](docs/todo/open/product/expose-base-c-extension-points-for-boss-encounters-health-meters-combat-multiplayer.mdc)

### TODO - Add a first-class `Mods` route when creating or loading a game

Add a first-class `Mods` route when creating or loading a game.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/add-a-first-class-mods-route-when-creating-or-loading-a-game.mdc](docs/todo/open/product/add-a-first-class-mods-route-when-creating-or-loading-a-game.mdc)

### TODO - Classify mods as visual-only, additive/story, gameplay-extension, or native-code, with…

Classify mods as visual-only, additive/story, gameplay-extension, or
native-code, with explicit compatibility and save-impact declarations.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/classify-mods-as-visual-only-additive-story-gameplay-extension-or-native-code-with.mdc](docs/todo/open/product/classify-mods-as-visual-only-additive-story-gameplay-extension-or-native-code-with.mdc)

### TODO - Allow visual-only mods to be reordered, enabled, disabled, or replaced from the main…

Allow visual-only mods to be reordered, enabled, disabled, or replaced from
the main menu without invalidating the active save.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/allow-visual-only-mods-to-be-reordered-enabled-disabled-or-replaced-from-the-main.mdc](docs/todo/open/product/allow-visual-only-mods-to-be-reordered-enabled-disabled-or-replaced-from-the-main.mdc)

### TODO - Treat story/additive mods as mutually incompatible by default unless their manifests…

Treat story/additive mods as mutually incompatible by default unless their
manifests explicitly declare a compatible composition contract.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/treat-story-additive-mods-as-mutually-incompatible-by-default-unless-their-manifests.mdc](docs/todo/open/product/treat-story-additive-mods-as-mutually-incompatible-by-default-unless-their-manifests.mdc)

### TODO - Permit nonvisual mods that do not mutate save/progression state, but show an explicit…

Permit nonvisual mods that do not mutate save/progression state, but show an
explicit compatibility warning before activation.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/permit-nonvisual-mods-that-do-not-mutate-save-progression-state-but-show-an-explicit.mdc](docs/todo/open/product/permit-nonvisual-mods-that-do-not-mutate-save-progression-state-but-show-an-explicit.mdc)

### TODO - Let mods declare deterministic hierarchy, load order, and override priority; unresolved…

Let mods declare deterministic hierarchy, load order, and override priority;
unresolved conflicts fail closed.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/let-mods-declare-deterministic-hierarchy-load-order-and-override-priority-unresolved.mdc](docs/todo/open/product/let-mods-declare-deterministic-hierarchy-load-order-and-override-priority-unresolved.mdc)

### TODO - Keep native C++ mods behind a trust scanner that reports filesystem, process, network,…

Keep native C++ mods behind a trust scanner that reports filesystem, process,
network, platform, save, and engine-surface access before the user decides
whether to load them.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/keep-native-c-mods-behind-a-trust-scanner-that-reports-filesystem-process-network.mdc](docs/todo/open/product/keep-native-c-mods-behind-a-trust-scanner-that-reports-filesystem-process-network.mdc)

### TODO - Make mission, model, material, texture, skeleton, coordinate, gameplay, achievement, and…

Make mission, model, material, texture, skeleton, coordinate, gameplay,
achievement, and UI definitions data-addressable for nonexpert mod authors.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/make-mission-model-material-texture-skeleton-coordinate-gameplay-achievement-and.mdc](docs/todo/open/product/make-mission-model-material-texture-skeleton-coordinate-gameplay-achievement-and.mdc)

### TODO - Deduplicate identical generated skeletons, textures, models, and other assets only when…

Deduplicate identical generated skeletons, textures, models, and other assets
only when deterministic evidence proves equivalence. Never assume all
characters or mods share one skeleton or asset layout.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/deduplicate-identical-generated-skeletons-textures-models-and-other-assets-only-when.mdc](docs/todo/open/product/deduplicate-identical-generated-skeletons-textures-models-and-other-assets-only-when.mdc)

### TODO - Let generated base characters opt into shared skeleton/material/model assets as an…

Let generated base characters opt into shared skeleton/material/model assets
as an optimization, with simple per-asset/per-mod overrides that can break
sharing without changing global contracts.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/let-generated-base-characters-opt-into-shared-skeleton-material-model-assets-as-an.mdc](docs/todo/open/product/let-generated-base-characters-opt-into-shared-skeleton-material-model-assets-as-an.mdc)

### TODO - Keep multiplayer as an extension-ready base capability rather than a fully implemented…

Keep multiplayer as an extension-ready base capability rather than a fully
implemented first-party mode during the port; mods must be able to add
replicated modes, lobbies, servers, missions, and progression namespaces.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/keep-multiplayer-as-an-extension-ready-base-capability-rather-than-a-fully-implemented.mdc](docs/todo/open/product/keep-multiplayer-as-an-extension-ready-base-capability-rather-than-a-fully-implemented.mdc)

### TODO - Keep DLSS and hardware-ray-tracing/RTX integrations behind optional capability adapters…

Keep DLSS and hardware-ray-tracing/RTX integrations behind optional capability
adapters so graphics mods can target them without making proprietary GPU
features mandatory for the base port.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/keep-dlss-and-hardware-ray-tracing-rtx-integrations-behind-optional-capability-adapters.mdc](docs/todo/open/product/keep-dlss-and-hardware-ray-tracing-rtx-integrations-behind-optional-capability-adapters.mdc)

### TODO - Preserve the delivery order: faithful port first, compatibility and quality-of-life…

Preserve the delivery order: faithful port first, compatibility and
quality-of-life improvements second, richer community mod content third.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/product/preserve-the-delivery-order-faithful-port-first-compatibility-and-quality-of-life.mdc](docs/todo/open/product/preserve-the-delivery-order-faithful-port-first-compatibility-and-quality-of-life.mdc)

## P5 — Final verification and publication

### TODO - Complete a start-to-finish playthrough without progression-blocking defects

Complete a start-to-finish playthrough without progression-blocking defects.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/complete-a-start-to-finish-playthrough-without-progression-blocking-defects.mdc](docs/todo/open/verification/complete-a-start-to-finish-playthrough-without-progression-blocking-defects.mdc)

### TODO - Compare missions, vehicles, collectibles, saves, localization, cinematics, world layout,…

Compare missions, vehicles, collectibles, saves, localization, cinematics,
world layout, and the ending against the original game.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/compare-missions-vehicles-collectibles-saves-localization-cinematics-world-layout.mdc](docs/todo/open/verification/compare-missions-vehicles-collectibles-saves-localization-cinematics-world-layout.mdc)

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

### TODO - Verify an AI agent can create and validate a mod using published skills

Verify an AI agent can create and validate a mod using published skills.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/verify-an-ai-agent-can-create-and-validate-a-mod-using-published-skills.mdc](docs/todo/open/verification/verify-an-ai-agent-can-create-and-validate-a-mod-using-published-skills.mdc)

### TODO - Record known compatibility limitations honestly

Record known compatibility limitations honestly.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/record-known-compatibility-limitations-honestly.mdc](docs/todo/open/verification/record-known-compatibility-limitations-honestly.mdc)

### TODO - Run the canonical global validation without cache

Run the canonical global validation without cache.

<!-- MarkdownLint-disable-next-line MD013 MD044 -->
[docs/todo/open/verification/run-the-canonical-global-validation-without-cache.mdc](docs/todo/open/verification/run-the-canonical-global-validation-without-cache.mdc)

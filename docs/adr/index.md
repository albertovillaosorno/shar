# Architecture decision record index

See the [SHAR Documentation Guide](../README.md) for documentation ownership,
maintenance, validation, and public-content rules.

This catalog lists current repository decisions. It does not preserve obsolete
filenames for compatibility. A document that explains implementation rather than
choosing a durable boundary belongs in the technical catalog, and every
repository reference must point to the current owning decision.

## Audio

- [Latin American Spanish audio fallback](audio/lmlm-spanish-latam-audio-fallback.md)
  — Accepted
- [Platform-native audio cooking and streaming](audio/platform-native-audio-cooking-and-streaming.md)
  — Accepted

## Cinematics

- [Local cinematic overrides](rmv/local-movie-overrides.md) — Accepted
- [Native cinematic package strategy](rmv/unreal-native-cinematic-package.md) —
  Accepted

## Engineering

- [Minimal hexagonal architecture](engineering/architecture/minimal-hexagonal-architecture.md)
  — Accepted
- [Portable core separation](engineering/architecture/project-core-separation.md)
  — Accepted
- [Single repository validation authority](engineering/quality/local-validation-configs.md)
  — Accepted
- [Repository quality policy](engineering/quality/repository-quality-policy.md)
  — Accepted
- [Strict validation and linting](engineering/quality/strict-validation-and-linting.md)
  — Accepted

## FBX conversion

- [Unsupported model evidence preservation](fbx/chunks/chunk-preservation-policy.md)
  — Accepted
- [First-principles FBX output contract](fbx/export/fbx-output-contract-boundary.md)
  — Accepted
- [Package evidence discovery boundary](fbx/extraction/source-discovery-boundary.md)
  — Accepted

## Gameplay

- [Collector cards, coins, rewards, gags, and wasps](gameplay/collectibles/collectibles-rewards-gags-and-wasps.md)
  — Accepted
- [Driving, traffic, and vehicle behavior parity](gameplay/vehicles/driving-traffic-and-vehicle-ai.md)
  — Accepted

## Governance

- [AI-first repository communication](governance/ai-first-repository.md) —
  Accepted
- [Decision and technical knowledge boundaries](governance/documentation-and-knowledge-boundaries.md)
  — Accepted
- [Issue-only collaboration](governance/issue-only-collaboration.md) — Accepted
- [Public automation and maintenance boundary](governance/public-automation-and-maintenance-boundary.md)
  — Accepted
- [Calendar versioning, Conventional Commits, and no releases](governance/versioning-commits-and-publication.md)
  — Accepted

## Legal and publication

- [Lawful local input and publication boundary](legal/lawful-local-input-and-publication-boundary.md)
  — Accepted

## Modding

- [Local drop-in mod packages and AI skills](modding/drop-in-mod-packages-and-ai-skills.md)
  — Accepted
- [Local mod trust and distribution boundary](modding/mod-safety-scanner-and-distribution.md)
  — Accepted
- [Voice and language mod packages](modding/voice-language-modding-suite.md) —
  Accepted

## Pipeline

- [Eleven-phase remake delivery roadmap](pipeline/eleven-phase-remake-delivery-roadmap.md)
  — Accepted
- [Extraction provenance and manifest linkage](pipeline/extraction/extraction-provenance-and-manifest-linkage.md)
  — Accepted
- [Lossless fail-closed extraction](pipeline/extraction/lossless-extraction-contract.md)
  — Accepted
- [Hexagonal scene export](pipeline/fbx/hexagonal-scene-export.md) — Accepted
- [Game manifest as a completeness ledger](pipeline/game-manifest-ledger.md) —
  Accepted
- [Minor-unit taxonomy and package value](pipeline/minor-unit-taxonomy-value-case.md)
  — Accepted
- [Orchestration, command-line, and language boundaries](pipeline/orchestration-cli-and-language-boundaries.md)
  — Accepted
- [Retire legacy conversion bridges](pipeline/retire-legacy-language-and-p3d-fbx-bridges.md)
  — Accepted
- [Background vista and occluder relocation](pipeline/unreal/background-vista-and-occluder-relocation.md)
  — Accepted
- [Native asset translation without copy-paste](pipeline/unreal/native-asset-translation-and-no-copy-paste.md)
  — Accepted
- [Faithful seven-level world set](pipeline/unreal/faithful-seven-level-world-set.md)
  — Accepted
- [World assembly from normalized chunks](pipeline/unreal/world-assembly-from-normalized-chunks.md)
  — Accepted
- [Faithful material normalization](pipeline/unreal/texture-superposition-shader.md)
  — Accepted
- [Three-base-world consolidation](pipeline/unreal/three-base-world-consolidation.md)
  — Accepted
- [Unreal manifest and package taxonomy](pipeline/unreal/unreal-manifest-and-package-taxonomy.md)
  — Accepted
- [Native world partition and data layers](pipeline/unreal/world-partition-and-data-layer-import.md)
  — Accepted

## Unreal

- [Unreal asset-conversion boundary](unreal/architecture.md) — Accepted
- [Minimal hexagonal native runtime](unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
  — Accepted
- [Blueprint-to-C++ authority taxonomy](unreal/authoring/blueprint-to-cpp-taxonomy.md)
  — Accepted
- [Deterministic editor change replay](unreal/authoring/change-replay-taxonomy.md)
  — Accepted
- [UI and level authoring taxonomy](unreal/authoring/ui-and-level-authoring-taxonomy.md)
  — Accepted
- [Converted asset ingestion boundary](unreal/import-adapters/converted-asset-ingestion-boundary.md)
  — Accepted
- [Import review boundary](unreal/import-adapters/import-review-boundary.md) —
  Accepted
- [Native MCP tool projection and protected skill guidance](unreal/mcp/native-tool-cli-projection-and-skills.md)
  — Accepted
- [Native Unreal MCP terminal bridge](unreal/mcp/native-unreal-mcp-terminal-bridge.md)
  — Accepted
- [Upstream native plugin and additive extension boundary](unreal/mcp/upstream-native-plugin-and-additive-extension-boundary.md)
  — Accepted
- [C++-primary and Blueprint-compatible Unreal project](unreal/project/cpp-primary-blueprint-compatible-project.md)
  — Accepted
- [Canonical seven-level campaign and world variants](unreal/runtime/canonical-seven-level-campaign-and-world-variants.md)
  — Accepted
- [Data-driven Unreal gameplay content catalog](unreal/runtime/data-driven-gameplay-content-catalog.md)
  — Accepted
- [Contextual interaction query and transaction boundary](unreal/runtime/contextual-interaction-query-and-transaction.md)
  — Accepted
- [Event-driven music and ambience](unreal/runtime/event-driven-music-and-ambience.md)
  — Accepted
- [Mass Entity ambient population](unreal/runtime/mass-entity-ambient-population.md)
  — Accepted
- [State-driven missions, interactions, interiors, and notoriety](unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
  — Accepted
- [Portable save storage and lifecycle](unreal/runtime/portable-save-storage-and-lifecycle.md)
  — Accepted
- [Runtime parity boundary](unreal/runtime/remake-parity-boundary.md) — Accepted
- [Graphics quality presets and platform support](unreal/runtime/graphics-quality-presets-and-platform-support.md)
  — Accepted
- [Shared runtime tagging, modding, and platform compatibility](unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
  — Accepted
- [Transactional phone-booth vehicle retrieval](unreal/runtime/transactional-phone-booth-vehicle-retrieval.md)
  — Accepted
- [Validated game-feature mod overlays](unreal/runtime/validated-game-feature-mod-overlays.md)
  — Accepted
- [Runtime parity test boundary](unreal/runtime/runtime-parity-test-boundary.md)
  — Accepted
- [Unreal support and bridge boundaries](unreal/support-and-bridge-boundaries.md)
  — Accepted
- [Common UI front end and progress projection](unreal/ui/common-ui-frontend-and-progress-projection.md)
  — Accepted
- [HUD, radar, camera, and navigation parity](unreal/ui/hud-radar-camera-and-navigation.md)
  — Accepted
- [UI parity boundary](unreal/ui/ui-parity-boundary.md) — Accepted

## Current coverage

- Accepted decision records: 66.
- Template records: 1.
- Review date: 2026-07-14.
- Status boundary: this catalog contains only accepted repository decisions;
  proposed work is not represented as current authority.

## Authoring rules

- Use the [ADR template](template.md).
- Record one durable repository-impacting decision per document.
- Keep implementation explanations in the technical catalog.
- Consolidate duplicate decisions and update every repository reference.
- Do not retain a placeholder ADR merely to preserve an old filename.
- Put external evidence in bibliography, research, or legal records.

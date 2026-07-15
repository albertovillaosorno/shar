# Architecture decision record index

See the [SHAR Documentation Guide](../README.md) for documentation ownership,
maintenance, validation, and public-content rules.

This catalog lists current repository decisions. It does not preserve obsolete
filenames for compatibility. A document that explains implementation rather than
choosing a durable boundary belongs in the technical catalog, and every
repository reference must point to the current owning decision.

## Audio

<!-- markdownlint-disable-next-line MD013 -->
- [Latin American Spanish audio fallback](audio/lmlm-spanish-latam-audio-fallback.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Platform-native audio cooking and streaming](audio/platform-native-audio-cooking-and-streaming.md)
  — Accepted

## Cinematics

- [Local cinematic overrides](rmv/local-movie-overrides.md) — Accepted
- [Native cinematic package strategy](rmv/unreal-native-cinematic-package.md) —
  Accepted

## Engineering

<!-- markdownlint-disable-next-line MD013 -->
- [Minimal hexagonal architecture](engineering/architecture/minimal-hexagonal-architecture.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Portable core separation](engineering/architecture/project-core-separation.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Single repository validation authority](engineering/quality/local-validation-configs.md)
  — Accepted
- [Repository quality policy](engineering/quality/repository-quality-policy.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Strict validation and linting](engineering/quality/strict-validation-and-linting.md)
  — Accepted

## FBX conversion

<!-- markdownlint-disable-next-line MD013 -->
- [Unsupported model evidence preservation](fbx/chunks/chunk-preservation-policy.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Character semantic texture, rig, outfit, and prop contract](fbx/export/character-semantic-texture-rig-and-outfit-contract.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [First-principles FBX output contract](fbx/export/fbx-output-contract-boundary.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Semantic component and geographic placement contract](fbx/export/semantic-component-and-geographic-placement-contract.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Package evidence discovery boundary](fbx/extraction/source-discovery-boundary.md)
  — Accepted

## Gameplay

<!-- markdownlint-disable-next-line MD013 -->
- [Collector cards, coins, rewards, gags, and wasps](gameplay/collectibles/collectibles-rewards-gags-and-wasps.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapters and world progression](gameplay/open-sandbox-chapters-and-world-progression.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Driving, traffic, and vehicle behavior parity](gameplay/vehicles/driving-traffic-and-vehicle-ai.md)
  — Accepted

## Governance

- [AI-first repository communication](governance/ai-first-repository.md) —
  Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Decision and technical knowledge boundaries](governance/documentation-and-knowledge-boundaries.md)
  — Accepted
- [Issue-only collaboration](governance/issue-only-collaboration.md) — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Public automation and maintenance boundary](governance/public-automation-and-maintenance-boundary.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Calendar versioning, Conventional Commits, and no releases](governance/versioning-commits-and-publication.md)
  — Accepted

## Legal and publication

<!-- markdownlint-disable-next-line MD013 -->
- [Lawful local input and publication boundary](legal/lawful-local-input-and-publication-boundary.md)
  — Accepted

## Modding

<!-- markdownlint-disable-next-line MD013 -->
- [Local drop-in mod packages and AI skills](modding/drop-in-mod-packages-and-ai-skills.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Local mod trust and distribution boundary](modding/mod-safety-scanner-and-distribution.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Mod-owned multiplayer adapters and community servers](modding/mod-owned-multiplayer-adapters-and-community-servers.md)
  — Accepted
- [Voice and language mod packages](modding/voice-language-modding-suite.md) —
  Accepted

## Pipeline

<!-- markdownlint-disable-next-line MD013 -->
- [Eleven-phase remake delivery roadmap](pipeline/eleven-phase-remake-delivery-roadmap.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Extraction provenance and manifest linkage](pipeline/extraction/extraction-provenance-and-manifest-linkage.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Lossless fail-closed extraction](pipeline/extraction/lossless-extraction-contract.md)
  — Accepted
- [Hexagonal scene export](pipeline/fbx/hexagonal-scene-export.md) — Accepted
- [Game manifest as a completeness ledger](pipeline/game-manifest-ledger.md) —
  Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Minor-unit taxonomy and package value](pipeline/minor-unit-taxonomy-value-case.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Orchestration, command-line, and language boundaries](pipeline/orchestration-cli-and-language-boundaries.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Retire legacy conversion bridges](pipeline/retire-legacy-language-and-p3d-fbx-bridges.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Background vista and occluder relocation](pipeline/unreal/background-vista-and-occluder-relocation.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset translation without copy-paste](pipeline/unreal/native-asset-translation-and-no-copy-paste.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Faithful seven-chapter open-world scope](pipeline/unreal/faithful-seven-chapter-open-world-scope.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [World assembly from normalized chunks](pipeline/unreal/world-assembly-from-normalized-chunks.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Faithful material normalization](pipeline/unreal/texture-superposition-shader.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Unified open world and chapter projection](pipeline/unreal/unified-open-world-and-chapter-projection.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Unreal manifest and package taxonomy](pipeline/unreal/unreal-manifest-and-package-taxonomy.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Native world partition and data layers](pipeline/unreal/world-partition-and-data-layer-import.md)
  — Accepted

## Unreal

- [Unreal asset-conversion boundary](unreal/architecture.md) — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Minimal hexagonal native runtime](unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Blueprint-to-C++ authority taxonomy](unreal/authoring/blueprint-to-cpp-taxonomy.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Deterministic editor change replay](unreal/authoring/change-replay-taxonomy.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [UI and level authoring taxonomy](unreal/authoring/ui-and-level-authoring-taxonomy.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Converted asset ingestion boundary](unreal/import-adapters/converted-asset-ingestion-boundary.md)
  — Accepted
- [Import review boundary](unreal/import-adapters/import-review-boundary.md) —
  Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Native MCP tool projection and protected skill guidance](unreal/mcp/native-tool-cli-projection-and-skills.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Native Unreal MCP terminal bridge](unreal/mcp/native-unreal-mcp-terminal-bridge.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Upstream native plugin and additive extension boundary](unreal/mcp/upstream-native-plugin-and-additive-extension-boundary.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [C++-primary and Blueprint-compatible Unreal project](unreal/project/cpp-primary-blueprint-compatible-project.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Data-driven Unreal gameplay content catalog](unreal/runtime/data-driven-gameplay-content-catalog.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Contextual interaction query and transaction boundary](unreal/runtime/contextual-interaction-query-and-transaction.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Native flying-hazard actors and StateTree execution](unreal/runtime/native-flying-hazard-actors-and-state-trees.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Typed StateTree action sequences](unreal/runtime/typed-state-tree-action-sequences.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Event-driven music and ambience](unreal/runtime/event-driven-music-and-ambience.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Mass Entity ambient population](unreal/runtime/mass-entity-ambient-population.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [State-driven missions, interactions, interiors, and notoriety](unreal/runtime/state-driven-missions-interactions-and-notoriety.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](unreal/runtime/portable-save-storage-and-lifecycle.md)
  — Accepted
- [Runtime parity boundary](unreal/runtime/remake-parity-boundary.md) — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Graphics quality presets and platform support](unreal/runtime/graphics-quality-presets-and-platform-support.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Shared runtime tagging, modding, and platform compatibility](unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Transactional phone-booth vehicle retrieval](unreal/runtime/transactional-phone-booth-vehicle-retrieval.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Validated game-feature mod overlays](unreal/runtime/validated-game-feature-mod-overlays.md)
  — Accepted
- [Runtime parity test boundary](unreal/runtime/runtime-parity-test-boundary.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Unreal support and bridge boundaries](unreal/support-and-bridge-boundaries.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](unreal/ui/common-ui-frontend-and-progress-projection.md)
  — Accepted
<!-- markdownlint-disable-next-line MD013 -->
- [HUD, radar, camera, and navigation parity](unreal/ui/hud-radar-camera-and-navigation.md)
  — Accepted
- [UI parity boundary](unreal/ui/ui-parity-boundary.md) — Accepted

## Current coverage

- Accepted decision records: 68.
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

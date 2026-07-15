# Technical documentation index

See the [SHAR Documentation Guide](../README.md) for documentation ownership,
maintenance, validation, and public-content rules.

This catalog explains how repository-owned code works. Specifications do not
make architecture decisions, contain concrete repository paths, or explain
proprietary external formats.

## Architecture

- [Minimal hexagonal system](architecture/minimal-hexagonal-system.md)

## Collaboration

- [Issue intake](collaboration/issue-intake.md)

## Documentation

- [Repository knowledge model](documentation/repository-knowledge-model.md)

## FBX conversion

- [Character semantic preparation](fbx/character-semantic-preparation.md)
- [Animation clip timing](fbx/animation/clip-timing.md)
- [Animation rig model](fbx/animation/rig-model.md)
- [First-principles scene writer](fbx/first-principles-scene-writer.md)
- [Mesh primitive model](fbx/geometry/mesh-primitives.md)
- [Surface vectors and texture coordinates](fbx/geometry/surface-vectors.md)
- [Material assignment](fbx/materials/material-assignment.md)
- [Texture evidence](fbx/materials/texture-evidence.md)
- [Skin attachment](fbx/skeletons/skin-attachment.md)
- [Model output verification](fbx/validation/output-verification.md)

## Gameplay

- [Open sandbox campaign design](gameplay/open-sandbox-campaign-design.md)

## Localization

<!-- markdownlint-disable-next-line MD013 -->
- [Normalized language interchange](localization/normalized-language-interchange.md)

## Modding

- [Local mod package model](modding/local-package-model.md)
- [Mod package validation](modding/package-validation.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Multiplayer adapter and community-server extension](modding/multiplayer-adapter-and-community-server-extension.md)

## Pipeline

<!-- markdownlint-disable-next-line MD013 -->
- [Deterministic conversion pipeline](pipeline/deterministic-conversion-pipeline.md)
- [Evidence and identity model](pipeline/evidence-and-identity-model.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Character capability evidence](pipeline/extraction/character-capability-evidence.md)
- [Model geometry evidence](pipeline/extraction/model-geometry-evidence.md)
- [Rig and motion evidence](pipeline/extraction/rig-and-motion-evidence.md)
- [Scene assembly evidence](pipeline/extraction/scene-assembly-evidence.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Texture and summary evidence](pipeline/extraction/texture-and-summary-evidence.md)
<!-- markdownlint-disable-next-line MD013 -->
- [World intersection evidence](pipeline/extraction/world-intersection-evidence.md)
- [World simulation evidence](pipeline/extraction/world-simulation-evidence.md)

## Unreal

<!-- markdownlint-disable-next-line MD013 -->
- [Ambient population and named-character runtime](unreal/ambient-population-and-named-character-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](unreal/application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored spatial placement and trigger runtime](unreal/authored-spatial-placement-and-trigger-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Camera rig, preset, and arbitration runtime](unreal/camera-rig-preset-and-arbitration-runtime.md)
- [Camera system runtime](unreal/camera-system-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Open sandbox chapter runtime](unreal/open-sandbox-chapter-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Developer command and diagnostic runtime](unreal/developer-command-and-diagnostic-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Device configuration and save-slot runtime](unreal/device-configuration-and-save-slot-runtime.md)
- [Frontend shell and menu runtime](unreal/frontend-shell-and-menu-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Unreal configuration and asset validation](unreal/config-and-asset-validation.md)
- [Unreal gameplay content catalog](unreal/gameplay-content-catalog.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Legacy runtime identity normalization](unreal/legacy-runtime-identity-normalization.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Memory ownership, budget, and diagnostics runtime](unreal/memory-ownership-budget-and-diagnostics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission, interaction, interior, and notoriety runtime](unreal/mission-interaction-and-notoriety-runtime.md)
- [Mod package overlay runtime](unreal/mod-package-overlay-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Music state and transition runtime](unreal/music-state-and-transition-runtime.md)
- [Unreal MCP terminal translator](unreal/mcp-terminal-translator.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native asset load request and streaming runtime](unreal/native-asset-load-request-and-streaming-runtime.md)
- [Native asset planning](unreal/native-asset-planning.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native import, material rebuild, and world assembly](unreal/native-import-material-and-world-assembly.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native platform bootstrap and error-recovery runtime](unreal/native-platform-bootstrap-and-error-recovery-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Physical material and impact-response runtime](unreal/physical-material-and-impact-response-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Persistent world-object state runtime](unreal/persistent-world-object-state-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform audio cooking and streaming](unreal/platform-audio-cooking-and-streaming.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform cinematic media packaging](unreal/platform-cinematic-media-packaging.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform, quality, and optimization contract](unreal/platform-quality-and-optimization.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Platform save storage and lifecycle](unreal/platform-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Progression, collectibles, cheats, and credits](unreal/progression-collectibles-and-cheats.md)
- [Race route and opponent runtime](unreal/race-route-and-opponent-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Semantic input, device, and haptics runtime](unreal/semantic-input-device-and-haptics-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Typed event and observation routing runtime](unreal/typed-event-and-observation-routing-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle access and roster runtime](unreal/vehicle-access-and-roster-runtime.md)
- [Vehicle AI and route runtime](unreal/vehicle-ai-and-route-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Vehicle retrieval and phone-booth runtime](unreal/vehicle-retrieval-and-phone-booth-runtime.md)
- [Editor guardrails](unreal/testing/editor-guardrails.md)
- [Unreal test taxonomy](unreal/testing/test-taxonomy.md)

## Validation

- [Deterministic validation](validation/deterministic-validation.md)

## Versioning

- [Calendar identities](versioning/calendar-identities.md)

## Current coverage

- Technical specifications: 65.
- Template records: 1.
- Review date: 2026-07-15.
- Status boundary: this catalog describes current repository behavior and does
  not create or replace architectural decisions.

## Authoring rules

- Use the [technical specification template](template.md).
- Explain repository-owned components, data flow, invariants, failures, and
  verification.
- Cite current decision titles without restating or changing their choices.
- Keep external format research outside this catalog.
- Keep concrete repository paths and command walkthroughs outside
  specifications.

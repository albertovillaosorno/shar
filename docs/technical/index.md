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

- [Animation clip timing](fbx/animation/clip-timing.md)
- [Animation rig model](fbx/animation/rig-model.md)
- [First-principles scene writer](fbx/first-principles-scene-writer.md)
- [Mesh primitive model](fbx/geometry/mesh-primitives.md)
- [Surface vectors and texture coordinates](fbx/geometry/surface-vectors.md)
- [Material assignment](fbx/materials/material-assignment.md)
- [Texture evidence](fbx/materials/texture-evidence.md)
- [Skin attachment](fbx/skeletons/skin-attachment.md)
- [Model output verification](fbx/validation/output-verification.md)

## Localization

- [Normalized language interchange](localization/normalized-language-interchange.md)

## Modding

- [Local mod package model](modding/local-package-model.md)
- [Mod package validation](modding/package-validation.md)

## Pipeline

- [Deterministic conversion pipeline](pipeline/deterministic-conversion-pipeline.md)
- [Evidence and identity model](pipeline/evidence-and-identity-model.md)
- [Character capability evidence](pipeline/extraction/character-capability-evidence.md)
- [Model geometry evidence](pipeline/extraction/model-geometry-evidence.md)
- [Rig and motion evidence](pipeline/extraction/rig-and-motion-evidence.md)
- [Scene assembly evidence](pipeline/extraction/scene-assembly-evidence.md)
- [Texture and summary evidence](pipeline/extraction/texture-and-summary-evidence.md)
- [World intersection evidence](pipeline/extraction/world-intersection-evidence.md)
- [World simulation evidence](pipeline/extraction/world-simulation-evidence.md)

## Unreal

- [Campaign level composition and progress](unreal/campaign-level-composition-and-progress.md)
- [Frontend shell and menu runtime](unreal/frontend-shell-and-menu-runtime.md)
- [Unreal configuration and asset validation](unreal/config-and-asset-validation.md)
- [Unreal gameplay content catalog](unreal/gameplay-content-catalog.md)
- [Mission, interaction, interior, and notoriety runtime](unreal/mission-interaction-and-notoriety-runtime.md)
- [Unreal MCP terminal translator](unreal/mcp-terminal-translator.md)
- [Native asset planning](unreal/native-asset-planning.md)
- [Platform audio cooking and streaming](unreal/platform-audio-cooking-and-streaming.md)
- [Platform cinematic media packaging](unreal/platform-cinematic-media-packaging.md)
- [Platform, quality, and optimization contract](unreal/platform-quality-and-optimization.md)
- [Platform save storage and lifecycle](unreal/platform-save-storage-and-lifecycle.md)
- [Progression, collectibles, cheats, and credits](unreal/progression-collectibles-and-cheats.md)
- [Editor guardrails](unreal/testing/editor-guardrails.md)
- [Unreal test taxonomy](unreal/testing/test-taxonomy.md)

## Validation

- [Deterministic validation](validation/deterministic-validation.md)

## Versioning

- [Calendar identities](versioning/calendar-identities.md)

## Current coverage

- Active technical specifications: 40.
- Template records: 1.
- Review date: 2026-07-14.
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

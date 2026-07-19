# Unreal pipeline contract index

- Status: Active
- Last reviewed: 2026-07-18
- Delivery authority: Phase 6 native content and Phase 7 runtime handoff

## Purpose

This directory is the single pipeline-facing authority for every artifact that
enters the Unreal project. The runtime is designed as a modern, independently
authored AAA game. Legacy data is migration input only. It never controls module
boundaries, content folders, object names, gameplay architecture, rendering
policy, networking, camera behavior, or mod extensibility.

The conversion pipeline must produce exactly the native identities, packages,
assets, schemas, dependencies, and validation evidence defined here. Runtime
code must consume those contracts through Asset Manager identities, typed
definitions, soft references, and explicit services. Neither side may rediscover
intent from filenames, local paths, editor selection, directory scans, or
historical object layout.

## Contract order

Read and implement these documents in order:

1. [Native game foundation](native-game-foundation.md)
<!-- markdownlint-disable-next-line MD013 -->
1. [Content roots, modules, and package layout](content-roots-modules-and-packages.md)
<!-- markdownlint-disable-next-line MD013 -->
1. [Identity, naming, revisions, and import plans](identity-naming-revisions-and-import-plans.md)
<!-- markdownlint-disable-next-line MD013 -->
1. [Materials, textures, UVs, and shaders](materials-textures-uvs-and-shaders.md)
1. [Shared character animation library](shared-character-animation-library.md)
<!-- markdownlint-disable-next-line MD013 -->
1. [Characters, rigs, animation, and selection](characters-rigs-animation-and-selection.md)
<!-- markdownlint-disable-next-line MD013 -->
1. [Vehicles, handling, damage, and phone booths](vehicles-handling-damage-and-phone-booths.md)
<!-- markdownlint-disable-next-line MD013 -->
1. [World, props, roads, interiors, and streaming](world-props-roads-interiors-and-streaming.md)
<!-- markdownlint-disable-next-line MD013 -->
1. [Missions, gameplay, rewards, and saves](missions-gameplay-rewards-and-saves.md)
<!-- markdownlint-disable-next-line MD013 -->
1. [Audio, UI, mods, networking, rendering, and platforms](platform-services-and-extension-contract.md)
<!-- markdownlint-disable-next-line MD013 -->
1. [Synthetic fixtures, validation, and promotion](synthetic-fixtures-validation-and-promotion.md)

## Binding rules

- `/Game/SHAR` is the only base-game content root.
- `/Game/Mods/<namespace>` is the only mounted mod content root.
- Canonical identifiers are lowercase ASCII `snake_case` and never localized.
- Every top-level gameplay concept is a versioned Primary Asset definition.
- Secondary assets have one canonical package location and are referenced
  softly.
- Runtime never loads by concatenated path, filename convention, or folder scan.
- Import plans are deterministic UTF-8 JSON and contain no machine-local paths.
- Final native assets use Unreal coordinates: centimeters, positive X forward,
  positive Z up, and applied transforms.
- UV channels are mesh data. Semantic region metadata is separate typed data; a
  detached UV file is never a runtime asset.
- Textures are external normalized inputs and final `UTexture` assets. They are
  not embedded in production FBX files.
- Missions bind reusable native tasks and policies; they do not create one-off
  code or Blueprint graphs per mission.
- Mods use namespaced Game Feature packages and stable extension points.
- The campaign remains single-player. The executable architecture is network-
  authority-ready and supports user-operated dedicated or listen servers for
  community sandbox and mod-defined modes without official hosting, accounts,
  matchmaking, or a first-party server fleet.
- Rendering correctness never depends on a vendor plugin. TSR is the guaranteed
  Unreal-native baseline; vendor upscalers are optional capability adapters.
- Generated `.uasset` and `.umap` files are reproducible outputs and remain
  untracked until an explicit generated-asset publication policy is accepted.
  Tiny synthetic normalized inputs may be tracked only in the declared fixture
  directory.

## Change control

A pipeline or runtime change that alters a field, folder, identity, asset type,
coordinate convention, texture role, rig semantic, mission operation, bundle,
network capability, or verification rule must update this directory first. The
code, schemas, fixtures, importer, and relevant tests then change in one
coherent batch.

No undocumented compatibility behavior is permitted. When a future requirement
cannot fit an existing contract, add a versioned extension or new asset family;
do not overload an unrelated field or silently reinterpret existing data.

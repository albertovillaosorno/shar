# SHAR

SHAR is an AI-first deterministic migration pipeline and Unreal Engine 5
reimplementation workspace. Its bounded goal is to rebuild a lawful local copy
of the original game as a clean, native, playable, and moddable project.

The repository contains independently authored source code, schemas, manifests,
conversion tools, validation rules, decision records, technical specifications,
and an Unreal project shell. It does not distribute the original game, extracted
assets, proprietary engine source, third-party replacement media, or generated
builds.

## Objective

A user supplies a lawful local installation. The pipeline validates and decodes
that installation, classifies deterministic packages, generates normalized
artifacts, creates native Unreal assets, compiles an independently authored C++
runtime, and produces validated native packages for supported Windows, Linux,
macOS, and Android targets.

The supported product includes:

- five graphics presets: **Low**, **Medium**, **High**, **Epic**, and
  **Ultra**, all using the same gameplay contract;
- validated native desktop and Android packages for the supported architecture
  matrix;
- local deterministic drop-in mods;
- user-facing AI skills for lawful mod creation; and
- native Unreal control for AI agents through the official MCP server.

The project excludes multiplayer, a connected sandbox, a server browser, a
hosted mod service, a marketplace, a social layer, and a general-purpose
launcher or editor.

## Legal and publication boundary

This is an independent interoperability and reimplementation project. It is not
affiliated with or endorsed by the original publishers, developers, licensors,
platform holders, Epic Games, NVIDIA, or any other named third party.

Users are responsible for obtaining a lawful game copy, complying with external
licenses and local law, preserving required notices, and verifying third-party
mod provenance. The repository does not authenticate ownership, download the
original game, or grant permission to redistribute generated output.

Repository-owned material is licensed under the MIT License. Third-party names,
software, formats, assets, and documentation remain governed by their respective
owners. The legal records are academic research used to evaluate project
feasibility and preserve traceability. They are not legal advice, a legal
recommendation, or permission to act. The controlling scope notice is the
[legal research disclaimer](docs/legal/disclaimer.md).

## Engineering model

The repository uses minimal hexagonal architecture. Domain and application rules
remain independent of external effects; ports and adapters exist only for real
boundaries. Rust owns deterministic pipeline behavior, C++ owns the native
runtime, and Python is limited to repository-owned integration tooling such as
the Unreal terminal translator.

The pipeline is fail-closed, deterministic, evidence-driven, and
content-addressed. Equivalent validated input and policy must produce stable
logical identities, ordering, plans, and reports. Silent data loss, guessed
capabilities, manual production assembly, and stale partial success are rejected.

The canonical model artifact is binary FBX 7.7 generated from first principles
by the repository-owned writer. Blender and Maya are not part of generation,
conversion, staging, repair, validation, or acceptance.

## AI-first documentation

Machine-readable authority, exact contracts, deterministic indexes, live tool
discovery, and validation evidence are primary. Human readability remains
required but is secondary when prose convenience conflicts with machine
certainty.

Documentation responsibilities are separated:

- [ADRs](docs/adr/index.md) record repository decisions only.
- [Technical specifications](docs/technical/index.md) explain repository-owned
  implementation only.
- [Skills](skills/) provide executable task guidance.
- [Bibliography](docs/bibliography/index.md) preserves external references.
- [Legal records](docs/legal/index.md) preserve risk analysis.

ADRs and technical specifications do not contain concrete repository paths.
Technical specifications do not explain proprietary external formats.

## Unreal MCP and skill audiences

AI agents control Unreal through the official native inbound MCP server. The
repository-owned terminal translator is an MCP client that converts terminal
intent into native lifecycle, discovery, and tool calls. It is not an MCP server
and does not replace or copy engine plugins.

Unreal skills are technical operating instructions for AI agents and repository
operators. They are not ordinary end-user modding guides. Other modding skills
are user-facing and should allow non-programmers to describe, validate, preview,
and install supported local changes.

## Roadmap

The primary delivery sequence is fixed by decision record. Current status is:

| Phase | Scope | Status |
| :--- | :--- | :--- |
| 1 | Decode required source evidence | Complete |
| 2 | Generate the minor-unit manifest | Complete |
| 3 | Classify deterministic packages | Complete |
| 4 | Generate first-principles binary FBX | In progress |
| 5 | Establish native Unreal MCP terminal control | In progress |
| 6 | Create native Unreal assets | Planned |
| 7 | Implement the complete native runtime | Planned |
| 8 | Verify Low through Ultra graphics presets | Planned |
| 9 | Add local mods and user-facing AI skills | Planned |
| 10 | Package validated native platform builds | Planned |
| 11 | Optimize, verify, document, and close | Planned |

The status table is informational, not a delivery warranty.

## Repository entry points

- [`AGENTS.md`](AGENTS.md) guides AI agents helping mod users.
- [`docs/adr/index.md`](docs/adr/index.md) is the decision catalog.
- [`docs/technical/index.md`](docs/technical/index.md) is the implementation
  knowledge catalog.
- [`skills/unreal/`](skills/unreal/) documents AI and operator access to native
  Unreal tools.

Local game input, decoded output, generated assets, caches, external toolchains,
and private evidence remain outside public tracked content according to the
repository publication boundary.

## Versioning and collaboration

Repository-owned version identities use Calendar Versioning, not Semantic
Versioning. Commit history uses Conventional Commits. Commit types do not derive
a calendar identifier.

The repository maintains no changelog, release notes, release branches, release
tags, or hosted releases. Public collaboration uses issues only; pull requests
are not part of the workflow.

## Maintenance and license

Public availability is not a service-level agreement. No maintenance schedule,
response time, compatibility window, issue-triage guarantee, or permanent
availability is promised. The repository may be archived at any time.

Repository-owned authored material is available under the MIT License in
[`LICENSE`](LICENSE). The license applies only to material the repository owner
has authority to license.

## Detailed project definition

SHAR is a deterministic migration pipeline and Unreal Engine 5
reimplementation workspace for rebuilding a lawful local copy of the original
game as a clean, native, playable project.

The repository contains first-party source code, schemas, manifests, conversion
tools, validation rules, architecture records, and an Unreal project shell. It
does **not** contain or distribute the original game, extracted game assets,
proprietary engine source, third-party audio, artwork, cinematics, executables,
or replacement content.

A user supplies a lawful local installation under `game/`. The pipeline is
intended to decode that installation, normalize its contents, construct native
Unreal assets, compile the independently authored runtime, and produce a local
standalone build.

### Complete product objective

The project has one bounded objective: produce a faithful, technically clean,
fully playable Unreal reimplementation that can be rebuilt from a user-supplied
game installation and modified through local packages.

The primary deliverables are:

1. lossless, fail-closed decoding of every required source format;
1. deterministic manifests and package identities;
1. normalized FBX, media, localization, UI, mission, and gameplay data;
1. native Unreal assets generated without manual editor assembly;
1. independently authored C++ runtime behavior;
1. **Low**, **Medium**, **High**, **Epic**, and **Ultra** graphics presets;
1. validated native Windows, Linux, macOS, and Android packages for the
   supported architecture matrix;
1. local drop-in mods supported by documented schemas and AI-agent skills;
1. one command that rebuilds and packages the selected supported target; and
1. end-to-end verification, optimization, and closure of the primary roadmap.

The project does not include a separate modern gameplay mode, a connected
sandbox, multiplayer, a server browser, a hosted mod service, a social layer, a
general-purpose launcher, or a Roblox-like editor. Those products are outside
this repository's scope.

### Detailed legal and project boundary

This is an independent interoperability and reimplementation project. It is not
affiliated with, endorsed by, sponsored by, or approved by the original game's
publishers, developers, licensors, platform holders, Epic Games, NVIDIA, or any
other third party named in documentation or compatibility targets.

Repository-owned material is licensed under the MIT License in
[`LICENSE`](LICENSE). The MIT License applies only to material the repository
owner has the authority to license. It does not grant rights in the original
game, third-party assets, trademarks, proprietary software, engine
distributions, external plugins, or user-supplied mods.

Users are responsible for:

- obtaining and using their own lawful game copy;
- complying with the terms that govern Unreal Engine and optional plugins;
- determining whether local conversion, modification, packaging, or
  redistribution is lawful in their jurisdiction;
- preserving required copyright and license notices; and
- verifying the provenance and trustworthiness of third-party mods.

The repository does not download the original game, authenticate ownership,
ship extracted payloads, or grant permission to redistribute a generated build.
See the
[lawful local input and publication ADR](docs/adr/legal/lawful-local-input-and-publication-boundary.md)
for the project-specific boundary. This documentation is not legal advice.

### Detailed engineering model

The pipeline is fail-closed, deterministic, and evidence-driven.

- A decoder either produces a typed, count-checked representation or reports a
  failure. Silent byte loss is not accepted.
- Generated identities, ordering, package selection, output names, and plans
  must remain stable for identical input.
- Extraction, classification, packaging, conversion, Unreal import, and runtime
  behavior remain separate architectural boundaries.
- Rust owns orchestration, parsing, manifests, deterministic transforms, and
  validation.
- C++ owns the Unreal runtime.
- Python is permitted only where Blender or Unreal exposes a materially better
  native integration boundary.
- Blueprints remain compatible for content inspection and authoring, but C++
  and validated data remain the source of truth.
- JSON is an intermediate review and interchange representation. It is not the
  final runtime format when Unreal provides an appropriate native asset type.
- Direct dragging and dropping is not the production import strategy. Asset
  creation must be reproducible from manifests and conversion plans.

The codebase uses explicit domain, application, port, and adapter boundaries.
Shared CLI and filesystem crates own stable mechanisms only; domain policy stays
inside the crate that owns the behavior.

## Current status

| Phase | Scope | Status |
| :--- | :--- | :--- |
| 1 | Decode source formats and create the game manifest | Complete |
| 2 | Generate the minor-unit manifest | Complete |
| 3 | Classify minor units into deterministic packages | Complete |
| 4 | Convert model packages to binary FBX 7.7 | In progress |
| 5 | Establish native Unreal MCP terminal control | In progress |
| 6 | Convert normalized data into native Unreal assets | Planned |
| 7 | Implement the complete Unreal runtime | Planned |
| 8 | Verify Low through Ultra graphics presets | Planned |
| 9 | Add drop-in mods and AI-agent skills | Planned |
| 10 | Package validated native platform builds | Planned |
| 11 | Optimize, verify, document, and close the roadmap | Planned |

The table describes the current public roadmap, not a delivery warranty.
Generated counts may change when additional lawful source editions are tested
or when stricter validation invalidates previously accepted evidence.

## Eleven-phase roadmap

### Phase 1 — Decode every required source format

**Status:** Complete.

**Executive result:** The local installation is converted from opaque legacy
containers into typed, auditable, deterministic source evidence. Downstream
systems no longer need to understand undocumented binary layouts.

Completed work:

- [x] Create the simple `game/manifest.jsonl` completeness contract without
  publishing original file names.
- [x] Decode RTF documentation into normalized Markdown.
- [x] Extract LMLM/LSPA archives with payload-size verification.
- [x] Extract RCF archives with validated headers, names, offsets, alignment,
  and deterministic output paths.
- [x] Convert RSD audio into native PCM WAV while preserving channels, sample
  rate, and bit depth.
- [x] Convert RMV cinematics into HAP video packages with numbered WAV tracks,
  probe metadata, decode reports, manifests, and timing ledgers.
- [x] Decode P3D geometry, primitive groups, indices, positions, normals,
  tangents, binormals, UV channels, colors, matrix palettes, weights, bounds,
  and render metadata.
- [x] Decode P3D textures and preserve typed texture metadata and image output.
- [x] Decode scene graphs, composite drawables, instance graphs, render
  references, transforms, visibility, attachments, cameras, and sort order.
- [x] Decode collision, physics, terrain, world-chunk membership, primitives,
  mass properties, inertia, ownership, and joint metadata.
- [x] Decode skeletons, joints, parent relationships, rest transforms, skins,
  controllers, animation groups, channels, keyframes, and vertex animation.
- [x] Decode supported UI, font, particle, and summary families into typed
  representations instead of generic byte summaries.
- [x] Parse MFK mission and gameplay scripts into structured JSON.
- [x] Parse CON vehicle and gameplay configuration into structured JSON.
- [x] Parse Scrooby PAG, SCR, and PRJ files into structured UI records.
- [x] Parse CHO choreography and rig-bank text into structured records.
- [x] Parse TextBible and language-channel files into normalized localization
  records.
- [x] Parse TYP sound-resource metadata into structured JSON.
- [x] Decode RADMusic RMS metadata and SPT text structures into deterministic
  intermediate records.
- [x] Mark logs, unsupported metadata, and non-runtime artifacts as explicit
  do-not-import inputs instead of pretending they are game assets.

Completion criteria:

- accepted arrays match their declared counts;
- geometry, rigging, collision, and animation have no silent header-only
  success path;
- malformed data fails closed;
- the game completeness manifest remains deterministic and public-safe; and
- extracted assets remain local and ignored by Git.

Relevant decisions:

- [Lossless extraction contract](docs/adr/pipeline/extraction/lossless-extraction-contract.md)
- [Extraction provenance and manifest linkage](docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md)
- [Game manifest ledger](docs/adr/pipeline/game-manifest-ledger.md)

### Phase 2 — Generate the minor-unit manifest

**Status:** Complete.

**Executive result:** Every normalized output becomes one typed, independently
auditable minor unit. The manifest is the structural source of truth for
classification and packaging.

Completed work:

- [x] Stage decoded package components and normalized loose files into one
  coherent structural view.
- [x] Assign deterministic, opaque, name-free identifiers.
- [x] Record obfuscated routes that preserve useful structure without exposing
  source names.
- [x] Record source chunk kind, source chunk ordinal, extension, provenance,
  and recovery status.
- [x] Define controlled taxonomy values with `snake_case` JSON fields and
  `kebab-case` domain values.
- [x] Classify supported units by type, subtype, kind, function, origin,
  schema, and intended normalization target.
- [x] Reject raw, metadata-only, unsupported, or error recovery rows as
  successful runtime units.
- [x] Audit the generated manifest until no accepted unit remains classified by
  the error sentinel.
- [x] Keep content hashes and real game names out of the public classification
  ledger.

Completion criteria:

- every accepted minor unit is fully decoded;
- every record conforms to the versioned taxonomy;
- manifest order is stable;
- source names remain undisclosed; and
- no later converter needs to rediscover extraction internals.

### Phase 3 — Organize minor units into packages

**Status:** Complete for the current evidence set.

**Executive result:** Isolated records are composed into deterministic entity
packages that downstream tools can consume without path guessing or manual
curation.

Completed work:

- [x] Build the generated package index from minor-unit records.
- [x] Assign one controlled category and subcategory to every accepted package.
- [x] Preserve members, member roles, source kinds, chunk kinds, and stable
  package identifiers.
- [x] Resolve packages by exact identifier, category, or subcategory prefix.
- [x] Define typed plans for FBX models, Unreal-native data, media, UI, audio,
  worlds, and do-not-import metadata.
- [x] Eliminate successful placeholder labels and error package identifiers.
- [x] Verify the current generated evidence set at 2,964 packages with no error
  package rows.

Completion criteria:

- every accepted minor unit belongs to the correct package or an explicit
  do-not-import package;
- package identity and member ordering are deterministic;
- package readers do not depend on original paths; and
- planners consume typed package records rather than ad hoc JSON.

Relevant decisions:

- [Unreal manifest and package taxonomy](docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md)
- [Native asset translation](docs/adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md)

### Phase 4 — Convert model packages to binary FBX 7.7

**Status:** In progress.

**Executive result:** Every model-like package receives a clean,
general-purpose interchange artifact that can be inspected outside the project
and imported into Unreal without carrying legacy format debt into the runtime.

Completed boundary work:

- [x] Make binary FBX 7.7 the canonical production FBX representation.
- [x] Select source packages from the generated package index.
- [x] Generate deterministic output identities and capability reports.
- [x] Embed referenced PNG textures inside the FBX artifact.
- [x] Implement typed scene, geometry, material, texture, skeleton, skin,
  animation, timing, camera, and transform domains.
- [x] Define package profiles for characters, vehicles, props, and terrain.
- [x] Preserve authored mesh partitions instead of forcibly merging
  unrelated geometry islands.
- [x] Provide optional Blender review and Maya import helpers without making
  either application the source of truth.
- [x] Complete the character FBX package lane for geometry, materials, texture
  references, skeletons, skin clusters, and native animation curves.
- [x] Verify the same canonical character FBX 7.7 artifact in Blender 5.1 and
  Maya 2027 without alternate scene serialization or manual rig repair.

The character writer lane is complete. Representative acceptance testing
confirms geometry, materials, embedded textures, authored mesh partitions,
skeleton hierarchy, skinning, native animation curves, source-rate timing, and
animated posing in both applications. Blender and Maya remain review adapters;
the canonical artifact is still binary FBX 7.7.

The next character milestone is deterministic catalog generation under
`fbx-assets/characters/`: one self-contained FBX file per character package,
stored directly in that directory, plus one manifest describing every result.
The directory is local generated evidence and remains ignored by Git.

Remaining work:

- [ ] Generate the complete character catalog and its deterministic manifest
  under `fbx-assets/characters/`.
- [ ] Complete prop FBX coverage.
- [ ] Complete vehicle FBX coverage.
- [ ] Complete terrain and world-piece FBX coverage.
- [ ] Complete remaining animated-object, camera, and effect FBX coverage.
- [ ] Prove material-slot, texture, coordinate-system, scale, pivot, skeleton,
  skin-weight, and animation-time consistency across the package index.
- [ ] Reject packages that claim model support while omitting a required
  capability.
- [ ] Produce deterministic conformance reports for every generated FBX file.
- [ ] Verify clean Unreal import without undocumented scene repair.

Completion criteria:

- every model-like package has a valid binary FBX 7.7 artifact or an explicit,
  justified non-FBX route;
- no package depends on hardcoded local paths;
- no ASCII FBX, DAE, MA, or MB output becomes canonical; and
- repeated conversion remains deterministic within the format boundary.

Relevant decisions:

- [Package evidence discovery boundary](docs/adr/fbx/extraction/source-discovery-boundary.md)
- [Unsupported model evidence preservation](docs/adr/fbx/chunks/chunk-preservation-policy.md)
- [Hexagonal scene export](docs/adr/pipeline/fbx/hexagonal-scene-export.md)
- [First-principles FBX output contract](docs/adr/fbx/export/fbx-output-contract-boundary.md)

### Phase 5 — Establish native Unreal MCP terminal control

**Status:** In progress. The repository-owned terminal MCP client and the
generated per-tool Unreal skill catalog exist; catalog documentation and
verification coverage are still being completed.

**Executive result:** A terminal-capable agent can discover, inspect, test, and
invoke every tool exposed by the unchanged Unreal Engine 5.8 native MCP server
without a repository-owned editor bridge or private engine patch.

This phase uses the experimental `ModelContextProtocol`, `ToolsetRegistry`, and
`AllToolsets` plugins supplied with Unreal Engine. The native server remains an
upstream dependency and is not copied, modified, repackaged, or published by
this repository.

Planned work:

- [ ] Enable the native Unreal MCP and required toolset plugins in the local
  project configuration without committing proprietary plugin source.
- [ ] Implement a repository-owned terminal MCP client outside `src/unreal`.
- [ ] Support initialization, capability and protocol-version negotiation,
  Streamable HTTP, structured errors, progress, pagination, cancellation, and
  bounded timeouts.
- [ ] Connect only through the loopback endpoint and reject remote, tunneled, or
  overlapping tool execution.
- [ ] Discover the live catalog through `list_toolsets`, `describe_toolset`, and
  `call_tool`, plus eager `tools/list` mode when enabled.
- [ ] Generate a deterministic machine-readable snapshot of every discovered
  toolset, tool, input schema, output schema, default, enum, and side effect.
- [ ] Map every discovered tool to a generic lossless JSON terminal call.
- [ ] Add typed CLI commands for the complete catalog without silently omitting
  difficult, experimental, destructive, or niche tools.
- [ ] Populate `skills/unreal/` with complete command syntax, parameters,
  examples, required editor state, approval rules, errors, and troubleshooting.
- [ ] Add catalog drift checks that fail when the selected engine adds, removes,
  renames, or changes a tool without a reviewed CLI and documentation update.
- [ ] Black-box test server lifecycle, discovery, schemas, valid and invalid
  calls, errors, refresh, reconnection, serial execution, and automation tests.
- [ ] Use the MCP Inspector as an independent UI and CLI reference client.
- [ ] Preserve known editor safety failures as observable regressions instead of
  custom bridge implementation details.

When a severe native defect blocks a required workflow, the project may add a
distinctly named Python or C++ toolset, client workaround, or validation command.
The fix must be additive, independently authored, regression-tested, and usable
against a clean unmodified engine installation. Repository-owned additive code
is MIT-licensed and may be used by Epic Games or any other recipient under those
terms; this does not imply endorsement or upstream acceptance.

Completion criteria:

- the discovered and documented tool counts match exactly;
- every tool has a tested terminal route and a lossless raw JSON route;
- a terminal-only agent can discover and use the full native tool surface;
- protocol, schema, timeout, cancellation, and tool failures fail closed;
- native source and installed engine files remain unchanged;
- no custom MCP server or editor bridge remains in the repository; and
- all committed examples use synthetic or repository-owned project content.

Relevant decisions:

- [Native Unreal MCP terminal bridge](docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md)
- [Native MCP tool CLI projection and Unreal skills](docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md)
- [Native plugin source and additive extension boundary](docs/adr/unreal/mcp/upstream-native-plugin-and-additive-extension-boundary.md)

### Phase 6 — Convert normalized data into native Unreal assets

**Status:** Planned.

**Executive result:** JSON, FBX, WAV, and normalized HAP cinematic evidence
become native Unreal assets and target-verified media variants through
deterministic conversion plans and the Phase 5 terminal MCP surface rather than
manual editor work.

`src/unreal` is the pipeline-owned planning library for this phase. It validates
normalized JSON, PCM WAV, HAP, and binary FBX 7.7 evidence and produces stable
native target identities, dependencies, import plans, and provenance. It never
opens an MCP connection or controls an Unreal process.

Planned work:

- [ ] Generate a committed, public-safe Unreal import manifest from opaque
  package identifiers through the `src/unreal` conversion boundary.
- [ ] Apply conversion plans through tested native MCP commands from Phase 5.
- [ ] Import FBX files as Static Meshes, Skeletal Meshes, Skeletons, Physics
  Assets, Animation Sequences, materials, textures, and cameras.
- [ ] Preserve PCM WAV as normalized audio evidence, construct canonical sound
  metadata and routing, and cook the verified loading, compression, streaming,
  cache, concurrency, and playback policy required by each native target.
- [ ] Preserve HAP video and numbered WAV tracks as synchronized normalized
  evidence, then generate and verify the media-player, container, codec, video,
  and audio variant required by each claimed native target.
- [ ] Convert localization records into String Tables and language assets.
- [ ] Convert mission, vehicle, gameplay, UI, collectible, and tuning records
  into Data Tables, Data Assets, State Trees, or purpose-built native assets.
- [ ] Convert Scrooby-derived UI records into UMG assets or validated native UI
  descriptions.
- [ ] Convert world packages into World Partition cells, Data Layers, streaming
  assets, collision data, and authored assembly records.
- [ ] Preserve import provenance and deterministic Unreal object identity.
- [ ] Make the entire import repeatable from a clean project state.

Completion criteria:

- every accepted package resolves to a native Unreal target or an explicit
  do-not-import result;
- deleting generated assets and rerunning the importer reproduces the same
  logical project state;
- every claimed native target has deterministic audio cooking and streaming
  policies that preserve dialogue, locale, loops, event timing, and required
  playback without network or external codec dependencies;
- every claimed native target has a deterministic cinematic variant with verified
  player, codec, container, audio synchronization, subtitles, event timing, and
  no required external codec or network dependency;
- failures identify the package, member, invariant, and corrective action; and
- no production asset requires undocumented editor-only repair.

Relevant decisions and specifications:

- [Platform-native audio cooking and streaming](docs/adr/audio/platform-native-audio-cooking-and-streaming.md)
- [Platform audio cooking and streaming](docs/technical/unreal/platform-audio-cooking-and-streaming.md)
- [Native cinematic package strategy](docs/adr/rmv/unreal-native-cinematic-package.md)
- [Platform cinematic media packaging](docs/technical/unreal/platform-cinematic-media-packaging.md)

### Phase 7 — Implement the complete Unreal runtime

**Status:** Planned. The C++-primary Unreal project shell and build targets
exist; complete gameplay behavior does not.

**Executive result:** The independently authored runtime can play the complete
game from beginning to end using the assets generated by Phase 6.

Planned work:

- [ ] Implement startup, versioned portable save data, device-local profiles and
  settings, loading, pause, migration, and progression.
- [ ] Implement player movement, camera behavior, interaction, vehicles,
  traffic, pedestrians, collisions, damage, and recovery.
- [ ] Implement missions, objectives, triggers, dialogue, rewards, collectibles,
  gags, races, and progression gates.
- [ ] Implement HUD, radar, navigation, menus, subtitles, localization, audio,
  cinematics, and accessibility settings.
- [ ] Implement world streaming, actor placement, physics, animation, effects,
  and platform input through native Unreal systems.
- [ ] Bind generated assets through stable ports rather than direct path
  assumptions.
- [ ] Add parity tests for observable gameplay behavior and state transitions.
- [ ] Keep all third-party proprietary runtime implementation outside tracked
  repository content.

Completion criteria:

- the complete game is playable from start to finish;
- known progression-blocking defects are absent;
- runtime behavior is driven by validated data and first-party C++;
- save/load, migration, interrupted-write recovery, and restart behavior are
  deterministic across supported x64 and ARM64 targets; and
- the Unreal project builds without untracked proprietary dependencies.

Relevant decisions:

- [Runtime parity boundary](docs/adr/unreal/runtime/remake-parity-boundary.md)
- [Runtime parity tests](docs/adr/unreal/runtime/runtime-parity-test-boundary.md)
- [Portable save storage and lifecycle](docs/adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
- [Platform save storage and lifecycle](docs/technical/unreal/platform-save-storage-and-lifecycle.md)
- [Hexagonal runtime](docs/adr/unreal/architecture/hexagonal-runtime-and-no-technical-debt.md)

### Phase 8 — Verify platform support and Low through Ultra graphics presets

**Status:** Planned.

**Executive result:** One gameplay implementation supports five ordered graphics
presets and validated native desktop and Android packages without changing
mission, physics, timing, progression, save, package, or mod semantics.

#### Platform matrix

- [ ] Package and launch Windows x64, Linux x64, macOS ARM64, and Android ARM64
  builds on representative native hardware.
- [ ] Treat Windows ARM64 and Linux ARM64 as required desktop compatibility
  targets and claim availability only after the selected Unreal toolchain
  produces validated native packages.
- [ ] Reject emulation, cross-compilation alone, and editor play as availability
  evidence.
- [ ] Keep gameplay, saves, package identities, and mod contracts identical
  across platforms and architectures.

#### Graphics presets

- [ ] `Low` is the lowest supported visual configuration. It deliberately uses
  very low native rendering settings while preserving every gameplay-relevant
  visual, collision, navigation, mission, and UI contract.
- [ ] Keep Low visually faithful to the original art direction, with a target
  broadly comparable to the original game or a seventh-generation console game.
- [ ] `Medium`, `High`, and `Epic` increase native Unreal quality monotonically.
- [ ] `Ultra` resolves every supported quality group and selected stable optional
  feature to the maximum validated setting for the active platform and hardware.
- [ ] Keep unsupported hardware and vendor features optional and provide native
  Unreal fallbacks without changing gameplay.

#### Android policy

- [ ] Expose and persist `Low` only on Android ARM64.
- [ ] Reject or normalize settings that request Medium, High, Epic, or Ultra.
- [ ] Enforce a maximum frame-rate cap of 144 frames per second; the cap is a
  ceiling, not a guarantee that every device sustains that rate.
- [ ] Do not infer a desktop frame-rate policy from the Android cap.

#### Optimization boundary

- [ ] Profile CPU, GPU, memory, storage, streaming, shader compilation, and frame
  pacing before and after performance work.
- [ ] Prefer native Unreal scalability, device profiles, streaming, visibility,
  shader, pipeline-cache, material-quality, and platform facilities.
- [ ] Optimize C++ hot paths from measured evidence while preserving
  deterministic behavior and domain invariants.
- [ ] Reject any claimed optimization that removes required content, degrades
  quality outside the selected preset, changes gameplay, hides a failure, or
  introduces a regression.
- [ ] Treat broader hardware compatibility as a desirable consequence of correct
  engineering, not as the product objective or a reason to compromise fidelity.

Completion criteria:

- every claimed platform has a native package validated on representative
  hardware;
- all desktop presets are ordered and monotonic from Low through Ultra;
- Android exposes Low only and never exceeds the 144-frames-per-second ceiling;
- graphics settings do not affect deterministic simulation behavior;
- visual comparison captures and performance evidence are reproducible; and
- no optimization introduces a gameplay, content, determinism, or visual defect.

Relevant decisions and specifications:

- [Graphics quality presets and platform support](docs/adr/unreal/runtime/graphics-quality-presets-and-platform-support.md)
- [Shared runtime tagging, modding, and platform compatibility](docs/adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
- [Platform, quality, and optimization contract](docs/technical/unreal/platform-quality-and-optimization.md)

### Phase 9 — Add drop-in mods and AI-agent skills

**Status:** Planned.

**Executive result:** A user can import a validated package through the native
storage adapter to replace or extend supported game data, and an AI coding agent
can create that package by following repository-owned skills and schemas.
Desktop targets may use a local `mods/` import root; Android uses managed import
and application-owned storage.

The project supplies the contract, not a hosted platform. There is no required
server, account, marketplace, dedicated graphical editor, or proprietary AI
service.

Planned work:

- [ ] Define deterministic mod package identity, priority, dependency,
  compatibility, supersession, and conflict rules.
- [ ] Support replacements and additions for models, textures, materials,
  animation, missions, localization, UI, audio, cinematics, tuning, and other
  explicitly modeled asset families.
- [ ] Load supported data and asset packages through one normalized import
  contract: a local `mods/` root on desktop and managed application storage on
  Android.
- [ ] Keep native-code mods behind an explicit trust boundary because native
  code is not safely sandboxed by file validation alone.
- [ ] Validate schema, normalized member paths, integrity, resource limits,
  references, package topology, target and ABI compatibility, version
  constraints, and deterministic load order before activation.
- [ ] Provide preview and dry-run commands that show exactly what a mod changes.
- [ ] Write practical `skills/` instructions for terminal-capable AI agents.
- [ ] Let an agent translate a natural-language request into required assets,
  mission logic, package changes, validation evidence, and a reviewable preview.
- [ ] Require the agent to ask for missing licensed assets or generate only
  content the user is authorized to create.
- [ ] Require explicit user approval before replacing existing local content or
  enabling trusted native code.

A representative agent workflow is:

1. the user describes a mission or asset change in ordinary language;
1. the agent identifies required models, animations, voices, rules, and rights;
1. the agent asks for missing inputs or offers lawful original placeholders;
1. the agent generates the mission and package data;
1. the agent validates references, load order, gameplay flow, and performance;
1. the agent presents a preview and asks whether the result is acceptable; and
1. after approval, the package is installed through the selected platform
   adapter: the desktop import root or Android managed application storage.

Completion criteria:

- a non-programmer can produce a valid mod with an agent and repository skills;
- the same workflow remains usable manually through documented files and CLI;
- invalid packages fail before runtime activation;
- equivalent desktop and Android imports produce the same logical package,
  preview, load order, and content-only activation result;
- load order and supersession are deterministic;
- native binaries remain target-specific and require explicit trust; and
- the project makes no claim that arbitrary third-party native code is safe.

Relevant decisions and specifications:

- [Drop-in mod packages and AI skills](docs/adr/modding/drop-in-mod-packages-and-ai-skills.md)
- [Local mod trust and distribution boundary](docs/adr/modding/mod-safety-scanner-and-distribution.md)
- [Local mod package model](docs/technical/modding/local-package-model.md)
- [Mod package validation](docs/technical/modding/package-validation.md)

### Phase 10 — Package validated native platform builds

**Status:** Planned.

**Executive result:** A user selects the path to a lawful game installation and
the pipeline performs extraction, normalization, packaging, conversion, Unreal
import, compilation, and local packaging without undocumented manual steps.

Target command:

```bash
pipeline full --game <path> --target <target-id> --preset <quality-id>
```

Canonical target and quality identifiers are defined by the
[platform, quality, and optimization contract](docs/technical/unreal/platform-quality-and-optimization.md).
Android accepts only `android-arm64` with `low`.

Planned work:

- [ ] Detect and validate the source installation.
- [ ] Run Phases 1 through 6 in dependency order.
- [ ] Create or update the Unreal project deterministically.
- [ ] Compile the Phase 7 runtime.
- [ ] Validate the requested Phase 8 graphics preset for the selected target and
  enforce `Low` for Android.
- [ ] Install the Phase 9 package schemas and configure the selected target's
  desktop import root or Android managed-storage adapter.
- [ ] Resume safely after interruption without accepting stale partial output.
- [ ] Report progress, warnings, failures, provenance, and final artifact paths.
- [ ] Generate or select verified target-specific audio cooking, streaming,
  cache, concurrency, and playback policies, and reject packaging when required
  local audio is unsupported.
- [ ] Generate or select the verified target-specific cinematic media variants
  and reject packaging when required local playback is unsupported.
- [ ] Package a native build and required runtime files for the selected
  supported platform and architecture.
- [ ] Verify the packaged build starts outside the editor and plays required
  cinematics without network access or external codec installation.

#### Required external installation

The packaging target is intentionally small, but a C++ Unreal project cannot be
built from nothing. The user must provide:

- the Unreal Engine version selected by the tracked project descriptor;
- the native C++ compiler, platform SDK, and packaging toolchain supported by
  that Unreal installation; and
- any Epic-provided prerequisites required by the selected packaged target.

#### Optional external installation

- The official NVIDIA Unreal or Streamline plugin may be required for the DLSS
  compatibility target when it is available and selected.
- Blender is an optional review adapter, not the canonical FBX generator.
- No third-party game assets or replacement-content packs are bundled.

The pipeline should manage repository-pinned Rust, Python, FFmpeg, and other
portable dependencies where licensing and platform policy permit it. It must not
silently download proprietary game content or accept external licenses on the
user's behalf.

Completion criteria:

- a verified build host for the selected target can produce a native packaged
  build from one source path and documented prerequisites;
- all intermediate stages are resumable and content-addressed;
- the final report identifies every external dependency and generated artifact;
- failure never leaves a misleading success marker; and
- no private local path or source name enters tracked public output.

### Phase 11 — Optimize, verify, document, and close the roadmap

**Status:** Planned.

**Executive result:** The game and pipeline are treated as a finished artifact,
not as a perpetual product roadmap.

Final checklist:

- [ ] Complete a full start-to-finish playthrough without known
  progression-blocking defects.
- [ ] Verify every mission, level transition, vehicle, collectible, cinematic,
  save point, localization path, and ending.
- [ ] Profile CPU, GPU, memory, storage, shader compilation, loading, streaming,
  and package-generation costs.
- [ ] Remove avoidable technical debt, nondeterminism, duplication, dead code,
  undocumented workarounds, and unsupported compatibility bridges.
- [ ] Verify every graphics preset and claimed platform on representative
  native hardware, including Android Low and its 144-frames-per-second ceiling.
- [ ] Rebuild from a clean source installation and compare deterministic
  manifests, packages, reports, and logical Unreal output.
- [ ] Create representative mods that replace a model, texture, mission,
  localization entry, UI element, audio asset, and gameplay rule.
- [ ] Verify an AI coding agent can create and validate a mod from the published
  skills without private repository knowledge.
- [ ] Record known limitations honestly.
- [ ] Record and publish a complete gameplay video link after final verification.
- [ ] Run the canonical global validation without cache.
- [ ] Tag the completed primary roadmap.

After Phase 11, the repository may remain public and active, but no maintenance
schedule, response time, issue triage, compatibility window, or future feature
work is promised. Reproducible defects may be fixed at the owner's discretion.
Issues may remain unanswered, and the repository may be archived at any time.
The MIT License permits others to inspect, download, fork, modify, and maintain
the repository-owned code under its terms.

Relevant decision:

- [Eleven-phase delivery roadmap](docs/adr/pipeline/eleven-phase-remake-delivery-roadmap.md)

## Repository layout

```text
game/             User-supplied lawful source installation. Ignored by Git,
                  except for the tracked obfuscated completeness manifest.
assets/           Local legacy and staged assets. Ignored by Git.
cache/            Local generated state. Ignored by Git.
dependencies/     Repository-managed toolchains and portable dependencies.
docs/adr/         Architecture decision records.
docs/bibliography/ Public references and third-party notices.
docs/legal/       Academic legal research records and the scope disclaimer.
docs/technical/   Repository-owned technical specifications.
extracted/        Local decoded and classified output. Ignored where required.
skills/           Planned practical instructions for mod authors and AI agents.
src/              Rust crates, asset conversion, and the C++ Unreal project.
temp/             Validation caches, reports, and review output. Ignored by Git.
validate.sh       Canonical repository validation entry point.
```

The root Rust workspace currently contains focused crates for shared CLI and
filesystem mechanisms, the pipeline, game-manifest handling, FBX export, LMLM,
P3D, RCF, RSD, RMV, RTF, and Unreal asset conversion.

## Game input

Place a lawful local copy under `game/`. The directory is ignored by Git except
for `game/manifest.jsonl`, which records obfuscated per-folder minimum counts.
Real source file names are not published.

Example obfuscated manifest row:

```json
{"dir":"ss/ms","ext":"mfk","min":2}
```

The row means that an obfuscated folder path requires at least two `.mfk` files.
It does not publish the original directory or file names.

The completeness commands are:

```bash
cargo run -p game-manifest --bin generate-manifest
cargo run -p game-manifest --bin validate-game
```

### Optional Latino Spanish input

A user may provide the supported Latino Spanish mod locally as:

```text
game/jebano_latino_mod.lmlm
```

The file is optional. It is not distributed by this repository. Absence must not
prevent the base installation from validating or building.

## Validation

Run the canonical validator from the repository root. Do not substitute direct
formatter, compiler, linter, or test commands for final evidence.

```bash
# Validate the complete repository.
bash validate.sh

# Validate one path and every child below it.
bash validate.sh src/fbx/

# Force deterministic diagnostic ordering.
bash validate.sh --deterministic

# Prove the current state without cache reuse.
bash validate.sh --no-cache

# Replace successful cache records for one scope.
bash validate.sh --refresh-cache src/pipeline/
```

Successful gate records are content-addressed and stored under ignored local
surfaces. Failed, interrupted, partial, or stale runs must never be cached as
success. Cache hits remain visible and must invalidate when relevant bytes,
configuration, tools, policies, environment, or toolchain versions change.

## Asset and confidentiality policy

Do not commit:

- the original game or any extracted payload;
- local launcher or game installations;
- proprietary engine material;
- third-party replacement models, textures, voices, audio, or cinematics;
- generated Unreal binaries, imported assets, caches, or derived data;
- private evidence paths, machine-specific paths, credentials, or tokens; or
- local review exports and temporary reports.

Tracked tests must use independently authored, synthetic, or otherwise lawfully
redistributable fixtures.

## License

Repository-owned authored material is available under the MIT License in
[`LICENSE`](LICENSE). Third-party names, software, game data, artwork, audio,
engine material, plugins, and documentation remain governed by their respective
owners' terms and are not relicensed by this repository.

## Appendix — Public code, automated analysis, and engineering responsibility

### Public availability and legal permission are different concepts

A public repository is technically available for humans and automated systems
to retrieve, index, analyze, and transform. That observable fact does not erase
copyright, contract, privacy, trademark, export-control, or platform rules.
Likewise, an abstract legal right does not make a public byte sequence
physically undiscoverable. Professional engineering requires acknowledging both
realities at the same time.

For repository-owned material, the owner affirmatively permits use under the
MIT License and does not object to lawful automated retrieval, indexing,
analysis, code search, model training, or code generation. That statement does
not grant rights the owner does not possess, does not relicense third-party
material, does not waive attribution or license conditions, and does not excuse
circumvention of access controls or misuse of personal data.

The governing rule is therefore simple: machines may read the public code to the
same extent that humans may lawfully read it, and outputs remain subject to the
same provenance, license, security, and correctness obligations.

### The author of a byte sequence is not a correctness argument

Whether code was typed by a person, emitted by a generator, synthesized by a
model, or produced through a mixture of those methods is not evidence that the
result is correct. Authorship does not prove memory safety, determinism,
maintainability, legal provenance, or fitness for purpose.

Generated code receives no exemption from review. Human code receives no
presumption of superiority. Both must satisfy the same contracts:

- explicit ownership and dependency direction;
- deterministic behavior where required;
- bounded resource use;
- complete error handling;
- testable invariants;
- lawful provenance;
- canonical validation; and
- understandable maintenance boundaries.

The compiler does not evaluate sincerity. The runtime does not reward effort.
A defect remains a defect after a week of careful manual typing, and a correct
implementation does not become defective because a tool produced the first
draft.

### Code may have aesthetic value, but aesthetics do not override engineering

Software can contain elegance, creativity, and cultural value. None of those
qualities suspends its operational obligations. In this repository, describing
code as art is never accepted as a defense for nondeterminism, hidden state,
unsafe memory behavior, unverifiable abstraction, or avoidable complexity.

Object orientation is not prohibited. Unjustified indirection is. A class
hierarchy that clarifies ownership and invariants may be useful; a hierarchy
that hides data movement, lifetime, allocation, or control flow is technical
debt. The same rule applies to functional, data-oriented, metaprogrammed, and
AI-generated code. Paradigm labels do not excuse poor boundaries.

The real form of software slop is not machine authorship. It is code whose
behavior, ownership, failure modes, and cost cannot be explained or verified.

### On technological Luddites

Technological Luddites who reject automation merely because it reduces manual
typing are defending a labor ritual, not an engineering principle. Skepticism is
valuable when it identifies a concrete failure mode: fabricated behavior,
license contamination, security defects, dependency risk, nondeterministic
output, or loss of human control. Skepticism becomes obstruction when it offers
no testable claim and treats the existence of a tool as the defect.

The modern engineer is not merely a typist. The engineer defines the contract,
chooses the data model, establishes trust boundaries, constrains the tool,
reviews the result, measures the system, and remains accountable for what ships.
Delegating mechanical work does not delegate responsibility.

### Durability, mortality, and the useful horizon

People are temporary. Public technical work can outlast its author, be forked by
strangers, be understood by machines, and acquire uses that were not predicted
when it was written. That is not a reason to pretend ownership disappears; it is
a reason to write clear licenses, stable schemas, deterministic tools, and
honest boundaries.

Human beings may reach Mars. This repository has a nearer and more measurable
horizon: convert a lawful local installation into a correct executable, preserve
the game's identity, make the result modifiable, document the process, and stop
when the engineering contract is complete.

### No warranty and no permanent service obligation

Public availability is not a service-level agreement. The repository is
provided under the warranty disclaimer in the MIT License. Automated systems,
mod authors, downstream maintainers, and end users must independently validate
their use. The owner does not promise continuous maintenance, compatibility with
future toolchains, review of every issue, acceptance of external patches, or
preservation of the repository in an unarchived state.

The durable artifact is the licensed source and its recorded contracts, not a
promise that its original author will remain available forever.

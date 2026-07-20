# SHAR

<!-- markdownlint-disable MD013 -->

SHAR is an AI-first deterministic migration pipeline and Unreal Engine 5
reimplementation workspace. Its bounded goal is to rebuild a lawful local copy
of the original game as a clean, native, playable, and moddable project.

The repository contains independently authored source code, schemas, manifests,
conversion tools, validation rules, decision records, technical specifications,
and an Unreal project shell. It does not distribute the original game, extracted
assets, proprietary engine source, third-party replacement media, or generated
builds.

## Objective

In the target workflow, a user supplies a lawful local installation. The
completed pipeline is intended to validate and decode that installation,
classify deterministic packages, generate normalized artifacts, create native
Unreal assets, compile an independently authored C++ runtime, and produce
validated native packages for the planned Windows, Linux, macOS, and Android
target matrix.

The target product is defined to include:

- five graphics presets: **Low**, **Medium**, **High**, **Epic**, and
  **Ultra**, all using the same gameplay contract;
- validated native desktop and Android packages for the supported architecture
  matrix;
- local deterministic drop-in mods;
- a stable multiplayer-adapter boundary for mod-owned community-server modes;
- user-facing AI skills for lawful mod creation; and
- native Unreal control for AI agents through the official MCP server.

These are acceptance targets, not current availability claims. The roadmap below
identifies completed, in-progress, and planned phases; native platform packaging
remains planned.

The base product excludes a first-party multiplayer campaign, matchmaking,
server browser, hosted service, marketplace, social layer, and general-purpose
launcher or editor. It does expose stable mod-facing server adapters so community
packages may implement and operate their own multiplayer modes and servers.

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
| 4 | Generate semantically prepared first-principles binary FBX | Complete |
| 5 | Establish native Unreal MCP terminal control | Complete |
| 6 | Create native Unreal assets | Planned |
| 7 | Implement the complete native runtime | Planned |
| 8 | Verify Low through Ultra graphics presets | Planned |
| 9 | Add local mods and user-facing AI skills | Planned |
| 10 | Package validated native platform builds | Planned |
| 11 | Optimize, verify, document, and close | Planned |

Phase 3 is complete. A fresh audit covered all 119,361 manifest units with zero
failures or error rows. The generated index contains 2,964 unique packages with
concrete classifications, complete one-time physical coverage, valid source
linkage, and no successful placeholder or error result. Two complete generations
were byte-identical, including package identities, ordering, membership, roles,
category and subcategory counts, and rendered rows.

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

The project does not ship a first-party multiplayer campaign, matchmaking,
server browser, hosted service, social layer, general-purpose launcher, or
Roblox-like editor. It does provide stable mod-facing networking and server
adapter contracts. Community packages may implement independently operated
servers and multiplayer modes, but they own transport, rules, discovery,
moderation, security, persistence, compatibility, and support. The base product
uses one connected single-player sandbox around the faithful original mission
sequences.

### Open-sandbox product model

The game has exactly two player-facing gameplay states:

- **non-mission**, the persistent open sandbox; and
- **mission**, one accepted story mission, side activity, race, taxi job, or boss
  encounter.

There is no Level 11 and no test level. Development uses ordinary isolated test
fixtures that never appear in the campaign, saves, map, achievements, or content
catalog.

The story is organized into seven narrative **chapters**, not seven isolated
player-facing levels. Historic level names remain source aliases only. Starting a
new game places Homer directly in the world in non-mission state, with a contextual
ambient vignette such as eating a donut, performing a gag, idling at home, or
appearing at Moe's Tavern.

The persistent map is connected with original or appropriately licensed bridges,
roads, tunnels, paths, and transitions. Chapter 1 initially permits ordinary play
only in terrain family 1. Later terrain families unlock through discovery and
chapter progression. Undiscovered map regions remain covered by attractive cloud
fog, while available mission markers remain visible without revealing hidden
roads or landmarks.

Mission-specific actors, vehicles, pickups, hazards, routes, and scripted changes
exist only while their mission is active. The game supports saving during missions
through deterministic checkpoints and resumes without duplicating rewards,
collectibles, currency, boss completion, or achievement progress.

Chapter collectible sets activate cumulatively. Chapter 1 cards, wasps, gags, and
other persistent placements are available at new game. Completing each chapter's
final story mission activates the next chapter's sets, and earlier uncollected
content remains available. A player who completes the story first may therefore
find all seven chapter sets active together.

The menu shows collectible, costume, character, and achievement categories from
the beginning. Purchased costumes are permanent and may be equipped from the menu.
Unlocked eligible characters may be selected outside missions. Bart unlocks after
Homer's final Chapter 1 mission, becomes unavailable after Chapter 2 until Chapter
4 completes, and Lisa's missions force Lisa.

Chapter transitions use chapter language, for example:

> Congratulations. You completed the final mission of Chapter 1.
>
> Characters unlocked: Bart.

The world runs a mandatory 24-real-minute sunrise, day, sunset, and night cycle.
One real minute equals one in-game hour. Missions may require a time window, and
the player may wait or sleep at declared homes, motels, or rest locations. Paid
sleep, permanent costumes, vehicles, taxi participation, and instant vehicle
repair provide fair recurring coin sinks.

A bounded set of coin sources regenerates when a world session begins. One-time
mission, collectible, chapter, boss, purchase, achievement, and discovery rewards
never regenerate. Economy curves must grow gradually without excessive grinding
and always preserve a recoverable path to story progress.

There are 49 collector cards: seven sets of seven. Completing a whole chapter set
unlocks one balanced passive ability. The base game therefore balances seven
meaningful passives rather than 49 individual abilities.

The game adds simple melee combat, sprint stamina, footprints, dirt, wetness, and
other scalable world details. Bart gains zip-line traversal and may break a window
only when the owning structure declares a real available interior and valid entry
route. Every structure records whether its interior capability is none, linked,
streamed, mission-only, or a future extension slot; structures and interiors
remain separate from terrain.

Burns' mansion becomes permanently accessible through a later traversal route
originating inside the nuclear plant. The route remains locked until it cannot
create unfair shortcuts for earlier missions that use terrain family 1.

Chapter 7 keeps the world clock but applies permanent irradiated cloud cover. Day
is humid, overcast, slightly brighter, and mildly hazy. Night is darker and more
threatening, with readable distance, monsters, and sustained environmental horror
rather than cheap jump scares. Radiation damages a visible health bar, zombies
may attack, and a nearby vehicle explosion may cause immediate death and mission
checkpoint restart. The Devil Homer costume suppresses ordinary zombie hostility
without granting radiation or explosion immunity.

The campaign reserves three boss slots, but only two are currently confirmed: a
mechanical dinosaur encounter associated with the stadium near the end of Chapter
2, and an Apu-associated Tyrannosaurus-skeleton encounter associated with the
museum. Their models are original, generic, and mod-replaceable. Completing the
encounters permanently opens the stadium and museum. The third boss remains
pending rather than being invented.

The purchasable taxi unlocks repeatable taxi side missions as a nod to classic
open-world taxi gameplay and driving-focused Simpsons games. Completing every
unique base taxi milestone earns an achievement; taxi work never gates the story.

Achievements are required but still **pending implementation**. The base set is
intentionally approachable and contains no missable achievements. Planned
categories include chapters, cards, wasps, costumes, current coin totals, side
missions, taxi milestones, 100 percent completion, every authored shortcut,
per-mission no-death records, major world expansions, purchases, and humorous
cumulative actions such as kicking 100 pedestrians.

The intended completion tone is summarized by the joke:

> The platinum trophy for Simpsons Hit & Run gave me Simpsonphobia.

Mods declare whether they preserve base-achievement eligibility, suspend affected
base progress, or provide namespaced mod-owned achievements. Mods may also replace
semantic gameplay and presentation slots, including generic boss, connector, and
Unused Content assets, without changing canonical identity or save meaning.

Known bugs that permit impossible or near-instant campaign completion are fixed
even when historical speedrun routes use them. Legitimate speedrunning remains
supported through movement, route planning, vehicle control, mission execution,
and intentional mechanics rather than corrupted state or computation defects.

The visual baseline uses original cel-shaded materials and outlines inspired by
the dimensional cartoon presentation of *The Simpsons Game*. No assets or
proprietary shaders are copied. The style remains compatible with the day-night
cycle, Chapter 7 horror, accessibility, Low through Ultra presets, and modded
materials.

Authoritative design and implementation records:

- [Open sandbox chapters and world progression](docs/adr/gameplay/open-sandbox-chapters-and-world-progression.md)
- [Open sandbox campaign design](docs/technical/gameplay/open-sandbox-campaign-design.md)
- [Open sandbox chapter runtime](docs/technical/unreal/open-sandbox-chapter-runtime.md)
- [Unified open world and chapter projection](docs/adr/pipeline/unreal/unified-open-world-and-chapter-projection.md)
- [Mod-owned multiplayer adapters and community servers](docs/adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md)
- [Multiplayer adapter and community-server extension](docs/technical/modding/multiplayer-adapter-and-community-server-extension.md)

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

### Pipeline process coordination

Normal pipeline commands use a cooperative Rust run registry. By default, one
active run blocks another pipeline process and reports the existing run instead
of silently starting duplicate extraction or conversion work. Inspect active
processes with:

```bash
pipeline active
```

`pipeline --active` is an equivalent inspection alias. Each line reports the
stable run identifier, operating-system PID, command, optional label, execution
mode, lifecycle state, current stage, completed and total work when known,
elapsed time, evidence-backed ETA, and current item. Unknown progress remains
`unknown` rather than being estimated without measurements.

Request cancellation at the next safe work boundary with:

```bash
pipeline cancel <run-id>
pipeline cancel all
```

`pipeline --cancel <run-id>` is an equivalent alias. Cancellation is cooperative:
the current atomic archive, package, or output transaction may finish before the
process exits, but the pipeline does not force termination halfway through one
artifact.

Intentional parallel execution requires explicit acknowledgement and should use
portable labels:

```bash
pipeline fbx-export-world <index> <game> <coordinates> <output-a> \
  --allow-concurrent --run-label world-a
pipeline fbx-export-vehicles <index> <game> <output-b> \
  --allow-concurrent --run-label vehicles-b
```

Every concurrent process still has an independent run identifier, heartbeat,
active record, cancellation route, and default diagnostic log under
`logs/pipeline/runs/<run-id>.jsonl`. An explicitly supplied `--log` path remains
unchanged. Derived registry state lives under `temp/pipeline/runtime/`; stale
crash residue is recoverable and is never repository or output authority.

## Current status

| Phase | Scope | Status |
| :--- | :--- | :--- |
| 1 | Decode source formats and create the game manifest | Complete |
| 2 | Generate the minor-unit manifest | Complete |
| 3 | Classify minor units into deterministic packages | Complete |
| 4 | Convert model packages to binary FBX 7.7 | Complete |
| 5 | Establish native Unreal MCP terminal control | Complete |
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

### Phase 4 — Generate semantically prepared first-principles binary FBX

**Status:** Complete.

**Executive result:** Every model-bearing Phase 3 package now has either a
validated binary FBX 7.7 artifact or an explicit non-FBX route. The final ignored
local publication contains 110 character packages, 73 non-world model props, 285
standalone world props, Wasp Camera, Wrench, 88 vehicles, and a separated world
collection assembled from 149 terrain packages across eight independent scopes.
The world publication contains 127 normally imported root FBXs and 89 isolated
review FBXs rather than one monolithic file.

Completed boundary work:

- [x] Make binary FBX 7.7 the canonical production FBX representation.
- [x] Select source packages from the generated Phase 3 package index.
- [x] Preserve authored topology, mesh partitions, pivots, rigid bindings,
  skeletons, skin clusters, animation timing, and supported animation curves.
- [x] Publish external PNG textures and prohibit packed image payloads in the
  canonical character catalog.
- [x] Classify transparent, glass, mirror, reflective, light-emitting, and visual
  effect surfaces from decoded material and geometry evidence.
- [x] Split shared source shaders into semantic FBX material variants when one
  source shader serves ordinary and light-emitting geometry.
- [x] Apply horizontal UV correction only to orientation-sensitive graphics such
  as text, logos, photographs, signs, labels, screens, decals, and liveries.
- [x] Preserve source UV orientation for ordinary paint, wheels, glass, terrain,
  repeating materials, and visual effects.
- [x] Assemble vehicle pieces at authored rest transforms and ground vehicles by
  their four authored road-wheel surfaces.
- [x] Publish malformed or unsupported source evidence as named sidecars instead
  of manufacturing replacement geometry.
- [x] Publish a globally aligned separated-world collection with collision
  inspection geometry and definition-only review galleries kept distinct.
- [x] Keep coordinate-comparison evidence transform-only: it may align canonical
  source geometry but never supplies public model, material, or texture payloads.
- [x] Route camera-only, controller-only, attribute-only, and gameplay-only
  packages to Phase 6 native Unreal conversion instead of empty FBX placeholders.

#### Character catalog

The final ignored character catalog is `fbx-assets/characters/`. It contains 110
package directories and one root `catalog.json`, totaling 711 files and
3,096,864,010 bytes. The complete publication snapshot SHA-256 is
`ce28cc0e3e6fe081795668bf77fce9c50c96ffee77d65dcbbeb7b4026bca64e6`,
and the root catalog SHA-256 is
`756bf9ef29020c0df2b34d76f914ef92ca67d5e403347c883ad6605904b7d1e7`.

Each package publishes one binary FBX, one deterministic texture plan, and its
referenced external PNG files. The catalog records zero packed images, preserved
source/output topology, non-empty skin clusters, and non-empty animation sets.
Across all packages it records 7,243 animation clips, 3,657 bones, 469 geometries,
5,140 clusters, 280 materials, 280 FBX texture bindings, and 490 published PNG
files. Character animation behavior is unchanged in this phase.

Semantic material classification is shared with the prop, vehicle, and world
writers. Evidence-backed glasses, lenses, transparent surfaces, reflectors, and
emitters remain independently addressable without converting unrelated materials
into glass or emissive surfaces. Character modernization does not increase source
polygon or vertex counts. It changes deterministic UV placement, texture
resolution, semantic material organization, and proven mesh presentation only.

Generate a fresh character catalog with:

```bash
pipeline fbx-export-characters extracted/minor-unit/index.jsonl \
  temp/characters .
```

#### Standalone animated props

`fbx-assets/props/wasp-camera/` contains five files totaling 732,554 bytes. Its
724,652-byte FBX contains 19 geometry groups, a pruned 16-bone rig, 19 rigid
clusters, preserved vertex colors, external textures, and the authored
`PTRN_beecamera` animation. The final FBX SHA-256 is
`ff123439b6dd169e211de2fe9928cdf936a539d9725ffa666efd5e0746ac4614`.
Shield, ray, particle, collision, state, explosion, and gameplay evidence remains
outside this model artifact.

`fbx-assets/props/wrench/` contains two files totaling 74,302 bytes. Its
72,604-byte FBX contains `wrench7Shape`, the required `wrench` and `wrench7`
joints, one rigid cluster, one material and texture binding, and the authored
cyclic `PTRN_wrench` animation. The final FBX SHA-256 is
`d5cfd951bea29b95e23588157f7e948264c410b4f2f69fe8acccdc1147f05ee3`.
Collection, glow, billboard, light, and particle evidence remains outside this
model artifact.

#### Non-world and standalone world props

The complete ignored non-world prop catalog is published beneath
`fbx-assets/props/`. Mission assets live under `missions/`, card-package models
under `cards/`, and the phone interaction model at
`phone-icon/phone-icon.fbx`. The batch reduces 74 model-bearing occurrences from
270 source packages to 73 unique assets: 71 mission models, one collectible-card
model, and one phone-interaction model.

The non-world publication contains 185 files and 9,821,919 bytes: 73 binary FBX
files, 111 external PNG files, and one catalog. It records two static assets and
71 rigid animated assets. Its snapshot SHA-256 is
`f10cdd45d0161003c89f711c81f0f078b5e3dad10b997925a0b40d881996cc47`,
and its catalog SHA-256 is
`86e4149c592952ba5b44344963076822240d4c9ebb331ef86e35f5e0673ae95c`.

Generate a fresh non-world prop catalog with:

```bash
pipeline fbx-export-props extracted/minor-unit/index.jsonl game \
  temp/non-world-props
```

The standalone world-prop catalog is `fbx-assets/props/world/`. It contains 875
files and 27,985,651 bytes, representing 285 readable prop names from 840
model-bearing occurrences across 149 terrain packages. It publishes 190 static
assets and 95 rigid animated assets, merges eight compatible same-name variants,
and retains ten incompatible visual variants as catalog evidence. Its snapshot
SHA-256 is
`108c5c57052175e332bb2bc754f4fe0f25370628ea54c3c7d89c925266f0504d`,
and its catalog SHA-256 is
`c2b66b85952cea0a99818d3f717a54405ad46ce871e4013056bb489951288452`.

Generate a fresh standalone world-prop catalog with:

```bash
pipeline fbx-export-world-props extracted/minor-unit/index.jsonl game \
  temp/world-props
```

Breakable tree FBXs contain only geometry owned by their selected model
composites. Foliage, leaf-drop effects, particles, collision, placement, cameras,
lights, sounds, scripts, and gameplay logic remain normalized inputs for Phase 6
rather than invented FBX objects.

#### Vehicle catalog

The complete ignored vehicle catalog is `fbx-assets/vehicles/`. Each of the 88
standalone vehicle packages receives one readable directory and matching binary
FBX file. The shared car runtime package remains source authority and does not
become a misleading standalone vehicle.

The final publication contains 2,570 files and 64,114,009 bytes. Its snapshot
SHA-256 is
`d8c7a05f0cca87ace6ae13e730cf719e5dbea68275d96e671a09a9142ac63403`,
and the root catalog SHA-256 is
`749c84b98a54ed3bac9cef154d1aefdea32d3347c1c668fb801bbce9550188b0`.
The catalog records 3,341 separated parts, 35 skeletal animations, 70 effect
animation sidecars, 1,292 external textures, 1,025 decoded shader sidecars, six
deferred geometry records, and 16 hidden wheel proxies.

Vehicle parts are split by source mesh, primitive group, composite instance, and
semantic role. The catalog records 324 glass, 319 reflective, 1,846
light-emitting, 2,074 transparent, and 12 visual-effect surfaces. Shared shaders
such as `pizza_vAll_m` and `cKlimo_vWheel_m` produce separate ordinary and
light-emitting FBX materials instead of making every user emissive. Reused wheel
geometry is instantiated at every authored wheel pivot.

The four exceptional vehicles `honor-v`, `hbike-v`, `mono-v`, and `frink-v`
retain four hidden source-backed wheel proxies each. The remaining deferred
records preserve one malformed `cmilk` billboard and the fully invalid
`comic-v` steering mesh without fabricating replacements. Partially invalid
meshes remain hard failures.

Blender 5.1 imported representative `pizza`, `cklimo`, `honor-v`, `hbike-v`,
`mono-v`, and `frink-v` outputs with their expected armatures and actions, zero
missing external images, separated glass and emitters, and distinct ordinary and
emissive material variants for the two shared-shader cases.

Generate a fresh vehicle catalog with:

```bash
pipeline fbx-export-vehicles extracted/minor-unit/index.jsonl game \
  temp/vehicles
```

#### Separated world baseline

The final ignored world publication is `fbx-assets/world/`. It is a separated,
globally aligned collection rather than one monolithic FBX. Import only the root
`*.fbx` files for the normal world scene. Files under `review/` are isolated
comparison galleries and must not be mixed into the normal import set.

The current source-generated verification publication contains 3,710 files and
644,449,224 bytes. The world catalog SHA-256 is
`00cf4788311682aed1810090f0b14a83a897bc848ee00324e1defa0cfb2cfa13`,
the coordinate-movement manifest SHA-256 is
`b52a57e71a90f3317a6f97763619136e058a04ccba2e68ca0f04762b0c3b001e`,
and the transform manifest SHA-256 is
`2bb47dcbda39afc2a7de6ef4f49e6bd5b86f901d4d081f98a699b53078129a31`.

The world catalog covers 129 source packages across the seven main-level scopes
and publishes 115 root world FBXs plus 82 isolated review FBXs. It records 19
interiors, 9,744 source meshes, 12,216 authored and reference-backed placements,
zero canonical placement fallbacks, 7,934 excluded collision meshes, 2,384
definition-only review meshes, and 442 review similarity groups. Six data-only
packages correctly publish no geometry. No auxiliary or bonus-area package enters
this stage.

Material semantics remain explicit across the collection. The catalog records
5,734 glass, eight mirror, 1,963 reflective, 4,536 light-emitting, 7,388
transparent, and 527 visual-effect geometries. Collision meshes are excluded;
definition-only review geometry remains isolated in similarity-overlaid inspection
galleries without being merged into or substituted for canonical source geometry.

`world.transforms.json` uses the
`shar.world-package-transforms.v5` contract. Every root file has baked coordinates
and requires zero additional translation, rotation, or scale. Importers may
create the shared `SHAR_Export_Root` axis-conversion transform; preserve that
common imported transform instead of applying per-package placement offsets.

The strict Blender 5.1 review scene loads 36 editing FBXs: three zone generals,
seven level variations, seven race variations, one mission-door set, and 18
interior variants. Those variants cover all 19 source interior packages because
the exact Levels 2 and 5 Moe's Tavern duplicate is represented once. All three
Kwik-E-Mart variants remain separate. The scene keeps every image external,
contains no linked libraries, and verifies the global non-interior mirror while
preserving source height and interior orientation.

Generate a fresh separated world baseline with:

```bash
pipeline fbx-export-world extracted/minor-unit/index.jsonl game \
  <coordinate-evidence-root> temp/world
```

The optional coordinate-evidence root is local comparison authority only. It is
never copied into the publication, named in public catalog paths, or treated as
model, topology, material, texture, or identity authority.

The ignored `fbx-assets/world/world.blend` file is a temporary coordinate-review
workspace. It is not model, topology, material, texture, validation, publication,
runtime, or production authority. Non-interior world objects are transform-locked.
Only the 18 interior variants are movable during this review pass. Variants must
not be deleted in Blender; the operator reports which variants should be merged,
discarded, or retained so those decisions can become deterministic source rules.
Vertex, topology, and material edits remain outside this pass.

The reviewed world uses three recurring exterior families. Zone 1 contains
Levels 1, 4, and 7; Zone 2 contains Levels 2 and 5; Zone 3 contains Levels 3 and
6. Zone 2 and Zone 3 retain their reviewed connected placements. A final global
source-X reflection then applies to every non-interior family, race prop, door,
and coordinate-bearing runtime record. This reflection cancels the horizontal
reversal introduced by the shared FBX export root. Interiors do not receive it.

The resulting source-space row-vector formulas are stable generation authority:

```text
Zone 1: X' = -X;                    Y' = Y; Z' = Z
Zone 2: X' =  Z - 989.247314453125; Y' = Y; Z' =  X - 360.1337585449219
Zone 3: X' = -Z - 745.36083984375;  Y' = Y; Z' = -X + 296.96331787109375
```

The Zone 3 placement was solved by matching stable vertex indices against the
untouched Level 3 general FBX; the maximum residual was below `0.00016` Blender
units. Blender height translation is ignored. The pipeline preserves source
height while applying placement and the final reflection to geometry, collision
evidence, doors, object placements, character and object spawns, mission
positions, triggers, cameras, locators, and lights. Interiors retain their
existing own-center horizontal reflection and remain independently placeable.

The strict local editing tree is organized by ownership rather than by a complete
copy of every level. Each recurring family has exactly one common baseline:

```text
fbx-assets/world/zone/1/general/
fbx-assets/world/zone/2/general/
fbx-assets/world/zone/3/general/
```

Only true level-specific zone and region differences live below the corresponding
`zone/<number>/level-<number>/` directory. Family-general geometry must not be
repeated in a level variation. Race props and mission doors are excluded from
zone FBXs and live below `race/` and `doors/`. A shared race-general file is
created only when exact shared geometry actually exists; an empty or misleading
common layer is never invented.

Interiors are separate from zones and grouped by stable source-backed identity:
elementary school (`i00`), Kwik-E-Mart (`i01`), Simpsons house (`i02`), DMV
(`i03`), Moe's Tavern (`i04`), Android's Dungeon (`i05`), observatory (`i06`), and
Bart's room (`i07`). Each non-identical source-level copy is exported below
`fbx-assets/world/interiors/<id>-<name>/level-<number>/`. Package-local geometry
is aligned to a local origin, source collision is excluded, and each variant keeps
the existing own-center horizontal reflection. Exact normalized package
duplicates collapse to one representative with every source level recorded.

The current evidence produces 18 variants from 19 packages. Moe's Tavern Levels
2 and 5 are the only exact duplicate pair. Elementary School, Kwik-E-Mart, and
the Simpsons house each retain distinct Levels 1, 4, and 7 variants; the remaining
identities retain each distinct recurring-family copy. The operator reviews these
objects side by side and reports which variants to merge, discard, or keep. Those
decisions, not scene deletion, become the next deterministic source-dependent
rules. The `.blend` itself never becomes runtime or generation authority.

A later manual mesh-correction pass uses a mirror directory with the same strict
FBX identities. Original and edited FBXs are compared to derive deterministic,
source-dependent algorithms for approved vertex movement, topology changes,
semantic separation, and material assignment. No standalone proprietary model
payload is embedded in those algorithms. Terrain and water that are better
represented by native Unreal systems may be replaced rather than retained inside
mixed static meshes.

Phase 6 owns the deferred presentation repairs: selective UV orientation, color
semantics, opaque window-frame separation from transparent glass, interior
material behavior, and lamps or emissive surfaces that react correctly to the
environment and time of day. The disposable `.blend`, strict editing FBXs,
comparison mirrors, and review catalogs are deleted after their corresponding
algorithms are implemented and verified.

Phase 4 exports only packages with actual model geometry. Camera-only,
controller-only, attribute-only, and gameplay-only packages remain normalized
inputs for Phase 6 native Unreal conversion. Phase 4 has no Unreal-import
acceptance gate; Phase 6 consumes the already prepared semantic regions, parts,
coordinates, pivots, and manifests.

Phase 4 completion evidence:

- [x] Characters, standalone animated props, non-world props, and standalone
  world props are published under stable readable paths.
- [x] Vehicles preserve authored assembly, grounded wheel surfaces, semantic
  material variants, supported animation, and malformed-evidence sidecars.
- [x] The separated world baseline is deterministic, globally aligned, importable
  without per-package offsets, and published with collision, review, catalog, and
  transform evidence.
- [x] Canonical FBX, P3D, and pipeline formatting, compile, test, and strict
  Clippy gates pass. Version-currency remains a non-blocking infrastructure check.

Relevant decisions:

- [Package evidence discovery boundary](docs/adr/fbx/extraction/source-discovery-boundary.md)
- [Unsupported model evidence preservation](docs/adr/fbx/chunks/chunk-preservation-policy.md)
- [Character semantic texture, rig, outfit, and prop contract](docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md)
- [Semantic component and geographic placement contract](docs/adr/fbx/export/semantic-component-and-geographic-placement-contract.md)
- [Hexagonal scene export](docs/adr/pipeline/fbx/hexagonal-scene-export.md)
- [First-principles FBX output contract](docs/adr/fbx/export/fbx-output-contract-boundary.md)

### Phase 5 — Establish native Unreal MCP terminal control

**Status:** Complete. The repository-owned terminal MCP client and the
deterministic per-tool Unreal skill catalog cover all 52 discovered toolsets and
830 tools. Every capability page has current manual guidance, including verified
success paths, fail-closed diagnostics, upstream UE 5.8 incompatibilities,
identity boundaries, interactive requirements, and cleanup procedures.

**Executive result:** A terminal-capable agent can discover, inspect, test, and
invoke every tool exposed by the unchanged Unreal Engine 5.8 native MCP server
without a repository-owned editor bridge or private engine patch.

This phase uses the experimental `ModelContextProtocol`, `ToolsetRegistry`, and
`AllToolsets` plugins supplied with Unreal Engine. The native server remains an
upstream dependency and is not copied, modified, repackaged, or published by
this repository.

Completion evidence is deterministic: the live catalog contains 52 toolsets and
830 tools, the generated skill tree contains 830 current capability pages and
zero review-required pages, protocol negotiation uses `2025-11-25`, and native
engine or installed plugin source remains unchanged.

Completed work:

- [x] Enable the native Unreal MCP and required toolset plugins in the local
  project configuration without committing proprietary plugin source.
- [x] Implement a repository-owned terminal MCP client outside `src/unreal`.
- [x] Support initialization, capability and protocol-version negotiation,
  Streamable HTTP, structured errors, progress, pagination, cancellation, and
  bounded timeouts.
- [x] Connect only through the loopback endpoint and reject remote, tunneled, or
  overlapping tool execution.
- [x] Discover the live catalog through `list_toolsets`, `describe_toolset`, and
  `call_tool`, plus eager `tools/list` mode when enabled.
- [x] Generate a deterministic machine-readable snapshot of every discovered
  toolset, tool, input schema, output schema, default, enum, and side effect.
- [x] Map every discovered tool to a generic lossless JSON terminal call.
- [x] Add typed CLI commands for the complete catalog without silently omitting
  difficult, experimental, destructive, or niche tools.
- [x] Populate `skills/unreal/` with complete command syntax, parameters,
  examples, required editor state, approval rules, errors, and troubleshooting.
- [x] Add catalog drift checks that fail when the selected engine adds, removes,
  renames, or changes a tool without a reviewed CLI and documentation update.
- [x] Black-box test server lifecycle, discovery, schemas, valid and invalid
  calls, errors, refresh, reconnection, serial execution, and automation tests.
- [x] Use the MCP Inspector as an independent UI and CLI reference client.
- [x] Preserve known editor safety failures as observable regressions instead of
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

**Status:** In progress. The native contract suite and character-first C++
foundation are established; bulk asset import remains pending.

**Executive result:** JSON, FBX, WAV, MOV, and normalized HAP cinematic evidence
become native Unreal assets and target-verified media variants through
deterministic conversion plans and the Phase 5 terminal MCP surface rather than
manual editor work.

The authoritative native asset, mission, world, vehicle, character, animation,
mod, networking, rendering, naming, folder, format, and validation contract lives
at [`docs/technical/pipeline/unreal/`](docs/technical/pipeline/unreal/index.md).
Pipeline work must implement that contract exactly before introducing a new native
asset family. The Unreal runtime is designed as a modern AAA game from first
principles; legacy layouts are migration evidence, never runtime architecture.
Characters are the first vertical slice because their normalized models are ready
and they exercise identity, materials, rigs, shared animation libraries, physics,
loading, selection, validation, and mod replacement.

`src/unreal` is the pipeline-owned planning library for this phase. It validates
normalized JSON, PCM WAV, MOV or HAP cinematic evidence, and binary FBX 7.7
artifacts and produces stable native target identities, dependencies, import
plans, and provenance. It never opens an MCP connection or controls an Unreal
process.

Mesh import is staged. Canonical FBX remains engine-independent source evidence,
while Phase 6 may generate destination UVs, rebake real textures, bind validated
optional normal or specular maps, create deterministic derived maps, refine
geometry through an approved recipe, decompose world geometry, and generate LOD
or HLOD assets before publishing final UAsset identities. These rules do not
change Phase 4 FBX generation.

Planned work:

- [ ] Generate a committed, public-safe Unreal import manifest from opaque
  package identifiers through the `src/unreal` conversion boundary.
- [ ] Apply conversion plans through tested native MCP commands from Phase 5.
- [ ] Import FBX files as Static Meshes, Skeletal Meshes, Skeletons, Physics
  Assets, Animation Sequences, materials, and textures.
- [ ] Import each compatible character animation once into the central shared
  rig-family library under `/Game/SHAR/Art/Characters/Animations`; never copy
  common clips into per-character folders.
- [ ] Convert camera-only packages such as `phonecamera` directly from normalized
  camera, controller, and animation evidence into native Unreal Camera Actors and
  Level Sequences without an intermediate FBX.
- [ ] Convert the 64-row `atc` destructible-object attribute table into a native
  Data Table or Data Asset preserving class name, sound, particle, animation,
  friction, mass, and elasticity fields without an intermediate FBX.
- [ ] Require real canonical texture evidence for every textured material slot
  and bind optional normal, specular, and related maps only when detection and
  native read-back validate their semantic role.
- [ ] Use an explicit neutral material value or no texture input when an optional
  map is absent; generate derived maps only through a deterministic recorded
  recipe.
- [ ] Stage mesh UAssets until destination UV generation, texture rebaking,
  material reconstruction, collision, and declared LOD validation pass.
- [ ] Keep geometry refinement and additional-vertex generation pending until a
  deterministic recipe proves silhouette, topology, skinning, collision, and
  animation preservation.
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
- [ ] Import each natural assembled world FBX as placement evidence, then split
  houses and every other shipping world component into stable native mesh
  identities and reconstruct them in one canonical map composition.
- [ ] Convert the approved world assembly into World Partition cells, Runtime Data
  Layers, streaming assets, collision data, and authored placement records.
- [ ] Generate component LOD and HLOD representations so required distant world
  geometry degrades through approved detail levels instead of arbitrary authored
  disappearance; native frustum, occlusion, and streaming culling remain valid.
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

- [Staged mesh import and world assembly](docs/adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)
- [Native import, material rebuild, and world assembly](docs/technical/unreal/native-import-material-and-world-assembly.md)
- [Native asset load request and streaming runtime](docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md)
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
- [Application lifecycle and mode runtime](docs/technical/unreal/application-lifecycle-and-mode-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend screen flow and settings runtime](docs/technical/unreal/frontend-screen-flow-and-settings-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Authored spatial placement and trigger runtime](docs/technical/unreal/authored-spatial-placement-and-trigger-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission definition, stage, and objective runtime](docs/technical/unreal/mission-definition-stage-and-objective-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission, interaction, interior, and notoriety runtime](docs/technical/unreal/mission-interaction-and-notoriety-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Mission world-entity and respawn runtime](docs/technical/unreal/mission-world-entity-and-respawn-runtime.md)
- [Pedestrian path runtime](docs/technical/unreal/pedestrian-path-runtime.md)
- [Presentation playback runtime](docs/technical/unreal/presentation-playback-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Native platform bootstrap and error-recovery runtime](docs/technical/unreal/native-platform-bootstrap-and-error-recovery-runtime.md)
- [Native asset load request and streaming runtime](docs/technical/unreal/native-asset-load-request-and-streaming-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Memory ownership, budget, and diagnostics runtime](docs/technical/unreal/memory-ownership-budget-and-diagnostics-runtime.md)
- [Developer command and diagnostic runtime](docs/technical/unreal/developer-command-and-diagnostic-runtime.md)
- [Device configuration and save-slot runtime](docs/technical/unreal/device-configuration-and-save-slot-runtime.md)
- [Legacy runtime identity normalization](docs/technical/unreal/legacy-runtime-identity-normalization.md)
- [Persistent world-object state runtime](docs/technical/unreal/persistent-world-object-state-runtime.md)
- [Semantic input, device, and haptics runtime](docs/technical/unreal/semantic-input-device-and-haptics-runtime.md)
- [Typed event and observation routing runtime](docs/technical/unreal/typed-event-and-observation-routing-runtime.md)
- [Portable save storage and lifecycle](docs/adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
- [Platform save storage and lifecycle](docs/technical/unreal/platform-save-storage-and-lifecycle.md)
- [Contextual interaction query and transaction boundary](docs/adr/unreal/runtime/contextual-interaction-query-and-transaction.md)
- [Contextual interaction runtime](docs/technical/unreal/contextual-interaction-runtime.md)
- [HUD, radar, camera, and navigation parity](docs/adr/unreal/ui/hud-radar-camera-and-navigation.md)
- [Camera system runtime](docs/technical/unreal/camera-system-runtime.md)
- [Camera rig, preset, and arbitration runtime](docs/technical/unreal/camera-rig-preset-and-arbitration-runtime.md)
- [Native flying-hazard actors and StateTree execution](docs/adr/unreal/runtime/native-flying-hazard-actors-and-state-trees.md)
- [Flying-hazard and projectile runtime](docs/technical/unreal/flying-hazard-and-projectile-runtime.md)
- [Typed StateTree action sequences](docs/adr/unreal/runtime/typed-state-tree-action-sequences.md)
- [Typed action-sequence runtime](docs/technical/unreal/typed-action-sequence-runtime.md)
- [Gameplay census, presentation, and development-content boundary](docs/technical/unreal/gameplay-census-presentation-and-development-boundary.md)
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
- Blender and Maya are optional experimental inspection aids only. They are not
  generation, conversion, staging, repair, validation, or acceptance
  dependencies.
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
- [ ] Verify every mission, chapter transition, vehicle, collectible, cinematic,
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

### Optional Latin American Spanish input

A user may explicitly configure an optional, lawfully obtained local `.lmlm`
archive as a Latin American Spanish override. No community-specific filename,
local route, version, or acquisition source is a public repository contract.

The archive is optional, is not distributed by this repository, and must pass
the bounded LMLM validation contract before use. Its absence must not prevent the
base installation from validating or building.

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

<!-- markdownlint-enable MD013 -->

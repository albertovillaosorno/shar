# AAA-native content and gameplay foundation

- Status: Accepted
- Decision date: 2026-07-18
- Scope: Unreal content, runtime architecture, pipeline handoff, modding, and
  self-hosted community networking

## Context

The conversion pipeline can produce normalized characters, vehicles, world data,
missions, media, and gameplay records, but native implementation cannot scale if
it treats the Unreal project as a thin port of a historical engine. Extreme
mods, future gameplay redesign, modern camera and rendering, new content,
platform profiles, and community-hosted multiplayer require stable domain
identity and native engine boundaries before bulk migration begins.

## Decision

SHAR is designed as a modern AAA Unreal game from first principles. Legacy data
is migration evidence only. The authoritative pipeline-facing contract lives
under `docs/technical/pipeline/unreal/` and defines every native content root,
package, name, schema, asset family, dependency, validation rule, and promotion
condition.

Runtime architecture is split into independently buildable domain modules around
a shared Primary Asset contract. Asset Manager identities, soft references,
versioned definitions, fixed bundle names, Gameplay Tags, Data Registries,
StateTree, Gameplay Ability System, Smart Objects, Mass Entity, Chaos, World
Partition, Common UI, Niagara, MetaSounds, and Game Features are used where
their native ownership matches the domain requirement. No gameplay behavior
depends on source filenames, mutable directory scans, concrete package paths,
historical manager arrays, or vendor-specific rendering technology.

The base campaign remains single-player. Runtime and content identities are
network-authority-ready, and the project supports user-operated dedicated or
listen servers for community sandbox and mod-defined modes. The project does not
operate a server fleet, hosted matchmaking, accounts, global discovery,
moderation, or persistence services.

Camera, controls, physics, animation, presentation, streaming, accessibility,
and rendering are modernized rather than numerically reproducing historical
technical behavior. Content identity and recognizable design intent remain
stable while the implementation may improve substantially.

TSR is the required Unreal-native temporal upscaler. Vendor integrations are
optional adapters selected through capability detection. They cannot become a
content requirement or gameplay dependency.

## Consequences

- The pipeline has one explicit target and can generate exact native assets
  rather than approximate editor imports.
- Characters are the first vertical slice because their converted evidence is
  ready and they exercise identity, mesh, rig, material, animation, physics,
  loading, validation, selection, and mod replacement boundaries.
- Vehicles and world content reuse the shared definition and publication model
  while retaining domain-specific modules and schemas.
- Refactors can replace concrete runtime classes or presentation assets without
  changing canonical save or network identity.
- Mods can add or replace namespaced content through reviewed extension points.
- Synthetic tracked fixtures prove import behavior without publishing private or
  extracted game assets.
- Up-front contracts are more demanding, but they remove undocumented manual
  assembly and prevent long-term technical debt.

## Rejected alternatives

- Building the game around historical file layout or engine managers.
- Importing assets first and deciding their runtime contract afterward.
- One monolithic gameplay module or one Blueprint per mission.
- Hardcoded arrays of characters, vehicles, missions, or mod slots.
- Runtime filename construction, directory scanning, or editor-state discovery.
- Making multiplayer depend on official hosted infrastructure.
- Requiring DLSS, FSR, or another vendor plugin for correctness.

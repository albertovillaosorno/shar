# Native game foundation

- Status: Active
- Last reviewed: 2026-07-18

## Product posture

SHAR is implemented as a modern Unreal Engine game designed from first
principles. The migration pipeline is responsible for translating lawful legacy
evidence into this architecture. The runtime does not preserve historical
technical limitations, custom engine subsystems, camera mathematics, object
managers, memory layouts, mission scripting syntax, renderer workarounds, or
platform restrictions.

Faithfulness means preserving recognizable intent, content identity,
progression, mission meaning, world geography, timing where required, and
player-facing tone. Implementation, presentation, responsiveness, accessibility,
streaming, camera, physics, animation, rendering, networking, and modding use
current native Unreal systems and may be substantially improved.

## Architectural rules

The game uses domain modules with explicit inward dependencies:

- `shar` is the minimal executable bootstrap;
- `SharContent` owns shared Primary Asset identity and loading contracts;
- `SharCharacters` owns character definitions and character construction ports;
- `SharVehicles` owns vehicle definitions and vehicle construction ports;
- `SharWorld` owns world definitions, placement, streaming, roads, and
  interiors;
- `SharMissions` owns mission definitions, execution policy, and progression;
- `SharCamera` owns camera intent, rigs, blending, and accessibility;
- `SharAudio` owns audio definitions, routing, and playback policy;
- `SharUI` owns Common UI screens, view models, and input presentation;
- `SharNetworking` owns replication-safe identities and self-hosted session
  ports;
- `SharModding` owns validated Game Feature overlays and extension transactions;
- editor import and validation code lives in separate editor-only modules.

A module may depend on `SharContent` and narrower shared domain contracts. It
may not reach sideways into another implementation to obtain state. Cross-domain
behavior uses typed interfaces, messages, observations, commands, and Primary
Asset identities.

## Native Unreal systems

The default architecture uses Unreal-native systems rather than parallel custom
engines:

- Asset Manager and Primary Data Assets for content identity and load bundles;
- Data Registries or generated Data Tables for large ordered row sets;
- Gameplay Tags for capabilities and classification, never identity;
- Enhanced Input for semantic input actions and remapping;
- StateTree for bounded mission, AI, interaction, and action orchestration;
- Gameplay Ability System for extensible attributes, abilities, costs,
  cooldowns, damage, status effects, and mod-added gameplay where replication is
  required;
- Smart Objects for world interactions such as phone booths and contextual use;
- Mass Entity for high-volume ambient population and traffic representations;
- Chaos for vehicles, rigid bodies, destruction policy, and physical simulation;
- World Partition, Data Layers, Level Instances, HLOD, Nanite, and native async
  loading for the connected world;
- Common UI and Model-View-ViewModel for platform-aware menus and HUD;
- Niagara for scalable VFX and MetaSounds or native Sound assets for audio;
- Game Features for validated, namespaced mod content overlays.

The presence of a native system does not make it domain authority. StateTree
does not own mission identity, GAS does not own save identity, Game Features do
not own package trust, and Asset Manager does not decide taxonomy.

## Refactorability contract

Gameplay and presentation must be replaceable without migrating canonical save
identity. A character, vehicle, mission, reward, location, ability, camera rig,
or world feature is addressed by a stable Primary Asset identity and revisioned
definition. Concrete classes, meshes, materials, animation Blueprints, physics
profiles, and UI widgets are soft dependencies behind that definition.

No gameplay rule may depend on:

- a concrete asset package path;
- a source filename or source ordinal;
- material slot position without a declared semantic role;
- an animation sequence name assembled at runtime;
- a bone name not resolved through a rig profile;
- an actor label, editor selection, or World Partition external-actor filename;
- fixed arrays of characters, vehicles, missions, or mod slots; or
- a vendor-specific renderer or networking service.

## Camera and feel

Camera behavior is intentionally modern rather than numerically identical to the
historical game. Character, vehicle, aiming, conversation, cinematic, interior,
and accessibility rigs consume semantic targets and data-driven tuning. The
camera system owns smoothing, collision, look-ahead, speed framing, field of
view, motion reduction, shoulder policy, and transitions. Mission or character
code requests camera intent; it never manipulates camera transforms directly.

## Multiplayer posture

The base campaign remains deterministic single-player. All runtime identities,
commands, state transitions, prediction-sensitive actions, and mod extension
points are designed so a user-operated dedicated or listen server can become
authoritative for community sandbox and mod-defined multiplayer modes.

The project may ship server targets, direct-connect and LAN-capable session
ports, and package compatibility checks. It does not operate servers, require
accounts, provide hosted matchmaking, maintain a global server browser, sell
server access, or promise moderation or uptime. Community operators own
deployment, discovery, rules, persistence, moderation, security, backups, and
support.

## Definition of done

A system is not considered native or scalable merely because it compiles. It
must have stable identities, typed boundaries, deterministic loading, failure
semantics, automation tests, data validation, mod extension behavior, save and
network implications, and a documented migration contract in this directory.

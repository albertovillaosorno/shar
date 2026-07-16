# Mod-owned multiplayer adapters and community servers

- Status: Accepted
- Decision date: 2026-07-15
- Scope: Base-game multiplayer boundary and mod-owned online extension support

## Context

The base product is a single-player open-sandbox campaign. It does not need an
official cooperative campaign, matchmaking service, hosted backend, account
system, or first-party server network.

The architecture should nevertheless avoid choices that make future multiplayer
mods impossible. Community authors may want to create custom multiplayer modes
and host independent servers in the same spirit as community-managed sandbox
servers. The server owner selects the mode and package set; players connect
using compatible local content. The base project does not operate the service.

A vague promise of multiplayer support would overstate current implementation.
The boundary must distinguish the base game, required extension seams, and
mod-owned future behavior.

## Decision

The base campaign remains single-player. No official multiplayer campaign mode,
cooperative story progression, matchmaking service, account service, hosted
server list, or first-party community server is part of the base product.

The base product may include a same-device local split-screen competitive
minigame. That session is locally authoritative, transient, and isolated from
campaign progression. Its local participants, controller assignments,
`ULocalPlayer` instances, viewports, and results are not network players, server
identities, replicated authority, or proof that campaign systems support online
play.

The repository must preserve a stable multiplayer-adapter extension boundary for
validated mods. That boundary may support mod-owned:

- dedicated or listen-server processes;
- custom multiplayer modes and maps;
- direct-address connection or a mod-provided server directory;
- authoritative session state and replication adapters;
- package-set and protocol compatibility negotiation;
- namespaced player, session, entity, event, and save identities;
- mod-owned administration, moderation, persistence, and achievements; and
- community-hosted servers whose operators choose the required package set.

The base project does not implement those modes now. It supplies documentation,
semantic identities, package declarations, adapter interfaces, validation rules,
and architecture seams that prevent the single-player runtime from becoming
unnecessarily hostile to a future multiplayer mod.

A multiplayer mod is a separate mode owned by its package. It cannot silently
turn the base campaign into networked play, mutate portable single-player
progression, grant base achievements, or reuse a local save as authoritative
server state without an explicit validated policy.

The server is authoritative for the mod-owned session. The base single-player
world remains locally authoritative when no multiplayer adapter is active.

Clients install and trust required packages locally before joining. A server
cannot automatically deliver or execute untrusted native code, replace local
packages without review, or bypass package validation.

A server and client negotiate exact protocol, runtime, content-catalog,
package-set, gameplay-schema, and mod revisions before admission. Incompatible
clients receive a typed explanation and do not join partially.

The project may expose adapter contracts for session creation, discovery,
connection, serialization, replication, authority, prediction, reconciliation,
identity, diagnostics, and teardown. It does not guarantee that every existing
single-player system is network-ready or replicated.

## Current behavior

- The shipped base campaign is single-player.
- Existing local split-screen bonus behavior remains separate from networked
  multiplayer.
- No official server executable, server browser, matchmaking backend, account
  service, or multiplayer campaign is currently implemented.
- Base saves and achievements remain single-player authority.

## Required architecture support

- Canonical identities and schemas must be serializable without pointer or load
  order authority.
- Domain commands, observations, and results must expose enough stable metadata
  for a future adapter to transport them safely.
- Mod manifests must be able to declare multiplayer capabilities, protocol
  revision, server/client roles, required package set, native-code trust, save
  policy, achievement policy, and teardown behavior.
- Runtime services must not assume process-global mutable singletons are the
  only possible authority boundary.
- Package validation must reject incompatible or undeclared network behavior.
- Multiplayer adapter activation and removal must be atomic and leave the
  single-player runtime recoverable.

## Deferred mod-owned extensibility

The following are deliberately not base deliverables:

- campaign cooperation or competitive campaign play;
- official game servers;
- official matchmaking or server discovery;
- anti-cheat service operation;
- account, friends, chat, voice, moderation, or social services;
- cloud-hosted progression;
- automatic mod download from servers; and
- certification of third-party multiplayer code or server operators.

Mod authors may implement these within validated package and trust boundaries.
The repository facilitates the work but does not promise to build or operate it.

## Consequences

- Documentation no longer describes multiplayer as architecturally forbidden.
- The base campaign remains finite, offline-capable, and single-player.
- Community servers are independent deployments owned by their operators.
- Multiplayer mods receive explicit package, protocol, identity, authority, and
  lifecycle seams.
- Base progression and achievements remain protected from untrusted server
  state.
- Network features cannot become a required dependency for ordinary play,
  conversion, validation, or packaging.

## Rejected alternatives

- Shipping an official cooperative campaign now.
- Treating every base subsystem as already network-replicated.
- Operating a first-party hosted server or matchmaking platform.
- Architecturally prohibiting all future multiplayer mods.
- Allowing a server to push and execute unreviewed packages automatically.
- Reusing ordinary single-player saves as server authority without a declared
  migration and ownership model.

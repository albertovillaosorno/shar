# Multiplayer adapter and community-server extension

- Status: Architecture support required; base implementation deferred
- Last reviewed: 2026-07-15

<!-- markdownlint-disable MD013 -->

## Governing decisions

- [Mod-owned multiplayer adapters and community servers](../../adr/modding/mod-owned-multiplayer-adapters-and-community-servers.md)
- [Local drop-in mod packages and AI skills](../../adr/modding/drop-in-mod-packages-and-ai-skills.md)
- [Local mod trust and distribution boundary](../../adr/modding/mod-safety-scanner-and-distribution.md)
- [Open sandbox chapters and world progression](../../adr/gameplay/open-sandbox-chapters-and-world-progression.md)

## Purpose

This specification defines the extension surface that permits a validated mod to
provide multiplayer modes and community-hosted servers without claiming that the
base campaign already supports networked play.

It separates:

| Layer | Contract |
| :--- | :--- |
| Current base behavior | Single-player campaign and local-only authority. |
| Required architecture support | Stable identities, schemas, package declarations, adapter ports, validation, and teardown seams. |
| Deferred mod implementation | Transport, replication, server process, discovery, administration, and custom multiplayer mode behavior. |

## Base boundary

The base game ships no official multiplayer campaign, cooperative story mode,
matchmaking service, account system, hosted server directory, or first-party
community server.

A multiplayer package creates a separate mod-owned mode. It may reference
base-game content through canonical identities, but it cannot silently replace
the ordinary campaign state machine or convert a single-player save into server
authority.

The base game remains fully playable, saveable, verifiable, and packageable with
no network connection.

## Extension topology

A multiplayer-capable package may provide adapters for:

- session definition and lifecycle;
- dedicated or listen-server process roles;
- transport and connection establishment;
- direct-address join or package-owned server discovery;
- protocol and package-set negotiation;
- player admission and removal;
- authoritative command acceptance;
- replicated state and observation delivery;
- optional prediction and reconciliation;
- mod-owned persistence and administration;
- custom achievement projection; and
- terminal disconnect and teardown.

The base project defines ports and schemas only where needed to preserve a clean
extension boundary. It does not provide a complete transport or replication
implementation in this phase.

## Package declaration

A multiplayer package declaration includes:

| Field | Contract |
| :--- | :--- |
| `MultiplayerModeId` | Stable namespaced mode identity. |
| `ProtocolId` | Stable protocol family identity. |
| `ProtocolRevision` | Exact compatible protocol revision. |
| `AuthorityModel` | Dedicated server, listen server, or another declared model. |
| `ClientRoles` | Supported player, spectator, administrator, or custom roles. |
| `ServerTargets` | Exact supported platform and architecture targets. |
| `RequiredPackages` | Deterministic package and revision closure. |
| `RequiredCatalogRevision` | Exact compatible gameplay and content schema. |
| `NativeCodePolicy` | Content-only or explicitly trusted native code. |
| `SavePolicy` | None, ephemeral session, or namespaced mod-owned persistence. |
| `AchievementPolicy` | Base-compatible, base-incompatible, or custom provider. |
| `DiscoveryPolicy` | Direct address or mod-owned directory adapter. |
| `TeardownPolicy` | Required cleanup and recovery behavior. |

Missing or ambiguous declarations reject activation.

## Community-server model

Community operators host independent servers and choose their mode,
configuration, and required package set. The project does not operate, endorse,
moderate, or vouch for those servers.

A server may publish connection metadata such as address, mode identity, protocol
revision, package-set digest, player count, and human-readable description. It
must not claim that connection metadata is package trust evidence.

Players install and validate required packages locally before joining. Joining a
server never authorizes automatic native-code download or execution.

## Compatibility handshake

Before admission, client and server compare:

- protocol identity and revision;
- runtime contract revision;
- platform and architecture support;
- active package identities, versions, and hashes;
- gameplay and event-schema revisions;
- content-catalog revision;
- required capabilities;
- authority and prediction policy;
- save and achievement policy; and
- optional mod-defined compatibility fields.

Any required mismatch rejects admission before world mutation. The result names
the incompatible field and expected revision.

## Identity and serialization

Transported values use canonical namespaced identities and bounded schemas.
Pointer addresses, actor load order, array positions, source enum ordinals,
filesystem paths, and display names are never network authority.

Every serialized message declares schema identity, revision, session identity,
source role, sequence or transaction identity, bounds, and validation policy.
Unknown required schemas fail closed.

## Authority

The active multiplayer adapter declares one authoritative session role. In the
ordinary community-server model, the server accepts commands and publishes
accepted state or observations.

Clients cannot commit portable base progression, base achievements, permanent
world state, economy changes, or campaign completion from server observations.
A mod may own namespaced session or server persistence through its declared save
policy.

When no multiplayer adapter is active, the local single-player world remains the
only authority. Multiplayer support cannot add latency, network dependencies, or
failure modes to ordinary offline play.

## Base campaign isolation

The base campaign does not become a multiplayer mode merely because an adapter is
installed. A package that wants cooperative or competitive story behavior must
define a separate namespaced mod mode, explicit mission semantics, authority,
checkpoint, save, achievement, and failure policies.

Such a mode is not certified as campaign parity by the base project and cannot
write ordinary campaign progress unless a future accepted decision explicitly
allows it.

## Save and persistence

Multiplayer modes use one of these policies:

- no persistence;
- ephemeral session persistence owned by the current server; or
- namespaced mod-owned durable persistence with an explicit schema and migration.

Base save slots remain separate. A server cannot overwrite, merge, or reinterpret
a base save slot. Export or import between a mod save and base save requires a
separate reviewed converter and is not implied by shared content identities.

## Achievements

Base achievement progress is suspended unless the package has a validated
base-compatible policy for the exact multiplayer behavior. The default for a
multiplayer package is base-incompatible.

Package-owned achievements use namespaced identities and mod-owned progress.
Server operators cannot grant base achievements by sending arbitrary counters or
completion messages.

## Discovery and frontend integration

The base frontend exposes no multiplayer command by default. A validated active
package may register a namespaced menu route for its mode, direct-connect screen,
or server directory.

The route must identify the providing package, trust state, required native code,
network risk, active package set, and whether base achievements are suspended.
Removing the package removes the route and restores the ordinary frontend.

## Security and trust

Network input is untrusted. Adapters validate message type, length, rate, role,
session identity, schema revision, sequencing, and authorization before domain
use.

The architecture does not promise safety for arbitrary third-party native code,
servers, administrators, or package sources. Native multiplayer packages require
explicit local trust under the ordinary mod safety policy.

A server cannot:

- bypass local package validation;
- push unreviewed executable code;
- access unrelated local files or saves through the adapter contract;
- claim official project endorsement;
- mutate base progression without an accepted future policy; or
- keep adapter callbacks active after disconnect or package removal.

## Lifecycle and teardown

Activation is atomic. The adapter acquires explicit handles for transport,
session, world, input, event, persistence, and frontend integration.

Disconnect, server failure, suspension, mode exit, package deactivation, process
shutdown, and validation failure release every handle. Late network callbacks are
ignored by session and adapter revision.

A failed activation or disconnect returns to a stable frontend or mod-defined
recovery state. It cannot corrupt the ordinary single-player world or save.

## Mods and server packages

A server package may be distributed separately from a client package when both
declare one compatible protocol and package closure. Server-only native binaries
must name exact targets and trust requirements.

Different community servers may run different package sets. The package-set
digest and exact revisions define compatibility; a display server name does not.

## Diagnostics

Development diagnostics record mode, session, protocol, package-set digest,
roles, connection state, message counts, schema failures, authority rejections,
latency observations, teardown, and stale callbacks.

Diagnostics redact addresses, credentials, authentication tokens, chat content,
and other private data according to policy.

## Failure behavior

Activation or admission fails closed on:

- unknown mode, protocol, role, or adapter identity;
- incompatible runtime, package, catalog, or schema revision;
- undeclared native code or target;
- invalid authority or save policy;
- malformed, oversized, stale, duplicated, or unauthorized message;
- untrusted package execution request;
- base-save or base-achievement mutation attempt;
- incomplete teardown; or
- network dependence introduced into ordinary offline play.

## Verification

Required architecture tests include:

- package declaration parsing and compatibility;
- deterministic package-set digest construction;
- client/server handshake acceptance and rejection;
- canonical identity and schema serialization;
- malformed and oversized message rejection;
- authority and role enforcement;
- base campaign, save, and achievement isolation;
- package-owned persistence namespaces;
- frontend route registration and removal;
- disconnect, suspension, failure, and late-callback teardown;
- offline base-game behavior with no adapter present; and
- two community servers using different deterministic package sets.

These tests validate the extension boundary. They do not claim that a complete
multiplayer transport or mode ships in the base product.

## Invariants

- The base campaign remains single-player.
- Multiplayer modes are package-owned and namespaced.
- Community servers are independently hosted and operated.
- Network play is optional and never required for base-game operation.
- Clients validate and trust packages locally before joining.
- Server authority cannot mutate ordinary base saves or achievements.
- Exact protocol and package revisions define compatibility.
- Adapter removal restores a clean single-player runtime.
- Architecture support is not an implementation claim.

<!-- markdownlint-enable MD013 -->

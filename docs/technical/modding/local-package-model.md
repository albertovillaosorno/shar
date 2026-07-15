# Local mod package model

- Status: Active
- Last reviewed: 2026-07-13

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Local drop-in mod packages and AI skills](../../adr/modding/drop-in-mod-packages-and-ai-skills.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)

## Purpose

This specification defines the repository-owned package declaration, transport,
platform import, normalized member identity, compatibility, activation, and
rollback model for local mods without a hosted service.

## Package declaration

A package declaration contains at least:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `CanonicalId` | Stable lowercase package identity independent of storage. |
| `ContractVersion` | Exact package-schema contract understood by the validator. |
| `PackageRevision` | Deterministic revision of this package identity. |
| `PackageKind` | `content` or `native`; mixed packages use the stricter native boundary. |
| `Priority` | Explicit signed ordering value; never inferred from discovery order. |
| `Dependencies` | Canonical package identities and accepted revision constraints. |
| `Conflicts` | Canonical package identities that cannot be active together. |
| `Supersedes` | Explicit replacement relationships. |
| `RequiredCapabilities` | Gameplay, asset, runtime, and platform capabilities required for activation. |
| `SupportedTargets` | Exact target identifiers required for native code or target-specific content. |
| `Members` | Normalized member identities, lengths, hashes, media types, and roles. |
| `Provenance` | Authorship, source, license, generation, and review evidence. |
| `TrustLevel` | Declared trust boundary used by validation and activation policy. |

<!-- markdownlint-enable MD013 -->

Package identity is the canonical declaration identity and revision. A filename,
archive name, physical directory, document-provider URI, import timestamp, or
discovery order never becomes package identity.

## Transport and platform import

A transport archive or selected document is untrusted input. Import first copies
or streams it into an isolated staging area, parses the declaration, normalizes
members, verifies lengths and hashes, and produces a dry-run preview. The active
package set is unchanged until validation succeeds.

Desktop adapters may discover packages through a user-visible local import root.
Android uses a native document-selection or equivalent platform import adapter
and copies accepted packages into application-owned storage. Android does not
require a public directory and does not activate a package directly from an
external provider URI.

Every import route converges on the same normalized package model. Moving or
reimporting an equivalent package does not change identity, priority,
dependencies, conflicts, or activation result.

## Member identity and path normalization

Archive and package member identities use one canonical separator, deterministic
Unicode normalization, and exact case rules defined by the package contract.
Validation rejects:

- absolute paths;
- drive-qualified or platform-rooted paths;
- parent traversal;
- empty, dot, or ambiguous segments;
- members that collide after case or Unicode normalization;
- duplicate normalized members;
- reserved control names;
- links or indirections that escape staging storage; and
- archive metadata that changes meaning by operating system.

Member order in an archive is not load order. The declaration and dependency
graph are the only ordering authorities.

## Portability and native code

A content-only package is portable when every required capability and referenced
canonical target is supported by the active game package. Platform adapters may
choose different native presentation implementations without changing the mod's
logical targets.

A native package declares every supported operating-system and architecture
target using the canonical target identifiers. Its binaries are separate members
with exact target, ABI, runtime, length, and hash metadata. A binary never loads
on an undeclared target, and a successful content validation does not imply that
native code is safe.

Android may reject native packages entirely until a separate accepted trust and
loading implementation exists. That rejection does not affect content-only
package portability.

## Activation model

Validation computes one deterministic activation plan from canonical package
identity, explicit priority, dependencies, supersession, conflicts,
capabilities,
and trust policy. The plan is previewed before mutation.

Activation creates a complete candidate active-set revision. The runtime
validates
its dependency closure and resolved gameplay targets before atomically replacing
the prior accepted active set. Failure preserves the previous revision and never
leaves a partially active package graph.

Removing or updating a package follows the same candidate-plan boundary. Saved
progression referencing unavailable content remains governed by the portable
save and gameplay catalog contracts; package removal does not silently delete
saved identities.

## Invariants

- Package identity is independent of storage location and import route.
- Desktop and Android imports produce the same normalized declaration and member
  identities for equivalent input.
- Explicit priority and dependency topology determine load order.
- Case, Unicode, separator, and archive-entry differences cannot create
  platform-
  specific package meaning.
- Content-only portability never grants native-code portability.
- A target-specific member is selected only for its exact declared target.
- Preview and validation complete before activation mutates local state.
- Activation, update, and removal replace one complete active-set revision.
- Missing licensed inputs are requested rather than invented.
- Native code remains an explicit trust boundary that static package checks
  cannot make safe.

## Failure behavior

The package remains inactive on:

- invalid or unsupported schema;
- missing provenance or integrity evidence required by policy;
- normalized member collisions or unsafe paths;
- incomplete, duplicate, or hash-mismatched members;
- missing references or incompatible capabilities;
- dependency cycles, unresolved conflicts, or ambiguous supersession;
- nondeterministic ordering;
- undeclared operating-system, architecture, ABI, or runtime requirements;
- unavailable native-code trust support;
- failed staging, storage, or candidate active-set replacement; or
- a dry-run result that differs from the computed activation plan.

Failure returns a typed diagnostic and preserves the previous accepted package
and activation state.

## Verification

- Equivalent desktop-directory and Android-managed imports produce identical
  normalized package records.
- Path fixtures cover separators, case collisions, Unicode collisions,
  traversal,
  absolute paths, links, duplicates, and archive-order variation.
- Topology tests cover priority, dependencies, conflicts, supersession, cycles,
  and deterministic ordering.
- Target tests cover every canonical desktop and Android target identifier and
  reject undeclared native binaries.
- Content-only packages produce equivalent logical changes across supported
  targets when required capabilities exist.
- Preview tests compare declared, normalized, and computed changes.
- Fault injection interrupts staging and every activation replacement stage and
  proves preservation of the prior accepted revision.
- Removal and update tests preserve saved canonical identities when referenced
  content becomes unavailable.

## Known limits

This specification does not create a hosted mod service, marketplace, remote
code-delivery system, arbitrary native-code safety guarantee, or automatic
cross-device package synchronization.

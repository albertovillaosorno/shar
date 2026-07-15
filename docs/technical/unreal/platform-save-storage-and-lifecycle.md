# Platform save storage and lifecycle

- Status: Active
- Last reviewed: 2026-07-14

## Governing decisions

- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
- [Data-driven Unreal gameplay content catalog](../../adr/unreal/runtime/data-driven-gameplay-content-catalog.md)

## Purpose

This specification defines the repository-owned save document, transaction,
migration, platform-storage adapter, and lifecycle behavior required to preserve
progression across supported operating systems and processor architectures.

Logical slot policy, device-local settings, slot summaries, storage-provider
readiness, quota, delete, reset, and user remediation follow the
[device configuration and save-slot runtime](device-configuration-and-save-slot-runtime.md).
Persistent destructible and consumable placement state follows the
[persistent world-object state runtime](persistent-world-object-state-runtime.md).

## Repository model

The gameplay domain produces an immutable logical save snapshot through a typed
save port. The snapshot contains schema-defined values only. A platform storage
adapter serializes, stages, validates, and commits that snapshot using the native
storage and lifecycle facilities of the selected target.

A logical save document contains at least:

| Field | Contract |
| :--- | :--- |
| `SchemaVersion` | Exact version of the logical save schema. |
| `RevisionToken` | Monotonic accepted revision identity for the logical slot. |
| `CatalogRevision` | Gameplay catalog revision required to resolve identities. |
| `ContentRequirements` | Canonical required base and mod content identities. |
| `ProgressionState` | Levels, missions, collectibles, rewards, vehicles, costumes, and bonus-state records. |
| `ResumeState` | Optional resumable mission and step identity recorded only at a valid checkpoint. |
| `IntegrityRecord` | Deterministic length and checksum evidence for the complete serialized candidate. |

A document never contains a native pointer, object address, compiler layout,
platform path, drive letter, mount point, package filename, localized display
name, or graphics-preset implementation detail as authority.

Schema-defined scalar values have explicit widths, ranges, and normalization
rules. Canonical identifiers use the gameplay catalog identity contract. Text
needed by the save schema uses a deterministic Unicode normalization rule.
Collections have deterministic key and element ordering before serialization.
The serializer does not expose host byte order or native struct padding.

## Portable and device-local state

Portable gameplay state includes progression, unlocks, canonical content
identities, valid checkpoint state, and migration metadata.

Device-local configuration includes:

- graphics preset and resolved quality groups;
- display mode, resolution, refresh selection, and frame pacing;
- hardware and vendor feature selection;
- keyboard, mouse, gamepad, and touch presentation mappings;
- mobile safe-area and display-cutout calibration;
- local accessibility presentation that depends on the active device; and
- platform storage routes and account-container identifiers.

Device-local configuration may be reset without changing gameplay progression.
A platform transfer imports the portable document only unless a later schema
explicitly defines portable user preferences.

## Save transaction

Each logical save operation follows this state machine:

1. capture an immutable domain snapshot at an accepted save boundary;
1. validate canonical identities, ranges, required content, and schema version;
1. serialize the complete candidate deterministically;
1. compute and append the integrity record;
1. write the candidate to a staging revision owned by the platform adapter;
1. durably flush the candidate where the platform exposes that capability;
1. read back and validate the complete staged revision;
1. atomically replace the accepted revision, or use an equivalent journaled or
   two-revision commit when native atomic replacement is unavailable;
1. record success only after the replacement revision is readable and valid; and
1. retain or clean staging evidence according to the typed result.

The accepted revision is never modified in place. A failed or interrupted
candidate cannot replace it. Recovery selects the newest complete accepted
revision and ignores uncommitted staging data.

## Schema migration

Migration is an ordered chain from one recognized schema version to the next.
Each step is deterministic, idempotent, and independently validated. Migration
uses canonical identities and explicit redirect maps; it does not guess from
display names, object paths, or filenames.

Before migration replaces an accepted revision, the original revision remains
available for recovery. An unknown future schema, missing migration step,
missing required content, alias cycle, invalid redirect, or checksum failure
returns a typed diagnostic and preserves the original save.

A content requirement that is temporarily unavailable does not erase its state.
The loader reports the unavailable identity and allows only behavior explicitly
permitted by the governing gameplay and mod contracts.

## Platform adapters

Windows, Linux, macOS, and Android adapters select native application storage
locations and permission models. Physical routes are never embedded in the
portable document or exposed as gameplay identity.

Adapters normalize differences in case sensitivity, path separators, file-lock
behavior, replacement semantics, storage quotas, and process lifecycle. They
must not normalize two distinct logical save-slot identities into one physical
record.

Support for x64 and ARM64 uses the same golden serialized documents. A package
cannot claim architecture support when it can write a save that another
supported architecture cannot read with the same logical result.

## Android lifecycle

Android foreground, background, suspension, low-memory, and termination signals
are translated into typed lifecycle events. A lifecycle event may request a
bounded flush of a snapshot that already represents committed gameplay state.
It cannot advance a mission, synthesize a checkpoint, skip schema validation, or
block indefinitely.

When the available lifecycle window is insufficient, the adapter abandons the
uncommitted candidate, preserves the last accepted revision, and records a
recoverable incomplete-flush result. Restart loads the last complete accepted
revision and never treats a staging file as successful progress.

## Invariants

- Every supported target uses one logical gameplay-save schema.
- x64 and ARM64 produce and consume logically equivalent documents.
- The accepted revision is never overwritten in place.
- A success result means the committed revision was read back and validated.
- Failed writes, migrations, permissions, lifecycle flushes, and checksums retain
  the last accepted revision.
- Portable progression never depends on native paths, pointer width, byte order,
  display names, or graphics settings.
- Device-local configuration cannot mutate portable gameplay state.
- Missing content is reported explicitly and never silently removed.
- Cloud synchronization and account transfer are absent unless separately
  authorized and implemented.

## Failure behavior

The save operation fails closed on:

- unsupported or future schema versions;
- missing or non-idempotent migration steps;
- invalid canonical identities or redirect cycles;
- out-of-range values or nondeterministic collection ordering;
- incomplete writes, short reads, checksum mismatches, or stale revision tokens;
- storage denial, quota exhaustion, replacement failure, or unavailable durable
  flush behavior required by the adapter contract;
- collision between logical slots after platform path normalization;
- architecture-dependent output for an equivalent snapshot;
- Android lifecycle expiration before commit completion; or
- an attempt to include device-local implementation state in portable
  progression.

Failure returns a typed result and leaves no misleading success marker.

## Verification

- Golden save documents generated from equivalent snapshots compare byte-for-byte
  across supported x64 and ARM64 test environments.
- Every supported target reads the same golden documents into equivalent domain
  snapshots.
- Migration fixtures cover every supported source version, repeated migration,
  unknown future versions, missing steps, and invalid redirects.
- Fault injection interrupts every transaction stage and proves that recovery
  selects the last accepted revision.
- Storage tests cover case-sensitive and case-insensitive behavior, permission
  denial, quota exhaustion, short writes, stale staging data, and replacement
  failure.
- Android tests terminate or background the process before staging, during
  write, before replacement, and after replacement.
- Catalog and mod tests prove that unavailable content is diagnosed without
  deleting saved progression.
- Device-local graphics and input changes leave the portable gameplay document
  unchanged.
- Native package tests verify save, load, restart, migration, and clean shutdown
  on every claimed platform and architecture.

## Known limits

This specification does not define cloud synchronization, platform-account
roaming, conflict resolution between independently advanced devices, console
storage, or a public save-file interchange promise. Those capabilities require
separate decisions and validation.

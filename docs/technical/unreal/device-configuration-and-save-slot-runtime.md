# Device configuration and save-slot runtime

- Status: Active
- Last reviewed: 2026-07-15

## Governing decisions and specifications

<!-- markdownlint-disable-next-line MD013 -->
- [Portable save storage and lifecycle](../../adr/unreal/runtime/portable-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Common UI front end and progress projection](../../adr/unreal/ui/common-ui-frontend-and-progress-projection.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Shared runtime tagging, modding, and platform compatibility](../../adr/unreal/runtime/shared-runtime-tagging-modding-and-platform-compatibility.md)
- [Platform save storage and lifecycle](platform-save-storage-and-lifecycle.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Application lifecycle and mode runtime](application-lifecycle-and-mode-runtime.md)

## Purpose

This specification defines typed device-local configuration, save-slot identity,
slot summaries, storage-provider discovery, asynchronous save operations,
delete and reset behavior, quota checks, and user-facing results.

It replaces hand-written text configuration parsing, fixed handler arrays, one
packed process-wide save blob, compile-time slot counts, raw drive pointers,
platform memory-card state machines, application-controlled device formatting,
and callback-order completion with native schemas and transaction handles.

## Ownership

<!-- markdownlint-disable MD013 -->

| Authority | Responsibility |
| :--- | :--- |
| Configuration schema | Device-local setting identities, types, bounds, defaults, and migration. |
| Configuration repository | Load, validate, stage, commit, reset, and observe local settings. |
| Save repository | Logical slot lookup, summary reads, portable snapshot transactions, and recovery. |
| Platform storage adapter | Native container, quota, permission, lifecycle, and atomicity behavior. |
| Profile service | Active local profile and account-container identity. |
| Front-end UI | Slot selection, confirmation, progress, failure, and recovery presentation. |
| Gameplay domains | Immutable portable snapshot and accepted checkpoint metadata. |

<!-- markdownlint-enable MD013 -->

Neither configuration nor slot presentation owns gameplay progression. The save
repository cannot apply a snapshot until all owning domains validate it.

## Runtime topology

The platform module owns these C++ types:

<!-- markdownlint-disable MD013 -->

| Type | Responsibility |
| :--- | :--- |
| `USharDeviceConfigurationSchema` | Immutable setting definitions and migration chain. |
| `USharDeviceConfigurationSubsystem` | Read model, staged edits, commit, reset, and observers. |
| `ISharDeviceConfigurationRepository` | Platform-local serialized configuration port. |
| `USharSaveSlotPolicy` | Product slot identities, visibility, ordering, and creation rules. |
| `USharSaveRepositorySubsystem` | Slot summaries, load, save, delete, recovery, and active handles. |
| `ISharPlatformStorageAdapter` | Native account container, quota, permissions, and durable operations. |
| `FSharSaveSlotId` | Stable logical profile and slot identity. |
| `FSharSaveSlotSummary` | Validated display projection for one accepted save revision. |
| `FSharStorageProviderState` | Availability, readiness, quota, permissions, and remediation. |
| `FSharSaveOperationHandle` | Move-only asynchronous operation and cancellation handle. |
| `FSharSaveOperationResult` | Closed terminal status with verified resulting revision. |

<!-- markdownlint-enable MD013 -->

Active operations belong to one game instance and profile. Platform storage
objects are hidden behind the adapter and never become gameplay identity.

## Device-configuration schema

Every setting definition contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `SettingId` | Stable canonical identity. |
| `ValueType` | Boolean, bounded integer, finite scalar, closed enum, or validated binding. |
| `DefaultValue` | Platform-neutral default or explicit adapter-resolved default. |
| `Bounds` | Closed range, allowed set, and normalization rules. |
| `Portability` | Device-local or explicitly portable preference. |
| `RestartPolicy` | Immediate, next world, renderer restart, or application restart. |
| `Availability` | Platform, hardware, input, display, and package capability predicate. |
| `Migration` | Ordered converters from recognized prior revisions. |
| `Presentation` | Localizable label, description, category, and accessibility metadata. |

<!-- markdownlint-enable MD013 -->

A setting name, section heading, text line, registration order, or handler
pointer
is not identity authority.

## Configuration document

The configuration repository stores one versioned document per local device and
profile scope selected by policy. The document contains:

- schema version;
- committed revision token;
- normalized setting map;
- capability evidence needed to explain resolved values;
- migration diagnostics; and
- deterministic integrity evidence.

Unknown future settings are preserved only when the schema explicitly supports
round-tripping. Unknown required sections, duplicate keys, invalid Unicode,
unsupported values, and type mismatches fail validation.

The repository may use a platform-native settings service or a deterministic
serialized document. Runtime code does not parse arbitrary section-and-property
text or depend on a fixed buffer size.

## Configuration load and defaulting

Configuration load returns one result:

<!-- markdownlint-disable MD013 -->

| Result | Meaning |
| :--- | :--- |
| `loaded` | Current schema read and validated. |
| `migrated` | Recognized prior schema converted and committed. |
| `defaulted` | No accepted configuration existed and defaults were committed. |
| `quarantined` | Invalid local data was preserved for diagnostics before defaults. |
| `failed` | No valid configuration or safe default commit was possible. |

<!-- markdownlint-enable MD013 -->

Defaulting never changes portable gameplay state. A hardware capability change
may alter the resolved presentation value without rewriting the user's requested
value unless policy explicitly requires normalization.

## Configuration edit transaction

Front-end settings edits use a staged transaction:

1. read the current committed revision;
1. create an isolated candidate setting map;
1. validate types, bounds, availability, and conflicts;
1. preview resolved engine effects;
1. apply reversible immediate changes to a test lease when permitted;
1. serialize and stage the complete candidate;
1. read back and validate the staged document;
1. atomically commit the candidate;
1. apply the committed engine projection; and
1. publish one terminal result.

Cancel restores the committed projection. A renderer or application restart
requirement is reported explicitly and never simulated by silently ignoring the
new value.

## Frontend settings edit sessions

The frontend opens one `FSharSettingsEditSession` against the accepted device
configuration revision. Category widgets edit a typed draft and cannot write
individual fields or platform state directly.

Input binding, sensitivity, inversion, haptics, audio, display, graphics,
language, accessibility, and presentation fields declare schema, range, default,
compatibility, preview, and persistence policy. Commit validates and applies the
complete draft atomically; cancellation restores every scoped preview.

Risky display changes use a temporary platform preview, monotonic confirmation
deadline, and known-safe rollback journal. Reject, timeout, focus loss,
suspension, adapter loss, failed verification, or interrupted startup restores
the accepted safe mode before normal frontend presentation.

The detailed capture, preview, commit, and recovery sequence follows the
<!-- markdownlint-disable-next-line MD013 -->
[frontend screen flow and settings runtime](frontend-screen-flow-and-settings-runtime.md).

## Logical save slots

Save slots use stable logical identities, for example `profile_primary_slot_01`.
Slot identity does not depend on array position, platform filename, drive index,
or display order.

`USharSaveSlotPolicy` declares:

- visible manual slots;
- optional autosave or checkpoint slots;
- creation and overwrite rules;
- ordering and default selection;
- profile and account scope;
- platform capability limits; and
- presentation labels.

A supported platform may expose a different physical storage model, but it
cannot
silently reinterpret one logical slot as another. Product slot count is a
validated policy, not a compile-time branch.

## Save-slot summary

A slot summary is derived from the accepted portable document and platform
commit
record. It contains:

<!-- markdownlint-disable MD013 -->

| Field | Contract |
| :--- | :--- |
| `SlotId` | Stable logical slot identity. |
| `RevisionToken` | Accepted portable revision. |
| `SchemaVersion` | Logical save schema version. |
| `TimestampUtc` | Adapter-normalized accepted commit timestamp. |
| `Checkpoint` | Optional canonical level, mission, and checkpoint identities. |
| `ProgressProjection` | Read-only completion summary from the campaign service. |
| `PlayTime` | Optional validated accumulated duration. |
| `ContentStatus` | Required content available, optional missing, or blocked. |
| `IntegrityStatus` | Verified, recoverable prior revision, or invalid. |
| `Presentation` | Localizable derived label and thumbnail identity. |

<!-- markdownlint-enable MD013 -->

Localized display strings are generated after validation. They are never stored
as checkpoint identity.

The most-recent slot is selected by accepted revision time, then revision token,
then stable slot identity. Invalid local clock data cannot reorder accepted
revisions without a deterministic fallback.

## Save composition

Gameplay systems contribute schema-owned sections through typed snapshot ports.
The save coordinator gathers immutable section snapshots under one shared domain
revision.

A section definition declares:

- canonical section identity;
- schema and migration owner;
- deterministic serializer;
- validation and default rules;
- required content dependencies; and
- participation in portable or device-local state.

Registration order and byte count cannot define the serialized layout. Missing a
required section fails snapshot capture. Optional sections use explicit
defaults.

## Save, load, and delete operations

Save and load use the transaction and migration rules in the platform save
specification. This companion adds slot and UI behavior.

Every operation records:

- operation identity;
- profile and slot identity;
- expected accepted revision;
- platform-container revision;
- start and terminal timestamps;
- cancellation and timeout policy; and
- verified resulting slot state.

Delete is a separate confirmed transaction. It removes the selected accepted
slot
and its staging records without affecting another slot or device configuration.
A failed delete reports the still-readable accepted state.

Reset-gameplay creates a new portable snapshot through domain reset ports. It is
not implemented by clearing an arbitrary memory buffer.

## Storage-provider state

The platform adapter projects storage through `FSharStorageProviderState`:

- provider and account-container identity;
- available, unavailable, or temporarily unavailable state;
- readable and writable capability;
- free and required quota when meaningful;
- permission or account requirement;
- durable replacement capability;
- active operation identity; and
- typed user remediation.

Desktop paths, Android application storage, and platform account containers are
adapter details. The UI does not expose drive letters, mount points, or raw
provider pointers as portable identity.

## Quota and creation size

Before creating or replacing a slot, the adapter computes a conservative
required
size for:

- staged candidate;
- accepted revision retained for recovery;
- platform metadata required by the package;
- journal or replacement overhead; and
- declared safety margin.

A quota check is advisory until the actual commit succeeds. A racing external
storage change may still fail the operation, in which case the prior accepted
revision remains authoritative.

## Platform metadata

Icons, banners, thumbnails, titles, and platform save-description resources are
packaged native assets or generated metadata. They are not loaded from arbitrary
runtime files before every save operation.

Platform presentation assets may differ without changing logical slot identity
or
portable save bytes.

## Storage remediation and formatting

The application never formats general user storage or a platform account volume.
When a provider reports an unsupported or uninitialized state, the adapter
returns
one typed remediation such as:

- retry after provider availability changes;
- request platform permission;
- sign in or select a profile;
- free storage through the platform UI;
- choose another application-supported container; or
- continue without saving when product policy permits it.

Any platform UI that can initialize dedicated application storage remains
outside
the gameplay save transaction and requires explicit user control.

## Asynchronous operation model

Raw callbacks are replaced by move-only handles and one terminal result. An
operation may be `queued`, `preparing`, `reading`, `writing`, `verifying`,
`committing`, `deleting`, `completed`, `failed`, `timed_out`, or `cancelled`.

Late adapter completion must match operation, profile, slot, container, and
expected revision identities. A late callback cannot complete a replacement
operation or dismiss a newer UI state.

A minimum visible spinner duration is presentation policy only. It cannot delay
durable commit, convert failure to success, or hold a platform lifecycle window
open.

## Suspension and shutdown

Application suspension and exit may request bounded completion or cancellation
of
active operations. The coordinator:

- refuses new conflicting operations;
- preserves the last accepted revision;
- attempts only platform-permitted bounded work;
- records incomplete staging for later cleanup; and
- returns lifecycle control before the platform deadline.

Exit cannot report completion while a required save is merely queued. Product
policy decides whether exit is blocked, cancelled, or continues with the prior
accepted revision.

## Failure behavior

The runtime fails closed on:

- unknown configuration setting or slot identity;
- invalid configuration type, range, or migration;
- duplicate or missing save section;
- stale profile, slot, or accepted revision;
- unavailable account container or permission;
- insufficient quota or short write;
- read-back, checksum, schema, or content validation failure;
- ambiguous most-recent-slot ordering;
- late completion for a replaced operation;
- deletion or reset targeting the wrong slot; or
- an attempt to format user storage from gameplay code.

Failure preserves the prior committed configuration and accepted save revision.

## Verification

Automated tests cover:

- configuration load, migration, default, quarantine, edit, cancel, and restart;
- schema type, range, duplicate, and unknown-field failures;
- logical slot identity under changed display order and physical routes;
- manual, autosave, overwrite, load, delete, and reset policies;
- deterministic slot summaries and most-recent selection;
- quota races, permission loss, provider removal, and account changes;
- interrupted writes and recovery to the last accepted revision;
- late asynchronous results and cancellation;
- suspension and exit deadlines;
- identical logical snapshots across x64 and ARM64; and
- separation of device configuration from portable progression.

## Invariants

- Device configuration and portable gameplay state remain separate.
- Logical slot identity never depends on filename or drive index.
- One operation publishes one terminal result.
- Success means the accepted revision was read back and validated.
- A failed candidate never replaces the prior accepted revision.
- Configuration defaults never reset progression.
- The application never formats general user storage.
- Presentation delay never changes transaction truth.

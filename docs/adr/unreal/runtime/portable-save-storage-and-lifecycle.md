# Portable save storage and lifecycle

- Status: Accepted
- Decision date: 2026-07-13
- Scope: Runtime save portability, physical storage, and platform lifecycle

## Context

The runtime targets Windows, Linux, macOS, and Android across x64 and ARM64.
Native integer layout, pointer width, byte order, filesystem behavior, path
syntax, case sensitivity, process lifecycle, and mobile suspension must not
create different progression or incompatible save identities.

Android may suspend, background, or terminate a process under a bounded
lifecycle
window. Desktop platforms also experience crashes, power loss, interrupted
writes, and storage failures. A partially written save cannot be accepted as
progress.

## Decision

The game uses one versioned logical gameplay-save schema across every supported
platform and architecture. The schema stores canonical gameplay identities and
explicit domain state. It never stores raw memory, pointers, native object
addresses, native struct layout, local filesystem paths, display names, or
platform-specific package locations as authority.

Serialization is deterministic and architecture-neutral. Every field has a
schema-defined representation independent of compiler ABI, pointer width, host
byte order, and operating-system conventions. Schema migrations are explicit,
ordered, versioned, and fail closed when a required migration is unavailable.

The domain owns save meaning through typed ports. Platform adapters own physical
storage locations, permission handling, durable replacement, lifecycle signals,
and operating-system diagnostics. A save operation stages and validates a
complete candidate before replacing the previous accepted revision. Failure
preserves the last valid save and never leaves a success marker for partial
data.

Android background and suspension signals may request a bounded flush of already
committed domain state. They do not invent progress, skip validation, or mutate
simulation to force a save. If the lifecycle budget is insufficient, the
adapter preserves the last accepted revision and reports the incomplete flush as
a recoverable condition.

Portable gameplay state is distinct from device-local configuration. Graphics
preset, display mode, resolution, frame pacing, input-device presentation,
mobile safe-area calibration, and hardware-specific options remain local unless
a later decision explicitly promotes a setting into the portable profile.

A save produced on one supported target uses the same logical schema as a save
produced on another target. This decision does not create cloud synchronization,
account transfer, automatic cross-device discovery, or a hosted save service.
Those mechanisms require separate authority.

## Consequences

- Windows, Linux, macOS, Android, x64, and ARM64 share progression and save
  keys.
- Save compatibility depends on schema and content revisions, not platform paths
  or native binary layout.
- Canonical catalog identities survive platform transfer and migration.
- Physical save locations and permission models remain platform-adapter details.
- Interrupted writes, process termination, and failed migrations preserve the
  last accepted revision.
- Device-local graphics and input presentation settings cannot change portable
  gameplay state.
- Missing mod or catalog content returns a typed unavailable or migration
  result;
  it is not silently deleted from progression.

## Rejected alternatives

- Serializing native C++ structs, pointers, object paths, or compiler-dependent
  layouts directly.
- Maintaining separate save formats for desktop, mobile, x64, or ARM64.
- Treating a successful file-open or partial write as a successful save.
- Overwriting the last valid revision before the replacement candidate is fully
  written and validated.
- Mutating gameplay or bypassing validation to satisfy a mobile suspend window.
- Mixing hardware-specific graphics and input settings into portable progression
  state.
- Promising cloud sync or cross-account transfer without a separate decision and
  implementation.

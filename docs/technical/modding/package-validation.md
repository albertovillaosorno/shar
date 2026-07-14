# Mod package validation

- Status: Active
- Last reviewed: 2026-07-13

## Governing decisions

- [Local mod trust and distribution boundary](../../adr/modding/mod-safety-scanner-and-distribution.md)
- [Local drop-in mod packages and AI skills](../../adr/modding/drop-in-mod-packages-and-ai-skills.md)

## Purpose

This specification defines how an untrusted local package is normalized,
validated, previewed, and admitted into a candidate active set before any runtime
state changes.

## Validation pipeline

Validation executes these phases in order:

1. isolate the transport input in staging storage;
1. bound archive, declaration, member-count, member-size, and expansion limits;
1. parse the exact package contract version;
1. normalize package and member identities;
1. validate member paths, lengths, hashes, media types, and roles;
1. validate provenance and trust metadata required by policy;
1. resolve target, architecture, ABI, runtime, and capability compatibility;
1. resolve dependencies, conflicts, supersession, and explicit priority;
1. resolve every declared gameplay and asset target through canonical identities;
1. compute a deterministic dry-run change and load-order plan;
1. compare the plan with declaration constraints and current active state;
1. validate a complete candidate active-set revision; and
1. authorize atomic activation only when every phase succeeds.

No later phase repairs or silently drops an error from an earlier phase.
Validation never executes package-provided native code.

## Transport and resource limits

Transport files, archives, and Android document-provider inputs are untrusted.
The validator rejects recursive containers, unsupported compression, duplicate
normalized members, expansion beyond configured limits, truncated data, trailing
unaccounted payloads when forbidden by the contract, and inputs that require
network access during validation.

Resource limits are applied before full extraction where metadata permits it.
An archive cannot consume unbounded memory, storage, file descriptors, CPU time,
or nested parsing depth. A resource-limit failure leaves the active package set
unchanged.

## Identity and path validation

Package identity is read from the declaration and normalized according to the
package contract. Storage filename, archive filename, external provider URI,
physical directory, and discovery order are evidence only.

Every member path is normalized before uniqueness checks. Validation rejects:

- absolute, drive-qualified, device-qualified, or platform-rooted members;
- parent traversal or ambiguous dot segments;
- separator, case, or Unicode collisions;
- empty or reserved control names;
- links, aliases, or indirections escaping staging storage;
- duplicate members or declarations;
- mismatched declared and observed lengths or hashes; and
- operating-system metadata that changes member meaning.

A package valid on a case-sensitive filesystem must normalize to the same member
set on a case-insensitive filesystem.

## Compatibility validation

Content-only packages declare required gameplay, asset, rendering, input, and
platform capabilities. A content package may activate on multiple targets only
when each required capability resolves on the active native game package.

Native packages declare exact canonical target identifiers, ABI, runtime contract,
binary role, length, and hash. Validation rejects a binary for an undeclared
operating system or architecture. Metadata scanning cannot establish native-code
safety; a separate explicit trust decision remains required even after structural
validation succeeds.

Android managed import and desktop directory discovery use the same compatibility
rules. Android may report native-code loading as unsupported while accepting an
otherwise portable content-only package.

## Dependency and conflict resolution

The validator constructs a graph keyed only by canonical package identities and
accepted revisions. It rejects:

- missing or incompatible dependencies;
- dependency and supersession cycles;
- ambiguous providers;
- unresolved conflicts;
- duplicate canonical identities;
- priority ties whose outcome is not fixed by the contract; and
- a dependency closure that changes with discovery or archive order.

The resulting load order is deterministic for equivalent declarations, active
state, policy, and capabilities.

## Target resolution and preview

Every declared change resolves to a supported canonical gameplay, catalog, asset,
configuration, or presentation target. Raw filesystem paths and mutable editor
locations are not valid public mod targets.

The dry-run preview records:

- package additions, updates, removals, and retained revisions;
- dependency and load order;
- superseded and conflicting packages;
- every canonical target changed;
- required content and platform capabilities;
- trust decisions still required;
- save identities that may become temporarily unavailable; and
- the exact candidate active-set revision.

Preview generation is side-effect-free. Repeating it with equivalent input and
active state produces an equivalent result.

## Activation admission

A validated preview is not active state. Admission reconstructs the candidate
plan from validated records, verifies that relevant package, policy, capability,
and active-set revisions have not changed, and then requests atomic replacement
of the complete active-set revision.

A stale preview, changed dependency, removed capability, storage failure, or
runtime read-back mismatch rejects admission. The previous accepted active set
remains authoritative.

## Invariants

- Package and member identity are independent of physical storage.
- Desktop and Android imports normalize equivalent input identically.
- Validation never executes package-provided native code.
- All resource, path, integrity, compatibility, topology, and target checks finish
  before activation.
- Load order is independent of directory enumeration and archive order.
- Native binaries load only after exact target compatibility and explicit trust.
- A preview is deterministic, side-effect-free, revision-bound, and reviewable.
- Activation replaces one complete candidate active set.
- Failed validation, preview, or admission leaves active state unchanged.

## Failure behavior

The package remains staged or rejected on:

- unsupported contract, parser ambiguity, or resource-limit violation;
- invalid declaration, provenance, trust, member, path, length, or hash evidence;
- unsafe archive structure or normalized identity collision;
- incompatible target, architecture, ABI, runtime, or capability;
- unresolved dependency, conflict, supersession, or priority topology;
- unknown or unsupported canonical change targets;
- a preview that is nondeterministic or differs from declaration constraints;
- stale package, policy, capability, or active-set revisions at admission;
- failed candidate storage, replacement, or runtime read-back; or
- any native-code package lacking the required explicit trust decision.

Every failure returns a typed finding with package identity, phase, invariant,
and corrective action. A failed preview or admission never mutates accepted local
state.

## Verification

- Parser and resource tests cover malformed declarations, truncation, expansion
  limits, nested containers, duplicate members, and unsupported compression.
- Path tests cover absolute paths, traversal, separators, case, Unicode, links,
  reserved names, and filesystem-normalization collisions.
- Integrity tests cover short reads, trailing data, length mismatches, and hash
  mismatches.
- Compatibility tests cover every canonical target identifier, content-only
  portability, undeclared native binaries, ABI mismatches, and Android native-code
  rejection.
- Topology tests cover dependencies, conflicts, supersession, cycles, priority,
  and discovery-order independence.
- Target tests resolve canonical gameplay and asset identities and reject raw
  physical paths.
- Preview tests compare repeated results and bind them to package, policy,
  capability, and active-set revisions.
- Fault injection interrupts staging, validation, preview, admission, replacement,
  and runtime read-back and proves that accepted state is preserved.
- Desktop-directory and Android-managed imports of equivalent input produce the
  same findings and candidate logical changes.

## Known limits

Static validation does not prove arbitrary native code is safe, does not provide
an operating-system sandbox, and does not create a hosted discovery or
distribution service.

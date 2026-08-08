# Native asset planning

- Status: Active
- Last reviewed: 2026-08-07

## Governing decisions

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Native asset translation without copy-paste](../../adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md) <!-- markdownlint-disable-line MD013 -->
<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md) <!-- markdownlint-disable-line MD013 -->

## Purpose

This specification explains how normalized packages become deterministic plans
for native Unreal assets without mutating editor state.

## Repository model

The planner consumes validated package identities, capabilities, dependencies,
normalized artifacts, and provenance. It emits native asset kinds, stable
logical targets, dependencies, import settings, construction steps, expected
verification, and provenance. A separate adapter applies the approved plan.

## Invariants

Planning never inspects mutable editor state to decide taxonomy. A plan contains
no proprietary payload, performs no engine mutation, and remains stable for
equivalent validated input.

## Multi-output native imports

A direct import operation owns an explicit ordered output inventory. The first
output is the primary asset; additional companion outputs declare their exact
object path, package path, class, expected dirty state after import, and
dependency-safe rollback order. Preflight requires every declared output to be
absent before mutation. The importer result must match the ordered inventory
exactly, every output is independently read back before save, all owned packages
are saved together, and every package must read back clean afterward.

The Unreal Engine 5.8.1 skeletal FBX route deliberately creates a new Skeleton
companion named from the SkeletalMesh, disables PhysicsAsset and animation
creation, and rejects pre-existing primary or companion packages. Reusing an
existing Skeleton is outside this transaction because import may mutate it and a
delete-only rollback cannot restore its previous state. Rollback deletes the
SkeletalMesh before the Skeleton so no companion is orphaned.

## Failure behavior

Unknown capabilities, conflicting target identity, missing dependencies,
unsupported mappings, nondeterministic ordering, or incomplete provenance
invalidate the plan before editor mutation.

## Verification

Planner tests require no engine. Integration tests compare applied native state
with the approved plan through read-only evidence.

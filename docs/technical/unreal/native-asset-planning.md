# Native asset planning

- Status: Active
- Last reviewed: 2026-07-13

## Governing decisions

<!-- markdownlint-disable-next-line MD013 -->
- [Native asset translation without copy-paste](../../adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md)
<!-- markdownlint-disable-next-line MD013 -->
- [Staged mesh import and world assembly](../../adr/unreal/import-adapters/staged-mesh-import-and-world-assembly.md)

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

## Failure behavior

Unknown capabilities, conflicting target identity, missing dependencies,
unsupported mappings, nondeterministic ordering, or incomplete provenance
invalidate the plan before editor mutation.

## Verification

Planner tests require no engine. Integration tests compare applied native state
with the approved plan through read-only evidence.

# Model output verification

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

<!-- markdownlint-disable-next-line MD013 -->
- [First-principles FBX output contract](../../../adr/fbx/export/fbx-output-contract-boundary.md)

## Purpose

This specification explains the repository-owned checks used to accept a
generated model artifact.

## Repository model

Acceptance combines structural read-back, identity checks, capability checks,
reference validation, timing validation, embedded-resource verification,
deterministic regeneration, and clean native-engine ingestion evidence.

## Invariants

- The artifact matches its declared package identity and capabilities.
- All serialized references resolve within the accepted object graph.
- Equivalent validated input produces equivalent logical output.

## Failure behavior

- Read-back mismatch, missing connections, invalid timing, missing resources,
  nondeterministic output, or native ingestion failure rejects the artifact.

## Verification

- Independent reader tests inspect the serialized object graph.
- Logical reports compare deterministic regeneration.
- Native ingestion tests verify expected asset families and settings.

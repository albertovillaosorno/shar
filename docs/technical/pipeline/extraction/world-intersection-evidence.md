# World intersection evidence

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Extraction provenance and manifest linkage](../../../adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md)

## Purpose

This specification explains how repository-owned extraction represents world
intersection and scene-membership data.

## Repository model

The decoder emits typed intersection primitives, spatial bounds, membership
identities, and provenance. Application logic consumes those records without
depending on source-format chunk names or binary layout.

## Invariants

- Spatial values are finite and ordered.
- Membership identities resolve to known world evidence.
- Equivalent source evidence produces stable normalized ordering.

## Failure behavior

- Invalid bounds, unknown membership, malformed primitive data, and truncated
  evidence fail closed.

## Verification

- Decoder tests cover spatial boundaries and malformed evidence.
- Identity tests reorder equivalent records.
- World-planning tests consume only normalized values.

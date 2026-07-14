# Scene assembly evidence

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Extraction provenance and manifest linkage](../../../adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md)

## Purpose

This specification explains how repository-owned extraction normalizes scene
graphs, instances, transforms, visibility, attachments, cameras, and ordering.

## Repository model

Scene decoders emit typed nodes and references. Application validation resolves
parentage, instance targets, attachments, visibility state, camera data, and
deterministic sort order without carrying source-layout mechanics into
downstream domains.

## Invariants

- Every reference resolves to known normalized evidence.
- Hierarchy is acyclic.
- Transforms are finite and use one declared convention.
- Ordering remains stable for equivalent evidence.

## Failure behavior

- Unknown targets, cycles, invalid transforms, duplicate identities, and
  contradictory visibility or ordering reject the scene capability.

## Verification

- Scene tests cover hierarchy, instance, attachment, and camera references.
- Ordering tests reorder equivalent source records.
- Planning tests consume normalized scene evidence only.

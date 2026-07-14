# Rig and motion evidence

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Lossless fail-closed extraction](../../../adr/pipeline/extraction/lossless-extraction-contract.md)

## Purpose

This specification explains how repository-owned extraction normalizes skeleton
hierarchy, transforms, animation channels, and timing.

## Repository model

Rig decoders emit canonical source identities for joints, parents, rest
transforms, animation groups, channels, keys, and rates. Cross-record validation
resolves references before package construction.

## Invariants

- Joint identity and parentage are explicit.
- Animation channels resolve to known joints or supported targets.
- Key ordering and declared timing remain deterministic.
- Rest evidence remains separate from animated values.

## Failure behavior

- Hierarchy cycles, orphan channels, invalid timing, duplicate identities, and
  truncated keys reject the affected evidence.

## Verification

- Hierarchy and timing tests cover invalid references and ordering.
- Normalization tests preserve source duration and identity.
- Package tests verify rig and motion capability linkage.

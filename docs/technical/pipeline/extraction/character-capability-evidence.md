# Character capability evidence

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

<!-- markdownlint-disable-next-line MD013 -->
- [Lossless fail-closed extraction](../../../adr/pipeline/extraction/lossless-extraction-contract.md)

## Purpose

This specification explains how repository-owned extraction links skeletons,
composite drawables, skins, controllers, and animations into one character
capability set.

## Repository model

Character normalization joins independently decoded evidence through canonical
identities and provenance. Package construction accepts the capability only
after mesh, rig, skin, controller, and animation references agree.

## Invariants

- All joined evidence belongs to one coherent package identity.
- Mesh, rig, skin, and animation references agree.
- Missing optional evidence remains explicit.
- Joined ordering is deterministic.

## Failure behavior

- Cross-package references, missing required components, conflicting identities,
  and incomplete provenance reject the character capability.

## Verification

- Join tests cover complete and partial capability sets.
- Identity tests reject cross-package contamination.
- Character export tests consume only accepted capability sets.

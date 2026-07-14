# World simulation evidence

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Extraction provenance and manifest linkage](../../../adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md)

## Purpose

This specification explains how repository-owned decoders represent collision,
physics, terrain, ownership, and world membership.

## Repository model

Decoders translate validated source evidence into typed shapes, physical
properties, ownership references, terrain relationships, and world-membership
identities. The normalized records preserve provenance and avoid carrying
source-layout structures into runtime policy.

## Invariants

- Counts and references agree with validated input boundaries.
- Physical properties are finite and use declared units.
- Ownership and world membership resolve to known identities.

## Failure behavior

- Truncated records, impossible counts, unknown owners, invalid numeric values,
  and contradictory membership reject the affected evidence.

## Verification

- Decoder tests cover count boundaries and invalid references.
- Normalization tests compare stable identities after input reordering.
- Provenance tests link every record to validated source evidence.

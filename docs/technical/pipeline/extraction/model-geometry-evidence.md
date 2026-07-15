# Model geometry evidence

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

<!-- markdownlint-disable-next-line MD013 -->
- [Lossless fail-closed extraction](../../../adr/pipeline/extraction/lossless-extraction-contract.md)

## Purpose

This specification explains how repository-owned extraction normalizes geometry,
topology, surface channels, bounds, and skin weights.

## Repository model

Geometry decoders emit typed vertex arrays, primitive groups, indices, surface
channels, palette references, influence data, and bounds. Application logic
validates cross-record counts and identities before package classification.

## Invariants

- Every index and influence references known evidence.
- Attribute cardinalities agree with declared ownership.
- Bounds are finite and contain the accepted geometry.
- Normalized ordering is deterministic.

## Failure behavior

- Invalid indices, contradictory counts, missing required attributes, unknown
  joints, and non-finite values reject the affected capability.

## Verification

- Decoder tests cover malformed topology and count boundaries.
- Normalization tests compare equivalent reordered evidence.
- Package tests verify geometry and skin capability reporting.

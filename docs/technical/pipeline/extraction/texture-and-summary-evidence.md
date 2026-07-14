# Texture and summary evidence

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Extraction provenance and manifest linkage](../../../adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md)

## Purpose

This specification explains how repository-owned extraction normalizes supported
image evidence, texture metadata, and summary records.

## Repository model

Decoders emit typed image payloads, dimensions, channel metadata, texture
identities, summary values, capability status, and provenance. Unsupported
families remain explicit and do not collapse into generic byte summaries.

## Invariants

- Image metadata agrees with accepted payload boundaries.
- Texture and summary identities are stable.
- Capability status distinguishes supported, unsupported, and invalid evidence.

## Failure behavior

- Invalid dimensions, truncated payloads, contradictory metadata, duplicate
  identities, and malformed summaries reject the affected evidence.

## Verification

- Image tests cover dimensions, channels, payload boundaries, and invalid input.
- Summary tests cover typed values and unsupported status.
- Package tests verify texture capability and provenance.

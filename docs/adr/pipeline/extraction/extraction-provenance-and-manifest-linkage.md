# Extraction provenance and manifest linkage

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Conversion evidence lineage

## Context

Generated artifacts are trustworthy only when the repository can explain which
input evidence, decoder, policy, and deterministic identity produced them.

## Decision

Every generated unit links to validated source evidence through deterministic
manifests. The link records source identity, selected capability, transformation
identity, relevant checks, and output identity without publishing confidential
source names.

Missing, ambiguous, contradictory, truncated, or unsupported evidence fails
closed. A later stage cannot repair missing provenance by inference.

## Consequences

- Repeated conversion can compare logical identity and evidence lineage.
- Stale partial output cannot masquerade as current success.
- Public-safe manifests may use opaque identities.

## Rejected alternatives

- Best-effort extraction with warnings for lost bytes.
- Path-derived identity.
- Later-stage guesses about artifact origin.

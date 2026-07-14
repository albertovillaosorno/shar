# Lossless fail-closed extraction

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Source decoding contract

## Context

Source decoding is the first evidence-preserving boundary in the conversion
pipeline. A best-effort decoder can silently discard bytes or counts and allow
every downstream artifact to inherit an undetected loss.

## Decision

A decoder either produces a typed, count-checked, provenance-linked
representation of every required byte or reports a failure. Silent byte loss,
best-effort repair, and unsupported omission are forbidden.

## Consequences

- Every required input byte is represented by typed, count-checked,
  provenance-linked output or causes a decoder failure.
- Downstream stages never receive a silently truncated representation.
- Unsupported or contradictory input remains visible at the decoding boundary.

## Rejected alternatives

- Best-effort decoding that skips unknown or malformed regions.
- Treating a parseable prefix as a successful complete extraction.
- Repairing missing values by guessing undocumented semantics.

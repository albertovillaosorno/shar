# Single repository validation authority

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Validation configuration

## Context

Validation must produce the same policy result across machines, editors, and
agents. Per-user or nested tool configuration creates divergent baselines and
turns local exceptions into hidden repository behavior.

## Decision

Each validation tool has one repository-owned authority. Narrow exceptions are
declared centrally and never depend on a particular user workstation.

## Consequences

- Every validation tool reads one repository-owned configuration authority.
- Exceptions remain narrow, reviewable, and independent of a contributor's
  workstation.
- Conflicting local configuration cannot silently change canonical results.

## Rejected alternatives

- Per-user or per-directory validation policy.
- Duplicating tool configuration across several repository locations.
- Weakening a shared rule when one exact exception is sufficient.

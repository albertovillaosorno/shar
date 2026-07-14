# Deterministic editor change replay

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Editor automation reproducibility

## Context

Editor mutations are otherwise easy to perform without a durable record of
order, inputs, or preconditions. Production authoring needs a replayable change
model so stale state and partial mutation can be detected.

## Decision

Every supported editor mutation is representable as an ordered, reviewable,
idempotent plan that can be replayed, verified, and rejected on stale
preconditions.

## Consequences

- Every supported editor mutation has an ordered plan, explicit preconditions,
  idempotent behavior, and post-mutation verification.
- Changes can be reviewed, replayed, and compared without relying on operator
  memory.
- Stale preconditions reject the plan before mutation begins.

## Rejected alternatives

- Undocumented manual editor changes as production state.
- One-shot scripts that cannot be replayed safely.
- Continuing after stale state or partial mutation without a new approved plan.

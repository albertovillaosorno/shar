# Native asset translation without copy-paste

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Native asset construction

## Context

Native assets must remain reproducible from normalized evidence. Manual copy-
paste, drag-and-drop, or undocumented editor repair creates hidden production
state that cannot be reviewed or replayed.

## Decision

Normalized evidence is translated into deterministic native assets through plans
and automation. Manual copy-paste, drag-and-drop, and undocumented editor repair
are not production steps.

## Consequences

- Native assets are created from reviewable deterministic plans and normalized
  evidence.
- Replaying an unchanged approved plan produces the same logical assets and
  verification targets.
- Manual editor repair cannot become hidden production state; stale or
  contradictory preconditions fail before mutation.

## Rejected alternatives

- Copy-paste, drag-and-drop, or hand-authored production assembly.
- Accepting assets whose creation steps and source evidence cannot be replayed.
- Repairing failed imports interactively without updating the owning plan.

# Faithful seven-level world set

- Status: Accepted
- Decision date: 2026-07-12
- Scope: World scope

## Context

Parity completion needs a finite canonical world scope. Without an explicit
boundary, additional worlds or experiments can be mistaken for requirements of
the faithful seven-level runtime.

## Decision

The faithful runtime preserves the original seven-level progression as the
canonical world set. Additional worlds are not part of parity completion.

## Consequences

- Faithful progression, identity, save behavior, and completion evidence cover
  exactly the canonical seven-level world set.
- Additional worlds remain optional scope and cannot substitute for an
  incomplete canonical level.
- World tests and manifests can use one fixed parity boundary.

## Rejected alternatives

- Declaring parity with fewer than the canonical seven levels.
- Treating additional worlds as required parity work.
- Expanding world scope before the canonical progression is complete.

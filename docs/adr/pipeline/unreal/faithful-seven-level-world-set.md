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
canonical campaign set. All seven levels project state over the one persistent
geographic world. Additional campaign levels are not part of parity completion.

A non-campaign `level_11_test` development state may use the same geography for
asset, mission, streaming, lighting, and dynamic day-night validation. It is not
a campaign level, does not contribute to progression or completion, and cannot
substitute for an incomplete canonical level.

## Consequences

- Faithful progression, identity, save behavior, and completion evidence cover
  exactly the canonical seven-level campaign set.
- The test state remains development-only campaign-external scope.
- Additional campaign levels remain optional scope and cannot substitute for an
  incomplete canonical level.
- World tests and manifests can use one fixed parity boundary while testing
  dynamic environment behavior separately.

## Rejected alternatives

- Declaring parity with fewer than the canonical seven levels.
- Treating additional worlds as required parity work.
- Expanding world scope before the canonical progression is complete.

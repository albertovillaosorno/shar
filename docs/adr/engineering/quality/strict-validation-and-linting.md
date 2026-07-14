# Strict validation and linting

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Canonical validation

## Context

Individual formatters, analyzers, compilers, and tests provide only partial
evidence. Repository health requires one orchestrated result that orders those
gates and prevents stale, interrupted, or incomplete runs from being reported as
success.

## Decision

One canonical validator coordinates formatting, static analysis, compilation,
tests, documentation checks, architecture checks, and confidentiality checks.
Failed, stale, interrupted, or partial runs never become success evidence.
Unreal's generated `SecurityToken` configuration key is not itself a validation
finding; publication controls must evaluate secret material by context instead
of rejecting that engine-owned key name categorically.

The platform-specific repository launchers are thin, behavior-equivalent entry
points for the command-owned `shar.validate` authority. The canonical command
refreshes every gate on each launcher invocation, including the time-dependent
version-currency gate. Validation logic must not return to either launcher.

## Consequences

- Final validation evidence comes from one coordinated repository command.
- Failed, interrupted, stale, or partial runs cannot be reused as successful
  evidence.
- Direct formatter, compiler, linter, or test execution remains diagnostic and
  cannot replace the canonical result.
- The engine-generated `SecurityToken` key does not create a false validation
  failure solely because the key is present.
- Both repository launchers execute the same full canonical command and contain
  no independent validation policy.

## Rejected alternatives

- Treating independently successful tools as an equivalent final gate.
- Caching partial or stale execution as success.
- Allowing each language surface to define an unrelated completion standard.

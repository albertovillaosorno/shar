# Local mod trust and distribution boundary

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Mod safety and distribution

## Context

Data packages and native code do not share the same trust boundary. Treating all
mods as equally inspectable would overstate static validation and could imply a
sandbox or distribution service the repository does not provide.

## Decision

Data and asset packages are statically validated before activation, while native
code remains an explicit trust boundary. The repository provides local package
contracts and does not operate a distribution service or claim arbitrary code is
sandboxed.

## Consequences

- Data and asset packages can be rejected through static validation before
  activation.
- Native code requires an explicit trust decision and is never represented as
  safely sandboxed by package metadata alone.
- Repository ownership stops at local package contracts; discovery and
  distribution remain external concerns.

## Rejected alternatives

- Treating a successful metadata scan as proof that arbitrary native code is
  safe.
- Operating a hosted mod marketplace or distribution service.
- Applying one trust model indiscriminately to data packages and executable
  extensions.

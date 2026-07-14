# Driving, traffic, and vehicle behavior parity

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Vehicle gameplay parity

## Context

Vehicle play combines handling, traffic, spawning, pursuit, damage, and
artificial intelligence. A faithful runtime needs explicit ownership of those
interacting contracts instead of a single opaque vehicle subsystem.

## Decision

Driving, traffic simulation, vehicle spawning, pursuit behavior, damage, and
vehicle artificial intelligence are independently authored native runtime
domains that preserve observable gameplay contracts.

## Consequences

- Driving, traffic, spawning, pursuit, damage, and vehicle AI have separate
  owners and can be verified independently.
- Internal implementations may differ from the original while observable
  handling and gameplay contracts remain the parity target.
- Integration tests must cover interactions among traffic, pursuit, damage, and
  player-controlled vehicles.

## Rejected alternatives

- Treating all vehicle behavior as one undifferentiated controller.
- Claiming parity from vehicle appearance or basic movement alone.
- Importing proprietary runtime code to reproduce original behavior.

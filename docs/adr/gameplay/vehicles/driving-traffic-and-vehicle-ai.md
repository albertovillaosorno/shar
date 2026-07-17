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

Standard fixed-topology drivable vehicles use Unreal's Chaos Vehicles system,
including native wheeled-vehicle movement, wheel, rigid-body, asynchronous
physics, and animation facilities. Project code owns semantic definitions,
commands, artificial-intelligence intent, damage, recovery, and presentation
requests; it does not recreate tire, suspension, drivetrain, collision, or
rigid-body simulation.

Chaos Modular Vehicles is not the default. Its experimental runtime construction
and destruction model requires a separate accepted decision for a concrete
feature with verified platform, networking, cooking, fallback, and migration
behavior.

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

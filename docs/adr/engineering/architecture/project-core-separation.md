# Portable core separation

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Portable repository logic

## Context

Core behavior is consumed through engine, protocol, process, storage, and user-
interface surfaces. Without a dependency boundary, adapter concerns leak into
domain logic and duplicated format primitives acquire competing owners.

## Decision

Portable domain and application behavior remains separate from engine, protocol,
process, storage, and user-interface adapters.

Small format primitives shared by independent crates belong to one portable
workspace utility when duplicating the algorithm would create competing owners.
Consumers retain adapters for their own naming and visibility boundaries.

## Consequences

- Domain and application behavior can be tested without engine, protocol,
  process, storage, or user-interface dependencies.
- Shared format primitives have one portable owner instead of competing crate
  implementations.
- Consumer-specific naming and visibility remain adapter responsibilities.

## Rejected alternatives

- Duplicating the same portable algorithm in multiple crates.
- Allowing engine or adapter types to become domain authority.
- Centralizing consumer-specific presentation rules in the shared core.

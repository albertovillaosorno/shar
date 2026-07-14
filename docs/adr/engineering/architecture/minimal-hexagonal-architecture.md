# Minimal hexagonal architecture

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Repository-owned code architecture

## Context

The codebase needs clear ownership and testable external boundaries without
enterprise ceremony applied mechanically.

## Decision

Repository-owned code uses minimal hexagonal architecture. Domain rules and
application orchestration remain independent of external mechanisms. Ports exist
only where a real boundary must be substituted or tested. Adapters implement
those ports and dependencies point inward.

Leaf logic remains direct when another layer would not isolate policy, state, or
an external effect. Shared command-line and filesystem components own stable
mechanisms only; domain policy stays with its capability.

## Consequences

- Architecture is measured by dependency direction, not folder count.
- Domain behavior is testable without external systems.
- Unnecessary service layers, buses, repositories, and abstractions are
  rejected.

## Rejected alternatives

- A monolith with external effects inside domain logic.
- Mandatory CQRS, DDD, or layered ceremony for every component.

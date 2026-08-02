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

Authored source is classified in the fixed order domain, function, kind, and
part. Admitted hexagonal kinds are domain, application, inbound port, outbound
port, inbound adapter, outbound adapter, contract, composition, and math.
Function-level package manifests and public facades remain at the function
boundary. Cross-boundary and integration tests remain outside authored source;
private unit modules may remain colocated when moving them would require wider
production visibility.

Leaf logic remains direct when another kind would not isolate policy, state, or
an external effect. Shared command-line and filesystem functions own stable
mechanisms only; domain policy stays with its capability. Engine-mandated project
structure is contained as one composition part rather than treated as a second
repository taxonomy.

## Consequences

- Every authored source part has one deterministic taxonomy identity.
- Folder conformance does not replace inward dependency direction as the
  architectural quality test.
- Domain behavior is testable without external systems.
- Unnecessary service layers, buses, repositories, and abstractions are
  rejected.

## Rejected alternatives

- A monolith with external effects inside domain logic.
- Mandatory CQRS, DDD, or layered ceremony for every component.

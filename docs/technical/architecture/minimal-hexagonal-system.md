# Minimal hexagonal system

- Status: Active
- Last reviewed: 2026-08-02

## Governing decision

<!-- markdownlint-disable-next-line MD044 -->
<!-- jig-ignore-next-line: Markdown link target is indivisible -->
- [Minimal hexagonal architecture](../../adr/engineering/architecture/minimal-hexagonal-architecture.md) <!-- markdownlint-disable-line MD013 -->

## Purpose

This specification explains how repository-owned domain, application, port, and
adapter components preserve inward dependency direction and isolate external
effects.

## Repository model

Every authored part is classified by domain, function, hexagonal kind, and
part identity. Domain components define identities and invariants without
external effects. Application components coordinate domain behavior through
ports. Inbound adapters translate requests into application calls.

Outbound
adapters implement storage, process, serialization, protocol, and engine
effects. Ports exist only for real substitution or isolation boundaries.
Composition parts own executable assembly and engine-mandated project structure.
Cross-boundary tests remain outside the authored-source taxonomy.

## Invariants

Dependencies point inward. External types are translated at adapter boundaries.
Shared mechanisms do not own domain policy. Pure leaf behavior remains direct
when another layer adds no boundary value.

## Failure behavior

External failures are translated into typed application failures. Domain logic
never parses operating-system messages, transport state, or serializer
implementation details.

## Verification

Domain tests run without external systems, application tests use port doubles,
and adapter tests use bounded synthetic integrations.

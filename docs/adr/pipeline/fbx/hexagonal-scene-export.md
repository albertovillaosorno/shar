# Hexagonal scene export

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Model package conversion

## Context

Model conversion needs one canonical scene representation independent of source
decoders, serialization details, and review applications.

## Decision

Model packages are converted into a repository-owned canonical scene domain.
Application services assemble that domain through input ports. A first-party
binary writer is a driven adapter. Independent repository-owned checks validate
the result.

The scene domain contains only concepts required by supported package profiles.
Source-format details and serializer-specific nodes do not leak into domain
policy.

## Consequences

- Source decoders and the binary writer evolve independently.
- Domain tests require no external content-authoring application.
- Package profiles reject unsupported capability combinations.

## Rejected alternatives

- Direct source-to-binary writing inside decoders.
- A third-party content-authoring application as scene authority.
- Mirroring every source chunk in the canonical domain.

# Unreal asset-conversion boundary

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Native asset planning and editor mutation

## Context

Asset planning and editor mutation have different dependencies, failure modes,
and verification needs. Combining them would force pure conversion policy to
open protocol connections and let mutable editor state redefine asset identity.

## Decision

Repository-owned planning validates normalized evidence and emits stable native
asset plans without opening protocol connections or mutating engine state. A
separate terminal integration applies approved plans through the native Unreal
MCP surface.

## Consequences

- Planning can be validated and tested without an editor session or protocol
  connection.
- Native mutation occurs only after an asset plan has been approved and passed
  to the separate terminal integration.
- Planner and editor failures remain attributable to different boundaries and
  cannot silently redefine one another.

## Rejected alternatives

- Combining evidence normalization, planning, protocol transport, and editor
  mutation in one component.
- Allowing the editor integration to invent asset identity or conversion policy.
- Mutating native state before the corresponding plan is reviewable.

# Unreal support and bridge boundaries

- Status: Accepted
- Decision date: 2026-07-12
- Scope: External integration ownership

## Context

Planning, MCP translation, additive extensions, engine plugins, and optional
external dependencies fail and evolve independently. Combining them behind one
bridge would obscure provenance, capability ownership, and recovery behavior.

## Decision

Repository planning, native MCP translation, additive extensions, engine
plugins, and optional external dependencies remain distinct ownership boundaries
with explicit provenance and failure behavior.

## Consequences

- Repository planning, terminal translation, additive extensions, native engine
  plugins, and optional external dependencies retain separate ownership and
  provenance.
- Failure in one boundary remains attributable and cannot silently fall through
  to another component with different authority.
- Optional dependencies never become implicit core requirements.

## Rejected alternatives

- Treating the terminal translator as the native MCP server.
- Copying or modifying engine plugin source as repository-owned implementation.
- Hiding boundary failure behind an undocumented fallback or optional tool.

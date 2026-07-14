# Upstream native plugin and additive extension boundary

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Unreal MCP dependencies

## Context

Native Unreal MCP plugins are upstream dependencies with their own ownership,
update path, and license boundary. Local patching would obscure provenance and
make repository behavior depend on an unreviewable fork.

## Decision

Native MCP plugins remain upstream dependencies and are not copied, modified,
repackaged, or redistributed. A blocking defect may be addressed only through a
separately named, independently authored additive workaround with its own
decision and regression evidence.

## Consequences

- Native MCP plugins remain external upstream dependencies with their original
  ownership, license, and update path.
- A repository workaround must be additive, separately named, independently
  authored, and justified by a reproduced blocking defect.
- Removing the upstream defect or changing plugin behavior can retire the
  workaround without forking the dependency.

## Rejected alternatives

- Copying, patching, repackaging, or redistributing native plugin source.
- Hiding an upstream defect behind an undocumented local modification.
- Adding a workaround without regression evidence and a distinct ownership
  boundary.

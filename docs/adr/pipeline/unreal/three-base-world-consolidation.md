# Three-base-world consolidation

- Status: Accepted
- Decision date: 2026-07-12
- Scope: World package architecture

## Context

Seven gameplay levels reuse substantial world structure while retaining distinct
progression and content identity. Native packaging needs a consolidation rule
that removes structural duplication without collapsing those level contracts.

## Decision

The native world representation consolidates reusable level structure into three
base world families while preserving seven-level gameplay identity through
deterministic data and layer composition.

## Consequences

- Reusable world structure is owned by three base families while seven-level
  gameplay identity remains explicit in deterministic data and layers.
- Shared-world changes require verification across every level identity that
  composes the affected base family.
- Consolidation reduces duplicated structure without collapsing progression or
  save identity.

## Rejected alternatives

- Maintaining seven fully duplicated native worlds for shared structure.
- Reducing the canonical seven gameplay levels to three level identities.
- Copying shared world changes independently into each level.

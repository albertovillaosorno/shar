# Native world partition and data layers

- Status: Accepted
- Decision date: 2026-07-12
- Scope: World asset planning

## Context

Large native worlds require stable streaming, partition, dependency, and layer
identities. Manual level placement cannot provide reproducible planning or
read-back evidence for those relationships.

## Decision

The one persistent geographic world is translated into native partition and
data-layer plans with stable component identities, geographic placements,
dependencies, streaming boundaries, level-state projections, and verification
instead of manual level placement.

The partition plan consumes canonical FBX component and placement evidence. Data
layers select the active campaign or test state; they do not create alternate
geographic identities for the same physical location.

## Consequences

- Partition cells, data layers, dependencies, streaming boundaries, coordinates,
  and native identities are generated from one deterministic world plan.
- Repeated import can compare planned state with native read-back evidence.
- Missing component, placement, dependency, or partition evidence fails before
  partial world placement.

## Rejected alternatives

- Building one monolithic level without explicit streaming boundaries.
- Using manual actor placement as partition authority.
- Deriving world structure from incidental editor state after import.

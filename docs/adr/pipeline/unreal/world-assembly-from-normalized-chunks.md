# World assembly from normalized chunks

- Status: Accepted
- Decision date: 2026-07-12
- Scope: World reconstruction

## Context

World reconstruction must preserve observable structure without importing
proprietary editor projects or source-engine runtime organization. Normalized
world evidence therefore needs one deterministic native assembly boundary.

## Decision

Faithful world assembly consumes validated normalized world packages and
reconstructs native world state deterministically without copying proprietary
editor projects or preserving source-engine runtime structures.

## Consequences

- Validated normalized world packages are the sole production inputs to faithful
  native world assembly.
- The resulting world is independently authored and can be regenerated without
  proprietary editor projects or source-engine runtime structures.
- Missing world evidence fails before an incomplete native world is accepted.

## Rejected alternatives

- Importing or copying a proprietary editor project.
- Preserving source-engine runtime structures as the native architecture.
- Completing faithful world assembly through undocumented manual placement.

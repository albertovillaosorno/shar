# Background vista and occluder relocation

- Status: Accepted
- Decision date: 2026-07-12
- Scope: World visual assembly

## Context

Background vistas and occluders carry placement and visibility behavior that is
not safely recoverable from copied editor state. Their native representation
must be reconstructed from validated scene and world evidence.

## Decision

Background vistas and occluders are reconstructed as explicit native world
elements whose placement is derived from validated scene and world evidence
rather than copied editor state.

## Consequences

- Background vistas and occluders become explicit native world elements with
  stable identities and evidence-derived placement.
- Visibility and occlusion behavior can be verified independently of incidental
  editor state.
- Rebuilding the same validated scene produces the same planned placement.

## Rejected alternatives

- Manually placing vistas or occluders until the scene appears correct.
- Copying proprietary editor state or world projects.
- Treating occluders as disposable decoration without gameplay-relevant
  verification.

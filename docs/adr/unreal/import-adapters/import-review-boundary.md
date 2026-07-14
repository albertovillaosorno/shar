# Import review boundary

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Native import acceptance

## Context

A successful engine call does not prove that an imported asset matches its
approved plan. Acceptance needs identity, dependency, setting, type, and native
read-back evidence rather than visual inspection alone.

## Decision

Native import is accepted only when planned identities, dependencies, settings,
resulting asset types, and read-back evidence match the approved conversion
plan.

## Consequences

- Import acceptance compares planned identities, dependencies, settings, native
  asset types, and read-back evidence with the approved conversion plan.
- Any mismatch remains a failed import rather than a successful transport with
  follow-up repair.
- Partial native state cannot satisfy the plan's acceptance boundary.

## Rejected alternatives

- Accepting an import because the editor reported completion.
- Using visual spot checks instead of planned identity and read-back evidence.
- Repairing mismatched output manually while preserving a successful import
  result.

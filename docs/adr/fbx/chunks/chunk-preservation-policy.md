# Unsupported model evidence preservation

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Model conversion completeness

## Context

Model packages can contain relevant evidence that the current converter does not
yet understand. Silently dropping those members would make an incomplete
conversion appear successful and destroy information needed for later support.

## Decision

Every relevant package member is deterministically classified as converted,
preserved as typed provenance evidence, or rejected as unsupported. Silent
omission is forbidden.

## Consequences

- Every relevant package member appears in the conversion result as converted,
  preserved evidence, or an explicit rejection.
- Unsupported content remains auditable instead of disappearing from reports.
- Completeness can be measured independently of how many member types are
  currently convertible.

## Rejected alternatives

- Dropping unknown or unsupported members silently.
- Reporting a package as complete when members were omitted.
- Converting unrecognized evidence with guessed semantics.

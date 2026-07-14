# Issue-only collaboration

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Public collaboration intake

## Context

The project needs one public surface for defect reports, evidence, and requests
without a parallel branch-review workflow.

## Decision

Issues are the only supported public collaboration intake. Pull requests are not
used or accepted as a repository workflow.

Authorized maintainers and agents investigate issues, change the repository
directly under canonical validation, and record accurate history. Downstream
users remain free to fork the MIT-licensed source.

## Consequences

- Guidance directs public contributors to issues only.
- Pull-request templates and automation are unnecessary.
- An issue is evidence or a request, not automatic edit authorization.

## Rejected alternatives

- Pull-request-first collaboration.
- Parallel issue and pull-request triage.

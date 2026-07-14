# Repository quality policy

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Authored source quality

## Context

Authored code and documentation span multiple languages, architecture
boundaries, provenance classes, and confidentiality levels. A single quality
policy is needed so local convenience cannot hide correctness, ownership, or
publication defects.

## Decision

Authored code and documentation must satisfy strict formatting, analysis,
testing, architecture, provenance, and confidentiality gates appropriate to
their language and boundary. Exceptions are narrow and explicit.

The spelling dictionaries distinguish reusable English vocabulary, reusable
technical vocabulary, and proper names. Published package names are technical
vocabulary; opaque identifiers and malformed fixture text require exact,
line-scoped suppression instead of dictionary expansion.

## Consequences

- Authored code and documentation must pass the complete quality boundary for
  their language and responsibility.
- Vocabulary classification remains intentional instead of growing one
  undifferentiated spelling dictionary.
- Opaque identifiers and malformed fixtures require exact local suppression,
  preserving future detection everywhere else.

## Rejected alternatives

- Treating successful compilation as sufficient repository quality evidence.
- Adding every proper name or malformed token to a global dictionary.
- Using broad suppressions that hide unrelated future findings.

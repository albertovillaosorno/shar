# Calendar versioning, Conventional Commits, and no releases

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Repository identity and history

## Context

The repository needs stable identifiers and reviewable history without implying
semantic compatibility guarantees or maintaining release management.

## Decision

Repository-owned version identifiers use Calendar Versioning, never Semantic
Versioning. The canonical form is `YY.M.V`, with no leading zeroes: `YY` is the
two-digit calendar year, `M` is the calendar month, and `V` is the zero-based
accepted compatibility-snapshot sequence for that month. The current identity is
`26.7.0`; later accepted snapshots in July 2026 use `26.7.1`, `26.7.2`, and so
on.

Commit messages follow Conventional Commits. Commit type and scope describe the
change but never calculate a calendar identifier.

The repository maintains no changelog, generated changelog, release notes,
release branches, release tags, or hosted releases. Calendar identifiers name
compatibility snapshots only and create no release promise.

## Consequences

- Compatibility is proved by contracts and validation evidence.
- Structured history does not drive release automation.
- No task may introduce Semantic Versioning by convention.

## Rejected alternatives

- Semantic Versioning for the repository.
- Version derivation from commit types.
- Changelog or release pipelines.

# Retire legacy conversion bridges

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Legacy bridge removal

## Context

Legacy conversion bridges duplicate ownership once repository-native typed
conversion exists. Leaving them active creates competing production paths,
divergent fixes, and uncertainty about which output is authoritative.

## Decision

Legacy language and direct source-to-FBX bridges are reference evidence only and
are removed from production ownership once equivalent repository-native typed
conversion exists.

## Consequences

- Production conversion has one repository-native typed owner after equivalence
  is proved.
- Legacy language implementations and direct source-to-FBX bridges remain
  reference evidence, not production dependencies.
- Retirement is gated by behavioral coverage rather than by a calendar date.

## Rejected alternatives

- Maintaining parallel production conversion paths indefinitely.
- Keeping a direct source-to-FBX bridge as an accepted output path.
- Removing legacy evidence before native conversion proves equivalent coverage.

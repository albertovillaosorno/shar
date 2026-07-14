# Package evidence discovery boundary

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Model package input

## Context

Logical package membership must come from authoritative manifests rather than
incidental directory layout. Local names and neighboring files can change
without changing the validated package identity.

## Decision

Model conversion consumes only package identities and evidence selected by
authoritative manifests. Local directory names and incidental neighboring files
never define package membership.

## Consequences

- Authoritative manifests define package membership and input identity.
- Renaming local directories or adding unrelated neighboring files cannot change
  the selected conversion input.
- Missing or contradictory manifest evidence fails before model conversion.

## Rejected alternatives

- Treating recursive directory discovery as package authority.
- Inferring membership from filenames or physical proximity.
- Importing incidental neighboring files into the conversion plan.

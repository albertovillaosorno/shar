# AI-first repository communication

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Repository communication and automation

## Context

The repository is developed and operated primarily through AI agents. Prose
optimized only for manual browsing can hide contracts and encourage invented
behavior.

## Decision

The repository is AI-first. Deterministic structure, explicit contracts, stable
identities, machine-readable evidence, complete indexes, live discovery, and
fail-closed validation take priority when they conflict with prose convenience.

Human readability remains required, but it is a secondary rendering of the same
authoritative contracts. No important rule may exist only as informal narrative.

## Consequences

- Agents discover authority before acting and never guess missing facts.
- Catalogs, manifests, schemas, and reports remain deterministic.
- Human prose uses exact repository terminology.
- Human convenience cannot justify weaker validation.

## Rejected alternatives

- Human-first documentation with machine support added later.
- Prompt-only conventions absent from repository authority.

# Converted asset ingestion boundary

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Native import adapters

## Context

Native import crosses from validated conversion artifacts into mutable engine
state. The adapter must not acquire package taxonomy or domain-policy ownership
merely because it performs the final mutation.

## Decision

Import adapters consume approved deterministic plans and normalized artifacts,
perform bounded native mutations, and return structured evidence without
deciding package taxonomy or domain policy.

## Consequences

- Import adapters execute approved plans and normalized artifacts without owning
  package taxonomy or domain policy.
- Native mutations remain bounded and return structured evidence for review and
  read-back.
- Adapter implementations can change without redefining conversion intent.

## Rejected alternatives

- Letting an import adapter decide package classification or gameplay policy.
- Feeding proprietary source material directly into native mutation code.
- Treating opaque editor completion as sufficient import evidence.

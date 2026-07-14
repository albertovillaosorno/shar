# Deterministic conversion pipeline

- Status: Active
- Last reviewed: 2026-07-13

## Governing decision

- [Eleven-phase remake delivery roadmap](../../adr/pipeline/eleven-phase-remake-delivery-roadmap.md)

## Purpose

This specification explains how repository-owned stages transform validated
local input into native runtime artifacts.

## Repository model

Each stage consumes typed evidence, validates preconditions, produces
deterministic output, and records provenance. The flow is source validation,
decoding, classification, package construction, normalized artifact generation,
native planning, engine application, runtime compilation, mod activation, and
packaging.

## Invariants

Equivalent validated input and policy produce stable logical identities,
ordering, plans, and reports. A stage never scans arbitrary neighboring state
after an authoritative manifest exists.

## Failure behavior

Missing capabilities, inconsistent counts, ambiguous identity, stale
dependencies, partial output, and failed downstream verification stop the stage
without recording success.

## Verification

Focused tests prove each stage contract, while end-to-end validation proves
dependency ordering, resumption, and final artifact integrity.

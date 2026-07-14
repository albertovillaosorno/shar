# Minor-unit taxonomy and package value

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Classification and package construction

## Context

Extracted evidence is too granular and too dependent on source organization to
serve directly as a stable conversion and modding contract.

## Decision

The pipeline classifies evidence into deterministic minor units and then into
packages with explicit capability, dependency, ownership, and identity. Package
selection is evidence-driven and never depends on hardcoded local routes.

A package groups only evidence that shares one coherent conversion and runtime
responsibility. Unrelated source neighbors remain separate.

## Consequences

- Conversion operates on semantic packages rather than arbitrary files.
- Package identity remains stable when local storage organization changes.
- Missing capabilities are reported before serialization.

## Rejected alternatives

- Treating every extracted file as a package.
- Inferring packages from directory names alone.
- Combining unrelated assets to reduce package count.

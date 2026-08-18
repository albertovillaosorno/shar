# Filesystem Function

## Purpose

Defines the `src/foundation/filesystem` boundary.

## Ownership

Owns the source responsibilities located directly below this boundary.

## Prohibitions

Does not own generated artifacts, local dependencies, or game content.

## Navigation

- `composition`: local I/O wiring, replacement/create-new writes, and provider
  traits.
- `domain`: portable path identity and containment policy.

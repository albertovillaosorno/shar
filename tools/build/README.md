# Build Function

## Purpose

Defines the `tools/build` boundary for user build selection, preflight,
dependency bootstrap, packaging, and optional launch helpers.

## Ownership

Owns the externally invoked repository tooling located below this boundary.

## Prohibitions

Does not own generated artifacts, local dependencies, or proprietary game data.

## Navigation

- `adapter-inbound`

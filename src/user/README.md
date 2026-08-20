# User Domain

## Purpose

Defines the `src/user` boundary for the lightweight player-facing SHAR release
surface.

## Ownership

Owns public end-user reconstruction/export entry points, release-local support
code, launch helpers, reviewed source-bound plans, and portable release
metadata.

## Prohibitions

Does not own original-game payloads, private algorithm authoring inputs or
masters, repository build caches, proprietary engine source, or developer-only
build state.

## Navigation

- `source-selection` owns read-only normalization of a user-selected lawful
  installation folder or `Simpsons.exe`.

The active exporter TODO still defines the planned `code/`, `scripts/`, `mods/`,
and reviewed `algorithms/` release surface.

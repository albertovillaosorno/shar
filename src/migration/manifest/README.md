# Manifest Function

## Purpose

Defines the `src/migration/manifest` boundary.

## Ownership

Owns the source responsibilities located directly below this boundary.

## Prohibitions

Does not own generated artifacts, local dependencies, or game content.

## Navigation

- `composition`
- `domain`

## Read-only count observation

`observe-manifest-counts [game-directory]` validates the same exact root source
evidence used by minimum-manifest generation, then writes only public-safe
obfuscated directory/extension count rows to stdout. Observed rows use `count`
rather than the tracked minimum-policy ledger's `min`, so empirical populations
cannot be confused with admission minima. Observation also requires at least one
countable structural row, so successful stdout is never an empty unusable ledger.
It never writes into the source tree, and source-path failures use a generic
diagnostic so local private paths are not copied into calibration output.

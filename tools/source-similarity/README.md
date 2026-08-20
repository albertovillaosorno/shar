# Source Similarity Function

## Purpose

Defines the `tools/source-similarity` boundary for content-free structural
similarity calibration over lawful source manifests. Comparison aggregates only
canonical generated collision ordinals within the unsigned 64-bit generator
range so optional directories cannot renumber otherwise shared structural
evidence. The CLI accepts two public JSONL count ledgers and emits aggregate
exact-rational evidence only.

## Calibration invocation

```text
python tools/source-similarity/adapter-inbound/main.py \
  reference.jsonl candidate.jsonl
```

The command prints reference/candidate/shared/union unit counts, reference
coverage, and weighted-Jaccard similarity. Tracked policy rows use `min`;
read-only observation rows use `count`. A row must select exactly one, and every
coordinate row in one ledger must use the same count field. Optional `kind` and
manifest-header metadata must retain their public JSON shapes but are ignored by
the metric. Directory coordinates must retain the producer's lowercase,
forward-slash normalization (with the empty string reserved for the game root).
Coordinate extensions must likewise be nonempty and lowercase; files without
extensions use the manifest's `(none)` token. Coordinate counts, required-file
minima, and each
ledger's total count must fit the unsigned 64-bit range used by the supported
64-bit Rust manifest producers. File inputs preserve the canonical producer
framing: UTF-8, LF-only record boundaries, no outer record whitespace, and a
final LF. Programmatic count
vectors are captured once and validated as stable snapshots before measurement,
so a stateful mapping cannot change counts between validation and scoring. The
command never
prints an admission result or embeds an acceptance threshold. Input failures
omit local ledger paths.

## Ownership

Owns the externally invoked repository tooling located below this boundary.

## Prohibitions

Does not own generated artifacts, local dependencies, or proprietary game data.

## Navigation

- `adapter-inbound`

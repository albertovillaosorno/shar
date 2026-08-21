# Source Similarity Function

## Purpose

Defines the `tools/source-similarity` boundary for content-free structural
similarity calibration over lawful source manifests. Comparison aggregates only
canonical generated collision ordinals within the unsigned 64-bit generator
range so optional directories cannot renumber otherwise shared structural
evidence. Parsed ledgers require each suffixed collision family to contain the
complete contiguous producer sequence from `~01` through at least `~02`, and
reject a simultaneous base alias without a suffix. The CLI accepts two public
JSONL
count ledgers and emits aggregate
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
manifest-header metadata must retain their public JSON shapes when present;
required-file rows are limited to the schema-v2 public path/minimum pairs, so
explicit JSON `null` is not treated as absence. Coordinate rows must also map to
one producer-classified bucket; an optional `kind` value must match that exact
classification. Coordinate rows must remain in the producer's deterministic
`(dir, ext)` order. Those metadata fields remain ignored by the metric.
Directory
coordinates must retain the producer's
lowercase, forward-slash
normalization (with the empty string reserved for the game root). After any
canonical collision ordinal is removed, every directory component must be the
concatenation of the producer's two lowercased endpoint characters. Normal
endpoints contribute one Unicode scalar each; the current Rust mapping has one
two-scalar lowercase expansion (`i` plus combining dot), which accounts for the
only valid 3- or 4-scalar component shapes. Other short readable aliases are
rejected instead of entering calibration evidence. Coordinate
extensions must likewise be nonempty and lowercase; files without
extensions use the manifest's `(none)` token. Coordinate counts, required-file
minima, and each
ledger's total count must fit the unsigned 64-bit range used by the supported
64-bit Rust manifest producers. File inputs and their lexical directory
parents must be non-redirected; the file itself must be regular and preserve
the canonical
producer framing: UTF-8, LF-only record boundaries, no outer record whitespace,
and a final LF. Each ledger is opened once and its device, inode, modification
time, and byte count must remain identical from pre-open inspection through the
completed read and final path check. Programmatic count
vectors are captured once and validated as stable snapshots before
measurement,
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

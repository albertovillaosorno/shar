# Source Variant Evidence Function

## Purpose

Defines a maintainer-facing evidence boundary for checking whether one private
common-byte artifact remains an ordered subsequence of local source variants.
The check is read-only and does not derive, publish, or reconstruct the common
artifact.

## Evidence invocation

```text
python tools/source-variant-evidence/adapter-inbound/main.py \
  common.bin variant-a.bin variant-b.bin
```

Output identifies variants only by one-based argument ordinal. It reports the
common byte count, each candidate byte count, the number of common bytes matched
in order, and whether the complete common artifact was observed. It never
prints input paths, payload bytes, or an admission result.

For algorithm authoring, `--projection` verifies that every supplied local
variant contains the complete common artifact in order and emits a public-safe
`offset-mask-set-v1` descriptor:

```text
python tools/source-variant-evidence/adapter-inbound/main.py \
  --projection common.bin variant-a.bin variant-b.bin > projection.json
```

Each distinct variant layout contributes one deterministic earliest-match mask;
duplicate layouts collapse to one alternative. The descriptor contains only
bounded spans and selected-offset masks. It contains no source payload bytes,
source hashes, or private paths. `algorithm create` can pair that descriptor
with the compact common source using `--source-projection projection.json`;
replay takes the caller's full variant and uses only the authenticated masks
already stored in the plan.

## Ownership

Owns private cross-variant ordered-byte comparison evidence only.

## Prohibitions

Does not define source admission, derive a common artifact, publish proprietary
payload evidence, or identify an executable by whole-file hash. Projection
output records positions only; `shar.algorithm.v1` remains responsible for
resource validation and authenticated replay.

## Navigation

- `adapter-inbound`

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

## Ownership

Owns private cross-variant ordered-byte comparison evidence only.

## Prohibitions

Does not define source admission, derive a common artifact, publish proprietary
payload evidence, bypass executable protection, or become a
`shar.algorithm.v1` replay-source projection.

## Navigation

- `adapter-inbound`

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
duplicate layouts collapse to one alternative, and output is limited to the
generic algorithm contract's 256 distinct alternatives. Projection mode also
loads the active algorithm settings and rejects reference/candidate sizes,
spans,
or aggregate mask bytes that exceed its file limit. Evidence inputs and
their lexical
directory parents must be real filesystem entries rather than symlinks or
Windows junctions. Each input snapshot verifies both the opened descriptor and
current lexical path identity before any payload read, then keeps device, inode,
modification time, platform `ctime`, and byte count stable through the read and
final path check. Descriptor/current-path parity carries redirect protection;
`ctime` is additional drift evidence where the host updates it. The descriptor
contains only bounded spans and selected-offset masks. Projection derivation
writes selected offsets directly into the packed one-bit mask rather than
retaining a byte-per-candidate flag buffer. It contains no source payload bytes,
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

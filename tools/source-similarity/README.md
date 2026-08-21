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
present taxonomy and required-file lists must exactly retain the producer's
canonical values and order. Required-file rows are limited to the schema-v2
public path/minimum pairs, so
explicit JSON `null` is not treated as absence. Excessively nested JSON also
fails through the same path-free malformed-ledger error rather than escaping as
a parser recursion failure. Coordinate rows must also map to
one producer-classified bucket; an optional `kind` value must match that exact
classification, including nested RTF `document` rows. JSON object members must
retain the producer's canonical field
order, including nested required-file metadata. Coordinate rows must remain in
the producer's deterministic `(dir, ext)` order. Those metadata fields remain
ignored by the metric.
Directory
coordinates must retain the producer's
lowercase, forward-slash
normalization (with the empty string reserved for the game root) and must not
contain host-reserved punctuation, controls, invisible path modifiers, or
trailing dot/space aliases rejected by source-tree intake. After any canonical
collision ordinal is removed, every directory component must be the
concatenation of the producer's two lowercased endpoint characters. The helper
mirrors the Unicode 17 case additions used by Rust 1.97 even though the bundled
Python 3.14 runtime exposes Unicode 16, so newly cased uppercase characters
cannot masquerade as producer-normalized aliases. Normal endpoints contribute
one Unicode scalar each; the current Rust mapping has one two-scalar lowercase
expansion (`i` plus combining dot), which accounts for the only valid 3- or
4-scalar component shapes. Other short readable aliases are
rejected instead of entering calibration evidence. Coordinate
extensions must likewise be nonempty and lowercase; files without
extensions use the manifest's `(none)` token. Coordinate counts, required-file
minima, and each
ledger's total count must fit the unsigned 64-bit range used by the supported
64-bit Rust manifest producers. Observation `count` rows must be positive, while
policy `min` rows may remain zero for optional coordinates. The generated
game-root `png` coordinate is producer-owned optional policy and therefore may
carry only zero; it cannot contribute observed or programmatic units. Every
ledger must still contain at least one coordinate row, and direct programmatic
candidate vectors must likewise contain at least one coordinate. Signed-zero
integer aliases are rejected instead of normalizing to zero. File inputs and
their
lexical directory
parents must be non-redirected; the file itself must be regular and preserve
the canonical
producer framing: UTF-8, compact JSON token framing, LF-only record boundaries,
no outer record whitespace, and a final LF. JSON whitespace between tokens
and escaped object-member names are rejected rather than normalized; valid JSON
string-value escapes remain decoded. Each ledger is opened once and its device,
inode, modification
time, platform `ctime`, and byte count must remain identical from pre-open
inspection through an immediate opened-path check, the completed read, and the
final path check. Descriptor/current-path parity carries redirect protection;
`ctime` is additional drift evidence where the host updates it. Programmatic
count vectors are captured once and validated as stable snapshots before
measurement; their coordinates must resolve to the same producer bucket
taxonomy and complete collision-family shape as parsed ledgers. A stateful
mapping therefore cannot change counts between validation and scoring. The
command never
prints an admission result or embeds an acceptance threshold. Input failures
omit local ledger paths.

## Ownership

Owns the externally invoked repository tooling located below this boundary.

## Prohibitions

Does not own generated artifacts, local dependencies, or proprietary game data.

## Navigation

- `adapter-inbound`

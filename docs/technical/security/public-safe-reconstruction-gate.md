# Public-safe reconstruction calibration boundary

This document defines measurable source-installation terms before SHAR chooses a
production similarity gate. It does **not** approve the current 60% planning
estimate or the tentative 45–55% investigation window.

## Hard validation remains authoritative

Similarity is never source evidence. A candidate installation must independently
satisfy exact-file requirements, the curated minimum manifest, format/structure
validation, provenance rules, and any applicable cryptographic checks. A future
similarity result may add diagnostic evidence only after those gates; it may not
supply missing bytes, forgive a failed exact identity, or turn an invalid source
into an accepted one.

The original installation is read-only. Reconstruction and calibration write
only to SHAR-owned cache/output locations. Exact hashes used by a
`shar.algorithm.v1` document bind that plan to its required local evidence and
protected target descriptors; they are not a separate source of reconstruction
bytes and are not an antivirus verdict.

## Measurable terms

A **100% installation** is a maintainer-controlled lawful reference installation
used privately for coverage calibration. For this metric it is represented only
as a nonnegative count vector keyed by the same public-safe structural
coordinates used by the minimum manifest: obfuscated directory coordinate plus
normalized extension. The percentage denominator is the sum of those reference
counts. Original payload bytes, full source paths, filenames, and a reversible
full-tree description are outside the metric and outside published recipes.

Minimum-manifest collision suffixes are not stable comparison identities.
When two private directories collapse to the same public alias, manifest
generation adds `~01`, `~02`, and later ordinals according to the directories
present in that scan. A candidate missing one colliding directory may therefore
expose the same structural family without the suffix. Calibration strips only
canonical generated ordinals and aggregates those family counts before
comparison. Hard manifest and deep validation keep their exact coordinates; this
normalization cannot admit a source.

A **minimum installation** is not “an installation that reaches 60%.” It is a
lawful candidate that first passes all hard source requirements and whose
optional-source population is then measured against representative complete
references. The current approximately 60% figure is only a hypothesis to test
against such candidates. No production code may infer validity from that number.

For empirical work, **reference coverage** is:

```text
sum(min(reference_count[k], candidate_count[k]))
-------------------------------------------------
sum(reference_count[k])
```

This answers how much of the reference count vector is represented by the
candidate, without rewarding extra candidate-only coordinates.

The current **candidate similarity metric** is a calibration candidate, not an
admission rule. It is weighted Jaccard similarity over the same count vectors:

```text
sum(min(reference_count[k], candidate_count[k]))
-------------------------------------------------
sum(max(reference_count[k], candidate_count[k]))
```

It is symmetric and penalizes both missing and extra structural counts. The
repository-owned stdlib Python calibration helper computes these two exact
rational values and deliberately exposes no acceptance result or threshold. The
`observe-manifest-counts` source adapter can produce a public-safe count ledger
from a lawful local installation without writing into it. Observation rows use
`count` while tracked minimum-policy rows use `min`; the similarity CLI accepts
either form but rejects rows that mix the two meanings. It reports only
aggregate units and ratios and keeps local input paths out of validation
diagnostics.

## Calibration requirements

Before any threshold can become production policy, lawful representative
installations must provide false-positive and false-negative evidence. The
45–55% range is only the first band to investigate. Calibration must record at
least the hard-gate outcome, reference coverage, weighted-Jaccard value, source
edition/language facts that are safe to retain, and whether the candidate is a
known valid or invalid installation. A threshold must be justified by those
observations rather than chosen because it matches the planning band.

Coverage/similarity evidence must never be trained from, or serialized as, a
full-tree diff against the private complete installation. The private reference
may produce aggregate count vectors for calibration; it must not become direct
recipe material.

## Source-bound plan publication boundary

Distributable plans live below a family-owned `algorithm/` directory and use
`shar.algorithm.v1`. Current family paths are:

```text
algorithms/game/algorithm/*.txt
algorithms/lang/<locale>/algorithm/*.txt
algorithms/muckluck/algorithm/*.txt
```

A reviewed plan may contain its schema/version metadata, deterministic operation
parameters, exact source-binding descriptors, protected target descriptors, and
source-bound protected target material produced by the generic algorithm engine.
It must not contain:

- plaintext original-game payload bytes or excerpts;
- a serialized/full-tree diff of the maintainer's private complete installation;
- private reference filenames or paths copied merely to reconstruct that tree;
- source hashes treated as substitutes for caller-supplied local source
  bytes; or
- a hardcoded 45–55% gate before calibration is accepted and versioned.

Repository policy additionally scans every publishable family
`algorithm/*.txt`. Whitespace-only placeholders remain admitted as explicitly
non-buildable state; every substantive plan must decode as a
`shar.algorithm.v1` object with the exact top-level source/target contract,
lowercase source/target hashes, 12-byte nonce evidence, and hexadecimal
protected target material whose encoded length includes the required 16-byte
authentication tag and whose declared resource use fits the active generic
algorithm settings. Generic replay checks the fixed nonce and protected-payload
encoded lengths before hex decoding, so malformed documents cannot expand those
fields beyond their declared target size. The authored icon reconstruction plan
exercises the same policy against a real non-placeholder document. Source and
directory-target record paths must also be canonical forward-slash relatives,
with an empty source path reserved for direct-file evidence. Source records also
retain the collector's input order without decreases, unique input/path
identities, and one root kind per input. This structural publication guard does
not turn a placeholder into a reviewed plan and does not implement a similarity
threshold.

The maintainer-only `in/`, `master/`, and shared `out/` workspace trees never
ship as reconstruction plans. Generic algorithm authoring also requires the
canonical target root to be disjoint from every canonical source root in both
containment directions; source payloads therefore cannot be republished merely
by selecting the source tree, or one of its descendants/ancestors, as the
protected target. Authoring also compares cross-platform physical file identity,
so a disjoint hard-link alias of a source file cannot become protected target
material. Source evidence itself must also contain one record per physical file;
hard-link aliases are rejected before source file/byte minima are evaluated and
cannot inflate admission evidence. Target collection also rejects repeated
physical files because this ordinary-file plan format cannot preserve hard-link
topology faithfully. Directory replay also rejects target paths
that overlap by ancestry, so one protected file cannot become the parent of
another protected file during sequential persistence. Target identities use the
same Unicode-uppercase portable comparison as filesystem evidence, so paths
that differ only by host-sensitive casing cannot collide at replay. A published
plan remains
unusable without the local source evidence admitted by its source-bound replay
contract. Algorithm authoring publishes the plan with create-new persistence,
so an
existing plan path is rejected atomically instead of being replaced. Replay uses
the same create-new byte persistence for every final recovered file, closing the
absence-check/replacement race at the filesystem write itself.
The
generic engine's filesystem diagnostics preserve the failed operation and I/O
error kind without embedding caller source, target, algorithm, or replay paths.
Each source and target file is read once into a validated in-memory snapshot;
source-key derivation and target encryption use those captured bytes rather than
reopening caller paths later in the same operation. Its external round-trip
tests require caller-supplied source at replay and verify that create/replay
does
not change the caller's source bytes or layout.

## Current status

The definitions and calibration metric above are now testable. Production
admission remains intentionally unimplemented. The 60% and 45–55% numbers remain
planning evidence, not legal conclusions, safe harbors, or product thresholds.

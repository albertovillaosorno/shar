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
only to SHAR-owned cache/output locations. `hash.txt` remains informational
integrity/modification evidence for expected distributable artifacts and is not
reconstruction input.

## Measurable terms

A **100% installation** is a maintainer-controlled lawful reference installation
used privately for coverage calibration. For this metric it is represented only
as a nonnegative count vector keyed by the same public-safe structural
coordinates used by the minimum manifest: obfuscated directory coordinate plus
normalized extension. The percentage denominator is the sum of those reference
counts. Original payload bytes, full source paths, filenames, and a reversible
full-tree description are outside the metric and outside published recipes.

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
stdlib Python helper `tools/source-similarity/main.py` computes these two exact
rational values and deliberately exposes no acceptance result or threshold.

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

## `build.txt` publication boundary

A future `algorithms/<arch>/<os>/build.txt` may contain procedure, schema/version
metadata, public tool identities, deterministic operation parameters, and names
of source evidence classes the user must supply locally. It must not contain:

- original-game payload bytes, excerpts, or reversible encodings;
- a serialized/full-tree diff of the maintainer's private installation;
- private filenames or paths copied merely to reconstruct the reference tree;
- source hashes used as substitutes for local source bytes; or
- a hardcoded 45–55% gate before calibration is accepted and versioned.

`build.txt` must remain useless as a substitute for a lawful local source copy.
Its procedure can describe how SHAR transforms validated user-owned inputs into
SHAR-owned outputs; it cannot carry the missing original content itself.

## Current status

The definitions and calibration metric above are now testable. Production
admission remains intentionally unimplemented. The 60% and 45–55% numbers remain
planning evidence, not legal conclusions, safe harbors, or product thresholds.

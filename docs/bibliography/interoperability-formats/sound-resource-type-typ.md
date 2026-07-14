# TYP Sound Resource Metadata

This non-governing record documents one interoperability subject without
granting rights in proprietary code, tools, documentation, game data, assets,
names, marks, or user-supplied content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Binary TYP handling, bounded field,
  count, name, and identifier summaries, and malformed-input rejection are
  verified for observed samples; the name expansion, authoritative field
  definitions, byte order, version history, platform variants, and rights chain
  remain unresolved.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local interoperability research only.
- Subject class: Proprietary binary sound-resource type metadata format.

## Covered Material

TYP files used by the pipeline to summarize sound-resource metadata, binary
fields, record counts, names, identifiers, and other bounded structural
evidence.

## Repository Use And Scope

The pipeline handles TYP as binary input rather than UTF-8 text and emits
structured JSON summaries. The current parser is limited to observed samples and
must reject malformed or unsupported layouts.

## Provenance And Version History

The game manifest and pipeline confirm TYP files in sound-related routes. The
expansion of TYP, authoritative field definitions, byte order, version history,
platform variants, and ownership chain remain unresolved.

## Authorship, Ownership, And Attribution

Historical developers, contributors, publishers, licensors, and any successors
retain applicable rights in upstream code, documentation, tools, marks, and
protected expression. SHAR claims rights only in independently authored
repository material to the extent supported by authorship evidence and law.

## License Or Terms Basis

No standalone public specification license or redistribution grant for this
proprietary subject has been verified. The SHAR MIT License applies only to
material the repository owner has authority to license and does not absorb
upstream expression, assets, marks, patents, trade secrets, or contracts.

## Distribution, Modification, And Compatibility

Independently observed functional facts may support compatibility work, but
successful parsing does not authorize distribution of the input, extracted
content, historical tools, or copied documentation. Copyright, contract, anti-
circumvention, trademark, patent, trade-secret, and jurisdiction questions
require separate fact-specific analysis in docs/legal.

## Compliance Posture

- Use only user-supplied local input obtained on a documented lawful basis.
- Keep original and extracted proprietary payloads outside Git and distributed
  artifacts.
- Use synthetic or independently authored fixtures for tracked regression tests.
- Preserve private hashes, acquisition dates, and version evidence without
  publishing local routes.
- Do not infer ownership, authorization, or redistribution rights from
  successful decoding.
- Preserve unknown bytes and unsupported fields as evidence rather than
  assigning invented meanings.
- Record sample hashes, file sizes, route context, and parser assumptions for
  each verified layout.

## Source References

- [Radical Entertainment historical toolchain provenance
  record](radical-entertainment-toolchain-and-formats.md).
- Historical Radical source notices reviewed locally; source material not
  distributed.
- SHAR repository (2026) game manifest and pipeline sound-type decoder.

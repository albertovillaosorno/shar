# RadSound RSD

This non-governing record documents one interoperability subject without
granting rights in proprietary code, tools, documentation, game data, assets,
names, marks, or user-supplied content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Supported RSD header, sample, channel,
  payload validation, fail-closed rejection, and deterministic PCM WAVE output
  are verified in repository code and tests; the full codec registry, platform
  encodings, middleware history, and present rights remain unresolved.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local interoperability research only.
- Subject class: Proprietary audio container and codec metadata family.

## Covered Material

RSD audio headers, sample metadata, channel layout, supported PCM payloads, and
other variants that SHAR can identify or reject.

## Repository Use And Scope

The src/rsd crate validates supported RSD metadata and emits deterministic PCM
WAVE output. Unsupported codecs and malformed frame shapes fail closed. The
repository does not publish source RSD audio.

## Provenance And Version History

Historical notices associate RSD with RadSound. The full codec registry,
platform-specific encodings, middleware version history, and present rights
authority have not been established.

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
- Document every supported and rejected codec identifier without claiming
  completeness.
- Treat generated WAVE files as transformations whose source-content rights
  remain unchanged.

## Source References

- [Radical Entertainment historical toolchain provenance
  record](radical-entertainment-toolchain-and-formats.md).
- Historical Radical source notices reviewed locally; source material not
  distributed.
- SHAR repository (2026) src/rsd, WAVE serialization tests, and pipeline audio
  adapters.

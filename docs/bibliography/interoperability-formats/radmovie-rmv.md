# RadMovie RMV

This non-governing record documents one interoperability subject without
granting rights in proprietary code, tools, documentation, game data, assets,
names, marks, or user-supplied content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — RMV header, stream, frame, audio-track,
  hash, and supported conversion-input handling is verified in repository code;
  the complete codec matrix, version history, platform variants, and present
  rights chain remain unresolved.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local interoperability research only.
- Subject class: Proprietary cinematic and synchronized-audio container family.

## Covered Material

RMV headers, stream metadata, frame and audio-track boundaries, integrity
evidence, and conversion inputs needed to produce local HAP and WAVE packages.

## Repository Use And Scope

The src/rmv crate and pipeline inspect user-supplied RMV files, preserve hash
and stream evidence, and invoke external media tooling for supported conversion.
The repository does not distribute original cinematics.

## Provenance And Version History

Historical notices associate RMV with RadMovie. The complete stream codec
matrix, version history, platform variants, and current rights chain remain
unresolved.

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
- Record stream counts, codecs, dimensions, frame timing, hashes, and conversion
  commands per sample.
- Keep media-tool licensing separate from rights in source and converted
  audiovisual content.

## Source References

- [Radical Entertainment historical toolchain provenance
  record](radical-entertainment-toolchain-and-formats.md).
- Historical Radical source notices reviewed locally; source material not
  distributed.
- SHAR repository (2026) src/rmv and pipeline movie-conversion adapters.

# TextBible Language Files

This non-governing record documents one interoperability subject without
granting rights in proprietary code, tools, documentation, game data, assets,
names, marks, or user-supplied content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — UTF-8 key, value, source-order, and
  current extension-to-language mappings are verified as SHAR implementation
  behavior; the authoritative extension registry, regional variants, hashing,
  encoding, fallback rules, and present rights remain unresolved.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local interoperability research only.
- Subject class: Proprietary localization text and language-channel family.

## Covered Material

TextBible source text and language-channel files recognized by the pipeline,
including TXT and the E, F, G, I, S, and X extensions, together with key, value,
language, and source-order evidence.

## Repository Use And Scope

The pipeline reads user-supplied UTF-8 files and emits normalized localization
JSON. Current repository mappings label E as English, F as French, G as German,
I as an Italian stub, S as Spanish, X as an unknown variant, and TXT as source
text. Those labels are implementation evidence, not an authoritative language
specification.

## Provenance And Version History

Historical context associates TextBible and Scrooby localization with the
Radical toolchain. The authoritative extension registry, regional variants,
hashing rules, text encoding, fallback behavior, and present rights authority
remain unresolved.

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
- Do not publish original dialogue, subtitles, or localized text bodies without
  publication authority.
- Treat language labels and fallback behavior as hypotheses until corroborated
  by primary evidence.

## Source References

- [Radical Entertainment historical toolchain provenance
  record](radical-entertainment-toolchain-and-formats.md).
- Historical Radical source notices reviewed locally; source material not
  distributed.
- SHAR repository (2026) pipeline TextBible decoder, language JSON export
  record, and tests.

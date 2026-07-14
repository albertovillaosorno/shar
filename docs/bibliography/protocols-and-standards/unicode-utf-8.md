# Unicode And UTF-8

This non-governing record documents character encoding and text interchange
rules without granting rights in encoded text, fonts, translations, or
third-party libraries.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository use, the Unicode Standard, and RFC 3629
  verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Deterministic text parsing and serialization.
- Subject class: Character repertoire and variable-width encoding standard.

## Covered Material

Unicode scalar values and UTF-8 byte sequences used by SHAR for source text,
JSON, Markdown, scripts, localization files, paths, diagnostics, and MCP data.

## Repository Use And Scope

SHAR validates UTF-8 at text boundaries and fails closed when a supported text
format contains malformed source bytes. Binary formats such as TYP are not
silently treated as UTF-8. Unicode validity does not establish the language,
meaning, ownership, or publication status of text.

## Provenance And Version History

The Unicode Consortium maintains the Unicode Standard. RFC 3629 defines UTF-8
for Internet use and constrains it to Unicode scalar values. Unicode versions,
normalization behavior, assigned characters, case mappings, and implementation
libraries may change independently.

## Authorship, Ownership, And Attribution

The Unicode Consortium and specification contributors retain applicable rights
in standards publications and data files. Authors and translators retain rights
in encoded text. Font software and glyph artwork retain separate rights.

## License Or Terms Basis

Unicode publishes terms for its data files and software. Use of UTF-8 syntax
does not license encoded prose, translations, fonts, identifiers, or
implementation libraries. The exact Unicode data and code incorporated into a
distribution must be reviewed separately.

## Distribution, Modification, And Compatibility

Well-formed UTF-8 may still contain confusable characters, control characters,
non-normalized sequences, private-use characters, or protected text. Re-encoding
does not change authorship or authorization.

## Compliance Posture

- Reject malformed UTF-8 at declared text boundaries.
- Record normalization and case-folding behavior where identity depends on it.
- Preserve source bytes privately when lossless provenance is required.
- Do not infer language or ownership from an extension or encoding alone.
- Review confusable, control-character, and path-safety risks separately.

## Technical Baseline And SHAR Profile

### Public baseline

RFC 3629 defines the UTF-8 encoding form and its valid octet sequences. Unicode
normalization is a separate policy described by Unicode Standard Annex #15.
Confusable detection and related identifier-security mechanisms are separate
again and are described by Unicode Technical Standard #39.

A statement that a field is UTF-8 therefore does not establish normalization,
case folding, grapheme handling, identifier comparison, path identity, or
confusable policy.

### SHAR profile status

Repository components generally use language-native Unicode strings after
successful UTF-8 decoding and apply boundary-specific validation. The reviewed
repository evidence does not establish one global normalization form, case-fold
rule, control-character policy, confusable policy, or cross-platform path-
identity contract.

No normalization step should be inferred or introduced merely to make two
visually similar values compare equal. Byte identity, Unicode scalar identity,
case-insensitive platform behavior, and user-visible text equivalence are
distinct concepts.

### Acceptance boundary

Each text boundary must specify:

- accepted encoding and invalid-sequence behavior;
- whether a byte-order mark is accepted, rejected, or stripped;
- normalization form or explicit preservation of original normalization;
- case-sensitive, ASCII-insensitive, Unicode-folded, or platform-defined
  comparison;
- permitted control and noncharacter code points;
- confusable or mixed-script handling for identifiers;
- filename and path identity on each supported filesystem; and
- round-trip, collision, and spoofing tests.

### Verified sources

- IETF (2003), *RFC 3629: UTF-8, a transformation format of ISO 10646*.
  <https://www.rfc-editor.org/rfc/rfc3629.html>
- Unicode Consortium, *Unicode Standard Annex #15: Unicode Normalization Forms*.
  <https://www.unicode.org/reports/tr15/>
- Unicode Consortium, *Unicode Technical Standard #39: Unicode Security
  Mechanisms*. <https://www.unicode.org/reports/tr39/>

## Source References

- Unicode Consortium (n.d.) *The Unicode Standard*. Available at:
  <https://www.unicode.org/standard/standard.html> (Accessed: 12 July 2026).
- IETF (2003) *RFC 3629: UTF-8, a transformation format of ISO 10646*. Available
  at: <https://www.rfc-editor.org/rfc/rfc3629.html> (Accessed: 12 July 2026).
- Unicode Consortium (n.d.) *Terms of Use*. Available at:
  <https://www.unicode.org/license.txt> (Accessed: 12 July 2026).
- SHAR repository (2026), text decoders, JSON serializers, and UTF-8 guards.

# INI Configuration Family

This non-governing record documents an informal configuration-file family
without asserting a universal INI standard or granting rights in configuration
content, parser code, or linked packages.

## Review Status And Scope

- Review status: Verified.
- Evidence status: Verified — Microsoft profile API behavior verified; current
  source treats package INI entries as opaque payloads.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local language-mod configuration and migration input.
- Subject class: Informal section-and-key configuration-file family.

## Covered Material

INI-like files used by the LMLM language workflow, including `CustomText.ini`,
section names, keys, values, comments, duplicate keys, escaping, encoding, line
endings, and ordering where relevant.

## Repository Use And Scope

SHAR may read a user-supplied INI-like file to produce normalized language JSON.
The repository must define its accepted grammar from observed evidence and
tests. Compatibility with one parser or application does not establish universal
INI compatibility.

## Provenance And Version History

INI is a family of related conventions rather than one complete normative
standard. Implementations differ on comments, quoting, escaping, case
sensitivity, duplicate keys, continuation lines, interpolation, and encoding.
The producing application and exact sample control.

## Authorship, Ownership, And Attribution

Parser authors and application maintainers retain rights in their
implementations. Authors and rights holders retain rights in configuration text,
translations, and referenced content. SHAR contributors retain rights in
independently authored parser and schema code.

## License Or Terms Basis

An INI-like syntax does not license configuration content or an implementation.
The producing tool's terms, the sample's provenance, and any linked or embedded
content require separate review.

## Distribution, Modification, And Compatibility

Configuration files may contain protected text, personal data, credentials,
paths, or third-party identifiers. Normalization to JSON does not make those
values distributable or remove their source provenance.

## Compliance Posture

- Document the exact accepted grammar and reject ambiguous constructs.
- Record encoding, case sensitivity, duplicate-key, and comment behavior.
- Keep credentials, private paths, and user-specific values outside Git.
- Preserve source hashes and acquisition evidence for migration inputs.
- Do not claim universal INI conformance.

## Technical Baseline And SHAR Profile

### Public baseline

INI is a family of application-defined text formats rather than one universal
standard. Microsoft profile APIs document one historically important Windows
model involving sections, keys, string values, case-insensitive lookup behavior,
quote handling, and possible registry mapping. Those API semantics do not define
every file named `.ini`.

### SHAR profile status

The current repository source contains no `CustomText.ini` parser or serializer.
The LMLM package parser validates container structure and may extract an INI-
named entry as opaque bytes; it does not claim a general INI grammar. Comments,
quoting, escaping, duplicate keys, continuation lines, case rules, encoding,
byte-order marks, and line endings therefore have no current SHAR contract.

Introducing a parser requires an owning ADR, a technical grammar,
producer-version evidence, a redacted variation matrix, and malformed-input
tests before any INI compatibility claim is made.

### Verified sources

- Microsoft, *GetPrivateProfileString function*.
  <!-- markdownlint-disable-next-line MD013 -->
  <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprivateprofilestring>
- Repository source inspection on 2026-07-13 found no `CustomText.ini` parser or
  serializer.

## Source References

- Microsoft (n.d.) *GetPrivateProfileString function*. Historical Windows
  profile-file behavior. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getprivateprofilestring>
  (Accessed: 12 July 2026).
- SHAR repository (2026), LMLM language JSON export and `CustomText.ini`
  references.

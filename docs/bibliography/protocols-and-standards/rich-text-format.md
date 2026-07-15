# Microsoft Rich Text Format

This non-governing record documents a published proprietary document format
without granting rights in Microsoft code, product documentation, source
documents, names, or marks.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository behavior and a Microsoft-published
  specification verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local document migration only.
- Subject class: Published proprietary document-interchange format.

## Covered Material

Rich Text Format control words, groups, destinations, character sets, Unicode
escapes, font tables, metadata, paragraph and line controls, and other
constructs interpreted by `src/rtf/`.

## Repository Use And Scope

SHAR independently parses user-supplied RTF documentation and emits normalized
Markdown. It does not embed Microsoft implementation code or claim complete
support for every RTF revision or application extension.

## Provenance And Version History

Microsoft published multiple RTF specifications. The repository uses the RTF
1.9.1 specification as the principal historical syntax reference while
preserving unknown controls and rejecting malformed syntax according to
repository policy. The historical version number is evidence, not an update
policy.

## Authorship, Ownership, And Attribution

Microsoft and other contributors retain applicable rights in the specification
and product implementations. SHAR contributors retain rights in the
independently authored parser and tests. Rights in source documents remain
separate.

## License Or Terms Basis

Publication of a specification does not by itself grant rights in Microsoft
product code, trademarks, document content, or every product extension. The
specification's publication terms and any patent or trademark notices must be
reviewed before redistributing specification text.

## Distribution, Modification, And Compatibility

Implementing documented syntax does not relicense input documents. Generated
Markdown may reproduce protected source text and therefore requires a separate
publication decision even when the parser is independently authored.

## Compliance Posture

- Use the Microsoft-published specification as the principal syntax reference.
- Record unsupported controls and destinations without claiming full
  conformance.
- Keep source RTF documents outside Git unless publication rights are
  established.
- Treat generated Markdown as content derived from the input document.
- Preserve source hashes and conversion evidence for consequential migrations.

## Technical Baseline And SHAR Profile

### Public baseline

Microsoft Rich Text Format 1.9.1, dated March 2008, is the identified public
syntax baseline. The version is historical evidence, not a claim that every
Microsoft application extension is documented or supported.

### SHAR profile

The repository contains an independently authored RTF parser and regression
suite under `src/rtf/`. The parser converts user-supplied RTF documents into
normalized output while preserving an explicit unsupported-behavior boundary.
The public specification does not determine which control words occur in the
operator's source corpus.

### Use-specific evidence limits

Before accepting RTF conversion for the operator's source corpus, maintain a
private sample matrix, identify the exact required extension set, and test the
registry of unsupported controls, destinations, character sets, and malformed-
input behavior.

### Verified sources

- Microsoft Corporation (2008), *Rich Text Format Specification, version 1.9.1*,
  archived copy of the Microsoft-distributed PDF:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://web.archive.org/web/20190708132914/http://www.kleinlercher.at/tools/Windows_Protocols/Word2007RTFSpec9.pdf>
- Microsoft protocol documentation identifying RTF 1.9.1 as the referenced
  specification:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://officeprotocoldoc.z19.web.core.windows.net/files/MS-OXCMAIL/%5BMS-OXCMAIL%5D-240820.pdf>

## Source References

- Microsoft Corporation (2008) *Rich Text Format specification, version 1.9.1*.
  Archived copy of the Microsoft-distributed PDF available via the Internet
  Archive at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://web.archive.org/web/20190708132914/http://www.kleinlercher.at/tools/Windows_Protocols/Word2007RTFSpec9.pdf>
  (Accessed: 13 July 2026). The former Microsoft archive endpoint was
  unavailable during this review.
- SHAR repository (2026) `src/rtf/` and RTF regression tests.

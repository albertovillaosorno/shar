# Comma-Separated Values

This non-governing record documents a tabular text interchange convention
without granting rights in exported records, source data, schemas, or parser
implementations.

## Review Status And Scope

- Review status: Verified.
- Evidence status: Verified — RFC 4180 verified; current source contains no
  production CSV writer or `.csv` output contract.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Manifest and review-table output where enabled.
- Subject class: Delimited tabular text convention and registered media type.

## Covered Material

CSV output associated with archive or manifest review, including records,
fields, commas, quotes, embedded line breaks, headers, encoding, and newline
handling.

## Repository Use And Scope

Where SHAR emits CSV, the output is a review or interchange artifact rather than
the sole semantic authority. The repository must define column names, ordering,
encoding, line endings, null representation, duplicate handling, and escaping.

## Provenance And Version History

RFC 4180 documents a common CSV format and registers `text/csv`, but CSV has
many dialects. Compliance with one reader does not establish universal
portability. Spreadsheet applications may also interpret formula-like cell
values.

## Authorship, Ownership, And Attribution

Standards authors retain applicable rights in published documents. Data authors
and source rights holders retain rights in exported content. Parser and
spreadsheet implementations retain separate rights.

## License Or Terms Basis

Use of comma-delimited syntax does not license the exported data or a parser.
Standards publication terms, source-data rights, and bundled implementation
licenses require separate review.

## Distribution, Modification, And Compatibility

A syntactically valid CSV can expose confidential paths, personal data,
protected names, or formula-injection payloads. Exporting data to CSV does not
make it safe or authorized for publication.

## Compliance Posture

- Define one repository CSV dialect and document every column.
- Quote fields deterministically and preserve embedded line breaks safely.
- Neutralize formula-like cells when files may be opened in spreadsheets.
- Record encoding, newline, header, and null-value conventions.
- Review exported content and provenance before publication.

## Technical Baseline And SHAR Profile

### Public baseline

RFC 4180 documents a common CSV profile and the `text/csv` media type, including
comma-separated fields, quote escaping, and CRLF-oriented record framing. It is
an informational interoperability baseline rather than proof that all software
named CSV uses one dialect.

The RFC does not define SHAR-specific null representation, schema identity,
alternative delimiters, character encoding beyond media-type considerations,
spreadsheet formula neutralization, or consumer-specific type inference.

### SHAR profile status

The reviewed authored source did not identify a current production CSV writer or
a canonical `.csv` output contract. SHAR therefore does not claim RFC 4180
conformance or define a repository-wide CSV dialect merely because earlier
research or tooling may have mentioned CSV.

### Repository boundary

The current source review found no production `.csv` writer or consumer. The two
`csv=p=0` literals are FFmpeg progress-format arguments and do not create a CSV
file contract. Introducing a CSV artifact requires its owning ADR and technical
specification to define delimiter, quoting, encoding, newline, schema, null,
type-rendering, control-character, and formula-neutralization behavior before
the artifact is treated as supported.

### Verified sources

- IETF (2005), *RFC 4180: Common Format and MIME Type for Comma-Separated Values
  Files*. <https://www.rfc-editor.org/rfc/rfc4180.html>
- Repository source inspection on 2026-07-13 found no production `.csv` writer
  or consumer.

## Source References

- IETF (2005) *RFC 4180: Common Format and MIME Type for CSV Files*. Available
  at: <https://www.rfc-editor.org/rfc/rfc4180.html> (Accessed: 12 July 2026).
- SHAR repository (2026), archive and manifest output contracts where CSV is
  produced.

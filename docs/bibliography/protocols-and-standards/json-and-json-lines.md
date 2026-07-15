# JSON And JSON Lines

This non-governing record documents an open data-interchange syntax and a
line-delimited convention without granting rights in encoded data, schemas, or
third-party implementations.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository behavior and authoritative JSON
  standards verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Repository interchange and manifest output.
- Subject class: Open data-interchange syntax and line-delimited convention.

## Covered Material

JSON texts used for normalized records, manifests, MCP requests, configuration
evidence, and JSON Lines files containing one independent JSON value per line.

## Repository Use And Scope

SHAR emits and validates strict JSON through Serde JSON and repository-owned
serializers. `game/manifest.jsonl` and other line-oriented outputs require each
line to be a complete JSON value with deterministic ordering and newline
handling.

## Provenance And Version History

RFC 8259 and ECMA-404 define JSON syntax and interoperability expectations. JSON
Lines is a widely used convention rather than an IETF or ECMA standard; its
framing requirements must therefore be stated explicitly by each repository
contract.

## Authorship, Ownership, And Attribution

The standards authors and standards bodies retain applicable rights in their
publications. JSON data retains the rights and provenance of its source. SHAR
contributors retain rights in independently authored schemas and serializers.

## License Or Terms Basis

Use of JSON syntax does not transfer rights in encoded data. Standards
publication terms, schema licenses, generated content, and bundled parser
libraries require separate review.

## Distribution, Modification, And Compatibility

Syntactic validity does not establish schema validity, semantic correctness,
provenance, authorization, or safe publication. JSON Lines readers must handle
line boundaries, final newlines, encoding, and malformed records
deterministically.

## Compliance Posture

- Use UTF-8 and emit complete JSON values without comments or trailing commas.
- Define JSON Lines framing and final-newline behavior in repository contracts.
- Validate schema, semantics, and provenance separately from syntax.
- Do not publish sensitive paths, identifiers, or third-party payloads merely
  because they are JSON.
- Record serializer and parser identities for consequential publication or
  distribution.

## Technical Baseline And SHAR Profile

### Public baseline

RFC 8259 and ECMA-404 define the JSON syntax baseline. RFC 8259 recommends
unique object-member names, warns that duplicate-name behavior is not
interoperable, requires UTF-8 for open-system exchange, and identifies the
exact-integer range commonly shared by IEEE 754 binary64 implementations.
ECMA-404 defines syntax rather than application semantics.

JSON Lines is a separate framing convention: UTF-8 text, one complete JSON value
per line, and an LF line terminator. Schema identity, member ordering, duplicate
rejection, and deterministic serialization remain application rules.

### SHAR profile

The game-manifest ledger uses a repository-owned canonical JSONL profile:

- the first line is a fixed taxonomy object with the repository-owned schema
  identity defined by the game-manifest implementation;
- requirement rows use the exact member order `dir`, `ext`, `min`, `kind`;
- rows are emitted without internal trailing whitespace;
- the complete file uses LF line endings and must end with LF;
- coordinates are ordered and duplicate coordinates are rejected;
- integer minima use canonical unsigned decimal syntax without redundant leading
  zeroes;
- string escaping covers quotation marks, reverse solidus, JSON short escapes,
  control characters, and valid surrogate pairs; and
- noncanonical row shapes, invalid taxonomy values, malformed escapes, and lone
  surrogates fail closed.

These rules are stricter than generic JSON and must not be generalized to every
JSON document in the repository.

### Verified sources

- IETF (2017), *RFC 8259: The JavaScript Object Notation Data Interchange
  Format*. <https://www.rfc-editor.org/rfc/rfc8259.html>
- Ecma International (2017), *ECMA-404: The JSON Data Interchange Syntax*.
  <!-- markdownlint-disable-next-line MD013 -->
  <https://ecma-international.org/publications-and-standards/standards/ecma-404/>
- JSON Lines, *JSON Lines*. <https://jsonlines.org/>
- SHAR repository evidence: `src/game-manifest/src/domain/json.rs`,
  `src/game-manifest/src/domain/domain.rs`, and
  `src/game-manifest/src/application/validate_manifest.rs`.

## Source References

- IETF (2017) *RFC 8259: The JavaScript Object Notation Data Interchange
  Format*. Available at:
  <https://www.rfc-editor.org/rfc/rfc8259.html> (Accessed: 12 July 2026).
- Ecma International (2017) *ECMA-404: The JSON Data Interchange Syntax*.
  Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://ecma-international.org/publications-and-standards/standards/ecma-404/>
  (Accessed: 12 July 2026).
- JSON Lines (n.d.) *JSON Lines documentation*. Available at:
  <https://jsonlines.org/> (Accessed: 12 July 2026).
- SHAR repository (2026), JSON serializers, schemas, and manifest contract.

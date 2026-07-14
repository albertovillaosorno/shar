# SPDX License Identifiers

This non-governing record documents a standardized license-identification
vocabulary without treating an identifier as a substitute for the controlling
license text, notices, provenance, or legal review.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository header use, official SPDX materials,
  and SPDX License List 3.28.0 latest-release identity and publication date were
  verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: Source-header and dependency-license identification.
- Subject class: Open license and exception identifier standard.

## Covered Material

SPDX license identifiers and expressions used in SHAR source headers, dependency
records, package metadata, and compliance inventories.

## Repository Use And Scope

SHAR uses SPDX identifiers such as `MIT` to state an intended license identity
in source headers. The identifier is machine-readable shorthand; the repository
license file and applicable notices remain controlling for the licensed
material.

## Provenance And Version History

The SPDX project maintains a versioned license list, expression grammar, and
specifications. Identifiers, deprecated entries, exceptions, matching guidance,
and list versions may change. A distribution inventory must record the list or
tool version used to normalize identifiers.

## Authorship, Ownership, And Attribution

The Linux Foundation, SPDX contributors, license authors, and rights holders
retain applicable rights in their respective materials. An SPDX identifier does
not assign authorship or ownership of a file.

## License Or Terms Basis

The SPDX specification and license-list data have their own licenses and terms.
An identifier does not alter the underlying license, cure missing permission, or
prove that the identified license applies to a particular file.

## Distribution, Modification, And Compatibility

License expressions must be matched to actual files, components, notices, and
exceptions. Automated classification can be wrong or incomplete. Compatibility,
source-offer, attribution, patent, copyleft, and notice obligations require the
controlling texts and distribution facts.

## Compliance Posture

- Pair each SPDX identifier with the controlling license file or authoritative
  upstream evidence.
- Record expressions and exceptions exactly; do not simplify them silently.
- Treat `NOASSERTION` and unknown results as unresolved, not permissive.
- Reverify deprecated or changed identifiers during publication or distribution
  review.
- Do not use an SPDX header to claim rights the contributor does not possess.

## Technical Baseline And SHAR Profile

### Public baseline

SPDX license identifiers and expressions are governed by the SPDX specification
and the official SPDX License List. The list is independently versioned and may
add, deprecate, or correct identifiers over time. The official release channel
identifies SPDX License List 3.28.0, published 20 February 2026, as the latest
release on 14 July 2026.

That observed list version is dated currentness evidence, not a permanent
repository pin, a minimum requirement, or a substitute for distribution-time
verification.

### SHAR profile

Repository-authored source headers use exact SPDX identifier syntax together
with the repository license file and copyright notice. An identifier names a
license or exception; it does not prove that the file is correctly licensed,
that all incorporated material is covered, or that distribution obligations are
satisfied.

Compound licensing must use a valid SPDX expression with explicit operators and
parentheses where required. Deprecated identifiers, custom references,
exceptions, generated files, vendored files, and files with mixed provenance
require separate evidence and must not be silently rewritten.

### Acceptance boundary

Distribution-time validation must record the SPDX specification and License List
versions used, verify every identifier and expression, map each distributed file
or component to its controlling license evidence, preserve required notices, and
report deprecated or unknown identifiers rather than guessing replacements.

### Verified sources

- SPDX, *SPDX License List*. <https://spdx.org/licenses/>
- SPDX, *SPDX Specification 2.3*, Annexes D and E.
  <https://spdx.github.io/spdx-spec/v2.3/>
- SHAR repository source headers and `LICENSE`.

## Source References

- SPDX Project (2026) *Version 3.28.0 of the SPDX License List*. Identified as
  the latest release, published 20 February 2026. Available at:
  <https://github.com/spdx/license-list-data/releases/tag/v3.28.0> (Accessed: 14
  July 2026).
- SPDX Project (n.d.) *SPDX specifications*. Available at:
  <https://spdx.dev/specifications/> (Accessed: 14 July 2026).
- SPDX Project (n.d.) *SPDX License List*. Available at:
  <https://spdx.org/licenses/> (Accessed: 14 July 2026).
- SPDX Project (n.d.) *Official GitHub organization*. Available at:
  <https://github.com/spdx> (Accessed: 14 July 2026).
- SHAR repository (2026), source headers and dependency-license records.

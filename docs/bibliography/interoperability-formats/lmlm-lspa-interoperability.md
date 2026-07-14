# LMLM And LSPA Interoperability

This non-governing record documents the repository-supported technical boundary
for a proprietary mod-container format family. It does not grant rights in any
launcher, mod, package, encrypted payload, service, API, asset, name, mark, or
user-supplied content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository parser behavior, the accepted
  `LSPA` container signature, structural validation, typed failures, and
  fail-closed extraction behavior are verified. A public format specification,
  complete variant history, public sample identity, ownership chain, and format
  license have not been established.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: User-supplied local interoperability input only.
- Subject class: Proprietary mod-container format family.

## Covered Material

The bounded LMLM and LSPA behavior required to recognize, validate, and extract
supported entries from an optional user-supplied local `.lmlm` archive.

The repository contract covers only the header fields, directory structure,
entry kinds, ranges, names, payload sizes, reserved-byte requirements, collision
rules, and publication behavior enforced by the current parser and tests. It does
not claim every historical or third-party LMLM or LSPA variant.

## Repository Use And Scope

SHAR contains an independently authored parser and extraction application for
bounded local interoperability. The parser validates the fixed `LSPA` signature
and supported header profile before reading entries. It rejects unsupported
versions, flags, entry kinds, malformed ranges, nonzero reserved regions,
trailing data outside the accepted profile, path collisions, and incomplete
publication.

The parser is not an asset-acquisition tool, launcher replacement, credential
client, service client, access-control bypass, or authorization to redistribute
an input archive or its contents. Technical claims in this record come from
repository-owned source and tests, not from a tutorial, transcript, download
page, or private acquisition event.

## Provenance And Version History

The repository records one current supported parser profile. No controlling
public specification, official historical version list, public sample identity,
producer statement, or complete relationship between the labels `LMLM` and
`LSPA` has been verified.

A successful parse proves only that an input satisfied the implemented profile.
It does not establish the input's author, ownership, license, authenticity,
current version, safety, or compatibility with another launcher or tool.

## Authorship, Ownership, And Attribution

The format, launchers, packages, mod content, translated audio, and game-derived
material may have different authors and rights holders. This record does not
assign ownership among them.

SHAR claims rights only in its independently authored parser, tests, synthetic
fixtures, and documentation to the extent supported by authorship evidence and
law.

## License Or Terms Basis

No verified public format license, complete specification, ownership chain, or
package-specific terms set has been identified. Public availability, a filename
extension, successful parsing, or local possession does not by itself create a
right to publish, sublicense, modify, or redistribute an archive or its contents.

Any contract, copyright, anti-circumvention, trademark, patent, trade-secret,
privacy, platform, or local-law issue requires separate analysis of the actual
facts. This bibliography record is not a legal conclusion.

## Distribution, Modification, And Compatibility

The repository may publish independently authored compatibility code, synthetic
fixtures, and abstract technical documentation within its legal and clean-room
boundaries. It must not publish user-supplied archives, extracted protected
content, launcher binaries, credentials, private endpoints, copied upstream
implementation, or direct acquisition instructions.

The parser rejects encrypted or unsupported content rather than attempting to
defeat an access control. That implementation fact does not decide whether a
contract, technological measure, or legal rule applies to another input.

## Compliance Posture

- Accept only user-supplied local input through the documented parser boundary.
- Fail closed on unsupported versions, flags, entry kinds, ranges, reserved
  bytes, trailing data, collisions, and incomplete output.
- Keep upstream source code, proprietary tools, credentials, private endpoints,
  archives, and extracted content outside the public repository.
- Do not infer authorization, authorship, ownership, safety, or redistribution
  rights from successful parsing.
- Do not claim general LMLM or LSPA compatibility without a separately verified
  variant matrix and reproducible tests.
- Stop when authorization, access-control status, or publication rights are
  materially unresolved.

## Source References

- SHAR repository (2026) independently authored LMLM parser, typed error model,
  extraction application, and synthetic regression tests.
- SHAR repository (2026) optional user-supplied local input and publication
  boundary.

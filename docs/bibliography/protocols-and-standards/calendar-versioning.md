# Calendar Versioning

This non-governing record documents SHAR's selected versioning family without
creating a release, changing package versions, or treating a date-based
identifier as proof of compatibility, stability, support, or legal compliance.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Official CalVer guidance and repository version
  surfaces verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not applicable as a versioning convention.
- As-of date: 2026-07-12.
- Distribution posture: Repository, package, schema, protocol, and artifact
  version metadata.
- Subject class: Calendar-based software versioning convention.

## Covered Material

Calendar Versioning, commonly called CalVer, including date-based segments such
as `YYYY`, `YY`, `0Y`, `MM`, `0M`, `WW`, `0W`, `DD`, and `0D`, plus optional
incrementing or modifier segments where the project defines them.

This record covers the versioning family only. SHAR's exact identity form and
its non-release meaning are controlled by the repository's accepted
[versioning ADR](../../adr/governance/versioning-commits-and-publication.md).

## Repository Use And Scope

SHAR uses a CalVer-derived form for repository-governed compatibility-snapshot
identities. Existing package, engine, toolchain, schema, protocol, and
third-party version fields may follow ecosystem-specific constraints and must
not be rewritten merely because SHAR uses that identity family.

A date-based identity does not by itself promise backward compatibility, API
stability, support length, freshness, security, migration safety, release
status, or a particular change class. Those properties require explicit
contracts, decisions, and validation evidence.

Upstream tool selection is governed separately from SHAR's own
release-versioning scheme. This record documents observed version evidence and
does not create upgrade policy.

## Provenance And Version History

CalVer is a family of calendar-based schemes rather than one universal format.
The official guidance recommends choosing a scheme that fits the project's
release and support model. It defines standard terminology for year, month,
week, day, micro, and modifier components and notes that projects may combine
calendar and incrementing segments.

The official guidance assumes the Gregorian calendar and UTC unless a project
states otherwise. SHAR must make its own calendar and timezone choice explicit
in the governing ADR.

## Authorship, Ownership, And Attribution

Mahmoud Hashemi and CalVer contributors retain applicable rights in the CalVer
website, repository, examples, and explanatory material. SHAR contributors
retain rights in independently authored versioning policy,
compatibility-snapshot metadata, and ADRs subject to the repository license.

A release date, tag, package version, or build identifier records metadata and
does not by itself establish authorship, ownership, review, approval, or legal
compliance.

## License Or Terms Basis

The official CalVer repository contains the source material for calver.org and
publishes its own license. Following the convention does not apply that license
to SHAR source, binaries, manifests, tags, or other published or distributed
artifacts.

Package ecosystems may impose syntax and ordering rules independent of CalVer.
Those ecosystem rules and the exact package metadata remain separately
controlling.

## Distribution, Modification, And Compatibility

Changing an ecosystem version number to CalVer can affect package-manager
ordering, upgrade comparisons, dependency ranges, caches, installers, update
channels, schemas, filenames, generated manifests, and user expectations. A
migration must be governed separately and tested in every affected ecosystem.

Multiple identities in the same calendar period require a deterministic
collision rule. SHAR's accepted ADR uses an increasing numeric suffix for
distinct compatibility snapshots accepted on the same day.

Conventional Commits may classify change intent, but SHAR does not derive its
CalVer identifier mechanically from commit types. Breaking-change disclosures
remain mandatory when a real contract changes, even though the version number is
calendar-based.

## Compliance Posture

- Apply the accepted ADR's `YY.M.V` compatibility-snapshot identity without
  leading zeroes and with a zero-based monthly snapshot sequence.
- Keep package, schema, protocol, artifact, and third-party versions under their
  own governing contracts unless another ADR changes that boundary.
- Document compatibility and breaking changes independently from the date.
- Do not treat a snapshot identity as a release, tag, support promise, or
  publication authorization.

## Source References

- Hashemi, M. and contributors (n.d.) *Calendar Versioning*. Available at:
  <https://calver.org/> (Accessed: 12 July 2026).
- Hashemi, M. and contributors (n.d.) *Official CalVer GitHub repository*.
  Available at: <https://github.com/mahmoud/calver> (Accessed: 12 July 2026).
- [Conventional Commits](conventional-commits.md).
- [Architecture Decision Records](architecture-decision-records.md).
- SHAR repository (2026), package manifests, project descriptors, schema
  versions, and version-drift policy.

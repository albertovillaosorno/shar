# LMLM And LSPA Interoperability

This non-governing record documents technical and acquisition provenance for a
proprietary mod-container and configuration format family. It does not grant
rights in any launcher, mod, package, encrypted payload, service, API, asset,
name, mark, or user-supplied content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository parser behavior verified;
  installation and creator-managed delivery instructions corroborated by
  operator-supplied transcripts; exact sample selection and download remain
  private operator evidence.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-13.
- Distribution posture: Local interoperability research only.
- Subject class: Proprietary mod-container and configuration format family.

## Covered Material

LMLM and LSPA container and configuration behavior needed to recognize,
validate, and extract supported data from an optional, user-supplied local
package.

## Repository Use And Scope

SHAR contains an independently authored parser for bounded local
interoperability. The parser must fail closed on unsupported headers,
unsupported package variants, malformed ranges, encryption indicators, and other
content outside its documented contract.

The repository owner used Jebano's installation tutorial identified in the
corresponding public-source record as the stable procedure reference. The
operator-supplied transcript states that the creator manages the delivery
locations, that the PC mod is packaged as `.lmlm`, and that users should select
the newest available version.

The operator then used a separate creator-published version announcement to
identify the newest version available at the time and obtained the PC package
from the creator-managed location. The announcement identifier, transient
version, and delivery locations remain private chain-of-custody evidence and are
not public acquisition instructions.

The sample was inspected locally through conventional file-format reverse
engineering. No third-party source code or private service access was used as
the implementation basis represented by this record.

The parser is not a general asset-acquisition tool, launcher replacement,
credential client, access-control bypass, or authorization to redistribute the
sample or its contents.

## Provenance And Version History

The exact sample version, file hash, download date, package variant, separate
version-announcement identifier, and creator-managed delivery source remain
private chain-of-custody evidence. They must be preserved locally without
publishing machine paths, transient announcement or delivery links, protected
payload inventories, or the sample itself.

The cited public installation tutorial establishes the stable procedure and
corroborates that the PC package uses the `.lmlm` extension and is obtained from
creator-managed locations. It does not identify the exact research sample or
make one transient version permanently current. The separate announcement used
for sample selection remains private provenance and does not establish a
complete format history, specification, ownership chain, or license lineage.

## Authorship, Ownership, And Attribution

The format implementation, launcher, package structure, mod content, translated
audio, game-derived material, tutorial, and linked download may have different
authors and rights holders. This record does not assign ownership among them.

SHAR claims rights only in its independently authored parser, tests, synthetic
fixtures, and documentation to the extent supported by authorship evidence and
law.

## License Or Terms Basis

No verified public format license, complete specification, ownership chain, or
binding terms record has been identified. Public availability, a download
instruction, successful parsing, or local possession does not by itself create a
right to publish, sublicense, modify, or redistribute the package or its
contents.

Any contract, copyright, anti-circumvention, trademark, patent, trade-secret,
privacy, platform, or local-law issue requires separate analysis of the actual
facts. This bibliography record is not a legal conclusion.

## Distribution, Modification, And Compatibility

The repository may publish independently authored compatibility code and
abstract technical documentation only within its legal and clean-room
boundaries. It must not publish the downloaded sample, extracted protected
content, launcher binaries, credentials, private endpoints, or copied upstream
implementation.

The current parser rejects encrypted or unsupported content rather than
attempting to defeat an access control. That implementation fact does not, by
itself, decide whether another technological measure, contract, or legal rule
applies.

## Compliance Posture

- User-supplied lawful local input only.
- No upstream source code or proprietary tool distribution.
- No private service endpoints, credentials, or account automation.
- No encrypted-content extraction or access-control bypass.
- No inference that successful parsing authorizes redistribution.
- No affiliation, endorsement, authorship, or ownership claim.
- Keep the sample hash, acquisition date, selected version, exact private
  announcement, and creator-managed delivery source outside Git.
- Do not publish the transient version-announcement URL or direct Drive,
  Discord, or download locations.
- Stop when authorization, access-control status, or publication rights are
  materially uncertain.

## Evidence Boundary

No controlling format specification, public format license, complete ownership
chain, package-specific terms set, or public sample identity is established by
the reviewed sources. The repository therefore supports only the bounded parser
behavior proven by synthetic tests and does not claim general LSPA compatibility
or redistribution authority.

## Repository Evidence

- `src/lmlm/` — independently authored parser and validation behavior.
- `README.md` — optional user-supplied local input boundary.
- Private operator evidence — sample hash, acquisition date, selected version,
  separate creator announcement, creator-managed delivery source, and local
  analysis record; excluded from Git and public documentation.

## Source References

- [Jebano Latin Spanish mod tutorial][jebano-source].
<!-- cspell:disable-next-line -- ESPAÑOL LATINO para Los -->
- Jebano (n.d.) \*¡Mod en ESPAÑOL LATINO para Los Simpson Hit & Run! |
  <!-- cspell:disable-next-line -- de Instalación -->
  Tutorial de Instalación\*. YouTube video `ZzXvcmzyoF4`. Available at:
  <https://www.youtube.com/watch?v=ZzXvcmzyoF4> (Accessed: 12 July 2026).
- SHAR repository (2026) `src/lmlm/`, independently authored parser and
  validation evidence.
- SHAR repository (2026) `README.md`, optional user-supplied local input
  boundary.
- Repository owner (2026) Operator provenance statement and supplied
  transcripts: the cited tutorial was used as the stable installation procedure;
  a separate then-current creator announcement was used to select and obtain the
  newest PC `.lmlm` package available at the time. The exact announcement,
  version, and delivery locations remain private. Unpublished project record.

[jebano-source]: ../research-sources/jebano-youtube-latin-spanish-mod-tutorial.md

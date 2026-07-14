# {Canonical Subject Name}

Each record covers one subject and documents source provenance, authorship,
ownership, intellectual-property notices, governing instruments, repository use,
scholarly evidence, and unresolved legal questions for that subject.

This non-governing record does not create repository policy, grant any right,
certify compliance, establish legal conclusions, or replace advice from
qualified counsel. Repository policy remains in the applicable architecture
decision records, licenses, contribution terms, and other authoritative files.

Before creating or updating a record, read the
[Bibliography Research Disclaimer](disclaimer.md) and the
[SHAR Documentation Guide](../README.md). Do not copy, paraphrase, or vary the
canonical legal disclaimer inside a subject record.

Replace every braced instruction that remains in an instantiated record. Do
not leave a required field blank. Use `Not applicable — {reason}` only after a
reasoned review. Use `Unknown — verification required` when evidence is absent
or inconclusive. Never convert uncertainty into a favorable assumption.

## Review Status And Scope

Every active record uses this baseline:

- Review status: {Open | Evidence recorded | Verified | Superseded | Archived}
- Evidence status: {Unverified | Partially verified | Verified | Disputed}
- Counsel review: {Not performed | Internal review | Counsel reviewed}
- As-of date: {YYYY-MM-DD}
- Subject class: {Neutral subject or related-family classification}

Add only the scoped fields that are material to the subject:

- Jurisdictional scope: {Required jurisdictions, `Not determined`, or
  `Not applicable — {reason}`}
- Legal scope: {Exact legal question or `Not applicable — {reason}`}
- Distribution posture: {Concise subject-specific description of repository use,
  publication, distribution, or local-only handling}
- Runtime or build dependency: {Yes | No | Conditional, with explanation}

Additional human-readable metadata may be added when it has one defined purpose
and does not duplicate another field. Do not publish preparer or model
identities, private review schedules, machine-specific routes, or private
evidence as bibliography metadata.

The top-level `Evidence status` field uses only its four listed values. Within
record prose and evidence ledgers, use the following claim-level labels
consistently:

- **Verified:** confirmed directly against a current primary or authoritative
  source whose identity and retrieval date are recorded.
- **Corroborated:** supported by at least two independent, reliable sources, but
  not confirmed by the controlling primary source.
- **Unverified:** reported or copied from a source that has not been checked
  against the controlling authority.
- **Inferred:** a reasoned conclusion derived from identified facts; it is not a
  quoted fact and must state its assumptions.
- **Disputed:** reliable sources conflict or the controlling interpretation is
  contested.
- **Unknown:** available evidence is insufficient to state a conclusion.

Apply these drafting rules throughout the record:

1. Separate source facts, reviewer interpretations, operational decisions, and
   unresolved questions.
1. Attribute every material factual proposition to a source identifier.
1. Give legal authorities a jurisdiction, effective date, and official source.
1. Quote only the minimum text necessary for accuracy and preserve exact
   wording.
1. Do not reproduce an entire license, terms document, standard, article, or
   documentation body unless reproduction is required, authorized, and verified.
1. Do not infer ownership, licensing authority, endorsement, compatibility, or
   permission from a name, logo, package location, repository host, or silence.
1. Record conflicts and uncertainty; do not resolve them by omission.
1. Distinguish the subject's own materials from bundled, linked, referenced,
   optional, user-supplied, and independently authored materials.
1. Treat "latest" as a dated factual claim, never as a permanent label. Record
   the observed version, its authoritative evidence, and the currentness
   verification date. When a governing decision requires latest-compatible
   selection or an explicit compatibility hold, cite that authority; this
   bibliography record does not create version-selection policy.

Every instantiated record retains these nine common second-level sections:
`Review Status And Scope`, `Covered Material`, `Repository Use And Scope`,
`Provenance And Version History`, `Authorship, Ownership, And Attribution`,
`License Or Terms Basis`, `Distribution, Modification, And Compatibility`,
`Compliance Posture`, and `Source References`. Every other review module below
is optional. Include one only when it contains material evidence or an explicit
unresolved boundary; omit unused modules instead of leaving instructions, empty
headings, or `Not applicable` boilerplate.

## Covered Material

### Subject Identity

- Canonical name: {Required}
- Subject key: {Lowercase machine-safe slug}
- Record filename: `{subject-key}.md`
- Primary function or description: {Neutral factual description}
- Observed update posture: {Current compatible version observed | Compatibility
  hold evidenced | Vendor-managed | Fixed historical edition | Not determined}
- Version observed by this record: {Exact version, edition, commit, release, or
  `Not applicable`}
- Version evidence: {Manifest, lockfile, package metadata, command output,
  official release page, or other authoritative source}
- Currentness verification date: {YYYY-MM-DD or `Not verified`}
- Known drift explanation: {Compatibility constraint, deliberate hold,
  unavailable package, delayed review, human oversight, or none known}
- Upstream project or issuing authority: {Required}
- Official publisher or distributor: {Required or reasoned unknown}
- Official website: {Canonical URL}
- Canonical source repository: {URL and host, when applicable}
- Package or artifact identifiers: {Package URL, registry id, CPE, DOI, ISBN,
  ISSN, standard number, docket number, or other stable identifiers}
- Primary language: {Language}
- Geographic origin, when legally material: {Jurisdiction and source}

### Included Components

List every component covered by this single-subject record. Separate components
when their rights, owners, licenses, or distribution terms differ.

| Component   | Version or date | Material type    | Included basis | Evidence  |
| :---------- | :-------------- | :--------------- | :------------- | :-------- |
| {Component} | {Value}         | {Code/docs/etc.} | {Why covered}  | {SRC-###} |

### Expressly Excluded Components

Record exclusions to prevent accidental license or ownership spillover.

| Excluded component | Reason excluded                      | Governing record or source |
| :----------------- | :----------------------------------- | :------------------------- |
| {Component}        | {Different subject, owner, or terms} | {Record or SRC-###}        |

### Identity Resolution

Document aliases, former names, forks, successor projects, acquired entities,
and confusingly similar names.

| Name or identifier | Relationship to subject       | Applicable dates | Evidence status      |
| :----------------- | :---------------------------- | :--------------- | :------------------- |
| {Name}             | {Alias/former name/fork/etc.} | {Dates}          | {Status and SRC-###} |

## Repository Use And Scope

### Repository Relationship

- Repository purpose for using the subject: {Required}
- Exact repository surfaces involved: {Paths, packages, build steps, or ADRs}
- Use mode: {Reference only | Development tool | Build tool | Runtime dependency
  | Optional integration | User-supplied input | Distributed component | Other}
- Acquisition mode: {Downloaded by user | Repository-managed | System-installed
  | Package-manager resolved | Referenced only | Other}
- Bundled in source distributions: {Yes | No | Conditional | Unknown}
- Bundled in binary distributions: {Yes | No | Conditional | Unknown}
- Invoked without redistribution: {Yes | No | Not applicable}
- Modified by the repository: {Yes | No | Unknown}
- Linked or combined with repository code: {Static | Dynamic | Process boundary
  | Data interchange | Not combined | Unknown}
- Network interaction: {None | API | Hosted service | Other}
- Outputs derived from the subject: {Describe or state none}
- User-supplied materials affected: {Describe or state none}
- Repository decision records: {Links to governing ADRs}
- Repository license interaction: {Describe without claiming compatibility}

### Operational Boundary

State precisely what the repository does and does not do with the subject.

- Repository-authored conduct: {Required}
- User-controlled conduct: {Required}
- Third-party-controlled conduct: {Required}
- Distribution boundary: {Required}
- Modification boundary: {Required}
- Data and telemetry boundary: {Required when a service or tool processes data}
- Authentication or account boundary: {Required when applicable}
- Geographic or export boundary: {Required when applicable}

### Non-Affiliation Statement

{State whether any affiliation, sponsorship, endorsement, certification,
partnership, or approval exists. When none exists, state that no such
relationship is represented. Do not use a trademark disclaimer as a substitute
for checking actual authorization.}

## Provenance And Version History

### Concise Provenance Narrative

{Describe the subject's origin, principal creators, institutional history,
transfers, forks, relicensing events, major ownership changes, and the lineage
of the version used by the repository. Every material statement must reference
one or more source identifiers. Distinguish confirmed history from inference.}

### Chronology

| Date   | Event   | Version or branch | Responsible entity | Legal relevance | Evidence  |
| :----- | :------ | :---------------- | :----------------- | :-------------- | :-------- |
| {Date} | {Event} | {Version}         | {Entity}           | {Why material}  | {SRC-###} |

### Version And Rights Lineage

| Reviewed version | Derived from  | Release date | Rights holder | Governing terms | Status          |
| :--------------- | :------------ | :----------- | :------------ | :-------------- | :-------------- |
| {Version}        | {Predecessor} | {Date}       | {Entity}      | {Instrument}    | {Verified/etc.} |

Record every known change in ownership, license, exception, contribution model,
or distribution channel that could affect the reviewed version. Do not assume
that current terms apply retroactively to historical releases.

## Authorship, Ownership, And Attribution

Authorship, copyright ownership, licensing authority, trademark ownership,
maintenance responsibility, and distribution control are different concepts.
Record each independently and cite the evidence for each statement.

### Persons And Entities

| Role                       | Person or entity | Applicable material | Applicable dates | Evidence  | Status   |
| :------------------------- | :--------------- | :------------------ | :--------------- | :-------- | :------- |
| Creator or original author | {Name}           | {Material}          | {Dates}          | {SRC-###} | {Status} |
| Principal maintainer       | {Name}           | {Material}          | {Dates}          | {SRC-###} | {Status} |
| Copyright claimant         | {Name}           | {Material}          | {Dates}          | {SRC-###} | {Status} |
| Current rights holder      | {Name}           | {Material}          | {Dates}          | {SRC-###} | {Status} |
| Licensor                   | {Name}           | {Material}          | {Dates}          | {SRC-###} | {Status} |
| Publisher or distributor   | {Name}           | {Material}          | {Dates}          | {SRC-###} | {Status} |
| Trademark owner            | {Name}           | {Mark}              | {Dates}          | {SRC-###} | {Status} |

### Contribution And Assignment Model

- Contribution mechanism: {Direct commits | Pull requests | Assignment | CLA |
  DCO | Employment | Commission | Unknown}
- Contributor terms source: {URL, file, version, and SRC-###}
- Assignment or license-back terms: {Exact source and summary}
- Employer or institutional ownership basis: {Evidence or unknown}
- Successor-in-interest evidence: {Evidence or unknown}
- Unresolved chain-of-title questions: {Required, including none found}

### Required Attribution

Record exact names, notices, acknowledgements, URLs, and placement requirements.
Do not normalize spelling, punctuation, capitalization, dates, or entity names
inside an exact notice.

| Attribution item | Exact text or reference            | Trigger   | Required location | Source    |
| :--------------- | :--------------------------------- | :-------- | :---------------- | :-------- |
| {Item}           | {Exact text or authoritative file} | {Trigger} | {Location}        | {SRC-###} |

## Scholarly Evidence And Source Quality

### Open Questions

- Primary research question: {Required}
- Secondary questions: {Required or none}
- Questions outside this record: {Required}

### Source Precedence

Use the source most authoritative for the proposition being supported. A useful
starting order is controlling legal authority, official license or terms text,
official release artifacts and metadata, official project documentation,
maintainer-authored materials, preserved archival copies, and reputable
secondary analysis. This order is not universal: explain any departure and do
not treat secondary commentary as controlling authority.

### Source Ledger

Assign a stable identifier to every source used by this record.

| Id      | Source class   | Author or issuer | Title   | Version/date | Locator    | Accessed | Integrity   | Status   |
| :------ | :------------- | :--------------- | :------ | :----------- | :--------- | :------- | :---------- | :------- |
| SRC-001 | {Primary/etc.} | {Name}           | {Title} | {Value}      | {URL/path} | {Date}   | {Hash/etc.} | {Status} |

For web sources, record the canonical URL, publication or effective date when
available, retrieval date, language, and archived locator. For repository
sources, record the host, owner, repository, tag or commit, path, and immutable
permalink. For legal authorities, record the jurisdiction, issuing body,
official identifier, effective date, amendment status, and official publication.

### Claim-To-Evidence Ledger

| Claim id | Factual proposition        | Source ids | Pinpoint location   | Evidence label | Reviewer note |
| :------- | :------------------------- | :--------- | :------------------ | :------------- | :------------ |
| CLM-001  | {One testable proposition} | {SRC-###}  | {Page/section/line} | {Label}        | {Note}        |

Do not combine multiple materially different propositions in one claim row.
Record negative findings, unavailable sources, withdrawn materials, and failed
verification attempts when they affect confidence.

### Conflicting Or Superseded Sources

| Source ids | Conflict or supersession | Resolution status     | Controlling basis      | Reviewer |
| :--------- | :----------------------- | :-------------------- | :--------------------- | :------- |
| {SRC-###}  | {Description}            | {Resolved/unresolved} | {Authority and reason} | {Name}   |

## Copyright And Related Rights

Copyright status and license status are separate. A work may be protected yet
licensed broadly, or publicly accessible yet not licensed for copying.

### Component-Level Rights Assessment

| Component   | Work type            | Claimed author | Claimed owner | Notice                | Status   | Evidence  |
| :---------- | :------------------- | :------------- | :------------ | :-------------------- | :------- | :-------- |
| {Component} | {Code/docs/art/etc.} | {Name}         | {Name}        | {Exact or none found} | {Status} | {SRC-###} |

### Copyright Notices

- Exact upstream notices: {Verbatim text or authoritative file references}
- Notice date ranges: {Required}
- Notice placement requirements: {Required}
- Registration or recordation data: {Jurisdiction, number, and source, or none}
- Publication status: {Published | Unpublished | Mixed | Unknown}
- Renewal or term evidence, when material: {Required or not applicable}
- Government-work status, when asserted: {Jurisdiction and authority}
- Public-domain status, when asserted: {Exact legal basis and primary authority}
- Crown copyright or comparable regime, when applicable: {Details}
- Moral-rights considerations, when applicable: {Jurisdiction and evidence}
- Database or catalogue rights, when applicable: {Jurisdiction and evidence}

Never label material `public domain`, `copyright-free`, `abandoned`, or
`unprotected` solely because no notice, owner, registration, or enforcement was
found.

## License Or Terms Basis

### Governing Instruments

List each instrument separately. Do not assume one license governs code,
documentation, examples, artwork, trademarks, hosted services, or historical
versions.

| Material   | Instrument           | Version/date | Identifier            | Issuer   | Authoritative source | Status   |
| :--------- | :------------------- | :----------- | :-------------------- | :------- | :------------------- | :------- |
| {Material} | {License/terms/etc.} | {Value}      | {SPDX or official id} | {Entity} | {SRC-###}            | {Status} |

- Controlling instrument for the reviewed use: {Required or unresolved}
- Instrument hierarchy or precedence clause: {Exact source}
- Choice-of-law clause: {Exact text reference and jurisdiction}
- Forum or dispute clause: {Exact text reference and forum}
- Effective date: {Required}
- Acceptance mechanism: {Clickwrap | Browsewrap | Installation | Use | Signed |
  Not applicable | Unknown}
- Parties identified by the instrument: {Required}
- Covered material definition: {Exact scope}
- Excluded material: {Exact scope}
- Additional terms, schedules, exceptions, or riders: {Required or none found}
- License-change history: {Required when versions differ}

### Grant And Reservation Of Rights

| Right or activity  | Expressly granted | Conditions   | Expressly reserved | Evidence  | Status   |
| :----------------- | :---------------- | :----------- | :----------------- | :-------- | :------- |
| Access or use      | {Yes/no/unclear}  | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Reproduction       | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Modification       | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Derivative works   | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Distribution       | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Public performance | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Public display     | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Sublicensing       | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Commercial use     | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Patent rights      | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |
| Trademark rights   | {Value}           | {Conditions} | {Reservations}     | {SRC-###} | {Status} |

### Conditions And Obligations

| Obligation                | Trigger   | Required act | Deadline | Delivery location | Evidence  | Status   |
| :------------------------ | :-------- | :----------- | :------- | :---------------- | :-------- | :------- |
| Preserve copyright notice | {Trigger} | {Act}        | {When}   | {Location}        | {SRC-###} | {Status} |
| Include license text      | {Trigger} | {Act}        | {When}   | {Location}        | {SRC-###} | {Status} |
| Disclose modifications    | {Trigger} | {Act}        | {When}   | {Location}        | {SRC-###} | {Status} |
| Provide source or offer   | {Trigger} | {Act}        | {When}   | {Location}        | {SRC-###} | {Status} |
| Provide attribution       | {Trigger} | {Act}        | {When}   | {Location}        | {SRC-###} | {Status} |
| State changes             | {Trigger} | {Act}        | {When}   | {Location}        | {SRC-###} | {Status} |
| Provide installation data | {Trigger} | {Act}        | {When}   | {Location}        | {SRC-###} | {Status} |
| Preserve notices          | {Trigger} | {Act}        | {When}   | {Location}        | {SRC-###} | {Status} |

Add rows for every applicable condition. Do not reduce a complex obligation to a
single label such as `permissive`, `copyleft`, `free`, `open`, or `proprietary`.

### Exceptions, Limitations, And Special Permissions

- Express exceptions: {Exact source and scope}
- Linking or classpath exception: {Exact source and scope}
- Documentation or example-code exception: {Exact source and scope}
- Additional permission: {Exact source and scope}
- Contributor exception: {Exact source and scope}
- Platform-specific terms: {Exact source and scope}
- Geographic restrictions: {Exact source and scope}
- Field-of-use restrictions: {Exact source and scope}
- Non-commercial restriction: {Exact source and scope}
- No-derivatives restriction: {Exact source and scope}
- Revocation or termination conditions: {Exact source and scope}
- Reinstatement or cure terms: {Exact source and scope}

### Compatibility And Combined-Work Questions

- Repository license: {Identifier and source}
- Subject license or terms: {Identifier and source}
- Combination mechanism: {Static/dynamic/process/data/reference/etc.}
- Claimed compatibility status: {Compatible | Incompatible | Conditional |
  Unresolved | Not applicable}
- Authority for the compatibility statement: {Required}
- Version-specific compatibility limits: {Required}
- Additional restriction analysis: {Required}
- Patent-clause interaction: {Required when applicable}
- Notice-stacking requirements: {Required}
- Counsel review required: {Yes/no and reason}

Do not state compatibility merely because two instruments are approved by an
organization, use similar labels, appear in the same dependency graph, or have
coexisted in another project.

### Authoritative Text Reproduction

- Full governing text included in this record: {Yes | No}
- Reproduction required by the instrument: {Yes | No | Unresolved}
- Permission or legal basis for reproduction: {Required when included}
- Authoritative source identifier: {SRC-###}
- Source text version or date: {Required}
- Source text hash: {Algorithm and digest}
- Exactness verified by: {Reviewer and date}
- Alterations from authoritative text: {None, or exact explanation}

Do not paste the full text by default. Prefer a separately tracked authoritative
license or notice file when the governing instrument requires distribution of
its text. When full text is reproduced, preserve it verbatim, identify its
source and version, and keep project-authored commentary outside the reproduced
text.

## Trademark, Patent, And Other Rights

### Trademarks And Branding

| Mark or branding element | Claimed owner | Registration or source | Repository use    | Permission status |
| :----------------------- | :------------ | :--------------------- | :---------------- | :---------------- |
| {Mark}                   | {Entity}      | {Record/SRC-###}       | {Nominative/etc.} | {Status}          |

- Logo or brand-asset use: {Describe or none}
- Naming-guideline source: {Required when applicable}
- Attribution or symbol requirement: {Required when applicable}
- No-endorsement requirement: {Required when applicable}
- Domain-name or product-name risk: {Describe or not applicable}

A copyright or software license does not ordinarily establish permission to use
names, logos, marks, trade dress, or branding as a source identifier. Record any
trademark permission separately.

### Patents

- Express patent grant: {Exact source and scope}
- Express patent reservation: {Exact source and scope}
- Patent retaliation or termination clause: {Exact source and scope}
- Identified patent claims or portfolios: {Official identifiers and sources}
- Standards-essential patent policy: {Source and scope}
- Patent-review status: {Not reviewed | Reviewed | Counsel reviewed}
- Unresolved patent questions: {Required}

Do not infer patent clearance from the absence of a patent notice or from a
copyright license that does not expressly address patents.

### Other Potential Rights Or Restrictions

- Trade secrets or confidential information: {Assessment and source}
- Rights of publicity or personality: {Jurisdiction and assessment}
- Privacy and personal-data obligations: {Jurisdiction and assessment}
- Contractual access restrictions: {Terms and source}
- Database rights: {Jurisdiction and assessment}
- Moral rights: {Jurisdiction and assessment}
- Export, sanctions, or controlled-technology concerns: {Assessment}
- Accessibility or consumer-law obligations: {Assessment when applicable}
- Sector-specific restrictions: {Assessment when applicable}

## Distribution, Modification, And Compatibility

Analyze each actual or reasonably planned repository scenario separately. A
license conclusion for internal use does not automatically apply to source,
binary, hosted, embedded, commercial, or modified distribution.

| Scenario                   | Material involved | Triggering act | Status   | Required actions | Evidence  | Reviewer |
| :------------------------- | :---------------- | :------------- | :------- | :--------------- | :-------- | :------- |
| Internal development use   | {Material}        | {Act}          | {Status} | {Actions}        | {SRC-###} | {Name}   |
| Source distribution        | {Material}        | {Act}          | {Status} | {Actions}        | {SRC-###} | {Name}   |
| Binary distribution        | {Material}        | {Act}          | {Status} | {Actions}        | {SRC-###} | {Name}   |
| Documentation distribution | {Material}        | {Act}          | {Status} | {Actions}        | {SRC-###} | {Name}   |
| Hosted or network service  | {Material}        | {Act}          | {Status} | {Actions}        | {SRC-###} | {Name}   |
| Modified redistribution    | {Material}        | {Act}          | {Status} | {Actions}        | {SRC-###} | {Name}   |
| User-supplied integration  | {Material}        | {Act}          | {Status} | {Actions}        | {SRC-###} | {Name}   |

Use only these status values unless a governing ADR defines stricter values:

- `Permitted — verified`
- `Permitted with conditions — verified`
- `Prohibited — verified`
- `Unresolved — legal review required`
- `Not applicable — reason recorded`

### Modification Record

- Upstream baseline: {Version, tag, commit, or artifact hash}
- Repository modifications: {Exact paths or patch set}
- Modification authors: {Names or accountable roles}
- Modification dates: {Dates}
- Change summary required by upstream terms: {Exact required wording or source}
- Modified-file notices required: {Requirements}
- Fork or derivative naming requirements: {Requirements}
- Source-availability obligations: {Requirements}
- Reproducible patch location: {Tracked path or approved external location}
- Whether modified material is distributed: {Yes/no/conditional}
- Derivative-work characterization: {Source fact, inference, or unresolved}

## Notices, Attribution, And Delivery Requirements

### Notice Package

| Notice artifact             | Required content | Trigger   | Destination               | Owner  | Verification |
| :-------------------------- | :--------------- | :-------- | :------------------------ | :----- | :----------- |
| License copy                | {Content/file}   | {Trigger} | {Source/binary/docs/etc.} | {Role} | {Status}     |
| Copyright notice            | {Exact text}     | {Trigger} | {Location}                | {Role} | {Status}     |
| Attribution notice          | {Exact text}     | {Trigger} | {Location}                | {Role} | {Status}     |
| Modification notice         | {Exact text}     | {Trigger} | {Location}                | {Role} | {Status}     |
| Source offer or source code | {Requirement}    | {Trigger} | {Location}                | {Role} | {Status}     |
| Warranty disclaimer         | {Requirement}    | {Trigger} | {Location}                | {Role} | {Status}     |
| Patent notice               | {Requirement}    | {Trigger} | {Location}                | {Role} | {Status}     |
| Trademark notice            | {Requirement}    | {Trigger} | {Location}                | {Role} | {Status}     |

### Delivery Verification

- Source package location: {Path or not applicable}
- Binary package location: {Path or not applicable}
- Installer or about-box location: {Path or not applicable}
- Documentation location: {Path or not applicable}
- Machine-readable metadata location: {Path or not applicable}
- Human-readable notice location: {Path or not applicable}
- Duplicate or conflicting notices: {Assessment}
- Notice preservation test: {Test or manual procedure}
- Packaging verification date: {Date}
- Packaging verifier: {Name or accountable role}

## Archival And Integrity Record

Preserve enough information to reproduce the review without committing source
material that the repository lacks permission to redistribute.

### Retrieval Record

- Retrieval date and time: {ISO 8601 with time zone}
- Retrieval method: {Browser, package manager, API, repository clone, etc.}
- Canonical locator: {URL or official identifier}
- Immutable locator: {Commit permalink, release URL, DOI, archive id, etc.}
- Archived locator: {Approved archive URL or `Not archived — reason`}
- Redirect chain reviewed: {Yes/no}
- Authentication required: {Yes/no and account class, never credentials}
- Content type: {MIME type}
- Character encoding: {Encoding}
- Byte length: {Value}
- Cryptographic digest: {Algorithm and digest}
- Digital signature: {Signer, format, verification result}
- Certificate chain: {Relevant identity only, no secret material}
- Local evidence location: {Ignored or approved evidence store, not a raw local
  machine path in tracked public text}
- Reproduction permission: {Permitted/prohibited/unresolved and source}

### Integrity And Authenticity Assessment

- Official-domain verification: {Result}
- Publisher identity verification: {Result}
- Release-signature verification: {Result}
- Hash comparison: {Result}
- Archive fidelity limitations: {Required}
- Dynamic-content limitations: {Required}
- Translation limitations: {Required}
- Missing attachments or incorporated documents: {Required}
- Known tampering, truncation, or corruption concerns: {Required}

A web archive or mirror may preserve evidence, but it does not automatically
become the controlling source. Record why an archived or mirrored copy is used
and whether it faithfully corresponds to an official version.

## Reference Forms

Use one selected Harvard author-date variant consistently for academic sources.
Record the institutional variant used by the repository; there is no permission
to mix incompatible house styles within one bibliography. Legal authorities must
use the official or jurisdiction-appropriate legal form in addition to the
academic reference when legal precision requires it.

### Selected Academic Style

- Style family: Harvard author-date
- Institutional variant: {University, publisher, or style guide}
- Style guide edition or date: {Required}
- Style guide source: {SRC-###}
- Name-order rule: {Required}
- Corporate-author rule: {Required}
- No-date rule: {Required}
- Access-date rule: {Required}
- Title-capitalization rule: {Required}
- Persistent-identifier rule: {Required}

### Academic Reference Templates

- **Web page:** {Author or organization} ({Year or `n.d.`}) *{Page title}*.
  {Website or publisher}. Available at: {URL} (Accessed: {Day Month Year}).
- **Software release:** {Author or organization} ({Year}) *{Software name}*
  (Version {version}) {Computer software}. {Publisher or repository}. Available
  at: {Persistent URL} (Accessed: {Day Month Year}).
- **Source repository:** {Owner or organization} ({Year}) *{Repository title}*
  (Commit {abbreviated immutable id}) {Source code}. {Host}. Available at:
  {Immutable URL} (Accessed: {Day Month Year}).
- **Standard:** {Issuing body} ({Year}) *{Identifier: title}*. {Edition}.
  {Place, when required}: {Publisher}.
- **Book or report:** {Author} ({Year}) *{Title}*. {Edition}. {Place, when
  required}: {Publisher}.
- **Article:** {Author} ({Year}) '{Article title}', *{Journal}*,
  {volume(issue)}, pp. {page range}. {DOI or persistent URL}.

Adapt punctuation only to the selected institutional variant. Do not invent a
publication date, author, place, publisher, DOI, edition, or access date.

### Legal Authority Reference Templates

- **Legislation or regulation:** {Jurisdiction}, {issuing body}, *{title}*,
  {official identifier}, {section or article}, {effective or publication date},
  {official source}.
- **Judicial decision:** *{Case name}*, {neutral or reporter citation}, {court},
  {year}, {pinpoint paragraph or page}.
- **Administrative or agency material:** {Agency}, *{title}*, {document or
  docket identifier}, {date}, {pinpoint section}, {official source}.
- **Contractual instrument:** {Issuing entity}, *{instrument title}*,
  {version/date}, {section}, {authoritative source}.

Use the form required by the applicable jurisdiction or reviewing institution.
Do not represent an unofficial summary, editorial note, blog, or search result
as the legal authority itself.

### Pinpoint Reference Rules

- Page or paragraph: {Required when stable pagination exists}
- Section, clause, or article: {Required for legal and license instruments}
- Repository path and lines: {Required for source evidence}
- Commit or release: {Required for mutable repositories}
- Timestamp: {Required for audiovisual evidence}
- Quoted language: {Exact text, quotation marks, and SRC-###}
- Translation: {Translator, source language, method, and verification status}

## Compliance Posture

### Overall Assessment

- Current posture: {Compliant evidence complete | Conditional | Remediation
  required | Unresolved | Not applicable}
- Assessment scope: {Exact use and distribution scenario}
- Assessment date: {YYYY-MM-DD}
- Assessor: {Name or accountable role}
- Counsel involvement: {None | Requested | Reviewed}
- Counsel memorandum or approval reference: {Private reference only, when
  appropriate; do not copy privileged analysis into this public record}
- Distribution blocker: {Yes/no and reason}
- Required remediation: {Actions, owners, and deadlines}
- Residual uncertainty: {Required}

This field records the review posture only. It must not state that the
repository or a distribution is legally approved unless an authorized human
reviewer has made that determination for the exact facts, jurisdiction, and
date.

### Obligation Checklist

- [ ] Subject identity and reviewed version are exact.
- [ ] Included and excluded components are distinguished.
- [ ] Authorship, ownership, and licensing authority have separate evidence.
- [ ] The controlling license or terms text has been retrieved and versioned.
- [ ] Historical versions and relicensing events have been reviewed.
- [ ] Repository use and distribution scenarios are accurately described.
- [ ] Modification and derivative-work questions are recorded.
- [ ] Copyright notices and attribution requirements are preserved.
- [ ] Source, license-copy, notice, and offer obligations are mapped.
- [ ] Trademark and branding use has been reviewed separately.
- [ ] Patent terms and unresolved patent questions are recorded.
- [ ] Privacy, publicity, database, moral-rights, and contractual issues were
      considered where applicable.
- [ ] Compatibility statements identify their authority and assumptions.
- [ ] Full-text reproduction, if any, is authorized and exact.
- [ ] Source locators, access dates, hashes, and archive details are recorded.
- [ ] Conflicting, superseded, or missing sources are disclosed.
- [ ] Academic references follow the selected Harvard variant consistently.
- [ ] Legal authorities use jurisdiction-appropriate official forms.
- [ ] No affiliation, endorsement, certification, or permission is implied.
- [ ] Unresolved issues have owners, deadlines, and distribution consequences.
- [ ] Required human or counsel review has been completed or explicitly blocked.

### Open Questions And Distribution Conditions

| Id    | Question or deficiency | Risk if unresolved | Owner  | Due date | Distribution effect     |
| :---- | :--------------------- | :----------------- | :----- | :------- | :---------------------- |
| Q-001 | {Question}             | {Risk}             | {Role} | {Date}   | {Block/condition/none}  |

### Review Triggers

Re-review this record when any of the following occurs:

- the repository changes how it acquires, combines, modifies, hosts, or
  distributes the subject;
- the subject publishes a new version, license, terms document, exception,
  policy, ownership statement, or contribution agreement;
- a source becomes unavailable, materially changes, or conflicts with another
  authoritative source;
- a new jurisdiction, customer, platform, packaging mode, or commercial use is
  introduced;
- a legal authority is amended, superseded, stayed, reversed, or reinterpreted;
- a security, provenance, authenticity, or chain-of-title concern is discovered;
- a distribution process no longer preserves required notices or source
  obligations; or
- the next mandatory review date is reached.

## Verification And Review

### Independent Verification

- Verification method: {Required}
- Reviewer independence: {Explain relationship to preparer}
- Sources re-opened and compared: {SRC-### list}
- Hashes or signatures rechecked: {Result}
- Material quotations checked: {Result}
- Legal authorities checked for currency: {Result and date}
- License text compared against distributed copy: {Result}
- Packaging and notice placement tested: {Result}
- Remaining limitations: {Required}

### Reviewer Sign-Off

| Review role            | Reviewer       | Date   | Scope   | Result   | Conditions   |
| :--------------------- | :------------- | :----- | :------ | :------- | :----------- |
| Technical provenance   | {Name}         | {Date} | {Scope} | {Result} | {Conditions} |
| Licensing review       | {Name}         | {Date} | {Scope} | {Result} | {Conditions} |
| Academic-source review | {Name}         | {Date} | {Scope} | {Result} | {Conditions} |
| Legal review           | {Name/counsel} | {Date} | {Scope} | {Result} | {Conditions} |

A signature or reviewer name records review activity; it does not create rights,
waive obligations, establish privilege, or guarantee legal compliance.

## Record Revision History

| Record version | Date         | Author | Change summary | Verification impact        |
| :------------- | :----------- | :----- | :------------- | :------------------------- |
| 0.1            | {YYYY-MM-DD} | {Name} | Initial draft  | Full verification required |

Do not erase prior material conclusions. When a conclusion changes, identify the
previous conclusion, the new evidence, the reason for change, and any affected
publication or distribution.

## Source References

### Primary Legal, License, And Terms Sources

- {SRC-###} {Complete reference in the selected academic and legal form}

### Official Project, Release, And Ownership Sources

- {SRC-###} {Complete reference}

### Archival And Integrity Sources

- {SRC-###} {Complete reference}

### Secondary Academic Or Professional Sources

- {SRC-###} {Complete reference and limitation}

### Sources Sought But Not Obtained

- {Source description, search date, access barrier, and effect on confidence}

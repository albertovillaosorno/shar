# SHAR Documentation Guide

The `docs/` directory is the public documentation surface for SHAR decisions,
repository-owned technical behavior, external evidence, legal research, and
bounded uncertainty.

This guide summarizes the current documentation model. It does not create a new
decision, legal conclusion, permission, or implementation contract. The
[decision and technical knowledge boundaries][knowledge-boundaries] remain the
governing authority.

## Start Here

- [Architecture decision records](adr/index.md) — durable repository decisions.
- [Technical specifications](technical/index.md) — current repository-owned
  behavior linked to governing decisions.
- [Bibliography and provenance](bibliography/index.md) — non-governing evidence,
  source, license, standard, tool, and provenance records.
- [Legal authorities and boundaries](legal/index.md) — dated legal research and
  fact-dependent application limits.

## Authority And Ownership

Every durable proposition has one owning documentation surface:

| Surface | Owns | Does not own |
| :------ | :--- | :----------- |
| ADR | Repository-impacting decisions | Commands, tutorials, implementation detail, or legal conclusions |
| Technical | Current repository-owned behavior | New decisions or proprietary external-format documentation |
| Bibliography | External evidence, provenance, confidence, and unresolved source questions | Architecture, legal permission, or operational authorization |
| Legal | Dated authority summaries, factual limits, and unresolved legal application | Technical architecture or permission to act |

When information is misplaced, move it to the owning record and update every
reference. Do not preserve obsolete files or duplicate one contract merely for
compatibility.

## Status Semantics

Status fields are local to their documentation family:

- ADR templates begin as `Proposed`; current cataloged decisions are `Accepted`.
- Technical templates begin as `Draft`; current cataloged specifications are
  `Active`.
- Bibliography `Review status` describes record lifecycle, while `Evidence
  status` describes proposition-level confidence. Neither grants permission or
  determines legal effect.
- Legal `Status` is a descriptive statement about authority verification and
  unresolved factual application. It is not an ADR, technical, or bibliography
  lifecycle value.

Do not compare unlike status fields or infer confidence, authority, permission,
or completeness from a status used by another documentation family.

## Architecture Decision Records

An active ADR:

- records one durable repository decision;
- declares status, decision date, and scope;
- separates context, decision, consequences, and rejected alternatives;
- avoids concrete repository paths, workstation preferences, commands,
  implementation tutorials, and proprietary external-format explanations;
- remains the authority for the decision until replaced by another current ADR.

Use the [ADR template](adr/template.md) and add the record to the
[ADR index](adr/index.md).

## Technical Specifications

An active technical specification:

- explains current repository-owned behavior in declarative present-tense prose;
- links at least one current governing ADR;
- describes the repository model, invariants, failure behavior, and
  verification;
- does not create policy, make architecture decisions, expose concrete
  repository paths, or document proprietary external formats;
- records only verified current limits rather than planned behavior.

Use the [technical specification template](technical/template.md) and add the
record to the [technical index](technical/index.md).

## Bibliography And Provenance

A bibliography record is non-governing. It documents one subject or explicitly
related family and separates repository evidence from external claims.

Every active record includes:

- review status and proposition-level evidence status;
- counsel-review status and an `As-of date`;
- covered material and repository use;
- provenance and source-quality limits;
- license or terms basis;
- distribution and compatibility boundaries;
- compliance posture and dated source references.

`Evidence recorded` means the bounded record is complete enough to retain. It
does not mean every proposition is verified. The top-level `Evidence status`
uses `Verified`, `Partially verified`, `Unverified`, or `Disputed`. Individual
claims may additionally be labeled `Corroborated`, `Inferred`, or `Unknown` when
the record identifies the supporting evidence, assumptions, or missing facts.

Public availability, successful parsing, attribution, or a download instruction
does not establish ownership, a license, permission, or redistribution rights.
Transient version announcements, direct delivery locations, local routes,
private hashes, credentials, proprietary samples, and extracted content remain
outside public documentation unless independently authorized and necessary.

Use the [bibliography record template](bibliography/template.md), read the
[bibliography research disclaimer](bibliography/disclaimer.md), and add the
record to the [bibliography index](bibliography/index.md).

## Legal Research

Legal records are dated research summaries, not legal advice or authorization.
Authority-analysis records identify the applicable jurisdiction or court,
authority, currentness, counsel-review status, verified propositions, missing
facts, contrary authority, and factual limits. Case records also identify the
decision date. Uncertainty remains explicit rather than being converted into a
categorical answer.

The [Legal Research Disclaimer](legal/disclaimer.md) is the single canonical
legal-scope notice. Link to it instead of copying or rewriting it in other
records. Use the [legal record template](legal/template.md) and add new records
to the [legal index](legal/index.md).

## Unresolved Research

SHAR does not maintain a standalone public research queue under `docs/`. An
unresolved question remains in the record that owns the proposition:

- bibliography records use evidence status and provenance sections for missing
  source, identity, version, or license evidence;
- legal records use dated `Not Established` or equivalent fact-dependent limits;
- technical records identify verification gaps only for current repository-owned
  behavior; and
- undecided repository choices remain `Proposed` ADRs rather than research notes.

Each unresolved item identifies the exact proposition, missing evidence, required
source class, governing jurisdiction or technical scope, and acceptance
condition. Verified findings update that same owning record. Missing private facts
are not inferred, and completed answer packets are not retained as a parallel
public authority.

## Editorial And Spelling Support

The files under `docs/cspell/` support repository spelling validation; they are
not documentation authorities or substitutes for source review:

- [`english.txt`](cspell/english.txt) contains genuine English words
  missing from the enabled standard dictionaries;
- [`technical.txt`](cspell/technical.txt) contains reusable technical terms,
  acronyms, formats, APIs, and repository-domain vocabulary;
- [`named-entities.txt`](cspell/named-entities.txt) contains verified
  people, organizations, products, characters, cases, and other proper names.

Keep every wordlist sorted, unique, and case-appropriate. Do not add malformed
text, synthetic fixture strings, one-off identifiers, or broad non-English
vocabulary merely to silence a finding. An intentional invalid sequence requires
an exact line-scoped CSpell suppression rather than a dictionary entry.

CSpell acceptance is editorial evidence only. It does not establish factual
accuracy, provenance, ownership, permission, or legal correctness.

## Public Content Boundary

Public documentation may describe repository-owned code, schemas, tests,
synthetic fixtures, lawful provenance, and independently verified facts. It must
not publish original game payloads, extracted proprietary assets, proprietary
engine source, credentials, private evidence, machine-specific state, or
unauthorized third-party replacement media.

Compatibility naming remains factual and visually independent. Documentation
must not imply affiliation, sponsorship, endorsement, approval, licensing, or
current official support.

Nothing in `docs/` grants permission, determines legality, authenticates
ownership, or authorizes copying, circumvention, publication, monetization, or
distribution. Read the canonical [Legal Research
Disclaimer](legal/disclaimer.md) before relying on legal material.

## Versioning And Publication Terminology

Repository-owned identities use Calendar Versioning in `YY.M.V` form and name
accepted compatibility snapshots. They are not releases. The repository
maintains no changelog, generated changelog, release notes, release branches,
release tags, or hosted releases.

Use publication or distribution terminology for public artifacts. Preserve
upstream projects' own release terminology when accurately describing their
software or source evidence.

## Maintenance Checklist

1. Choose the correct owning documentation surface.
1. Start from that surface's current template.
1. Write finished declarative prose, not template instructions or roadmap text.
1. Preserve uncertainty and distinguish observed facts from inference.
1. Cite current primary sources and record retrieval dates where applicable.
1. Keep private, proprietary, transient, and machine-specific evidence outside
   public documentation.
1. Add or update the owning index and verify every local link and anchor.
1. Update status, decision date, review date, or `As-of date` as appropriate.
1. Use the repository's canonical `validate.sh` flow. Do not substitute direct
   formatter or linter commands for canonical validation.

A validation environment failure must be reported accurately. Do not weaken a
rule, suppress a finding broadly, or modify an unrelated dependency merely to
claim a green documentation result.

[knowledge-boundaries]:
  adr/governance/documentation-and-knowledge-boundaries.md

# File Format Copyrightability

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: Governing authorities verified; format-specific result requires
  element evidence.
- Jurisdiction: United States baseline; other jurisdictions unresolved.
- Authority level: Cross-authority research record.
- As-of date: 2026-07-13.
- Counsel review: Not performed.

## Question Presented

Which elements of a file format are facts, ideas, processes, systems, methods of
operation, necessary interfaces, merger-bound choices, or scenes a faire, and
which elements may embody protectable original expression, selection, or
arrangement?

## Verified Baseline

A file-format label does not answer copyrightability. The analysis must identify
and filter the exact elements claimed: signatures, field names, byte order,
record hierarchy, offsets, flags, constants, defaults, commands, ordering,
examples, documentation, source code, and selection or arrangement.

17 U.S.C. § 102(b) excludes ideas, procedures, processes, systems, methods of
operation, concepts, principles, and discoveries from copyright protection, but
it does not make every expression used to describe or implement them free to
copy.

The verified authorities provide a balanced framework:

- *Google v. Oracle* did not decide interface copyrightability; it assumed the
  issue and resolved fair use on the specific record.
- *Lexmark* and *RJ Control* apply merger and scenes-a-faire principles where
  efficiency, hardware, compatibility, programming practices, or industry
  standards constrain expressive choice.
- *SAS v. World Programming* requires a claimant to identify specific
  protectable nonliteral elements and filter unprotectable material rather than
  rely on abstract categories such as input or output formats.
- *Premier Dealer* confirms that standard or functional components do not
  automatically eliminate protection from an independently original selection,
  coordination, or arrangement.
- The verified *Lotus* First Circuit opinion holds that the Lotus menu command
  hierarchy is uncopyrightable subject matter as a method of operation under 17
  U.S.C. § 102(b). The Supreme Court's equal-division affirmance created no
  nationwide majority rationale, and the Federal Circuit later rejected the
  First Circuit's reasoning while applying Ninth Circuit law. An official
  authenticated copy was not located in the reviewed official repositories, and
  current inside-circuit treatment has not been established here.

## Element-By-Element Analysis

For each asserted format element, record:

1. The exact bytes, text, label, sequence, grouping, or behavior.
1. Its source and first observed version.
1. Whether compatibility requires the same value, position, or ordering.
1. The technically viable alternatives and their consequences.
1. Applicable standards, hardware, platform, efficiency, or industry
   constraints.
1. Whether the element is a fact, command, method, rule, identifier, example,
   documentation passage, implementation code, or expressive arrangement.
1. Whether SHAR copied it, independently inferred it, or chose a distinct
   expression.
1. The claimed work, ownership, registration, and governing jurisdiction.

## Not Established

- That a filename extension or byte layout is categorically uncopyrightable.
- That every interface, command, field name, schema, or ordering is a method of
  operation.
- That the availability of multiple theoretical alternatives proves protection.
- That functional purpose alone defeats originality in selection or arrangement.
- That clean-room or independent implementation permits copying documentation,
  examples, comments, source code, or nonfunctional design choices.
- That copyright is the only relevant regime; patent, contract, trade-secret,
  trademark, and anti-circumvention law remain separate.

## SHAR Compliance Posture

- Implement only the functional behavior demonstrated as necessary by evidence.
- Use repository-authored names, comments, schemas, examples, and organization
  where compatibility does not require upstream expression.
- Record alternatives considered and the reasons a compatible value is required.
- Keep historical source bodies and non-public documentation outside public Git.
- Use synthetic fixtures and independent tests for published verification.
- Do not state that an entire proprietary format is unprotected or public
  domain.
- Obtain qualified counsel before publishing a contested detailed specification
  or relying on a format-wide copyrightability conclusion.

## Primary Authorities

- [17 U.S.C. § 102(b)](../statutes/17-usc-102b.md).
- [Google LLC v. Oracle America, Inc.](../cases/google-v-oracle.md).
- [Lexmark International, Inc. v. Static Control Components, Inc.](../cases/lexmark-v-static-control.md).
- [Lotus Development Corp. v. Borland International, Inc.](../cases/lotus-v-borland.md).
- [SAS Institute, Inc. v. World Programming Limited](../cases/sas-v-world-programming.md).
- [RJ Control Consultants, Inc. v. Multiject, LLC](../cases/rj-control-v-multiject.md).
- [Premier Dealer Services, Inc. v. Allegiance Administrators, LLC](../cases/premier-dealer-v-allegiance.md).

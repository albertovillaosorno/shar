# MDY Industries, LLC v. Blizzard Entertainment, Inc

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: Primary Ninth Circuit opinion verified.
- Court: United States Court of Appeals for the Ninth Circuit.
- Authority: 629 F.3d 928 (9th Cir. 2010), amended 2011-02-17.
- Decision date: 2010-12-14.
- As-of date: 2026-07-12.
- Counsel review: Not performed.

## Question Presented

How did the Ninth Circuit distinguish contractual covenants, copyright-license
conditions, infringement, and DMCA access-control liability in a software
compatibility dispute?

## Verified Holding And Record

The court held that a software-license breach constitutes copyright infringement
only when the breached term conditions the license's scope and the conduct
implicates an exclusive right under 17 U.S.C. § 106. The anti-bot provisions at
issue were contractual covenants rather than copyright conditions because their
breach did not independently implicate an exclusive copyright right.

The court treated 17 U.S.C. § 1201(a) as creating an access-control right
distinct from infringement. It rejected liability concerning literal code and
individual nonliteral elements that users could access from local storage
without defeating the challenged measure, while sustaining the addressed §
1201(a)(2) theory for dynamic nonliteral elements encountered only after the
access-control process.

The amended disposition did not decide a § 1201(f) defense because the issue was
not preserved in the manner required for appellate review.

## Limits And Procedural Boundary

- The covenant-versus-condition analysis depends on the exact agreement and the
  exclusive right allegedly implicated.
- The DMCA analysis depends on the exact work, measure, access route, tool, and
  statutory subsection.
- The case does not establish that every bot, compatibility tool, parser, or
  independently authored client is lawful.
- Contract damages and other claims may remain even when copyright infringement
  is not established.
- The Ninth Circuit's interpretation does not automatically control other
  jurisdictions.

## Repository Relevance

SHAR must analyze each agreement term separately from copyright and each alleged
technological measure separately from downstream use restrictions. A contractual
restriction does not become a copyright condition merely because it appears in a
software license, and a restriction on use is not necessarily an access control.

## Primary Sources

- United States Court of Appeals for the Ninth Circuit (2011), *MDY Industries,
  LLC v. Blizzard Entertainment, Inc.*, amended opinion. Available at:
  <https://cdn.ca9.uscourts.gov/datastore/opinions/2011/02/17/09-15932.pdf>
  (Accessed: 12 July 2026).
<!-- markdownlint-disable-next-line MD013 -->
- [Software-license reverse-engineering clauses](../contracts/software-license-reverse-engineering-clauses.md).
<!-- markdownlint-disable-next-line MD013 -->
- [17 U.S.C. § 1201 tool-distribution research](../statutes/17-usc-1201-trafficking.md).

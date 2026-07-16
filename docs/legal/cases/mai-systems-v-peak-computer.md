# MAI Systems Corp. v. Peak Computer, Inc

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: Reported Ninth Circuit opinion text verified from a public-domain
  reporter duplicate; an official archival court copy was not located.
- Court: United States Court of Appeals for the Ninth Circuit.
- Authority: 991 F.2d 511 (9th Cir. 1993).
- Dockets: 92-55363 and 93-55106.
- Decision date: 1993-04-07.
- As-of date: 2026-07-16.
- Counsel review: Not performed.

## Question Presented

Did an independent maintenance company's loading of licensed operating software
into customer-computer RAM create fixed copies outside the licenses and outside
§ 117's then-existing owner protections?

## Verified Holding And Record

The Ninth Circuit affirmed the copyright ruling against the independent service
company. Customer licenses permitted customers to load the software for their
own internal processing, but did not authorize third-party service technicians
to load or use it. The technician's acts therefore fell outside the license
scope on the reviewed record.

The court held that loading the operating software from storage into RAM created
a copy. The technician could view the system error log and use the RAM
representation to diagnose the machine, and the record did not establish that
the embodiment was too transitory to be fixed under § 101.

The court also treated the customers as licensees rather than owners of the
program copies and therefore held that they could not invoke § 117's owner-only
protection for the service company's copying. The opinion predates the Ninth
Circuit's later ownership framework in *Vernor v. Autodesk, Inc.*

## Later Statutory And Case Context

Congress later added § 117(c), which addresses copies created solely by
activating a lawfully programmed machine for maintenance or repair by the owner,
lessee, or an authorized service provider. That maintenance-specific rule did
not exist when *MAI Systems* was decided.

*Storage Technology Corp. v. Custom Hardware Engineering & Consulting, Inc.*
later applied § 117(c) in the Federal Circuit and found an independent service
company likely protected on its preliminary-injunction record. The machines'
owners authorized activation and maintenance, the RAM copies arose
automatically, and the copies were likely destroyed when the service engagement
ended.

## Limits And Procedural Context

- The RAM-copy holding depended on evidence that the representation could be
  perceived and used for diagnosis for more than a transitory duration.
- The opinion does not hold that every buffer, cache, register, packet, or
  momentary runtime state is a fixed copy.
- The license result depended on the exact customer license and lack of
  third-party authorization.
- The statement that licensed customers were not owners predates later ownership
  tests and should not be applied without current controlling authority.
- Section 117(c) now supplies a maintenance-specific rule with exact activation,
  use, access, destruction, machine, and authorization conditions.
- The case does not decide access-control circumvention, § 1201 trafficking,
  modern cloud transfers, or first sale.
- Copyright, contract, trade-secret, and employee-duty rulings in the broader
  opinion must remain separated.

## Repository Relevance

SHAR must identify every runtime copy instead of assuming that RAM is legally
invisible. A loaded program, decompressed payload, generated in-memory asset, or
diagnostic state may require fixation and authorization analysis when it remains
stable enough to be perceived, reproduced, or communicated.

At the same time, *MAI Systems* cannot be used to erase § 117(c), later
ownership law, or the requirement for exact technical evidence. Authorized
machine maintenance is distinct from content extraction, conversion,
modification, or repository publication.

## Required Facts

- The program copy, storage medium, license, owner or lessee, and service agent.
- The exact loading process, RAM region, duration, stability, and diagnostic
  use.
- Who authorized activation, use, copying, maintenance, and third-party access.
- Whether § 117(a) ownership or § 117(c) maintenance conditions are satisfied.
- Every buffer, cache, decompressed representation, output, and retained copy.
- The controlling circuit, later authority, and current agreement text.

## Sources

- Public.Resource.Org (n.d.), *MAI Systems Corporation v. Peak Computer, Inc.*,
  991 F.2d 511 (9th Cir. 1993), public-domain reporter duplicate. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://law.resource.org/pub/us/case/reporter/F2/991/991.F2d.511.92-55363.93-55106.html>
  (Accessed: 16 July 2026).
- [17 U.S.C. § 101](../statutes/17-usc-101.md).
- [17 U.S.C. § 117](../statutes/17-usc-117.md).
<!-- markdownlint-disable-next-line MD013 -->
- [Storage Technology Corp. v. Custom Hardware Engineering & Consulting, Inc.](storage-technology-v-custom-hardware.md).
- [Vernor v. Autodesk, Inc.](vernor-v-autodesk.md).

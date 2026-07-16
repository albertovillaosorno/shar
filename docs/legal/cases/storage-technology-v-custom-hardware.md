# Storage Technology Corp. v. Custom Hardware Engineering & Consulting, Inc

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: Reported Federal Circuit opinion text verified from a public-domain
  reporter duplicate; an official archival court copy was not located.
- Court: United States Court of Appeals for the Federal Circuit.
- Authority: 421 F.3d 1307 (Fed. Cir. 2005).
- Docket: 04-1462.
- Decision date: 2005-08-24.
- Procedural posture: Preliminary injunction vacated and case remanded.
- As-of date: 2026-07-16.
- Counsel review: Not performed.

## Question Presented

Did an independent service company likely infringe software copyrights or
violate § 1201(a) when it activated customer-owned storage machines, copied
embedded code into RAM, used maintenance diagnostics, and bypassed a maintenance
password protocol?

## Verified Holding And Record

The Federal Circuit vacated a preliminary injunction. It held that the service
company was likely to prevail on its § 117(c) defense for activation-created RAM
copies. Unlike the pre-§ 117(c) record in *MAI Systems Corp. v. Peak Computer,
Inc.*, the machine owners authorized the company to power on and maintain their
systems, the full program was automatically copied into RAM during startup, and
that copying was necessary to operate the machines.

The majority treated maintenance as broader than one isolated repair. Monitoring
an operating system for faults over an extended maintenance engagement could
qualify as servicing the machine to keep it working according to its authorized
specifications. On the reviewed record, rebooting when the maintenance
engagement ended destroyed the RAM copy and likely satisfied § 117(c)(1).

The majority also concluded that the diagnostic code was likely necessary for
activation within § 117(c)(2). It rejected both an interpretation covering every
program loaded during startup and an interpretation limited to the smallest code
needed for any response. The relevant question required examining the machine as
a functioning system and Congress's maintenance purpose.

The customers' software licenses supplied an alternative basis. They authorized
copying the full program into RAM to activate the equipment. The service
company, acting as the customers' maintenance agent, was therefore also likely
protected by the customers' activation rights even though separate use of the
maintenance code was restricted.

For the DMCA claim, the court applied *Chamberlain Group, Inc. v. Skylink
Technologies, Inc.* The maintenance devices bypassed a password protocol, but
the automatic RAM copy occurred whenever the equipment started, with or without
those devices. The court found no sufficient connection between circumvention
and infringement of a copyright right. A possible violation of contractual
maintenance restrictions was not itself a copyright right.

## Limits And Dissent

- The decision reviewed likelihood of success at the preliminary-injunction
  stage; it was not a final merits judgment after trial.
- Section 117(c) applies to copies made solely by activating a machine that
  lawfully contains an authorized program copy for maintenance or repair of that
  machine, subject to use, destruction, and access limits.
- The opinion does not turn repair, diagnostics, reverse engineering, or local
  modification into a general copyright exception.
- The majority's extended-maintenance and activation-necessity analysis depended
  on the integrated machine, startup behavior, service purpose, and record.
- The dissent viewed continuous monitoring and use of maintenance code as
  outside § 117(c), emphasizing limited-duration service and stricter necessity.
- The Federal Circuit's access-to-infringement nexus is not nationwide. *MDY
  Industries, LLC v. Blizzard Entertainment, Inc.* rejected that nexus in the
  Ninth Circuit.
- Contract, trade-secret, access-control, and copyright claims remain separate.
- A game-content modification is not machine maintenance merely because software
  is activated or diagnostic tools are used.

## Repository Relevance

SHAR must separate ordinary machine activation for authorized maintenance from
analysis or modification of game content. A maintenance provider needs evidence
that the owner or lessee authorized the work, the machine lawfully contains the
program copy, activation alone creates the copy, the purpose is only maintenance
or repair, the copy has no other use, and it is destroyed when that work ends.

A password or diagnostic-access bypass also requires a claim-specific DMCA
analysis. Under the Federal Circuit authority, a contractual restriction alone
did not establish the necessary copyright nexus. Other circuits may apply a
broader access-control rule, and no workflow may assume *StorageTek* controls
without resolving jurisdiction.

## Required Facts

- The machine, program, embedded copy, owner or lessee, and acquisition history.
- The exact activation process and every RAM or other copy it creates.
- The maintenance or repair objective and authorized specifications.
- Duration of service and when each activation-created copy is destroyed.
- Which code is necessary for activation and how other code is accessed or used.
- The service provider's agency and the customer's exact license rights.
- The technological measure, bypass, access obtained, and alleged infringement.
- Governing circuit law, later authority, agreements, and trade-secret duties.

## Sources

- Public.Resource.Org (n.d.), *Storage Technology Corporation v. Custom Hardware
  Engineering & Consulting, Inc.*, 421 F.3d 1307 (Fed. Cir. 2005), public-domain
  reporter duplicate. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://law.resource.org/pub/us/case/reporter/F3/421/421.F3d.1307.04-1462.html>
  (Accessed: 16 July 2026).
- [17 U.S.C. § 117](../statutes/17-usc-117.md).
- [MAI Systems Corp. v. Peak Computer, Inc.](mai-systems-v-peak-computer.md).
<!-- markdownlint-disable-next-line MD013 -->
- [17 U.S.C. § 1201 tool-distribution research](../statutes/17-usc-1201-trafficking.md).
<!-- markdownlint-disable-next-line MD013 -->
- [Chamberlain Group, Inc. v. Skylink Technologies, Inc.](chamberlain-v-skylink.md).
<!-- markdownlint-disable-next-line MD013 -->
- [MDY Industries, LLC v. Blizzard Entertainment, Inc.](mdy-v-blizzard.md).

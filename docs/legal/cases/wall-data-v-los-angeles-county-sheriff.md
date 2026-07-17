# Wall Data Inc. v. Los Angeles County Sheriff's Department

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: Reported Ninth Circuit opinion text verified from a public-domain
  reporter duplicate; an official archival court copy was not located.
- Court: United States Court of Appeals for the Ninth Circuit.
- Authority: 447 F.3d 769 (9th Cir. 2006).
- Docket: 03-56559.
- Decision date: 2006-05-17.
- Disposition: District-court judgment, damages, fees, and costs affirmed.
- As-of date: 2026-07-16.
- Counsel review: Not performed.

## Question Presented

Did a licensee infringe by installing software on more computers than its
license count when access controls limited simultaneous use, and did fair use or
§ 117's essential-step rule excuse those additional copies?

## Verified Holding And Record

The Sheriff's Department purchased 3,663 licenses for Wall Data's RUMBA
software but installed RUMBA Office on 6,007 computers through hard-drive
imaging. Its password and workstation controls allegedly kept simultaneous
access within the purchased license count.

The Ninth Circuit nevertheless held that the additional installations were
copyright-infringing copies. The reviewed license restricted use to a designated
computer and did not grant a concurrent-user license. Dormant installed copies
remained complete copies that an administrator could activate later.

The court emphasized that technical efficiency was not itself the problem. The
Department could have used imaging to install the software on the number of
computers covered by its licenses. The infringement arose from creating copies
beyond the negotiated installation scope while preserving flexibility that a
broader license would have supplied.

## Fair-Use Analysis

The court held that none of the four fair-use factors favored the Department.
The installations were exact copies used for the same operational purpose as
the licensed software. The Department obtained a practical and financial
benefit by avoiding additional per-computer licenses or a more flexible license.

The software was functional but remained copyrightable and reflected substantial
development investment. The Department copied the complete program, not a
limited portion needed for analysis or a different purpose. The dormant copies
also affected the licensing market because they could be activated without a
new installation and made actual usage difficult for the vendor to verify.

The decision does not hold that efficient deployment is categorically unfair.
Its result depended on complete same-purpose copies, installation beyond the
licensed count, and the reviewed per-computer license market.

## Section 117 Analysis

The court treated the Department as a licensee rather than an owner under the
then-controlling *MAI Systems Corp. v. Peak Computer, Inc.* framework because
its agreements imposed significant transfer and use restrictions. It therefore
could not rely on § 117(a)'s owner-only essential-step rule.

The court also held that the defense failed for a more fundamental reason even
if ownership were assumed. Imaging RUMBA Office onto nearly every computer was
a matter of convenience, timing, and deployment flexibility rather than an
essential step in using the licensed copies. The Department could have imaged
only the computers for which it had licenses.

*Vernor v. Autodesk, Inc.* later supplied the Ninth Circuit's current three-part
software-copy ownership framework. Current analysis must therefore apply
*Vernor* rather than treating *Wall Data*'s reliance on *MAI Systems* as the
final ownership test.

## Limits And Unresolved Questions

- The holding depends on a per-computer license, not a concurrent-user license.
- Installed but inaccessible copies were still complete hard-drive copies.
- The opinion does not make every deployment image, backup, or dormant file
  infringing.
- Imaging within the authorized installation count was not condemned.
- The court did not establish that every license-scope breach is copyright
  infringement; the copying and operative grant must be analyzed precisely.
- Later ownership authority, assent, contract formation, and exact license text
  remain necessary.
- Section 117 does not excuse copies created only for convenience or operational
  flexibility when they are not essential to using an owned program copy.
- Interoperability, reverse engineering, access controls, cloud deployment, and
  maintenance copies require separate authority and facts.

## Repository Relevance

SHAR must distinguish the number of protected copies created from the number of
copies simultaneously executable. A dormant installation, generated image,
container layer, virtual-machine snapshot, build cache, or packaged game copy
can require separate authorization even when runtime controls limit active
users.

Deployment automation should preserve the exact license model, authorized
installation count, device or user assignment, transfer rules, and deletion
behavior. Calling a system `concurrent`, `floating`, or `inactive` does not make
that license model or legal consequence true.

## Required Facts

- The exact program, version, license grant, and accepted agreement.
- Whether the license is per device, per user, concurrent, floating, or
  sitewide.
- Every installation, image, snapshot, cache, package, and dormant copy.
- The number of authorized and actual copies, users, devices, and activations.
- Access controls, administrator powers, auditability, and deletion procedures.
- Whether the claimant owns or licenses each program copy under current law.
- Why each additional copy was essential rather than merely convenient.
- The purpose, amount, market, and alternatives relevant to fair use.

## Sources

- Public.Resource.Org (n.d.), *Wall Data Incorporated v. Los Angeles County
  Sheriff's Department*, 447 F.3d 769 (9th Cir. 2006), public-domain reporter
  duplicate. Available at:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://law.resource.org/pub/us/case/reporter/F3/447/447.F3d.769.03-56559.html>
  (Accessed: 16 July 2026).
- [17 U.S.C. § 107](../statutes/17-usc-107.md).
- [17 U.S.C. § 117](../statutes/17-usc-117.md).
- [MAI Systems Corp. v. Peak Computer, Inc.](mai-systems-v-peak-computer.md).
- [Vernor v. Autodesk, Inc.](vernor-v-autodesk.md).
<!-- markdownlint-disable-next-line MD013 -->
- [Lawful Copy And Local Game Modification](../doctrines/lawful-copy-and-local-game-modification.md).

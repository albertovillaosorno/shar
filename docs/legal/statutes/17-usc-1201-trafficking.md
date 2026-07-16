# 17 U.S.C. § 1201(a)(2) And § 1201(b) — Tool Distribution

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: Current statutory structure verified; fact-specific application not
  determined.
- Jurisdiction: United States federal law.
- Authority level: Federal statute.
- Statutory currentness: Text contains laws in effect on June 24, 2026.
- As-of date: 2026-07-16.
- Counsel review: Not performed.

## Question Presented

Could a particular SHAR technology, product, service, component, command,
documentation package, or distribution practice fall within either statutory
trafficking prohibition?

## Verified Statutory Structure

Section 1201(a)(2) concerns technologies, products, services, devices,
components, or parts associated with measures that effectively control access to
a protected work. It addresses three alternative characteristics: primary design
or production for circumvention, only limited commercially significant purpose
or use apart from circumvention, or knowing marketing for circumvention.

Section 1201(b) separately addresses technologies, products, services, devices,
components, or parts associated with measures that effectively protect an
exclusive right of a copyright owner. Access-control and right-protection
analysis must not be collapsed into one category.

The temporary exemptions created through § 1201(a)(1) rulemaking do not by
themselves create a defense to § 1201(a)(2) or § 1201(b). Any permanent
exception, including § 1201(f), must be applied through its own exact
conditions.

*Chamberlain Group, Inc. v. Skylink Technologies, Inc.* required the claimant to
prove unauthorized access and, in the Federal Circuit, a connection between that
access and infringement of a copyright right. It rejected liability where
homeowners were authorized to use replacement transmitters and no infringement
or facilitating connection was alleged. The Ninth Circuit later rejected that
access-to-infringement nexus in *MDY Industries, LLC v. Blizzard Entertainment,
Inc.*, so the required relationship is jurisdiction-specific.

*Davidson & Associates, Inc. v. Jung* affirmed anti-trafficking liability for a
game-service emulator on a record involving circumvention of a server
authentication sequence, access to an online-service mode without valid unique
product keys, limited commercially significant purpose apart from avoiding the
service restrictions, and distribution of the emulator. The holding is
fact-specific and does not classify every compatible server or emulator as a
circumvention tool.

## Tool-Specific Evidence Required

For every challenged item, identify:

1. The protected work and exact technological measure.
1. Whether the measure controls access, protects an exclusive right, or both.
1. The ordinary authorized process required to gain access.
1. Every function of the item and the evidence of actual use.
1. The primary design and production purpose.
1. Commercially significant uses unrelated to circumvention.
1. Marketing, documentation, naming, demonstrations, and user instructions.
1. Distribution channels, recipients, territories, and versions.
1. Any § 1201(f), security-research, encryption-research, or other exception
   asserted and every condition supporting it.
1. Copyright-infringement and other-law analysis independent of § 1201.

## Not Established

- That parser error handling, absence of encryption support, or read-only design
  conclusively eliminates trafficking risk.
- That a general-purpose label proves substantial non-circumvention uses.
- That a tool's author intended only lawful use.
- That absence of infringement ends the § 1201 inquiry.
- That a temporary exemption authorizes manufacturing, offering, providing, or
  distributing a tool.
- That documentation or commands can never qualify as a service or component.

## Repository Posture

- Do not publish instructions intended to bypass an effective access control.
- Keep encrypted, authenticated, or access-controlled input outside supported
  workflows unless an independently reviewed legal basis exists.
- Describe legitimate parser and validation functions accurately without
  circumvention marketing.
- Preserve design, tests, uses, documentation, and build or distribution
  evidence for each distributed version.

## Primary Sources

- Office of the Law Revision Counsel, 17 U.S.C. § 1201:
  <!-- markdownlint-disable-next-line MD013 -->
  <https://uscode.house.gov/view.xhtml?req=granuleid:USC-prelim-title17-section1201&num=0&edition=prelim>
  (Accessed: 16 July 2026).
<!-- markdownlint-disable-next-line MD013 -->
- [Chamberlain Group, Inc. v. Skylink Technologies, Inc.](../cases/chamberlain-v-skylink.md),
  for the Federal Circuit's authorization, claimant-burden, and nexus analysis.
- [Davidson & Associates, Inc. v. Jung](../cases/davidson-v-jung.md), for the
  Eighth Circuit's game-service authentication and emulator analysis.
- [17 U.S.C. § 1201(f)](17-usc-1201f.md).
- [Temporary Section 1201 exemptions](37-cfr-201-40-temporary-exemptions.md).

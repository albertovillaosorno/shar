# Software Interoperability

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

<!-- cspell:ignore MGE -->

- Status: Governing authorities verified; workflow-specific result requires
  exact facts.
- Jurisdiction: United States baseline; other jurisdictions unresolved.
- Authority level: Cross-authority research record.
- As-of date: 2026-07-16.
- Counsel review: Not performed.

## Question Presented

Which copyright, anti-circumvention, contract, trade-secret, patent, trademark,
and procedural rules apply to the exact SHAR interoperability workflow?

## Verified Baseline

Interoperability is a technical relationship and possible purpose; it is not a
universal legal conclusion. Each legal regime has separate elements, defenses,
exceptions, remedies, jurisdictions, and factual predicates.

The verified authorities support these bounded propositions:

- *Google v. Oracle* assumed copyrightability and held the particular copying
  fair use. It did not hold that APIs or interfaces are categorically
  uncopyrightable.
- *Lexmark* rejected the asserted access-control theory where the challenged
  sequence did not prevent access to otherwise available program code. It did
  not create a universal compatibility exception.
- *Chamberlain* required proof of unauthorized access and an access-to-
  infringement nexus in the Federal Circuit where homeowners used compatible
  replacement transmitters with purchased equipment.
- *StorageTek* applied that nexus to maintenance-password devices and separately
  treated activation-created RAM copies as likely protected by § 117(c) and
  customer license rights on the preliminary-injunction record.
- *MDY* separates copyright-license conditions from contractual covenants,
  rejects the *Chamberlain* nexus in the Ninth Circuit, and treats § 1201(a)
  access-control analysis as distinct from infringement.
- *MGE UPS Systems* requires actor-specific circumvention proof in the Fifth
  Circuit. Using an already altered copy did not itself prove that the accused
  employees disabled or bypassed the measure. Its earlier 612 F.3d 760 opinion
  was withdrawn and is not controlling.
- *Davidson* enforced accepted reverse-engineering restrictions, sustained DMCA
  claims involving a game-service authentication bypass, and rejected § 1201(f)
  where the emulator omitted product-key validation and enabled unauthorized
  copies to use protected online-service features.
- *Nguyen* and *Berman* require fact-specific online notice and assent analysis;
  wrap labels alone do not determine formation.
- *SAS*, *RJ Control*, and *Premier Dealer* require granular identification and
  filtration of software or format elements while preserving the possibility of
  protection for independently original unconstrained expression or arrangement.
- The *Sega* and *Connectix* opinions are verified with reporter pinpoints from
  permitted secondary duplicates: intermediate copying can infringe, and it is
  fair use where a legitimate purpose exists and no other means of access to the
  unprotected elements is available. Official authenticated official copies were
  not located in the reviewed court repositories.
- The *Lotus* First Circuit holding (menu command hierarchy as an
  uncopyrightable method of operation), the Supreme Court's equally divided
  affirmance, and material outside-circuit treatment are verified; an official
  authenticated First Circuit copy was not located in the reviewed official
  repositories, and current inside-circuit treatment has not been established.
- The *Specht* holding (no assent where terms sat below the download button) is
  verified from a permitted secondary duplicate; an official authenticated copy
  was not located in the reviewed official repositories, and later controlling
  Second Circuit treatment has not been established here.

## Claim-Separation Matrix

<!-- markdownlint-disable MD013 -->

| Issue                    | Required factual focus                                                                                      | Principal records                                                                                                                                                                                    |
| :----------------------- | :---------------------------------------------------------------------------------------------------------- | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Copyrightability         | Exact element, originality, function, constraints, merger, selection, and arrangement                       | `google-v-oracle.md`, `lexmark-v-static-control.md`, `sas-v-world-programming.md`, `rj-control-v-multiject.md`, `premier-dealer-v-allegiance.md`                                                     |
| Fair use                 | Purpose, character, necessity, amount, final output, market, alternatives, and jurisdiction                 | `sega-v-accolade.md`, `sony-v-connectix.md`, `google-v-oracle.md`                                                                                                                                    |
| Access controls          | Protected work, measure, ordinary access route, circumvention act, tool design, marketing, and distribution | `chamberlain-v-skylink.md`, `storage-technology-v-custom-hardware.md`, `lexmark-v-static-control.md`, `mdy-v-blizzard.md`, `davidson-v-jung.md`                                                      |
| Contract                 | Exact terms, notice, assent, condition or covenant, scope, governing law, and remedy                        | `davidson-v-jung.md`, `mdy-v-blizzard.md`, `bowers-v-baystate-technologies.md`, `vault-v-quaid-software.md`, `nguyen-v-barnes-and-noble.md`, `berman-v-freedom-financial.md`, `specht-v-netscape.md` |
| Confidential information | Source, secrecy, acquisition, duties, exposure, independent derivation, and disclosure                      | `../statutes/18-usc-1836-and-1839.md`                                                                                                                                                                |
| Patent                   | Exact algorithm or claim, territory, legal status, implementation, and license                              | `patent-and-codec-risk.md`                                                                                                                                                                           |
| Trademark                | Exact public presentation, mark, owner, goods or services, audience, and confusion evidence                 | `trademark-and-compatibility-naming.md`                                                                                                                                                              |

<!-- markdownlint-enable MD013 -->

## SHAR Fact Matrix Required

Before relying on any interoperability authority, the repository record must
identify:

1. The exact lawfully acquired program, data, package, or service.
1. Every applicable agreement and assent event.
1. The exact information sought and why it is necessary for compatibility.
1. Whether equivalent information was readily available through another route.
1. Every intermediate copy, observation, trace, or transformation made.
1. Every technological measure encountered and what access it actually controls.
1. The independently authored final code and any upstream expression it
   contains.
1. The public and private outputs, recipients, distribution channels, and
   territories.
1. The actual and reasonably likely markets or licensing markets affected.
1. The evidence of independent derivation and the people or agents exposed to
    each source.

## Not Established

- That independent implementation resolves every claim.
- That lawful possession authorizes every analysis, modification, or
  publication.
- That noncommercial, educational, preservation, or interoperability purpose is
  automatically fair use.
- That § 1201(f) applies without satisfying every statutory condition.
- That lack of copyright infringement eliminates DMCA, contract, patent,
  trademark, trade-secret, privacy, or platform risk.
- That a parser's rejection of encrypted input conclusively resolves trafficking
  or access-control analysis.
- That United States law governs every user, act, agreement, or distribution.

## Compliance Posture

- Keep original and extracted proprietary payloads outside Git and distributed
  artifacts.
- Publish independently authored code and synthetic or otherwise authorized
  fixtures only.
- Preserve source, hash, acquisition, exposure, test, and implementation
  history.
- Fail closed on encrypted, authenticated, unsupported, or ambiguous input.
- Separate legal research from technical decisions and source implementation.
- Reverify current law, operative agreements, and jurisdiction before disputed
  publication or distribution.
- Obtain qualified counsel before relying on a contested exception, defense, or
  enforceability conclusion.

## Primary Authorities

- [Google LLC v. Oracle America, Inc.](../cases/google-v-oracle.md).
<!-- markdownlint-disable-next-line MD013 -->
- [Chamberlain Group, Inc. v. Skylink Technologies, Inc.](../cases/chamberlain-v-skylink.md).
<!-- markdownlint-disable-next-line MD013 -->
- [Storage Technology Corp. v. Custom Hardware Engineering & Consulting, Inc.](../cases/storage-technology-v-custom-hardware.md).
- [Davidson & Associates, Inc. v. Jung](../cases/davidson-v-jung.md).
<!-- markdownlint-disable-next-line MD013 -->
- [Lexmark International, Inc. v. Static Control Components, Inc.](../cases/lexmark-v-static-control.md).
<!-- markdownlint-disable-next-line MD013 -->
- [MDY Industries, LLC v. Blizzard Entertainment, Inc.](../cases/mdy-v-blizzard.md).
- [Sega Enterprises Ltd. v. Accolade, Inc.](../cases/sega-v-accolade.md).
<!-- markdownlint-disable-next-line MD013 -->
- [Sony Computer Entertainment, Inc. v. Connectix Corp.](../cases/sony-v-connectix.md).
- [17 U.S.C. § 1201(f)](../statutes/17-usc-1201f.md).
- [17 U.S.C. § 107](../statutes/17-usc-107.md).

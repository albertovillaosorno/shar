# Google LLC v. Oracle America, Inc

> [Legal Research Disclaimer](../disclaimer.md) applies to this record.

- Status: Primary Supreme Court opinion verified.
- Court: Supreme Court of the United States.
- Authority: 593 U.S. 1 (2021).
- Decision date: 2021-04-05.
- As-of date: 2026-07-12.
- Counsel review: Not performed.

## Question Presented

What did the Supreme Court decide about Google's copying of Java API declaring
code when creating Android?

## Verified Holding And Record

The Court expressly assumed, for argument's sake, that the copied material was
copyrightable and decided the case on fair use. The record involved roughly
11,500 lines of declaring code associated with 37 Java API packages. Google
wrote the implementing code used by Android independently.

The Court held the copying fair use as a matter of law on the record before it.
Its analysis emphasized the programmer-facing interface, the functional context
of computer programs, the transformative purpose of enabling programmers to use
familiar calls in a new computing environment, the amount copied relative to the
larger work, and the market evidence.

The Court also stated that the ultimate fair-use determination is a legal
question, while underlying historical facts may remain for a factfinder.

## Limits And Contrary Reasoning

- The Court did not decide whether the copied API material was copyrightable.
- The decision does not hold that every API, interface, command set, or file
  format is uncopyrightable.
- The decision does not make every reimplementation or compatibility use fair.
- The decision does not resolve contract, patent, trademark, trade-secret, or
  anti-circumvention issues.
- Justice Thomas, joined by Justice Alito, dissented and would have reached a
  different result on copyrightability and fair use.

## Repository Relevance

The opinion supports a fact-specific inquiry into the nature of interface
material, the purpose and necessity of copying, the independently authored final
implementation, the amount taken, and market effects. It does not supply a
categorical legal status for SHAR parsers or binary formats.

## Open Questions

- Which SHAR elements, if any, are analogous to declaring code or a
  programmer-facing interface?
- What material was copied, if any, rather than independently inferred?
- What amount was necessary for compatibility, and what alternatives existed?
- Does any final output contain protected upstream expression?

## Primary Sources

- Supreme Court of the United States (2021), *Google LLC v. Oracle America,
  Inc.*, 593 U.S. 1. Available at:
  <https://www.supremecourt.gov/opinions/20pdf/18-956_d18f.pdf> (Accessed: 12
  July 2026).
- [17 U.S.C. § 107](../statutes/17-usc-107.md).
- [Software interoperability research](../doctrines/software-interoperability.md).

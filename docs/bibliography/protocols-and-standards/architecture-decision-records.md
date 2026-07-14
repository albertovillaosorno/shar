# Architecture Decision Records

This non-governing record documents the architecture-decision-record practice
used by SHAR without treating an ADR as a statute, contract, license, proof of
legal compliance, or substitute for implementation evidence.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository use, the original Nygard article, and
  the ADR community reference site verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not applicable as a documentation practice.
- As-of date: 2026-07-12.
- Distribution posture: Repository architecture and engineering decision
  history.
- Subject class: Lightweight architectural knowledge-management practice.

## Covered Material

Architecture Decision Records under `docs/adr/`, including context, decision,
status, consequences, supersession, cross-references, implementation evidence,
and the relationship between architecture decisions and technical documentation.

## Repository Use And Scope

SHAR uses ADRs to preserve architecturally significant decisions affecting
structure, quality attributes, dependencies, interfaces, construction methods,
validation, data boundaries, and repository policy. Technical facts in
bibliography or research records may cite the responsible ADR, but the ADR does
not replace upstream evidence or source-code verification.

Legal research and legal conclusions do not belong in ADRs. ADRs may state a
repository risk boundary or link to `docs/legal`, but statutes, cases,
contracts, licenses, ownership research, and jurisdictional analysis remain
under `docs/legal`.

## Provenance And Version History

Michael Nygard's 2011 article popularized a lightweight record with context,
decision, status, and consequences. The ADR community describes an ADR as a
record of one architecturally significant decision and its rationale, with the
collection forming a decision log. SHAR may use stricter repository-specific
fields and validation rules.

## Authorship, Ownership, And Attribution

Nygard, Cognitect, ADR community contributors, and other authors retain
applicable rights in their publications and templates. SHAR contributors retain
rights in independently authored repository ADRs subject to the repository
license. Citing a template does not transfer authorship of project decisions.

## License Or Terms Basis

The original Nygard article states a CC0 dedication to the extent possible under
law. Other ADR templates, tools, and guidance may use different licenses. SHAR
must review the exact source before copying substantial template text or
tooling.

## Distribution, Modification, And Compatibility

An ADR records a decision at a point in time. It may become superseded,
deprecated, contradicted by implementation, or stale as context changes. An
accepted status is an internal governance state, not external certification.

## Compliance Posture

- Keep one architecturally significant decision per ADR.
- Preserve superseded records and link to replacements rather than rewriting
  history invisibly.
- Cite source, test, configuration, and upstream evidence for material facts.
- Keep legal analysis in `docs/legal` and bibliography evidence in
  `docs/bibliography`.
- Update or supersede ADRs when implementation or context materially changes.
- Use `validate.sh` to verify ADR structure, links, and repository policy.

## Source References

- Nygard, M. (2011) *Documenting Architecture Decisions*. Available at:
  <https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions>
  (Accessed: 12 July 2026).
- ADR GitHub organization (n.d.) *Architectural Decision Records*. Available at:
  <https://adr.github.io/> (Accessed: 12 July 2026).
- SHAR repository (2026) `docs/adr/`, repository ADR policy, and canonical
  validation rules.

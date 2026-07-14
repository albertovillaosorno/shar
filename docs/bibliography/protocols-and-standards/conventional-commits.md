# Conventional Commits

This non-governing record documents a commit-message convention without creating
a Git commit, changing repository history, or treating message syntax as proof
of code quality, authorship, licensing, or publication or distribution
correctness.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Official Conventional Commits specification and
  repository commit-governance requirements verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not applicable as a commit-message convention.
- As-of date: 2026-07-12.
- Distribution posture: Human- and machine-readable change-history metadata.
- Subject class: Lightweight structured commit-message specification.

## Covered Material

Conventional Commits 1.0.0 message structure, including type, optional scope,
optional breaking-change marker, description, body, and footers. It also covers
the relationship between commit messages, Git trailers, changelog generation,
and release-automation inputs.

## Repository Use And Scope

SHAR uses conventional-style commit subjects to communicate the primary nature
and scope of a bounded repository change. Repository-specific rules may be
stricter than the upstream specification, including permitted types, scope
vocabulary, body requirements, atomicity, validation evidence, authorship, and
co-author trailers.

This record does not authorize a commit. Repository changes remain uncommitted
unless the operator explicitly requests a commit after validation.

## Provenance And Version History

The Conventional Commits project publishes a versioned specification. Version
1.0.0 defines a required type and description, optional scope, optional body,
optional footers, and two forms of breaking-change indication. It requires
`feat` for features and `fix` for bug fixes, while allowing additional types.

The upstream specification includes an optional mapping from commit types to a
different release-numbering system. SHAR does not adopt that mapping. Commit
messages do not independently determine a CalVer identifier. SHAR has no release
automation, release branches, changelog, or hosted releases; the accepted
versioning ADR controls those boundaries.

## Authorship, Ownership, And Attribution

Conventional Commits contributors retain applicable rights in the specification,
website, translations, and implementation repository. SHAR contributors retain
rights in independently authored commit messages and repository changes subject
to applicable law and repository policy.

A commit author, committer, signer, or trailer records metadata and does not by
itself prove copyright ownership, authorization, originality, review, or
employment status.

## License Or Terms Basis

The published specification page identifies its specification text under
Creative Commons Attribution 3.0. The official website repository identifies its
implementation content under the MIT License. Those licenses do not apply to
SHAR source or commit history merely because SHAR follows the convention.

## Distribution, Modification, And Compatibility

A syntactically conforming message may still describe a non-atomic, misleading,
unvalidated, unauthorized, or legally problematic change. Automated changelog or
release systems must not treat a type label as stronger evidence than the actual
diff, tests, contract analysis, and CalVer policy.

Breaking changes must be identified based on the real affected contract, not
merely the presence or absence of `!` or a `BREAKING CHANGE` footer. Revert,
merge, squash, cherry-pick, and history-rewrite workflows require separate
repository rules.

## Compliance Posture

- Use the repository-approved type and scope vocabulary.
- Keep each commit bounded to one coherent change.
- Make the subject accurately describe the verified diff.
- Put explanatory context and validation evidence in the body when required.
- Use trailers only for accurate, authorized metadata.
- Mark breaking changes from actual contract analysis.
- Do not derive a CalVer identifier mechanically from `feat`, `fix`, or other
  commit types.
- Do not commit, amend, rebase, sign, tag, or push without operator authority.
- Reverify the current repository commit rules before every commit.

## Source References

- Conventional Commits contributors (n.d.) *Conventional Commits 1.0.0*.
  Available at: <https://www.conventionalcommits.org/en/v1.0.0/> (Accessed: 12
  July 2026).
- Conventional Commits contributors (n.d.) *Official GitHub repository*.
  Available at:
  <https://github.com/conventional-commits/conventionalcommits.org> (Accessed:
  12 July 2026).
- Git Project (n.d.) *git-interpret-trailers documentation*. Available at:
  <https://git-scm.com/docs/git-interpret-trailers> (Accessed: 12 July 2026).
- [Calendar Versioning](calendar-versioning.md).
- SHAR repository (2026), repository commit-message and validation policy.

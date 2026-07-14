# ripgrep

This non-governing record documents ripgrep (`rg`) as an operator-facing source
search utility. It is not a canonical validator, source authority, or substitute
for repository-specific file inventories and legal review.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Installed ripgrep 15.1.0 revision
  `af60c2de9d`, PCRE2 capability, local role, latest signed upstream release,
  release date, and upstream license statement were verified.
- Counsel review: Not performed.
- Legal scope: Upstream licensing and the current non-distributed local-tool
  posture; no legal opinion is asserted.
- As-of date: 2026-07-14.
- Distribution posture: Local construction and investigation utility.
- Subject class: Open-source recursive regular-expression search tool.

## Covered Material

| Component | Function | Repository relationship |
| :-------- | :------- | :---------------------- |
| `rg` executable | Recursive line-oriented search | Operator investigation |
| Regex engine | Pattern matching | Internal ripgrep component |
| Ignore processing | Gitignore-aware filtering | Search-scope behavior |
| Shell integration | Invocation and output handling | Local environment only |

## Repository Use And Scope

The current operator environment exposes `rg` version 15.1.0. ripgrep is used
for bounded repository searches, source discovery, and maintenance diagnostics.
It is not currently invoked by `validate.sh`, and its presence is not required
for canonical repository acceptance.

Upstream describes ripgrep as a line-oriented recursive search tool that
respects Git ignore rules by default and skips hidden and binary content unless
options change that behavior. Search results are evidence of matched text only;
they do not establish completeness when ignore rules, binary detection, file
filters, encoding, permissions, or command options exclude content.

## Provenance And Version History

The observed executable reports ripgrep 15.1.0, revision `af60c2de9d`, PCRE2
10.45 with JIT support, and runtime SIMD support through AVX2. GitHub identifies
15.1.0, published 22 October 2025, as the latest non-draft, non-prerelease
release and marks the release and source commit as signed and verified.

The installed version therefore matches the reviewed upstream release as of
14 July 2026. These values are dated environment and currentness evidence, not a
permanent latest-version claim or repository requirement. The official upstream
repository separately establishes the project identity, documented search
behavior, and dual-license statement.

## Authorship, Ownership, And Attribution

ripgrep and its upstream documentation are authored by their upstream authors
and contributors. SHAR claims only its independently authored documentation,
configuration, and repository material. Running ripgrep against repository files
does not transfer authorship, ownership, or licensing between ripgrep and the
searched material.

## License Or Terms Basis

The official upstream repository states that ripgrep is dual-licensed under the
MIT License or the Unlicense. The exact acquisition channel, binary package,
bundled notices, and any downstream packaging modifications must be reviewed
before redistribution.

Invoking a locally installed executable does not apply ripgrep's license to the
searched repository. Bundling the executable or derived package contents may
create notice or source-package obligations that depend on the selected license
and the exact distributed artifact.

## Distribution, Modification, And Compatibility

- The validation environment currently supplies ripgrep 15.1.0.
- SHAR does not bundle or redistribute that executable.
- No redistribution license has been selected because no ripgrep artifact is
  part of the repository or its distributed payload.
- A binary-package notice inventory is therefore outside the current SHAR
  distribution boundary and becomes required only if that boundary changes.
- ripgrep remains optional construction tooling, not a runtime or acceptance
  dependency.

## Compliance Posture

- Treat ripgrep as an investigation accelerator, not canonical validation.
- Record search options when completeness matters.
- Do not assume ignored, hidden, binary, unreadable, or excluded files were
  searched.
- Preserve upstream notices if the executable is redistributed.
- Reverify the exact package and license choice before bundling.

## Source References

- Gallant, A. and contributors (2025) *ripgrep 15.1.0*. Identified as the
  latest release, published 22 October 2025, with signed and verified release
  and source-commit evidence. Available at:
  <https://github.com/BurntSushi/ripgrep/releases/tag/15.1.0> (Accessed: 14 July
  2026).
- Gallant, A. and contributors (n.d.) *ripgrep official GitHub repository*.
  Available at: <https://github.com/BurntSushi/ripgrep> (Accessed: 14 July
  2026).
- SHAR operator environment (2026), `rg --version` output identifying ripgrep
  15.1.0 revision `af60c2de9d`, PCRE2 10.45 with JIT, and runtime AVX2 support.

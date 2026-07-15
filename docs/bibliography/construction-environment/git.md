# Git

This non-governing record documents the version-control system used by the
repository and does not apply Git's license to tracked SHAR content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository role, observed Git for
  Windows 2.54.0.windows.1 identity, official Git 2.55.0 current release, Git
  for
  Windows 2.55.0.windows.2 release, source mirrors, and GPLv2 license text were
  verified. Configuration, hooks, filters, and signing state remain local.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Distributed version-control system.

## Covered Material

Git command-line tooling, repository objects, index and history behavior, and
the source distribution relevant to SHAR development and publication.

## Repository Use And Scope

SHAR is maintained as a Git repository with a main-only guarded workflow. Git
records repository history and metadata but does not grant rights in tracked
content. The repository license, contributor authority, and third-party rights
remain independent.

## Provenance And Version History

The observed operator environment reports Git for Windows
2.54.0.windows.1. Git's official installation page identifies Git 2.55.0 as the
current upstream release, while the Git for Windows project identifies
2.55.0.windows.2 as its latest Windows distribution. The observed executable is
therefore older than both reviewed current identities as of 14 July 2026.

The exact Git executable, distribution, configuration, hooks, filters, and
signing settings used for a publication must be recorded where they affect
reproducible history or evidence. This dated comparison does not make a
bibliography record the runtime or upgrade authority.

## Authorship, Ownership, And Attribution

Git was originally written by Linus Torvalds and is maintained by project
contributors. Individual platform distributions and bundled components may have
additional notices.

## License Or Terms Basis

Git's official source tree distributes the core project under GNU General Public
License version 2. The source tree also identifies exceptions and separately
licensed components where applicable. The exact source revision, `COPYING` file,
and component notices control redistribution; the repository name or installed
command alone is not sufficient license evidence.

## Distribution, Modification, And Compatibility

Using Git does not relicense repository files. Redistributing Git binaries or
source requires the applicable GPLv2 and component obligations. Hosting Git data
on GitHub is governed separately by the GitHub-user agreement.

## Compliance Posture

Preserve executable, version, configuration, and signing provenance for
published artifacts. Upgrade the observed Git for Windows installation or record
a narrow compatibility reason before representing it as current. Keep Git
licensing, GitHub hosting terms, and SHAR content rights separate.

## Source References

- Git Project (2026) *Install*. Identifies Git 2.55.0 as the latest version.
  Available at: <https://git-scm.com/install/> (Accessed: 14 July 2026).
- Git for Windows contributors (2026) *Git for Windows
  v2.55.0.windows.2*. Identified as the latest signed release, with published
  SHA-256 values for distribution artifacts. Available at:
  <https://github.com/git-for-windows/git/releases/tag/v2.55.0.windows.2>
  (Accessed: 14 July 2026).
- Git Project (n.d.) *Official GitHub source mirror*. Available at:
  <https://github.com/git/git> (Accessed: 14 July 2026).
- Git Project (n.d.) *COPYING*. Available at:
  <https://github.com/git/git/blob/master/COPYING> (Accessed: 14 July 2026).
- SHAR repository and operator environment (2026), Git history, repository
  workflow documentation, and observed `git version 2.54.0.windows.1`.

# Cygwin, GNU Bash, And cygpath

This non-governing record documents the Windows POSIX-compatibility execution
layer used by canonical validation and does not apply Cygwin or Bash licensing
to SHAR scripts, source files, or generated artifacts.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository requirements and authoritative upstream
  sources verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-12.
- Distribution posture: External development and validation prerequisite on
  Windows.
- Subject class: POSIX-compatibility environment, shell, and path utility.

## Covered Material

The Cygwin environment required to execute the repository's Bash validator on
Windows, including the Cygwin runtime, GNU Bash, and `cygpath` path-conversion
utility. Other Cygwin packages are outside this record unless actually used.

## Repository Use And Scope

`validate.sh` requires Bash semantics. Its Unreal C# validation also requires
`cygpath` to convert repository paths into Windows paths consumed by the .NET
compiler and Unreal Build Tool assemblies. The validator may use GNU Coreutils
`shuf` when available and otherwise uses a Python fallback. The current operator
environment reports a Cygwin-hosted Bash and Cygwin `cygpath`.

Cygwin is a development environment, not a runtime dependency of SHAR's
published game or Rust tools. The repository does not link SHAR binaries against
the Cygwin API merely by invoking Bash or `cygpath` as separate processes.

## Provenance And Version History

SHAR records the compatible Cygwin packages and Bash release actually available
to the operator environment. An observed installation may lag because of package
availability, compatibility, delayed review, or human oversight. Validation
evidence must preserve the actual Bash, Cygwin, and `cygpath` identities used
for the run rather than relying on this bibliography record.

## Authorship, Ownership, And Attribution

Cygwin, Newlib, GNU Bash, and the individual packaged utilities have distinct
contributors and may have distinct licenses. Red Hat and Cygwin contributors
retain applicable rights in the Cygwin runtime and project materials. The Free
Software Foundation and Bash contributors retain applicable rights in Bash.

## License Or Terms Basis

Cygwin's official licensing page states that most packaged tools use the GNU
GPL, while the Cygwin API library is licensed under the GNU LGPL version 3 or
later with the stated Cygwin linking exception. GNU's official Bash page states
that Bash is licensed under GNU GPL version 3 or later. Individual Cygwin
packages and utilities require package-specific license verification.

## Distribution, Modification, And Compatibility

Invoking unmodified tools as separate processes does not relicense SHAR source.
SHAR does not distribute the operator's Cygwin installation. Bundling Cygwin,
Bash, or related packages requires complete package-level source, license,
notice, and corresponding-source analysis for the actual binaries conveyed.

## Compliance Posture

- Treat Bash and `cygpath` as explicit Windows validation prerequisites.
- Record the actual tool identities in validation evidence.
- Do not call the environment Git Bash when the observed runtime is Cygwin.
- Do not infer one license for every package in a Cygwin installation.
- Keep Cygwin distribution obligations separate from SHAR source licensing.
- Recheck package licenses before any redistribution.

## Source References

- Cygwin project (n.d.) *Licensing Terms*. Available at:
  <https://cygwin.com/licensing.html> (Accessed: 12 July 2026).
- Cygwin project (n.d.) *Cygwin in Git*. Available at:
  <https://cygwin.com/git.html> (Accessed: 12 July 2026).
- GNU Project (n.d.) *GNU Bash*. Available at:
  <https://www.gnu.org/software/bash/> (Accessed: 12 July 2026).
- SHAR repository (2026) `validate.sh`, Bash execution contract and `cygpath`
  requirement.

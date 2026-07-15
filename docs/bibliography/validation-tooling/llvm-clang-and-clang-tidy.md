# LLVM Clang, Clang-Tidy, And Clang-Format

This non-governing record documents a compiler, formatter, and static-analysis
tool family without applying LLVM licensing to independently authored SHAR
source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The managed Clang-Tidy 22.1.8 binary,
  command-owned invocation and configuration, official LLVM 22.1.8 release,
  source repository, and license were verified. No managed Clang or Clang-Format
  executable or canonical Clang-Format gate was present; target, standard
  library, SDK, and compile-command details remain run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source C and C++ compiler, formatter, and static-analysis
  tooling.

## Covered Material

The LLVM Clang family relevant to SHAR validation. The current managed and
invoked component is Clang-Tidy. Clang and Clang-Format remain related upstream
subjects but are not current managed validation executables.

## Repository Use And Scope

Clang-Tidy enforces semantic and static-analysis policy for authored C and C++
translation units through the command-owned LLVM 22.1.8 binary and
configuration.
The current validation planner does not invoke Clang-Format or a standalone
Clang compiler, so this record does not claim formatter or compiler acceptance.
Formatting remains a repository contract that requires separate enforcement.
Unreal Engine code, vendor compiler components, Windows SDK material, and
third-party Clang plug-ins remain outside this record.

## Provenance And Version History

The managed Clang-Tidy executable reports LLVM 22.1.8. The official LLVM release
page identifies 22.1.8 as the latest release, published 16 June 2026, and
provides signed or attested packages for common platforms. The command-owned
runtime record also selects 22.1.8 for Clang-Tidy.

The exact binary archive, signatures or attestations, target, compile arguments,
SDK, system headers, and included components remain artifact-specific evidence.
No managed Clang-Format or Clang executable was present in the reviewed LLVM
payload.

## Authorship, Ownership, And Attribution

The LLVM Project and contributors retain applicable upstream rights. Third-party
components included in a particular LLVM distribution may carry separate
copyrights, licenses, and notices.

## License Or Terms Basis

LLVM Project material is generally licensed under Apache License 2.0 with LLVM
Exceptions, subject to the exact upstream license and notice files. That posture
must not be generalized to third-party components or vendor distributions
without verification.

## Distribution, Modification, And Compatibility

Running Clang, Clang-Tidy, or Clang-Format does not relicense checked source.
Redistributing LLVM binaries, libraries, headers, tools, or plug-ins requires
preservation of applicable licenses, notices, and third-party component records.

## Compliance Posture

Canonical validation and repository configuration confirm Clang-Tidy use, not a
complete Clang or Clang-Format toolchain. Record exact version, provenance,
compile arguments, component inventory, and notices before distributing any LLVM
bundle. Do not claim Clang-Format enforcement until a managed executable and
canonical gate independently prove it.

## Source References

- LLVM Project (2026) *LLVM 22.1.8 release*. Identified as the latest release,
  published 16 June 2026, with signed or attested platform packages. Available
  at: <https://github.com/llvm/llvm-project/releases/tag/llvmorg-22.1.8>
  (Accessed: 14 July 2026).
- LLVM Project (n.d.) *Clang-Tidy documentation*. Available at:
  <https://clang.llvm.org/extra/clang-tidy/> (Accessed: 14 July 2026).
- LLVM Project (n.d.) *Clang-Format documentation*. Available at:
  <https://clang.llvm.org/docs/ClangFormat.html> (Accessed: 14 July 2026).
- LLVM Project (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/llvm/llvm-project> (Accessed: 14 July 2026).
- LLVM Project (n.d.) *LLVM Project License*. Available at:
  <https://raw.githubusercontent.com/llvm/llvm-project/main/LICENSE.TXT>
  (Accessed: 14 July 2026).
- SHAR repository and command authority (2026), managed Clang-Tidy 22.1.8
  executable, `.clang-tidy`, runtime mapping, and canonical validation planner.

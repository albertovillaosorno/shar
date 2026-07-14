# LLVM Clang, Clang-Tidy, And Clang-Format

This non-governing record documents a compiler, formatter, and static-analysis
tool family without applying LLVM licensing to independently authored SHAR
source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository invocation and configuration,
  official LLVM documentation, source repository, and license verified; exact
  Clang, Clang-Tidy, Clang-Format, target, standard-library, and SDK versions
  remain run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Open-source C and C++ compiler, formatter, and static-analysis
  tooling.

## Covered Material

The Clang family used by SHAR validation, including Clang diagnostics,
Clang-Tidy, and Clang-Format.

## Repository Use And Scope

Clang-Tidy enforces semantic and static-analysis policy, while Clang-Format
checks canonical formatting for authored C and C++ source. These are development
and validation tools unless a build expressly packages LLVM components. Unreal
Engine code, vendor compiler components, Windows SDK material, and third-party
Clang plug-ins remain outside this record.

## Provenance And Version History

Repository `.clang-tidy`, formatting configuration, architecture records, and
validation scripts verify the intended tool family. The exact LLVM version,
binary distributor, target, build options, and included components are
artifact-specific evidence.

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

Canonical validation and repository configuration confirm use of the LLVM tool
family. Record exact version, provenance, component inventory, and notices
before distributing any LLVM toolchain bundle.

## Source References

- LLVM Project (n.d.) *Clang-Tidy documentation*. Available at:
  <https://clang.llvm.org/extra/clang-tidy/> (Accessed: 12 July 2026).
- LLVM Project (n.d.) *Clang-Format documentation*. Available at:
  <https://clang.llvm.org/docs/ClangFormat.html> (Accessed: 12 July 2026).
- LLVM Project (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/llvm/llvm-project> (Accessed: 12 July 2026).
- LLVM Project (n.d.) *LLVM Project License*. Available at:
  <https://raw.githubusercontent.com/llvm/llvm-project/main/LICENSE.TXT>
  (Accessed: 12 July 2026).
- SHAR repository (2026) `.clang-tidy`, formatting configuration, and
  `validate.sh`.

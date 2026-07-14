# Microsoft C++ Build Toolchain

This non-governing record documents a potential Windows build prerequisite and
does not claim that the Visual Studio IDE was used to author SHAR.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository requirement, installed
  Visual Studio 18 Community toolchain identity, MSBuild 18.7.8.30822, MSVC
  14.51.36231, compiler 19.51.36248 for x64, Windows SDK 10.0.26100.0, Epic's
  Unreal setup guidance, and Microsoft's toolchain composition were verified.
  Accepted installation terms and Unreal build acceptance remain environment-
  and build-specific.
- Operator-use status: Installation is observed; Visual Studio IDE authorship use
  is not attested.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Proprietary compiler, linker, SDK, and build tooling.

## Covered Material

The Microsoft standalone Build Tools components required by the selected Unreal
Engine installation on Windows. Microsoft's current product page identifies the
MSVC compiler and linker, standard library, ATL and MFC, Windows SDK, Clang
tools for Windows, AddressSanitizer, and vcpkg as available Build Tools
components.
Only the installed subset used by SHAR belongs in a reproducible build record.

## Repository Use And Scope

SHAR source may be edited in Cursor or another editor. The selected Unreal
Engine version requires a compatible C++ compiler and Windows SDK before a
Windows C++ target can be compiled or packaged. The reviewed environment contains
an x64 MSVC compiler, MSBuild, multiple MSVC toolsets, and Windows SDK
10.0.26100.0. Their presence establishes tool availability, not successful Unreal
compilation or packaging.

This record does not establish use of the Visual Studio IDE for authorship. The
installed Community edition may supply command-line build components without
being the operator's editor.

## Provenance And Version History

The reviewed installation exposes MSBuild 18.7.8.30822, MSVC toolsets
14.44.35207 and 14.51.36231, compiler 19.51.36248 for x64, and Windows SDK
10.0.26100.0. These are dated observations from the local environment, not
minimum-version requirements and not a permanent supported range.

Build and packaging evidence must identify the exact compiler, selected toolset,
Windows SDK, MSBuild components, redistributables, target architecture, Unreal
Build Tool version, and command line actually used. A later compatible toolchain
may replace these observations without requiring policy prose to encode a
floating `>=` range.

## Authorship, Ownership, And Attribution

Microsoft and its licensors retain applicable rights in the compiler, SDK,
runtimes, installer, documentation, and branding. SHAR claims no rights in those
components.

## License Or Terms Basis

The standalone product page identifies Build Tools as command-line and
continuous-integration components outside the Visual Studio IDE. It does not, by
itself, establish the license accepted for one installation. The toolchain is
governed by the Microsoft terms presented for the installed edition, channel,
workloads, and components. Redistributable packages, Windows SDK material,
vcpkg, and separately licensed open-source components may carry distinct
conditions. The installed terms and component notices control.

## Distribution, Modification, And Compatibility

No Microsoft compiler, SDK, or IDE payload is intended for repository
publication. A packaged Windows product must inventory every Microsoft runtime
or redistributable actually shipped and satisfy its distribution terms.

## Compliance Posture

Treat the Microsoft C++ build toolchain as a required external prerequisite only
when the selected Unreal Engine version requires it. Keep the selected toolchain
current under the governing compatibility policy, but preserve exact versions as
build evidence rather than expressing an unbounded `>=` requirement. Do not list
the Visual Studio IDE as an operator-used editor without direct evidence, and do
not infer build acceptance from installation alone.

## Source References

- Epic Games (2026) *Setting Up Visual Studio Development Environment for C++
  Projects in Unreal Engine 5.8*. Available at: [Epic C++ setup][epic-cpp]
  (Accessed: 13 July 2026).
- Microsoft (n.d.) *Visual Studio Build Tools for C++*. Available at:
  <https://visualstudio.microsoft.com/visual-cpp-build-tools/> (Accessed: 14
  July 2026).
- SHAR repository and operator environment (2026), `README.md` Phase 9 external
  prerequisites, installed Visual Studio 18 Community layout, MSBuild
  18.7.8.30822, MSVC toolsets 14.44.35207 and 14.51.36231, compiler
  19.51.36248 for x64, and Windows SDK 10.0.26100.0.

[epic-cpp]:
  https://dev.epicgames.com/documentation/en-us/unreal-engine/setting-up-visual-studio-development-environment-for-cplusplus-projects-in-unreal-engine

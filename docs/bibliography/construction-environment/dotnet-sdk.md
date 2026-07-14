# .NET SDK And Runtime

This non-governing record documents build and validation tooling without
applying .NET licensing to independently authored SHAR source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository role, official source
  repositories, runtime license, and current support policy verified; the exact
  local SDK and runtime resolution remains run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Open-source SDK, compiler host, runtime, libraries, and build
  tooling with separately licensed components.

## Covered Material

The `dotnet` SDK and runtime used by canonical Unreal C# validation and by
Unreal Build Tool-related build-rule compilation.

## Repository Use And Scope

`validate.sh` requires `dotnet` when authored Unreal C# files are in scope and
invokes the installed Roslyn compiler. The repository does not treat the .NET
SDK or runtime as SHAR-authored material and does not publish an installed SDK.

## Provenance And Version History

The validator resolves the installed SDK version at execution time. Microsoft's
current support policy treats the SDK, runtime, ASP.NET Core, and Entity
Framework Core as related .NET technologies and requires supported installations
to remain current on patch updates. A publication, distribution, or validation
record must preserve the exact SDK, runtime, reference packs, Roslyn compiler,
platform architecture, and patch level actually used.

## Authorship, Ownership, And Attribution

Microsoft, the .NET Foundation, contributors, and third-party component authors
retain applicable upstream rights. SHAR contributors retain rights in
independently authored repository source.

## License Or Terms Basis

The official .NET runtime and SDK repositories use the MIT License and include
third-party notice files. The runtime license requires preservation of its
copyright and permission notice in copies or substantial portions. Individual
workloads, reference packs, proprietary Microsoft components, installers, and
redistributed runtimes may have additional or different terms. Exact installed
component evidence controls.

## Distribution, Modification, And Compatibility

Using the SDK for validation does not relicense checked source. A self-contained
or framework-dependent distribution must inventory the runtime, libraries,
native components, notices, and redistribution conditions actually shipped.

## Compliance Posture

Record exact SDK and runtime versions for reproducible validation. Preserve MIT
license material and all applicable third-party notices for any distributed .NET
components.

## Source References

- .NET Foundation and contributors (n.d.) *.NET SDK official GitHub repository*.
  Available at: <https://github.com/dotnet/sdk> (Accessed: 13 July 2026).
- .NET Foundation and contributors (n.d.) *.NET Runtime official GitHub
  repository*. Available at: <https://github.com/dotnet/runtime> (Accessed: 13
  July 2026).
- .NET Foundation and contributors (n.d.) *.NET Runtime License*. Available at:
  <https://github.com/dotnet/runtime/blob/main/LICENSE.TXT> (Accessed: 13 July
  2026).
- Microsoft (2026) *.NET Support Policy*, updated 9 June. Available at:
  <https://dotnet.microsoft.com/en-us/platform/support/policy> (Accessed: 13
  July 2026).
- Microsoft (n.d.) *.NET documentation*. Available at:
  <https://learn.microsoft.com/dotnet/> (Accessed: 13 July 2026).
- SHAR repository (2026) `validate.sh` and authored Unreal C# build rules.

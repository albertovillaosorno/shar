# .NET SDK And Runtime

This non-governing record documents build and validation tooling without
applying .NET licensing to independently authored SHAR source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Authored Unreal C# build-rule use,
  official source repositories, runtime license, current support policy, and the
  current validation omission were verified. No canonical Roslyn or `dotnet`
  compiler gate is scheduled for the authored `.Target.cs` and `.Build.cs` files;
  the exact SDK, runtime, compiler, and Unreal Build Tool environment remain
  build-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source SDK, compiler host, runtime, libraries, and build
  tooling with separately licensed components.

## Covered Material

The `dotnet` SDK, runtime, and Roslyn compiler that may participate when Unreal
Build Tool compiles authored C# target and module rules.

## Repository Use And Scope

SHAR contains authored Unreal `.Target.cs` and `.Build.cs` files, but the current
canonical validation plans for those files do not invoke `dotnet`, Roslyn, or an
Unreal Build Tool compilation. Their compiler acceptance therefore remains an
engine-build verification requirement rather than current canonical validator
evidence.

The repository does not treat the .NET SDK or runtime as SHAR-authored material
and does not publish an installed SDK.

## Provenance And Version History

An Unreal build or future C# compiler gate must resolve and record the exact SDK
and compiler identity at execution time. Microsoft's current support policy
treats the SDK, runtime, ASP.NET Core, and Entity Framework Core as related .NET
technologies and requires supported installations to remain current on patch
updates. A publication, distribution, or engine-build record must preserve the
exact SDK, runtime, reference packs, Roslyn compiler, platform architecture, and
patch level actually used.

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

Do not claim C# compiler validation from the current canonical plan. Record exact
SDK, runtime, Roslyn, Unreal Build Tool, and engine identities when an engine
build or future compiler gate supplies that evidence. Preserve MIT license
material and all applicable third-party notices for any distributed .NET
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
- SHAR repository (2026) authored Unreal C# build rules and canonical validation
  plans reviewed 14 July 2026; no current Roslyn or `dotnet` compiler gate was
  scheduled for the `.Target.cs` or `.Build.cs` files.

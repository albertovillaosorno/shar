# .NET SDK And Runtime

This non-governing record documents build and validation tooling without
applying .NET licensing to independently authored SHAR source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Authored Unreal C# build-rule use, the
  managed .NET SDK 11.0.100-preview.5 identity, included runtimes, official
  preview status, source repositories, runtime license, support policy, and the
  current validation omission were verified. No canonical Roslyn or `dotnet`
  compiler gate is scheduled for the authored `.Target.cs` and `.Build.cs` files;
  Unreal Build Tool acceptance remains build-specific.
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

The managed runtime reports SDK 11.0.100-preview.5.26302.115, MSBuild
18.8.0-preview, and matching .NET, ASP.NET Core, and Windows Desktop runtimes on
Windows x64. Microsoft's official .NET 11 page identifies Preview 5, released
9 June 2026, as the current .NET 11 preview and states that preview releases are
generally not supported for production use. The managed SDK therefore matches
the reviewed preview release but is not a production-support claim.

An Unreal build or future C# compiler gate must still record the exact SDK,
Roslyn compiler, Unreal Build Tool, reference packs, workload state, target,
architecture, and patch identity used. The current managed environment reports no
installed workloads.

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

Do not claim C# compiler validation from the current canonical plan or production
support from the preview SDK. Keep the temporary .NET 11 preview allowance
bounded to the period before stable .NET 11 exists. Record exact SDK, runtime,
Roslyn, Unreal Build Tool, workload, and engine identities when an engine build
or future compiler gate supplies that evidence. Preserve MIT license material
and all applicable third-party notices for distributed .NET components.

## Source References

- Microsoft (2026) *.NET 11 Preview*. Identifies Preview 5 as the current .NET
  11 preview, released 9 June 2026, with SDK full version
  11.0.100-preview.5.26302.115 and matching runtimes. Available at:
  <https://dotnet.microsoft.com/en-us/download/dotnet/11.0> (Accessed: 14 July
  2026).
- .NET Foundation and contributors (n.d.) *.NET SDK official GitHub repository*.
  Available at: <https://github.com/dotnet/sdk> (Accessed: 14 July 2026).
- .NET Foundation and contributors (n.d.) *.NET Runtime official GitHub
  repository*. Available at: <https://github.com/dotnet/runtime> (Accessed: 14
  July 2026).
- .NET Foundation and contributors (n.d.) *.NET Runtime License*. Available at:
  <https://github.com/dotnet/runtime/blob/main/LICENSE.TXT> (Accessed: 14 July
  2026).
- Microsoft (2026) *.NET Support Policy*, updated 9 June. Available at:
  <https://dotnet.microsoft.com/en-us/platform/support/policy> (Accessed: 14
  July 2026).
- Microsoft (n.d.) *.NET documentation*. Available at:
  <https://learn.microsoft.com/dotnet/> (Accessed: 14 July 2026).
- SHAR repository and managed runtime (2026), authored Unreal C# build rules,
  SDK 11.0.100-preview.5.26302.115, matching runtimes, no installed workloads,
  and canonical validation plans reviewed 14 July 2026; no Roslyn or `dotnet`
  compiler gate was scheduled for `.Target.cs` or `.Build.cs` files.

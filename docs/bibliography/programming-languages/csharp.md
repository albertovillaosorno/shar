# C\#

This non-governing record distinguishes the C# language and standard from
compiler implementations, .NET libraries, Unreal Build Tool, and independently
authored SHAR build-rule source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository C# use, official language-
  specification work, Roslyn source, Microsoft documentation, and the current
  compiler-validation omission were verified. The exact compiler, language
  version, reference assemblies, SDK, and Unreal Build Tool environment remain
  build-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Standardized programming language.

## Covered Material

The C# language used by Unreal target and module rule files, together with the
public language-standard and compiler references needed to interpret that code.

## Repository Use And Scope

SHAR contains authored `.Target.cs` and `.Build.cs` files consumed by Unreal
Build Tool. The current canonical validation plans for those files do not
compile
or analyze them with Roslyn, `dotnet`, or Unreal Build Tool, so syntax and
compiler acceptance remain unproven until an engine build or future compiler
gate
supplies that evidence.

C# is a language, not a license grant for Roslyn, .NET, Unreal Engine, Microsoft
documentation, or the resulting SHAR source.

## Provenance And Version History

The effective language version is determined by the compiler and Unreal Build
Tool environment used for a build. Build and distribution evidence must
identify the exact .NET SDK, Roslyn compiler, Unreal Engine version, and target
configuration.

## Authorship, Ownership, And Attribution

The standardization bodies, Microsoft, compiler contributors, and documentation
contributors retain applicable rights in their respective materials. SHAR
contributors retain rights in independently authored build rules subject to the
repository license.

## License Or Terms Basis

The public C# specification work and Roslyn compiler are maintained in separate
repositories with their own license and notice files. Published standards text,
compiler binaries, SDK components, and documentation must be reviewed
individually; use of the language does not reproduce or distribute them.

## Distribution, Modification, And Compatibility

Publishing C# source does not necessarily distribute a compiler or runtime.
Packaging must inventory any .NET runtime, compiler component, reference
assembly, or Microsoft redistributable actually delivered.

## Compliance Posture

Do not describe the authored build rules as compiler-validated while the current
canonical plan omits a C# gate. Preserve exact compiler, SDK, Unreal Build Tool,
and engine provenance when an engine build or future compiler gate verifies
them.
Keep standards references, implementation licenses, Unreal Engine terms, and
SHAR-authored source rights separate.

## Source References

- .NET Foundation and contributors (n.d.) *C# language specification working
  repository*. Available at: <https://github.com/dotnet/csharpstandard>
  (Accessed: 12 July 2026).
- .NET Foundation and contributors (n.d.) *Roslyn official GitHub repository*.
  Available at: <https://github.com/dotnet/roslyn> (Accessed: 12 July 2026).
- Microsoft (n.d.) *C# documentation*. Available at:
  <https://learn.microsoft.com/dotnet/csharp/> (Accessed: 12 July 2026).
- SHAR repository (2026) authored Unreal `.Target.cs` and `.Build.cs` files and
  canonical validation plans reviewed 14 July 2026; no current C# compiler gate
  was scheduled for those files.

# GNU Bash And Windows PowerShell

This non-governing record documents the two shell environments used by the
public Windows validation entry point. It does not apply Bash, Windows, or
PowerShell licensing to SHAR scripts, source files, or generated artifacts.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The public wrapper, Bash requirement,
  `powershell.exe` bridge, GNU Bash identity and license, and the distinction
  between Windows PowerShell and PowerShell 7 were verified. The exact Bash
  distribution, Windows PowerShell version, Windows component terms, and host
  build remain execution-specific.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: External development and validation prerequisites on
  Windows; neither shell is bundled by SHAR.
- Subject class: Command shells and validation process bridge.

## Covered Material

GNU Bash as the interpreter for the repository's shell wrapper and Windows
PowerShell as the `powershell.exe` process used to invoke the canonical command
runner on Windows.

The record does not cover Cygwin, `cygpath`, PowerShell 7, or another POSIX
compatibility environment unless a particular validation run separately records
that component.

## Repository Use And Scope

The public wrapper requires Bash semantics for strict shell execution, argument
escaping, and workspace resolution. It then invokes `powershell.exe` with the
repository command runner and forwards the requested validation arguments.

The wrapper contains no Cygwin or `cygpath` invocation. It also does not compile
or analyze C# itself. A shell environment is compatible only when it can execute
the wrapper and launch the required Windows PowerShell process with equivalent
arguments and exit behavior.

Neither shell is a runtime dependency of the published game or Rust tools.
Invoking a separately installed process does not make that process part of the
SHAR distribution.

## Provenance And Version History

The exact Bash implementation and version depend on the shell environment used
for a validation run. The executable name `powershell.exe` identifies the
Windows PowerShell command surface, not the cross-platform PowerShell 7 command
`pwsh`.

PowerShell's official repository distinguishes Windows PowerShell 5.1 from
PowerShell 7 and later. The repository's MIT license applies to the open-source
PowerShell project and must not be assumed to license Windows PowerShell as an
installed Windows component. Reproducible validation evidence must preserve the
actual executable identities and versions used.

## Authorship, Ownership, And Attribution

The Free Software Foundation and Bash contributors retain applicable rights in
GNU Bash. Microsoft and the relevant Windows and PowerShell contributors retain
applicable rights in their respective software and documentation. SHAR
contributors retain rights in the independently authored validation wrapper.

## License Or Terms Basis

GNU's official Bash page states that Bash is licensed under GNU GPL version 3 or
later. The open-source PowerShell repository uses the MIT License and includes
third-party notices, but those files do not establish the license for Windows
PowerShell or every Windows component used by `powershell.exe`.

Exact installed component evidence controls any redistribution analysis.

## Distribution, Modification, And Compatibility

SHAR does not distribute Bash, Windows PowerShell, PowerShell 7, Windows, or a
POSIX compatibility environment. Bundling any shell, runtime, Windows component,
or compatibility layer requires separate license, notice, source, and
redistribution review for the exact component conveyed.

A successful validation run establishes observed process compatibility only. It
does not establish that every Bash distribution, PowerShell edition, or Windows
version is supported.

## Compliance Posture

- Treat Bash and `powershell.exe` as the public Windows wrapper requirements.
- Do not claim that Cygwin or `cygpath` is required without an actual repository
  dependency.
- Do not substitute PowerShell 7 licensing for Windows PowerShell licensing.
- Record exact shell and Windows identities for reproducible validation evidence.
- Keep external shell distribution obligations separate from SHAR source
  licensing.
- Recheck exact component terms before redistribution.

## Source References

- GNU Project (n.d.) *GNU Bash*. Available at:
  <https://www.gnu.org/software/bash/> (Accessed: 14 July 2026).
- PowerShell contributors (n.d.) *PowerShell official repository*. The project
  distinguishes Windows PowerShell 5.1 from PowerShell 7 and later and publishes
  PowerShell 7 under the MIT License. Available at:
  <https://github.com/PowerShell/PowerShell> (Accessed: 14 July 2026).
- PowerShell contributors (n.d.) *PowerShell License*. Available at:
  <https://github.com/PowerShell/PowerShell/blob/master/LICENSE.txt> (Accessed:
  14 July 2026).
- SHAR repository (2026) public Bash validation wrapper and Windows PowerShell
  command bridge.

# Windows 11

This non-governing record documents construction provenance and does not
license, distribute, endorse, or certify Windows.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Official product, system-requirement,
  lifecycle, and licensing sources and the repository Windows target boundary
  were verified. No exact construction-host edition, feature update, or build is
  established by public evidence.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Proprietary operating system and development environment.

## Covered Material

Windows 11 as a proprietary host operating-system family relevant to building,
reviewing, and validating Windows-targeted SHAR artifacts. This record does not
identify one exact construction host, local edition, feature-update version, or
build.

## Repository Use And Scope

Windows is a supported target and possible host environment, not a repository
dependency or a component distributed under the SHAR MIT License.
Platform-specific builds may require a lawful, user-provided Windows installation
and separately licensed Microsoft tooling.

## Provenance And Version History

Microsoft publishes Windows 11 requirements and lifecycle information at the
product-family level, but support and terms remain edition-, channel-, and
feature-update-specific. The exact local edition, build, acquisition channel,
and accepted license terms are not established by this record. Capture those
facts when a build or distribution analysis depends on a specific SDK, runtime,
redistributable, or system component.

## Authorship, Ownership, And Attribution

Microsoft and its licensors retain rights in Windows, its documentation,
branding, and bundled components. SHAR claims rights only in independently
authored repository material.

## License Or Terms Basis

Windows 11 is proprietary software governed by edition- and channel-specific
Microsoft Software License Terms. The installed terms, not this summary, control
use. No Windows binary, system file, SDK payload, or documentation body is
relicensed by SHAR.

## Distribution, Modification, And Compatibility

No Windows component is intended for repository distribution. A packaged build
must separately inventory Microsoft redistributables, SDK components, runtime
prerequisites, and installer notices actually shipped.

## Compliance Posture

Do not infer an exact construction host from Windows-target support. Before
distributing Windows-targeted binaries, verify every Microsoft component and
preserve all applicable terms, notices, and redistribution conditions.

## Source References

- Microsoft (n.d.) *Windows 11*. Available at:
  <https://www.microsoft.com/windows/windows-11> (Accessed: 13 July 2026).
- Microsoft (n.d.) *Windows 11 Specs And System Requirements*. Available at:
  <https://www.microsoft.com/en-us/windows/windows-11-specifications>
  (Accessed: 13 July 2026).
- Microsoft (n.d.) *Windows 11 Home And Pro — Microsoft Lifecycle*.
  Available at:
  <https://learn.microsoft.com/en-us/lifecycle/products/windows-11-home-and-pro>
  (Accessed: 13 July 2026).
- Microsoft (n.d.) *Microsoft Software License Terms*. Available at:
  <https://www.microsoft.com/useterms> (Accessed: 13 July 2026).
- SHAR repository (2026) Windows target and platform-tooling boundary.

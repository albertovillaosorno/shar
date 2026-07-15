# Windows 11

This non-governing record documents construction provenance and does not
license, distribute, endorse, or certify Windows.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Official product, system-requirement,
  lifecycle, and licensing sources, repository Windows target boundary, and the
  observed construction host were verified. The reviewed host reports Windows
  11 Pro, feature update 25H2, OS version 10.0.26200, build 26200.8655, and a
  Client installation. Acquisition channel and accepted terms remain local.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Proprietary operating system and development environment.

## Covered Material

Windows 11 as a proprietary host operating-system family relevant to building,
reviewing, and validating Windows-targeted SHAR artifacts. The reviewed
construction host is Windows 11 Pro 25H2, OS version 10.0.26200, build
26200.8655. Those values are dated provenance, not minimum requirements or a
permanent compatibility range.

## Repository Use And Scope

Windows is a supported target and possible host environment, not a repository
dependency or a component distributed under the SHAR MIT License.
Platform-specific builds may require a lawful, user-provided Windows
installation
and separately licensed Microsoft tooling.

## Provenance And Version History

The reviewed host reports `Microsoft Windows 11 Pro` through the
operating-system
management interface, `Professional` edition and `25H2` through current-version
registry metadata, and build 26200 with update revision 8655. Some legacy
Windows
metadata still reports `Windows 10 Pro` or version `2009`; those strings are not
used alone to identify the host because the operating-system interfaces and
build evidence identify Windows 11.

Microsoft publishes requirements and lifecycle information at the product-family
level, but support and terms remain edition-, channel-, and feature-update-
specific. Capture the exact edition, build, acquisition channel, and accepted
terms when a build or distribution analysis depends on a specific SDK, runtime,
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

Preserve the exact observed host identity with build evidence, but do not turn
it
into an unbounded minimum-version requirement or infer that every Windows 11
edition and channel is compatible. Before distributing Windows-targeted
binaries,
verify every Microsoft component and preserve all applicable terms, notices, and
redistribution conditions.

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
  <https://www.microsoft.com/useterms> (Accessed: 14 July 2026).
- SHAR repository and operator environment (2026), Windows target and
  platform-tooling boundary, operating-system management output identifying
  Windows 11 Pro, registry evidence for `Professional` edition and `25H2`, and
  build 26200.8655.

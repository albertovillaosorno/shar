# Blender

This non-governing record documents Blender as a possible experimental model
inspection application. It does not make Blender the canonical serializer, a
bundled dependency, a validation authority, or an endorser of SHAR.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — The installed Blender 5.1.2 build,
  Windows release identity, build date, source commit, official Blender 5.2.0
  LTS release artifacts and checksum publication, source mirror, license
  boundary,
  and repository experimental-script boundary were verified. Add-on set,
  settings profile, and importer acceptance remain unverified.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source 3D authoring and experimental inspection
  application.

## Covered Material

The Blender application and official documentation relevant to possible local
FBX model, material, skeleton, and animation inspection.

## Repository Use And Scope

The repository contains an optional experimental Blender inspection-script
generator. The reviewed host has Blender 5.1.2 available, but installation alone
does not prove that the helper was run or that an FBX import is correct. The
script is not a production conversion, validation, repair, or acceptance path.
SHAR's independently authored binary FBX writer and repository-owned tests
remain
authoritative.

Blender executables, bundled libraries, add-ons, and generated inspection
artifacts containing proprietary game material are not distributed by SHAR.

## Provenance And Version History

The installed executable reports Blender 5.1.2 for Windows, built 19 May 2026
from source commit `ec6e62d40fa9` on the `blender-v5.1-release` branch.
Blender's
official download server published Blender 5.2.0 Windows x64 artifacts and a
checksum manifest on 14 July 2026, and the official developer documentation
labels the release Blender 5.2 LTS. The installed application is therefore one
minor release behind the reviewed upstream release.

These values are dated environment and currentness evidence, not a minimum
requirement or permanent compatibility range. Any later Blender version, add-on,
Python API, settings profile, package, bundled component, or importer result
must
be identified separately before its behavior or terms are relied upon.

## Authorship, Ownership, And Attribution

The Blender Foundation and contributors retain rights in Blender. Individual
add-ons and bundled components may have different owners and licenses. Blender
names and marks are used nominatively to identify the review application.

## License Or Terms Basis

The current official Blender source mirror describes Blender as a whole as
licensed under GNU General Public License version 3, while individual files may
use different compatible licenses. The repository `COPYING` file directs readers
to the complete license text and states that Blender is not offered under an
alternative license. The exact Blender revision, individual-file notices, and
bundled third-party inventory control. Using Blender to create or inspect a file
does not determine the copyright or license of that file. Add-ons, scripts,
linked libraries, and copied manual material require separate analysis.

## Distribution, Modification, And Compatibility

SHAR does not redistribute Blender. Independently authored experimental
inspection helpers remain repository material only to the extent they do not
copy Blender source or other protected upstream expression. Local outputs remain
governed by the rights in their underlying content.

## Compliance Posture

Treat Blender 5.1.2 as observed installation evidence only. Upgrade it to the
reviewed 5.2 LTS line or document a narrow compatibility reason before
describing
the installation as current. Do not treat the installation or inspection script
as proof of successful import, compatibility, validation, or acceptance, and do
not convert either observed version into an unbounded `>=` requirement.
Distribution of Blender, an add-on, linked library, or copied documentation
requires a component-level GPL, notice, and source-availability review.

## Source References

- Blender Foundation (2026) *Blender 5.2 LTS Release Notes*. Identifies the
  5.2 line as an LTS release. Available at:
  <https://developer.blender.org/docs/release_notes/5.2/> (Accessed: 14 July
  2026).
- Blender Foundation (2026) *Blender 5.2 release artifacts*. Publishes Blender
  5.2.0 Windows x64 installers and archives dated 14 July 2026. Available at:
  <https://download.blender.org/release/Blender5.2/> (Accessed: 14 July 2026).
- Blender Foundation (2026) *Blender 5.2.0 checksum manifest*. Publishes SHA-256
  identities for the official release artifacts. Available at:
  <https://download.blender.org/release/Blender5.2/blender-5.2.0.sha256>
  (Accessed: 14 July 2026).
- Blender Foundation (n.d.) *License*. Available at:
  <https://www.blender.org/about/license/> (Access attempted: 14 July 2026).
- Blender Foundation (n.d.) *Blender Manual*. Available at:
  <https://docs.blender.org/manual/en/latest/> (Accessed: 14 July 2026).
- Blender Foundation (n.d.) *Official Blender GitHub mirror*. Available at:
  <https://github.com/blender/blender> (Accessed: 14 July 2026).
- Blender Foundation (n.d.) *COPYING*. Available at:
  <https://github.com/blender/blender/blob/main/COPYING> (Accessed: 14 July
  2026).
- SHAR repository and operator environment (2026), canonical FBX boundary,
  binary writer tests, experimental Blender inspection-helper source, and
  Blender 5.1.2 Windows build output identifying build date 19 May 2026 and
  source commit `ec6e62d40fa9`.

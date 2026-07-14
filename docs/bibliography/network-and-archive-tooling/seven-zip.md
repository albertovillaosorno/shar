# 7-Zip

This non-governing record documents a repository-managed extraction tool and
does not apply 7-Zip licensing to the archive contents, FFmpeg, or SHAR source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository acquisition path, cached
  7-Zip 26.02 `7zr.exe` identity, local SHA-256, official 26.02 current release,
  source repository, and upstream license were verified. The resolver uses a
  mutable versionless URL and does not verify a publisher checksum before
  execution.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: Downloaded external bootstrap tool in an ignored cache.
- Subject class: Archive extraction utility.

## Covered Material

The reduced standalone `7zr.exe` extractor downloaded from the official 7-Zip
site and used to unpack the configured `.7z` FFmpeg archive. The full graphical
7-Zip application and unrelated format handlers are outside the repository use
unless separately introduced.

## Repository Use And Scope

When the required media tools are absent, SHAR downloads the official reduced
extractor from the versionless `7zr.exe` alias into
`dependencies/ffmpeg/bootstrap/` and invokes it with bounded arguments to
extract the FFmpeg archive into an ignored staging directory. The reviewed cache
contains 7-Zip 26.02 for x86 with local SHA-256
`56b8cc9f4971cef253644fafe54063ed7fdca551d4dee0f8c6baa81b855acd72`.
The executable is not tracked as repository-authored source.

The current resolver downloads and executes the alias without comparing the
artifact to a publisher-provided digest or pinned release identity. Extraction
success therefore does not authenticate the extractor or archive, establish
licensing, or make extracted files safe. Archive path traversal, unexpected
files, binary provenance, checksums, signatures, and post-extraction validation
remain independent security requirements.

## Provenance And Version History

The cached extractor identifies itself as 7-Zip 26.02, dated 25 June 2026. The
official 7-Zip site and release repository identify 26.02 as the current release.
The versionless download alias returned a 602,112-byte artifact last modified on
25 June 2026 during this review, which is consistent with the cached identity but
is not a cryptographic verification.

For an acquisition that uses 7-Zip, the exact standalone extractor actually
retrieved remains the controlling evidence. The URL, response metadata, local
checksum, file size, upstream release identity, retrieval date, and any
publisher signature or digest must be captured for the acquisition. A mutable
alias may later resolve to different bytes without a repository change.

## Authorship, Ownership, And Attribution

Igor Pavlov and identified third-party contributors retain applicable rights.
Specific files and codec implementations inside the broader 7-Zip distribution
may carry distinct license terms and restrictions.

## License Or Terms Basis

The official 7-Zip license states that most code is under the GNU Lesser General
Public License, while identified portions use BSD licenses and some RAR-related
code carries the unRAR restriction. The official license states that all files
other than `7z.dll` use the GNU LGPL. The exact downloaded standalone artifact
and accompanying license material control.

## Distribution, Modification, And Compatibility

Local execution does not relicense archives or extracted files. SHAR does not
redistribute `7zr.exe`. Redistribution requires preservation of the applicable
LGPL terms, license information, notices, and component-specific conditions for
the exact artifact conveyed.

## Compliance Posture

- Replace the mutable alias with a pinned release artifact or record the resolved
  release identity before execution.
- Download only from the configured official upstream location.
- Add publisher-backed checksum or signature verification before execution; the
  current resolver does not perform it.
- Extract only into a fresh, bounded staging directory.
- Validate resulting paths and expected executable identities.
- Keep the bootstrap executable and archive outside Git.
- Recheck the exact artifact license and notices before redistribution.

## Source References

- 7-Zip (2026) *Official website*. Identifies 7-Zip 26.02, dated 25 June
  2026, as the current release and summarizes component licensing. Available at:
  <https://www.7-zip.org/> (Accessed: 14 July 2026).
- 7-Zip (2026) *7-Zip 26.02*. Identified as the latest source release.
  Available at: <https://github.com/ip7z/7zip/releases/tag/26.02> (Accessed: 14
  July 2026).
- 7-Zip (n.d.) *License for use and distribution*. Available at:
  <https://www.7-zip.org/license.txt> (Accessed: 14 July 2026).
- SHAR repository and ignored dependency cache (2026),
  `src/pipeline/src/adapters/driven/local/one/media_dependencies.rs`, cached
  7-Zip 26.02 identity, response headers, and local SHA-256 evidence.

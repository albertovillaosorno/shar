# 7-Zip

This non-governing record documents a repository-managed extraction tool and
does not apply 7-Zip licensing to the archive contents, FFmpeg, or SHAR source.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository acquisition path and authoritative
  upstream license verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-12.
- Distribution posture: Downloaded external bootstrap tool in an ignored cache.
- Subject class: Archive extraction utility.

## Covered Material

The reduced standalone `7zr.exe` extractor downloaded from the official 7-Zip
site and used to unpack the configured `.7z` FFmpeg archive. The full graphical
7-Zip application and unrelated format handlers are outside the repository use
unless separately introduced.

## Repository Use And Scope

When the required media tools are absent, SHAR downloads the official reduced
extractor into `dependencies/ffmpeg/bootstrap/` and invokes it with bounded
arguments to extract the FFmpeg archive into an ignored staging directory. The
executable is not tracked as repository-authored source.

Extraction success does not authenticate the archive, establish its license, or
make extracted files safe. Archive path traversal, unexpected files, binary
provenance, checksums, signatures, and post-extraction validation remain
independent security requirements.

## Provenance And Version History

For an acquisition that uses 7-Zip, the official standalone extractor actually
retrieved is the relevant evidence. A downloaded executable may lag upstream
because of cached state, compatibility, delayed review, or human oversight. The
exact URL, checksum, file size, upstream release identity, and retrieval date
must be captured for the actual acquisition.

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

- Download only from the configured official upstream location.
- Verify checksum and expected artifact identity before execution.
- Extract only into a fresh, bounded staging directory.
- Validate resulting paths and expected executable identities.
- Keep the bootstrap executable and archive outside Git.
- Recheck the exact artifact license and notices before redistribution.

## Source References

- 7-Zip (n.d.) *Official website*. Available at: <https://www.7-zip.org/>
  (Accessed: 12 July 2026).
- 7-Zip (n.d.) *License for use and distribution*. Available at:
  <https://www.7-zip.org/license.txt> (Accessed: 12 July 2026).
- 7-Zip (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/ip7z/7zip> (Accessed: 12 July 2026).
- SHAR repository (2026)
  `src/pipeline/src/adapters/driven/local/one/media_dependencies.rs`.

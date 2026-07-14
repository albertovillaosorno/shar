# FFmpeg And FFprobe

This non-governing record documents repository-managed media tooling and does
not apply FFmpeg licensing to source media, generated packages, or SHAR code.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository resolver behavior, selected
  Windows build family, FFmpeg licensing modes, and Gyan build composition
  verified; the exact downloaded version, checksum, configuration, enabled
  library set, and corresponding source remain run-specific.
- Counsel review: Not performed.
- As-of date: 2026-07-13.
- Subject class: Open-source multimedia framework and command-line tooling.

## Covered Material

FFmpeg, FFprobe, the selected portable build, and codecs or libraries enabled in
that build, including the HAP encoder required by the movie pipeline.

## Repository Use And Scope

The pipeline resolves FFmpeg and FFprobe from an override, the repository-local
ignored dependency cache, or `PATH`. When bootstrapping on Windows, it downloads
the Gyan.dev release-full archive because the required HAP encoder is not
present in the smaller build selected by the repository analysis. The downloaded
archive and extracted binaries remain ignored external dependencies, not
repository-owned source.

## Provenance And Version History

For a SHAR workflow that uses FFmpeg, the observed compatible build is the
relevant evidence for the required media capabilities. A cached or installed
build may lag upstream because of compatibility, delayed review, or human
oversight. The legal posture depends on the exact build configuration and linked
libraries, not only the FFmpeg release number. Build and distribution evidence
must preserve the binary distributor, version output, configuration flags,
published checksum, local checksum, enabled codecs, source commit, notices,
and corresponding-source obligations.

## Authorship, Ownership, And Attribution

FFmpeg contributors and third-party library authors retain applicable rights.
Rights in input and output media remain separate from rights in the media tool.

## License Or Terms Basis

FFmpeg's official legal page states that the project is licensed under the GNU
Lesser General Public License version 2.1 or later unless optional GPL-covered
parts are enabled, in which case the GPL applies to the full FFmpeg build.
Gyan.dev identifies its Windows variants as 64-bit static GPLv3 builds,
publishes separate essentials and full library inventories, and provides
version and
SHA-256 sidecars. Nonfree or separately licensed components can change
redistribution eligibility. The exact downloaded build, configuration, notices,
corresponding source, and official FFmpeg legal checklist control.

## Distribution, Modification, And Compatibility

Local invocation does not relicense media. Redistributing an FFmpeg build may
require license texts, attribution, corresponding source or relinking materials,
build configuration, and notices for every included library. Patent and codec
questions are jurisdiction- and use-specific.

## Compliance Posture

Do not redistribute an unidentified third-party build. Before execution, compare
the archive against the distributor-published SHA-256 sidecar. Before packaging,
record the exact build, source commit, GPL mode, source location, configuration,
notices, library inventory, and corresponding-source delivery method.

## Source References

- FFmpeg Project (n.d.) *License and legal considerations*. Available at:
  <https://ffmpeg.org/legal.html> (Accessed: 13 July 2026).
- FFmpeg Project (n.d.) *Official GitHub source mirror*. Available at:
  <https://github.com/FFmpeg/FFmpeg> (Accessed: 13 July 2026).
- Gyan Doshi (2026) *FFmpeg builds for Windows*, page version 58. Available at:
  <https://www.gyan.dev/ffmpeg/builds/> (Accessed: 13 July 2026).
- SHAR repository (2026) media dependency resolver and movie-conversion code
  under `src/pipeline/`.

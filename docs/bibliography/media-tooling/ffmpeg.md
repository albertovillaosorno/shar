# FFmpeg And FFprobe

This non-governing record documents repository-managed media tooling and does
not apply FFmpeg licensing to source media, generated packages, or SHAR code.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository resolver behavior, cached
  Gyan FFmpeg 8.1.2 full-build identity, FFprobe identity, published and local
  matching archive SHA-256, configuration, HAP encoder, GPLv3 build posture,
  official FFmpeg current release, and Gyan composition were verified. The
  resolver uses a mutable URL and does not verify the published digest.
- Counsel review: Not performed.
- As-of date: 2026-07-14.
- Subject class: Open-source multimedia framework and command-line tooling.

## Covered Material

FFmpeg, FFprobe, the selected portable build, and codecs or libraries enabled in
that build, including the HAP encoder required by the movie pipeline.

## Repository Use And Scope

The pipeline resolves FFmpeg and FFprobe from an override, the repository-local
ignored dependency cache, or `PATH`. When bootstrapping on Windows, it downloads
the mutable Gyan.dev `ffmpeg-release-full.7z` alias because the required HAP
encoder is not present in the smaller build selected by repository analysis.
The reviewed cache contains FFmpeg and FFprobe 8.1.2 full builds, compiled with
GCC 16.1.0, with GPL and version-3 components enabled and the Vidvox HAP encoder
available.

The cached archive SHA-256 is
`0fff188997a499b5382e0f66e845d4556c48c54f0113ebed4853d556dbdd7059`,
which matched Gyan's published sidecar during this review. The current resolver
does not fetch or verify that sidecar before extraction, so the manual match is
evidence for the reviewed cache rather than proof of resolver integrity. The
archive and extracted binaries remain ignored external dependencies, not
repository-owned source.

## Provenance And Version History

FFmpeg identifies 8.1.2, released 17 June 2026, as the latest stable release from
the 8.1 branch. Gyan identifies its 8.1.2 release build, dated 27 June 2026, as
the current Windows release package and links it to FFmpeg source commit
`38b88335f9`. The mutable release alias resolved to that version during this
review, but may resolve to different bytes after a future upstream update.

For a SHAR workflow that uses FFmpeg, the observed compatible build remains the
controlling evidence for required media capabilities. The legal posture depends
on the exact build configuration and linked libraries, not only the release
number. Build and distribution evidence must preserve the binary distributor,
version output, configuration flags, published checksum, local checksum,
enabled codecs, source commit, notices, and corresponding-source obligations.

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

Do not redistribute an unidentified third-party build. Replace the mutable alias
with a pinned artifact or record the resolved version for every acquisition. Add
automatic comparison against the distributor-published SHA-256 sidecar before
extraction; the current resolver does not perform that check. Before packaging,
record the exact build, source commit, GPL mode, source location, configuration,
notices, library inventory, and corresponding-source delivery method.

## Source References

- FFmpeg Project (2026) *Download FFmpeg*. Identifies FFmpeg 8.1.2 as the latest
  stable 8.1 release, dated 17 June 2026, and documents signed release
  verification. Available at: <https://ffmpeg.org/download.html> (Accessed: 14
  July 2026).
- FFmpeg Project (n.d.) *License and legal considerations*. Available at:
  <https://ffmpeg.org/legal.html> (Accessed: 14 July 2026).
- FFmpeg Project (n.d.) *Official GitHub source mirror*. Available at:
  <https://github.com/FFmpeg/FFmpeg> (Accessed: 14 July 2026).
- Gyan Doshi (2026) *FFmpeg builds for Windows*, page version 58. Identifies the
  current 8.1.2 release-full build, source commit, GPLv3 build posture, version
  sidecar, and SHA-256 sidecar. Available at:
  <https://www.gyan.dev/ffmpeg/builds/> (Accessed: 14 July 2026).
- SHAR repository and ignored dependency cache (2026), media dependency
  resolver, cached FFmpeg and FFprobe 8.1.2 outputs, HAP encoder inventory,
  configuration, archive hash, and published-sidecar comparison.

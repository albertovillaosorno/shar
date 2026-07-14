# curl

This non-governing record documents a system command used by the media
dependency bootstrap and does not apply curl licensing to downloaded archives,
SHAR source, or network content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository invocation, observed curl
  8.19.0 executable and linked-library inventory, official curl 8.21.0 current
  release, command documentation, source repository, and license were verified.
  Executable provenance, package origin, and trust-store state remain local.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-14.
- Distribution posture: External development and bootstrap prerequisite.
- Subject class: Network data-transfer command-line tool.

## Covered Material

The `curl` command-line tool invoked by SHAR to retrieve repository-managed,
ignored dependency artifacts over HTTPS. The libcurl programming library is
covered only to the extent it is incorporated into the installed curl binary;
SHAR does not link directly to libcurl.

## Repository Use And Scope

The media dependency resolver invokes `curl` with bounded command arguments to
download the configured FFmpeg archive and standalone 7-Zip extractor into the
ignored `dependencies/ffmpeg/` cache. The command is resolved from the operator
machine environment and is not copied into Git by SHAR.

This use does not establish that the remote artifact is trustworthy, licensed
for redistribution, current, authentic, or suitable merely because curl
successfully transferred it. URL authority, transport security, checksums,
signatures, artifact provenance, license evidence, and extraction safety remain
separate controls.

## Provenance And Version History

The observed operator environment reports curl 8.19.0 for Windows x64, released
11 March 2026, using Schannel and an identified set of compression, identity,
SSH, and protocol-support libraries. The official curl download page identifies
8.21.0, released 24 June 2026, as the current source release. The observed
executable is therefore older than the reviewed upstream release.

For a SHAR workflow that uses curl, the executable actually resolved from the
operator environment remains the controlling run evidence. Validation or
publication records must capture command output, executable path, package
origin, linked-library inventory, trust-store behavior, and provenance. This
dated comparison does not authorize an upgrade or establish that the observed
binary is insecure.

## Authorship, Ownership, And Attribution

Daniel Stenberg and curl contributors retain applicable rights in curl and
libcurl. Libraries incorporated into a particular binary retain their separate
rights, licenses, notices, and security history.

## License Or Terms Basis

The curl project publishes curl and libcurl under the curl license, identified
by SPDX as `curl`. The project describes that license as inspired by MIT/X but
not identical. Redistribution requires preservation of the copyright and
permission notice, and the exact binary's bundled components require separate
notice review.

## Distribution, Modification, And Compatibility

Invoking curl does not relicense downloaded material or SHAR code. SHAR does not
distribute the operator's curl executable. Bundling curl requires preservation
of the curl license and all licenses and notices for libraries included in the
selected binary.

## Compliance Posture

- Resolve curl from an explicit, auditable operator environment.
- Upgrade the observed executable or document a narrow compatibility reason
  before representing it as current.
- Use HTTPS and fail closed on transfer errors.
- Add cryptographic integrity verification before treating downloads as trusted.
- Record source URL, retrieval date, checksum, and artifact identity privately.
- Do not infer redistribution authority from successful download.
- Recheck the installed binary's component and license inventory before
  bundling.

## Source References

- curl project (2026) *Releases and downloads*. Identifies curl 8.21.0 as the
  current release, dated 24 June 2026, and provides signed source archives.
  Available at: <https://curl.se/download.html> (Accessed: 14 July 2026).
- curl project (n.d.) *Copyright and license*. Available at:
  <https://curl.se/docs/copyright.html> (Accessed: 14 July 2026).
- curl project (n.d.) *curl command-line manual*. Available at:
  <https://curl.se/docs/manpage.html> (Accessed: 14 July 2026).
- curl project (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/curl/curl> (Accessed: 14 July 2026).
- SHAR repository and operator environment (2026),
  `src/pipeline/src/adapters/driven/local/one/media_dependencies.rs` and
  observed curl 8.19.0 Windows x64 version and component output.

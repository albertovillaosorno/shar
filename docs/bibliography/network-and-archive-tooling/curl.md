# curl

This non-governing record documents a system command used by the media
dependency bootstrap and does not apply curl licensing to downloaded archives,
SHAR source, or network content.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Verified — Repository invocation and authoritative upstream
  sources verified.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-12.
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

For a SHAR workflow that uses curl, the observed compatible executable supplied
by the operator environment is the relevant evidence. It may lag upstream
because of operating-system packaging, compatibility, delayed review, or human
oversight. Validation or distribution records must capture command output and
executable provenance for the actual run.

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
- Use HTTPS and fail closed on transfer errors.
- Add cryptographic integrity verification before treating downloads as trusted.
- Record source URL, retrieval date, checksum, and artifact identity privately.
- Do not infer redistribution authority from successful download.
- Recheck the installed binary's component and license inventory before
  bundling.

## Source References

- curl project (n.d.) *Copyright and license*. Available at:
  <https://curl.se/docs/copyright.html> (Accessed: 12 July 2026).
- curl project (n.d.) *curl command-line manual*. Available at:
  <https://curl.se/docs/manpage.html> (Accessed: 12 July 2026).
- curl project (n.d.) *Official GitHub repository*. Available at:
  <https://github.com/curl/curl> (Accessed: 12 July 2026).
- SHAR repository (2026)
  `src/pipeline/src/adapters/driven/local/one/media_dependencies.rs`.

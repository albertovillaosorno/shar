# Voice and language mod packages

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Localized mod capabilities

## Context

Voice and language changes affect locale selection, fallback, provenance, and
package compatibility. A separate ad hoc workflow would duplicate mod policy and
make localized content harder to validate consistently.

## Decision

Voice and language mods use deterministic package identity, provenance,
compatibility, fallback, preview, and validation. Stable local aliases select
supported behavior without hardcoding release titles or versions.

The remaster role may replace only identities present in the unmodified source
installation. Members without an original identity remain optional and are
skipped. The Latino role may add only isolated Latin-American voice and
cinematic audio and cannot overwrite base or remaster output.

## Consequences

- Voice and language packages participate in the same identity, dependency,
  compatibility, preview, and validation flow as other supported mods.
- Locale fallback remains explicit and testable instead of depending on loose
  replacement-file discovery.
- Invalid localized packages fail before partial audio or text activation.
- Either supported package, both packages, or no package remains valid.
- The repository provides compatibility only and does not redistribute or claim
  authorship of third-party packages.

## Rejected alternatives

- Maintaining a separate ad hoc installer for language and voice changes.
- Applying loose replacement files without package identity or provenance.
- Bundling third-party localized media with the repository.

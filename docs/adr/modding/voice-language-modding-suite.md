# Voice and language mod packages

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Localized mod capabilities

## Context

Voice and language changes affect locale selection, fallback, provenance, and
package compatibility. A separate ad hoc workflow would duplicate mod policy and
make localized content harder to validate consistently.

## Decision

Voice and language mods use the same deterministic package identity, provenance,
compatibility, fallback, preview, and validation model as other supported data
and asset packages.

## Consequences

- Voice and language packages participate in the same identity, dependency,
  compatibility, preview, and validation flow as other supported mods.
- Locale fallback remains explicit and testable instead of depending on loose
  replacement-file discovery.
- Invalid localized packages fail before partial audio or text activation.

## Rejected alternatives

- Maintaining a separate ad hoc installer for language and voice changes.
- Applying loose replacement files without package identity or provenance.
- Bundling third-party localized media with the repository.

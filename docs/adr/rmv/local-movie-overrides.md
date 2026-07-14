# Local cinematic overrides

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Optional user-provided cinematic media

## Context

Optional cinematic media must be selectable without becoming canonical source
input or public repository content. Incidental filenames, directory order, or
missing local files would otherwise produce inconsistent playback state.

## Decision

Validated local cinematic overrides may replace supported media without becoming
required input, changing canonical source evidence, or entering public history.
Desktop targets import overrides through a local user-visible source, while
Android uses managed document import into application-owned storage. Both routes
resolve the same canonical cinematic and track identities.

## Consequences

- A supported local override can replace cinematic media without redefining the
  canonical source package or becoming a required dependency.
- Override identity, validation, and selection remain deterministic while the
  media itself stays outside public history.
- Physical paths and Android provider identifiers never become override identity.
- An override must satisfy the canonical timeline and the selected target's
  verified media-player, codec, container, audio, and lifecycle contract.
- Missing or invalid override media falls back to the supported canonical target
  variant instead of leaving a partial cinematic state.

## Rejected alternatives

- Treating optional replacement media as mandatory source input.
- Publishing user-supplied cinematic media or recording its local path.
- Selecting overrides by incidental filename or directory order.

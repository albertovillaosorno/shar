# Lawful local input and publication boundary

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Inputs, outputs, and tracked material

## Context

The project interoperates with a user-supplied copy of a commercial game and
must remain independently authored and safe to publish. Its legal records also
need one stable scope statement so research summaries cannot be mistaken for a
legal recommendation or authorization to act.

## Decision

The repository contains only repository-owned code, schemas, tests,
documentation, and synthetic or otherwise redistributable evidence. Users supply
their own lawful game installation and optional replacement content.

Original payloads, extracted assets, proprietary engine source, third-party
replacement media, credentials, private evidence, and machine-specific state are
not published. The project does not authenticate ownership, download the game,
or authorize redistribution of generated builds.

Public naming uses plain-text factual compatibility references only to the
extent needed to identify the supported product or technology. Third-party
logos, character art, trade dress, and wording that implies affiliation,
sponsorship, endorsement, approval, or current official support are outside the
repository baseline.

The canonical legal research disclaimer is the single source of truth for the
scope of legal research. The public overview and every legal record link to that
notice instead of duplicating or varying it. Legal records may explain
verified authorities and fact-dependent risk boundaries, but they do not grant
permission, determine legality, or replace advice for a particular person,
copy, agreement, jurisdiction, or publication.

## Consequences

- Tracked fixtures require lawful provenance.
- Local conversion output is user evidence, not distributable project content.
- Public manifests avoid unnecessary original names.
- Repository and package presentation remains visually independent from
  third-party branding.
- Legal-scope language remains consistent through one disclaimer record.

## Rejected alternatives

- Bundling source game files or extracted payloads.
- Treating interoperability as redistribution permission.

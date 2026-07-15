# Local drop-in mod packages and AI skills

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Local mod creation and activation

## Context

Local mods need repeatable identity, ordering, compatibility, and activation
semantics across desktop and Android storage models. Without a package contract,
manual file replacement and AI-assisted changes create unreviewable state that
cannot be previewed or reproduced. A public desktop directory is not a portable
storage contract for mobile platforms.

## Decision

Mods are deterministic local packages with explicit identity, priority,
dependencies, compatibility, supersession, conflicts, trust level, provenance,
dry-run preview, and fail-closed validation. User-facing AI skills translate
authorized requests into reviewable packages.

Package identity, dependency order, and activation state are independent of the
physical storage adapter. Desktop targets may discover packages through a local
user-visible directory. Android imports a selected package into
application-owned
storage through a native platform adapter. Both routes produce the same
validated
package model before preview or activation.

Content-only packages are portable when their declared capabilities are
supported. Native-code packages declare exact operating-system, architecture,
and runtime compatibility and remain inactive on every unmatched target.

## Consequences

- Mod ordering, dependencies, compatibility, supersession, conflicts, trust, and
  provenance are explicit package data.
- User-facing AI guidance produces a reviewable package and dry-run preview
  instead of applying opaque editor or filesystem changes.
- Invalid packages fail before activation and cannot leave a partial mod state.
- Desktop directory discovery and Android managed import converge on one package
  identity, validation, load-order, and activation contract.
- Case, separator, Unicode, archive-entry, and storage-location differences
  cannot alter package identity or priority.
- Native code never inherits portability from a content-only package.

## Rejected alternatives

- Loose file overrides whose priority depends on directory enumeration.
- Direct AI mutation without a package, preview, and validation boundary.
- Resolving conflicts implicitly or after partial activation.
- Requiring every platform to expose a public `mods/` directory.
- Deriving priority or identity from filesystem enumeration, filename case, or
  archive-entry order.
- Loading a native binary on an undeclared operating system or architecture.

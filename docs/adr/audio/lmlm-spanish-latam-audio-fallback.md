# Latin American Spanish audio fallback

- Status: Accepted
- Decision date: 2026-07-12
- Scope: Optional localized audio selection

## Context

Locale selection needs an explicit precedence rule when optional Latin American
Spanish audio is present. Without a fixed fallback chain, equivalent local
installations can resolve different media or confuse user-provided overrides
with repository-distributed content.

## Decision

When a validated Latin American Spanish override exists for a supported asset,
it takes precedence for that locale. Otherwise the supported base Spanish asset
remains the locale fallback, and the global base-language asset remains the
final fallback. The repository never distributes the optional override.

Locale resolution selects one canonical audio identity before target cooking.
Windows, Linux, macOS, Android, x64, ARM64, compression, streaming, cache, and
physical storage differences cannot change the fallback result.

## Consequences

- Locale resolution is deterministic across the override, base-Spanish, and
  global fallback tiers.
- A missing optional override never blocks supported base audio.
- Optional localized media remains user-supplied and outside repository
  publication.

## Rejected alternatives

- Requiring the optional override before Latin American Spanish can resolve.
- Guessing locale assets from filenames or distributing the override with the
  repository.

# SHAR mod-package contract

`shar_mod_package` owns the storage-independent manifest model used by generated
official mods and user-authored SHAR mods. Package identity comes from the
manifest, never from a folder name, archive filename, import path, or discovery
order.

Version 1 is intentionally narrow and fail-closed. It validates deterministic
identity, exact revisions, explicit priority, dependency/conflict/supersession
relations, content/native trust boundaries, portable member paths, byte lengths,
SHA-256 identities, provenance, capabilities, and target declarations. One
package identity can name at most one exact dependency revision and cannot be
simultaneously required and declared conflicting or superseded.
Member construction, whole-manifest validation, and content-revision derivation
share the same reserved-path, portable-path, size, digest, media-type, and role
rules so invalid member records cannot acquire a canonical package revision.

Variable-length path, media-type, and role fields are length-prefixed in the
content-revision digest so distinct canonical field boundaries cannot alias.
Manifest validation recomputes that content revision from canonical members and
rejects stale or arbitrary package revision tokens before serialization/import.
The pure `dependency_load_order` helper validates a complete candidate slice,
requires exact dependency revisions, rejects duplicate identities and cycles,
and uses canonical identity to make dependency-ready ordering independent of
package discovery order. The separate `validate_active_conflicts` helper rejects
any declared conflict whose canonical identity is present in the active
candidate set while ignoring conflicts with inactive packages.

Active
supersession cycles are rejected independently as well. Full priority and
supersession winner-selection policy remains outside these declaration-level
helpers.

Transport (`directory`, `.zip`, Android/iOS document selection), staging,
preview,
and atomic activation remain separate adapters/use cases. This crate does not
execute package-provided code and a structurally valid native package is not a
claim of safety.

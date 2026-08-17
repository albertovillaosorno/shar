# SHAR mod-package contract

`shar_mod_package` owns the storage-independent manifest model used by generated
official mods and user-authored SHAR mods. Package identity comes from the
manifest, never from a folder name, archive filename, import path, or discovery
order.

Version 1 is intentionally narrow and fail-closed. It validates deterministic
identity, exact revisions, explicit priority, dependency/conflict/supersession
relations, content/native trust boundaries, portable member paths, byte lengths,
SHA-256 identities, provenance, capabilities, and target declarations.

Transport (`directory`, `.zip`, Android/iOS document selection), staging, preview,
and atomic activation remain separate adapters/use cases. This crate does not
execute package-provided code and a structurally valid native package is not a
claim of safety.

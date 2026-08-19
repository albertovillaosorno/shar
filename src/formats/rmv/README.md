# Rmv Function

## Purpose

Defines the `src/formats/rmv` boundary.

## Ownership

Owns movie-container classification and validation used by RMV audit and source
authority checks. Signature classification is explicitly non-authoritative.
Complete-byte classification validates Bink size and dimension fields, complete
Ogg page framing and checksums, and XMV fixed/per-track header bounds. Radical
`rmv` prefixes remain descriptive evidence only and do not pass complete-byte
validation.

## Prohibitions

Does not own generated artifacts, local dependencies, or game content.

## Navigation

- `composition`
- `domain`

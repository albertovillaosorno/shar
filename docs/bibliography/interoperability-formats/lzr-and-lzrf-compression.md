# LZR And LZRF Compression

This non-governing record documents proprietary compression variants encountered
in P3D extraction without granting rights in historical code, tools,
documentation, compressed content, or decompressed assets.

## Review Status And Scope

- Review status: Evidence recorded.
- Evidence status: Partially verified — Repository implementation and local
  historical evidence verified; public authoritative specification not located.
- Counsel review: Not performed.
- Jurisdictional scope: Not determined.
- As-of date: 2026-07-12.
- Distribution posture: Local decompression of user-supplied P3D input.
- Subject class: Proprietary LZ-family compression variants used with P3D data.

## Covered Material

The `LZR` and `LZRF` stream identities recognized by `src/p3d/`, including
literal commands, match commands, length extensions, offsets, output-size
validation, stream termination, and P3DZ wrapper evidence.

## Repository Use And Scope

SHAR contains independently authored bounded decompressors used only when a
user-supplied P3D source identifies a supported compressed form. The parser
checks input and output bounds, invalid offsets, truncation, and expected output
length. It does not distribute historical compressors, decompressor source, or
compressed game payloads.

## Provenance And Version History

Repository evidence distinguishes `LZR` and `LZRF` behavior and labels a P3DZ
compression path. No authoritative public specification, complete version
history, compressor identity, patent inventory, or public license has been
verified. Similarity to a general LZ family does not establish equivalence to a
published algorithm.

## Authorship, Ownership, And Attribution

Historical developers, tool authors, publishers, licensors, and successors
retain applicable rights in upstream implementations and documentation. SHAR
contributors retain rights only in independently authored repository code to the
extent supported by authorship evidence and law. Rights in compressed and
decompressed content remain separate.

## License Or Terms Basis

No standalone public license or specification grant for LZR or LZRF has been
verified. The repository MIT License does not apply to historical code,
compressed content, proprietary tools, patents, trade secrets, or third-party
contracts.

## Distribution, Modification, And Compatibility

Decompression changes representation, not ownership. Successful output does not
establish that the source, decompressed asset, algorithm, or historical tool may
be distributed. Patent, trade-secret, contract, and anti-circumvention questions
require separate fact-specific review.

## Compliance Posture

- Accept only explicitly recognized wrappers and fail closed on malformed input.
- Bound all input reads, output writes, lengths, and match offsets.
- Record source hash, wrapper identity, expected size, and decompressed hash.
- Keep compressed and decompressed proprietary payloads outside Git and
  releases.
- Use synthetic independently authored fixtures for tracked tests.
- Do not describe LZR or LZRF as an open or standardized algorithm.
- Preserve the unresolved public-specification and rights questions explicitly
  in this record and the corresponding legal records.

## Source References

- SHAR repository (2026) `src/p3d/src/domain/extract.rs` and compression tests.
- Historical local Radical material reviewed for interoperability; source code
  and private routes not distributed.
- [Pure3D P3D](pure3d-p3d.md).
- [SHAR legal research index](../../legal/index.md).

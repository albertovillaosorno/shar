# Base game local input

This directory holds lawful local evidence used while defining the base-game
minimum. Its payloads are intentionally ignored by Git.

The current maintainer reference set contains:

- `Simpsons.exe` as a private cross-variant common-byte comparison artifact;
- `Simpsons-no-cd-v1.exe`, `Simpsons-no-cd-v2.exe`, and
  `Simpsons-no-cd-v3.exe` as retained executable-variant evidence;
- `Simpsons.ico`;
- `ambience.rcf`, `carsound.rcf`, `dialog.rcf`, `nis.rcf`, `scripts.rcf`, and
  `soundfx.rcf`;
- `music00.rcf`, `music01.rcf`, `music02.rcf`, and `music03.rcf`; and
- `movies/fmv1A.rmv`, plus `movies/fmv2.rmv` through `movies/fmv8.rmv`.

These files describe this local authoring workspace. The common-byte artifact
is derived comparison evidence, not an original executable edition and not a
direct `shar.algorithm.v1` replay input. The generic algorithm contract binds
exact source bytes, so cross-edition replay would first need a public-safe,
deterministic projection from the user's lawful executable. These files do not
by themselves define the final public source-admission gate, and users are not
expected to supply every executable variant.

# LMLM compatibility tool

This directory is the complete user-facing LMLM compatibility boundary. It is
separate from SHAR's faithful base extraction/build pipeline and uses only the
Python standard library.

Do **not** start new mods in LMLM. This tool exists only to recover inspectable
content from supported legacy packages so an author can adapt that content into
a modern SHAR mod.

## Download and run

Copy or download this `tools/lmlm/` directory. Python 3.12 or newer is enough;
the SHAR repository, Rust, Unreal Engine, and the original game are not runtime
dependencies of this tool.

Inspect without extracting content:

```text
python main.py inspect MyLegacyMod.lmlm
```

Create an open conversion workspace:

```text
python main.py convert MyLegacyMod.lmlm converted-mod
```

Optionally create a ZIP of that workspace too:

```text
python main.py convert MyLegacyMod.lmlm converted-mod --zip converted-mod.zip
```

The workspace contains `content/` with strictly validated extracted files and
`conversion-report.json` with source and per-file SHA-256 evidence. Its status
is `extracted-needs-shar-package-adaptation`: the final SHAR mod package schema
is still being defined, so this tool does not pretend an extracted legacy
package is already a valid modern SHAR mod.

## Safety and legal boundary

The tool never installs a mod or reads SHAR's `game/` directory. It refuses to
overwrite an existing conversion directory or ZIP and validates archive paths,
portable path collisions, reserved bytes, table/payload separation, alignment,
and overlapping payload ranges before publishing output.

`decompilable_mods_only` is fixed to `true`. Unsupported or opaque legacy
behavior fails or remains something the mod author must recreate explicitly.
Users are responsible for having the rights needed to inspect, convert, modify,
or redistribute the content they provide to the tool.

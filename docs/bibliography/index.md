# SHAR Bibliography And Provenance Index

See the [SHAR Documentation Guide](../README.md) for documentation ownership,
maintenance, validation, and public-content rules. Read the
[Bibliography Research Disclaimer](disclaimer.md) before relying on a source or
provenance record.

This directory contains non-governing source, provenance, license, standards,
and third-party notice records. Legal analysis belongs in
[SHAR Legal Authorities And Repository Boundaries](../legal/index.md). Technical
decisions remain in [`docs/adr`](../adr/), and the ADR practice itself is
documented here.

Use the [bibliography record template](template.md) for new records.

## Version Selection And Drift Evidence

Each subject record distinguishes the version observed in repository evidence
from the most recent upstream release verified on its access date.
Compatibility, reproducibility, security, and supported-platform constraints can
justify an older observed version; this index does not mandate upgrades or
represent every installed component as current.

An installed or observed version is time-bounded evidence. It may lag because of
compatibility work, a deliberate stability hold, unavailable packaging, delayed
review, or ordinary maintenance lag. A record must not call a version latest
unless verified against an authoritative source on the access date.

For a specific build or validation run, manifests, lockfiles, descriptors,
package metadata, executable output, checksums, and recorded evidence establish
the observed component identity.

## Construction Environment

Host, editor, build, hosting, and operator-facing tools.

- [Comet Browser](construction-environment/comet-browser.md)
- [Cursor](construction-environment/cursor.md)
- [GNU Bash And Windows PowerShell][bash-powershell]
- [.NET SDK And Runtime](construction-environment/dotnet-sdk.md)
- [Git](construction-environment/git.md)
- [GitHub](construction-environment/github.md)
- [Microsoft C++ Build Toolchain][cpp-build-toolchain]
- [Node.js](construction-environment/nodejs.md)
- [pnpm](construction-environment/pnpm.md)
- [ripgrep](construction-environment/ripgrep.md)
- [Windows 11](construction-environment/windows-11.md)

## External Assistance Services

Optional external service and terms records.

- [Anthropic Claude](ai-assistance/anthropic-claude.md)
- [OpenAI Codex And ChatGPT](ai-assistance/openai-codex-and-chatgpt.md)

## Organizations And Rights Holders

Organizations that issue relevant product terms, provide technology, or control
marks and other rights. Product-specific licensing remains in the corresponding
subject record and controlling instrument.

- [Epic Games](organizations/epic-games.md)

## Platform And Creator Policies

Official platform rules and creator-content policies used as non-governing
source evidence for publication and monetization analysis.

- [Xbox Game Content Usage Rules][xbox-content-rules]
- [YouTube Game Content And Monetization Policies][youtube-content-policy]

## Model Review Applications

Applications used to inspect generated interchange artifacts.

- [Autodesk Maya 2027](model-review-applications/autodesk-maya-2027.md)
- [Blender](model-review-applications/blender.md)

## Programming Languages

Languages and official toolchains, distinct from libraries and runtimes.

- [C++](programming-languages/cpp.md)
- [C\#](programming-languages/csharp.md)
- [Python](programming-languages/python.md)
- [Rust](programming-languages/rust.md)

## Third-Party Libraries

Direct external libraries with independent license evidence.

- [Serde JSON](third-party-libraries/serde-json.md)
- [Serde](third-party-libraries/serde.md)

## Packaging Tooling

Package build backends and package-construction tools.

- [Hatchling](packaging-tooling/hatchling.md)

## Protocols, Standards, And Documentation Practices

Published specifications, encodings, interchange conventions, ADRs, and
change-history practices.

- [Architecture Decision Records][adr-records]
- [Autodesk FBX](protocols-and-standards/autodesk-fbx.md)
- [Bink Video](protocols-and-standards/bink-video.md)
- [Calendar Versioning](protocols-and-standards/calendar-versioning.md)
- [Comma-Separated Values](protocols-and-standards/comma-separated-values.md)
- [Conventional Commits](protocols-and-standards/conventional-commits.md)
- [EditorConfig](protocols-and-standards/editorconfig.md)
- [HAP Video Codec](protocols-and-standards/hap-video-codec.md)
- [INI Configuration Family][ini-family]
- [JSON And JSON Lines](protocols-and-standards/json-and-json-lines.md)
- [Markdown And CommonMark](protocols-and-standards/markdown-and-commonmark.md)
- [Model Context Protocol](protocols-and-standards/model-context-protocol.md)
- [Portable Network Graphics][png]
- [DDS, TGA, And BMP Raster Texture Inputs][raster-texture-inputs]
- [Microsoft Rich Text Format](protocols-and-standards/rich-text-format.md)
- [RIFF, WAVE, And PCM](protocols-and-standards/riff-wave-pcm.md)
- [SHA-256](protocols-and-standards/sha-256.md)
- [SPDX License Identifiers][spdx]
- [TOML](protocols-and-standards/toml.md)
- [Unicode And UTF-8](protocols-and-standards/unicode-utf-8.md)

## Engine And Plugins

Unreal Engine and enabled external engine capabilities.

- [Unreal Engine](engine-and-plugins/unreal-engine.md)
- [Unreal Modeling Tools Editor Mode][unreal-modeling-tools]
- [Unreal Native MCP Plugins](engine-and-plugins/unreal-native-mcp-plugins.md)

## Media Tooling

External media inspection and conversion tools.

- [FFmpeg And FFprobe](media-tooling/ffmpeg.md)

## Network And Archive Tooling

External download and archive-extraction tools.

- [curl](network-and-archive-tooling/curl.md)
- [7-Zip](network-and-archive-tooling/seven-zip.md)

## Validation Tooling

Linters, formatters, type checkers, tests, and coverage tools.

- [BasedPyright](validation-tooling/basedpyright.md)
- [Clippy](validation-tooling/clippy.md)
- [CSpell](validation-tooling/cspell.md)
- [Coverage.py](validation-tooling/coverage-py.md)
- [LLVM Clang, Clang-Tidy, And Clang-Format][llvm-validation]
- [markdownlint-cli2 And markdownlint](validation-tooling/markdownlint-cli2.md)
- [pytest-cov](validation-tooling/pytest-cov.md)
- [pytest](validation-tooling/pytest.md)
- [Ruff](validation-tooling/ruff.md)
- [rustfmt](validation-tooling/rustfmt.md)

## Proprietary Product Subjects

Proprietary products studied only as compatibility and historical subjects.

- [The Simpsons: Hit & Run](proprietary-knowledge/the-simpsons-hit-and-run.md)

## Supporting Sources

Secondary navigation and acquisition-provenance references.

- [Jebano Latin Spanish Mod Tutorial][jebano-tutorial]
- [The Simpsons Hit And Run Wiki][shar-wiki]

## Interoperability Formats

Independently studied proprietary and unresolved format families.

- [CHO Choreography](interoperability-formats/choreography-cho.md)
- [ERR Diagnostic Log](interoperability-formats/error-log-err.md)
- [LMLM And LSPA Interoperability][lmlm-lspa]
- [LZR And LZRF Compression][lzr-lzrf]
- [MFK And CON Command Scripts][mfk-con]
- [Pure3D P3D](interoperability-formats/pure3d-p3d.md)
- [RadCore Cement RCF](interoperability-formats/radcore-cement-rcf.md)
- [Radical Entertainment Historical Toolchain][radical-toolchain]
- [RadMovie RMV](interoperability-formats/radmovie-rmv.md)
- [RadSound RSD](interoperability-formats/radsound-rsd.md)
- [Scrooby PAG, SCR, And PRJ](interoperability-formats/scrooby-pag-scr-prj.md)
- [TYP Sound Resource Metadata][typ-metadata]
- [SPA Format Identity](interoperability-formats/spa-format-identity.md)
- [TextBible Language Files][textbible]

[bash-powershell]:
  construction-environment/bash-and-windows-powershell.md
[cpp-build-toolchain]:
  construction-environment/microsoft-cpp-build-toolchain.md
[xbox-content-rules]:
  platform-policies/xbox-game-content-usage-rules.md
[youtube-content-policy]:
  platform-policies/youtube-game-content-and-monetization.md
[adr-records]:
  protocols-and-standards/architecture-decision-records.md
[ini-family]:
  protocols-and-standards/ini-configuration-family.md
[png]:
  protocols-and-standards/portable-network-graphics.md
[raster-texture-inputs]:
  protocols-and-standards/raster-texture-inputs-dds-tga-bmp.md
[spdx]:
  protocols-and-standards/spdx-license-identifiers.md
[unreal-modeling-tools]:
  engine-and-plugins/unreal-modeling-tools-editor-mode.md
[llvm-validation]:
  validation-tooling/llvm-clang-and-clang-tidy.md
[jebano-tutorial]:
  research-sources/jebano-youtube-latin-spanish-mod-tutorial.md
[shar-wiki]:
  research-sources/simpsons-hit-and-run-fandom-wiki.md
[lmlm-lspa]:
  interoperability-formats/lmlm-lspa-interoperability.md
[lzr-lzrf]:
  interoperability-formats/lzr-and-lzrf-compression.md
[mfk-con]:
  interoperability-formats/mission-and-console-scripts-mfk-con.md
[radical-toolchain]:
  interoperability-formats/radical-entertainment-toolchain-and-formats.md
[typ-metadata]:
  interoperability-formats/sound-resource-type-typ.md
[textbible]:
  interoperability-formats/textbible-language-files.md

## Catalog Admission Criteria

These criteria describe bibliography catalog structure and evidence labeling
only. They do not create technical, legal, publication, distribution, or
operational authority.

1. One active file covers one subject or one explicitly related family.
1. Material claims identify their evidence class and source.
1. Construction tools are not runtime dependencies by implication.
1. Public availability and successful parsing are not license grants.
1. Direct dependencies and material formats receive first-class records.
1. Transitive dependencies remain governed by distribution-time inventories.
1. Currentness must be verified for the relevant run.
1. Legal conclusions belong in `docs/legal`.
1. Technical decisions belong in `docs/adr` and must link to evidence.
1. Commit and compatibility-snapshot conventions never authorize a commit,
   publication, or distribution.

`Evidence recorded` means a record has captured the available evidence,
repository relationship, and known limitations. It does not mean every
proposition is verified; each record's evidence-status language controls that
assessment.

## Current Coverage

- Active subject records: 78.
- Template records: 1.
- Review date: 2026-07-14.
- Legal conclusions: not provided by this index.

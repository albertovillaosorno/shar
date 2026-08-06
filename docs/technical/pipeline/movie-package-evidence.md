# Movie package evidence

- Status: Active
- Last reviewed: 2026-08-05

## Purpose

This specification defines the public evidence produced when one admitted movie
source is prepared as an Unreal HAP package. It covers package-local paths,
source identity, diagnostics, media-tool selection, and fail-closed behavior.
It does not define shipping platform media formats or runtime playback policy.

## Package contents

One movie package owns these stable relative identities:

```text
.
├── manifest.json
├── movie.mov
├── movie.bk2
├── audio_track_01.wav
├── audio_track_02.wav
├── decode-report.json
├── source-video.ffprobe.json
└── timing.tsv
```

`movie.bk2` is an optional compatibility destination. The HAP video, decoded
audio tracks, probe evidence, timing evidence, and both JSON documents remain
inside the same package directory.

## Manifest path contract

The `shar-schoenwald.rmv-unreal-hap-package.v1` manifest publishes only logical
or package-relative paths:

- `logical_path` and `selected_source` use `/` separators;
- `movie_directory` is `.`;
- `hap_video_path` is `movie.mov`;
- `audio_track_pattern` is `audio_track_%02d.wav`;
- `timing_manifest_path` is `timing.tsv`;
- `source_probe_path` is `source-video.ffprobe.json`; and
- `optional_bk2_path` is `movie.bk2`.

The `shar-schoenwald.rmv-hap-export.v1` decode report follows the same rule.
Its `video_path` is `movie.mov`, and its source-probe reference is
`source-video.ffprobe.json`.

A manifest cannot contain the extraction root, transaction staging name,
operator home, drive prefix, or another package's path. An artifact that cannot
be relativized beneath its owning package fails before the document is written.

## Source and provenance

The package records the normalized logical movie identity, source-relative path,
container classification, source byte count, SHA-256 digest, embedded provenance
summary, frame-rate evidence, and selected cinematic target. Relocating an
equivalent lawful source installation or extraction root cannot change those
identities.

Physical input paths remain process arguments only. They are not evidence and
are not serialized.

## External media tools

Movie conversion may use FFmpeg and FFprobe from an explicit local override, the
ignored repository dependency cache, or the operator `PATH`. A filesystem
candidate is accepted only when its direct entry is a regular file. Directories,
symlinks, junctions, and other special entries are not selected as cached or
overridden tools.

Tool-start and dependency-filesystem failures report:

- the attempted public action;
- the logical source identity when one movie is affected; and
- the operating-system error class.

They do not copy the physical executable path, source path, cache path, or raw
operating-system error text into the public diagnostic.

## Failure behavior

The pipeline fails closed when:

- a planned movie artifact escapes its package directory;
- a selected tool candidate is not a direct regular file;
- FFmpeg or FFprobe cannot start;
- a probe or conversion process returns failure;
- required video or audio output is absent or empty; or
- generated JSON is malformed.

A failed complete extraction remains isolated by the recoverable extraction
transaction. A failed partial movie export remains serialized by the shared
output lease, but it does not claim whole-root atomic publication.

## Verification

Unit tests construct plans below deliberately sensitive staging-like roots and
prove that neither JSON document contains those roots. They also verify stable
forward-slash logical paths, exact package-relative field values, rejection of
escaped artifacts, public-safe tool diagnostics, and regular-file-only tool
selection.

Repository validation additionally checks generated source files for
machine-specific paths and runs the pipeline test and Clippy gates.

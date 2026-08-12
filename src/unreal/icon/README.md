# Unreal icon pipeline

This function owns deterministic cross-platform application icon generation for
SHAR. The public `composition/pipeline/main.py` command derives every path from
its repository location; it never embeds a workstation path.

## Source boundary

The locally extracted `game/*.ico` files are evidence only. They are already
excluded by the repository game-content boundary and are not copied into source.
The authored SVG masters under `composition/algorithm/assets/` are also local
oracle material and are ignored after authoring.

`composition/algorithm/main.rs` is the distributable source-bound transform. It
contains authenticated protected target material, but reconstruction succeeds
only when the admitted original `game/*.ico` evidence matches the source used to
author the transform. Extra files in `game/` are irrelevant: the transform
validates only the source paths declared in its authenticated snapshot.

## Commands

Run the pipeline from the repository root:

```text
python src/unreal/icon/composition/pipeline/main.py build
```

`build` is also the default command. It reconstructs the SVG tree from
`game/*.ico` plus `composition/algorithm/main.rs`, then creates all platform
exports. The other explicit commands are:

- `author`: regenerate `main.rs` from local `game/*.ico` and ignored authored
  SVGs;
- `reconstruct`: recover only the SVG masters into the ignored output tree;
- `export`: package an already reconstructed SVG tree;
- `export-local`: package the ignored authored SVGs directly for review; and
- `all`: author, reconstruct, and export in one invocation.

`author` is the only normal workflow that requires
`composition/algorithm/assets/`. Consumers of the committed transform need only
the matching local game ICO evidence.

## Outputs

`composition/algorithm/out/` is ignored generated state. A normal build emits:

- a PNG-backed multi-resolution Windows ICO from `windows-linux.svg`;
- a macOS ICNS and iconset from `macos-linux.svg`;
- Linux hicolor PNG/SVG trees, with `windows-linux.svg` as the primary style and
  `macos-linux.svg` as an alternate style; and
- Android adaptive resources with `android.svg` as the transparent
  foreground, a separate black background, black-backed legacy icons, and
  both opaque and transparent-reference 512 px store assets; and
- an iOS `Assets.xcassets/AppIcon.appiconset` generated from `ios.svg`,
  using the current single-size 1024 px asset-catalog workflow.

The Linux tree preserves a scalable SVG and supplies common raster sizes for
launchers that do not render SVG application icons directly. The authored
`android.svg` intentionally keeps transparency around its circular artwork;
the exporter fits that artwork inside Android's adaptive safe zone, supplies
black as a separate background, and composites the same black below
legacy/store rasters. The authored `ios.svg` remains the
single source for the iOS icon; the platform applies its own final mask.

## Dependencies

The pipeline has no project Python dependency manifest. Inkscape is used when it
is already available. Otherwise `resvg_py` is provisioned only into a temporary
directory for the current process; `uv` is preferred when present and
`pip --target` is the compatibility fallback. The temporary dependency directory
and its cache are removed when the renderer closes.

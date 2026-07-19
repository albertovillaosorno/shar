# Synthetic Unreal character-pipeline transport fixture

- Status: Active
- Last reviewed: 2026-07-18

## Purpose

This directory contains a tiny independently authored transport fixture for the
public Unreal import contract. It does not represent a shipping character and
does not contain extracted, branded, private, or third-party content.

The fixture proves that a deterministic normalized package can carry geometry,
UVs, vertex colors, material identity, a lossless texture, native target
identity, and expected read-back values before real game assets are available.

## Files

- `SM_unreal_fixture_triangle.fbx` is a binary FBX 7.7 file containing one
  100-centimeter right triangle, one UV channel, three vertex colors, one normal
  direction, one material slot, no rig, and no animation.
- `T_unreal_fixture_triangle_BC.png` is an 8-by-8 RGBA checkerboard authored by
  the fixture generator.
- `unreal-import-plan.json` is the deterministic Phase 6 import-plan envelope.
- `expected-native-read-back.json` defines the native Static Mesh values that an
  editor-side import test must verify.

## Deterministic provenance

The FBX is generated with the repository-owned public `fbx` crate using the same
synthetic triangle domain values on every run. The PNG is generated from fixed
RGBA rows with the Python standard library and zlib compression level nine. The
JSON files use sorted keys, UTF-8, LF endings, and a final newline.

Current SHA-256 digests:

```text
77233971822a36a4e01ca42daa07e6a50574f7c7dc6a4a51c91b0928916eeb1e  SM_unreal_fixture_triangle.fbx
0c1cdacf6d41ca1a607be2e2a41b18707cc531bcf64326a3dcfbe0dd3892170b  T_unreal_fixture_triangle_BC.png
b2c4c0608f6c78f147d92c3cc0d176f29f9933e0f20b6092edf957e21cc8d538  unreal-import-plan.json
3ae21c1748932fe83f9cd97804fd9a6fb85cb1e36c06e753730c392d5fed5a4b  expected-native-read-back.json
```

## Native test destination

Editor automation imports into `/Game/SHAR/Tests/Generated`. Generated `.uasset`
and `.umap` files remain ignored. A clean test deletes that native test root,
imports the fixture, validates the asset, reads it back, compares the expected
contract, and deletes the generated result.

The fixture cannot be promoted into shipping content and cannot be used as
visual fallback art.

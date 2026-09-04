# Synthetic Unreal character-pipeline transport fixture

- Status: Active
- Last reviewed: 2026-08-06

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
  direction, explicit polygon smoothing metadata, one material slot, no rig,
  and no animation.
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
4c2ab049d7726f179fd60ef8fde1df44c6a51ad6a191f1917764c25d26915ee3  SM_unreal_fixture_triangle.fbx
0c1cdacf6d41ca1a607be2e2a41b18707cc531bcf64326a3dcfbe0dd3892170b  T_unreal_fixture_triangle_BC.png
83a3b7d8ff344c58d3e7e3b93165879066299d159098116192c8860523a71d24  unreal-import-plan.json
34850f18e3725e8f79a0a3a0f53fd35aa71601acf6fe74b9bed138930853750c  expected-native-read-back.json
```

## Native test destination

Editor automation imports into `/Game/SHAR/Tests/Generated`. Generated `.uasset`
and `.umap` files remain ignored. A clean test deletes that native test root,
imports the fixture, validates the asset, reads it back, compares the expected
contract, and deletes the generated result.

## Verified native read-back

A clean Unreal MCP transaction on 2026-08-06 imported the FBX with the native
`StaticMeshTools.import_file` command, read the generated object, and deleted
the generated asset. `ObjectTools.get_class` returned
`/Script/Engine.StaticMesh`; the mesh tools returned three vertices, one
triangle, one LOD, the `fixture_material` slot, and bounds with an extent of
`[50.0, 0.0, 50.0]` centimeters. A second existence check confirmed that the
asset was absent after deletion.

The current native tool catalog does not expose a Static Mesh UV-channel count,
so the one-channel expectation remains a fixture contract awaiting an explicit
read-back tool. It must not be reported as native evidence until that tool
exists and the clean transaction verifies it.

The fixture cannot be promoted into shipping content and cannot be used as
visual fallback art.

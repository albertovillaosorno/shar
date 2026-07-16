// File:
//   - binary_only_contract.rs
// Path:
//   - src/fbx/tests/binary_only_contract.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Regression coverage for the binary-only FBX export architecture.
// - Must-Not:
//   - Parse private assets, invoke review tools, or duplicate format tests.
// - Allows:
//   - Inspect tracked composition-root source text for retired export surfaces.
// - Split-When:
//   - Another format becomes an independently approved canonical artifact.
// - Merge-When:
//   - A repository architecture gate owns the identical source assertions.
// - Summary:
//   - Prevents alternate FBX formats and authoring-helper reintroduction.
// - Description:
//   - Proves the direct binary writer API and absence of external helpers.
// - Usage:
//   - Run through the fbx crate test suite.
// - Defaults:
//   - Documentation may name rejected formats when explaining the boundary.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Binary-only FBX export architecture regression.
//!
//! This test inspects only public composition-root source text. It guards the
//! direct binary writer API, FBX 7.7 identity, and retirement of external
//! authoring helpers without reading generated assets or local machine paths.
//!
//! Rejected legacy tokens are intentionally kept out of implementation source;
//! explanatory documentation may still name retired formats.

use fbx as _;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

const DRIVEN_MODULE: &str = include_str!("../src/adapters/driven.rs");
const PIPELINE_EXPORT: &str =
    include_str!("../../pipeline/src/adapters/driven/local/fbx_export.rs");
const PIPELINE_CLI: &str =
    include_str!("../../pipeline/src/adapters/driving/cli.rs");
const PIPELINE_OPTIONS: &str =
    include_str!("../../pipeline/src/adapters/driving/cli/options.rs");
const PIPELINE_PORTS: &str = include_str!("../../pipeline/src/ports.rs");
const SEMANTIC_TEXTURE_CLI: &str =
    include_str!("../src/semantic_character_texture_cli.rs");
const SEMANTIC_TEXTURE_PACKAGE: &str = include_str!(
    "../src/adapters/driven/semantic_character_texture/package.rs",
);
const SEMANTIC_TEXTURE_PUBLICATION: &str = include_str!(
    "../src/adapters/driven/semantic_character_texture/publication.rs",
);

#[test]
fn exposes_only_binary_fbx_7700() -> Result<(), String> {
    let contract_sources = [
        DRIVEN_MODULE,
        PIPELINE_EXPORT,
        PIPELINE_CLI,
        PIPELINE_OPTIONS,
        PIPELINE_PORTS,
        SEMANTIC_TEXTURE_CLI,
        SEMANTIC_TEXTURE_PACKAGE,
        SEMANTIC_TEXTURE_PUBLICATION,
    ]
    .join("\n");
    for required in [
        "pub mod binary_character_writer;",
        "mod binary_fbx;",
        "write_binary_character_fbx",
        "write_binary_character_fbx_embedded",
        "embed_textures",
        "&prepared.animations",
        "body-atlas.png",
    ] {
        if !contract_sources.contains(required) {
            return Err(format!("missing binary-only contract: {required}"));
        }
    }
    for forbidden in [
        "ascii_character_writer",
        "ascii_scene_writer",
        "write_character_fbx",
        "7_400",
        "7400",
        ".maya.fbx",
        "MayaAscii",
        "maya_ascii",
        "blender_review_helper",
        "blender_scene_writer",
        "maya_import_helper",
        "write_review_helper",
        "write_maya_import_helper",
        "--blender-helper",
        "--maya",
        ".maya.py",
    ] {
        if contract_sources.contains(forbidden) {
            return Err(
                format!("retired FBX export surface returned: {forbidden}"),
            );
        }
    }
    Ok(())
}

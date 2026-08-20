// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Binary only contract test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Binary only contract test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Binary only contract test module.

use fbx as _;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

const DRIVEN_MODULE: &str = include_str!(
    "../../../src/formats/fbx/composition/adapter-outbound/mod.rs",
);
const BINARY_CHARACTER_WRITER: &str = include_str!(
    // jig-ignore-next-line: exact source contract path is indivisible
    "../../../src/formats/fbx/composition/adapter-outbound/binary_character_writer.rs",
);
const PIPELINE_EXPORT: &str = include_str!(
    // jig-ignore-next-line: exact source contract path is indivisible
    "../../../src/migration/pipeline/composition/adapter-outbound/local/fbx_export.rs",
);
const PIPELINE_CLI: &str = include_str!(
    "../../../src/migration/pipeline/composition/adapter-inbound/cli.rs"
);
const PIPELINE_OPTIONS: &str = include_str!(
    "../../../src/migration/pipeline/composition/adapter-inbound/cli/options.rs"
);
const PIPELINE_PORTS: &str =
    include_str!("../../../src/migration/pipeline/port-outbound/mod.rs");
const SEMANTIC_TEXTURE_CLI: &str = include_str!(
    "../../../src/formats/fbx/composition/semantic_character_texture_cli.rs"
);
const SEMANTIC_TEXTURE_PACKAGE: &str = include_str!(
    // jig-ignore-next-line: exact syntax is indivisible
    "../../../src/formats/fbx/composition/adapter-outbound/semantic_character_texture/package.rs",
);
const SEMANTIC_TEXTURE_PUBLICATION: &str = include_str!(
    // jig-ignore-next-line: exact syntax is indivisible
    "../../../src/formats/fbx/composition/adapter-outbound/semantic_character_texture/publication.rs",
);

#[test]
fn exposes_only_binary_fbx_7700() -> Result<(), String> {
    let contract_sources = [
        DRIVEN_MODULE,
        BINARY_CHARACTER_WRITER,
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
            return Err(format!(
                "retired FBX export surface returned: {forbidden}"
            ));
        }
    }
    Ok(())
}

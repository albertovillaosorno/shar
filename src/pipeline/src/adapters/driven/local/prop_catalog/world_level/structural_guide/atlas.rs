// File:
//   - atlas.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     structural_guide/atlas.rs
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
//   - Source-SHA atlas discovery and deterministic opaque RGB variants.
// - Must-Not:
//   - Change source UVs, resize textures, rotate tiles, or serialize FBX.
// - Allows:
//   - SHA verification, alpha flattening, source-wide presentation averages,
//     swatches, per-polygon wrap assignments, and deterministic packing.
// - Summary:
//   - Builds one atlas tile per deduplicated source texture.
//
// LARGE-FILE:
// - owner: Structural-guide atlas
// - reason: Source verification, bounded presentation aggregation, source-SHA
//   deduplication, and layout projection form one atlas transaction.
// - split: Rectangle packing and RGB modulation live in child modules.
// - validation: Atlas packing, RGB PNG, source deduplication, and layout tests.
// - review: Split if non-diffuse source channels are introduced.

//! Structural-guide source-SHA RGB atlas construction.

use std::collections::BTreeMap;

use fbx::adapters::driven::semantic_texture_png::{
    decode_png_bytes, encode_rgb_png_bytes,
};
use fbx::domain::mesh::PrimitiveGroup;
use fbx::domain::texture::MaterialBinding;
use shar_sha256::digest_hex;

use super::super::export::MasterContent;
use super::model::{
    AtlasAssignment, AtlasBuild, AtlasLayout, AtlasLayoutEntry, SurfaceKey,
    VertexColorBake,
};
use crate::domain::PipelineError;

mod modulation;
mod packing;

pub(super) const ATLAS_SIZE: u32 = 4_096;
pub(super) const ATLAS_PADDING: u32 = 2;
const SWATCH_SIZE: u32 = 4;

/// One useful atlas tile before deterministic placement.
#[derive(Clone)]
pub(super) struct AtlasVariant {
    pub(super) source_name: String,
    pub(super) source_sha256: String,
    pub(super) variant_sha256: String,
    pub(super) presentation_bake: VertexColorBake,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) pixels: Vec<[u8; 3]>,
    pub(super) useful_x: u32,
    pub(super) useful_y: u32,
    pub(super) surface_keys: Vec<SurfaceKey>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct TintAggregate {
    sums: [u64; 4],
    samples: u64,
    first: Option<[u8; 4]>,
    approximated: bool,
}

impl TintAggregate {
    fn add(
        &mut self,
        color: [u8; 4],
        occurrences: u64,
        approximate: bool,
    ) -> Result<(), PipelineError> {
        if occurrences == 0 {
            return Ok(());
        }
        self.approximated |= approximate;
        self.approximated |= self
            .first
            .is_some_and(|first| first != color);
        self.first
            .get_or_insert(color);
        for (sum, channel) in self
            .sums
            .iter_mut()
            .zip(color)
        {
            *sum = sum
                .checked_add(u64::from(channel).saturating_mul(occurrences))
                .ok_or_else(
                    || PipelineError::new("atlas tint sum overflowed"),
                )?;
        }
        self.samples = self
            .samples
            .checked_add(occurrences)
            .ok_or_else(
                || PipelineError::new("atlas tint sample count overflowed"),
            )?;
        Ok(())
    }

    fn finish(self) -> Result<VertexColorBake, PipelineError> {
        if self.samples == 0 {
            return Err(PipelineError::new("atlas tint aggregate is empty"));
        }
        let rgba8 = self
            .sums
            .map(
                |sum| {
                    let rounded =
                        sum.saturating_add(self.samples / 2) / self.samples;
                    u8::try_from(rounded).unwrap_or(u8::MAX)
                },
            );
        Ok(
            if self.approximated {
                VertexColorBake::SourceAverage {
                    rgba8,
                    sample_count: self.samples,
                }
            } else {
                VertexColorBake::Uniform {
                    rgba8,
                    sample_count: self.samples,
                }
            },
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SourcePixels {
    name: String,
    sha256: String,
    width: u32,
    height: u32,
    pixels: Vec<[u8; 4]>,
}

struct SourceAggregate {
    source: SourcePixels,
    tint: TintAggregate,
    surfaces: Vec<SurfaceKey>,
}

/// Build the complete opaque atlas and every material/wrap assignment.
pub(super) fn build(
    content: &MasterContent
) -> Result<AtlasBuild, PipelineError> {
    let surfaces = discover_surfaces(content)?;
    let mut sources = BTreeMap::<String, SourceAggregate>::new();
    for (surface, vertex_bake) in surfaces {
        let material = content
            .materials
            .get(&surface.material)
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "structural-guide material binding is missing: {}",
                            surface.material
                        ),
                    )
                },
            )?;
        let source = source_pixels(
            content, material,
        )?;
        let presentation_tint = combine_tints(
            material.base_color_rgba8,
            vertex_bake.rgba8(),
        );
        let entry = sources
            .entry(
                source
                    .sha256
                    .clone(),
            )
            .or_insert_with(
                || SourceAggregate {
                    source: source.clone(),
                    tint: TintAggregate::default(),
                    surfaces: Vec::new(),
                },
            );
        validate_same_source(
            &entry.source,
            &source,
        )?;
        entry
            .tint
            .add(
                presentation_tint,
                vertex_bake.sample_count(),
                vertex_bake.is_approximate(),
            )?;
        entry
            .surfaces
            .push(surface);
    }
    let mut variants = sources
        .into_values()
        .map(build_variant)
        .collect::<Result<Vec<_>, _>>()?;
    let pixels = packing::pack(&mut variants)?;
    let png_bytes = encode_rgb_png_bytes(
        ATLAS_SIZE, ATLAS_SIZE, &pixels,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("structural-guide atlas PNG failed: {error:?}"),
            )
        },
    )?;
    let assignments = assignments(&variants)?;
    let mut entries = variants
        .into_iter()
        .map(
            |variant| AtlasLayoutEntry {
                source_name: variant.source_name,
                source_sha256: variant.source_sha256,
                variant_sha256: variant.variant_sha256,
                presentation_bake: variant.presentation_bake,
                x: variant.useful_x,
                y: variant.useful_y,
                width: variant.width,
                height: variant.height,
                wrap_mode: wrap_mode(&variant.surface_keys),
            },
        )
        .collect::<Vec<_>>();
    entries.sort_by(
        |left, right| {
            left.source_sha256
                .cmp(&right.source_sha256)
                .then_with(
                    || {
                        left.variant_sha256
                            .cmp(&right.variant_sha256)
                    },
                )
        },
    );
    Ok(
        AtlasBuild {
            png_bytes,
            layout: AtlasLayout {
                schema_version: 1,
                atlas_width: ATLAS_SIZE,
                atlas_height: ATLAS_SIZE,
                padding_pixels: ATLAS_PADDING,
                rotation_allowed: false,
                entries,
            },
            assignments,
        },
    )
}

/// Build one stable material/wrap key for a primitive group.
pub(super) fn surface_key(group: &PrimitiveGroup) -> SurfaceKey {
    SurfaceKey {
        material: group
            .shader
            .clone(),
        repeat: source_repeats(group),
    }
}

fn discover_surfaces(
    content: &MasterContent
) -> Result<BTreeMap<SurfaceKey, VertexColorBake>, PipelineError> {
    let mut surfaces = BTreeMap::<SurfaceKey, TintAggregate>::new();
    for mesh in &content.meshes {
        for group in &mesh.groups {
            let aggregate = surfaces
                .entry(surface_key(group))
                .or_default();
            if group
                .colors
                .is_empty()
            {
                aggregate.add(
                    [u8::MAX; 4],
                    u64::try_from(
                        group
                            .positions
                            .len(),
                    )
                    .map_err(|error| PipelineError::new(error.to_string()))?,
                    false,
                )?;
            } else {
                for color in &group.colors {
                    aggregate.add(
                        quantize_rgba(*color)?,
                        1,
                        false,
                    )?;
                }
            }
        }
    }
    if surfaces.is_empty() {
        return Err(
            PipelineError::new(
                "structural-guide atlas has no visible surfaces",
            ),
        );
    }
    surfaces
        .into_iter()
        .map(
            |(key, aggregate)| {
                Ok(
                    (
                        key,
                        aggregate.finish()?,
                    ),
                )
            },
        )
        .collect()
}

fn build_variant(
    source: SourceAggregate
) -> Result<AtlasVariant, PipelineError> {
    let presentation_bake = source
        .tint
        .finish()?;
    let pixels = modulation::bake(
        &source
            .source
            .pixels,
        source
            .source
            .width,
        source
            .source
            .height,
        [u8::MAX; 4],
        presentation_bake.rgba8(),
    )?;
    let mut identity = Vec::with_capacity(
        pixels
            .len()
            .saturating_mul(3)
            .saturating_add(160),
    );
    identity.extend_from_slice(
        source
            .source
            .sha256
            .as_bytes(),
    );
    presentation_bake.append_identity(&mut identity);
    identity.extend(
        pixels
            .iter()
            .flat_map(|pixel| pixel.iter())
            .copied(),
    );
    Ok(
        AtlasVariant {
            source_name: source
                .source
                .name,
            source_sha256: source
                .source
                .sha256,
            variant_sha256: digest_hex(&identity),
            presentation_bake,
            width: source
                .source
                .width,
            height: source
                .source
                .height,
            pixels,
            useful_x: 0,
            useful_y: 0,
            surface_keys: source.surfaces,
        },
    )
}

fn assignments(
    variants: &[AtlasVariant]
) -> Result<BTreeMap<SurfaceKey, AtlasAssignment>, PipelineError> {
    let mut assignments = BTreeMap::new();
    for variant in variants {
        let common = AtlasAssignment {
            offset: [
                atlas_component(variant.useful_x)? + atlas_half_texel(),
                atlas_component(variant.useful_y)? + atlas_half_texel(),
            ],
            scale: [
                atlas_component(
                    variant
                        .width
                        .saturating_sub(1),
                )?,
                atlas_component(
                    variant
                        .height
                        .saturating_sub(1),
                )?,
            ],
            repeat: 0.0,
            approximated_vertex_color: variant
                .presentation_bake
                .is_approximate(),
        };
        for key in &variant.surface_keys {
            let assignment = AtlasAssignment {
                repeat: f32::from(u8::from(key.repeat)),
                ..common
            };
            if assignments
                .insert(
                    key.clone(),
                    assignment,
                )
                .is_some()
            {
                return Err(
                    PipelineError::new(
                        "structural-guide atlas surface assignment repeats",
                    ),
                );
            }
        }
    }
    Ok(assignments)
}

fn source_pixels(
    content: &MasterContent,
    material: &MaterialBinding,
) -> Result<SourcePixels, PipelineError> {
    if let Some(file_name) = material
        .texture_file_name
        .as_deref()
    {
        let texture = content
            .textures
            .get(file_name)
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "structural-guide texture payload is missing: \
                             {file_name}"
                        ),
                    )
                },
            )?;
        let actual_sha256 = digest_hex(&texture.bytes);
        if actual_sha256 != texture.sha256 {
            return Err(
                PipelineError::new(
                    format!(
                        "structural-guide texture hash changed: {file_name}"
                    ),
                ),
            );
        }
        let image = decode_png_bytes(&texture.bytes).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "structural-guide texture decode failed: \
                         {file_name}:{error:?}"
                    ),
                )
            },
        )?;
        return Ok(
            SourcePixels {
                name: file_name.to_owned(),
                sha256: texture
                    .sha256
                    .clone(),
                width: image.width(),
                height: image.height(),
                pixels: image
                    .pixels()
                    .iter()
                    .map(|pixel| pixel.channels())
                    .collect(),
            },
        );
    }
    let color = material.base_color_rgba8;
    let source_identity = [
        b"swatch:".as_slice(),
        color.as_slice(),
    ]
    .concat();
    Ok(
        SourcePixels {
            name: format!(
                "swatch-{:02x}{:02x}{:02x}{:02x}.rgb",
                color[0], color[1], color[2], color[3]
            ),
            sha256: digest_hex(&source_identity),
            width: SWATCH_SIZE,
            height: SWATCH_SIZE,
            pixels: vec![[u8::MAX; 4]; 16],
        },
    )
}

fn validate_same_source(
    canonical: &SourcePixels,
    candidate: &SourcePixels,
) -> Result<(), PipelineError> {
    if canonical.width != candidate.width
        || canonical.height != candidate.height
        || canonical.pixels != candidate.pixels
    {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide source SHA maps to different pixels: {}",
                    canonical.sha256
                ),
            ),
        );
    }
    Ok(())
}

fn combine_tints(
    material: [u8; 4],
    vertex: [u8; 4],
) -> [u8; 4] {
    [
        multiply_channel(
            material[0],
            vertex[0],
        ),
        multiply_channel(
            material[1],
            vertex[1],
        ),
        multiply_channel(
            material[2],
            vertex[2],
        ),
        multiply_channel(
            material[3],
            vertex[3],
        ),
    ]
}

fn multiply_channel(
    left: u8,
    right: u8,
) -> u8 {
    let product = u16::from(left).saturating_mul(u16::from(right));
    u8::try_from((product + 127) / 255).unwrap_or(u8::MAX)
}

fn wrap_mode(surfaces: &[SurfaceKey]) -> &'static str {
    let repeat = surfaces
        .iter()
        .any(|surface| surface.repeat);
    let clamp = surfaces
        .iter()
        .any(|surface| !surface.repeat);
    match (
        repeat, clamp,
    ) {
        (true, true) => "perPolygon",
        (true, false) => "repeat",
        (false, true) => "clamp",
        (false, false) => "clamp",
    }
}

fn atlas_component(value: u32) -> Result<f32, PipelineError> {
    let bounded = u16::try_from(value)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    Ok(f32::from(bounded) / f32::from(4_096_u16))
}

fn atlas_half_texel() -> f32 {
    0.5 / f32::from(4_096_u16)
}

fn source_repeats(group: &PrimitiveGroup) -> bool {
    group
        .uvs
        .iter()
        .flatten()
        .any(|value| *value < -0.001 || *value > 1.001)
}

fn quantize_rgba(value: [f32; 4]) -> Result<[u8; 4], PipelineError> {
    Ok(
        [
            quantize_color(value[0])?,
            quantize_color(value[1])?,
            quantize_color(value[2])?,
            quantize_color(value[3])?,
        ],
    )
}

#[expect(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Validated zero-through-one colors are rounded into exact RGBA8 \
              storage."
)]
fn quantize_color(value: f32) -> Result<u8, PipelineError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(
            PipelineError::new(
                "structural-guide color is outside zero through one",
            ),
        );
    }
    Ok((value * 255.0).round() as u8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_average_combines_different_presentations() {
        let mut aggregate = TintAggregate::default();
        assert!(
            aggregate
                .add(
                    [
                        255, 0, 0, 255
                    ],
                    1,
                    false
                )
                .is_ok()
        );
        assert!(
            aggregate
                .add(
                    [
                        0, 0, 255, 255
                    ],
                    1,
                    false
                )
                .is_ok()
        );
        assert_eq!(
            aggregate.finish(),
            Ok(
                VertexColorBake::SourceAverage {
                    rgba8: [
                        128, 0, 128, 255
                    ],
                    sample_count: 2,
                }
            )
        );
    }

    #[test]
    fn one_exact_source_presentation_remains_uniform() {
        let mut aggregate = TintAggregate::default();
        assert!(
            aggregate
                .add(
                    [
                        64, 96, 128, 255
                    ],
                    3,
                    false
                )
                .is_ok()
        );
        assert_eq!(
            aggregate.finish(),
            Ok(
                VertexColorBake::Uniform {
                    rgba8: [
                        64, 96, 128, 255
                    ],
                    sample_count: 3,
                }
            )
        );
    }

    #[test]
    fn per_polygon_wrap_is_preserved_without_duplicate_tiles() {
        let surfaces = vec![
            SurfaceKey {
                material: "repeat".to_owned(),
                repeat: true,
            },
            SurfaceKey {
                material: "clamp".to_owned(),
                repeat: false,
            },
        ];
        assert_eq!(
            wrap_mode(&surfaces),
            "perPolygon"
        );
    }

    #[test]
    fn material_and_vertex_tints_are_combined_before_source_average() {
        assert_eq!(
            combine_tints(
                [
                    128, 255, 64, 255
                ],
                [
                    128, 64, 255, 128
                ]
            ),
            [
                64, 64, 64, 128
            ]
        );
    }
}

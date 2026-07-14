// File:
//   - scrooby.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/stragglers/scrooby.rs
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
//   - The scrooby contract for pipeline phase two stragglers.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute scrooby.
// - Split-When:
//   - Split when scrooby contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Scrooby for pipeline phase two stragglers.
// - Description:
//   - Defines scrooby data and behavior for pipeline phase two stragglers.
// - Usage:
//   - Used by pipeline phase two stragglers code that needs scrooby.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Scrooby for pipeline phase two stragglers.
//!
//! This boundary keeps scrooby for pipeline phase two stragglers explicit and
//! returns deterministic results to pipeline callers.
use std::collections::BTreeMap;

use super::json::{JsonObject, json_string};

/// Append summary.
pub(super) fn append_summary(
    json: &mut JsonObject,
    text: &str,
) {
    let elements = xml_elements(text);
    let mut counts = BTreeMap::<String, usize>::new();
    let mut root = String::from("none");
    let mut p3d_refs = 0usize;
    let mut text_bible_refs = 0usize;
    let mut tag_sequence = Vec::<String>::new();
    let mut quoted_values = Vec::<String>::new();

    for element in &elements {
        if root == "none" {
            root.clone_from(&element.tag);
        }
        tag_sequence.push(
            element
                .tag
                .clone(),
        );
        if element
            .tag
            .contains("pure3d")
        {
            p3d_refs = p3d_refs.saturating_add(1);
        }
        if element
            .tag
            .contains("textbible")
        {
            text_bible_refs = text_bible_refs.saturating_add(1);
        }
        quoted_values.extend(
            element
                .attributes
                .values()
                .cloned(),
        );
        let count = counts
            .entry(
                element
                    .tag
                    .clone(),
            )
            .or_insert(0);
        *count = count.saturating_add(1);
    }

    json.field(
        "root_tag", &root,
    );
    json.number(
        "element_count",
        u64::try_from(elements.len()).unwrap_or(u64::MAX),
    );
    json.number(
        "pure3d_reference_tag_count",
        u64::try_from(p3d_refs).unwrap_or(u64::MAX),
    );
    json.number(
        "text_bible_reference_tag_count",
        u64::try_from(text_bible_refs).unwrap_or(u64::MAX),
    );
    json.map(
        "tag_counts",
        &counts,
    );
    json.string_array(
        "tag_sequence",
        &tag_sequence,
    );
    json.string_array(
        "quoted_values",
        &quoted_values,
    );
    json.raw_json(
        "xml_elements",
        &xml_elements_json(&elements),
    );
}

/// Schema for.
pub(super) fn schema_for(ext: &str) -> &'static str {
    match ext {
        "pag" => "shar-schoenwald.straggler.scrooby-page.v1",
        "scr" => "shar-schoenwald.straggler.scrooby-screen.v1",
        "prj" => "shar-schoenwald.straggler.scrooby-project.v1",
        _ => "shar-schoenwald.straggler.scrooby.v1",
    }
}

/// Xmlelement.
struct XmlElement {
    /// Ordinal.
    ordinal: usize,
    /// Tag.
    tag: String,
    /// Attributes.
    attributes: BTreeMap<String, String>,
}

/// Xml elements.
fn xml_elements(text: &str) -> Vec<XmlElement> {
    let mut elements = Vec::new();
    let mut cursor = text;
    while let Some(start) = cursor.find('<') {
        cursor = cursor
            .get(start.saturating_add(1)..)
            .unwrap_or_default();
        if cursor.starts_with('/')
            || cursor.starts_with('?')
            || cursor.starts_with('!')
        {
            continue;
        }
        let tag = cursor
            .chars()
            .take_while(
                |character| {
                    character.is_ascii_alphanumeric()
                        || *character == '_'
                        || *character == '-'
                },
            )
            .collect::<String>()
            .to_ascii_lowercase();
        if tag.is_empty() {
            continue;
        }
        let element_body = cursor
            .split('>')
            .next()
            .unwrap_or_default();
        let ordinal = elements
            .len()
            .saturating_add(1);
        elements.push(
            XmlElement {
                ordinal,
                tag,
                attributes: attributes_from_start_tag(element_body),
            },
        );
    }
    elements
}

/// Attributes from start tag.
fn attributes_from_start_tag(value: &str) -> BTreeMap<String, String> {
    let mut attrs = BTreeMap::new();
    let mut parts = value.split_whitespace();
    let _ = parts.next();
    for part in parts {
        if let Some((name, raw_value)) = part.split_once('=') {
            let without_closing = raw_value.trim_end_matches('/');
            let clean = without_closing
                .trim_matches('"')
                .to_owned();
            if !name.is_empty() && !clean.is_empty() {
                drop(
                    attrs.insert(
                        name.to_ascii_lowercase(),
                        clean,
                    ),
                );
            }
        }
    }
    attrs
}

/// Xml elements json.
fn xml_elements_json(elements: &[XmlElement]) -> String {
    let mut out = String::from("[");
    for (index, element) in elements
        .iter()
        .enumerate()
    {
        if index > 0 {
            out.push(',');
        }
        out.push('{');
        out.push_str("\"ordinal\":");
        out.push_str(
            &element
                .ordinal
                .to_string(),
        );
        out.push_str(",\"tag\":");
        out.push_str(&json_string(&element.tag));
        out.push_str(",\"attributes\":{");
        for (attr_index, (key, value)) in element
            .attributes
            .iter()
            .enumerate()
        {
            if attr_index > 0 {
                out.push(',');
            }
            out.push_str(&json_string(key));
            out.push(':');
            out.push_str(&json_string(value));
        }
        out.push_str("}}");
    }
    out.push(']');
    out
}

#[cfg(test)]
mod tests {
    use super::attributes_from_start_tag;

    #[test]
    fn self_closing_attributes_drop_syntax_delimiters() {
        let attributes = attributes_from_start_tag("widget name=\"value\"/");
        assert_eq!(
            attributes
                .get("name")
                .map(String::as_str),
            Some("value")
        );
    }
}

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
//   - Exact reviewed structural adaptations for legacy mission scripts.
// - Must-Not:
//   - Match by filename alone or infer gameplay semantics.
// - Allows:
//   - Adapt one logical path only when its complete command window matches.
// - Split-When:
//   - Split when another compatibility family has independent evidence.
// - Merge-When:
//   - Merge when no reviewed structural adaptations remain.
// - Summary:
//   - Legacy mission context compatibility registry.
// - Description:
//   - Keeps source defects visible while yielding a deterministic context
//     graph.
// - Usage:
//   - Consulted only by mission-script context validation.
// - Defaults:
//   - Any path, ordinal, command, or argument drift disables adaptation.
//

//! Reviewed legacy mission context adaptations.

use std::path::Path;

use super::super::super::json::json_string;
use super::super::CommandInvocation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in super::super) struct ContextAdaptation {
    pub(super) ordinal: usize,
    pub(super) command: &'static str,
    pub(super) code: &'static str,
}

const L2_M6SDI: &str = "scripts/missions/level02/m6sdi.mfk";
const L7_M7I: &str = "scripts/missions/level07/m7i.mfk";

const IGNORE_ORPHAN_CONDITION_CLOSE: &str =
    "legacy-l2-m6sdi-ignore-orphan-condition-close-v1";
const CLOSE_CONDITION_BEFORE_STAGE_COMPLETE: &str =
    "legacy-l7-m7i-close-keepbarrel-before-stage-complete-v1";

pub(super) fn reviewed_adaptations(
    relative: &Path,
    invocations: &[CommandInvocation],
) -> Vec<ContextAdaptation> {
    let logical = relative.to_string_lossy().replace('\\', "/");
    let mut adaptations = Vec::new();
    if logical == L2_M6SDI && matches_l2_m6sdi_window(invocations) {
        adaptations.push(ContextAdaptation {
            ordinal: 70,
            command: "closecondition",
            code: IGNORE_ORPHAN_CONDITION_CLOSE,
        });
    }
    if logical == L7_M7I && matches_l7_m7i_window(invocations) {
        adaptations.push(ContextAdaptation {
            ordinal: 114,
            command: "showstagecomplete",
            code: CLOSE_CONDITION_BEFORE_STAGE_COMPLETE,
        });
    }
    adaptations
}

pub(super) fn ignores_orphan_condition_close(
    adaptations: &[ContextAdaptation],
    ordinal: usize,
) -> bool {
    has_adaptation(
        adaptations,
        ordinal,
        "closecondition",
        IGNORE_ORPHAN_CONDITION_CLOSE,
    )
}

pub(super) fn closes_condition_before_stage_complete(
    adaptations: &[ContextAdaptation],
    ordinal: usize,
) -> bool {
    has_adaptation(
        adaptations,
        ordinal,
        "showstagecomplete",
        CLOSE_CONDITION_BEFORE_STAGE_COMPLETE,
    )
}

pub(super) fn adaptations_json(adaptations: &[ContextAdaptation]) -> String {
    let mut out = String::from("[");
    for (index, adaptation) in adaptations.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str("{\"ordinal\":");
        out.push_str(&adaptation.ordinal.to_string());
        out.push_str(",\"command\":");
        out.push_str(&json_string(adaptation.command));
        out.push_str(",\"code\":");
        out.push_str(&json_string(adaptation.code));
        out.push('}');
    }
    out.push(']');
    out
}

fn has_adaptation(
    adaptations: &[ContextAdaptation],
    ordinal: usize,
    command: &str,
    code: &str,
) -> bool {
    adaptations.iter().any(|adaptation| {
        adaptation.ordinal == ordinal
            && adaptation.command == command
            && adaptation.code == code
    })
}

fn matches_l2_m6sdi_window(invocations: &[CommandInvocation]) -> bool {
    matches_invocation(invocations, 1, "selectmission", &["m6sd"])
        && matches_invocation(invocations, 68, "addstagemusicchange", &[])
        && matches_invocation(invocations, 69, "setstagemusicalwayson", &[])
        && matches_invocation(invocations, 70, "closecondition", &[])
        && matches_invocation(invocations, 71, "closestage", &[])
}

fn matches_l7_m7i_window(invocations: &[CommandInvocation]) -> bool {
    matches_invocation(invocations, 112, "stagestartmusicevent", &["L7_drama"])
        && matches_invocation(invocations, 113, "addcondition", &[
            "keepbarrel",
            "2",
        ])
        && matches_invocation(invocations, 114, "showstagecomplete", &[])
        && matches_invocation(invocations, 115, "closestage", &[])
}

fn matches_invocation(
    invocations: &[CommandInvocation],
    ordinal: usize,
    name: &str,
    arguments: &[&str],
) -> bool {
    invocations.iter().any(|invocation| {
        invocation.ordinal == ordinal
            && invocation.name == name
            && invocation.arguments.len() == arguments.len()
            && invocation
                .arguments
                .iter()
                .zip(arguments.iter())
                .all(|(actual, expected)| actual == expected)
    })
}

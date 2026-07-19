// File:
//   - codec.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry/model/codec.rs
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
//   - Stable JSON parsing and serialization for active-run records.
// - Must-Not:
//   - Read files, estimate progress, or render terminal diagnostics.
// - Allows:
//   - Translate the typed state model to and from derived JSON bytes.
// - Summary:
//   - Active-run record codec.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Stable JSON codec for active pipeline run records.

use serde_json::{Value, json};

use super::{RunMode, RunSnapshot, RunState, SCHEMA};

impl RunSnapshot {
    /// Parse one persisted active-run JSON payload.
    pub(in super::super) fn parse(bytes: &[u8]) -> Result<Self, String> {
        let value = serde_json::from_slice::<Value>(bytes)
            .map_err(|error| format!("active-run JSON is invalid: {error}"))?;
        let schema = string_field(
            &value, "schema",
        )?;
        if schema != SCHEMA {
            return Err(format!("unsupported active-run schema: {schema}"));
        }
        let mode_text = string_field(
            &value, "mode",
        )?;
        let mode = RunMode::parse(mode_text).ok_or_else(
            || format!("unsupported active-run mode: {mode_text}"),
        )?;
        let state_text = string_field(
            &value, "state",
        )?;
        let state = RunState::parse(state_text).ok_or_else(
            || format!("unsupported active-run state: {state_text}"),
        )?;
        Ok(
            Self {
                run_id: string_field(
                    &value, "run_id",
                )?
                .to_owned(),
                pid: u32::try_from(
                    integer_field(
                        &value, "pid",
                    )?,
                )
                .map_err(
                    |error| format!("active-run PID is invalid: {error}"),
                )?,
                command: string_field(
                    &value, "command",
                )?
                .to_owned(),
                label: optional_string_field(
                    &value, "label",
                )?,
                mode,
                state,
                started_unix_ms: integer_field(
                    &value,
                    "started_unix_ms",
                )?,
                heartbeat_unix_ms: integer_field(
                    &value,
                    "heartbeat_unix_ms",
                )?,
                stage_started_unix_ms: integer_field(
                    &value,
                    "stage_started_unix_ms",
                )?,
                stage: string_field(
                    &value, "stage",
                )?
                .to_owned(),
                done: optional_integer_field(
                    &value, "done",
                )?,
                total: optional_integer_field(
                    &value, "total",
                )?,
                item: optional_string_field(
                    &value, "item",
                )?,
                eta_seconds: optional_integer_field(
                    &value,
                    "eta_seconds",
                )?,
            },
        )
    }

    /// Serialize one active-run state to stable pretty JSON bytes.
    pub(in super::super) fn json_bytes(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec_pretty(
            &json!({
                "schema": SCHEMA,
                "run_id": self.run_id,
                "pid": self.pid,
                "command": self.command,
                "label": self.label,
                "mode": self.mode.as_str(),
                "state": self.state.as_str(),
                "started_unix_ms": self.started_unix_ms,
                "heartbeat_unix_ms": self.heartbeat_unix_ms,
                "stage_started_unix_ms": self.stage_started_unix_ms,
                "stage": self.stage,
                "done": self.done,
                "total": self.total,
                "item": self.item,
                "eta_seconds": self.eta_seconds,
            }),
        )
        .map_err(|error| format!("active-run JSON encoding failed: {error}"))
    }
}

/// Return one required string field.
fn string_field<'value>(
    value: &'value Value,
    name: &str,
) -> Result<&'value str, String> {
    value
        .get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("active-run field {name} is missing or invalid"))
}

/// Return one optional string field.
fn optional_string_field(
    value: &Value,
    name: &str,
) -> Result<Option<String>, String> {
    let Some(field) = value.get(name) else {
        return Err(format!("active-run field {name} is missing"));
    };
    if field.is_null() {
        return Ok(None);
    }
    field
        .as_str()
        .map(|text| Some(text.to_owned()))
        .ok_or_else(|| format!("active-run field {name} is invalid"))
}

/// Return one required unsigned integer field.
fn integer_field(
    value: &Value,
    name: &str,
) -> Result<u64, String> {
    value
        .get(name)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("active-run field {name} is missing or invalid"))
}

/// Return one optional unsigned integer field.
fn optional_integer_field(
    value: &Value,
    name: &str,
) -> Result<Option<u64>, String> {
    let Some(field) = value.get(name) else {
        return Err(format!("active-run field {name} is missing"));
    };
    if field.is_null() {
        return Ok(None);
    }
    field
        .as_u64()
        .map(Some)
        .ok_or_else(|| format!("active-run field {name} is invalid"))
}

# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - The repository behavior implemented by this source file.
# - Must-Not:
#   - Bypass the contracts or authority boundaries of its owning package.
# - Allows:
#   - Inputs: values admitted by the file's public or internal interface.
#   - Outputs: deterministic values or effects declared by that interface.
#   - Side effects: only those explicitly owned by the implementation.
# - Split-When:
#   - Split when one responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another file owns the exact same responsibility.
# - Summary:
#   - Deterministic std-only Rust emission for protected exact transforms.
# - Description:
#   - Implements the responsibility summarized by this module.
# - Usage:
#   - Used through the owning package, executable, or document boundary.
# - Defaults:
#   - Invalid inputs or broken invariants fail closed.
#

"""Deterministic std-only Rust emission for protected exact transforms."""

from __future__ import annotations

import json
from pathlib import Path
from secrets import token_hex
from stat import S_ISDIR
from stat import S_ISLNK
from stat import S_ISREG
from typing import TYPE_CHECKING

from .protected import ProtectedExactPlan
from .protected import ProtectedMetadata
from .protected import protected_plan_aad

if TYPE_CHECKING:
    from .source_binding import ThresholdBinding

_RUNTIME_TEMPLATE = Path(__file__).with_name("rust_runtime.rs")
_DEFAULT_OUTPUT_PATH = Path("generated/main.rs")
_BEGIN = "// BEGIN GENERATED CONSTANTS"
_END = "// END GENERATED CONSTANTS"
_HEX_CHUNK = 64


class RustEmissionError(ValueError):
    """Raised when protected metadata cannot be emitted deterministically."""


def _u64(value: int) -> bytes:
    if value < 0 or value >= (1 << 64):
        message = "Rust transform integer exceeds unsigned 64-bit framing"
        raise RustEmissionError(message)
    return value.to_bytes(8, byteorder="big")


def _frame(value: bytes) -> bytes:
    return _u64(len(value)) + value


def _binding_bytes(binding: ThresholdBinding) -> bytes:
    parts = [
        _frame(binding.context),
        _u64(binding.threshold),
        _u64(binding.minimum_anchor_files),
        _u64(binding.secret_length),
        _frame(binding.secret_commitment),
        _u64(binding.anchor_policy.window_bytes),
        _u64(binding.anchor_policy.selection_modulus),
        _u64(len(binding.shares)),
    ]
    for share in binding.shares:
        parts.extend((
            _frame(share.source_path.encode("utf-8")),
            _frame(share.anchor_digest),
            _u64(share.x),
            _frame(share.masked_share),
        ))
    return b"".join(parts)


def _rust_string(value: str) -> str:
    return json.dumps(value, ensure_ascii=True)


def _hex_constant(name: str, data: bytes) -> str:
    encoded = data.hex()
    if not encoded:
        return f'const {name}: &str = "";'
    chunks = [
        encoded[offset : offset + _HEX_CHUNK]
        for offset in range(0, len(encoded), _HEX_CHUNK)
    ]
    if len(chunks) == 1:
        return f'const {name}: &str = concat!("{chunks[0]}",);'
    lines = [f"const {name}: &str = concat!("]
    lines.extend(f'    "{chunk}",' for chunk in chunks)
    lines.append(");")
    return "\n".join(lines)


def _constants(plan: ProtectedExactPlan, profile: str) -> str:
    metadata = ProtectedMetadata(
        source=plan.source,
        target=plan.target,
        instructions=plan.instructions,
        passthrough_roots=plan.passthrough_roots,
    )
    aad = protected_plan_aad(metadata, context=plan.context)
    return "\n".join((
        _BEGIN,
        f"const PROFILE: &str = {_rust_string(profile)};",
        _hex_constant("AAD_HEX", aad),
        _hex_constant("BINDING_HEX", _binding_bytes(plan.binding)),
        _hex_constant("NONCE_HEX", plan.nonce),
        _hex_constant("CIPHERTEXT_HEX", plan.payload.ciphertext),
        _hex_constant("TAG_HEX", plan.payload.tag),
        _END,
    ))



def _generated_header() -> str:
    return "\n".join((
        "// Copyright:",
        "//   - Copyright (c) 2026 Alberto Villa Osorno.",
        "// SPDX-License-Identifier:",
        "//   - MIT",
        "// Confidential:",
        "//   - false",
        "// License-File:",
        "//   - LICENSE-MIT",
        "//",
        "// Boundary-Contract:",
        "// - Owns:",
        "//   - The repository behavior implemented by this source file.",
        "// - Must-Not:",
        "//   - Bypass the owning boundary.",
        "// - Allows:",
        "//   - Inputs: declared source evidence.",
        "//   - Outputs: authenticated target bytes.",
        "//   - Side effects: transactional target publication.",
        "// - Split-When:",
        "//   - Split when one responsibility gains an independent lifecycle.",
        "// - Merge-When:",
        "//   - Merge when another file owns the exact same responsibility.",
        "// - Summary:",
        "//   - Standalone source-bound transform.",
        "// - Description:",
        "//   - Implements the responsibility summarized by this module.",
        "// - Usage:",
        "//   - Invoked with source and output roots.",
        "// - Defaults:",
        "//   - Invalid inputs or broken invariants fail closed.",
        "//",
        "",
        "//! Standalone source-bound exact transform.",
        "",
        "",
    ))

def _runtime_template_text() -> str:
    try:
        template = _RUNTIME_TEMPLATE.read_text(encoding="utf-8")
    except (OSError, UnicodeError) as error:
        message = (
            f"Rust runtime template read failed: {_RUNTIME_TEMPLATE}: {error}"
        )
        raise RustEmissionError(message) from error
    marker = "use std::collections::BTreeSet;"
    start = template.find(marker)
    if start < 0:
        raise RustEmissionError(
            "Rust runtime template lost implementation marker"
        )
    return template[start:]


def _validate_emission_inputs(
    plan: object, profile: object, output_path: object
) -> None:
    if type(plan) is not ProtectedExactPlan:
        message = (
            "Rust emission plan must use the exact ProtectedExactPlan type"
        )
        raise RustEmissionError(message)
    if type(profile) is not str or not profile:
        message = "Rust transform profile must use a non-empty exact string"
        raise RustEmissionError(message)
    if not isinstance(output_path, Path):
        message = "Rust transform output path must use pathlib Path"
        raise RustEmissionError(message)


def emit_rust_transform(
    plan: ProtectedExactPlan,
    profile: str,
    output_path: Path = _DEFAULT_OUTPUT_PATH,
) -> str:
    """Render one standalone Rust exact-transform source file.

    Returns:
        Deterministic UTF-8 Rust source with protected metadata embedded as hex.

    Raises:
        RustEmissionError: Profile/template state is invalid.

    """
    _validate_emission_inputs(plan, profile, output_path)
    template = _runtime_template_text()
    start = template.find(_BEGIN)
    end = template.find(_END)
    if start < 0 or end < start:
        message = "Rust runtime template lost generated-constant markers"
        raise RustEmissionError(message)
    end += len(_END)
    body = template[:start] + _constants(plan, profile) + template[end:]
    rendered = _generated_header() + body
    return rendered.replace("\r\n", "\n")


def _output_mode(path: Path, description: str) -> int | None:
    try:
        return path.lstat().st_mode
    except FileNotFoundError:
        return None
    except OSError as error:
        message = f"Rust output {description} status failed: {path}: {error}"
        raise RustEmissionError(message) from error


def _output_redirects(path: Path, mode: int) -> bool:
    return S_ISLNK(mode) or path.is_junction()


def _validate_output_leaf(output_path: Path) -> None:
    mode = _output_mode(output_path, "file")
    if mode is None:
        return
    if _output_redirects(output_path, mode) or not S_ISREG(mode):
        message = (
            f"Rust output path must be a regular non-linked file: {output_path}"
        )
        raise RustEmissionError(message)


def _validate_output_parent(path: Path, mode: int | None) -> None:
    if mode is None:
        return
    if _output_redirects(path, mode):
        message = f"Rust output parent must not be linked: {path}"
        raise RustEmissionError(message)
    if not S_ISDIR(mode):
        message = f"Rust output parent must be a directory: {path}"
        raise RustEmissionError(message)


def _validate_output_parent_chain(parent: Path) -> None:
    candidate = parent
    while True:
        _validate_output_parent(candidate, _output_mode(candidate, "parent"))
        ancestor = candidate.parent
        if ancestor == candidate:
            return
        candidate = ancestor


def _prepare_output_path(output_path: Path) -> Path:
    _validate_output_leaf(output_path)
    _validate_output_parent_chain(output_path.parent)
    try:
        output_path.parent.mkdir(parents=True, exist_ok=True)
    except OSError as error:
        message = (
            f"Rust output parent creation failed: {output_path.parent}: {error}"
        )
        raise RustEmissionError(message) from error
    _validate_output_parent_chain(output_path.parent)
    return output_path


def _temporary_output_path(output_path: Path) -> Path:
    return output_path.with_name(f".{output_path.name}.{token_hex(8)}.tmp")


def _cleanup_owned_temporary(path: Path) -> str | None:
    try:
        path.unlink()
    except FileNotFoundError:
        return None
    except OSError as error:
        return str(error)
    return None


def _claim_and_write_temporary(path: Path, source: str) -> None:
    owned = False
    try:
        with path.open("x", encoding="utf-8", newline="\n") as stream:
            owned = True
            _ = stream.write(source)
            stream.flush()
    except OSError:
        if owned:
            _ = _cleanup_owned_temporary(path)
        raise


def _raise_publication_error(
    error: OSError, *, cleanup_error: str | None = None
) -> None:
    message = f"Rust transform publication failed: {error}"
    if cleanup_error is not None:
        message = f"{message}; temporary cleanup failed: {cleanup_error}"
    raise RustEmissionError(message) from error


def write_rust_transform(
    plan: ProtectedExactPlan,
    profile: str,
    output_path: Path,
) -> None:
    """Write one generated transform atomically."""
    source = emit_rust_transform(plan, profile, output_path)
    prepared_output = _prepare_output_path(output_path)
    temporary = _temporary_output_path(prepared_output)
    try:
        _claim_and_write_temporary(temporary, source)
    except OSError as error:
        _raise_publication_error(error)
    try:
        _validate_output_leaf(prepared_output)
        _validate_output_parent_chain(prepared_output.parent)
        _ = temporary.replace(prepared_output)
    except OSError as error:
        cleanup_error = _cleanup_owned_temporary(temporary)
        _raise_publication_error(error, cleanup_error=cleanup_error)

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
#   - Decode the authenticated plan embedded in generated ``algorithm/main.rs``.
# - Must-Not:
#   - Cross declared architecture boundaries or persist undeclared dependencies.
# - Allows:
#   - Inputs: values admitted by this module interface.
#   - Outputs: deterministic values or effects declared by that interface.
#   - Side effects: only those explicitly owned by the implementation.
# - Split-When:
#   - Split when another responsibility gains an independent lifecycle.
# - Merge-When:
#   - Merge when another module owns the identical responsibility.
# - Summary:
#   - Decode the authenticated plan embedded in generated ``algorithm/main.rs``.
# - Description:
#   - Implements the declared responsibility for the Unreal icon pipeline.
# - Usage:
#   - Consumed through the owning icon function boundary.
# - Defaults:
#   - Invalid or missing inputs fail explicitly.
#

"""Decode the authenticated plan embedded in generated ``algorithm/main.rs``."""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import re

from .vendor.source_bound_diff.fingerprints import AnchorPolicy
from .vendor.source_bound_diff.model import (
    FileRecord,
    SourceSlice,
    TreeSnapshot,
)
from .vendor.source_bound_diff.payload import AuthenticatedPayload
from .vendor.source_bound_diff.protected import (
    PayloadSlice,
    ProtectedExactPlan,
    ProtectedInstruction,
    ProtectedInstructionKind,
)
from .vendor.source_bound_diff.source_binding import (
    BoundShare,
    ThresholdBinding,
)

_AAD_MAGIC = b"source-bound-exact-plan-aad-v2\0"


@dataclass(slots=True)
class _Cursor:
    data: bytes
    offset: int = 0

    def take(self, count: int) -> bytes:
        end = self.offset + count
        if count < 0 or end > len(self.data):
            raise ValueError("generated frame is truncated")
        value = self.data[self.offset:end]
        self.offset = end
        return value

    def byte(self) -> int:
        return self.take(1)[0]

    def u64(self) -> int:
        return int.from_bytes(self.take(8), "big")

    def frame(self) -> bytes:
        return self.take(self.u64())

    def text(self) -> str:
        return self.frame().decode("utf-8")

    def finish(self) -> None:
        if self.offset != len(self.data):
            raise ValueError("generated frame has trailing bytes")


def _rust_hex(source: str, name: str) -> bytes:
    pattern = rf"const\s+{re.escape(name)}:\s*&str\s*=\s*concat!\((.*?)\);"
    match = re.search(pattern, source, flags=re.DOTALL)
    if match is None:
        raise ValueError(f"generated transform is missing {name}")
    chunks = re.findall(r'"([0-9A-Fa-f]*)"', match.group(1))
    if not chunks:
        raise ValueError(f"generated transform has malformed {name}")
    return bytes.fromhex("".join(chunks))


def _snapshot(data: bytes) -> TreeSnapshot:
    cursor = _Cursor(data)
    files = tuple(
        FileRecord(
            path=cursor.text(),
            sha256=cursor.text(),
            size=cursor.u64(),
        )
        for _ in range(cursor.u64())
    )
    cursor.finish()
    return TreeSnapshot(files=files)


def _optional_text(cursor: _Cursor) -> str | None:
    tag = cursor.byte()
    if tag == 0:
        return None
    if tag == 1:
        return cursor.text()
    raise ValueError("invalid optional text tag")


def _optional_slice(cursor: _Cursor) -> PayloadSlice | None:
    tag = cursor.byte()
    if tag == 0:
        return None
    if tag == 1:
        return PayloadSlice(offset=cursor.u64(), length=cursor.u64())
    raise ValueError("invalid optional slice tag")


def _instruction(cursor: _Cursor) -> ProtectedInstruction:
    tags = {
        ord("C"): ProtectedInstructionKind.COPY_SOURCE,
        ord("P"): ProtectedInstructionKind.PATCH_SOURCE,
        ord("L"): ProtectedInstructionKind.PAYLOAD,
    }
    try:
        kind = tags[cursor.byte()]
    except KeyError as error:
        raise ValueError("invalid generated instruction kind") from error

    output_path = cursor.text()
    expected_sha256 = cursor.text()
    source_path = _optional_text(cursor)
    payload_slice = _optional_slice(cursor)
    segments = []
    for _ in range(cursor.u64()):
        tag = cursor.byte()
        offset = cursor.u64()
        length = cursor.u64()
        if tag == ord("S"):
            segments.append(SourceSlice(offset=offset, length=length))
        elif tag == ord("P"):
            segments.append(PayloadSlice(offset=offset, length=length))
        else:
            raise ValueError("invalid generated segment kind")

    return ProtectedInstruction(
        output_path=output_path,
        kind=kind,
        expected_sha256=expected_sha256,
        source_path=source_path,
        payload_slice=payload_slice,
        segments=tuple(segments),
    )


def _metadata(aad: bytes):
    if not aad.startswith(_AAD_MAGIC):
        raise ValueError("generated transform AAD magic mismatch")
    cursor = _Cursor(aad[len(_AAD_MAGIC):])
    context = cursor.frame()
    if not context:
        raise ValueError("generated transform context is empty")
    source = _snapshot(cursor.frame())
    target = _snapshot(cursor.frame())
    passthrough = tuple(cursor.text() for _ in range(cursor.u64()))
    instructions = tuple(_instruction(cursor) for _ in range(cursor.u64()))
    cursor.finish()
    return context, source, target, passthrough, instructions


def _binding(data: bytes) -> ThresholdBinding:
    cursor = _Cursor(data)
    context = cursor.frame()
    threshold = cursor.u64()
    minimum_anchor_files = cursor.u64()
    secret_length = cursor.u64()
    secret_commitment = cursor.frame()
    window_bytes = cursor.u64()
    selection_modulus = cursor.u64()
    shares = tuple(
        BoundShare(
            source_path=cursor.text(),
            anchor_digest=cursor.frame(),
            x=cursor.u64(),
            masked_share=cursor.frame(),
        )
        for _ in range(cursor.u64())
    )
    cursor.finish()
    return ThresholdBinding(
        context=context,
        threshold=threshold,
        minimum_anchor_files=minimum_anchor_files,
        secret_length=secret_length,
        secret_commitment=secret_commitment,
        anchor_policy=AnchorPolicy(
            window_bytes=window_bytes,
            selection_modulus=selection_modulus,
        ),
        shares=shares,
    )


def load_generated_plan(path: Path) -> ProtectedExactPlan:
    """Load the source-bound envelope from one generated Rust transform."""
    source_text = path.read_text(encoding="utf-8")
    aad = _rust_hex(source_text, "AAD_HEX")
    binding = _rust_hex(source_text, "BINDING_HEX")
    nonce = _rust_hex(source_text, "NONCE_HEX")
    ciphertext = _rust_hex(source_text, "CIPHERTEXT_HEX")
    tag = _rust_hex(source_text, "TAG_HEX")
    context, source, target, passthrough, instructions = _metadata(aad)
    return ProtectedExactPlan(
        source=source,
        target=target,
        instructions=instructions,
        passthrough_roots=passthrough,
        context=context,
        nonce=nonce,
        payload=AuthenticatedPayload(ciphertext=ciphertext, tag=tag),
        binding=_binding(binding),
    )

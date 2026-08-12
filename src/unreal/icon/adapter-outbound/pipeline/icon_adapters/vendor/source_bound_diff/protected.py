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
#   - Protected exact-baseline plans with source-bound authenticated literals.
# - Description:
#   - Implements the responsibility summarized by this module.
# - Usage:
#   - Used through the owning package, executable, or document boundary.
# - Defaults:
#   - Invalid inputs or broken invariants fail closed.
#

"""Protected exact-baseline plans with source-bound authenticated literals."""

from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
import hashlib
from typing import TYPE_CHECKING
from typing import cast

from .admission import IdentityTree
from .exact import materialize_exact_plan
from .exact import snapshot_tree_excluding
from .model import ExactAuthoringPlan
from .model import ExactInstruction
from .model import ExactInstructionKind
from .model import OracleLiteral
from .model import SourceSlice
from .model import TreeSnapshot
from .payload import AuthenticatedPayload
from .payload import chacha20_poly1305_decrypt
from .payload import chacha20_poly1305_encrypt
from .source_binding import SourceBindingError
from .source_binding import bind_secret
from .source_binding import hkdf_expand_sha256
from .source_binding import hkdf_extract_sha256
from .source_binding import recover_secret
from .source_binding import validate_threshold_binding

if TYPE_CHECKING:
    from pathlib import Path

    from .model import ExactSegment
    from .source_binding import SourceBindingPolicy
    from .source_binding import ThresholdBinding

_ZERO = 0
_ONE = 1
_PAYLOAD_KEY_BYTES = 32
_SINGLE_MESSAGE_NONCE = bytes(12)
_FRAME_BYTES = 8
_SHA256_HEX_LENGTH = 64
_HEX_DIGITS = frozenset("0123456789abcdef")
_AAD_MAGIC = b"source-bound-exact-plan-aad-v2\0"
_KEY_DOMAIN = b"source-bound-exact-plan-source-key-v1\0"
_BINDING_CONTEXT_DOMAIN = b"source-bound-exact-plan-binding-v1\0"


class ProtectedPlanError(ValueError):
    """Raised when protected-plan framing or payload references are invalid."""


class ProtectedInstructionKind(StrEnum):
    """How a protected exact output instruction obtains target bytes."""

    COPY_SOURCE = "copy-source"
    PATCH_SOURCE = "patch-source"
    PAYLOAD = "payload"


def _require_sha256_hex(value: object, context: str) -> None:
    if type(value) is not str or len(value) != _SHA256_HEX_LENGTH:
        message = f"{context} must be 64 lowercase hex digits"
        raise ProtectedPlanError(message)
    if any(char not in _HEX_DIGITS for char in value):
        message = f"{context} must be 64 lowercase hex digits"
        raise ProtectedPlanError(message)


def _validate_protected_segments(value: object) -> None:
    if type(value) is not tuple:
        message = "protected instruction segments must use an immutable tuple"
        raise ProtectedPlanError(message)
    segments = cast("tuple[object, ...]", value)
    if any(type(item) not in {SourceSlice, PayloadSlice} for item in segments):
        message = "protected instruction contains a foreign segment record"
        raise ProtectedPlanError(message)


@dataclass(frozen=True, slots=True)
class PayloadSlice:
    """A byte range inside the authenticated literal plaintext stream."""

    offset: int
    length: int

    def __post_init__(self) -> None:
        """Reject negative protected-payload ranges.

        Raises:
            ProtectedPlanError: Offset or length is negative.

        """
        if type(self.offset) is not int or type(self.length) is not int:
            message = "payload slice coordinates must use exact integers"
            raise ProtectedPlanError(message)
        if self.offset < _ZERO or self.length < _ZERO:
            message = "payload slices require non-negative offset and length"
            raise ProtectedPlanError(message)


ProtectedSegment = SourceSlice | PayloadSlice


@dataclass(frozen=True, slots=True)
class ProtectedInstruction:
    """One exact output instruction with no plaintext oracle bytes."""

    output_path: str
    kind: ProtectedInstructionKind
    expected_sha256: str
    source_path: str | None = None
    payload_slice: PayloadSlice | None = None
    segments: tuple[ProtectedSegment, ...] = ()

    def __post_init__(self) -> None:
        """Reject foreign metadata or fields inconsistent with the kind.

        Raises:
            ProtectedPlanError: Metadata types or kind-specific fields are
                invalid.

        """
        self._validate_metadata()
        if not self._shape_is_valid():
            message = f"invalid protected instruction fields for {self.kind!r}"
            raise ProtectedPlanError(message)

    def _validate_metadata(self) -> None:
        if type(self.output_path) is not str or not self.output_path:
            message = "protected instruction output path must be non-empty"
            raise ProtectedPlanError(message)
        if type(self.kind) is not ProtectedInstructionKind:
            message = "protected instruction kind must use the exact enum type"
            raise ProtectedPlanError(message)
        _require_sha256_hex(
            self.expected_sha256, "protected instruction sha256"
        )
        if self.source_path is not None and (
            type(self.source_path) is not str or not self.source_path
        ):
            message = (
                "protected instruction source path must be non-empty or None"
            )
            raise ProtectedPlanError(message)
        if (
            self.payload_slice is not None
            and type(self.payload_slice) is not PayloadSlice
        ):
            message = (
                "protected instruction payload slice must use the exact record"
            )
            raise ProtectedPlanError(message)
        _validate_protected_segments(self.segments)

    def _shape_is_valid(self) -> bool:
        if self.kind is ProtectedInstructionKind.COPY_SOURCE:
            return (
                self.source_path is not None
                and self.payload_slice is None
                and not self.segments
            )
        if self.kind is ProtectedInstructionKind.PATCH_SOURCE:
            return (
                self.source_path is not None
                and self.payload_slice is None
                and bool(self.segments)
            )
        return (
            self.kind is ProtectedInstructionKind.PAYLOAD
            and self.source_path is None
            and self.payload_slice is not None
            and not self.segments
        )


@dataclass(frozen=True, slots=True)
class ProtectedMetadata:
    """Authenticated static metadata for one protected exact plan."""

    source: TreeSnapshot
    target: TreeSnapshot
    instructions: tuple[ProtectedInstruction, ...]
    passthrough_roots: tuple[str, ...] = ()

    def __post_init__(self) -> None:
        """Require exact immutable authenticated metadata records.

        Raises:
            ProtectedPlanError: Metadata is mutable, foreign, or malformed.

        """
        if (
            type(self.source) is not TreeSnapshot
            or type(self.target) is not TreeSnapshot
        ):
            message = (
                "protected metadata snapshots must use exact TreeSnapshot "
                "records"
            )
            raise ProtectedPlanError(message)
        if type(self.instructions) is not tuple or any(
            type(item) is not ProtectedInstruction for item in self.instructions
        ):
            message = (
                "protected metadata instructions must be exact immutable "
                "records"
            )
            raise ProtectedPlanError(message)
        if type(self.passthrough_roots) is not tuple:
            message = "protected passthrough roots must use an immutable tuple"
            raise ProtectedPlanError(message)
        if any(
            type(root) is not str or not root for root in self.passthrough_roots
        ):
            message = "protected passthrough roots must be non-empty strings"
            raise ProtectedPlanError(message)


@dataclass(frozen=True, slots=True)
class ProtectedExactPlan:
    """Exact metadata with authenticated source-bound literals."""

    source: TreeSnapshot
    target: TreeSnapshot
    instructions: tuple[ProtectedInstruction, ...]
    passthrough_roots: tuple[str, ...]
    context: bytes
    nonce: bytes
    payload: AuthenticatedPayload
    binding: ThresholdBinding

    def __post_init__(self) -> None:
        """Require exact immutable protected-plan envelope types.

        Raises:
            ProtectedPlanError: Plan envelope metadata is foreign or malformed.

        """
        _ = ProtectedMetadata(
            source=self.source,
            target=self.target,
            instructions=self.instructions,
            passthrough_roots=self.passthrough_roots,
        )
        if type(self.context) is not bytes or not self.context:
            message = "protected plan context must use non-empty exact bytes"
            raise ProtectedPlanError(message)
        if type(self.nonce) is not bytes or len(self.nonce) != len(
            _SINGLE_MESSAGE_NONCE
        ):
            message = "protected plan nonce must use exact 12-byte input"
            raise ProtectedPlanError(message)
        if type(self.payload) is not AuthenticatedPayload:
            message = (
                "protected plan payload must use the exact authenticated type"
            )
            raise ProtectedPlanError(message)
        try:
            _ = validate_threshold_binding(self.binding)
        except SourceBindingError as error:
            message = f"protected plan binding is invalid: {error}"
            raise ProtectedPlanError(message) from error


@dataclass(slots=True)
class _PayloadBuilder:
    data: bytearray

    def append(self, literal: bytes) -> PayloadSlice:
        """Append literal bytes.

        Returns:
            Deterministic range of the appended bytes.

        """
        start = len(self.data)
        self.data.extend(literal)
        return PayloadSlice(offset=start, length=len(literal))


def _frame_bytes(value: bytes) -> bytes:
    return len(value).to_bytes(_FRAME_BYTES, byteorder="big") + value


def _frame_text(value: str) -> bytes:
    return _frame_bytes(value.encode("utf-8"))


def _u64(value: int) -> bytes:
    if value < _ZERO or value >= (1 << 64):
        message = "protected-plan integer exceeds unsigned 64-bit framing"
        raise ProtectedPlanError(message)
    return value.to_bytes(_FRAME_BYTES, byteorder="big")


def _snapshot_bytes(snapshot: TreeSnapshot) -> bytes:
    parts = [_u64(len(snapshot.files))]
    for record in snapshot.files:
        parts.extend((
            _frame_text(record.path),
            _frame_text(record.sha256),
            _u64(record.size),
        ))
    return b"".join(parts)


def _segment_bytes(segment: ProtectedSegment) -> bytes:
    if isinstance(segment, SourceSlice):
        return b"S" + _u64(segment.offset) + _u64(segment.length)
    return b"P" + _u64(segment.offset) + _u64(segment.length)


def _instruction_bytes(instruction: ProtectedInstruction) -> bytes:
    if instruction.kind is ProtectedInstructionKind.COPY_SOURCE:
        kind = b"C"
    elif instruction.kind is ProtectedInstructionKind.PATCH_SOURCE:
        kind = b"P"
    else:
        kind = b"L"
    source_path = (
        b"\x00"
        if instruction.source_path is None
        else b"\x01" + _frame_text(instruction.source_path)
    )
    payload_slice = (
        b"\x00"
        if instruction.payload_slice is None
        else (
            b"\x01"
            + _u64(instruction.payload_slice.offset)
            + _u64(instruction.payload_slice.length)
        )
    )
    segments = b"".join(_segment_bytes(item) for item in instruction.segments)
    return b"".join((
        kind,
        _frame_text(instruction.output_path),
        _frame_text(instruction.expected_sha256),
        source_path,
        payload_slice,
        _u64(len(instruction.segments)),
        segments,
    ))


def _roots_bytes(roots: tuple[str, ...]) -> bytes:
    return b"".join((_u64(len(roots)), *(_frame_text(root) for root in roots)))


def protected_plan_aad(
    metadata: ProtectedMetadata,
    *,
    context: bytes,
) -> bytes:
    """Serialize deterministic authenticated metadata for one protected plan.

    Returns:
        Stable binary AAD independent of ciphertext and source-binding shares.

    Raises:
        ProtectedPlanError: Context is empty.

    """
    if type(metadata) is not ProtectedMetadata:
        message = "protected metadata must use the exact ProtectedMetadata type"
        raise ProtectedPlanError(message)
    if type(context) is not bytes or not context:
        message = "protected-plan context must be non-empty exact bytes"
        raise ProtectedPlanError(message)
    instruction_bytes = b"".join(
        _instruction_bytes(instruction) for instruction in metadata.instructions
    )
    return b"".join((
        _AAD_MAGIC,
        _frame_bytes(context),
        _frame_bytes(_snapshot_bytes(metadata.source)),
        _frame_bytes(_snapshot_bytes(metadata.target)),
        _roots_bytes(metadata.passthrough_roots),
        _u64(len(metadata.instructions)),
        instruction_bytes,
    ))


def _metadata(
    source: TreeSnapshot,
    target: TreeSnapshot,
    instructions: tuple[ProtectedInstruction, ...],
    *,
    passthrough_roots: tuple[str, ...],
) -> ProtectedMetadata:
    return ProtectedMetadata(
        source=source,
        target=target,
        instructions=instructions,
        passthrough_roots=passthrough_roots,
    )


def _protected_segment(
    segment: ExactSegment,
    builder: _PayloadBuilder,
) -> ProtectedSegment:
    if isinstance(segment, SourceSlice):
        return segment
    return builder.append(segment.data)


def _protect_instruction(
    instruction: ExactInstruction,
    builder: _PayloadBuilder,
) -> ProtectedInstruction:
    if instruction.kind is ExactInstructionKind.COPY_SOURCE:
        return ProtectedInstruction(
            output_path=instruction.output_path,
            kind=ProtectedInstructionKind.COPY_SOURCE,
            expected_sha256=instruction.expected_sha256,
            source_path=instruction.source_path,
        )
    if instruction.kind is ExactInstructionKind.PATCH_SOURCE:
        return ProtectedInstruction(
            output_path=instruction.output_path,
            kind=ProtectedInstructionKind.PATCH_SOURCE,
            expected_sha256=instruction.expected_sha256,
            source_path=instruction.source_path,
            segments=tuple(
                _protected_segment(segment, builder)
                for segment in instruction.segments
            ),
        )
    if instruction.literal is None:
        message = "literal instruction lost authoring bytes before protection"
        raise ProtectedPlanError(message)
    return ProtectedInstruction(
        output_path=instruction.output_path,
        kind=ProtectedInstructionKind.PAYLOAD,
        expected_sha256=instruction.expected_sha256,
        payload_slice=builder.append(instruction.literal),
    )


def _source_identity_digest(reference_identity: IdentityTree) -> bytes:
    digest = hashlib.sha256()
    digest.update(b"source-bound-exact-plan-identity-v1\0")
    digest.update(_u64(len(reference_identity.files)))
    for identity_file in reference_identity.files:
        digest.update(_frame_text(identity_file.path))
        digest.update(_frame_bytes(identity_file.canonical))
    return digest.digest()


def _derive_payload_key(
    reference_identity: IdentityTree,
    *,
    plaintext: bytes,
    aad: bytes,
    context: bytes,
) -> bytes:
    source_material = _source_identity_digest(reference_identity)
    aad_digest = hashlib.sha256(aad).digest()
    plaintext_digest = hashlib.sha256(plaintext).digest()
    salt = hashlib.sha256(
        _KEY_DOMAIN + _frame_bytes(context) + _frame_bytes(aad_digest)
    ).digest()
    pseudorandom_key = hkdf_extract_sha256(salt, source_material)
    info = b"literal-payload-key-v2" + _frame_bytes(plaintext_digest)
    return hkdf_expand_sha256(
        pseudorandom_key,
        info,
        _PAYLOAD_KEY_BYTES,
    )


def _binding_context(aad: bytes, context: bytes) -> bytes:
    digest = hashlib.sha256()
    digest.update(_BINDING_CONTEXT_DOMAIN)
    digest.update(_frame_bytes(context))
    digest.update(_frame_bytes(hashlib.sha256(aad).digest()))
    return digest.digest()


def _validate_protect_inputs(
    plan: object, reference_identity: object, context: object
) -> None:
    if type(plan) is not ExactAuthoringPlan:
        message = (
            "protected authoring plan must use the exact ExactAuthoringPlan "
            "type"
        )
        raise ProtectedPlanError(message)
    if type(reference_identity) is not IdentityTree:
        message = "protected reference must use the exact IdentityTree type"
        raise ProtectedPlanError(message)
    if type(context) is not bytes or not context:
        message = "protected authoring context must use non-empty exact bytes"
        raise ProtectedPlanError(message)


def protect_exact_plan(
    plan: ExactAuthoringPlan,
    reference_identity: IdentityTree,
    *,
    binding_policy: SourceBindingPolicy,
    context: bytes,
) -> ProtectedExactPlan:
    """Protect all exact-plan oracle literals behind source-bound AEAD material.

    One plan derives one 256-bit payload key from canonical source evidence,
    authenticated metadata, and the literal-stream digest. The all-zero nonce
    is therefore never reused for a different plaintext under the same derived
    key inside this deterministic construction.

    Returns:
        Deterministic exact plan containing no plaintext oracle literals.

    """
    _validate_protect_inputs(plan, reference_identity, context)
    builder = _PayloadBuilder(bytearray())
    instructions = tuple(
        _protect_instruction(instruction, builder)
        for instruction in plan.instructions
    )
    plaintext = bytes(builder.data)
    metadata = _metadata(
        plan.source,
        plan.target,
        instructions,
        passthrough_roots=plan.passthrough_roots,
    )
    aad = protected_plan_aad(metadata, context=context)
    key = _derive_payload_key(
        reference_identity,
        plaintext=plaintext,
        aad=aad,
        context=context,
    )
    payload = chacha20_poly1305_encrypt(
        key,
        _SINGLE_MESSAGE_NONCE,
        plaintext,
        aad=aad,
    )
    binding = bind_secret(
        reference_identity,
        key,
        policy=binding_policy,
        context=_binding_context(aad, context),
    )
    return ProtectedExactPlan(
        source=plan.source,
        target=plan.target,
        instructions=instructions,
        passthrough_roots=plan.passthrough_roots,
        context=context,
        nonce=_SINGLE_MESSAGE_NONCE,
        payload=payload,
        binding=binding,
    )


def _payload_slice(plaintext: bytes, reference: PayloadSlice) -> bytes:
    end = reference.offset + reference.length
    if end > len(plaintext):
        message = "protected payload slice exceeds authenticated plaintext"
        raise ProtectedPlanError(message)
    return plaintext[reference.offset : end]


def _exact_segment(
    segment: ProtectedSegment,
    plaintext: bytes,
) -> ExactSegment:
    if isinstance(segment, SourceSlice):
        return segment
    return OracleLiteral(_payload_slice(plaintext, segment))


def _exact_instruction(
    instruction: ProtectedInstruction,
    plaintext: bytes,
) -> ExactInstruction:
    if instruction.kind is ProtectedInstructionKind.COPY_SOURCE:
        return ExactInstruction(
            output_path=instruction.output_path,
            kind=ExactInstructionKind.COPY_SOURCE,
            expected_sha256=instruction.expected_sha256,
            source_path=instruction.source_path,
        )
    if instruction.kind is ProtectedInstructionKind.PATCH_SOURCE:
        return ExactInstruction(
            output_path=instruction.output_path,
            kind=ExactInstructionKind.PATCH_SOURCE,
            expected_sha256=instruction.expected_sha256,
            source_path=instruction.source_path,
            segments=tuple(
                _exact_segment(segment, plaintext)
                for segment in instruction.segments
            ),
        )
    if instruction.payload_slice is None:
        message = "protected payload instruction lost its payload range"
        raise ProtectedPlanError(message)
    return ExactInstruction(
        output_path=instruction.output_path,
        kind=ExactInstructionKind.LITERAL_ORACLE,
        expected_sha256=instruction.expected_sha256,
        literal=_payload_slice(plaintext, instruction.payload_slice),
    )


def _require_protected_exact_plan(value: object) -> ProtectedExactPlan:
    if type(value) is not ProtectedExactPlan:
        message = "protected plan must use the exact ProtectedExactPlan type"
        raise ProtectedPlanError(message)
    return value


def recover_exact_plan(
    plan: ProtectedExactPlan,
    candidate_identity: IdentityTree,
) -> ExactAuthoringPlan:
    """Recover authenticated local literals only after source binding succeeds.

    Returns:
        In-memory exact plan suitable for transactional materialization.

    """
    plan = _require_protected_exact_plan(plan)
    metadata = _metadata(
        plan.source,
        plan.target,
        plan.instructions,
        passthrough_roots=plan.passthrough_roots,
    )
    aad = protected_plan_aad(metadata, context=plan.context)
    key = recover_secret(plan.binding, candidate_identity)
    plaintext = chacha20_poly1305_decrypt(
        key,
        plan.nonce,
        plan.payload,
        aad=aad,
    )
    instructions = tuple(
        _exact_instruction(instruction, plaintext)
        for instruction in plan.instructions
    )
    return ExactAuthoringPlan(
        source=plan.source,
        target=plan.target,
        instructions=instructions,
        passthrough_roots=plan.passthrough_roots,
    )


def materialize_protected_exact_plan(
    source_root: Path,
    candidate_identity: IdentityTree,
    *,
    plan: ProtectedExactPlan,
    output_root: Path,
) -> None:
    """Recover and authenticate literals before publishing exact output.

    Raises:
        ProtectedPlanError: The exact source snapshot changed.

    """
    plan = _require_protected_exact_plan(plan)
    if (
        snapshot_tree_excluding(source_root, plan.passthrough_roots)
        != plan.source
    ):
        message = "source tree does not match protected exact source snapshot"
        raise ProtectedPlanError(message)
    exact = recover_exact_plan(plan, candidate_identity)
    materialize_exact_plan(source_root, exact, output_root)

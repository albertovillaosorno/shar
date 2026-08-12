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
#   - Deterministic data model for authoring tree transformations.
# - Description:
#   - Implements the responsibility summarized by this module.
# - Usage:
#   - Used through the owning package, executable, or document boundary.
# - Defaults:
#   - Invalid inputs or broken invariants fail closed.
#

"""Deterministic data model for authoring tree transformations."""

from __future__ import annotations

from dataclasses import dataclass
from enum import StrEnum
from typing import cast

_ZERO = 0
_SHA256_HEX_LENGTH = 64
_HEX_DIGITS = frozenset("0123456789abcdef")


class TreeModelError(ValueError):
    """Raised when a tree or transformation violates the model contract."""


def _validate_sha256_hex(value: object, context: str) -> None:
    if type(value) is not str or len(value) != _SHA256_HEX_LENGTH:
        message = f"{context} must be 64 lowercase hex digits"
        raise TreeModelError(message)
    if any(char not in _HEX_DIGITS for char in value):
        message = f"{context} must be 64 lowercase hex digits"
        raise TreeModelError(message)


@dataclass(frozen=True, slots=True, order=True)
class FileRecord:
    """One regular file in a deterministic tree snapshot."""

    path: str
    sha256: str
    size: int

    def __post_init__(self) -> None:
        """Require exact immutable snapshot metadata types.

        Raises:
            TreeModelError: Snapshot metadata uses an invalid runtime value.

        """
        if type(self.path) is not str or not self.path:
            message = "file record path must be a non-empty string"
            raise TreeModelError(message)
        _validate_sha256_hex(self.sha256, "file record sha256")
        if type(self.size) is not int or self.size < _ZERO:
            message = "file record size must be a non-negative integer"
            raise TreeModelError(message)


@dataclass(frozen=True, slots=True)
class TreeSnapshot:
    """Sorted regular-file snapshot used for deterministic verification."""

    files: tuple[FileRecord, ...]

    def __post_init__(self) -> None:
        """Require immutable exact records in deterministic path order.

        Raises:
            TreeModelError: File records are mutable, foreign, or unordered.

        """
        if type(self.files) is not tuple:
            message = (
                "tree snapshot files must use the exact immutable tuple type"
            )
            raise TreeModelError(message)
        if any(type(item) is not FileRecord for item in self.files):
            message = "tree snapshot contains a foreign file record"
            raise TreeModelError(message)
        paths = tuple(item.path for item in self.files)
        if paths != tuple(sorted(set(paths))):
            message = "tree snapshot paths must be unique and sorted"
            raise TreeModelError(message)


class ExactInstructionKind(StrEnum):
    """How one exact-baseline output file obtains its bytes."""

    COPY_SOURCE = "copy-source"
    PATCH_SOURCE = "patch-source"
    LITERAL_ORACLE = "literal-oracle"


@dataclass(frozen=True, slots=True)
class SourceSlice:
    """A byte range reused from an admitted source file."""

    offset: int
    length: int

    def __post_init__(self) -> None:
        """Reject nonsensical source ranges.

        Raises:
            TreeModelError: The range is negative or empty.

        """
        if type(self.offset) is not int or type(self.length) is not int:
            message = "source slice coordinates must use exact integers"
            raise TreeModelError(message)
        if self.offset < _ZERO or self.length <= _ZERO:
            message = (
                "source slices require a non-negative offset and positive "
                "length"
            )
            raise TreeModelError(message)


@dataclass(frozen=True, slots=True)
class OracleLiteral:
    """Target-only bytes retained only inside the local authoring plan."""

    data: bytes

    def __post_init__(self) -> None:
        """Require exact immutable target bytes.

        Raises:
            TreeModelError: Target literal is not exact bytes.

        """
        if type(self.data) is not bytes:
            message = "oracle literal must use exact bytes"
            raise TreeModelError(message)


ExactSegment = SourceSlice | OracleLiteral


def _validate_exact_segments(value: object) -> None:
    if type(value) is not tuple:
        message = "exact instruction segments must use an immutable tuple"
        raise TreeModelError(message)
    segments = cast("tuple[object, ...]", value)
    if any(
        type(segment) not in {SourceSlice, OracleLiteral}
        for segment in segments
    ):
        message = "exact instruction contains a foreign segment record"
        raise TreeModelError(message)


def _validate_exact_instruction_metadata(instruction: ExactInstruction) -> None:
    if type(instruction.output_path) is not str or not instruction.output_path:
        message = "exact instruction output path must be a non-empty string"
        raise TreeModelError(message)
    if type(instruction.kind) is not ExactInstructionKind:
        message = "exact instruction kind must use the exact enum type"
        raise TreeModelError(message)
    _validate_sha256_hex(
        instruction.expected_sha256, "exact instruction sha256"
    )
    if instruction.source_path is not None and (
        type(instruction.source_path) is not str or not instruction.source_path
    ):
        message = "exact instruction source path must be non-empty or None"
        raise TreeModelError(message)
    if (
        instruction.literal is not None
        and type(instruction.literal) is not bytes
    ):
        message = "exact instruction literal must use exact bytes or None"
        raise TreeModelError(message)
    _validate_exact_segments(instruction.segments)


@dataclass(frozen=True, slots=True)
class ExactInstruction:
    """One output-file instruction in a local authoring plan.

    Literal oracle bytes are intentionally allowed here because this model is
    local authoring evidence, not a distributable transform. Public emission
    must source-bind such material before serialization.
    """

    output_path: str
    kind: ExactInstructionKind
    expected_sha256: str
    source_path: str | None = None
    literal: bytes | None = None
    segments: tuple[ExactSegment, ...] = ()

    def __post_init__(self) -> None:
        """Reject ambiguous or incomplete instruction payloads.

        Raises:
            TreeModelError: The instruction payload does not match its kind.

        """
        _validate_exact_instruction_metadata(self)
        if self.kind is ExactInstructionKind.COPY_SOURCE:
            self._validate_copy()
            return
        if self.kind is ExactInstructionKind.PATCH_SOURCE:
            self._validate_patch()
            return
        if self.kind is ExactInstructionKind.LITERAL_ORACLE:
            self._validate_literal()
            return
        message = f"unsupported exact instruction kind: {self.kind!r}"
        raise TreeModelError(message)

    def _validate_copy(self) -> None:
        if (
            self.source_path is None
            or self.literal is not None
            or self.segments
        ):
            message = "copy-source requires only source_path"
            raise TreeModelError(message)

    def _validate_patch(self) -> None:
        if (
            self.source_path is None
            or self.literal is not None
            or not self.segments
        ):
            message = "patch-source requires source_path and segments"
            raise TreeModelError(message)

    def _validate_literal(self) -> None:
        if (
            self.literal is None
            or self.source_path is not None
            or self.segments
        ):
            message = "literal-oracle requires only literal bytes"
            raise TreeModelError(message)


@dataclass(frozen=True, slots=True)
class ExactAuthoringPlan:
    """Local exact plan produced from a source tree and oracle tree."""

    source: TreeSnapshot
    target: TreeSnapshot
    instructions: tuple[ExactInstruction, ...]
    passthrough_roots: tuple[str, ...] = ()

    def __post_init__(self) -> None:
        """Require exact snapshots and immutable plan metadata.

        Raises:
            TreeModelError: Plan metadata is mutable, foreign, or malformed.

        """
        if (
            type(self.source) is not TreeSnapshot
            or type(self.target) is not TreeSnapshot
        ):
            message = "exact plan snapshots must use exact TreeSnapshot records"
            raise TreeModelError(message)
        if type(self.instructions) is not tuple or any(
            type(item) is not ExactInstruction for item in self.instructions
        ):
            message = "exact plan instructions must be exact immutable records"
            raise TreeModelError(message)
        if type(self.passthrough_roots) is not tuple:
            message = "exact plan passthrough roots must use an immutable tuple"
            raise TreeModelError(message)
        if any(
            type(root) is not str or not root for root in self.passthrough_roots
        ):
            message = "exact plan passthrough roots must be non-empty strings"
            raise TreeModelError(message)

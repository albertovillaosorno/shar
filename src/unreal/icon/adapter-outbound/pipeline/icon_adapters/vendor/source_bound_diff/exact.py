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
#   - Exact deterministic tree planning and materialization for authoring.
# - Description:
#   - Implements the responsibility summarized by this module.
# - Usage:
#   - Used through the owning package, executable, or document boundary.
# - Defaults:
#   - Invalid inputs or broken invariants fail closed.
#

"""Exact deterministic tree planning and materialization for authoring."""

from __future__ import annotations

from dataclasses import dataclass
import hashlib
from pathlib import PurePosixPath
import shutil
from stat import S_ISDIR
from stat import S_ISLNK
from stat import S_ISREG
from typing import TYPE_CHECKING

from .model import ExactAuthoringPlan
from .model import ExactInstruction
from .model import ExactInstructionKind
from .model import FileRecord
from .model import OracleLiteral
from .model import SourceSlice
from .model import TreeModelError
from .model import TreeSnapshot
from .publication import publish_directory_no_replace

if TYPE_CHECKING:
    from pathlib import Path

    from .model import ExactSegment

_BACKSLASH = "\\"
_UNSAFE_PATH_PARTS = frozenset({"", ".", ".."})
_STAGING_SUFFIX = ".staging"
_MATCH_BLOCK_BYTES = 32
_ZERO = 0


class ExactTreeError(RuntimeError):
    """Raised when exact planning or materialization cannot proceed safely."""


@dataclass(frozen=True, slots=True)
class _MatchContext:
    """Immutable byte-matching inputs shared across one file diff."""

    source: bytes
    target: bytes
    index: dict[bytes, int]


@dataclass(frozen=True, slots=True)
class _PlanningContext:
    """Immutable tree indexes shared across target-file planning."""

    source_root: Path
    oracle_root: Path
    source_by_path: dict[str, FileRecord]
    source_by_content: dict[tuple[str, int], tuple[str, ...]]


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _validate_relative_path(relative_path: str) -> str:
    """Validate and normalize one portable relative file path.

    Returns:
        The unchanged canonical POSIX-style relative path.

    Raises:
        TreeModelError: The path is empty, unsafe, or non-canonical.

    """
    if not relative_path or _BACKSLASH in relative_path:
        message = f"invalid relative path: {relative_path!r}"
        raise TreeModelError(message)
    candidate = PurePosixPath(relative_path)
    unsafe_part = any(part in _UNSAFE_PATH_PARTS for part in candidate.parts)
    if candidate.is_absolute() or unsafe_part:
        message = f"unsafe relative path: {relative_path!r}"
        raise TreeModelError(message)
    normalized = candidate.as_posix()
    if normalized != relative_path:
        message = (
            f"relative path is not canonically normalized: {relative_path!r}"
        )
        raise TreeModelError(message)
    return normalized


def _normalized_roots(roots: tuple[str, ...]) -> tuple[str, ...]:
    normalized = tuple(_validate_relative_path(root) for root in roots)
    if normalized != tuple(sorted(set(normalized))):
        message = "passthrough roots must be unique and sorted"
        raise ExactTreeError(message)
    for index, root in enumerate(normalized):
        prefix = root + "/"
        if any(other.startswith(prefix) for other in normalized[index + 1 :]):
            message = "passthrough roots must not overlap"
            raise ExactTreeError(message)
    return normalized


def _path_in_roots(path: str, roots: tuple[str, ...]) -> bool:
    return any(path == root or path.startswith(root + "/") for root in roots)


def _filter_snapshot(
    snapshot: TreeSnapshot,
    roots: tuple[str, ...],
    *,
    inside: bool,
) -> TreeSnapshot:
    return TreeSnapshot(
        files=tuple(
            record
            for record in snapshot.files
            if _path_in_roots(record.path, roots) is inside
        )
    )


def _raise_tree_walk_error(error: OSError) -> None:
    message = f"tree traversal failed: {error}"
    raise ExactTreeError(message) from error


def _tree_paths(root: Path) -> tuple[Path, ...]:
    paths: list[Path] = []
    for directory, directories, filenames in root.walk(
        on_error=_raise_tree_walk_error
    ):
        paths.extend(directory / name for name in directories)
        paths.extend(directory / name for name in filenames)
    return tuple(sorted(paths))


def _walk_relative_path(root: Path, path: Path) -> str:
    return path.relative_to(root).as_posix()


def _prune_excluded_directories(
    root: Path,
    directory: Path,
    directories: list[str],
    *,
    roots: tuple[str, ...],
) -> tuple[Path, ...]:
    admitted: list[Path] = []
    kept: list[str] = []
    for name in directories:
        path = directory / name
        if _path_in_roots(_walk_relative_path(root, path), roots):
            continue
        admitted.append(path)
        kept.append(name)
    directories[:] = kept
    return tuple(admitted)


def _admitted_files(
    root: Path,
    directory: Path,
    filenames: list[str],
    *,
    roots: tuple[str, ...],
) -> tuple[Path, ...]:
    paths = (directory / name for name in filenames)
    return tuple(
        path
        for path in paths
        if not _path_in_roots(_walk_relative_path(root, path), roots)
    )


def _tree_paths_excluding(
    root: Path, roots: tuple[str, ...]
) -> tuple[Path, ...]:
    paths: list[Path] = []
    for directory, directories, filenames in root.walk(
        top_down=True,
        on_error=_raise_tree_walk_error,
    ):
        paths.extend(
            _prune_excluded_directories(
                root, directory, directories, roots=roots
            )
        )
        paths.extend(_admitted_files(root, directory, filenames, roots=roots))
    return tuple(sorted(paths))


def _entry_mode(path: Path, context: str) -> int | None:
    try:
        return path.lstat().st_mode
    except FileNotFoundError:
        return None
    except OSError as error:
        message = f"{context} status failed: {error}"
        raise ExactTreeError(message) from error


def _is_redirect(path: Path, mode: int) -> bool:
    return S_ISLNK(mode) or path.is_junction()


def _read_file_bytes(path: Path, context: str) -> bytes:
    try:
        return path.read_bytes()
    except OSError as error:
        message = f"{context} read failed: {path}: {error}"
        raise ExactTreeError(message) from error


def _record_file(root: Path, path: Path) -> FileRecord | None:
    mode = _entry_mode(path, "tree entry")
    if mode is None:
        message = f"tree entry disappeared during traversal: {path}"
        raise ExactTreeError(message)
    if _is_redirect(path, mode):
        message = f"symlinks are not supported: {path}"
        raise ExactTreeError(message)
    if S_ISDIR(mode):
        return None
    if not S_ISREG(mode):
        message = f"special filesystem entry is not supported: {path}"
        raise ExactTreeError(message)
    relative = path.relative_to(root).as_posix()
    relative_path = _validate_relative_path(relative)
    data = _read_file_bytes(path, "tree entry")
    return FileRecord(path=relative_path, sha256=_sha256(data), size=len(data))


def _resolved_tree_root(root: Path) -> Path:
    mode = _entry_mode(root, "tree root")
    if mode is None:
        message = f"tree root is not a directory: {root}"
        raise ExactTreeError(message)
    if _is_redirect(root, mode):
        message = f"tree root must not be linked: {root}"
        raise ExactTreeError(message)
    if not S_ISDIR(mode):
        message = f"tree root is not a directory: {root}"
        raise ExactTreeError(message)
    try:
        return root.resolve(strict=True)
    except OSError as error:
        message = f"tree root resolution failed: {root}: {error}"
        raise ExactTreeError(message) from error


def snapshot_tree(root: Path) -> TreeSnapshot:
    """Snapshot regular files under ``root`` in stable path order.

    Symlinks and special filesystem objects fail closed. Empty directories are
    outside the version-one model because they carry no byte content.

    Returns:
        A path-sorted regular-file snapshot.

    """
    resolved_root = _resolved_tree_root(root)
    records = (
        record
        for path in _tree_paths(resolved_root)
        if (record := _record_file(resolved_root, path)) is not None
    )
    return TreeSnapshot(files=tuple(sorted(records)))


def snapshot_tree_excluding(
    root: Path,
    excluded_roots: tuple[str, ...],
) -> TreeSnapshot:
    """Snapshot one tree while excluding explicit root subtrees.

    Returns:
        Static snapshot containing no files under the excluded roots.

    """
    roots = _normalized_roots(excluded_roots)
    resolved_root = _resolved_tree_root(root)
    records = (
        record
        for path in _tree_paths_excluding(resolved_root, roots)
        if (record := _record_file(resolved_root, path)) is not None
    )
    return TreeSnapshot(files=tuple(sorted(records)))


def _source_paths_by_content(
    source: TreeSnapshot,
) -> dict[tuple[str, int], tuple[str, ...]]:
    paths: dict[tuple[str, int], list[str]] = {}
    for record in source.files:
        key = (record.sha256, record.size)
        paths.setdefault(key, []).append(record.path)
    return {key: tuple(sorted(value)) for key, value in paths.items()}


def _tree_bytes(root: Path, relative_path: str) -> bytes:
    relative = PurePosixPath(relative_path)
    path = root.joinpath(*relative.parts)
    return _read_file_bytes(path, "tree file")


def _oracle_bytes(oracle_root: Path, target_record: FileRecord) -> bytes:
    literal = _tree_bytes(oracle_root, target_record.path)
    if _sha256(literal) != target_record.sha256:
        message = f"oracle file changed while planning: {target_record.path}"
        raise ExactTreeError(message)
    return literal


def _block_index(source: bytes) -> dict[bytes, int]:
    last_start = len(source) - _MATCH_BLOCK_BYTES
    if last_start < _ZERO:
        return {}
    offsets = range(_ZERO, last_start + 1, _MATCH_BLOCK_BYTES)
    index: dict[bytes, int] = {}
    for offset in offsets:
        block = source[offset : offset + _MATCH_BLOCK_BYTES]
        _ = index.setdefault(block, offset)
    return index


def _extend_match_backward(
    context: _MatchContext,
    source_offset: int,
    target_offset: int,
    *,
    literal_start: int,
) -> tuple[int, int]:
    while (
        source_offset > _ZERO
        and target_offset > literal_start
        and context.source[source_offset - 1]
        == context.target[target_offset - 1]
    ):
        source_offset -= 1
        target_offset -= 1
    return source_offset, target_offset


def _extend_match_forward(
    context: _MatchContext,
    source_offset: int,
    target_offset: int,
) -> int:
    length = _MATCH_BLOCK_BYTES
    while (
        source_offset + length < len(context.source)
        and target_offset + length < len(context.target)
        and context.source[source_offset + length]
        == context.target[target_offset + length]
    ):
        length += 1
    return length


def _matching_slice(
    context: _MatchContext,
    target_offset: int,
    literal_start: int,
) -> tuple[int, int, int] | None:
    block_end = target_offset + _MATCH_BLOCK_BYTES
    if block_end > len(context.target):
        return None
    block = context.target[target_offset:block_end]
    source_offset = context.index.get(block)
    if source_offset is None:
        return None
    source_offset, target_offset = _extend_match_backward(
        context,
        source_offset,
        target_offset,
        literal_start=literal_start,
    )
    length = _extend_match_forward(context, source_offset, target_offset)
    return source_offset, target_offset, length


def _append_literal(
    segments: list[ExactSegment],
    target: bytes,
    *,
    start: int,
    end: int,
) -> None:
    if end > start:
        segments.append(OracleLiteral(target[start:end]))


def _build_patch_segments(
    source: bytes, target: bytes
) -> tuple[ExactSegment, ...]:
    context = _MatchContext(
        source=source,
        target=target,
        index=_block_index(source),
    )
    if not context.index:
        return (OracleLiteral(target),)
    segments: list[ExactSegment] = []
    literal_start = _ZERO
    cursor = _ZERO
    while cursor < len(target):
        match = _matching_slice(context, cursor, literal_start)
        if match is None:
            cursor += 1
            continue
        source_offset, target_offset, length = match
        _append_literal(
            segments,
            target,
            start=literal_start,
            end=target_offset,
        )
        segments.append(SourceSlice(offset=source_offset, length=length))
        cursor = target_offset + length
        literal_start = cursor
    _append_literal(
        segments,
        target,
        start=literal_start,
        end=len(target),
    )
    return tuple(segments)


def _contains_source_slice(segments: tuple[ExactSegment, ...]) -> bool:
    return any(isinstance(segment, SourceSlice) for segment in segments)


def _patch_instruction(
    source_root: Path,
    target_record: FileRecord,
    target_bytes: bytes,
) -> ExactInstruction:
    source_bytes = _tree_bytes(source_root, target_record.path)
    segments = _build_patch_segments(source_bytes, target_bytes)
    if _contains_source_slice(segments):
        return ExactInstruction(
            output_path=target_record.path,
            kind=ExactInstructionKind.PATCH_SOURCE,
            source_path=target_record.path,
            segments=segments,
            expected_sha256=target_record.sha256,
        )
    return ExactInstruction(
        output_path=target_record.path,
        kind=ExactInstructionKind.LITERAL_ORACLE,
        literal=target_bytes,
        expected_sha256=target_record.sha256,
    )


def _instruction_for_target(
    context: _PlanningContext,
    target_record: FileRecord,
) -> ExactInstruction:
    key = (target_record.sha256, target_record.size)
    candidates = context.source_by_content.get(key, ())
    if candidates:
        source_path = (
            target_record.path
            if target_record.path in candidates
            else candidates[0]
        )
        return ExactInstruction(
            output_path=target_record.path,
            kind=ExactInstructionKind.COPY_SOURCE,
            source_path=source_path,
            expected_sha256=target_record.sha256,
        )
    target_bytes = _oracle_bytes(context.oracle_root, target_record)
    if target_record.path in context.source_by_path:
        return _patch_instruction(
            context.source_root,
            target_record,
            target_bytes,
        )
    return ExactInstruction(
        output_path=target_record.path,
        kind=ExactInstructionKind.LITERAL_ORACLE,
        literal=target_bytes,
        expected_sha256=target_record.sha256,
    )


def build_exact_plan(
    source_root: Path,
    oracle_root: Path,
    *,
    passthrough_roots: tuple[str, ...] = (),
) -> ExactAuthoringPlan:
    """Build a deterministic exact-baseline plan from two local trees.

    Exact target bytes are represented by whole-file source copies, source
    slices plus local literals, or local literals when no source reuse exists.

    Returns:
        A deterministic, non-distributable authoring plan.

    Raises:
        ExactTreeError: Passthrough policy is invalid or differs at authoring.

    """
    roots = _normalized_roots(passthrough_roots)
    source_full = snapshot_tree(source_root)
    target_full = snapshot_tree(oracle_root)
    source_passthrough = _filter_snapshot(source_full, roots, inside=True)
    target_passthrough = _filter_snapshot(target_full, roots, inside=True)
    if source_passthrough != target_passthrough:
        message = "passthrough roots differ between source and authoring oracle"
        raise ExactTreeError(message)
    source = _filter_snapshot(source_full, roots, inside=False)
    target = _filter_snapshot(target_full, roots, inside=False)
    context = _PlanningContext(
        source_root=source_root,
        oracle_root=oracle_root,
        source_by_path={record.path: record for record in source.files},
        source_by_content=_source_paths_by_content(source),
    )
    instructions = tuple(
        _instruction_for_target(context, record) for record in target.files
    )
    return ExactAuthoringPlan(
        source=source,
        target=target,
        instructions=instructions,
        passthrough_roots=roots,
    )


def _safe_tree_path(root: Path, relative_path: str) -> Path:
    normalized = _validate_relative_path(relative_path)
    path = root.joinpath(*PurePosixPath(normalized).parts)
    try:
        resolved_path = path.resolve()
        resolved_root = root.resolve()
    except OSError as error:
        message = f"tree path resolution failed: {relative_path!r}: {error}"
        raise ExactTreeError(message) from error
    try:
        _ = resolved_path.relative_to(resolved_root)
    except ValueError as error:
        message = f"path escapes tree root: {relative_path!r}"
        raise ExactTreeError(message) from error
    return path


def _patch_bytes(source: bytes, segments: tuple[ExactSegment, ...]) -> bytes:
    parts: list[bytes] = []
    for segment in segments:
        if isinstance(segment, OracleLiteral):
            parts.append(segment.data)
            continue
        end = segment.offset + segment.length
        if end > len(source):
            message = "source slice exceeds source file"
            raise ExactTreeError(message)
        parts.append(source[segment.offset : end])
    return b"".join(parts)


def _instruction_bytes(
    source_root: Path,
    instruction: ExactInstruction,
) -> bytes:
    if instruction.kind is ExactInstructionKind.LITERAL_ORACLE:
        if instruction.literal is None:
            message = "literal-oracle instruction lost its literal bytes"
            raise ExactTreeError(message)
        return instruction.literal
    if instruction.source_path is None:
        message = "source-backed instruction lost its source path"
        raise ExactTreeError(message)
    source_path = _safe_tree_path(source_root, instruction.source_path)
    source = _read_file_bytes(source_path, "instruction source")
    if instruction.kind is ExactInstructionKind.COPY_SOURCE:
        return source
    return _patch_bytes(source, instruction.segments)


def _write_output_file(path: Path, data: bytes, context: str) -> None:
    try:
        path.parent.mkdir(parents=True, exist_ok=True)
        _ = path.write_bytes(data)
    except OSError as error:
        message = f"{context} write failed: {path}: {error}"
        raise ExactTreeError(message) from error


def _make_output_directory(
    path: Path,
    context: str,
    *,
    parents: bool,
    exist_ok: bool,
) -> None:
    try:
        path.mkdir(parents=parents, exist_ok=exist_ok)
    except OSError as error:
        message = f"{context} creation failed: {path}: {error}"
        raise ExactTreeError(message) from error


def _write_instruction(
    source_root: Path,
    staging_root: Path,
    instruction: ExactInstruction,
) -> None:
    data = _instruction_bytes(source_root, instruction)
    if _sha256(data) != instruction.expected_sha256:
        message = (
            "instruction bytes do not match expected hash for "
            f"{instruction.output_path}"
        )
        raise ExactTreeError(message)
    output_path = _safe_tree_path(staging_root, instruction.output_path)
    _write_output_file(output_path, data, "instruction output")


def _verify_source(source_root: Path, plan: ExactAuthoringPlan) -> None:
    current = snapshot_tree_excluding(source_root, plan.passthrough_roots)
    if current != plan.source:
        message = "source tree does not match exact authoring snapshot"
        raise ExactTreeError(message)


def _prepare_staging(output_root: Path) -> Path:
    if _entry_mode(output_root, "output root") is not None:
        message = f"output root already exists: {output_root}"
        raise ExactTreeError(message)
    staging_name = f".{output_root.name}{_STAGING_SUFFIX}"
    staging_root = output_root.with_name(staging_name)
    if _entry_mode(staging_root, "staging root") is not None:
        message = f"staging root already exists: {staging_root}"
        raise ExactTreeError(message)
    _make_output_directory(
        output_root.parent,
        "output parent",
        parents=True,
        exist_ok=True,
    )
    _make_output_directory(
        staging_root,
        "staging root",
        parents=False,
        exist_ok=False,
    )
    return staging_root


def _copy_passthrough_file(source: Path, target: Path) -> None:
    data = _read_file_bytes(source, "passthrough file")
    _write_output_file(target, data, "passthrough output")


def _copy_passthrough_entry(
    staging_root: Path, path: Path, relative: str
) -> None:
    mode = _entry_mode(path, "passthrough entry")
    if mode is None:
        message = f"passthrough entry disappeared: {relative}"
        raise ExactTreeError(message)
    if _is_redirect(path, mode):
        message = f"symlink is not supported in passthrough root: {relative}"
        raise ExactTreeError(message)
    output = _safe_tree_path(staging_root, relative)
    if S_ISDIR(mode):
        _make_output_directory(
            output,
            "passthrough directory",
            parents=True,
            exist_ok=True,
        )
    elif S_ISREG(mode):
        _copy_passthrough_file(path, output)
    else:
        message = f"special passthrough entry is not supported: {relative}"
        raise ExactTreeError(message)


def _copy_passthrough_directory(
    source_root: Path,
    staging_root: Path,
    source: Path,
) -> None:
    for path in _tree_paths(source):
        relative = path.relative_to(source_root).as_posix()
        _copy_passthrough_entry(staging_root, path, relative)


def _copy_passthrough_root(
    source_root: Path,
    staging_root: Path,
    relative_root: str,
) -> None:
    source = _safe_tree_path(source_root, relative_root)
    target = _safe_tree_path(staging_root, relative_root)
    mode = _entry_mode(source, "passthrough root")
    if mode is None:
        message = f"missing passthrough root: {relative_root}"
        raise ExactTreeError(message)
    if _is_redirect(source, mode):
        message = (
            f"symlink is not supported in passthrough root: {relative_root}"
        )
        raise ExactTreeError(message)
    if S_ISREG(mode):
        _copy_passthrough_file(source, target)
        return
    if S_ISDIR(mode):
        _make_output_directory(
            target,
            "passthrough root directory",
            parents=True,
            exist_ok=True,
        )
        _copy_passthrough_directory(source_root, staging_root, source)
        return
    message = f"special passthrough entry is not supported: {relative_root}"
    raise ExactTreeError(message)


def _copy_passthrough_roots(
    source_root: Path,
    staging_root: Path,
    roots: tuple[str, ...],
) -> None:
    for root in roots:
        _copy_passthrough_root(source_root, staging_root, root)


def _populate_staging(
    source_root: Path,
    staging_root: Path,
    plan: ExactAuthoringPlan,
) -> None:
    for instruction in plan.instructions:
        _write_instruction(source_root, staging_root, instruction)
    _copy_passthrough_roots(
        source_root,
        staging_root,
        plan.passthrough_roots,
    )
    static_output = _filter_snapshot(
        snapshot_tree(staging_root),
        plan.passthrough_roots,
        inside=False,
    )
    if static_output != plan.target:
        message = "materialized tree does not match target snapshot"
        raise ExactTreeError(message)


def _resolved_output_root(output_root: Path) -> Path:
    mode = _entry_mode(output_root, "output root")
    if mode is not None and _is_redirect(output_root, mode):
        message = f"output root must not be linked: {output_root}"
        raise ExactTreeError(message)
    try:
        return output_root.resolve()
    except OSError as error:
        message = f"output root resolution failed: {output_root}: {error}"
        raise ExactTreeError(message) from error


def _cleanup_exact_staging(path: Path) -> str | None:
    try:
        shutil.rmtree(path)
    except FileNotFoundError:
        return None
    except OSError as error:
        return str(error)
    return None


def _raise_exact_cleanup_failure(error: Exception, cleanup_error: str) -> None:
    message = f"{error}; staging cleanup failed: {cleanup_error}"
    raise ExactTreeError(message) from error


def _publish_exact_output(staging: Path, destination: Path) -> None:
    try:
        publish_directory_no_replace(staging, destination)
    except OSError as error:
        message = f"exact output publication failed: {error}"
        raise ExactTreeError(message) from error


def materialize_exact_plan(
    source_root: Path,
    plan: ExactAuthoringPlan,
    output_root: Path,
) -> None:
    """Verify an exact plan completely before publishing its output tree.

    The candidate source must match the authoring snapshot exactly. Fuzzy
    admission belongs to later layers.

    """
    resolved_source = _resolved_tree_root(source_root)
    resolved_output = _resolved_output_root(output_root)
    _verify_source(resolved_source, plan)
    staging_root = _prepare_staging(resolved_output)
    try:
        _populate_staging(resolved_source, staging_root, plan)
        _publish_exact_output(staging_root, resolved_output)
    except Exception as error:
        cleanup_error = _cleanup_exact_staging(staging_root)
        if cleanup_error is not None:
            _raise_exact_cleanup_failure(error, cleanup_error)
        raise

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
#   - Strict portable parsing of the supported LSPA/LMLM container layout.
# - Must-Not:
#   - Install mods, infer undocumented formats, or write caller content.
# - Allows:
#   - Complete local LMLM archive bytes and validated entry records.
# - Split-When:
#   - Container parsing and path policy gain independent lifecycles.
# - Merge-When:
#   - LMLM compatibility is retired.
# - Summary:
#   - Parses supported LMLM archives fail-closed.
# - Description:
#   - Validates the bounded LSPA v5 layout used by mod conversion.
# - Usage:
#   - Imported by tools/lmlm/main.py and standalone tests.
# - Defaults:
#   - Unsupported structure, unsafe paths, and hidden bytes fail.
#

"""Strict parser for the supported LSPA v5 container used by LMLM mods."""

from __future__ import annotations

from dataclasses import dataclass

BLOCK = 0x200
ROOT_BLOCK = 0x400
FIRST_ENTRY = 0x600
ENTRY_KIND = 2
MAX_DIRECTORY_DEPTH = 64
MAGIC = b"LSPA"
VERSION = 5
VERSION_OFFSET = 4
HEADER_FLAGS = 0x0200_0000
HEADER_FLAGS_OFFSET = 0x0C
_MAX_COMPONENT_UTF16_UNITS = 255
_MAX_PATH_UTF16_UNITS = 259
_UNICODE_PATH_MODIFIERS = frozenset(
    [0x061C, 0xFEFF]
    + list(range(0x200B, 0x2010))
    + list(range(0x202A, 0x202F))
    + list(range(0x2060, 0x2065))
    + list(range(0x2066, 0x2070))
)


class LmlmError(ValueError):
    """A deterministic failure while validating an LMLM archive."""


@dataclass(frozen=True, slots=True)
class FileEntry:
    """One validated file payload declared by an LMLM archive."""

    path: str
    offset: int
    size: int


def _slice(data: bytes, start: int, length: int) -> bytes:
    if start < 0 or length < 0 or start + length > len(data):
        raise LmlmError("archive is truncated or malformed")
    return data[start : start + length]


def _uint(data: bytes, start: int, length: int) -> int:
    return int.from_bytes(_slice(data, start, length), "little", signed=False)


def _first_nonzero(data: bytes, start: int, length: int) -> tuple[int, int] | None:
    end = start + length
    _slice(data, start, length)
    if data.count(0, start, end) == length:
        return None
    for offset in range(start, end):
        value = data[offset]
        if value:
            return offset, value
    return None


def _zero(data: bytes, start: int, length: int, label: str) -> None:
    found = _first_nonzero(data, start, length)
    if found is not None:
        offset, value = found
        raise LmlmError(f"{label} at {offset:#x} is nonzero: {value:#04x}")


def _validate_header(data: bytes) -> None:
    observed = _slice(data, 0, 4)
    if observed != MAGIC:
        raise LmlmError(
            "not an LSPA (.lmlm) archive; observed magic: "
            f"0x{int.from_bytes(observed, 'big'):08x}"
        )
    version = _uint(data, VERSION_OFFSET, 4)
    if version != VERSION:
        raise LmlmError(f"unsupported LSPA archive version: {version}")
    flags = _uint(data, HEADER_FLAGS_OFFSET, 4)
    if flags != HEADER_FLAGS:
        raise LmlmError(f"unsupported LSPA archive header flags: {flags:#010x}")
    _zero(data, 8, 4, "reserved LSPA header byte")
    _zero(data, 0x10, BLOCK - 0x10, "reserved LSPA header byte")
    _zero(data, BLOCK, BLOCK, "reserved LSPA container byte")


def _root_count(data: bytes) -> int:
    _zero(data, ROOT_BLOCK, 2, "reserved LSPA root byte")
    _zero(data, ROOT_BLOCK + 8, BLOCK - 8, "reserved LSPA root byte")
    flags = _uint(data, ROOT_BLOCK + 4, 4)
    if flags > 1:
        raise LmlmError(f"unsupported LSPA root flags: {flags:#010x}")
    return _uint(data, ROOT_BLOCK + 2, 2)


def _utf16_units(value: str) -> int:
    return len(value.encode("utf-16-le")) // 2


def _reserved_device(name: str) -> bool:
    stem = name.split(".", 1)[0].rstrip(" .").upper()
    if stem in {"CON", "PRN", "AUX", "NUL", "CONIN$", "CONOUT$"}:
        return True
    for prefix in ("COM", "LPT"):
        if stem.startswith(prefix):
            suffix = stem[len(prefix) :]
            if suffix in {"1", "2", "3", "4", "5", "6", "7", "8", "9", "¹", "²", "³"}:
                return True
    return False


def _safe_component(name: str) -> bool:
    if not name or name in {".", ".."}:
        return False
    if _utf16_units(name) > _MAX_COMPONENT_UTF16_UNITS:
        return False
    if "/" in name or "\\" in name or name.endswith((".", " ")):
        return False
    if _reserved_device(name):
        return False
    for character in name:
        point = ord(character)
        if character in '<>:"|?*' or point < 0x20 or point == 0x7F:
            return False
        if point in _UNICODE_PATH_MODIFIERS:
            return False
    return True


def _register_path(name: str, prefix: str, seen: dict[str, str]) -> str:
    if not _safe_component(name):
        raise LmlmError(f"unsafe path in archive: {name!r}")
    path = f"{prefix}{name}"
    if _utf16_units(path) > _MAX_PATH_UTF16_UNITS:
        raise LmlmError(f"unsafe path in archive: {path!r}")
    identity = path.upper()
    previous = seen.get(identity)
    if previous is not None:
        raise LmlmError(
            f"archive paths collide on a portable filesystem: {previous!r} and {path!r}"
        )
    seen[identity] = path
    return path


def _read_name(data: bytes, position: int) -> str:
    block = _slice(data, position + 2, BLOCK - 2)
    terminator = None
    for index in range(0, len(block), 2):
        if _uint(block, index, 2) == 0:
            terminator = index
            break
    if terminator is None:
        raise LmlmError(f"LSPA entry name at {position:#x} has no UTF-16 terminator")
    _zero(
        data,
        position + 2 + terminator + 2,
        len(block) - terminator - 2,
        "LSPA entry name padding",
    )
    try:
        return block[:terminator].decode("utf-16-le", errors="strict")
    except UnicodeDecodeError as error:
        raise LmlmError(f"entry name at {position:#x} is not valid UTF-16") from error


def _metadata_offset(data: bytes, metadata: int) -> int:
    _zero(data, metadata, 2, "LSPA entry metadata padding")
    _zero(data, metadata + 0x0B, 1, "LSPA entry metadata padding")
    offset = _uint(data, metadata + 0x14, 8)
    _zero(
        data,
        metadata + 0x1C,
        BLOCK - 0x1C,
        "LSPA entry metadata padding",
    )
    return offset


def _directory_record(
    data: bytes,
    metadata: int,
    path: str,
    depth: int,
) -> tuple[int, int]:
    control_offset = metadata + 0x0E
    control = _uint(data, control_offset, 1)
    if control > 1:
        raise LmlmError(
            "unsupported LSPA directory child-kind control at "
            f"{control_offset:#x}: {control:#04x}"
        )
    _zero(
        data,
        control_offset + 1,
        5,
        "LSPA entry metadata padding",
    )
    child_depth = depth + 1
    if child_depth > MAX_DIRECTORY_DEPTH:
        raise LmlmError(
            f"archive directory nesting is too deep: {path!r} at depth {child_depth}"
        )
    return _uint(data, metadata + 0x0C, 2), child_depth


def _validate_file_control(data: bytes, start: int) -> int:
    control = _uint(data, start, 1)
    if control > 1:
        raise LmlmError(
            f"unsupported LSPA file transition control at {start:#x}: {control:#04x}"
        )
    _zero(
        data,
        start + 1,
        BLOCK - 1,
        "LSPA file transition padding",
    )
    return start + BLOCK


def _file_record_end(
    data: bytes,
    metadata_end: int,
    globally_final: bool,
    entries: list[FileEntry],
) -> int:
    if not globally_final:
        return _validate_file_control(data, metadata_end)
    earliest_payload = min((entry.offset for entry in entries), default=None)
    control_end = metadata_end + BLOCK
    if earliest_payload is not None and earliest_payload >= control_end:
        return _validate_file_control(data, metadata_end)
    return metadata_end


def _parse_entries(
    data: bytes,
    position: int,
    count: int,
    prefix: str,
    entries: list[FileEntry],
    seen: dict[str, str],
    depth: int,
    globally_final_branch: bool,
    table_end: list[int],
) -> int:
    for index in range(count):
        globally_final = globally_final_branch and index + 1 == count
        kind = _uint(data, position, 2)
        if kind != ENTRY_KIND:
            raise LmlmError(
                f"unsupported LSPA entry kind at {position:#x}: {kind}"
            )
        full_path = _register_path(_read_name(data, position), prefix, seen)
        metadata = position + BLOCK
        metadata_end = metadata + BLOCK
        table_end[0] = max(table_end[0], metadata_end)
        offset = _metadata_offset(data, metadata)
        if offset == 0:
            child_count, child_depth = _directory_record(
                data,
                metadata,
                full_path,
                depth,
            )
            position = _parse_entries(
                data,
                position + BLOCK * 2,
                child_count,
                f"{full_path}/",
                entries,
                seen,
                child_depth,
                globally_final,
                table_end,
            )
            continue
        size = _uint(data, metadata + 0x0C, 8)
        entries.append(FileEntry(full_path, offset, size))
        position = _file_record_end(
            data,
            metadata_end,
            globally_final,
            entries,
        )
        table_end[0] = max(table_end[0], position)
    return position


def _validate_payloads(
    data: bytes,
    entries: list[FileEntry],
    table_end: int,
) -> None:
    for entry in entries:
        if entry.offset < table_end:
            raise LmlmError(
                "archive entry payload overlaps the table: "
                f"{entry.path!r} at {entry.offset}, table ends at {table_end}"
            )
        if entry.offset % BLOCK:
            raise LmlmError(
                "archive entry payload is not block aligned: "
                f"{entry.path!r} at {entry.offset}"
            )
        if entry.offset + entry.size > len(data):
            raise LmlmError(
                "archive entry payload is out of bounds: "
                f"{entry.path!r} at {entry.offset} for {entry.size} bytes"
            )
    ordered = sorted(
        entries,
        key=lambda entry: (entry.offset, entry.size, entry.path),
    )
    for first, second in zip(ordered, ordered[1:], strict=False):
        if second.offset < first.offset + first.size:
            raise LmlmError(
                "archive entry payloads overlap: "
                f"{first.path!r} and {second.path!r}"
            )
    claimed_end = table_end
    for entry in ordered:
        _zero(
            data,
            claimed_end,
            entry.offset - claimed_end,
            "unclaimed LSPA byte",
        )
        claimed_end = entry.offset + entry.size
    payload_end = max(
        [table_end, *(entry.offset + entry.size for entry in entries)]
    )
    _zero(
        data,
        payload_end,
        len(data) - payload_end,
        "trailing LSPA byte",
    )


def parse_archive(data: bytes) -> tuple[FileEntry, ...]:
    """Parse and fully validate one supported LMLM archive."""
    _validate_header(data)
    entries: list[FileEntry] = []
    seen: dict[str, str] = {}
    table_end = [FIRST_ENTRY]
    _parse_entries(
        data,
        FIRST_ENTRY,
        _root_count(data),
        "",
        entries,
        seen,
        0,
        True,
        table_end,
    )
    _validate_payloads(data, entries, table_end[0])
    return tuple(entries)


def entry_bytes(data: bytes, entry: FileEntry) -> bytes:
    """Return the validated payload bytes for one parsed entry."""
    return _slice(data, entry.offset, entry.size)

# Copyright:
#   - Copyright © 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE-MIT
#
# Boundary-Contract:
# - Owns:
#   - Canonical build-runner project-state migration tests.
# - Must-Not:
#   - Invoke Unreal or mutate the repository project.
# - Allows:
#   - Temporary project roots, directory links, and injected failures.
# - Split-When:
#   - Build-runner tests gain an independent platform integration lifecycle.
# - Merge-When:
#   - Project-state migration moves to a dedicated adapter.
# - Summary:
#   - Build runner project-state migration tests.
# - Description:
#   - Proves generated Unreal project roots live physically below build cache.
# - Usage:
#   - Run directly with Python's standard-library unittest runner.
# - Defaults:
#   - Tests use isolated temporary directories only.
#

"""Tests for canonical build-runner project-state migration."""

# CSpell:ignore APPL BNDL FMWK PHDR RVA SHLIB dylinker linkedit
# CSpell:ignore osabi phdr rva shlib

from __future__ import annotations

from collections.abc import Callable
from collections.abc import Iterator
import hashlib
import importlib.util
import io
import os
from pathlib import Path
import tempfile
import unittest
from unittest import mock

_RUN_PATH = (
    Path(__file__).resolve().parents[2]
    / "tools"
    / "build"
    / "adapter-inbound"
    / "run.py"
)
_SPEC = importlib.util.spec_from_file_location("shar_build_run", _RUN_PATH)
if _SPEC is None or _SPEC.loader is None:
    raise RuntimeError("cannot load build runner for tests")
_RUN = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(_RUN)


def _synthetic_elf(
    machine: int,
    *,
    image_type: int = 3,
    entrypoint: int = 0x400000,
    segment_offset: int = 120,
    segment_file_size: int = 1,
    segment_memory_size: int = 1,
) -> bytes:
    """Return one minimal loadable little-endian ELF64 image."""
    header = bytearray(64)
    header[:4] = b"\x7fELF"
    header[4] = 2
    header[5] = 1
    header[6] = 1
    header[16:18] = image_type.to_bytes(2, "little")
    header[18:20] = machine.to_bytes(2, "little")
    header[20:24] = (1).to_bytes(4, "little")
    header[24:32] = entrypoint.to_bytes(8, "little")
    header[32:40] = (64).to_bytes(8, "little")
    header[52:54] = (64).to_bytes(2, "little")
    header[54:56] = (56).to_bytes(2, "little")
    header[56:58] = (1).to_bytes(2, "little")
    program = bytearray(56)
    program[:4] = (1).to_bytes(4, "little")
    program[4:8] = (0x5).to_bytes(4, "little")
    program[8:16] = segment_offset.to_bytes(8, "little")
    program[16:24] = (0x400000).to_bytes(8, "little")
    program[32:40] = segment_file_size.to_bytes(8, "little")
    program[40:48] = segment_memory_size.to_bytes(8, "little")
    return bytes(header + program + b"\0")


def _synthetic_elf_with_interpreter(
    path: bytes,
    *,
    interpreter_first: bool = True,
    duplicate: bool = False,
) -> bytes:
    """Return one ELF executable with explicit interpreter program headers."""
    program_count = 3 if duplicate else 2
    program_end = 64 + (56 * program_count)
    load_offset = program_end + len(path)
    baseline = bytearray(
        _synthetic_elf(
            0x003E,
            image_type=2,
            segment_offset=load_offset,
        )
    )
    header = bytearray(baseline[:64])
    load = bytearray(baseline[64:120])
    header[56:58] = program_count.to_bytes(2, "little")
    load[8:16] = load_offset.to_bytes(8, "little")
    interpreter = bytearray(56)
    interpreter[:4] = (3).to_bytes(4, "little")
    interpreter[8:16] = program_end.to_bytes(8, "little")
    interpreter[32:40] = len(path).to_bytes(8, "little")
    interpreter[40:48] = len(path).to_bytes(8, "little")
    if interpreter_first:
        programs = [interpreter]
        if duplicate:
            programs.append(bytearray(interpreter))
        programs.append(load)
    else:
        programs = [load, interpreter]
    return bytes(header + b"".join(programs) + path + b"\0")


def _synthetic_elf_with_program_header(
    *,
    phdr_first: bool = True,
    duplicate: bool = False,
    offset_delta: int = 0,
    size_delta: int = 0,
    mapped: bool = True,
) -> bytes:
    """Return one ELF executable carrying an explicit PT_PHDR segment."""
    program_count = 3 if duplicate else 2
    program_end = 64 + (56 * program_count)
    load_offset = 0 if mapped else program_end
    load_size = program_end + 1 if mapped else 1
    entrypoint = 0x400000 + (program_end if mapped else 0)
    baseline = bytearray(
        _synthetic_elf(
            0x003E,
            image_type=2,
            entrypoint=entrypoint,
            segment_offset=load_offset,
            segment_file_size=load_size,
            segment_memory_size=load_size,
        )
    )
    header = bytearray(baseline[:64])
    load = bytearray(baseline[64:120])
    header[56:58] = program_count.to_bytes(2, "little")
    load[8:16] = load_offset.to_bytes(8, "little")
    load[32:40] = load_size.to_bytes(8, "little")
    load[40:48] = load_size.to_bytes(8, "little")
    table_size = 56 * program_count
    phdr = bytearray(56)
    phdr[:4] = (6).to_bytes(4, "little")
    phdr[8:16] = (64 + offset_delta).to_bytes(8, "little")
    phdr[16:24] = (0x400040).to_bytes(8, "little")
    declared_size = table_size + size_delta
    phdr[32:40] = declared_size.to_bytes(8, "little")
    phdr[40:48] = declared_size.to_bytes(8, "little")
    if phdr_first:
        programs = [phdr]
        if duplicate:
            programs.append(bytearray(phdr))
        programs.append(load)
    else:
        programs = [load, phdr]
    return bytes(header + b"".join(programs) + b"\0")


def _synthetic_big_endian_elf(machine: int) -> bytes:
    """Return the minimal ELF fixture encoded as big endian."""
    payload = bytearray(_synthetic_elf(machine))
    payload[5] = 2
    for start, width, value in (
        (16, 2, 3),
        (18, 2, machine),
        (20, 4, 1),
        (24, 8, 0x400000),
        (32, 8, 64),
        (52, 2, 64),
        (54, 2, 56),
        (56, 2, 1),
        (64, 4, 1),
        (68, 4, 0x5),
        (72, 8, 120),
        (80, 8, 0x400000),
        (96, 8, 1),
        (104, 8, 1),
    ):
        payload[start : start + width] = value.to_bytes(width, "big")
    return bytes(payload)


def _synthetic_pe(
    machine: int,
    *,
    section_count: int = 1,
    characteristics: int = 0x0002,
    section_characteristics: int = 0x60000020,
    section_raw_offset: int | None = None,
    section_raw_size: int = 0x200,
) -> bytes:
    """Return one minimal bounded PE32+ image fixture."""
    offset = 0x80
    optional_size = 112
    file_alignment = 0x200
    section_alignment = 0x1000
    section_table = offset + 24 + optional_size
    header_end = section_table + (40 * section_count)
    data_offset = (
        (header_end + file_alignment - 1) // file_alignment * file_alignment
    )
    payload_size = data_offset + (section_raw_size if section_count else 0)
    payload = bytearray(payload_size)
    payload[:2] = b"MZ"
    payload[0x3C:0x40] = offset.to_bytes(4, "little")
    payload[offset : offset + 4] = b"PE\0\0"
    coff = offset + 4
    payload[coff : coff + 2] = machine.to_bytes(2, "little")
    payload[coff + 2 : coff + 4] = section_count.to_bytes(2, "little")
    payload[coff + 16 : coff + 18] = optional_size.to_bytes(2, "little")
    payload[coff + 18 : coff + 20] = characteristics.to_bytes(2, "little")
    optional = coff + 20
    payload[optional : optional + 2] = bytes.fromhex("0b02")
    payload[optional + 16 : optional + 20] = (0x1000).to_bytes(4, "little")
    payload[optional + 32 : optional + 36] = section_alignment.to_bytes(
        4,
        "little",
    )
    payload[optional + 36 : optional + 40] = file_alignment.to_bytes(
        4,
        "little",
    )
    payload[optional + 56 : optional + 60] = (0x2000).to_bytes(4, "little")
    payload[optional + 60 : optional + 64] = data_offset.to_bytes(4, "little")
    if section_count:
        raw_offset = (
            data_offset if section_raw_offset is None else section_raw_offset
        )
        payload[section_table + 8 : section_table + 12] = (1).to_bytes(
            4, "little"
        )
        payload[section_table + 12 : section_table + 16] = (0x1000).to_bytes(
            4, "little"
        )
        payload[section_table + 16 : section_table + 20] = (
            section_raw_size.to_bytes(4, "little")
        )
        payload[section_table + 20 : section_table + 24] = raw_offset.to_bytes(
            4,
            "little",
        )
        payload[section_table + 36 : section_table + 40] = (
            section_characteristics.to_bytes(4, "little")
        )
    return bytes(payload)


def _synthetic_linkedit_segment(file_size: int) -> bytes:
    """Return one minimal read-only LC_SEGMENT_64 __LINKEDIT command."""
    return (
        (0x19).to_bytes(4, "little")
        + (72).to_bytes(4, "little")
        + b"__LINKEDIT"
        + (b"\0" * 6)
        + (0x100001000).to_bytes(8, "little")
        + (0).to_bytes(8, "little")
        + file_size.to_bytes(8, "little")
        + (0).to_bytes(8, "little")
        + (0x1).to_bytes(4, "little")
        + (0x1).to_bytes(4, "little")
        + (0).to_bytes(8, "little")
    )


def _synthetic_build_version(platform: int) -> bytes:
    """Return one minimal LC_BUILD_VERSION command for a target platform."""
    return (
        (0x32).to_bytes(4, "little")
        + (24).to_bytes(4, "little")
        + platform.to_bytes(4, "little")
        + (0x000D0000).to_bytes(4, "little")
        + (0x000D0000).to_bytes(4, "little")
        + (0).to_bytes(4, "little")
    )


def _synthetic_dylinker_command() -> bytes:
    """Return one aligned LC_LOAD_DYLINKER command with a local dyld path."""
    path = b"/usr/lib/dyld\0"
    command_size = 32
    return (
        (0xE).to_bytes(4, "little")
        + command_size.to_bytes(4, "little")
        + (12).to_bytes(4, "little")
        + path
        + (b"\0" * (command_size - 12 - len(path)))
    )


def _synthetic_macho(
    cpu: int,
    file_type: int = 2,
    *,
    command: int = 0x80000028,
    entry_offset: int | None = None,
    prefix_command_size: int = 0,
    platform: int = 1,
) -> bytes:
    """Return one minimal little-endian Mach-O64 image fixture."""
    entry_command_size = 24
    prefix_command = b""
    if prefix_command_size:
        prefix_command = (
            (0x1B).to_bytes(4, "little")
            + prefix_command_size.to_bytes(4, "little")
            + (b"\0" * (prefix_command_size - 8))
        )
    segment_size = 72
    linkedit_size = segment_size
    dylinker = _synthetic_dylinker_command()
    dylinker_size = len(dylinker)
    build_version = _synthetic_build_version(platform)
    command_count = 5 + bool(prefix_command)
    command_bytes = (
        segment_size
        + len(prefix_command)
        + entry_command_size
        + linkedit_size
        + dylinker_size
        + len(build_version)
    )
    command_end = 32 + command_bytes
    resolved_entry = command_end if entry_offset is None else entry_offset
    file_size = command_end + 1
    segment = (
        (0x19).to_bytes(4, "little")
        + segment_size.to_bytes(4, "little")
        + b"__TEXT"
        + (b"\0" * 10)
        + (0x100000000).to_bytes(8, "little")
        + (0x1000).to_bytes(8, "little")
        + (0).to_bytes(8, "little")
        + file_size.to_bytes(8, "little")
        + (0x5).to_bytes(4, "little")
        + (0x5).to_bytes(4, "little")
        + (0).to_bytes(8, "little")
    )
    linkedit = _synthetic_linkedit_segment(file_size)
    return (
        bytes.fromhex("cffaedfe")
        + cpu.to_bytes(4, "little")
        + (0).to_bytes(4, "little")
        + file_type.to_bytes(4, "little")
        + int(command_count).to_bytes(4, "little")
        + command_bytes.to_bytes(4, "little")
        + (0).to_bytes(8, "little")
        + segment
        + prefix_command
        + command.to_bytes(4, "little")
        + entry_command_size.to_bytes(4, "little")
        + resolved_entry.to_bytes(8, "little")
        + (0).to_bytes(8, "little")
        + linkedit
        + dylinker
        + build_version
        + b"\0"
    )


def _synthetic_thread_entry_macho(
    cpu: int,
    *,
    flavor: int = 6,
    count: int = 68,
    pc: int | None = None,
) -> bytes:
    """Return one little-endian Mach-O64 legacy thread-entry fixture."""
    segment_size = 72
    thread_size = 288
    dylinker = _synthetic_dylinker_command()
    dylinker_size = len(dylinker)
    build_version = _synthetic_build_version(1)
    command_bytes = (
        segment_size
        + thread_size
        + segment_size
        + dylinker_size
        + len(build_version)
    )
    command_end = 32 + command_bytes
    resolved_pc = 0x100000000 + command_end if pc is None else pc
    file_size = command_end + 1
    state = bytearray(272)
    state[256:264] = resolved_pc.to_bytes(8, "little")
    body = (
        flavor.to_bytes(4, "little")
        + count.to_bytes(4, "little")
        + bytes(state)
    )
    segment = (
        (0x19).to_bytes(4, "little")
        + segment_size.to_bytes(4, "little")
        + b"__TEXT"
        + (b"\0" * 10)
        + (0x100000000).to_bytes(8, "little")
        + (0x1000).to_bytes(8, "little")
        + (0).to_bytes(8, "little")
        + file_size.to_bytes(8, "little")
        + (0x5).to_bytes(4, "little")
        + (0x5).to_bytes(4, "little")
        + (0).to_bytes(8, "little")
    )
    linkedit = _synthetic_linkedit_segment(file_size)
    return (
        bytes.fromhex("cffaedfe")
        + cpu.to_bytes(4, "little")
        + (0).to_bytes(4, "little")
        + (2).to_bytes(4, "little")
        + (5).to_bytes(4, "little")
        + command_bytes.to_bytes(4, "little")
        + (0).to_bytes(8, "little")
        + segment
        + (0x5).to_bytes(4, "little")
        + thread_size.to_bytes(4, "little")
        + body
        + linkedit
        + dylinker
        + build_version
        + b"\0"
    )


def _synthetic_fat_macho(
    cpu_types: tuple[int, ...],
    platform: int = 1,
) -> bytes:
    """Return one page-aligned big-endian universal Mach-O fixture."""
    entry_size = 20
    table_end = 8 + (entry_size * len(cpu_types))
    slices = [_synthetic_macho(cpu, platform=platform) for cpu in cpu_types]
    cursor = table_end
    entries: list[bytes] = []
    body = bytearray()
    for cpu, payload in zip(cpu_types, slices, strict=True):
        alignment_power = 14 if cpu == _RUN._MACHO_ARM64_CPU else 12
        alignment = 1 << alignment_power
        offset = (cursor + alignment - 1) & -alignment
        body.extend(b"\0" * (offset - cursor))
        body.extend(payload)
        entries.append(
            cpu.to_bytes(4, "big")
            + (b"\0" * 4)
            + offset.to_bytes(4, "big")
            + len(payload).to_bytes(4, "big")
            + alignment_power.to_bytes(4, "big")
        )
        cursor = offset + len(payload)
    return (
        bytes.fromhex("cafebabe")
        + len(cpu_types).to_bytes(4, "big")
        + b"".join(entries)
        + bytes(body)
    )


def _synthetic_fat64_macho(*, reserved: int = 0) -> bytes:
    """Return one page-aligned big-endian FAT64 ARM64 fixture."""
    payload = _synthetic_macho(_RUN._MACHO_ARM64_CPU)
    entry_size = 32
    offset = 0x4000
    entry = (
        _RUN._MACHO_ARM64_CPU.to_bytes(4, "big")
        + (0).to_bytes(4, "big")
        + offset.to_bytes(8, "big")
        + len(payload).to_bytes(8, "big")
        + (14).to_bytes(4, "big")
        + reserved.to_bytes(4, "big")
    )
    return (
        bytes.fromhex("cafebabf")
        + (1).to_bytes(4, "big")
        + entry
        + (b"\0" * (offset - 8 - entry_size))
        + payload
    )


def _android_pool_length8(value: int) -> bytes:
    """Encode one synthetic Android UTF-8 string-pool length."""
    if value < 0x80:
        return bytes([value])
    return bytes([0x80 | (value >> 8), value & 0xFF])


def _android_pool_length16(value: int) -> bytes:
    """Encode one synthetic Android UTF-16 string-pool length."""
    if value < 0x8000:
        return value.to_bytes(2, "little")
    first = 0x8000 | (value >> 16)
    return first.to_bytes(2, "little") + (value & 0xFFFF).to_bytes(2, "little")


def _synthetic_android_manifest(
    *,
    root_name: str = "manifest",
    package_name: str | None = "org.shar.game",
    split_name: str | None = None,
    utf8: bool = True,
    child_name: str | None = None,
) -> bytes:
    """Return one minimal Android resource binary XML manifest fixture."""
    names = [root_name]
    attributes: list[tuple[int, int]] = []
    if package_name is not None:
        key_index = len(names)
        names.append("package")
        value_index = len(names)
        names.append(package_name)
        attributes.append((key_index, value_index))
    if split_name is not None:
        key_index = len(names)
        names.append("split")
        value_index = len(names)
        names.append(split_name)
        attributes.append((key_index, value_index))
    child_index: int | None = None
    if child_name is not None:
        child_index = len(names)
        names.append(child_name)
    offsets: list[int] = []
    string_data = bytearray()
    for name in names:
        offsets.append(len(string_data))
        if utf8:
            encoded = name.encode("utf-8")
            string_data += _android_pool_length8(len(name))
            string_data += _android_pool_length8(len(encoded)) + encoded + b"\0"
        else:
            encoded = name.encode("utf-16-le")
            string_data += _android_pool_length16(len(name)) + encoded + b"\0\0"
    while len(string_data) % 4:
        string_data += b"\0"
    strings_start = 28 + (4 * len(names))
    string_pool_size = strings_start + len(string_data)
    string_pool = (
        (0x0001).to_bytes(2, "little")
        + (28).to_bytes(2, "little")
        + string_pool_size.to_bytes(4, "little")
        + len(names).to_bytes(4, "little")
        + (0).to_bytes(4, "little")
        + (0x100 if utf8 else 0).to_bytes(4, "little")
        + strings_start.to_bytes(4, "little")
        + (0).to_bytes(4, "little")
        + b"".join(offset.to_bytes(4, "little") for offset in offsets)
        + bytes(string_data)
    )

    def start_element(
        name_index: int,
        entries: tuple[tuple[int, int], ...] = (),
    ) -> bytes:
        encoded_attributes = bytearray()
        for key_index, value_index in entries:
            encoded_attributes += (
                (0xFFFFFFFF).to_bytes(4, "little")
                + key_index.to_bytes(4, "little")
                + value_index.to_bytes(4, "little")
                + (8).to_bytes(2, "little")
                + b"\0"
                + b"\x03"
                + value_index.to_bytes(4, "little")
            )
        chunk_size = 36 + len(encoded_attributes)
        return (
            (0x0102).to_bytes(2, "little")
            + (16).to_bytes(2, "little")
            + chunk_size.to_bytes(4, "little")
            + (1).to_bytes(4, "little")
            + (0xFFFFFFFF).to_bytes(4, "little")
            + (0xFFFFFFFF).to_bytes(4, "little")
            + name_index.to_bytes(4, "little")
            + (20).to_bytes(2, "little")
            + (20).to_bytes(2, "little")
            + len(entries).to_bytes(2, "little")
            + (b"\0" * 6)
            + bytes(encoded_attributes)
        )

    nodes = start_element(0, tuple(attributes))
    if child_index is not None:
        nodes += start_element(child_index)
    size = 8 + len(string_pool) + len(nodes)
    return (
        (0x0003).to_bytes(2, "little")
        + (8).to_bytes(2, "little")
        + size.to_bytes(4, "little")
        + string_pool
        + nodes
    )


def _write_android_apk(
    path: Path,
    machine: int = 0x00B7,
    *,
    entrypoint: int = 0x400000,
) -> None:
    """Write one synthetic APK with manifest and native library entries."""
    with _RUN.zipfile.ZipFile(path, "w") as archive:
        archive.writestr("AndroidManifest.xml", _synthetic_android_manifest())
        archive.writestr(
            "lib/arm64-v8a/libUnreal.so",
            _synthetic_elf(machine, entrypoint=entrypoint),
        )


def _unix_special_zip_member(name: str, kind: int) -> object:
    """Return one ZIP member carrying a declared Unix special file type."""
    info = _RUN.zipfile.ZipInfo(name)
    info.create_system = 3
    info.external_attr = (kind | 0o777) << 16
    return info


def _corrupt_stored_zip_member(path: Path, member: str) -> None:
    """Flip one stored byte without changing central metadata.

    Raises:
        AssertionError: If the synthetic ZIP fixture is malformed.

    """
    with _RUN.zipfile.ZipFile(path) as archive:
        info = archive.getinfo(member)
        offset = info.header_offset
    with path.open("r+b") as handle:
        handle.seek(offset)
        local = handle.read(30)
        if len(local) != 30 or local[:4] != bytes.fromhex("504b0304"):
            raise AssertionError("invalid synthetic ZIP local header")
        name_size = int.from_bytes(local[26:28], "little")
        extra_size = int.from_bytes(local[28:30], "little")
        handle.seek(offset + 30 + name_size + extra_size)
        value = handle.read(1)
        if not value:
            raise AssertionError("synthetic ZIP member is empty")
        handle.seek(-1, 1)
        handle.write(bytes([value[0] ^ 0xFF]))


def _write_ios_ipa(
    path: Path,
    cpu: int = 0x0100000C,
    *,
    binary: bytes | None = None,
    bundle_id: object = "org.shar.game",
    package_type: object | None = "APPL",
) -> None:
    """Write one synthetic IPA with a declared main executable."""
    document: dict[str, object] = {"CFBundleExecutable": "shar"}
    if bundle_id is not None:
        document["CFBundleIdentifier"] = bundle_id
    if package_type is not None:
        document["CFBundlePackageType"] = package_type
    with _RUN.zipfile.ZipFile(path, "w") as archive:
        archive.writestr(
            "Payload/SHAR.app/Info.plist",
            _RUN.plistlib.dumps(document),
        )
        archive.writestr(
            "Payload/SHAR.app/shar",
            _synthetic_macho(cpu, platform=2) if binary is None else binary,
        )


class ProjectStateMigrationTests(unittest.TestCase):
    """Exercise build-state adoption without running Unreal."""

    def _fixture(self) -> tuple[tempfile.TemporaryDirectory[str], Path, Path]:
        temporary = tempfile.TemporaryDirectory(prefix="shar-build-state-")
        root = Path(temporary.name)
        project_dir = root / "project"
        project_dir.mkdir()
        project = project_dir / "shar.uproject"
        project.write_text("{}\n", encoding="utf-8")
        return temporary, root, project

    def _unlink_project_state(self, project: Path) -> None:
        for name in _RUN._PROJECT_STATE_NAMES:
            path = project.parent / name
            if _RUN._is_directory_link(path):
                _RUN._remove_directory_link(path)

    def test_adopts_all_legacy_project_state_roots(self) -> None:
        temporary, root, project = self._fixture()
        try:
            for name in _RUN._PROJECT_STATE_NAMES:
                source = project.parent / name
                source.mkdir()
                (source / "sentinel.txt").write_text(name, encoding="utf-8")

            state_root = _RUN._prepare_project_state(root, project)

            self.assertEqual(state_root, root / _RUN._PROJECT_STATE_ROOT)
            for name in _RUN._PROJECT_STATE_NAMES:
                link = project.parent / name
                canonical = state_root / name
                self.assertTrue(_RUN._is_directory_link(link))
                self.assertEqual(link.resolve(), canonical.resolve())
                self.assertEqual(
                    (canonical / "sentinel.txt").read_text(encoding="utf-8"),
                    name,
                )
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()

    def test_reuses_existing_canonical_links(self) -> None:
        temporary, root, project = self._fixture()
        try:
            first = _RUN._prepare_project_state(root, project)
            second = _RUN._prepare_project_state(root, project)
            self.assertEqual(first, second)
            for name in _RUN._PROJECT_STATE_NAMES:
                self.assertTrue(_RUN._is_directory_link(project.parent / name))
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()

    def test_rejects_legacy_and_canonical_state_conflict(self) -> None:
        temporary, root, project = self._fixture()
        try:
            legacy = project.parent / "Saved"
            legacy.mkdir()
            (legacy / "legacy.txt").write_text("legacy", encoding="utf-8")
            canonical = root / _RUN._PROJECT_STATE_ROOT / "Saved"
            canonical.mkdir(parents=True)
            (canonical / "canonical.txt").write_text(
                "canonical",
                encoding="utf-8",
            )

            with self.assertRaisesRegex(_RUN.RunFailure, "both exist"):
                _RUN._prepare_project_state(root, project)

            self.assertTrue((legacy / "legacy.txt").is_file())
            self.assertTrue((canonical / "canonical.txt").is_file())
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()

    def test_rejects_linked_repository_cache_root(self) -> None:
        temporary, root, project = self._fixture()
        try:
            cache_root = root / ".cache"
            cache_root.mkdir()
            original = _RUN._is_directory_link

            def report_cache_as_link(path: Path) -> bool:
                return path == cache_root or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_cache_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "repository cache root must be a real directory",
                ),
            ):
                _RUN._prepare_project_state(root, project)

            self.assertFalse((root / _RUN._PROJECT_STATE_ROOT).exists())
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()

    def test_rejects_linked_build_cache_root(self) -> None:
        temporary, root, project = self._fixture()
        try:
            build_root = root / ".cache/build"
            build_root.mkdir(parents=True)
            original = _RUN._is_directory_link

            def report_build_as_link(path: Path) -> bool:
                return path == build_root or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_build_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "build cache root must be a real directory",
                ),
            ):
                _RUN._prepare_project_state(root, project)

            self.assertFalse((root / _RUN._PROJECT_STATE_ROOT).exists())
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()

    def test_rolls_back_when_a_later_link_creation_fails(self) -> None:
        temporary, root, project = self._fixture()
        try:
            for name in _RUN._PROJECT_STATE_NAMES:
                source = project.parent / name
                source.mkdir()
                (source / "sentinel.txt").write_text(name, encoding="utf-8")
            original = _RUN._create_directory_link

            def fail_on_intermediate(link: Path, target: Path) -> None:
                if link.name == "Intermediate":
                    raise _RUN.RunFailure("injected link failure")
                original(link, target)

            with (
                mock.patch.object(
                    _RUN,
                    "_create_directory_link",
                    side_effect=fail_on_intermediate,
                ),
                self.assertRaisesRegex(_RUN.RunFailure, "injected"),
            ):
                _RUN._prepare_project_state(root, project)

            for name in _RUN._PROJECT_STATE_NAMES:
                source = project.parent / name
                self.assertTrue(source.is_dir())
                self.assertFalse(_RUN._is_directory_link(source))
                self.assertEqual(
                    (source / "sentinel.txt").read_text(encoding="utf-8"),
                    name,
                )
        finally:
            self._unlink_project_state(project)
            temporary.cleanup()


class UatLauncherTests(unittest.TestCase):
    """Require the process launcher to be a real host-runnable file."""

    def test_rejects_linked_uat_launcher(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-launcher-") as raw:
            engine = Path(raw)
            launcher = engine / "Engine/Build/BatchFiles/RunUAT.sh"
            launcher.parent.mkdir(parents=True)
            launcher.write_text("#!/bin/sh\n", encoding="utf-8")
            launcher.chmod(0o755)
            original = Path.is_symlink

            def report_launcher_as_link(path: Path) -> bool:
                return path == launcher or original(path)

            with (
                mock.patch.object(
                    Path,
                    "is_symlink",
                    report_launcher_as_link,
                ),
                mock.patch.object(_RUN.os, "name", "posix"),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "launcher must be a real file",
                ),
            ):
                _RUN._uat_path(engine)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_rejects_redirected_uat_parent(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-launcher-") as raw:
            root = Path(raw)
            engine = root / "UE_5.8"
            build = engine / "Engine/Build"
            build.mkdir(parents=True)
            external = root / "external-batch"
            external.mkdir()
            launcher = external / "RunUAT.sh"
            launcher.write_text("#!/bin/sh\n", encoding="utf-8")
            launcher.chmod(0o755)
            (build / "BatchFiles").symlink_to(
                external,
                target_is_directory=True,
            )

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "launcher parent must be a real directory",
            ):
                _RUN._uat_path(engine)

    def test_rejects_hard_linked_uat_launcher(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-launcher-") as raw:
            engine = Path(raw)
            launcher = engine / "Engine/Build/BatchFiles/RunUAT.sh"
            launcher.parent.mkdir(parents=True)
            external = engine / "external-uat.sh"
            external.write_text("#!/bin/sh\n", encoding="utf-8")
            external.chmod(0o755)
            launcher.hardlink_to(external)

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "launcher must have one filesystem link",
            ):
                _RUN._uat_path(engine)

    @unittest.skipIf(_RUN.os.name == "nt", "POSIX launcher permission")
    def test_requires_executable_uat_launcher(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-launcher-") as raw:
            engine = Path(raw)
            launcher = engine / "Engine/Build/BatchFiles/RunUAT.sh"
            launcher.parent.mkdir(parents=True)
            launcher.write_text("#!/bin/sh\n", encoding="utf-8")
            launcher.chmod(0o644)

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "launcher is not executable",
            ):
                _RUN._uat_path(engine)

            launcher.chmod(0o755)
            self.assertEqual(_RUN._uat_path(engine), launcher)


class BuildWorkRootTests(unittest.TestCase):
    """Keep Turnkey and UAT work below real repository cache roots."""

    def test_rejects_linked_shared_run_root_before_sdk_verification(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-run-root-") as raw:
            root = Path(raw)
            build_root = root / ".cache/build"
            build_root.mkdir(parents=True)
            run_root = root / _RUN._WORK_ROOT
            run_root.mkdir()
            original = _RUN._is_directory_link

            def report_run_as_link(path: Path) -> bool:
                return path == run_root or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_run_as_link,
                ),
                mock.patch.object(_RUN, "_verify_sdk") as verify_sdk,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "build run root must be a real directory",
                ),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project/shar.uproject"),
                    _RUN._TARGETS_BY_ID["linux-x64"],
                    validate_only=True,
                )
            verify_sdk.assert_not_called()

    def test_rejects_linked_target_work_root_before_sdk_verification(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-target-work-") as raw:
            root = Path(raw)
            run_root = root / _RUN._WORK_ROOT
            run_root.mkdir(parents=True)
            target = _RUN._TARGETS_BY_ID["linux-x64"]
            work = run_root / target.identifier
            work.mkdir()
            original = _RUN._is_directory_link

            def report_work_as_link(path: Path) -> bool:
                return path == work or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_work_as_link,
                ),
                mock.patch.object(_RUN, "_verify_sdk") as verify_sdk,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "target work root must be a real directory",
                ),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project/shar.uproject"),
                    target,
                    validate_only=True,
                )
            verify_sdk.assert_not_called()


class BuildScratchPathTests(unittest.TestCase):
    """Reject redirected UAT candidate and staging scratch identities."""

    def _assert_linked_scratch_rejected(self, name: str, label: str) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-build-scratch-") as raw:
            root = Path(raw)
            work = root / _RUN._WORK_ROOT / "linux-x64"
            work.mkdir(parents=True)
            scratch = work / name
            scratch.mkdir()
            original = _RUN._is_directory_link
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def report_scratch_as_link(path: Path) -> bool:
                return path == scratch or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_scratch_as_link,
                ),
                mock.patch.object(_RUN, "_verify_sdk"),
                mock.patch.object(_RUN, "_run_uat") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    f"{label} must be a real directory",
                ),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    validate_only=False,
                )

            process.assert_not_called()
            self.assertTrue(scratch.is_dir())

    def test_rejects_linked_candidate_scratch_before_cleanup(self) -> None:
        self._assert_linked_scratch_rejected(
            "candidate",
            "candidate scratch root",
        )

    def test_rejects_linked_staging_scratch_before_cleanup(self) -> None:
        self._assert_linked_scratch_rejected(
            "stage",
            "staging scratch root",
        )


class UatWorkPathTests(unittest.TestCase):
    """Keep UAT logs and caches under real work-root identities."""

    def test_rejects_linked_log_before_subprocess(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-log-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            log = work / "build.log"
            log.write_text("sentinel\n", encoding="utf-8")
            original = Path.is_symlink

            def report_log_as_link(path: Path) -> bool:
                return path == log or original(path)

            with (
                mock.patch.object(Path, "is_symlink", report_log_as_link),
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT log must be a real file",
                ),
            ):
                _RUN._run_uat(root, Path("/uat"), ["probe"], log)

            self.assertEqual(log.read_text(encoding="utf-8"), "sentinel\n")
            process.assert_not_called()

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_rejects_log_replacement_before_truncate(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-log-race-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            log = work / "build.log"
            log.write_text("sentinel\n", encoding="utf-8")
            external = root / "external.log"
            external.write_text("outside\n", encoding="utf-8")
            displaced = root / "displaced.log"
            real_identity = _RUN._real_file_identity
            replaced = False

            def replace_after_identity(
                path: Path, label: str
            ) -> tuple[int, ...]:
                nonlocal replaced
                identity = real_identity(path, label)
                if path == log and not replaced:
                    log.replace(displaced)
                    log.symlink_to(external)
                    replaced = True
                return identity

            with (
                mock.patch.object(
                    _RUN,
                    "_real_file_identity",
                    side_effect=replace_after_identity,
                ),
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT log changed before opening",
                ),
            ):
                _RUN._run_uat(root, Path("/uat"), ["probe"], log)

            self.assertEqual(external.read_text(encoding="utf-8"), "outside\n")
            self.assertEqual(
                displaced.read_text(encoding="utf-8"), "sentinel\n"
            )
            process.assert_not_called()

    def test_rejects_hard_linked_log_before_subprocess(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-log-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            outside = root / "outside.log"
            outside.write_text("sentinel\n", encoding="utf-8")
            log = work / "build.log"
            log.hardlink_to(outside)

            with (
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT log must have one filesystem link",
                ),
            ):
                _RUN._run_uat(root, Path("/uat"), ["probe"], log)

            self.assertEqual(outside.read_text(encoding="utf-8"), "sentinel\n")
            process.assert_not_called()

    def test_rejects_linked_automation_saved_before_subprocess(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-saved-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            automation = work / "automation-saved"
            automation.mkdir()
            original = _RUN._is_directory_link

            def report_automation_as_link(path: Path) -> bool:
                return path == automation or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_automation_as_link,
                ),
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT saved root must be a real directory",
                ),
            ):
                _RUN._run_uat(
                    root,
                    Path("/uat"),
                    ["probe"],
                    work / "build.log",
                )

            process.assert_not_called()

    def test_rejects_linked_ddc_before_subprocess(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-uat-ddc-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            ddc = work / "ddc"
            ddc.mkdir()
            original = _RUN._is_directory_link

            def report_ddc_as_link(path: Path) -> bool:
                return path == ddc or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_ddc_as_link,
                ),
                mock.patch.object(_RUN.subprocess, "run") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "UAT DDC root must be a real directory",
                ),
            ):
                _RUN._run_uat(
                    root,
                    Path("/uat"),
                    ["probe"],
                    work / "build.log",
                )

            process.assert_not_called()


class TurnkeyReportTests(unittest.TestCase):
    """Require Turnkey SDK evidence to remain a real cache-owned file."""

    def test_rejects_preexisting_linked_sdk_report_before_uat(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            report.write_text("sentinel\n", encoding="utf-8")
            target = _RUN._TARGETS_BY_ID["linux-x64"]
            original = Path.is_symlink

            def report_as_link(path: Path) -> bool:
                return path == report or original(path)

            with (
                mock.patch.object(Path, "is_symlink", report_as_link),
                mock.patch.object(_RUN, "_run_uat") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report must be a real file",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

            self.assertEqual(report.read_text(encoding="utf-8"), "sentinel\n")
            process.assert_not_called()

    def test_rejects_preexisting_hard_linked_sdk_report_before_uat(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            outside = root / "outside.txt"
            outside.write_text("sentinel\n", encoding="utf-8")
            report = work / "turnkey.txt"
            report.hardlink_to(outside)
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            with (
                mock.patch.object(_RUN, "_run_uat") as process,
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report must have one filesystem link",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

            self.assertEqual(outside.read_text(encoding="utf-8"), "sentinel\n")
            self.assertTrue(report.exists())
            process.assert_not_called()

    def test_rejects_hard_linked_sdk_report_after_uat(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            outside = root / "outside.txt"
            outside.write_text(
                "Linux: (Status=Valid, MinAllowed=0)\n",
                encoding="utf-8",
            )
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def write_hard_linked_report(
                *_args: object,
                **_kwargs: object,
            ) -> None:
                report.hardlink_to(outside)

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_hard_linked_report,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report must have one filesystem link",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_rejects_sdk_report_replacement_during_read(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-turnkey-report-race-"
        ) as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            external = root / "external.txt"
            displaced = root / "displaced.txt"
            target = _RUN._TARGETS_BY_ID["windows-x64"]
            external.write_text(
                "Win64: (Status=Valid, MinAllowed=0)\n",
                encoding="utf-8",
            )
            real_identity = _RUN._real_file_identity
            replaced = False

            def write_invalid_report(
                *_args: object,
                **_kwargs: object,
            ) -> None:
                report.write_text(
                    "Win64: (Status=Invalid,)\n",
                    encoding="utf-8",
                )

            def replace_after_identity(
                path: Path, label: str
            ) -> tuple[int, ...]:
                nonlocal replaced
                identity = real_identity(path, label)
                if path == report and not replaced:
                    report.replace(displaced)
                    report.symlink_to(external)
                    replaced = True
                return identity

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_invalid_report,
                ),
                mock.patch.object(
                    _RUN,
                    "_real_file_identity",
                    side_effect=replace_after_identity,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report changed while reading",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

            self.assertTrue(report.is_symlink())
            self.assertEqual(
                displaced.read_text(encoding="utf-8"),
                "Win64: (Status=Invalid,)\n",
            )
            self.assertEqual(
                external.read_text(encoding="utf-8"),
                "Win64: (Status=Valid, MinAllowed=0)\n",
            )

    def test_accepts_exact_platform_sdk_row(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def write_valid_report(*_args: object, **_kwargs: object) -> None:
                report.write_text(
                    "Turnkey report\nLinux: (Status=Valid, MinAllowed=0)\n",
                    encoding="utf-8",
                )

            with mock.patch.object(
                _RUN,
                "_run_uat",
                side_effect=write_valid_report,
            ):
                actual = _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

            self.assertEqual(actual, report)

    def test_rejects_prefixed_platform_sdk_row(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def write_wrong_platform(*_args: object, **_kwargs: object) -> None:
                report.write_text(
                    "FakeLinux: (Status=Valid, MinAllowed=0)\n",
                    encoding="utf-8",
                )

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_wrong_platform,
                ),
                self.assertRaisesRegex(_RUN.RunFailure, "SDK is invalid"),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

    def test_rejects_non_utf8_sdk_report(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            target = _RUN._TARGETS_BY_ID["linux-x64"]

            def write_non_utf8_report(
                *_args: object,
                **_kwargs: object,
            ) -> None:
                report.write_bytes(bytes([255, 254, 253]))

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_non_utf8_report,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "cannot read Turnkey SDK report",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )

    def test_rejects_linked_sdk_report_after_uat(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-turnkey-report-") as raw:
            root = Path(raw)
            work = root / "work"
            work.mkdir()
            report = work / "turnkey.txt"
            target = _RUN._TARGETS_BY_ID["linux-x64"]
            original = Path.is_symlink

            def write_linked_report(*_args: object, **_kwargs: object) -> None:
                report.write_text(
                    "Linux: (Status=Valid, MinAllowed=0)\n",
                    encoding="utf-8",
                )

            def report_as_link(path: Path) -> bool:
                return path == report or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_linked_report,
                ),
                mock.patch.object(Path, "is_symlink", report_as_link),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Turnkey SDK report must be a real file",
                ),
            ):
                _RUN._verify_sdk(
                    root,
                    Path("/uat"),
                    Path("/project"),
                    target,
                    work,
                )


class PublicationTransactionTests(unittest.TestCase):
    """Keep malformed existing publications unchanged on rejection."""

    def test_rejects_existing_file_before_publication_mutation(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-publish-file-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            candidate.mkdir()
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            destination = root / "dist/linux-x64"
            destination.parent.mkdir()
            destination.write_text("old", encoding="utf-8")
            backup = destination.with_name(".linux-x64.previous")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "published target must be a real directory",
            ):
                _RUN._publish(candidate, destination)

            self.assertEqual(destination.read_text(encoding="utf-8"), "old")
            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse(backup.exists())

    def test_invalid_candidate_preserves_publication_state(self) -> None:
        for stale_backup in (False, True):
            with (
                self.subTest(stale_backup=stale_backup),
                tempfile.TemporaryDirectory(
                    prefix="shar-publish-invalid-candidate-"
                ) as raw,
            ):
                root = Path(raw)
                candidate = root / "candidate"
                candidate.mkdir()
                runtime = candidate / "shar"
                runtime.write_bytes(b"not-elf")
                if _RUN.os.name != "nt":
                    runtime.chmod(0o755)
                destination = root / "dist/linux-x64"
                backup = destination.with_name(".linux-x64.previous")
                if stale_backup:
                    destination.parent.mkdir()
                    backup.mkdir()
                    (backup / "old.txt").write_text("old", encoding="utf-8")

                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._publish(
                        candidate,
                        destination,
                        _RUN._TARGETS_BY_ID["linux-x64"],
                    )

                self.assertTrue(runtime.is_file())
                self.assertFalse(destination.exists())
                if stale_backup:
                    self.assertEqual(
                        (backup / "old.txt").read_text(encoding="utf-8"),
                        "old",
                    )
                else:
                    self.assertFalse(destination.parent.exists())

    def test_rejects_linked_dist_root_before_candidate_moves(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-publish-link-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            candidate.mkdir()
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            dist = root / "dist"
            dist.mkdir()
            destination = dist / "linux-x64"
            original = _RUN._is_directory_link

            def report_dist_as_link(path: Path) -> bool:
                return path == dist or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_dist_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "publication root must be a real directory",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse(destination.exists())

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_rejects_dist_root_replacement_before_swap(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-publish-root-race-"
        ) as raw:
            root = Path(raw)
            candidate = root / "candidate"
            candidate.mkdir()
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            dist = root / "dist"
            dist.mkdir()
            destination = dist / "linux-x64"
            displaced = root / "displaced-dist"
            outside = root / "outside"
            outside.mkdir()
            real_validate = _RUN._validate_publication_candidate

            def redirect_after_validation(
                package: Path,
                target: object,
            ) -> object:
                tree = real_validate(package, target)
                dist.replace(displaced)
                dist.symlink_to(outside, target_is_directory=True)
                return tree

            with (
                mock.patch.object(
                    _RUN,
                    "_validate_publication_candidate",
                    side_effect=redirect_after_validation,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "publication root changed before publication",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse((outside / "linux-x64").exists())
            self.assertTrue(dist.is_symlink())
            self.assertTrue(displaced.is_dir())

    def test_rejects_candidate_drift_before_candidate_moves(self) -> None:
        for replacement in ("directory", "runtime"):
            with (
                self.subTest(replacement=replacement),
                tempfile.TemporaryDirectory(
                    prefix="shar-publish-candidate-race-"
                ) as raw,
            ):
                root = Path(raw)
                candidate = root / "candidate"
                candidate.mkdir()
                runtime = candidate / "shar"
                runtime.write_bytes(_synthetic_elf(0x003E))
                if _RUN.os.name != "nt":
                    runtime.chmod(0o755)
                destination = root / "dist/linux-x64"
                destination.parent.mkdir()
                displaced = root / "displaced-candidate"
                real_state = _RUN._require_publication_destination_state

                injected = (
                    replacement,
                    candidate,
                    displaced,
                    runtime,
                    real_state,
                )

                def replace_after_validation(
                    target_path: Path,
                    expected: tuple[int, int, int] | None,
                    state: tuple[
                        str,
                        Path,
                        Path,
                        Path,
                        Callable[[Path, tuple[int, int, int] | None], None],
                    ] = injected,
                ) -> None:
                    replace_kind, package, stale, binary, check_state = state
                    check_state(target_path, expected)
                    if replace_kind == "directory":
                        package.replace(stale)
                        package.mkdir()
                        (package / "unvalidated.txt").write_text(
                            "replacement", encoding="utf-8"
                        )
                    else:
                        binary.unlink()
                        binary.write_text("replacement", encoding="utf-8")

                with (
                    mock.patch.object(
                        _RUN,
                        "_require_publication_destination_state",
                        side_effect=replace_after_validation,
                    ),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "candidate package changed before publication",
                    ),
                ):
                    _RUN._publish(
                        candidate,
                        destination,
                        _RUN._TARGETS_BY_ID["linux-x64"],
                    )

                self.assertFalse(destination.exists())
                if replacement == "directory":
                    self.assertTrue((displaced / "shar").is_file())
                    self.assertTrue((candidate / "unvalidated.txt").is_file())
                else:
                    self.assertEqual(
                        runtime.read_text(encoding="utf-8"),
                        "replacement",
                    )

    def test_rejects_destination_drift_before_candidate_moves(self) -> None:
        for initially_present in (False, True):
            with (
                self.subTest(initially_present=initially_present),
                tempfile.TemporaryDirectory(
                    prefix="shar-publish-target-race-"
                ) as raw,
            ):
                root = Path(raw)
                candidate = root / "candidate"
                candidate.mkdir()
                (candidate / "new.txt").write_text(
                    "new", encoding="utf-8"
                )
                destination = root / "dist/linux-x64"
                destination.parent.mkdir()
                displaced = root / "displaced-target"
                if initially_present:
                    destination.mkdir()
                    (destination / "old.txt").write_text(
                        "old", encoding="utf-8"
                    )
                real_validate = _RUN._validate_publication_candidate

                def replace_after_validation(
                    package: Path,
                    target: object,
                    *,
                    present: bool = initially_present,
                    target_path: Path = destination,
                    displaced_path: Path = displaced,
                    validate: Callable[[Path, object], object] = real_validate,
                ) -> object:
                    tree = validate(package, target)
                    if present:
                        target_path.replace(displaced_path)
                    target_path.mkdir()
                    (target_path / "intruder.txt").write_text(
                        "intruder", encoding="utf-8"
                    )
                    return tree

                with (
                    mock.patch.object(
                        _RUN,
                        "_validate_publication_candidate",
                        side_effect=replace_after_validation,
                    ),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "published target changed before publication",
                    ),
                ):
                    _RUN._publish(candidate, destination)

                self.assertTrue((candidate / "new.txt").is_file())
                self.assertTrue((destination / "intruder.txt").is_file())
                if initially_present:
                    self.assertTrue((displaced / "old.txt").is_file())

    def test_rejects_broken_link_backup_before_candidate_moves(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-backup-link-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            destination = root / "dist/linux-x64"
            candidate.mkdir()
            destination.parent.mkdir()
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            backup = destination.with_name(".linux-x64.previous")
            backup.mkdir()
            original_exists = Path.exists
            original_link = _RUN._is_directory_link

            def report_backup_missing(path: Path) -> bool:
                if path == backup:
                    return False
                return original_exists(path)

            def report_backup_as_link(path: Path) -> bool:
                return path == backup or original_link(path)

            with (
                mock.patch.object(Path, "exists", report_backup_missing),
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_backup_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "publication backup must be a real directory",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse(destination.exists())
            self.assertTrue(backup.is_dir())

    def test_rejects_backup_replacement_before_cleanup(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-publish-backup-race-"
        ) as raw:
            root = Path(raw)
            candidate = root / "candidate"
            candidate.mkdir()
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            destination = root / "dist/linux-x64"
            destination.parent.mkdir()
            backup = destination.with_name(".linux-x64.previous")
            backup.mkdir()
            (backup / "old.txt").write_text("old", encoding="utf-8")
            displaced = root / "displaced-backup"
            real_validate = _RUN._validate_publication_candidate

            def replace_after_validation(
                package: Path,
                target: object,
            ) -> object:
                snapshot = real_validate(package, target)
                backup.replace(displaced)
                backup.mkdir()
                (backup / "intruder.txt").write_text(
                    "intruder", encoding="utf-8"
                )
                return snapshot

            with (
                mock.patch.object(
                    _RUN,
                    "_validate_publication_candidate",
                    side_effect=replace_after_validation,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "publication backup changed before publication",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse(destination.exists())
            self.assertEqual(
                (displaced / "old.txt").read_text(encoding="utf-8"),
                "old",
            )
            self.assertEqual(
                (backup / "intruder.txt").read_text(encoding="utf-8"),
                "intruder",
            )

    def test_cleanup_failure_rolls_back_publication_swap(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-publish-rollback-",
        ) as raw:
            root = Path(raw)
            candidate = root / "candidate"
            destination = root / "dist/linux-x64"
            candidate.mkdir()
            destination.mkdir(parents=True)
            (candidate / "new.txt").write_text("new", encoding="utf-8")
            (destination / "old.txt").write_text("old", encoding="utf-8")
            backup = destination.with_name(".linux-x64.previous")
            original = _RUN.shutil.rmtree

            def fail_backup_cleanup(
                path: Path,
                *args: object,
                **kwargs: object,
            ) -> None:
                if Path(path) == backup:
                    raise OSError("injected backup cleanup failure")
                original(path, *args, **kwargs)

            with (
                mock.patch.object(
                    _RUN.shutil,
                    "rmtree",
                    side_effect=fail_backup_cleanup,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "publication cleanup failed",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((destination / "old.txt").is_file())
            self.assertFalse((destination / "new.txt").exists())
            self.assertTrue((candidate / "new.txt").is_file())
            self.assertFalse(backup.exists())

    def test_rejects_hard_linked_candidate_file_before_publication(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-candidate-hard-link-",
        ) as raw:
            root = Path(raw)
            outside = root / "outside.bin"
            outside.write_bytes(b"runtime")
            candidate = root / "candidate"
            candidate.mkdir()
            linked = candidate / "runtime.bin"
            linked.hardlink_to(outside)
            destination = root / "dist/linux-x64"

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "candidate package contains a hard-linked file",
            ):
                _RUN._publish(candidate, destination)

            self.assertEqual(outside.read_bytes(), b"runtime")
            self.assertTrue(linked.is_file())
            self.assertFalse(destination.exists())

    def test_rejects_linked_candidate_before_publication(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-link-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            candidate.mkdir()
            (candidate / "runtime.bin").write_bytes(b"runtime")
            destination = root / "dist/linux-x64"
            original = _RUN._is_directory_link

            def report_candidate_as_link(path: Path) -> bool:
                return path == candidate or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_candidate_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "candidate package must be a real directory",
                ),
            ):
                _RUN._publish(candidate, destination)

            self.assertTrue((candidate / "runtime.bin").is_file())
            self.assertFalse(destination.exists())


class PublicationArtifactTests(unittest.TestCase):
    """Keep runtime publication separate from cached build diagnostics."""

    def test_windows_publication_caches_manifests_and_pdbs(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-publication-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            binary = candidate / "shar/Binaries/Win64/shar.exe"
            symbols = candidate / "shar/Binaries/Win64/shar.pdb"
            binary.parent.mkdir(parents=True)
            binary.write_bytes(b"runtime")
            symbols.write_bytes(b"symbols")
            (candidate / "Manifest_UFSFiles_Win64.txt").write_text(
                "manifest\n",
                encoding="utf-8",
            )
            stale = work / "symbols/stale.pdb"
            stale.parent.mkdir(parents=True)
            stale.write_bytes(b"stale")

            target = _RUN._TARGETS_BY_ID["windows-x64"]
            _RUN._cache_nonruntime_artifacts(candidate, work, target)

            self.assertEqual(binary.read_bytes(), b"runtime")
            self.assertFalse(symbols.exists())
            self.assertFalse(
                (candidate / "Manifest_UFSFiles_Win64.txt").exists()
            )
            self.assertEqual(
                (work / "symbols/shar/Binaries/Win64/shar.pdb").read_bytes(),
                b"symbols",
            )
            self.assertEqual(
                (
                    work / "publication-metadata/Manifest_UFSFiles_Win64.txt"
                ).read_text(encoding="utf-8"),
                "manifest\n",
            )
            self.assertFalse(stale.exists())


class ArtifactCacheEntryTests(unittest.TestCase):
    """Require cached packaging diagnostics to be regular files."""

    def test_rejects_manifest_named_directory(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-entry-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            candidate.mkdir()
            (candidate / "Manifest_UFSFiles_Linux.txt").mkdir()

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "packaging manifest must be a real file",
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

    def test_rejects_malformed_manifest_before_moving_valid_one(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-entry-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            candidate.mkdir()
            valid = candidate / "Manifest_A.txt"
            invalid = candidate / "Manifest_B.txt"
            valid.write_text("valid\n", encoding="utf-8")
            invalid.mkdir()
            cached = work / "publication-metadata/Manifest_Previous.txt"
            cached.parent.mkdir(parents=True)
            cached.write_text("previous\n", encoding="utf-8")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "packaging manifest must be a real file",
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

            self.assertTrue(valid.is_file())
            self.assertTrue(invalid.is_dir())
            self.assertEqual(
                cached.read_text(encoding="utf-8"),
                "previous\n",
            )

    def test_rejects_pdb_named_directory(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-entry-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            symbol = candidate / "shar/Binaries/Win64/shar.pdb"
            symbol.mkdir(parents=True)

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "debug symbol must be a real file",
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_malformed_pdb_before_moving_valid_one(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-entry-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            valid = candidate / "shar/Binaries/Win64/a.pdb"
            invalid = candidate / "shar/Binaries/Win64/b.pdb"
            valid.parent.mkdir(parents=True)
            valid.write_bytes(b"symbols")
            invalid.mkdir()
            cached = work / "symbols/shar/Binaries/Win64/previous.pdb"
            cached.parent.mkdir(parents=True)
            cached.write_bytes(b"previous")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "debug symbol must be a real file",
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            self.assertTrue(valid.is_file())
            self.assertTrue(invalid.is_dir())
            self.assertEqual(cached.read_bytes(), b"previous")

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_manifest_replacement_before_snapshot_preserves_cache(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-race-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            candidate.mkdir()
            manifest = candidate / "Manifest_UFSFiles_Linux.txt"
            manifest.write_text("local manifest\n", encoding="utf-8")
            displaced = root / "displaced-manifest.txt"
            external = root / "external-manifest.txt"
            external.write_text("external manifest\n", encoding="utf-8")
            work = root / "work"
            cached = work / "publication-metadata/Manifest_Previous.txt"
            cached.parent.mkdir(parents=True)
            cached.write_text("previous manifest\n", encoding="utf-8")
            real_require = _RUN._require_real_file
            replaced = False

            def replace_after_file_check(path: Path, label: str) -> None:
                nonlocal replaced
                real_require(path, label)
                if path == manifest and not replaced:
                    manifest.replace(displaced)
                    manifest.symlink_to(external)
                    replaced = True

            with (
                mock.patch.object(
                    _RUN,
                    "_require_real_file",
                    side_effect=replace_after_file_check,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "packaging manifest changed while reading",
                ),
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

            self.assertEqual(
                cached.read_text(encoding="utf-8"),
                "previous manifest\n",
            )
            self.assertTrue(manifest.is_symlink())
            self.assertEqual(
                displaced.read_text(encoding="utf-8"),
                "local manifest\n",
            )
            self.assertEqual(
                external.read_text(encoding="utf-8"),
                "external manifest\n",
            )

    def test_scan_failure_preserves_existing_symbol_cache(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-entry-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            nested = candidate / "shar/Binaries/Win64"
            nested.mkdir(parents=True)
            (nested / "shar.pdb").write_bytes(b"new symbols")
            work = root / "work"
            cached = work / "symbols/shar/Binaries/Win64/shar.pdb"
            cached.parent.mkdir(parents=True)
            cached.write_bytes(b"previous symbols")
            original = Path.iterdir

            def fail_nested_scan(path: Path) -> Iterator[Path]:
                if path == nested:
                    raise PermissionError("injected candidate scan failure")
                return original(path)

            with (
                mock.patch.object(Path, "iterdir", fail_nested_scan),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "candidate package could not be scanned",
                ),
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            self.assertEqual(cached.read_bytes(), b"previous symbols")


class ArtifactCachePathTests(unittest.TestCase):
    """Reject redirected metadata and symbol cache roots before moving files."""

    def _assert_linked_cache_rejected(
        self,
        target_id: str,
        cache_name: str,
        source_relative: str,
        label: str,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-artifact-cache-") as raw:
            root = Path(raw)
            candidate = root / "candidate"
            work = root / "work"
            candidate.mkdir()
            work.mkdir()
            source = candidate / source_relative
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_bytes(b"artifact")
            cache = work / cache_name
            cache.mkdir()
            original = _RUN._is_directory_link

            def report_cache_as_link(path: Path) -> bool:
                return path == cache or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_cache_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    f"{label} must be a real directory",
                ),
            ):
                _RUN._cache_nonruntime_artifacts(
                    candidate,
                    work,
                    _RUN._TARGETS_BY_ID[target_id],
                )

            self.assertTrue(source.is_file())
            self.assertTrue(cache.is_dir())

    def test_rejects_linked_publication_metadata_cache(self) -> None:
        self._assert_linked_cache_rejected(
            "linux-x64",
            "publication-metadata",
            "Manifest_UFSFiles_Linux.txt",
            "publication metadata cache",
        )

    def test_rejects_linked_symbol_cache(self) -> None:
        self._assert_linked_cache_rejected(
            "windows-x64",
            "symbols",
            "shar/Binaries/Win64/shar.pdb",
            "symbol cache",
        )


class CandidateTreeTests(unittest.TestCase):
    """Require packaged candidates to remain self-contained real trees."""

    def test_rejects_hard_linked_file(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-tree-") as raw:
            candidate = Path(raw) / "candidate"
            candidate.mkdir()
            source = candidate / "runtime.bin"
            source.write_bytes(b"fixture")
            alias = candidate / "runtime-copy.bin"
            alias.hardlink_to(source)

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "candidate package contains a hard-linked file",
            ):
                _RUN._validate_candidate_tree(candidate)

    def test_rejects_nested_link_without_following_it(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-tree-") as raw:
            candidate = Path(raw) / "candidate"
            nested = candidate / "nested"
            nested.mkdir(parents=True)
            linked = nested / "runtime.bin"
            linked.write_bytes(b"fixture")
            original = _RUN._is_directory_link

            def report_runtime_as_link(path: Path) -> bool:
                return path == linked or original(path)

            with (
                mock.patch.object(
                    _RUN,
                    "_is_directory_link",
                    side_effect=report_runtime_as_link,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "candidate package contains a linked entry",
                ),
            ):
                _RUN._validate_candidate_tree(candidate)

    def test_rejects_entry_metadata_failure(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-tree-") as raw:
            candidate = Path(raw) / "candidate"
            candidate.mkdir()
            runtime = candidate / "runtime.bin"
            runtime.write_bytes(b"fixture")
            original = Path.is_file

            def fail_runtime_metadata(path: Path) -> bool:
                if path == runtime:
                    raise PermissionError("injected candidate metadata failure")
                return original(path)

            with (
                mock.patch.object(Path, "is_file", fail_runtime_metadata),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "candidate package entry could not be inspected",
                ),
            ):
                _RUN._validate_candidate_tree(candidate)

    def test_rejects_nested_scan_failure(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-tree-") as raw:
            candidate = Path(raw) / "candidate"
            nested = candidate / "nested"
            nested.mkdir(parents=True)
            runtime = nested / "shar-Linux-Shipping"
            runtime.write_bytes(_synthetic_elf(0x003E))
            runtime.chmod(0o755)
            original = Path.iterdir

            def fail_nested_scan(path: Path) -> Iterator[Path]:
                if path == nested:
                    raise PermissionError("injected candidate scan failure")
                return original(path)

            with (
                mock.patch.object(Path, "iterdir", fail_nested_scan),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "candidate package could not be scanned",
                ),
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

    def test_build_validates_candidate_tree_before_artifact_caching(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-candidate-order-") as raw:
            root = Path(raw)
            target = _RUN._TARGETS_BY_ID["linux-x64"]
            with (
                mock.patch.object(_RUN, "_verify_sdk"),
                mock.patch.object(_RUN, "_run_uat"),
                mock.patch.object(
                    _RUN,
                    "_validate_candidate_tree",
                    side_effect=_RUN.RunFailure("candidate tree drift"),
                ),
                mock.patch.object(
                    _RUN,
                    "_cache_nonruntime_artifacts",
                ) as cache_artifacts,
                self.assertRaisesRegex(_RUN.RunFailure, "candidate tree drift"),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project/shar.uproject"),
                    target,
                    validate_only=False,
                )
            cache_artifacts.assert_not_called()


class PeOptionalHeaderTests(unittest.TestCase):
    """Require Windows PE32+ loader alignment fields to be coherent."""

    def test_rejects_32_bit_machine_flag_for_64_bit_targets(self) -> None:
        offset = 0x80
        file_size = 0x400
        for machine in (0x8664, 0xAA64):
            for characteristics, admitted in (
                (0x0002, True),
                (0x0022, True),
                (0x0102, False),
            ):
                coff = bytearray(20)
                coff[:2] = machine.to_bytes(2, "little")
                coff[2:4] = (1).to_bytes(2, "little")
                coff[16:18] = (112).to_bytes(2, "little")
                coff[18:20] = characteristics.to_bytes(2, "little")
                with self.subTest(
                    machine=hex(machine),
                    characteristics=hex(characteristics),
                ):
                    actual = _RUN._pe_optional_layout(
                        bytes(coff),
                        machine,
                        offset,
                        file_size,
                    )
                    self.assertEqual(actual is not None, admitted)

    def test_rejects_obsolete_working_set_characteristic(self) -> None:
        offset = 0x80
        file_size = 0x400
        for characteristics, admitted in (
            (0x0002, True),
            (0x0022, True),
            (0x0012, False),
        ):
            coff = bytearray(20)
            coff[:2] = (0x8664).to_bytes(2, "little")
            coff[2:4] = (1).to_bytes(2, "little")
            coff[16:18] = (112).to_bytes(2, "little")
            coff[18:20] = characteristics.to_bytes(2, "little")
            with self.subTest(characteristics=hex(characteristics)):
                actual = _RUN._pe_optional_layout(
                    bytes(coff),
                    0x8664,
                    offset,
                    file_size,
                )
                self.assertEqual(actual is not None, admitted)

    def test_rejects_deprecated_coff_symbol_table_metadata(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-windows-header-",
        ) as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            for reason, offset in (
                ("symbol-table-pointer", 0x8C),
                ("symbol-count", 0x90),
            ):
                payload = bytearray(_synthetic_pe(0x8664))
                payload[offset : offset + 4] = (1).to_bytes(4, "little")
                executable.write_bytes(payload)
                with (
                    self.subTest(reason=reason),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "Windows SHAR executable",
                    ),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["windows-x64"],
                    )

    def test_rejects_nonempty_data_directory_at_zero_address(self) -> None:
        cases = ((0, 1, 120), (4, 5, 152))
        for index, count, optional_size in cases:
            with self.subTest(index=index):
                optional = bytearray(optional_size)
                optional[56:60] = (0x2000).to_bytes(4, "little")
                optional[108:112] = count.to_bytes(4, "little")
                start = 112 + (8 * index)
                optional[start + 4 : start + 8] = (1).to_bytes(4, "little")
                self.assertFalse(
                    _RUN._pe_data_directories_are_valid(
                        bytes(optional),
                        0x4000,
                    )
                )

    def test_rejects_misaligned_certificate_table(self) -> None:
        for reason, address, size, admitted in (
            ("valid", 0x200, 8, True),
            ("offset", 0x201, 8, False),
            ("size", 0x200, 7, False),
        ):
            optional = bytearray(152)
            optional[56:60] = (0x2000).to_bytes(4, "little")
            optional[108:112] = (5).to_bytes(4, "little")
            start = 112 + (4 * 8)
            optional[start : start + 4] = address.to_bytes(4, "little")
            optional[start + 4 : start + 8] = size.to_bytes(4, "little")
            with self.subTest(reason=reason):
                self.assertEqual(
                    _RUN._pe_data_directories_are_valid(
                        bytes(optional),
                        0x4000,
                    ),
                    admitted,
                )

    def test_rejects_reserved_data_directory_metadata(self) -> None:
        cases = (
            ("architecture-rva", 7, 0x1000, 0),
            ("architecture-size", 7, 0, 8),
            ("global-pointer-size", 8, 0x1000, 8),
            ("reserved-rva", 15, 0x1000, 0),
            ("reserved-size", 15, 0, 8),
        )
        for reason, index, address, size in cases:
            optional = bytearray(112 + (16 * 8))
            optional[56:60] = (0x3000).to_bytes(4, "little")
            optional[108:112] = (16).to_bytes(4, "little")
            start = 112 + (index * 8)
            optional[start : start + 4] = address.to_bytes(4, "little")
            optional[start + 4 : start + 8] = size.to_bytes(4, "little")
            with self.subTest(reason=reason):
                self.assertFalse(
                    _RUN._pe_data_directories_are_valid(
                        bytes(optional),
                        0x4000,
                    )
                )

        optional = bytearray(112 + (16 * 8))
        optional[56:60] = (0x3000).to_bytes(4, "little")
        optional[108:112] = (16).to_bytes(4, "little")
        global_pointer = 112 + (8 * 8)
        optional[global_pointer : global_pointer + 4] = (0x1000).to_bytes(
            4,
            "little",
        )
        self.assertTrue(
            _RUN._pe_data_directories_are_valid(bytes(optional), 0x4000)
        )

    def test_rejects_data_directory_outside_image(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-header-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            payload = bytearray(_synthetic_pe(0x8664))
            section = bytes(payload[0x108:0x130])
            payload[0x110:0x138] = section
            payload[0x108:0x110] = b"\0" * 8
            payload[0x94:0x96] = (120).to_bytes(2, "little")
            payload[0x104:0x108] = (1).to_bytes(4, "little")
            payload[0x108:0x10C] = (0x2000).to_bytes(4, "little")
            payload[0x10C:0x110] = (1).to_bytes(4, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_reserved_dll_characteristic_bits(self) -> None:
        for value, admitted in (
            (0x0001, False),
            (0x0002, False),
            (0x0004, False),
            (0x0008, False),
            (0x0100, True),
        ):
            optional = bytearray(112)
            optional[24:32] = (0x140000000).to_bytes(8, "little")
            optional[32:36] = (0x1000).to_bytes(4, "little")
            optional[36:40] = (0x200).to_bytes(4, "little")
            optional[70:72] = value.to_bytes(2, "little")
            with self.subTest(value=hex(value)):
                self.assertEqual(
                    _RUN._pe_optional_fields_are_valid(bytes(optional)),
                    admitted,
                )

    def test_rejects_misaligned_image_base(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-windows-alignment-",
        ) as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            payload = bytearray(_synthetic_pe(0x8664))
            payload[0xB0:0xB8] = (1).to_bytes(8, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_malformed_loader_fields(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-windows-header-",
        ) as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            cases = (
                ("win32-version", 0xCC, 4, 1),
                ("image-size", 0xD0, 4, 1),
                ("image-size-zero", 0xD0, 4, 0),
                ("header-size", 0xD4, 4, 1),
                ("header-size-zero", 0xD4, 4, 0),
                ("loader-flags", 0x100, 4, 1),
                ("directory-count", 0x104, 4, 1),
            )
            for reason, offset, width, value in cases:
                payload = bytearray(_synthetic_pe(0x8664))
                payload[offset : offset + width] = value.to_bytes(
                    width,
                    "little",
                )
                executable.write_bytes(payload)
                with (
                    self.subTest(reason=reason),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "Windows SHAR executable",
                    ),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["windows-x64"],
                    )

    def test_limits_pe32_plus_image_size_to_two_gib(self) -> None:
        base = bytearray(112)
        base[32:36] = (0x1000).to_bytes(4, "little")
        base[36:40] = (0x200).to_bytes(4, "little")
        base[60:64] = (0x200).to_bytes(4, "little")
        for image_size, admitted in (
            (0x2000, True),
            (0x80000000, True),
            (0x80001000, False),
            (0xFFFFF000, False),
        ):
            optional = bytearray(base)
            optional[56:60] = image_size.to_bytes(4, "little")
            with self.subTest(image_size=hex(image_size)):
                self.assertEqual(
                    _RUN._pe_loader_fields_are_valid(
                        bytes(optional),
                        header_end=0x180,
                        file_size=0x1000,
                    ),
                    admitted,
                )

    def test_rejects_misaligned_section_virtual_address(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-windows-alignment-",
        ) as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            payload = bytearray(_synthetic_pe(0x8664))
            payload[0xA8:0xAC] = (0x1001).to_bytes(4, "little")
            payload[0x114:0x118] = (0x1001).to_bytes(4, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_invalid_loader_alignments(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-windows-alignment-",
        ) as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            for section_alignment, file_alignment in (
                (0x100, 0x200),
                (0x400, 0x200),
                (0x1000, 0x201),
            ):
                payload = bytearray(_synthetic_pe(0x8664))
                payload[0xB8:0xBC] = section_alignment.to_bytes(4, "little")
                payload[0xBC:0xC0] = file_alignment.to_bytes(4, "little")
                executable.write_bytes(payload)
                with (
                    self.subTest(
                        section_alignment=section_alignment,
                        file_alignment=file_alignment,
                    ),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "Windows SHAR executable",
                    ),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["windows-x64"],
                    )


class PeSectionLayoutTests(unittest.TestCase):
    """Require PE section file layout to follow loader alignment."""

    def test_rejects_virtual_section_overlapping_headers(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-section-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            payload = bytearray(_synthetic_pe(0x8664))
            payload[0x110:0x114] = (0x1001).to_bytes(4, "little")
            payload[0x114:0x118] = (0).to_bytes(4, "little")
            payload[0xA8:0xAC] = (1).to_bytes(4, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_raw_section_overlapping_headers(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-section-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            executable.write_bytes(
                _synthetic_pe(0x8664, section_raw_offset=0)
            )
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_object_only_section_metadata(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-section-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            for reason, offset, width in (
                ("relocation-pointer", 0x120, 4),
                ("line-number-pointer", 0x124, 4),
                ("relocation-count", 0x128, 2),
                ("line-number-count", 0x12A, 2),
            ):
                payload = bytearray(_synthetic_pe(0x8664))
                payload[offset : offset + width] = (1).to_bytes(width, "little")
                executable.write_bytes(payload)
                with (
                    self.subTest(reason=reason),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "Windows SHAR executable",
                    ),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["windows-x64"],
                    )

    def test_rejects_object_group_section_names(self) -> None:
        baseline = bytearray(_synthetic_pe(0x8664))
        section = bytearray(baseline[0x108:0x130])
        for name, admitted in (
            (b".text\0\0\0", True),
            (b".text$X\0", False),
            (b"$bad\0\0\0\0", False),
            (b"\0" * 8, True),
        ):
            candidate = bytearray(section)
            candidate[:8] = name
            with self.subTest(name=name):
                self.assertEqual(
                    _RUN._pe_section_image_metadata_is_valid(bytes(candidate)),
                    admitted,
                )

    def test_rejects_object_only_section_characteristics(self) -> None:
        object_only_flags = (
            0x00000008,
            0x00000200,
            0x00000800,
            0x00001000,
            0x00100000,
            0x00E00000,
            0x01000000,
        )
        baseline = bytearray(_synthetic_pe(0x8664))
        section = bytearray(baseline[0x108:0x130])
        original = int.from_bytes(section[36:40], "little")
        for flag in object_only_flags:
            candidate = bytearray(section)
            candidate[36:40] = (original | flag).to_bytes(4, "little")
            with self.subTest(flag=hex(flag)):
                self.assertFalse(
                    _RUN._pe_section_image_metadata_is_valid(bytes(candidate))
                )

        section[36:40] = (original | 0x02000000).to_bytes(4, "little")
        self.assertTrue(
            _RUN._pe_section_image_metadata_is_valid(bytes(section))
        )

    def test_rejects_nonadjacent_section_virtual_addresses(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-section-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            payload = bytearray(_synthetic_pe(0x8664, section_count=2))
            payload[0x138:0x13C] = (1).to_bytes(4, "little")
            payload[0x13C:0x140] = (0x3000).to_bytes(4, "little")
            payload[0x154:0x158] = (0x40000040).to_bytes(4, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_raw_sections_out_of_virtual_address_order(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-section-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            payload = bytearray(_synthetic_pe(0x8664, section_count=2))
            payload.extend(b"\0" * 0x200)
            payload[0x11C:0x120] = (0x400).to_bytes(4, "little")
            payload[0x138:0x13C] = (1).to_bytes(4, "little")
            payload[0x13C:0x140] = (0x2000).to_bytes(4, "little")
            payload[0x140:0x144] = (0x200).to_bytes(4, "little")
            payload[0x144:0x148] = (0x200).to_bytes(4, "little")
            payload[0x154:0x158] = (0x40000040).to_bytes(4, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_low_alignment_requires_matching_raw_offset(self) -> None:
        for raw_offset, admitted in ((0x200, False), (0x1000, True)):
            payload = bytearray(
                _synthetic_pe(
                    0x8664,
                    section_raw_offset=raw_offset,
                )
            )
            required = raw_offset + 0x200
            if len(payload) < required:
                payload.extend(b"\0" * (required - len(payload)))
            payload[0xB8:0xBC] = (0x200).to_bytes(4, "little")
            payload[0xBC:0xC0] = (0x200).to_bytes(4, "little")
            payload[0xD0:0xD4] = (0x1200).to_bytes(4, "little")
            stream = io.BytesIO(payload)
            prefix = stream.read(4)
            with self.subTest(raw_offset=hex(raw_offset)):
                self.assertEqual(
                    _RUN._matches_pe(
                        stream,
                        prefix,
                        "amd64",
                        len(payload),
                    ),
                    admitted,
                )

    def test_rejects_misaligned_raw_section_data(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-section-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar-Win64-Shipping.exe"
            for reason, offset in (("raw-size", 0x118), ("raw-offset", 0x11C)):
                payload = bytearray(_synthetic_pe(0x8664))
                payload[offset : offset + 4] = (1).to_bytes(4, "little")
                executable.write_bytes(payload)
                with (
                    self.subTest(reason=reason),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "Windows SHAR executable",
                    ),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["windows-x64"],
                    )


class PeEntrypointTests(unittest.TestCase):
    """Require Windows process entrypoints to resolve to file-backed code."""

    def test_rejects_descending_section_addresses(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-entry-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar/Binaries/Win64/shar.exe"
            executable.parent.mkdir(parents=True)
            payload = bytearray(_synthetic_pe(0x8664, section_count=2))
            payload[0xA8:0xAC] = (0x2000).to_bytes(4, "little")
            payload[0x114:0x118] = (0x2000).to_bytes(4, "little")
            payload[0x138:0x13C] = (1).to_bytes(4, "little")
            payload[0x13C:0x140] = (0x1000).to_bytes(4, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_entrypoint_in_raw_padding(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-entry-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar/Binaries/Win64/shar.exe"
            executable.parent.mkdir(parents=True)
            payload = bytearray(_synthetic_pe(0x8664))
            payload.extend(b"\0")
            payload[0xA8:0xAC] = (0x1001).to_bytes(4, "little")
            payload[0x118:0x11C] = (2).to_bytes(4, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_rejects_entrypoint_in_zero_fill_tail(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-windows-entry-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar/Binaries/Win64/shar.exe"
            executable.parent.mkdir(parents=True)
            payload = bytearray(_synthetic_pe(0x8664))
            payload[0xA8:0xAC] = (0x1001).to_bytes(4, "little")
            payload[0x110:0x114] = (2).to_bytes(4, "little")
            payload[0x118:0x11C] = (1).to_bytes(4, "little")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )


class ElfEntrypointTests(unittest.TestCase):
    """Require Linux process entrypoints to resolve to file-backed code."""

    def test_rejects_big_endian_supported_images(self) -> None:
        for target_id, platform, machine in (
            ("linux-x64", "Linux", 0x003E),
            ("linux-arm64", "LinuxArm64", 0x00B7),
        ):
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(prefix="shar-linux-entry-") as raw,
            ):
                candidate = Path(raw)
                executable = (
                    candidate
                    / "shar"
                    / "Binaries"
                    / platform
                    / f"shar-{platform}-Shipping"
                )
                executable.parent.mkdir(parents=True)
                executable.write_bytes(_synthetic_big_endian_elf(machine))
                if _RUN.os.name != "nt":
                    executable.chmod(0o755)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

    def test_rejects_supported_architecture_processor_flags(self) -> None:
        for target_id, platform, machine in (
            ("linux-x64", "Linux", 0x003E),
            ("linux-arm64", "LinuxArm64", 0x00B7),
        ):
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(prefix="shar-linux-entry-") as raw,
            ):
                candidate = Path(raw)
                executable = (
                    candidate
                    / "shar"
                    / "Binaries"
                    / platform
                    / f"shar-{platform}-Shipping"
                )
                executable.parent.mkdir(parents=True)
                payload = bytearray(_synthetic_elf(machine))
                payload[48:52] = (1).to_bytes(4, "little")
                executable.write_bytes(payload)
                if _RUN.os.name != "nt":
                    executable.chmod(0o755)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

    def test_rejects_nonzero_ident_reserved_bytes(self) -> None:
        for reason, offset in (
            ("abi-version", 8),
            ("padding-first", 9),
            ("padding-last", 15),
        ):
            with (
                self.subTest(reason=reason),
                tempfile.TemporaryDirectory(prefix="shar-linux-entry-") as raw,
            ):
                candidate = Path(raw)
                executable = (
                    candidate / "shar/Binaries/Linux/shar-Linux-Shipping"
                )
                executable.parent.mkdir(parents=True)
                payload = bytearray(_synthetic_elf(0x003E))
                payload[offset] = 1
                executable.write_bytes(payload)
                if _RUN.os.name != "nt":
                    executable.chmod(0o755)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["linux-x64"],
                    )

    def test_rejects_foreign_osabi(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-linux-entry-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar/Binaries/Linux/shar-Linux-Shipping"
            executable.parent.mkdir(parents=True)
            payload = bytearray(_synthetic_elf(0x003E))
            payload[7] = 9
            executable.write_bytes(payload)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Linux SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

    def test_rejects_unsorted_load_segments(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-linux-entry-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar/Binaries/Linux/shar-Linux-Shipping"
            executable.parent.mkdir(parents=True)
            payload = bytearray(_synthetic_elf(0x003E))
            first = bytearray(payload[64:120])
            second = bytearray(first)
            payload[24:32] = (0x500000).to_bytes(8, "little")
            payload[56:58] = (2).to_bytes(2, "little")
            first[8:16] = (176).to_bytes(8, "little")
            first[16:24] = (0x500000).to_bytes(8, "little")
            second[8:16] = (177).to_bytes(8, "little")
            second[16:24] = (0x400000).to_bytes(8, "little")
            executable.write_bytes(payload[:64] + first + second + b"\0\0")
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Linux SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

    def test_validates_dynamic_segment_array(self) -> None:
        program = bytearray(56)
        program[:4] = (2).to_bytes(4, "little")
        program[8:16] = (64).to_bytes(8, "little")
        cases = (
            ("valid", (1).to_bytes(8, "little") + (b"\0" * 24), True),
            ("no-null", (1).to_bytes(8, "little") + (b"\0" * 8), False),
            ("short", b"\0" * 15, False),
            ("partial", b"\0" * 17, False),
        )
        for reason, payload, admitted in cases:
            candidate = bytearray(program)
            candidate[32:40] = len(payload).to_bytes(8, "little")
            stream = io.BytesIO((b"\0" * 64) + payload)
            stream.seek(32)
            with self.subTest(reason=reason):
                self.assertEqual(
                    _RUN._elf_dynamic_array_is_valid(
                        stream,
                        bytes(candidate),
                        "little",
                    ),
                    admitted,
                )
                self.assertEqual(stream.tell(), 32)

    def test_validates_thread_local_storage_segment(self) -> None:
        base = bytearray(56)
        base[:4] = (7).to_bytes(4, "little")
        base[4:8] = (0x4).to_bytes(4, "little")
        base[8:16] = (0x200).to_bytes(8, "little")
        base[16:24] = (0x400200).to_bytes(8, "little")
        base[32:40] = (8).to_bytes(8, "little")
        base[40:48] = (16).to_bytes(8, "little")
        base[48:56] = (8).to_bytes(8, "little")
        cases = (
            ("valid", None, True),
            ("flags", (4, 4, 0x5), False),
            ("file-size", (32, 8, 17), False),
            ("alignment", (48, 8, 3), False),
            ("congruence", (8, 8, 0x201), False),
            ("virtual-wrap", (16, 8, (1 << 64) - 8), False),
        )
        for reason, mutation, admitted in cases:
            program = bytearray(base)
            if mutation is not None:
                offset, width, value = mutation
                program[offset : offset + width] = value.to_bytes(
                    width,
                    "little",
                )
            with self.subTest(reason=reason):
                self.assertEqual(
                    _RUN._elf_tls_segment_is_valid(bytes(program), "little"),
                    admitted,
                )

    def test_validates_program_header_segment(self) -> None:
        cases = (
            ("valid", _synthetic_elf_with_program_header(), True),
            (
                "after-load",
                _synthetic_elf_with_program_header(phdr_first=False),
                False,
            ),
            (
                "duplicate",
                _synthetic_elf_with_program_header(duplicate=True),
                False,
            ),
            (
                "offset",
                _synthetic_elf_with_program_header(offset_delta=1),
                False,
            ),
            (
                "size",
                _synthetic_elf_with_program_header(size_delta=-1),
                False,
            ),
            (
                "unmapped",
                _synthetic_elf_with_program_header(mapped=False),
                False,
            ),
        )
        for reason, payload, admitted in cases:
            stream = io.BytesIO(payload)
            prefix = stream.read(4)
            with self.subTest(reason=reason):
                self.assertEqual(
                    _RUN._matches_elf(
                        stream,
                        prefix,
                        "amd64",
                        len(payload),
                        require_entrypoint=True,
                    ),
                    admitted,
                )

    def test_validates_interpreter_program_header(self) -> None:
        cases = (
            ("valid", _synthetic_elf_with_interpreter(b"/lib/ld.so\0"), True),
            (
                "after-load",
                _synthetic_elf_with_interpreter(
                    b"/lib/ld.so\0",
                    interpreter_first=False,
                ),
                False,
            ),
            (
                "duplicate",
                _synthetic_elf_with_interpreter(
                    b"/lib/ld.so\0",
                    duplicate=True,
                ),
                False,
            ),
            (
                "unterminated",
                _synthetic_elf_with_interpreter(b"/lib/ld.so"),
                False,
            ),
            (
                "embedded-nul",
                _synthetic_elf_with_interpreter(b"/lib\0x\0"),
                False,
            ),
            ("empty", _synthetic_elf_with_interpreter(b"\0"), False),
        )
        for reason, payload, admitted in cases:
            stream = io.BytesIO(payload)
            prefix = stream.read(4)
            with self.subTest(reason=reason):
                self.assertEqual(
                    _RUN._matches_elf(
                        stream,
                        prefix,
                        "amd64",
                        len(payload),
                        require_entrypoint=True,
                    ),
                    admitted,
                )

    def test_rejects_reserved_shlib_segment(self) -> None:
        base = bytearray(
            _synthetic_elf(
                0x003E,
                image_type=2,
                segment_offset=176,
            )
        )
        header = bytearray(base[:64])
        load = bytearray(base[64:120])
        header[56:58] = (2).to_bytes(2, "little")
        load[8:16] = (176).to_bytes(8, "little")
        shlib = bytearray(56)
        shlib[:4] = (5).to_bytes(4, "little")
        payload = bytes(header + shlib + load + b"\0")
        stream = io.BytesIO(payload)
        prefix = stream.read(4)
        self.assertFalse(
            _RUN._matches_elf(
                stream,
                prefix,
                "amd64",
                len(payload),
                require_entrypoint=True,
            )
        )

    def test_rejects_out_of_file_supplementary_segment(self) -> None:
        base = bytearray(
            _synthetic_elf(
                0x003E,
                image_type=2,
                segment_offset=176,
            )
        )
        header = bytearray(base[:64])
        load = bytearray(base[64:120])
        header[56:58] = (2).to_bytes(2, "little")
        load[8:16] = (176).to_bytes(8, "little")
        for dynamic_offset, admitted in ((176, True), (0x1000, False)):
            dynamic = bytearray(56)
            dynamic[:4] = (2).to_bytes(4, "little")
            dynamic[8:16] = dynamic_offset.to_bytes(8, "little")
            dynamic[32:40] = (16).to_bytes(8, "little")
            dynamic[40:48] = (16).to_bytes(8, "little")
            payload = bytes(header + dynamic + load + (b"\0" * 16))
            stream = io.BytesIO(payload)
            prefix = stream.read(4)
            with self.subTest(dynamic_offset=hex(dynamic_offset)):
                self.assertEqual(
                    _RUN._matches_elf(
                        stream,
                        prefix,
                        "amd64",
                        len(payload),
                        require_entrypoint=True,
                    ),
                    admitted,
                )

    def test_rejects_wrapping_load_segment_virtual_range(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-linux-entry-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar/Binaries/Linux/shar-Linux-Shipping"
            executable.parent.mkdir(parents=True)
            payload = bytearray(_synthetic_elf(0x003E))
            maximum = (1 << 64) - 1
            payload[24:32] = maximum.to_bytes(8, "little")
            payload[80:88] = maximum.to_bytes(8, "little")
            payload[104:112] = (2).to_bytes(8, "little")
            executable.write_bytes(payload)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Linux SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

    def test_rejects_entrypoint_in_elf_metadata(self) -> None:
        for reason, segment_offset in (
            ("elf-header", 0),
            ("program-header", 64),
        ):
            with (
                self.subTest(reason=reason),
                tempfile.TemporaryDirectory(prefix="shar-linux-entry-") as raw,
            ):
                candidate = Path(raw)
                executable = (
                    candidate / "shar/Binaries/Linux/shar-Linux-Shipping"
                )
                executable.parent.mkdir(parents=True)
                executable.write_bytes(
                    _synthetic_elf(
                        0x003E,
                        segment_offset=segment_offset,
                    )
                )
                if _RUN.os.name != "nt":
                    executable.chmod(0o755)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["linux-x64"],
                    )

    def test_rejects_entrypoint_in_zero_fill_tail(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-linux-entry-") as raw:
            candidate = Path(raw)
            executable = candidate / "shar/Binaries/Linux/shar-Linux-Shipping"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(
                _synthetic_elf(
                    0x003E,
                    entrypoint=0x400001,
                    segment_memory_size=2,
                )
            )
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Linux SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )


class MachOFatSliceTests(unittest.TestCase):
    """Require dyld-compatible universal ARM64 slice placement."""

    def test_rejects_nonzero_fat64_reserved_word(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-fat64-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(_synthetic_fat64_macho(reserved=1))
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_allows_zero_fat64_reserved_word(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-fat64-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(_synthetic_fat64_macho())
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["macos-arm64"],
            )

    def test_rejects_arm64_slice_outside_sixteen_kilobyte_page(self) -> None:
        valid_arm64 = _synthetic_fat_macho((_RUN._MACHO_ARM64_CPU,))
        source_offset = int.from_bytes(valid_arm64[16:20], "big")
        source_size = int.from_bytes(valid_arm64[20:24], "big")
        page_misaligned = bytearray(valid_arm64[:28])
        page_misaligned[16:20] = (4096).to_bytes(4, "big")
        page_misaligned[24:28] = (12).to_bytes(4, "big")
        page_misaligned.extend(b"\0" * (4096 - 28))
        page_misaligned.extend(
            valid_arm64[source_offset : source_offset + source_size]
        )
        with tempfile.TemporaryDirectory(prefix="shar-macos-fat-page-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(page_misaligned)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_rejects_arm64_slice_with_mismatched_cpu_subtype(self) -> None:
        fat = bytearray(_synthetic_fat_macho((_RUN._MACHO_ARM64_CPU,)))
        fat[12:16] = (1).to_bytes(4, "big")
        with tempfile.TemporaryDirectory(
            prefix="shar-macos-fat-subtype-"
        ) as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(fat)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_allows_arm64_subtype_feature_bit_difference(self) -> None:
        fat = bytearray(_synthetic_fat_macho((_RUN._MACHO_ARM64_CPU,)))
        fat[12:16] = (0x80000000).to_bytes(4, "big")
        with tempfile.TemporaryDirectory(
            prefix="shar-macos-fat-subtype-"
        ) as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(fat)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["macos-arm64"],
            )


def _synthetic_macho_with_linkedit_payload(*, inside: bool) -> bytes:
    """Return one Mach-O whose link-edit command has one data byte."""
    payload = bytearray(
        _synthetic_macho(
            _RUN._MACHO_ARM64_CPU,
            prefix_command_size=16,
        )
    )
    if inside:
        payload.extend(b"\0\0")
    command = 32 + 72
    data_offset = len(payload) - 1
    payload[command : command + 4] = (0x26).to_bytes(4, "little")
    payload[command + 8 : command + 12] = data_offset.to_bytes(4, "little")
    payload[command + 12 : command + 16] = (1).to_bytes(4, "little")
    if inside:
        payload[32 + 48 : 32 + 56] = data_offset.to_bytes(8, "little")
        linkedit = 32 + 72 + 16 + 24
        payload[linkedit + 32 : linkedit + 40] = (1).to_bytes(8, "little")
        payload[linkedit + 40 : linkedit + 48] = data_offset.to_bytes(
            8,
            "little",
        )
        payload[linkedit + 48 : linkedit + 56] = (1).to_bytes(8, "little")
    return bytes(payload)


class MachOLinkEditDataTests(unittest.TestCase):
    """Require Mach-O link-edit data commands to stay file-bounded."""

    def test_requires_positive_payloads_inside_linkedit_segment(self) -> None:
        for inside in (True, False):
            payload = _synthetic_macho_with_linkedit_payload(inside=inside)
            stream = io.BytesIO(payload)
            prefix = stream.read(4)
            with self.subTest(inside=inside):
                self.assertEqual(
                    _RUN._matches_macho(
                        stream,
                        prefix,
                        "macos",
                        "arm64",
                        len(payload),
                    ),
                    inside,
                )

    def test_validates_linkedit_data_command_ranges(self) -> None:
        for reason, command_size, offset, size, file_size, admitted in (
            ("valid", 16, 0x100, 0x20, 0x200, True),
            ("empty", 16, 0, 0, 0x200, True),
            ("command-size", 24, 0x100, 0x20, 0x200, False),
            ("offset", 16, 0x201, 0, 0x200, False),
            ("range", 16, 0x1F0, 0x20, 0x200, False),
        ):
            body = offset.to_bytes(4, "little") + size.to_bytes(4, "little")
            if command_size > 16:
                body += b"\0" * (command_size - 16)
            with self.subTest(reason=reason):
                self.assertEqual(
                    _RUN._macho_linkedit_data_is_valid(
                        body,
                        command_size,
                        "little",
                        file_size,
                    ),
                    admitted,
                )

    def test_rejects_overlapping_linkedit_payloads(self) -> None:
        segments = [
            _RUN._MachOSegment(
                b"__LINKEDIT",
                0x1000,
                0x100,
                0x100,
                0x100,
                0x1,
            )
        ]
        for ranges, admitted in (
            ([(0x120, 0x20), (0x140, 0x20)], True),
            ([(0x120, 0x20), (0x130, 0x20)], False),
        ):
            with self.subTest(ranges=ranges):
                self.assertEqual(
                    _RUN._macho_linkedit_ranges_fit_segment(
                        segments,
                        ranges,
                    ),
                    admitted,
                )

    def test_applies_bounds_to_all_linkedit_command_ids(self) -> None:
        commands = (
            0x1D,
            0x1E,
            0x26,
            0x29,
            0x2B,
            0x2E,
            0x80000033,
            0x80000034,
        )
        for command in commands:
            with self.subTest(command=hex(command)):
                self.assertIsNone(
                    _RUN._macho_auxiliary_command(
                        command,
                        16,
                        (0x200).to_bytes(4, "little")
                        + (1).to_bytes(4, "little"),
                        "little",
                        0x200,
                    )
                )


class MachOPlatformTests(unittest.TestCase):
    """Bind ARM64 Mach-O admission to its target Apple platform."""

    def test_requires_exact_build_platform(self) -> None:
        for system, expected, other in (("macos", 1, 2), ("ios", 2, 1)):
            for platform, admitted in ((expected, True), (other, False)):
                payload = _synthetic_macho(
                    _RUN._MACHO_ARM64_CPU,
                    platform=platform,
                )
                stream = io.BytesIO(payload)
                prefix = stream.read(4)
                with self.subTest(system=system, platform=platform):
                    self.assertEqual(
                        _RUN._matches_macho(
                            stream,
                            prefix,
                            system,
                            "arm64",
                            len(payload),
                        ),
                        admitted,
                    )

    def test_rejects_legacy_platform_commands(self) -> None:
        legacy_commands = (0x24, 0x25, 0x2F, 0x30)
        for system, platform in (("macos", 1), ("ios", 2)):
            for command in legacy_commands:
                payload = bytearray(
                    _synthetic_macho(
                        _RUN._MACHO_ARM64_CPU,
                        platform=platform,
                        prefix_command_size=16,
                    )
                )
                payload[104:108] = command.to_bytes(4, "little")
                stream = io.BytesIO(payload)
                prefix = stream.read(4)
                with self.subTest(system=system, command=command):
                    self.assertFalse(
                        _RUN._matches_macho(
                            stream,
                            prefix,
                            system,
                            "arm64",
                            len(payload),
                        )
                    )


class MachODynamicLinkerTests(unittest.TestCase):
    """Require structurally complete dynamic-linker load commands."""

    def test_rejects_bare_dynamic_linker_marker(self) -> None:
        self.assertIsNone(
            _RUN._macho_auxiliary_command(0xE, 8, b"", "little", 0x200)
        )

    def test_rejects_malformed_dynamic_linker_paths(self) -> None:
        command = _synthetic_dylinker_command()
        for reason in ("invalid-offset", "unterminated"):
            body = bytearray(command[8:])
            if reason == "invalid-offset":
                body[:4] = (8).to_bytes(4, "little")
            else:
                body[4:] = b"A" * (len(body) - 4)
            with self.subTest(reason=reason):
                self.assertIsNone(
                    _RUN._macho_auxiliary_command(
                        0xE,
                        len(command),
                        bytes(body),
                        "little",
                        0x200,
                    )
                )


class MachOLoaderSegmentTests(unittest.TestCase):
    """Require dyld loader-facing segment identities and permissions."""

    def test_rejects_missing_linkedit_segment(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-linkedit-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            payload = bytearray(_synthetic_macho(_RUN._MACHO_ARM64_CPU))
            payload[128:132] = (0x1B).to_bytes(4, "little")
            executable.write_bytes(payload)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_rejects_static_execute_without_dylinker(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-static-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            payload = bytearray(_synthetic_macho(_RUN._MACHO_ARM64_CPU))
            command = _synthetic_dylinker_command()
            offset = payload.find(command)
            self.assertNotEqual(offset, -1)
            payload[offset : offset + 4] = (0x1B).to_bytes(4, "little")
            executable.write_bytes(payload)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_rejects_loader_segment_permissions(self) -> None:
        cases = (
            ("writable-text", 88, 92, 0x7),
            ("writable-linkedit", 184, 188, 0x3),
        )
        for reason, maximum_offset, initial_offset, protection in cases:
            with (
                self.subTest(reason=reason),
                tempfile.TemporaryDirectory(
                    prefix="shar-macos-permissions-"
                ) as raw,
            ):
                candidate = Path(raw)
                executable = candidate / "SHAR.app/Contents/MacOS/shar"
                executable.parent.mkdir(parents=True)
                payload = bytearray(_synthetic_macho(_RUN._MACHO_ARM64_CPU))
                payload[maximum_offset : maximum_offset + 4] = (
                    protection.to_bytes(4, "little")
                )
                encoded = protection.to_bytes(4, "little")
                payload[initial_offset : initial_offset + 4] = encoded
                executable.write_bytes(payload)
                if _RUN.os.name != "nt":
                    executable.chmod(0o755)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "macOS SHAR app bundle",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["macos-arm64"],
                    )


class MachOSegmentOrderTests(unittest.TestCase):
    """Require dyld-compatible Mach-O segment load-command ordering."""

    def test_rejects_segment_layout_order_inversions(self) -> None:
        text = _RUN._MachOSegment(
            b"__TEXT", 0x100000000, 0x100, 0, 0x100, 0x5
        )
        cases = (
            (
                "virtual",
                [
                    text,
                    _RUN._MachOSegment(
                        b"__LINKEDIT",
                        0x100003000,
                        0x100,
                        0x300,
                        0x100,
                        0x1,
                    ),
                    _RUN._MachOSegment(
                        b"__DATA",
                        0x100002000,
                        0x100,
                        0x400,
                        0x100,
                        0x3,
                    ),
                ],
            ),
            (
                "file",
                [
                    text,
                    _RUN._MachOSegment(
                        b"__DATA",
                        0x100002000,
                        0x100,
                        0x400,
                        0x100,
                        0x3,
                    ),
                    _RUN._MachOSegment(
                        b"__LINKEDIT",
                        0x100003000,
                        0x100,
                        0x300,
                        0x100,
                        0x1,
                    ),
                ],
            ),
        )
        for reason, segments in cases:
            with self.subTest(reason=reason):
                self.assertFalse(
                    _RUN._macho_segments_follow_layout_order(segments)
                )

    def test_requires_linkedit_to_have_last_file_offset(self) -> None:
        text = _RUN._MachOSegment(
            b"__TEXT", 0x1000, 0x1000, 0, 0x200, 0x5
        )
        data = _RUN._MachOSegment(
            b"__DATA", 0x2000, 0x1000, 0x200, 0x100, 0x3
        )
        linkedit = _RUN._MachOSegment(
            b"__LINKEDIT", 0x3000, 0x1000, 0x300, 0x100, 0x1
        )
        late = _RUN._MachOSegment(
            b"__LATE", 0x4000, 0x1000, 0x400, 0x100, 0x1
        )
        self.assertTrue(
            _RUN._macho_entrypoint_matches_segments(
                [text, data, linkedit],
                [("main", 0x100)],
                0x80,
            )
        )
        self.assertFalse(
            _RUN._macho_entrypoint_matches_segments(
                [text, data, linkedit, late],
                [("main", 0x100)],
                0x80,
            )
        )

    def test_allows_dwarf_segment_order_exception(self) -> None:
        segments = [
            _RUN._MachOSegment(
                b"__TEXT", 0x100000000, 0x100, 0, 0x100, 0x5
            ),
            _RUN._MachOSegment(
                b"__DWARF", 0x100004000, 0x100, 0x500, 0x100, 0x1
            ),
            _RUN._MachOSegment(
                b"__LINKEDIT",
                0x100003000,
                0x100,
                0x400,
                0x100,
                0x1,
            ),
        ]
        self.assertTrue(_RUN._macho_segments_follow_layout_order(segments))


class MachOEntrypointTests(unittest.TestCase):
    """Require one unambiguous Mach-O process entry command."""

    def test_rejects_32_bit_segment_command(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-command-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            invalid = bytearray(
                _synthetic_macho(
                    _RUN._MACHO_ARM64_CPU,
                    prefix_command_size=8,
                )
            )
            invalid[104:108] = (0x1).to_bytes(4, "little")
            executable.write_bytes(invalid)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_rejects_nonzero_reserved_header_word(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-header-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            payload = bytearray(_synthetic_macho(_RUN._MACHO_ARM64_CPU))
            payload[28:32] = (1).to_bytes(4, "little")
            executable.write_bytes(payload)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_rejects_overlapping_segment_mappings(self) -> None:
        text_segment = _RUN._MachOSegment(
            name=b"__TEXT",
            virtual_address=0x100000000,
            virtual_size=0x200,
            file_offset=0,
            file_size=0x200,
            initial_protection=0x5,
        )
        cases = (
            (
                "virtual",
                _RUN._MachOSegment(
                    name=b"__DATA",
                    virtual_address=0x100000100,
                    virtual_size=0x100,
                    file_offset=0x200,
                    file_size=0x100,
                    initial_protection=0x3,
                ),
            ),
            (
                "file",
                _RUN._MachOSegment(
                    name=b"__DATA",
                    virtual_address=0x100000200,
                    virtual_size=0x100,
                    file_offset=0x100,
                    file_size=0x100,
                    initial_protection=0x3,
                ),
            ),
        )
        for reason, overlapping in cases:
            with self.subTest(reason=reason):
                self.assertFalse(
                    _RUN._macho_entrypoint_matches_segments(
                        [text_segment, overlapping],
                        [("main", 0x80)],
                        64,
                    )
                )

    def test_rejects_lc_main_offset_beyond_text_file_bytes(self) -> None:
        text_segment = _RUN._MachOSegment(
            name=b"__TEXT",
            virtual_address=0x100000000,
            virtual_size=0x1000,
            file_offset=0,
            file_size=0x100,
            initial_protection=0x5,
        )
        later_segment = _RUN._MachOSegment(
            name=b"__ALT",
            virtual_address=0x100000100,
            virtual_size=0x100,
            file_offset=0x100,
            file_size=0x100,
            initial_protection=0x5,
        )
        self.assertFalse(
            _RUN._macho_entrypoint_matches_segments(
                [text_segment, later_segment],
                [("main", 0x100)],
                64,
            )
        )

    def test_rejects_lc_main_inside_load_commands(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-entry-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(
                _synthetic_macho(
                    _RUN._MACHO_ARM64_CPU,
                    entry_offset=32,
                )
            )
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_rejects_entrypoint_in_zero_fill_tail(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-entry-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(
                _synthetic_macho(
                    _RUN._MACHO_ARM64_CPU,
                    entry_offset=512,
                )
            )
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_rejects_duplicate_entry_commands(self) -> None:
        text_segment = _RUN._MachOSegment(
            name=b"__TEXT",
            virtual_address=0x100000000,
            virtual_size=0x1000,
            file_offset=0,
            file_size=0x200,
            initial_protection=0x5,
        )
        linkedit_segment = _RUN._MachOSegment(
            name=b"__LINKEDIT",
            virtual_address=0x100001000,
            virtual_size=0,
            file_offset=0x200,
            file_size=0,
            initial_protection=0x1,
        )
        self.assertFalse(
            _RUN._macho_entrypoint_matches_segments(
                [text_segment, linkedit_segment],
                [("main", 0x100), ("main", 0x100)],
                64,
            )
        )

    def test_rejects_invalid_section_alignment(self) -> None:
        payload = _synthetic_macho(_RUN._MACHO_ARM64_CPU)
        body = bytearray(payload[40:104])
        body[56:60] = (1).to_bytes(4, "little")
        virtual_address = int.from_bytes(body[16:24], "little")
        for reason, address, alignment_power in (
            ("misaligned-address", virtual_address + 1, 4),
            ("oversized-exponent", virtual_address, 64),
        ):
            section = bytearray(80)
            section[16:32] = body[:16]
            section[32:40] = address.to_bytes(8, "little")
            section[40:48] = (1).to_bytes(8, "little")
            section[48:52] = (128).to_bytes(4, "little")
            section[52:56] = alignment_power.to_bytes(4, "little")
            with self.subTest(reason=reason):
                self.assertIsNone(
                    _RUN._macho_segment64(
                        bytes(body + section),
                        152,
                        "little",
                        len(payload),
                    )
                )

    def test_rejects_section_with_mismatched_segment_name(self) -> None:
        payload = _synthetic_macho(_RUN._MACHO_ARM64_CPU)
        body = bytearray(payload[40:104])
        body[56:60] = (1).to_bytes(4, "little")
        section = bytearray(80)
        section[16:32] = b"__DATA" + (b"\0" * 10)
        virtual_address = int.from_bytes(body[16:24], "little")
        section[32:40] = virtual_address.to_bytes(8, "little")
        section[40:48] = (1).to_bytes(8, "little")
        section[48:52] = (128).to_bytes(4, "little")
        self.assertIsNone(
            _RUN._macho_segment64(
                bytes(body + section),
                152,
                "little",
                len(payload),
            )
        )

    def test_rejects_section_file_offset_inconsistent_with_type(self) -> None:
        payload = _synthetic_macho(_RUN._MACHO_ARM64_CPU)
        body = bytearray(payload[40:104])
        body[56:60] = (1).to_bytes(4, "little")
        virtual_address = int.from_bytes(body[16:24], "little")
        for reason, section_type, section_offset in (
            ("regular-without-file-offset", 0x0, 0),
            ("zero-fill-with-file-offset", 0x1, 1),
            ("gb-zero-fill-with-file-offset", 0xC, 1),
            ("tls-zero-fill-with-file-offset", 0x12, 1),
        ):
            section = bytearray(80)
            section[32:40] = virtual_address.to_bytes(8, "little")
            section[40:48] = (1).to_bytes(8, "little")
            section[48:52] = section_offset.to_bytes(4, "little")
            section[64:68] = section_type.to_bytes(4, "little")
            with self.subTest(reason=reason):
                self.assertIsNone(
                    _RUN._macho_segment64(
                        bytes(body + section),
                        152,
                        "little",
                        len(payload),
                    )
                )

    def test_rejects_section_beyond_segment_virtual_extent(self) -> None:
        payload = _synthetic_macho(_RUN._MACHO_ARM64_CPU)
        body = bytearray(payload[40:104])
        body[56:60] = (1).to_bytes(4, "little")
        section = bytearray(80)
        virtual_address = int.from_bytes(body[16:24], "little")
        virtual_size = int.from_bytes(body[24:32], "little")
        section[32:40] = (virtual_address + virtual_size).to_bytes(8, "little")
        section[40:48] = (1).to_bytes(8, "little")
        self.assertIsNone(
            _RUN._macho_segment64(
                bytes(body + section),
                152,
                "little",
                len(payload),
            )
        )

    def test_rejects_section_beyond_segment_file_extent(self) -> None:
        payload = _synthetic_macho(_RUN._MACHO_ARM64_CPU)
        body = bytearray(payload[40:104])
        body[56:60] = (1).to_bytes(4, "little")
        section = bytearray(80)
        section[40:48] = (2).to_bytes(8, "little")
        section[48:52] = (128).to_bytes(4, "little")
        self.assertIsNone(
            _RUN._macho_segment64(
                bytes(body + section),
                152,
                "little",
                len(payload),
            )
        )

    def test_rejects_invalid_segment_protection_flags(self) -> None:
        for reason, protection in (
            ("executable-only", 0x4),
            ("reserved-bit", 0xD),
        ):
            with (
                self.subTest(reason=reason),
                tempfile.TemporaryDirectory(
                    prefix="shar-macos-segment-"
                ) as raw,
            ):
                candidate = Path(raw)
                executable = candidate / "SHAR.app/Contents/MacOS/shar"
                executable.parent.mkdir(parents=True)
                invalid = bytearray(_synthetic_macho(_RUN._MACHO_ARM64_CPU))
                invalid[88:92] = protection.to_bytes(4, "little")
                invalid[92:96] = protection.to_bytes(4, "little")
                executable.write_bytes(invalid)
                if _RUN.os.name != "nt":
                    executable.chmod(0o755)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "macOS SHAR app bundle",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["macos-arm64"],
                    )

    def test_rejects_initial_protection_beyond_maximum(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-segment-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            invalid = bytearray(_synthetic_macho(_RUN._MACHO_ARM64_CPU))
            invalid[88:92] = (0x1).to_bytes(4, "little")
            executable.write_bytes(invalid)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

    def test_rejects_non_executable_entry_segment(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-segment-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            non_executable = bytearray(
                _synthetic_macho(_RUN._MACHO_ARM64_CPU)
            )
            non_executable[92:96] = (0x1).to_bytes(4, "little")
            executable.write_bytes(non_executable)
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )


class MobileArchiveTreeTests(unittest.TestCase):
    """Require mobile ZIP inventories to describe realizable trees."""

    def test_mobile_candidate_rejects_file_ancestor_conflict(self) -> None:
        cases = (
            (
                "android-arm64",
                "shar.apk",
                _write_android_apk,
                "lib",
                "Android APK",
            ),
            ("ios-arm64", "shar.ipa", _write_ios_ipa, "Payload", "iOS IPA"),
        )
        for target_id, name, write_valid, conflict, label in cases:
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(
                    prefix="shar-mobile-member-tree-"
                ) as raw,
            ):
                candidate = Path(raw)
                package = candidate / name
                write_valid(package)
                with _RUN.zipfile.ZipFile(package, "a") as archive:
                    archive.writestr(conflict, b"file ancestor")
                with self.assertRaisesRegex(_RUN.RunFailure, label):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

    def test_mobile_candidate_rejects_file_directory_alias(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-mobile-member-tree-"
        ) as raw:
            candidate = Path(raw)
            package = candidate / "shar.apk"
            _write_android_apk(package)
            with _RUN.zipfile.ZipFile(package, "a") as archive:
                archive.writestr("assets/conflict", b"file")
                archive.writestr("assets/conflict/", b"")
            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )


class AndroidManifestStructureTests(unittest.TestCase):
    """Require APK manifests to retain compiled binary-XML structure."""

    def test_rejects_malformed_binary_xml_manifests(self) -> None:
        baseline = _synthetic_android_manifest()
        cases: list[tuple[str, bytes, bool]] = [("valid", baseline, True)]
        cases.append(
            (
                "wrong-root-name",
                _synthetic_android_manifest(root_name="application"),
                False,
            )
        )
        for reason, package_name in (
            ("missing-package", None),
            ("package-no-separator", "shar"),
            ("package-leading-digit", "9shar.game"),
            ("package-invalid-segment", "shar.-game"),
        ):
            cases.append(
                (
                    reason,
                    _synthetic_android_manifest(package_name=package_name),
                    False,
                )
            )
        cases.append(
            (
                "split-apk",
                _synthetic_android_manifest(split_name="feature.camera"),
                False,
            )
        )
        plain = b"synthetic manifest"
        cases.append(("plain-text", plain, False))
        string_pool_size = int.from_bytes(baseline[12:16], "little")
        start = 8 + string_pool_size
        mutations = (
            ("root-type", 0, 2, 0x0002),
            ("root-size", 4, 4, len(baseline) + 4),
            ("string-pool-header", 10, 2, 24),
            ("string-count", 16, 4, 0xFFFFFFFF),
            ("style-count", 20, 4, 1),
            ("string-start", 28, 4, 0xFFFFFFFF),
            ("string-terminator", start - 1, 1, 1),
            ("root-name-index", start + 20, 4, 1),
            ("node-before-strings", 8, 2, 0x0180),
            ("missing-start", start, 2, 0x0103),
            ("misaligned-child", start + 4, 4, 35),
            ("attribute-overflow", start + 28, 2, 2),
        )
        for reason, offset, width, value in mutations:
            payload = bytearray(baseline)
            payload[offset : offset + width] = value.to_bytes(width, "little")
            cases.append((reason, bytes(payload), False))
        for reason, manifest, admitted in cases:
            with (
                self.subTest(reason=reason),
                tempfile.TemporaryDirectory(
                    prefix="shar-android-manifest-"
                ) as raw,
            ):
                package = Path(raw) / "shar.apk"
                with _RUN.zipfile.ZipFile(package, "w") as archive:
                    archive.writestr("AndroidManifest.xml", manifest)
                    archive.writestr(
                        "lib/arm64-v8a/libUnreal.so",
                        _synthetic_elf(0x00B7),
                    )
                self.assertEqual(_RUN._is_android_apk(package), admitted)

    def test_applies_package_length_bound_to_both_string_encodings(
        self,
    ) -> None:
        for length, admitted in ((128, True), (223, True), (224, False)):
            package_name = "a." + ("b" * (length - 2))
            for utf8 in (True, False):
                with self.subTest(length=length, utf8=utf8):
                    manifest = _synthetic_android_manifest(
                        package_name=package_name,
                        utf8=utf8,
                    )
                    layout = _RUN._android_xml_root_layout(manifest)
                    self.assertIsNotNone(layout)
                    if layout is None:
                        continue
                    actual = _RUN._android_xml_children_are_valid(
                        manifest,
                        *layout,
                    )
                    self.assertEqual(actual, admitted)

    def test_accepts_supported_root_string_encodings_and_child_tags(
        self,
    ) -> None:
        cases = (
            ("utf8", _synthetic_android_manifest()),
            ("utf16", _synthetic_android_manifest(utf8=False)),
            (
                "android-package",
                _synthetic_android_manifest(package_name="android"),
            ),
            ("empty-split", _synthetic_android_manifest(split_name="")),
            (
                "child-tag",
                _synthetic_android_manifest(child_name="application"),
            ),
        )
        for reason, manifest in cases:
            with (
                self.subTest(reason=reason),
                tempfile.TemporaryDirectory(
                    prefix="shar-android-manifest-"
                ) as raw,
            ):
                package = Path(raw) / "shar.apk"
                with _RUN.zipfile.ZipFile(package, "w") as archive:
                    archive.writestr("AndroidManifest.xml", manifest)
                    archive.writestr(
                        "lib/arm64-v8a/libUnreal.so",
                        _synthetic_elf(0x00B7),
                    )
                self.assertTrue(_RUN._is_android_apk(package))


class MobileArchiveMemberTypeTests(unittest.TestCase):
    """Require loader-critical mobile ZIP members to be regular files."""

    def test_android_rejects_special_required_members(self) -> None:
        cases = (
            ("manifest", "AndroidManifest.xml"),
            ("native", "lib/arm64-v8a/libUnreal.so"),
        )
        for reason, special_name in cases:
            with (
                self.subTest(reason=reason),
                tempfile.TemporaryDirectory(
                    prefix="shar-android-member-type-"
                ) as raw,
            ):
                package = Path(raw) / "shar.apk"
                with _RUN.zipfile.ZipFile(package, "w") as archive:
                    manifest = _synthetic_android_manifest()
                    native = _synthetic_elf(0x00B7)
                    for name, payload in (
                        ("AndroidManifest.xml", manifest),
                        ("lib/arm64-v8a/libUnreal.so", native),
                    ):
                        member = (
                            _unix_special_zip_member(name, _RUN.stat.S_IFLNK)
                            if name == special_name
                            else name
                        )
                        archive.writestr(member, payload)
                self.assertFalse(_RUN._is_android_apk(package))

    def test_ios_rejects_special_required_members(self) -> None:
        cases = (
            ("plist", "Payload/SHAR.app/Info.plist"),
            ("binary", "Payload/SHAR.app/shar"),
        )
        for reason, special_name in cases:
            with (
                self.subTest(reason=reason),
                tempfile.TemporaryDirectory(
                    prefix="shar-ios-member-type-"
                ) as raw,
            ):
                package = Path(raw) / "shar.ipa"
                with _RUN.zipfile.ZipFile(package, "w") as archive:
                    plist = _RUN.plistlib.dumps(
                        {
                            "CFBundleExecutable": "shar",
                            "CFBundleIdentifier": "org.shar.game",
                            "CFBundlePackageType": "APPL",
                        }
                    )
                    binary = _synthetic_macho(
                        _RUN._MACHO_ARM64_CPU,
                        platform=2,
                    )
                    for name, payload in (
                        ("Payload/SHAR.app/Info.plist", plist),
                        ("Payload/SHAR.app/shar", binary),
                    ):
                        member = (
                            _unix_special_zip_member(name, _RUN.stat.S_IFLNK)
                            if name == special_name
                            else name
                        )
                        archive.writestr(member, payload)
                self.assertFalse(_RUN._is_ios_ipa(package))

    def test_ios_allows_unrelated_symlink_member(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-ios-member-type-") as raw:
            package = Path(raw) / "shar.ipa"
            _write_ios_ipa(package)
            with _RUN.zipfile.ZipFile(package, "a") as archive:
                archive.writestr(
                    _unix_special_zip_member(
                        "Payload/SHAR.app/Frameworks/Current",
                        _RUN.stat.S_IFLNK,
                    ),
                    b"A",
                )
            self.assertTrue(_RUN._is_ios_ipa(package))


class MobileArtifactMultiplicityTests(unittest.TestCase):
    """Require one unambiguous package for each mobile target."""

    def test_rejects_additional_mobile_package(self) -> None:
        cases = (
            ("android-arm64", "shar.apk", "stale.apk", _write_android_apk),
            ("ios-arm64", "shar.ipa", "stale.ipa", _write_ios_ipa),
        )
        for target_id, valid_name, extra_name, write_valid in cases:
            for extra_valid in (False, True):
                with (
                    self.subTest(target=target_id, extra_valid=extra_valid),
                    tempfile.TemporaryDirectory(
                        prefix="shar-mobile-multiplicity-"
                    ) as raw,
                ):
                    candidate = Path(raw)
                    write_valid(candidate / valid_name)
                    extra = candidate / extra_name
                    if extra_valid:
                        write_valid(extra)
                    else:
                        extra.write_bytes(b"invalid package")
                    with self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "expected exactly one",
                    ):
                        _RUN._validate_candidate_artifact(
                            candidate,
                            _RUN._TARGETS_BY_ID[target_id],
                        )


class IosBundleMetadataTests(unittest.TestCase):
    """Require iOS app-bundle identity metadata needed by signing."""

    def test_ios_candidate_requires_valid_bundle_identifier(self) -> None:
        invalid_values: tuple[object, ...] = (
            None,
            "",
            "org/shar/game",
            "org shar.game",
            "org.shar.🔥",
            7,
        )
        target = _RUN._TARGETS_BY_ID["ios-arm64"]
        for value in invalid_values:
            with (
                self.subTest(value=value),
                tempfile.TemporaryDirectory(
                    prefix="shar-ios-bundle-id-"
                ) as raw,
            ):
                candidate = Path(raw)
                _write_ios_ipa(candidate / "shar.ipa", bundle_id=value)
                with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                    _RUN._validate_candidate_artifact(candidate, target)

        for value in ("org.shar.game", "A-1.b-2", "single"):
            with (
                self.subTest(valid=value),
                tempfile.TemporaryDirectory(
                    prefix="shar-ios-bundle-id-"
                ) as raw,
            ):
                candidate = Path(raw)
                _write_ios_ipa(candidate / "shar.ipa", bundle_id=value)
                _RUN._validate_candidate_artifact(candidate, target)

    def test_ios_candidate_rejects_contradictory_bundle_type(self) -> None:
        target = _RUN._TARGETS_BY_ID["ios-arm64"]
        for value in ("FMWK", "BNDL", "", 7):
            with (
                self.subTest(value=value),
                tempfile.TemporaryDirectory(
                    prefix="shar-ios-bundle-type-"
                ) as raw,
            ):
                candidate = Path(raw)
                _write_ios_ipa(candidate / "shar.ipa", package_type=value)
                with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                    _RUN._validate_candidate_artifact(candidate, target)

        for value in (None, "APPL"):
            with (
                self.subTest(valid=value),
                tempfile.TemporaryDirectory(
                    prefix="shar-ios-bundle-type-"
                ) as raw,
            ):
                candidate = Path(raw)
                _write_ios_ipa(candidate / "shar.ipa", package_type=value)
                _RUN._validate_candidate_artifact(candidate, target)


class CandidateArtifactTests(unittest.TestCase):
    """Require each candidate to contain its declared runnable artifact."""

    def test_android_candidate_requires_apk(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-android-candidate-",
        ) as raw:
            candidate = Path(raw)
            apk = candidate / "shar.apk"
            apk.write_bytes(b"not a package")

            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )

            with _RUN.zipfile.ZipFile(apk, "w") as archive:
                archive.writestr(
                    "lib/arm64-v8a/libUnreal.so",
                    _synthetic_elf(0x00B7),
                )
            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )

            with _RUN.zipfile.ZipFile(apk, "w") as archive:
                archive.writestr("AndroidManifest.xml", b"")
                archive.writestr(
                    "lib/arm64-v8a/libUnreal.so",
                    _synthetic_elf(0x00B7),
                )
            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )

            _write_android_apk(apk, machine=0x003E)
            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )

            with _RUN.zipfile.ZipFile(apk, "w") as archive:
                archive.writestr(
                    "AndroidManifest.xml",
                    _synthetic_android_manifest(),
                )
                archive.writestr(
                    "lib/arm64-v8a/libUnreal.so",
                    _synthetic_elf(0x00B7, image_type=2),
                )
            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )

            with _RUN.zipfile.ZipFile(apk, "w") as archive:
                archive.writestr(
                    "AndroidManifest.xml",
                    _synthetic_android_manifest(),
                )
                archive.writestr(
                    "lib/arm64-v8a/libUnreal.so",
                    _synthetic_big_endian_elf(0x00B7),
                )
            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )

            with _RUN.zipfile.ZipFile(apk, "w") as archive:
                archive.writestr(
                    "AndroidManifest.xml",
                    _synthetic_android_manifest(),
                )
                archive.writestr(
                    "lib/arm64-v8a/libUnreal.so",
                    _synthetic_elf(
                        0x00B7,
                        segment_file_size=0,
                        segment_memory_size=1,
                    ),
                )
            with self.assertRaisesRegex(_RUN.RunFailure, "Android APK"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["android-arm64"],
                )

            for alignment in (3, 0x1000):
                native = bytearray(_synthetic_elf(0x00B7))
                native[112:120] = alignment.to_bytes(8, "little")
                with _RUN.zipfile.ZipFile(apk, "w") as archive:
                    archive.writestr(
                        "AndroidManifest.xml",
                        _synthetic_android_manifest(),
                    )
                    archive.writestr(
                        "lib/arm64-v8a/libUnreal.so",
                        native,
                    )
                with (
                    self.subTest(alignment=alignment),
                    self.assertRaisesRegex(_RUN.RunFailure, "Android APK"),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["android-arm64"],
                    )

            _write_android_apk(apk)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["android-arm64"],
            )

            _write_android_apk(apk, entrypoint=0)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["android-arm64"],
            )

    def test_android_candidate_requires_loader_library_path(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-android-candidate-",
        ) as raw:
            candidate = Path(raw)
            apk = candidate / "shar.apk"
            target = _RUN._TARGETS_BY_ID["android-arm64"]
            invalid_members = (
                "lib/arm64-v8a/nested/libUnreal.so",
                "lib/arm64-v8a/Unreal.so",
                "lib/arm64-v8a/lib.so",
            )
            for member in invalid_members:
                with self.subTest(member=member):
                    with _RUN.zipfile.ZipFile(apk, "w") as archive:
                        archive.writestr(
                            "AndroidManifest.xml",
                            _synthetic_android_manifest(),
                        )
                        archive.writestr(member, _synthetic_elf(0x00B7))
                    with self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "Android APK",
                    ):
                        _RUN._validate_candidate_artifact(candidate, target)

    def test_ios_candidate_requires_ipa(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-ios-candidate-") as raw:
            candidate = Path(raw)
            ipa = candidate / "shar.ipa"
            ipa.write_bytes(b"not a package")

            with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["ios-arm64"],
                )

            _write_ios_ipa(ipa, cpu=0x01000007)
            with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["ios-arm64"],
                )

            _write_ios_ipa(ipa)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["ios-arm64"],
            )

            non_executable = bytearray(
                _synthetic_macho(_RUN._MACHO_ARM64_CPU)
            )
            non_executable[92:96] = (0x1).to_bytes(4, "little")
            _write_ios_ipa(ipa, binary=bytes(non_executable))
            with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["ios-arm64"],
                )

            _write_ios_ipa(
                ipa,
                binary=_synthetic_macho(_RUN._MACHO_ARM64_CPU, file_type=6),
            )
            with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["ios-arm64"],
                )

            _write_ios_ipa(
                ipa,
                binary=_synthetic_macho(_RUN._MACHO_ARM64_CPU, command=0x1B),
            )
            with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["ios-arm64"],
                )

            _write_ios_ipa(
                ipa,
                binary=_synthetic_macho(
                    _RUN._MACHO_ARM64_CPU,
                    entry_offset=4096,
                ),
            )
            with self.assertRaisesRegex(_RUN.RunFailure, "iOS IPA"):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["ios-arm64"],
                )

            _write_ios_ipa(
                ipa,
                binary=_synthetic_fat_macho(
                    (0x01000007, _RUN._MACHO_ARM64_CPU),
                    platform=2,
                ),
            )
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["ios-arm64"],
            )

    def test_mobile_candidate_rejects_unsafe_archive_member_path(self) -> None:
        cases = (
            ("android-arm64", "shar.apk", _write_android_apk, "Android APK"),
            ("ios-arm64", "shar.ipa", _write_ios_ipa, "iOS IPA"),
        )
        unsafe_members = (
            "../escape.bin",
            "/rooted.bin",
            "nested\\escape.bin",
            "nested//escape.bin",
            "./alias.bin",
            "C:/escape.bin",
            "C:escape.bin",
        )
        for target_id, name, write_valid, label in cases:
            for member in unsafe_members:
                with (
                    self.subTest(target=target_id, member=member),
                    tempfile.TemporaryDirectory(
                        prefix="shar-mobile-member-path-"
                    ) as raw,
                ):
                    candidate = Path(raw)
                    package = candidate / name
                    write_valid(package)
                    with _RUN.zipfile.ZipFile(package, "a") as archive:
                        archive.writestr(member, b"unsafe")
                    with self.assertRaisesRegex(_RUN.RunFailure, label):
                        _RUN._validate_candidate_artifact(
                            candidate,
                            _RUN._TARGETS_BY_ID[target_id],
                        )

    def test_mobile_candidate_rejects_corrupt_unrelated_member(self) -> None:
        cases = (
            ("android-arm64", "shar.apk", _write_android_apk, "Android APK"),
            ("ios-arm64", "shar.ipa", _write_ios_ipa, "iOS IPA"),
        )
        member = "assets/integrity.bin"
        for target_id, name, write_valid, label in cases:
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(
                    prefix="shar-mobile-integrity-"
                ) as raw,
            ):
                candidate = Path(raw)
                package = candidate / name
                write_valid(package)
                with _RUN.zipfile.ZipFile(package, "a") as archive:
                    archive.writestr(member, b"integrity evidence")
                _corrupt_stored_zip_member(package, member)
                with self.assertRaisesRegex(_RUN.RunFailure, label):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_mobile_candidate_rejects_transient_external_archive(self) -> None:
        cases = (
            (
                "android-arm64",
                "shar.apk",
                _write_android_apk,
                "_is_android_apk",
                "Android APK",
            ),
            (
                "ios-arm64",
                "shar.ipa",
                _write_ios_ipa,
                "_is_ios_ipa",
                "iOS IPA",
            ),
        )
        for target_id, name, write_valid, validator_name, label in cases:
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(
                    prefix="shar-mobile-runtime-race-"
                ) as raw,
            ):
                root = Path(raw)
                candidate = root / "candidate"
                candidate.mkdir()
                package = candidate / name
                package.write_bytes(b"invalid local package")
                external = root / f"external-{name}"
                write_valid(external)
                displaced = root / f"displaced-{name}"
                real_validator = getattr(_RUN, validator_name)

                def transient_external(
                    path: Path,
                    *,
                    local_path: Path = package,
                    external_path: Path = external,
                    displaced_path: Path = displaced,
                    validate: Callable[[Path], bool] = real_validator,
                ) -> bool:
                    local_path.replace(displaced_path)
                    local_path.symlink_to(external_path)
                    try:
                        return validate(path)
                    finally:
                        local_path.unlink()
                        displaced_path.replace(local_path)

                with (
                    mock.patch.object(
                        _RUN,
                        validator_name,
                        side_effect=transient_external,
                    ),
                    self.assertRaisesRegex(_RUN.RunFailure, label),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

                self.assertEqual(package.read_bytes(), b"invalid local package")
                self.assertTrue(external.is_file())

    def test_linux_candidate_requires_shar_executable(self) -> None:
        for target_id, platform in (
            ("linux-arm64", "LinuxArm64"),
            ("linux-x64", "Linux"),
        ):
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(
                    prefix="shar-linux-candidate-",
                ) as raw,
            ):
                candidate = Path(raw)
                (candidate / "README.txt").write_text(
                    "not a runtime\n",
                    encoding="utf-8",
                )
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

                binary = (
                    candidate
                    / "shar"
                    / "Binaries"
                    / platform
                    / f"shar-{platform}-Shipping"
                )
                binary.parent.mkdir(parents=True)
                binary.write_bytes(b"not an ELF")
                if _RUN.os.name != "nt":
                    binary.chmod(0o755)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )
                machine = 0x00B7 if target_id == "linux-arm64" else 0x003E
                header = bytearray(_synthetic_elf(machine))
                binary.write_bytes(header)
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID[target_id],
                )

                header[16:18] = (1).to_bytes(2, "little")
                binary.write_bytes(header)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

                header = bytearray(_synthetic_elf(machine))
                header[64:68] = (0).to_bytes(4, "little")
                binary.write_bytes(header)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

                binary.write_bytes(
                    _synthetic_elf(machine, segment_offset=4096)
                )
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

                binary.write_bytes(
                    _synthetic_elf(
                        machine,
                        segment_file_size=2,
                        segment_memory_size=1,
                    )
                )
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

                for alignment, segment_offset in (
                    (3, 120),
                    (0x1000, 120),
                ):
                    payload = bytearray(
                        _synthetic_elf(machine, segment_offset=segment_offset)
                    )
                    payload[112:120] = alignment.to_bytes(8, "little")
                    binary.write_bytes(payload)
                    with (
                        self.subTest(
                            target=target_id,
                            alignment=alignment,
                            segment_offset=segment_offset,
                        ),
                        self.assertRaisesRegex(
                            _RUN.RunFailure,
                            "Linux SHAR executable",
                        ),
                    ):
                        _RUN._validate_candidate_artifact(
                            candidate,
                            _RUN._TARGETS_BY_ID[target_id],
                        )

                header = bytearray(_synthetic_elf(machine))
                header[68:72] = (0x4).to_bytes(4, "little")
                binary.write_bytes(header)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

                for entrypoint in (0, 0x500000):
                    binary.write_bytes(
                        _synthetic_elf(machine, entrypoint=entrypoint)
                    )
                    with (
                        self.subTest(target=target_id, entrypoint=entrypoint),
                        self.assertRaisesRegex(
                            _RUN.RunFailure,
                            "Linux SHAR executable",
                        ),
                    ):
                        _RUN._validate_candidate_artifact(
                            candidate,
                            _RUN._TARGETS_BY_ID[target_id],
                        )

                header = bytearray(_synthetic_elf(machine))
                wrong_machine = 0x003E if machine == 0x00B7 else 0x00B7
                header[18:20] = wrong_machine.to_bytes(2, "little")
                binary.write_bytes(header)
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_rejects_runtime_replacement_before_signature_read(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-native-runtime-race-"
        ) as raw:
            root = Path(raw)
            candidate = root / "candidate"
            runtime = (
                candidate
                / "shar/Binaries/Linux/shar-Linux-Shipping"
            )
            runtime.parent.mkdir(parents=True)
            runtime.write_bytes(b"not an ELF runtime")
            runtime.chmod(0o755)
            external = root / "external-runtime"
            external.write_bytes(_synthetic_elf(0x003E))
            external.chmod(0o755)
            displaced = root / "displaced-runtime"
            real_identity = _RUN._real_file_identity
            replaced = False

            def replace_after_identity(
                path: Path, label: str
            ) -> tuple[int, ...]:
                nonlocal replaced
                identity = real_identity(path, label)
                if path == runtime and not replaced:
                    runtime.replace(displaced)
                    runtime.symlink_to(external)
                    replaced = True
                return identity

            with (
                mock.patch.object(
                    _RUN,
                    "_real_file_identity",
                    side_effect=replace_after_identity,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ),
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["linux-x64"],
                )

            self.assertTrue(runtime.is_symlink())
            self.assertEqual(displaced.read_bytes(), b"not an ELF runtime")
            self.assertEqual(external.read_bytes(), _synthetic_elf(0x003E))

    def test_macos_candidate_requires_shar_app_runtime(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-candidate-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(b"")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

            executable.write_bytes(b"not Mach-O")
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )
            macho = bytearray(_synthetic_macho(_RUN._MACHO_ARM64_CPU))
            executable.write_bytes(macho)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["macos-arm64"],
            )

            macho[12:16] = (6).to_bytes(4, "little")
            executable.write_bytes(macho)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

            macho = bytearray(
                _synthetic_macho(_RUN._MACHO_ARM64_CPU, command=0x1B)
            )
            executable.write_bytes(macho)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

            executable.write_bytes(
                _synthetic_macho(
                    _RUN._MACHO_ARM64_CPU,
                    entry_offset=4096,
                )
            )
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

            executable.write_bytes(
                _synthetic_macho(
                    _RUN._MACHO_ARM64_CPU,
                    prefix_command_size=9,
                )
            )
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

            macho = bytearray(_synthetic_macho(_RUN._MACHO_ARM64_CPU))
            macho[4:8] = (0x01000007).to_bytes(4, "little")
            executable.write_bytes(macho)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "macOS SHAR app bundle",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["macos-arm64"],
                )

            fat_entry = (
                _RUN._MACHO_ARM64_CPU.to_bytes(4, "big") + (b"\0" * 16)
            )
            truncated_fat = (
                bytes.fromhex("cafebabe")
                + (2).to_bytes(4, "big")
                + fat_entry
            )
            zero_slice_fat = (
                bytes.fromhex("cafebabe")
                + (1).to_bytes(4, "big")
                + fat_entry
            )
            in_bounds_non_macho = (
                bytes.fromhex("cafebabe")
                + (1).to_bytes(4, "big")
                + _RUN._MACHO_ARM64_CPU.to_bytes(4, "big")
                + (b"\0" * 4)
                + (28).to_bytes(4, "big")
                + (8).to_bytes(4, "big")
                + (b"\0" * 4)
                + b"invalid!"
            )
            overlapping_fat = bytearray(_synthetic_fat_macho((
                _RUN._MACHO_ARM64_CPU,
                0x01000007,
            )))
            overlapping_fat[36:40] = overlapping_fat[16:20]
            misaligned_fat = bytearray(_synthetic_fat_macho((
                _RUN._MACHO_ARM64_CPU,
                0x01000007,
            )))
            misaligned_fat[24:28] = (15).to_bytes(4, "big")
            for malformed in (
                truncated_fat,
                zero_slice_fat,
                in_bounds_non_macho,
                bytes(overlapping_fat),
                bytes(misaligned_fat),
            ):
                with (
                    self.subTest(malformed_size=len(malformed)),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "macOS SHAR app bundle",
                    ),
                ):
                    executable.write_bytes(malformed)
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["macos-arm64"],
                    )

            executable.write_bytes(_synthetic_fat_macho((
                0x01000007,
                _RUN._MACHO_ARM64_CPU,
            )))
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["macos-arm64"],
            )

    def test_macos_legacy_thread_entrypoint_requires_arm64_state(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-macos-thread-") as raw:
            candidate = Path(raw)
            executable = candidate / "SHAR.app/Contents/MacOS/shar"
            executable.parent.mkdir(parents=True)
            executable.write_bytes(
                _synthetic_thread_entry_macho(_RUN._MACHO_ARM64_CPU)
            )
            if _RUN.os.name != "nt":
                executable.chmod(0o755)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["macos-arm64"],
            )

            invalid_threads = (
                (
                    "wrong-flavor",
                    _synthetic_thread_entry_macho(
                        _RUN._MACHO_ARM64_CPU,
                        flavor=1,
                    ),
                ),
                (
                    "wrong-count",
                    _synthetic_thread_entry_macho(
                        _RUN._MACHO_ARM64_CPU,
                        count=67,
                    ),
                ),
                (
                    "pc-inside-load-commands",
                    _synthetic_thread_entry_macho(
                        _RUN._MACHO_ARM64_CPU,
                        pc=0x100000080,
                    ),
                ),
                (
                    "zero-pc",
                    _synthetic_thread_entry_macho(
                        _RUN._MACHO_ARM64_CPU,
                        pc=0,
                    ),
                ),
            )
            for reason, invalid_thread in invalid_threads:
                executable.write_bytes(invalid_thread)
                with (
                    self.subTest(reason=reason),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "macOS SHAR app bundle",
                    ),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["macos-arm64"],
                    )

    def test_windows_candidate_requires_shar_executable(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-windows-candidate-",
        ) as raw:
            candidate = Path(raw)
            (candidate / "CrashReportClient.exe").write_bytes(b"engine helper")
            (candidate / "shareware.exe").write_bytes(b"lookalike")
            (candidate / "shar-helper.exe").write_bytes(b"prefixed helper")

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            executable = candidate / "shar-Win64-Shipping.exe"
            executable.write_bytes(b"not PE")
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )
            payload = bytearray(_synthetic_pe(0x8664))
            executable.write_bytes(payload)
            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["windows-x64"],
            )

            executable.write_bytes(_synthetic_pe(0x8664, section_count=0))
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            executable.write_bytes(_synthetic_pe(0x8664, section_count=97))
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            executable.write_bytes(_synthetic_pe(0x8664, characteristics=0))
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            executable.write_bytes(
                _synthetic_pe(0x8664, characteristics=0x2002)
            )
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            executable.write_bytes(
                _synthetic_pe(0x8664, section_characteristics=0x40000040)
            )
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            executable.write_bytes(
                _synthetic_pe(0x8664, section_raw_offset=4096)
            )
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            for entrypoint in (0, 0x2000):
                payload = bytearray(_synthetic_pe(0x8664))
                payload[0xA8:0xAC] = entrypoint.to_bytes(4, "little")
                executable.write_bytes(payload)
                with (
                    self.subTest(entrypoint=entrypoint),
                    self.assertRaisesRegex(
                        _RUN.RunFailure,
                        "Windows SHAR executable",
                    ),
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID["windows-x64"],
                    )

            payload = bytearray(_synthetic_pe(0x8664))
            payload[0x98:0x9A] = bytes.fromhex("0b01")
            executable.write_bytes(payload)
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

            executable.write_bytes(_synthetic_pe(0xAA64))
            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "Windows SHAR executable",
            ):
                _RUN._validate_candidate_artifact(
                    candidate,
                    _RUN._TARGETS_BY_ID["windows-x64"],
                )

    def test_candidate_rejects_empty_declared_artifact(self) -> None:
        cases = (
            ("android-arm64", "shar.apk", "valid ARM64 Android APK"),
            ("ios-arm64", "shar.ipa", "valid ARM64 iOS IPA"),
            (
                "windows-x64",
                "shar.exe",
                "non-empty Windows SHAR executable",
            ),
        )
        for target_id, filename, label in cases:
            with (
                self.subTest(target=target_id),
                tempfile.TemporaryDirectory(
                    prefix="shar-empty-mobile-candidate-",
                ) as raw,
            ):
                candidate = Path(raw)
                (candidate / filename).write_bytes(b"")
                with self.assertRaisesRegex(
                    _RUN.RunFailure,
                    f"no {label}",
                ):
                    _RUN._validate_candidate_artifact(
                        candidate,
                        _RUN._TARGETS_BY_ID[target_id],
                    )

    def test_mobile_artifact_may_be_nested_in_uat_archive(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-mobile-candidate-",
        ) as raw:
            candidate = Path(raw)
            nested = candidate / "package"
            nested.mkdir()
            _write_android_apk(nested / "shar.apk")

            _RUN._validate_candidate_artifact(
                candidate,
                _RUN._TARGETS_BY_ID["android-arm64"],
            )

    def test_build_revalidates_runtime_after_diagnostic_caching(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-runtime-drift-") as raw:
            root = Path(raw)
            target = _RUN._TARGETS_BY_ID["linux-x64"]
            runtime_path: Path | None = None

            def write_candidate(
                _root: Path,
                _uat: Path,
                arguments: list[str],
                _log: Path,
            ) -> None:
                nonlocal runtime_path
                archive = next(
                    value
                    for value in arguments
                    if value.startswith("-ArchiveDirectory=")
                )
                candidate = Path(archive.split("=", 1)[1])
                runtime_path = (
                    candidate / "shar/Binaries/Linux/shar-Linux-Shipping"
                )
                runtime_path.parent.mkdir(parents=True)
                runtime_path.write_bytes(_synthetic_elf(0x003E))
                runtime_path.chmod(0o755)

            real_cache = _RUN._cache_nonruntime_artifacts

            def drift_after_cache(
                candidate: Path,
                work: Path,
                selected: object,
                tree: object,
            ) -> None:
                real_cache(candidate, work, selected, tree)
                if runtime_path is None:
                    raise AssertionError("runtime fixture was not created")
                runtime_path.write_bytes(b"drifted runtime")

            with (
                mock.patch.object(_RUN, "_verify_sdk"),
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_candidate,
                ),
                mock.patch.object(
                    _RUN,
                    "_cache_nonruntime_artifacts",
                    side_effect=drift_after_cache,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Linux SHAR executable",
                ),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project/shar.uproject"),
                    target,
                    validate_only=False,
                )

            self.assertFalse((root / "dist/linux-x64").exists())
            self.assertIsNotNone(runtime_path)
            if runtime_path is not None:
                self.assertEqual(runtime_path.read_bytes(), b"drifted runtime")

    def test_build_rejects_wrong_mobile_artifact_before_publication(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-mobile-build-") as raw:
            root = Path(raw)
            target = _RUN._TARGETS_BY_ID["android-arm64"]

            def write_wrong_archive(
                _root: Path,
                _uat: Path,
                arguments: list[str],
                _log: Path,
            ) -> None:
                archive = next(
                    value
                    for value in arguments
                    if value.startswith("-ArchiveDirectory=")
                )
                candidate = Path(archive.split("=", 1)[1])
                (candidate / "not-an-apk.txt").write_text(
                    "wrong artifact\n",
                    encoding="utf-8",
                )

            with (
                mock.patch.object(_RUN, "_verify_sdk"),
                mock.patch.object(
                    _RUN,
                    "_run_uat",
                    side_effect=write_wrong_archive,
                ),
                mock.patch.object(
                    _RUN,
                    "_cache_nonruntime_artifacts",
                ) as cache_artifacts,
                self.assertRaisesRegex(_RUN.RunFailure, "Android APK"),
            ):
                _RUN._build_target(
                    root,
                    Path("/uat"),
                    Path("/project/shar.uproject"),
                    target,
                    validate_only=False,
                )

            cache_artifacts.assert_not_called()
            self.assertFalse((root / "dist/android-arm64").exists())


class ArchitectureRevalidationTests(unittest.TestCase):
    """Require direct runner use to consume stable revalidated evidence."""

    def test_invokes_arch_revalidation_for_exact_saved_path(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-arch-revalidate-") as raw:
            root = Path(raw)
            arch_path = root / ".cache/build/data/arch.json"
            arch_path.parent.mkdir(parents=True)
            arch_path.write_bytes(b"stable architecture evidence")
            result = mock.Mock(returncode=0)

            with mock.patch.object(
                _RUN.subprocess,
                "run",
                return_value=result,
            ) as run:
                snapshot = _RUN._revalidate_arch(root, arch_path)

            self.assertEqual(snapshot, b"stable architecture evidence")
            run.assert_called_once_with(
                [
                    _RUN.sys.executable,
                    str(root / "tools/build/adapter-inbound/arch.py"),
                    "--revalidate",
                    "--output",
                    str(arch_path),
                    "--expected-sha256",
                    hashlib.sha256(
                        b"stable architecture evidence"
                    ).hexdigest(),
                ],
                cwd=root,
                check=False,
            )

    def test_rejects_failed_architecture_revalidation(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-arch-revalidate-") as raw:
            root = Path(raw)
            arch_path = root / ".cache/build/data/arch.json"
            arch_path.parent.mkdir(parents=True)
            arch_path.write_bytes(b"saved architecture evidence")
            result = mock.Mock(returncode=7)

            with (
                mock.patch.object(
                    _RUN.subprocess,
                    "run",
                    return_value=result,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "architecture decision did not revalidate",
                ),
            ):
                _RUN._revalidate_arch(root, arch_path)

    def test_rejects_architecture_drift_during_revalidation(self) -> None:
        with tempfile.TemporaryDirectory(prefix="shar-arch-revalidate-") as raw:
            root = Path(raw)
            arch_path = root / ".cache/build/data/arch.json"
            arch_path.parent.mkdir(parents=True)
            arch_path.write_bytes(b"before")

            def replace_evidence(
                *_args: object,
                **_kwargs: object,
            ) -> mock.Mock:
                arch_path.write_bytes(b"after")
                return mock.Mock(returncode=0)

            with (
                mock.patch.object(
                    _RUN.subprocess,
                    "run",
                    side_effect=replace_evidence,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "architecture decision changed during revalidation",
                ),
            ):
                _RUN._revalidate_arch(root, arch_path)

    def test_rejects_preflight_drift_during_revalidation(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-check-revalidate-",
        ) as raw:
            root = Path(raw)
            check_path = root / ".cache/build/data/check.json"
            check_path.parent.mkdir(parents=True)
            check_path.write_bytes(b"before")

            def replace_evidence(
                *_args: object,
                **_kwargs: object,
            ) -> mock.Mock:
                check_path.write_bytes(b"after")
                return mock.Mock(returncode=0)

            with (
                mock.patch.object(
                    _RUN.subprocess,
                    "run",
                    side_effect=replace_evidence,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "build preflight changed during revalidation",
                ),
            ):
                _RUN._revalidate_check(root, check_path)

    def test_project_evidence_rejects_descriptor_drift(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-run-project-drift-"
        ) as raw:
            root = Path(raw)
            project = root / _RUN._PROJECT_PATH
            project.parent.mkdir(parents=True)
            project.write_text(
                '{"EngineAssociation":"5.8"}\n',
                encoding="utf-8",
            )
            unreal = {
                "project": str(project),
                "project_sha256": "0" * 64,
            }

            with self.assertRaisesRegex(
                _RUN.RunFailure,
                "project descriptor no longer matches preflight",
            ):
                _RUN._project_from_evidence(root, unreal)

    @unittest.skipIf(os.name == "nt", "symlink setup is Unix-focused")
    def test_project_evidence_rejects_replacement_during_read(self) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-run-project-race-"
        ) as raw:
            root = Path(raw)
            project = root / _RUN._PROJECT_PATH
            project.parent.mkdir(parents=True)
            payload = b'{"EngineAssociation":"5.8"}\n'
            project.write_bytes(payload)
            external = root / "external.uproject"
            external.write_bytes(payload)
            displaced = root / "displaced.uproject"
            unreal = {
                "project": str(project),
                "project_sha256": hashlib.sha256(payload).hexdigest(),
            }
            real_identity = _RUN._real_file_identity
            replaced = False

            def replace_after_identity(
                path: Path, label: str
            ) -> tuple[int, ...]:
                nonlocal replaced
                identity = real_identity(path, label)
                if path == project and not replaced:
                    project.replace(displaced)
                    project.symlink_to(external)
                    replaced = True
                return identity

            with (
                mock.patch.object(
                    _RUN,
                    "_real_file_identity",
                    side_effect=replace_after_identity,
                ),
                self.assertRaisesRegex(
                    _RUN.RunFailure,
                    "Unreal project descriptor changed while reading",
                ),
            ):
                _RUN._project_from_evidence(root, unreal)

            self.assertTrue(project.is_symlink())
            self.assertEqual(displaced.read_bytes(), payload)
            self.assertEqual(external.read_bytes(), payload)

    def test_project_evidence_accepts_current_canonical_descriptor(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory(
            prefix="shar-run-project-current-"
        ) as raw:
            root = Path(raw)
            project = root / _RUN._PROJECT_PATH
            project.parent.mkdir(parents=True)
            payload = b'{"EngineAssociation":"5.8"}\n'
            project.write_bytes(payload)
            unreal = {
                "project": str(project),
                "project_sha256": hashlib.sha256(payload).hexdigest(),
            }

            self.assertEqual(
                _RUN._project_from_evidence(root, unreal),
                project,
            )

    def test_main_consumes_revalidated_snapshots(self) -> None:
        root = Path("/repo")
        arch_snapshot = b"validated arch"
        check_snapshot = b"validated check"
        unreal = {
            "project": "/repo/project/shar.uproject",
            "project_sha256": "a" * 64,
            "root": "/engine",
            "version": "5.8.1",
        }
        with (
            mock.patch.object(_RUN, "_root", return_value=root),
            mock.patch.object(
                _RUN,
                "_revalidate_arch",
                return_value=arch_snapshot,
            ) as revalidate_arch,
            mock.patch.object(
                _RUN,
                "_selected_targets",
                return_value=[],
            ) as selected_targets,
            mock.patch.object(
                _RUN,
                "_revalidate_check",
                return_value=check_snapshot,
            ) as revalidate_check,
            mock.patch.object(
                _RUN,
                "_check_evidence",
                return_value={"unreal": unreal},
            ) as check_evidence,
            mock.patch.object(
                _RUN,
                "_require_unreal_evidence",
                return_value=unreal,
            ),
            mock.patch.object(
                _RUN,
                "_project_from_evidence",
                return_value=Path("/repo/project/shar.uproject"),
            ),
            mock.patch.object(_RUN, "_prepare_project_state"),
            mock.patch.object(_RUN, "_uat_path", return_value=Path("/uat")),
            mock.patch.object(_RUN.sys, "argv", ["run.py", "--validate-only"]),
        ):
            self.assertEqual(_RUN.main(), 0)

        arch_path = root / _RUN._ARCH_PATH
        check_path = root / _RUN._CHECK_PATH
        revalidate_arch.assert_called_once_with(root, arch_path)
        selected_targets.assert_called_once_with(arch_snapshot)
        revalidate_check.assert_called_once_with(root, check_path)
        check_evidence.assert_called_once_with(check_snapshot)


if __name__ == "__main__":
    unittest.main()

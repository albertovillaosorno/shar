# File:
#   - filesystem_skill_store.py
# Path:
#   - src/mcp/src/adapters/driven/filesystem_skill_store.py
#
# Copyright:
#   - Copyright (c) 2026 Alberto Villa Osorno.
# SPDX-License-Identifier:
#   - MIT
# Confidential:
#   - false
# License-File:
#   - LICENSE
# Path-Rule:
#   - All paths in this header are repository-root relative.
#
# Boundary-Contract:
# - Owns:
#   - Safe generated replacement with protected manual-field preservation.
# - Must-Not:
#   - Modify workflows, invoke Unreal, or interpret live MCP metadata.
# - Allows:
#   - Removing stale tools and migrating human fields by native identity.
# - Split-When:
#   - Atomic writes and stale-file cleanup need independent adapters.
# - Merge-When:
#   - Another filesystem adapter owns the same generated skill surface.
# - Summary:
#   - Persists generated Unreal MCP skill documents safely.
# - Description:
#   - Restricts writes to generated indexes and capability files.
# - Usage:
#   - Injected into the Unreal skill export application use case.
# - Defaults:
#   - Fails before mutation when a retained manual field is malformed.
#
# ADRs:
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: generated Unreal skill filesystem storage
#   - reason: safe paths, atomic writes, and stale cleanup form one boundary
#   - split: extract stale cleanup if generated ownership expands
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess after generated output ownership changes
#
"""Filesystem storage for generated native Unreal MCP skills."""

from __future__ import annotations

from pathlib import Path, PurePosixPath
from typing import TYPE_CHECKING

from mcp.src.adapters.driven.skill_index_renderer import (
    replace_manual_review_summary,
)
from mcp.src.adapters.driven.skill_manual_fields import (
    extract_manual_fields,
    merge_manual_fields,
)
from mcp.src.adapters.driven.skill_manual_review import (
    MANUAL_REVIEW_FIELD_KEY,
    manual_review_state,
)
from mcp.src.adapters.driven.skill_native_identity import (
    extract_native_tool_identity,
)
from mcp.src.domain.errors import fail_protocol

if TYPE_CHECKING:
    from mcp.src.domain.skill_documents import SkillDocument

_INDEX_PATH = PurePosixPath("index.md")
_CAPABILITIES_ROOT = PurePosixPath("capabilities")


class FilesystemSkillStore:
    """Replace only the generator-owned Unreal skill document surface."""

    def __init__(self, output_root: Path) -> None:
        """Create a store rooted at one skill directory."""
        self._output_root = output_root

    @property
    def display_path(self) -> str:
        """The operator-facing output path."""
        return self._output_root.as_posix()

    def replace(self, documents: tuple[SkillDocument, ...]) -> None:
        """Persist a complete generated document set."""
        targets = _checked_targets(documents)
        merged_targets = self._merge_existing_manual_fields(targets)
        finalized_targets = self._finalize_manual_review_summary(merged_targets)
        self._output_root.mkdir(parents=True, exist_ok=True)
        self._remove_stale_capability_files(frozenset(finalized_targets))
        for relative_path, content in sorted(finalized_targets.items()):
            self._write_atomic(relative_path, content)
        self._remove_empty_capability_directories()

    def _merge_existing_manual_fields(
        self,
        targets: dict[str, str],
    ) -> dict[str, str]:
        existing_by_identity = self._existing_capabilities_by_identity()
        merged: dict[str, str] = {}
        for relative_path, generated_content in targets.items():
            path = PurePosixPath(relative_path)
            if path == _INDEX_PATH:
                merged[relative_path] = generated_content
                continue
            target = self._output_root.joinpath(*path.parts)
            if target.exists() and not target.is_file():
                fail_protocol(
                    f"generated skill target is not a file: {relative_path}"
                )
            identity = extract_native_tool_identity(
                generated_content,
                context=relative_path,
            )
            existing = existing_by_identity.get(identity)
            existing_content = existing[1] if existing is not None else None
            merged[relative_path] = merge_manual_fields(
                generated_content,
                existing_content,
                context=relative_path,
            )
        return merged

    @staticmethod
    def _finalize_manual_review_summary(
        targets: dict[str, str],
    ) -> dict[str, str]:
        current_count = 0
        review_required_count = 0
        for relative_path, content in targets.items():
            path = PurePosixPath(relative_path)
            if path == _INDEX_PATH:
                continue
            values = extract_manual_fields(
                content,
                context=relative_path,
                require_complete=True,
            )
            reviewed_revision = values[MANUAL_REVIEW_FIELD_KEY]
            state = manual_review_state(
                content,
                reviewed_revision,
                context=relative_path,
            )
            if state.is_current:
                current_count += 1
            else:
                review_required_count += 1
        index_key = _INDEX_PATH.as_posix()
        finalized = dict(targets)
        finalized[index_key] = replace_manual_review_summary(
            targets[index_key],
            current_count=current_count,
            review_required_count=review_required_count,
        )
        return finalized

    def _existing_capabilities_by_identity(
        self,
    ) -> dict[str, tuple[str, str]]:
        capabilities_root = self._output_root / _CAPABILITIES_ROOT
        if not capabilities_root.exists():
            return {}
        existing: dict[str, tuple[str, str]] = {}
        for path in sorted(capabilities_root.rglob("*.md")):
            relative_path = path.relative_to(self._output_root).as_posix()
            content = path.read_text(encoding="utf-8")
            if (
                "## Native identities" not in content
                and "MANUAL FIELD:" not in content
            ):
                continue
            identity = extract_native_tool_identity(
                content,
                context=relative_path,
            )
            previous = existing.get(identity)
            if previous is not None:
                message = (
                    f"duplicate native tool identity in generated skills: "
                    f"{identity}: {previous[0]} and {relative_path}"
                )
                fail_protocol(message)
            existing[identity] = (relative_path, content)
        return existing

    def _remove_stale_capability_files(
        self,
        expected_paths: frozenset[str],
    ) -> None:
        capabilities_root = self._output_root / _CAPABILITIES_ROOT
        if not capabilities_root.exists():
            return
        for path in capabilities_root.rglob("*.md"):
            relative_path = path.relative_to(self._output_root).as_posix()
            if relative_path not in expected_paths:
                path.unlink()

    def _write_atomic(self, relative_path: str, content: str) -> None:
        target = self._output_root.joinpath(*PurePosixPath(relative_path).parts)
        target.parent.mkdir(parents=True, exist_ok=True)
        temporary = target.with_name(f".{target.name}.tmp")
        _ = temporary.write_text(content, encoding="utf-8", newline="\n")
        _ = temporary.replace(target)

    def _remove_empty_capability_directories(self) -> None:
        capabilities_root = self._output_root / _CAPABILITIES_ROOT
        if not capabilities_root.exists():
            return
        directories = sorted(
            (path for path in capabilities_root.rglob("*") if path.is_dir()),
            key=lambda path: len(path.parts),
            reverse=True,
        )
        for directory in directories:
            if not any(directory.iterdir()):
                directory.rmdir()


def _checked_targets(documents: tuple[SkillDocument, ...]) -> dict[str, str]:
    targets: dict[str, str] = {}
    for document in documents:
        path = PurePosixPath(document.relative_path)
        if path.is_absolute() or ".." in path.parts:
            fail_protocol(
                f"unsafe generated skill path: {document.relative_path}"
            )
        if path != _INDEX_PATH and (
            not path.parts or path.parts[0] != _CAPABILITIES_ROOT.name
        ):
            fail_protocol(
                f"generated skill path is outside owned surface: {path}"
            )
        normalized = path.as_posix()
        if normalized in targets:
            fail_protocol(f"duplicate generated skill path: {normalized}")
        if not document.content.endswith("\n"):
            fail_protocol(f"generated skill lacks final newline: {normalized}")
        targets[normalized] = document.content
    if _INDEX_PATH.as_posix() not in targets:
        fail_protocol("generated skill set omitted index.md")
    return targets

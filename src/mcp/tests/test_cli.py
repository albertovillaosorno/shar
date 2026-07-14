# File:
#   - test_cli.py
# Path:
#   - src/mcp/tests/test_cli.py
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
#   - End-to-end tests for the terminal driving adapter.
# - Must-Not:
#   - Require Unreal binaries, plugin code, or external networks.
# - Allows:
#   - Synthetic CLI lifecycle, discovery, and invocation tests.
# - Split-When:
#   - The module gains two independently testable contracts.
# - Merge-When:
#   - Another module owns the same contract without a distinct invariant.
# - Summary:
#   - Guards terminal behavior and stable exit codes.
# - Description:
#   - Exercises validated commands against the fake MCP server.
# - Usage:
#   - Executed by pytest through the canonical validator workflow.
# - Defaults:
#   - Each integration test uses an ephemeral loopback endpoint.
#
# ADRs:
# - docs/adr/unreal/mcp/native-unreal-mcp-terminal-bridge.md
# - docs/adr/unreal/mcp/native-tool-cli-projection-and-skills.md
#
# Large file:
#   - true
# LARGE-FILE:
#   - owner: terminal adapter regression tests
#   - reason: exit-code and output assertions share one CLI fixture family
#   - split: split by command family when additional commands are introduced
#   - validation: bash validate.sh --refresh-cache src/mcp/
#   - review: reassess on responsibility or line-count growth
#
from __future__ import annotations

import json
from typing import TYPE_CHECKING, cast

from mcp.src.adapters.driving.cli import main
from mcp.src.domain.json_types import require_json_object

from tests.fake_unreal_server import FakeUnrealServer

if TYPE_CHECKING:
    import pytest


def test_unknown_command_fails_before_opening_a_session(
    capsys: pytest.CaptureFixture[str],
) -> None:
    exit_code = main(("unknown",))
    captured = capsys.readouterr()
    assert exit_code == 2
    assert "unknown command" in captured.err
    assert not captured.out


def test_cli_rejects_duplicate_argument_keys_before_session(
    capsys: pytest.CaptureFixture[str],
) -> None:
    exit_code = main(
        (
            "raw-call",
            "call_tool",
            "--arguments",
            '{"name":"first","name":"second"}',
        )
    )
    captured = capsys.readouterr()

    assert exit_code == 2
    assert "duplicate JSON key: name" in captured.err
    assert not captured.out


def test_cli_doctor_and_toolset_discovery(
    capsys: pytest.CaptureFixture[str],
) -> None:
    with FakeUnrealServer() as server:
        doctor_code = main(("--endpoint", server.endpoint, "doctor"))
        doctor_output = capsys.readouterr()
        doctor_payload = require_json_object(
            cast("object", json.loads(doctor_output.out)),
            context="doctor output",
        )
        assert doctor_code == 0
        assert doctor_payload["ready"] is True
        assert doctor_payload["missingMetaTools"] == []
        assert doctor_payload["toolsetCount"] == 1

        toolsets_code = main(("--endpoint", server.endpoint, "toolsets"))
        toolsets_output = capsys.readouterr()
        toolsets_payload = require_json_object(
            cast("object", json.loads(toolsets_output.out)),
            context="toolsets output",
        )
        assert toolsets_code == 0
        assert "EditorToolset" in toolsets_output.out
        assert "toolsets" in toolsets_payload
        assert server.session_closed


def test_cli_describe_call_and_markdown_catalog(
    capsys: pytest.CaptureFixture[str],
) -> None:
    with FakeUnrealServer() as server:
        describe_code = main(
            ("--endpoint", server.endpoint, "describe", "EditorToolset")
        )
        describe_output = capsys.readouterr()
        assert describe_code == 0
        assert "create_asset" in describe_output.out

        call_code = main(
            (
                "--endpoint",
                server.endpoint,
                "call",
                "EditorToolset",
                "EditorToolset.create_asset",
                "--arguments",
                '{"name":"A"}',
            )
        )
        call_output = capsys.readouterr()
        assert call_code == 0
        assert "native-ok:create_asset" in call_output.out

        catalog_code = main(
            (
                "--endpoint",
                server.endpoint,
                "catalog",
                "--format",
                "markdown",
            )
        )
        catalog_output = capsys.readouterr()
        assert catalog_code == 0
        assert "# Unreal native MCP tool catalog" in catalog_output.out
        assert "`EditorToolset.EditorToolset`" in catalog_output.out
        assert (
            "`EditorToolset.EditorToolset.create_asset`" in catalog_output.out
        )


def test_cli_rejects_unsafe_skill_output_before_network(
    capsys: pytest.CaptureFixture[str],
) -> None:
    """An escaping output path fails locally without opening an MCP session."""
    code = main(
        (
            "--endpoint",
            "http://127.0.0.1:65534/mcp",
            "skills",
            "--output",
            "../outside",
        )
    )
    captured = capsys.readouterr()

    assert code == 2
    assert "repository-relative child path" in captured.err
    assert not captured.out


def test_cli_doctor_rejects_empty_toolset_registry(
    capsys: pytest.CaptureFixture[str],
) -> None:
    """Registered meta-tools are insufficient without a usable toolset."""
    with FakeUnrealServer(empty_toolsets=True) as server:
        code = main(("--endpoint", server.endpoint, "doctor"))
        captured = capsys.readouterr()
        payload = require_json_object(
            cast("object", json.loads(captured.out)),
            context="empty registry doctor output",
        )

        assert code == 1
        assert payload["ready"] is False
        assert payload["missingMetaTools"] == []
        assert payload["toolsetCount"] == 0

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
import sys
from typing import TYPE_CHECKING, cast

from mcp.src.adapters.driving.arguments import UsageError
from mcp.src.adapters.driving.cli import main
from mcp.src.domain.json_types import require_json_object

from tests.fake_unreal_server import FakeUnrealServer

if TYPE_CHECKING:
    import pytest


def test_cli_maps_stdout_unicode_error_to_failure(
    monkeypatch: pytest.MonkeyPatch,
    capsys: pytest.CaptureFixture[str],
) -> None:
    """Console encoding failures return one stable runtime exit code."""

    def reject_output(value: str) -> None:
        codec = "ascii"
        reason = "ordinal not in range"
        raise UnicodeEncodeError(
            codec,
            value,
            0,
            1,
            reason,
        )

    monkeypatch.setattr(
        "mcp.src.adapters.driving.cli._write_stdout",
        reject_output,
    )

    exit_code = main(("help",))
    captured = capsys.readouterr()

    assert exit_code == 1
    assert "error:" in captured.err
    assert not captured.out


def test_cli_escapes_stderr_when_console_rejects_unicode(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """Runtime diagnostics remain deliverable on ASCII-only consoles."""

    class AsciiWriter:
        def __init__(self) -> None:
            self.values: list[str] = []

        def write(self, value: str) -> int:
            _ = value.encode("ascii")
            self.values.append(value)
            return len(value)

    def reject_arguments(_arguments: tuple[str, ...]) -> None:
        message = "invalid snowman ☃"
        raise UsageError(message)

    writer = AsciiWriter()
    monkeypatch.setattr(
        "mcp.src.adapters.driving.cli.parse_invocation",
        reject_arguments,
    )
    monkeypatch.setattr(
        "mcp.src.adapters.driving.cli.sys.stderr",
        writer,
    )

    exit_code = main(("doctor",))

    assert exit_code == 2
    rendered = "".join(writer.values)
    assert "error: invalid snowman \\u2603" in rendered
    assert "☃" not in rendered


def test_cli_returns_exit_code_when_stderr_is_unavailable(
    monkeypatch: pytest.MonkeyPatch,
) -> None:
    """A broken diagnostic stream must not replace the stable exit code."""

    class BrokenWriter:
        def __init__(self) -> None:
            self.calls = 0

        def write(self, _value: str) -> int:
            self.calls += 1
            message = "stderr is unavailable"
            raise OSError(message)

    def reject_arguments(_arguments: tuple[str, ...]) -> None:
        message = "invalid arguments"
        raise UsageError(message)

    writer = BrokenWriter()
    monkeypatch.setattr(
        "mcp.src.adapters.driving.cli.parse_invocation",
        reject_arguments,
    )
    monkeypatch.setattr(
        "mcp.src.adapters.driving.cli.sys.stderr",
        writer,
    )

    assert main(("doctor",)) == 2
    assert writer.calls == 1


def test_unknown_command_fails_before_opening_a_session(
    capsys: pytest.CaptureFixture[str],
) -> None:
    exit_code = main(("unknown",))
    captured = capsys.readouterr()
    assert exit_code == 2
    assert "unknown command" in captured.err
    assert not captured.out


def test_unknown_command_error_escapes_untrusted_text(
    capsys: pytest.CaptureFixture[str],
) -> None:
    """Unknown command evidence remains ASCII and single-line."""
    exit_code = main(("bad\ninjected-☃",))
    captured = capsys.readouterr()

    assert exit_code == 2
    assert "error: unknown command: bad\\ninjected-\\u2603" in captured.err
    assert "bad\ninjected" not in captured.err
    assert not captured.out


def test_cli_rejects_non_finite_timeout_before_session(
    capsys: pytest.CaptureFixture[str],
) -> None:
    for value in ("nan", "inf", "-inf"):
        exit_code = main(("--timeout", value, "doctor"))
        captured = capsys.readouterr()

        assert exit_code == 2
        assert "--timeout must be finite and positive" in captured.err
        assert not captured.out


def test_cli_rejects_invalid_operands_before_session(
    capsys: pytest.CaptureFixture[str],
) -> None:
    exit_code = main(
        (
            "--endpoint",
            "http://127.0.0.1:65534/mcp",
            "raw-call",
            "call_tool",
            "--arguments",
            "{",
        )
    )
    captured = capsys.readouterr()

    assert exit_code == 2
    assert "--arguments is not valid JSON" in captured.err
    assert "failed" not in captured.err
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


def test_cli_rejects_non_finite_json_arguments_as_usage(
    capsys: pytest.CaptureFixture[str],
) -> None:
    for literal in ("NaN", "Infinity", "-Infinity"):
        exit_code = main(
            (
                "raw-call",
                "call_tool",
                "--arguments",
                f'{{"value":{literal}}}',
            )
        )
        captured = capsys.readouterr()

        assert exit_code == 2
        assert "JSON number must be finite" in captured.err
        assert not captured.out


def test_cli_rejects_excessive_json_nesting_as_usage(
    capsys: pytest.CaptureFixture[str],
) -> None:
    depth = sys.getrecursionlimit() + 100
    nested = "[" * depth + "0" + "]" * depth
    exit_code = main(
        (
            "raw-call",
            "call_tool",
            "--arguments",
            f'{{"value":{nested}}}',
        )
    )
    captured = capsys.readouterr()

    assert exit_code == 2
    assert "JSON nesting is too deep" in captured.err
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


def test_cli_rejects_windows_rooted_skill_output_before_network(
    capsys: pytest.CaptureFixture[str],
) -> None:
    """Windows rooted paths must not escape the repository output root."""
    for output_path in (r"\outside", r"D:outside"):
        code = main(
            (
                "--endpoint",
                "http://127.0.0.1:65534/mcp",
                "skills",
                "--output",
                output_path,
            )
        )
        captured = capsys.readouterr()

        assert code == 2
        assert "repository-relative child path" in captured.err
        assert not captured.out


def test_cli_rejects_nonportable_skill_output_before_network(
    capsys: pytest.CaptureFixture[str],
) -> None:
    """Windows-invalid output segments fail before opening an MCP session."""
    for output_path in ("con", "skills/name:", "skills/trailing."):
        code = main(
            (
                "--endpoint",
                "http://127.0.0.1:65534/mcp",
                "skills",
                "--output",
                output_path,
            )
        )
        captured = capsys.readouterr()

        assert code == 2
        assert "portable path" in captured.err
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

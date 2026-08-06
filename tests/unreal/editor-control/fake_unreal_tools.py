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
#   - Deterministic native tool responses for the fake Unreal MCP server.
# - Must-Not:
#   - Access the filesystem, network, or real Unreal state.
# - Allows:
#   - Synthetic schemas, asset state, and structured native outcomes.
# - Split-When:
#   - Split when another fake native domain gains an independent lifecycle.
# - Merge-When:
#   - Merge when the fake protocol owns identical response construction.
# - Summary:
#   - Fake Unreal native tool responses.
# - Description:
#   - Supports generic protocol tests and opt-in plan application integration.
# - Usage:
#   - Called only by the fake Streamable HTTP request handler.
# - Defaults:
#   - Plan execution behavior is disabled unless explicitly requested.
#

"""Deterministic native tool responses for the fake Unreal MCP server."""

from __future__ import annotations

import json

from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import JsonValue
from mcp.domain.json_types import require_json_object

_ASSET_TOOLSET = "editor_toolset.toolsets.asset.AssetTools"
_TEXTURE_TOOLSET = "editor_toolset.toolsets.texture.TextureTools"
_IMPORT_TOOLSET = "SharImportEditor.SharImportToolset"


def tool_result(
    tool_name: object,
    arguments: JsonObject,
    *,
    empty_toolsets: bool,
    plan_execution: bool,
    assets: dict[str, str],
    dirty_assets: set[str],
) -> tuple[str, JsonValue | None]:
    """Return deterministic text and optional structured content."""
    if plan_execution:
        return _plan_tool_result(
            tool_name,
            arguments,
            assets=assets,
            dirty_assets=dirty_assets,
        )
    return tool_text(
        tool_name,
        arguments,
        empty_toolsets=empty_toolsets,
    ), None


def tool_text(
    tool_name: object,
    arguments: JsonObject,
    *,
    empty_toolsets: bool,
) -> str:
    """Return one deterministic legacy native tool response text."""
    if tool_name == "list_toolsets":
        if empty_toolsets:
            return ""
        return (
            "- EditorToolset.EditorToolset: Editor operations\n\n"
            "Provides:\n"
            "- create_asset: synthetic asset creation\n"
        )
    if tool_name == "describe_toolset":
        return json.dumps(_editor_schema(), separators=(",", ":"))
    if tool_name == "call_tool":
        native_name = arguments.get("tool_name", "unknown")
        return f"native-ok:{native_name}"
    return "raw-ok"


def _plan_tool_result(
    tool_name: object,
    arguments: JsonObject,
    *,
    assets: dict[str, str],
    dirty_assets: set[str],
) -> tuple[str, JsonValue | None]:
    if tool_name == "list_toolsets":
        return (
            f"- {_ASSET_TOOLSET}: Synthetic assets\n"
            f"- {_TEXTURE_TOOLSET}: Synthetic textures\n"
            f"- {_IMPORT_TOOLSET}: Synthetic SHAR imports\n",
            None,
        )
    if tool_name == "describe_toolset":
        toolset = arguments.get("toolset_name")
        schema = _plan_schema(toolset)
        return json.dumps(schema, separators=(",", ":")), None
    if tool_name != "call_tool":
        return "raw-ok", None
    native_name = arguments.get("tool_name")
    native_arguments = require_json_object(
        arguments.get("arguments", {}),
        context="fake native arguments",
    )
    structured = _plan_native_call(
        native_name,
        native_arguments,
        assets=assets,
        dirty_assets=dirty_assets,
    )
    return json.dumps(structured, separators=(",", ":")), structured


def _plan_native_call(
    native_name: JsonValue | None,
    arguments: JsonObject,
    *,
    assets: dict[str, str],
    dirty_assets: set[str],
) -> JsonObject:
    if native_name == "exists":
        return {"returnValue": str(arguments["path"]) in assets}
    if native_name in {"import_file", "ImportSoundWave"}:
        is_sound_wave = native_name == "ImportSoundWave"
        asset_name = str(
            arguments["assetName"] if is_sound_wave else arguments["asset_name"]
        )
        folder_path = str(
            arguments["folderPath"]
            if is_sound_wave
            else arguments["folder_path"]
        )
        package_path = "/".join((folder_path, asset_name))
        target_class = "SoundWave" if is_sound_wave else "Texture2D"
        assets[package_path] = target_class
        dirty_assets.add(package_path)
        if is_sound_wave:
            return {"returnValue": [f"{package_path}.{asset_name}"]}
        return {
            "returnValue": [
                {"assetClass": target_class, "packagePath": package_path}
            ]
        }
    if native_name == "get_asset_class":
        return {"returnValue": assets[str(arguments["asset_path"])]}
    if native_name == "save_assets":
        paths = arguments["asset_paths"]
        if not isinstance(paths, list):
            raise AssertionError("fake save paths are not an array")
        existing = all(str(path) in assets for path in paths)
        if existing:
            dirty_assets.difference_update(str(path) for path in paths)
        return {"returnValue": existing}
    if native_name == "is_dirty":
        return {"returnValue": str(arguments["asset_path"]) in dirty_assets}
    if native_name == "delete":
        path = str(arguments["path"])
        existed = assets.pop(path, None) is not None
        dirty_assets.discard(path)
        return {"returnValue": existed}
    raise AssertionError(f"unexpected fake native tool: {native_name}")


def _plan_schema(toolset: JsonValue | None) -> JsonObject:
    if toolset == _ASSET_TOOLSET:
        return _asset_schema()
    if toolset == _TEXTURE_TOOLSET:
        return _texture_schema()
    if toolset == _IMPORT_TOOLSET:
        return _import_schema()
    raise AssertionError(f"unexpected fake toolset: {toolset}")


def _editor_schema() -> JsonObject:
    return {
        "description": "Editor operations",
        "tools": [
            {
                "name": "create_asset",
                "description": "Create one synthetic asset.",
                "inputSchema": _object_schema(
                    {
                        "name": {
                            "type": "string",
                            "minLength": 1,
                            "pattern": "^[A-Za-z][A-Za-z0-9_]*$",
                        }
                    },
                    "name",
                ),
                "outputSchema": {"type": "object"},
            }
        ],
    }


def _asset_schema() -> JsonObject:
    text = {"type": "string"}
    boolean_output = _object_schema(
        {"returnValue": {"type": "boolean"}},
        "returnValue",
    )
    return {
        "description": "Synthetic asset lifecycle.",
        "tools": [
            {
                "name": "delete",
                "description": "Delete one synthetic asset.",
                "inputSchema": _object_schema({"path": text}, "path"),
                "outputSchema": boolean_output,
            },
            {
                "name": "exists",
                "description": "Test synthetic asset existence.",
                "inputSchema": _object_schema({"path": text}, "path"),
                "outputSchema": boolean_output,
            },
            {
                "name": "get_asset_class",
                "description": "Return one synthetic asset class.",
                "inputSchema": _object_schema(
                    {"asset_path": text},
                    "asset_path",
                ),
                "outputSchema": _object_schema(
                    {"returnValue": text},
                    "returnValue",
                ),
            },
            {
                "name": "is_dirty",
                "description": "Return synthetic package dirty state.",
                "inputSchema": _object_schema(
                    {"asset_path": text},
                    "asset_path",
                ),
                "outputSchema": boolean_output,
            },
            {
                "name": "save_assets",
                "description": "Save synthetic assets.",
                "inputSchema": _object_schema(
                    {
                        "asset_paths": {
                            "type": "array",
                            "items": text,
                            "minItems": 1,
                        }
                    },
                    "asset_paths",
                ),
                "outputSchema": boolean_output,
            },
        ],
    }


def _texture_schema() -> JsonObject:
    text = {"type": "string"}
    return {
        "description": "Synthetic texture import.",
        "tools": [
            {
                "name": "import_file",
                "description": "Import one synthetic texture.",
                "inputSchema": _object_schema(
                    {
                        "asset_name": text,
                        "folder_path": text,
                        "source_file": text,
                    },
                    "asset_name",
                    "folder_path",
                    "source_file",
                ),
                "outputSchema": _object_schema(
                    {
                        "returnValue": {
                            "type": "array",
                            "items": {"type": "object"},
                        }
                    },
                    "returnValue",
                ),
            }
        ],
    }


def _import_schema() -> JsonObject:
    text = {"type": "string"}
    return {
        "description": "Synthetic SHAR generated content import.",
        "tools": [
            {
                "name": "ImportSoundWave",
                "description": "Import one synthetic SoundWave.",
                "inputSchema": _object_schema(
                    {
                        "assetName": text,
                        "folderPath": text,
                        "sourceFile": text,
                    },
                    "assetName",
                    "folderPath",
                    "sourceFile",
                ),
                "outputSchema": _object_schema(
                    {
                        "returnValue": {
                            "type": "array",
                            "items": text,
                        }
                    },
                    "returnValue",
                ),
            }
        ],
    }


def _object_schema(
    properties: JsonObject,
    *required: str,
) -> JsonObject:
    return {
        "type": "object",
        "properties": properties,
        "required": list(required),
        "additionalProperties": False,
    }

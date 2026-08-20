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
#   - Fail-closed validation of native Unreal tool arguments.
# - Must-Not:
#   - Invoke tools, infer defaults, or reinterpret native schemas.
# - Allows:
#   - Validate strict JSON values against the supported live schema vocabulary.
# - Split-When:
#   - Split when schema compilation and instance validation diverge.
# - Merge-When:
#   - Merge when argument validation has no independent lifecycle.
# - Summary:
#   - Native tool argument schema validation.
# - Description:
#   - Rejects malformed schemas and invalid arguments before editor mutation.
# - Usage:
#   - Called after live tool discovery and before the native call meta-tool.
# - Defaults:
#   - Unsupported assertion keywords and ambiguous schemas fail closed.
#

"""Validate native Unreal tool arguments against live JSON Schema."""

from __future__ import annotations

from decimal import Decimal
from decimal import InvalidOperation
import json
import re
from typing import TYPE_CHECKING
from typing import cast

from mcp.domain.errors import ProtocolError
from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import JsonValue
from mcp.domain.json_types import require_json_object

if TYPE_CHECKING:
    from collections.abc import Iterable

_JSON_TYPES = frozenset({
    "array",
    "boolean",
    "integer",
    "null",
    "number",
    "object",
    "string",
})
_ANNOTATION_KEYWORDS = frozenset({
    "$comment",
    "$id",
    "$schema",
    "default",
    "deprecated",
    "description",
    "examples",
    "format",
    "readOnly",
    "title",
    "writeOnly",
})
_ASSERTION_KEYWORDS = frozenset({
    "additionalProperties",
    "allOf",
    "anyOf",
    "const",
    "enum",
    "exclusiveMaximum",
    "exclusiveMinimum",
    "items",
    "maxItems",
    "maxLength",
    "maxProperties",
    "maximum",
    "minItems",
    "minLength",
    "minProperties",
    "minimum",
    "multipleOf",
    "not",
    "oneOf",
    "pattern",
    "properties",
    "required",
    "type",
    "uniqueItems",
})
_SUPPORTED_KEYWORDS = _ANNOTATION_KEYWORDS | _ASSERTION_KEYWORDS
_MAX_SCHEMA_DEPTH = 64


def validate_tool_arguments(
    schema: JsonObject,
    arguments: JsonObject,
    *,
    context: str,
) -> None:
    """Validate one argument object against a discovered native input schema.

    Args:
        schema: Exact input schema returned by the live Toolset Registry.
        arguments: Strict JSON object supplied by the caller.
        context: Stable diagnostic identity that contains no argument values.

    """
    _validate_schema(schema, context=f"{context}.schema", depth=0)
    _validate_value(schema, arguments, context=context, depth=0)


def _validate_schema(schema: JsonObject, *, context: str, depth: int) -> None:
    _require_depth(depth, context=context)
    unsupported = set(schema).difference(_SUPPORTED_KEYWORDS)
    if unsupported:
        fail_protocol(f"{context}: unsupported schema keyword")
    _schema_types(schema, context=context)
    _validate_enum_schema(schema, context=context)
    _validate_numeric_schema(schema, context=context)
    _validate_size_schema(schema, context=context)
    _validate_pattern_schema(schema, context=context)
    _validate_object_schema(schema, context=context, depth=depth)
    _validate_array_schema(schema, context=context, depth=depth)
    _validate_composition_schema(schema, context=context, depth=depth)


def _validate_enum_schema(schema: JsonObject, *, context: str) -> None:
    enum = schema.get("enum")
    if enum is not None:
        if not isinstance(enum, list) or not enum:
            fail_protocol(f"{context}.enum must be a non-empty array")
        if _contains_duplicate_json_values(enum):
            fail_protocol(f"{context}.enum contains duplicate values")
    if "const" in schema and enum is not None:
        const = cast("JsonValue", schema["const"])
        if not any(_json_equal(const, item) for item in enum):
            fail_protocol(f"{context}: const is excluded by enum")


def _validate_numeric_schema(schema: JsonObject, *, context: str) -> None:
    for keyword in (
        "exclusiveMaximum",
        "exclusiveMinimum",
        "maximum",
        "minimum",
        "multipleOf",
    ):
        value = schema.get(keyword)
        if value is not None and not _is_json_number(value):
            fail_protocol(f"{context}.{keyword} must be a number")
    multiple = schema.get("multipleOf")
    if multiple is not None and _decimal(multiple, context=context) <= 0:
        fail_protocol(f"{context}.multipleOf must be positive")
    minimum = schema.get("minimum")
    maximum = schema.get("maximum")
    if (
        minimum is not None
        and maximum is not None
        and _decimal(minimum, context=context)
        > _decimal(maximum, context=context)
    ):
        fail_protocol(f"{context}: minimum exceeds maximum")


def _validate_size_schema(schema: JsonObject, *, context: str) -> None:
    for keyword in (
        "maxItems",
        "maxLength",
        "maxProperties",
        "minItems",
        "minLength",
        "minProperties",
    ):
        value = schema.get(keyword)
        if value is not None and not _is_nonnegative_integer(value):
            fail_protocol(f"{context}.{keyword} must be a nonnegative integer")
    for minimum, maximum in (
        ("minItems", "maxItems"),
        ("minLength", "maxLength"),
        ("minProperties", "maxProperties"),
    ):
        lower = schema.get(minimum)
        upper = schema.get(maximum)
        if lower is not None and upper is not None and lower > upper:
            fail_protocol(f"{context}: {minimum} exceeds {maximum}")
    unique = schema.get("uniqueItems")
    if unique is not None and not isinstance(unique, bool):
        fail_protocol(f"{context}.uniqueItems must be boolean")


def _validate_pattern_schema(schema: JsonObject, *, context: str) -> None:
    pattern = schema.get("pattern")
    if pattern is None:
        return
    if not isinstance(pattern, str):
        fail_protocol(f"{context}.pattern must be text")
    try:
        _ = re.compile(pattern)
    except re.error as error:
        fail_protocol(f"{context}.pattern is invalid", cause=error)


def _validate_object_schema(
    schema: JsonObject,
    *,
    context: str,
    depth: int,
) -> None:
    properties_value = schema.get("properties")
    properties = (
        {}
        if properties_value is None
        else require_json_object(
            properties_value, context=f"{context}.properties"
        )
    )
    for name, value in properties.items():
        child = require_json_object(
            value,
            context=f"{context}.properties.{_diagnostic_segment(name)}",
        )
        _validate_schema(
            child,
            context=f"{context}.properties.{_diagnostic_segment(name)}",
            depth=depth + 1,
        )
    required_value = schema.get("required")
    if required_value is not None:
        if not isinstance(required_value, list):
            fail_protocol(f"{context}.required must be an array")
        required: set[str] = set()
        for value in required_value:
            if not isinstance(value, str):
                fail_protocol(f"{context}.required must contain text")
            if value in required:
                fail_protocol(f"{context}.required contains a duplicate")
            required.add(value)
        if not required.issubset(properties):
            fail_protocol(f"{context}.required names an undeclared property")
    additional = schema.get("additionalProperties")
    if additional is not None and not isinstance(additional, bool | dict):
        fail_protocol(
            f"{context}.additionalProperties must be boolean or an object"
        )
    if isinstance(additional, dict):
        additional_schema = cast("JsonObject", additional)
        _validate_schema(
            additional_schema,
            context=f"{context}.additionalProperties",
            depth=depth + 1,
        )


def _validate_array_schema(
    schema: JsonObject,
    *,
    context: str,
    depth: int,
) -> None:
    items = schema.get("items")
    if items is None:
        return
    child = require_json_object(items, context=f"{context}.items")
    _validate_schema(child, context=f"{context}.items", depth=depth + 1)


def _validate_composition_schema(
    schema: JsonObject,
    *,
    context: str,
    depth: int,
) -> None:
    for keyword in ("allOf", "anyOf", "oneOf"):
        branches = schema.get(keyword)
        if branches is None:
            continue
        if not isinstance(branches, list) or not branches:
            fail_protocol(f"{context}.{keyword} must be a non-empty array")
        for index, value in enumerate(branches):
            child = require_json_object(
                value,
                context=f"{context}.{keyword}[{index}]",
            )
            _validate_schema(
                child,
                context=f"{context}.{keyword}[{index}]",
                depth=depth + 1,
            )
    negated = schema.get("not")
    if negated is not None:
        child = require_json_object(negated, context=f"{context}.not")
        _validate_schema(child, context=f"{context}.not", depth=depth + 1)


def _validate_value(
    schema: JsonObject,
    value: JsonValue,
    *,
    context: str,
    depth: int,
) -> None:
    _require_depth(depth, context=context)
    types = _schema_types(schema, context=f"{context}.schema")
    if types and not any(_matches_type(value, name) for name in types):
        fail_protocol(f"{context}: value does not match the declared type")
    enum = schema.get("enum")
    if isinstance(enum, list) and not any(
        _json_equal(value, item) for item in enum
    ):
        fail_protocol(f"{context}: value is not in the declared enum")
    if "const" in schema and not _json_equal(value, schema["const"]):
        fail_protocol(f"{context}: value does not match const")
    _validate_composed_value(schema, value, context=context, depth=depth)
    if isinstance(value, dict):
        _validate_object_value(schema, value, context=context, depth=depth)
    elif isinstance(value, list):
        _validate_array_value(schema, value, context=context, depth=depth)
    elif isinstance(value, str):
        _validate_string_value(schema, value, context=context)
    elif _is_json_number(value):
        _validate_number_value(schema, value, context=context)


def _validate_composed_value(
    schema: JsonObject,
    value: JsonValue,
    *,
    context: str,
    depth: int,
) -> None:
    for keyword in ("allOf", "anyOf", "oneOf"):
        branches = schema.get(keyword)
        if not isinstance(branches, list):
            continue
        matches = sum(
            _branch_matches(
                cast("JsonObject", branch),
                value,
                context=f"{context}.{keyword}[{index}]",
                depth=depth + 1,
            )
            for index, branch in enumerate(branches)
        )
        if keyword == "allOf" and matches != len(branches):
            fail_protocol(f"{context}: value does not satisfy allOf")
        if keyword == "anyOf" and matches == 0:
            fail_protocol(f"{context}: value does not satisfy anyOf")
        if keyword == "oneOf" and matches != 1:
            fail_protocol(
                f"{context}: value does not satisfy exactly one branch"
            )
    negated = schema.get("not")
    if isinstance(negated, dict) and _branch_matches(
        cast("JsonObject", negated),
        value,
        context=f"{context}.not",
        depth=depth + 1,
    ):
        fail_protocol(f"{context}: value matches the forbidden schema")


def _branch_matches(
    schema: JsonObject,
    value: JsonValue,
    *,
    context: str,
    depth: int,
) -> bool:
    try:
        _validate_value(schema, value, context=context, depth=depth)
    except ProtocolError:
        return False
    return True


def _validate_object_value(
    schema: JsonObject,
    value: JsonObject,
    *,
    context: str,
    depth: int,
) -> None:
    minimum = cast("int | None", schema.get("minProperties"))
    maximum = cast("int | None", schema.get("maxProperties"))
    if minimum is not None and len(value) < minimum:
        fail_protocol(f"{context}: object has too few properties")
    if maximum is not None and len(value) > maximum:
        fail_protocol(f"{context}: object has too many properties")
    properties_value = schema.get("properties")
    properties = (
        {} if properties_value is None else cast("JsonObject", properties_value)
    )
    required = schema.get("required", [])
    if isinstance(required, list):
        for name in required:
            if isinstance(name, str) and name not in value:
                fail_protocol(f"{context}: required property is missing")
    additional = schema.get("additionalProperties", True)
    for name, child_value in value.items():
        child_schema_value = properties.get(name)
        if child_schema_value is not None:
            child_schema = cast("JsonObject", child_schema_value)
        elif additional is False:
            fail_protocol(f"{context}: object contains an undeclared property")
        elif isinstance(additional, dict):
            child_schema = cast("JsonObject", additional)
        else:
            continue
        _validate_value(
            child_schema,
            child_value,
            context=f"{context}.{_diagnostic_segment(name)}",
            depth=depth + 1,
        )


def _validate_array_value(
    schema: JsonObject,
    value: list[JsonValue],
    *,
    context: str,
    depth: int,
) -> None:
    minimum = cast("int | None", schema.get("minItems"))
    maximum = cast("int | None", schema.get("maxItems"))
    if minimum is not None and len(value) < minimum:
        fail_protocol(f"{context}: array has too few items")
    if maximum is not None and len(value) > maximum:
        fail_protocol(f"{context}: array has too many items")
    if schema.get("uniqueItems") is True and _contains_duplicate_json_values(
        value
    ):
        fail_protocol(f"{context}: array items are not unique")
    items = schema.get("items")
    if isinstance(items, dict):
        item_schema = cast("JsonObject", items)
        for index, item in enumerate(value):
            _validate_value(
                item_schema,
                item,
                context=f"{context}[{index}]",
                depth=depth + 1,
            )


def _validate_string_value(
    schema: JsonObject,
    value: str,
    *,
    context: str,
) -> None:
    minimum = cast("int | None", schema.get("minLength"))
    maximum = cast("int | None", schema.get("maxLength"))
    if minimum is not None and len(value) < minimum:
        fail_protocol(f"{context}: text is shorter than minLength")
    if maximum is not None and len(value) > maximum:
        fail_protocol(f"{context}: text is longer than maxLength")
    pattern = schema.get("pattern")
    if isinstance(pattern, str) and re.search(pattern, value) is None:
        fail_protocol(f"{context}: text does not match pattern")


def _validate_number_value(
    schema: JsonObject,
    value: float,
    *,
    context: str,
) -> None:
    number = _decimal(value, context=context)
    minimum = schema.get("minimum")
    maximum = schema.get("maximum")
    exclusive_minimum = schema.get("exclusiveMinimum")
    exclusive_maximum = schema.get("exclusiveMaximum")
    if minimum is not None and number < _decimal(minimum, context=context):
        fail_protocol(f"{context}: number is below minimum")
    if maximum is not None and number > _decimal(maximum, context=context):
        fail_protocol(f"{context}: number is above maximum")
    if exclusive_minimum is not None and number <= _decimal(
        exclusive_minimum, context=context
    ):
        fail_protocol(f"{context}: number is not above exclusiveMinimum")
    if exclusive_maximum is not None and number >= _decimal(
        exclusive_maximum, context=context
    ):
        fail_protocol(f"{context}: number is not below exclusiveMaximum")
    multiple = schema.get("multipleOf")
    if multiple is not None:
        divisor = _decimal(multiple, context=context)
        if number % divisor != 0:
            fail_protocol(f"{context}: number is not a multipleOf value")


def _schema_types(schema: JsonObject, *, context: str) -> tuple[str, ...]:
    raw = schema.get("type")
    if raw is None:
        return ()
    if isinstance(raw, str):
        names = (raw,)
    elif isinstance(raw, list) and raw:
        if not all(isinstance(item, str) for item in raw):
            fail_protocol(f"{context}.type must contain text")
        names = tuple(cast("list[str]", raw))
    else:
        fail_protocol(f"{context}.type must be text or a non-empty array")
    if any(name not in _JSON_TYPES for name in names):
        fail_protocol(f"{context}.type contains an unsupported JSON type")
    if len(set(names)) != len(names):
        fail_protocol(f"{context}.type contains a duplicate")
    return names


def _matches_type(  # noqa: PLR0911 - JSON Schema has seven primitive branches.
    value: JsonValue,
    name: str,
) -> bool:
    if name == "null":
        return value is None
    if name == "boolean":
        return isinstance(value, bool)
    if name == "integer":
        if not _is_json_number(value):
            return False
        number = _decimal(value, context="integer type")
        return number == number.to_integral_value()
    if name == "number":
        return _is_json_number(value)
    if name == "string":
        return isinstance(value, str)
    if name == "array":
        return isinstance(value, list)
    if name == "object":
        return isinstance(value, dict)
    return False


def _is_json_number(value: object) -> bool:
    return isinstance(value, int | float) and not isinstance(value, bool)


def _is_nonnegative_integer(value: object) -> bool:
    return isinstance(value, int) and not isinstance(value, bool) and value >= 0


def _decimal(value: object, *, context: str) -> Decimal:
    if not _is_json_number(value):
        fail_protocol(f"{context}: expected a JSON number")
    try:
        return Decimal(str(value))
    except InvalidOperation as error:
        fail_protocol(f"{context}: invalid numeric constraint", cause=error)


def _contains_duplicate_json_values(values: Iterable[JsonValue]) -> bool:
    seen: list[JsonValue] = []
    for value in values:
        if any(_json_equal(value, prior) for prior in seen):
            return True
        seen.append(value)
    return False


def _json_equal(  # noqa: PLR0911 - equality mirrors JSON's value families.
    left: object,
    right: object,
) -> bool:
    if left is None or right is None:
        return left is None and right is None
    if isinstance(left, bool) or isinstance(right, bool):
        return (
            isinstance(left, bool) and isinstance(right, bool) and left == right
        )
    if _is_json_number(left) and _is_json_number(right):
        return _decimal(left, context="JSON equality") == _decimal(
            right, context="JSON equality"
        )
    if isinstance(left, str) or isinstance(right, str):
        return (
            isinstance(left, str) and isinstance(right, str) and left == right
        )
    if isinstance(left, list) or isinstance(right, list):
        return (
            isinstance(left, list)
            and isinstance(right, list)
            and len(left) == len(right)
            and all(map(_json_equal, left, right, strict=True))
        )
    if isinstance(left, dict) or isinstance(right, dict):
        if not isinstance(left, dict) or not isinstance(right, dict):
            return False
        if set(left) != set(right):
            return False
        return all(_json_equal(left[key], right[key]) for key in left)
    return False


def _diagnostic_segment(value: str) -> str:
    escaped = json.dumps(value, ensure_ascii=True)
    if len(escaped) > 80:
        return "<long-key>"
    return escaped


def _require_depth(depth: int, *, context: str) -> None:
    if depth > _MAX_SCHEMA_DEPTH:
        fail_protocol(f"{context}: schema nesting is too deep")

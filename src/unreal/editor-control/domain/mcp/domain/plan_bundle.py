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
#   - Independent validation of generated Unreal plan bundle evidence.
# - Must-Not:
#   - Read files, contact Unreal Editor, or apply native operations.
# - Allows:
#   - Parse canonical JSON, recompute revisions, and validate dependency graphs.
# - Split-When:
#   - Split when another plan schema gains an independent lifecycle.
# - Merge-When:
#   - Merge when plan application owns identical preflight policy.
# - Summary:
#   - Generated Unreal plan bundle preflight domain.
# - Description:
#   - Rejects stale, partial, noncanonical, internally inconsistent, or
#     semantically unbound release bundles.
# - Usage:
#   - Called by a read-only filesystem adapter before any native MCP mutation.
# - Defaults:
#   - Exactly six plans targeting Unreal Engine 5.8.1 are accepted.
#

"""Independent validation of generated Unreal plan bundle evidence."""

from __future__ import annotations

from collections import Counter
from collections import OrderedDict
import hashlib
import json
import re
from typing import NamedTuple
from typing import cast
import unicodedata

from mcp.domain.errors import fail_protocol
from mcp.domain.json_types import JsonObject
from mcp.domain.json_types import JsonValue
from mcp.domain.json_types import normalize_json
from mcp.domain.json_types import reject_duplicate_json_object
from mcp.domain.json_types import require_json_object

_PLAN_SCHEMA = "shar-schoenwald.unreal-plan.v1"
_BUNDLE_SCHEMA = "shar-schoenwald.unreal-plan-bundle.v4"
_TARGET_ENGINE_VERSION = "5.8.1"
_TARGET_PLATFORM = "editor"
_SHA256 = re.compile(r"^[0-9a-f]{64}$")
_OPERATION_ID = re.compile(r"^operation-[0-9a-f]{16}$")
_IDENTITY = re.compile(r"^[A-Za-z0-9_.-]{1,240}$")
_UNREAL_NAME = re.compile(r"^[A-Za-z0-9_]+$")

_SEMANTIC_ARTIFACT_SPECS: tuple[tuple[str, str], ...] = (
    ("mission-definitions", "mission-definitions.jsonl"),
    ("mission-tuning", "mission-tuning.jsonl"),
    ("vehicle-tuning", "vehicle-tuning.jsonl"),
    ("vehicle-tuning-usage", "vehicle-tuning-usage.jsonl"),
)
_PLAN_SPECS: tuple[tuple[str, str, tuple[str, ...]], ...] = (
    ("asset-import-plan", "asset-import-plan.json", ()),
    (
        "asset-construction-plan",
        "asset-construction-plan.json",
        ("asset-import-plan",),
    ),
    (
        "world-assembly-plan",
        "world-assembly-plan.json",
        ("asset-construction-plan", "asset-import-plan"),
    ),
    (
        "runtime-binding-plan",
        "runtime-binding-plan.json",
        (
            "asset-construction-plan",
            "asset-import-plan",
            "world-assembly-plan",
        ),
    ),
    (
        "validation-plan",
        "validation-plan.json",
        (
            "asset-construction-plan",
            "asset-import-plan",
            "runtime-binding-plan",
            "world-assembly-plan",
        ),
    ),
    ("package-plan", "package-plan.json", ("validation-plan",)),
)

_BASE_REQUIREMENTS = {
    "case-insensitive-destinations-unique",
    "generated-root-confined",
    "revision-matches-canonical-body",
    "schema-supported",
}
_FAMILY_REQUIREMENTS: dict[str, set[str]] = {
    "asset-import-plan": {
        "import-settings-match-profile",
        "saved-class-matches-plan",
        "source-bytes-match-revision",
    },
    "asset-construction-plan": {
        "native-factory-available",
        "normalized-json-schema-valid",
        "saved-class-matches-plan",
    },
    "world-assembly-plan": {
        "authored-transforms-preserved",
        "streaming-ownership-valid",
    },
    "runtime-binding-plan": {"all-stable-references-resolve"},
    "validation-plan": {
        "dependencies-resolve-after-restart",
        "semantic-digest-reproducible",
    },
    "package-plan": {
        "cook-succeeds",
        "packaged-build-loads-generated-assets",
    },
}

_INDEX_FIELDS = (
    "schema",
    "revision",
    "source_manifest_revision",
    "engine_contract_revision",
    "target_engine_version",
    "target_platform",
    "semantic_blocker_count",
    "semantic_blockers",
    "semantic_artifacts",
    "plans",
)
_PLAN_FIELDS = (
    "schema",
    "plan_id",
    "revision",
    "source_manifest_revision",
    "engine_contract_revision",
    "target_engine_version",
    "target_platform",
    "dependencies",
    "outputs",
    "operations",
    "validation",
)
_OPERATION_FIELDS = (
    "operation_id",
    "package_identity",
    "source_identity",
    "source_format",
    "target_family",
    "source_path",
    "source_revision",
    "destination",
    "target_class",
    "importer",
    "import_profile",
    "dependencies",
    "readiness",
    "world_owned",
    "runtime_bound",
)


class PlanSummary(NamedTuple):
    """Validated summary for one canonical plan family."""

    plan_id: str
    revision: str
    filename: str
    operation_count: int


class PlanOperation(NamedTuple):
    """One fully validated generated Unreal operation."""

    plan_id: str
    operation_id: str
    package_identity: str
    source_identity: str
    source_format: str
    target_family: str
    source_path: str
    source_revision: str
    destination: str
    target_class: str
    importer: str
    import_profile: str
    dependencies: tuple[str, ...]
    readiness: str
    world_owned: bool
    runtime_bound: bool


class SemanticBlockerSummary(NamedTuple):
    """One public-safe class of unresolved semantic conversion work."""

    category: str
    target_kind: str
    import_profile: str
    count: int

    def to_json(self) -> JsonObject:
        """Render one canonical blocker class without source identity."""
        return {
            "category": self.category,
            "count": self.count,
            "importProfile": self.import_profile,
            "targetKind": self.target_kind,
        }


class SemanticArtifactSummary(NamedTuple):
    """One exact semantic sidecar bound into the release index."""

    artifact_id: str
    filename: str
    revision: str
    byte_count: int

    def to_json(self) -> JsonObject:
        """Render canonical public semantic-artifact evidence."""
        return {
            "artifactId": self.artifact_id,
            "byteCount": self.byte_count,
            "filename": self.filename,
            "revision": self.revision,
        }


class PlanBundleReport(NamedTuple):
    """Independent preflight evidence for one complete plan bundle."""

    revision: str
    source_manifest_revision: str
    engine_contract_revision: str
    target_engine_version: str
    target_platform: str
    semantic_blocker_count: int
    operation_count: int
    readiness_counts: dict[str, int]
    plans: tuple[PlanSummary, ...]
    semantic_blockers: tuple[SemanticBlockerSummary, ...] = ()
    semantic_artifacts: tuple[SemanticArtifactSummary, ...] = ()

    def to_json(self) -> JsonObject:
        """Render a deterministic public report without physical paths."""
        return {
            "engineContractRevision": self.engine_contract_revision,
            "operationCount": self.operation_count,
            "plans": [
                {
                    "filename": item.filename,
                    "operationCount": item.operation_count,
                    "planId": item.plan_id,
                    "revision": item.revision,
                }
                for item in self.plans
            ],
            "readinessCounts": dict(sorted(self.readiness_counts.items())),
            "revision": self.revision,
            "semanticArtifactCount": len(self.semantic_artifacts),
            "semanticArtifacts": [
                artifact.to_json() for artifact in self.semantic_artifacts
            ],
            "semanticBlockerCount": self.semantic_blocker_count,
            "semanticBlockers": [
                blocker.to_json() for blocker in self.semantic_blockers
            ],
            "sourceManifestRevision": self.source_manifest_revision,
            "targetEngineVersion": self.target_engine_version,
            "targetPlatform": self.target_platform,
        }


class ValidatedPlanBundle(NamedTuple):
    """Canonical report and typed operations from one accepted bundle."""

    report: PlanBundleReport
    operations: tuple[PlanOperation, ...]


def validate_plan_bundle(
    index_text: str,
    plan_texts: dict[str, str],
) -> PlanBundleReport:
    """Validate one complete seven-file plan bundle from canonical text."""
    return parse_plan_bundle(index_text, plan_texts).report


def parse_plan_bundle(  # noqa: PLR0912,PLR0914 - closed schema parser
    index_text: str,
    plan_texts: dict[str, str],
) -> ValidatedPlanBundle:
    """Parse one complete canonical bundle into immutable typed evidence."""
    index = _parse_document(index_text, context="plan bundle index")
    _require_exact_fields(index, _INDEX_FIELDS, context="plan bundle index")
    if _text(index, "schema", context="plan bundle index") != _BUNDLE_SCHEMA:
        fail_protocol("plan bundle index schema is not supported")
    context = _bundle_context(index, label="plan bundle index")
    index_revision = _sha256_field(
        index, "revision", context="plan bundle index"
    )
    semantic_blocker_count = _nonnegative_integer(
        index,
        "semantic_blocker_count",
        context="plan bundle index",
    )
    semantic_blockers = _semantic_blocker_summaries(index)
    semantic_artifacts = _semantic_artifact_summaries(index)
    semantic_total = sum(blocker.count for blocker in semantic_blockers)
    if semantic_total != semantic_blocker_count:
        fail_protocol("plan bundle semantic blocker total is inconsistent")
    raw_entries = _array(index, "plans", context="plan bundle index")
    if len(raw_entries) != len(_PLAN_SPECS):
        fail_protocol("plan bundle index does not contain exactly six plans")

    summaries: list[PlanSummary] = []
    revision_by_id: dict[str, str] = {}
    expected_files: set[str] = set()
    for position, ((plan_id, filename, _), raw_entry) in enumerate(
        zip(_PLAN_SPECS, raw_entries, strict=True)
    ):
        entry = require_json_object(
            raw_entry,
            context=f"plan bundle index entry {position}",
        )
        _require_exact_fields(
            entry,
            ("plan_id", "revision", "filename", "operation_count"),
            context="plan bundle index entry",
        )
        if (
            _text(entry, "plan_id", context="plan bundle index entry")
            != plan_id
        ):
            fail_protocol("plan bundle index plan order is not canonical")
        if (
            _text(entry, "filename", context="plan bundle index entry")
            != filename
        ):
            fail_protocol("plan bundle index filename is not canonical")
        revision = _sha256_field(
            entry,
            "revision",
            context="plan bundle index entry",
        )
        operation_count = _nonnegative_integer(
            entry,
            "operation_count",
            context="plan bundle index entry",
        )
        summaries.append(
            PlanSummary(plan_id, revision, filename, operation_count)
        )
        revision_by_id[plan_id] = revision
        expected_files.add(filename)

    if set(plan_texts) != expected_files:
        fail_protocol("plan bundle file inventory is not exact")

    all_operations: list[tuple[str, JsonObject]] = []
    typed_operations: list[PlanOperation] = []
    readiness = Counter[str]()
    for summary, (_, _, dependency_ids) in zip(
        summaries,
        _PLAN_SPECS,
        strict=True,
    ):
        operations = _validate_plan(
            plan_texts[summary.filename],
            summary=summary,
            expected_dependencies=dependency_ids,
            revision_by_id=revision_by_id,
            expected_context=context,
        )
        for operation in operations:
            all_operations.append((summary.plan_id, operation))
            typed = _typed_operation(summary.plan_id, operation)
            typed_operations.append(typed)
            readiness[typed.readiness] += 1

    _validate_operation_set(all_operations)
    canonical_index = _canonical_index(index, revision=index_revision)
    if index_text != f"{canonical_index}\n":
        fail_protocol("plan bundle index JSON is not canonical")
    preimage = _canonical_index(index, revision="")
    if _digest(preimage) != index_revision:
        fail_protocol("plan bundle index revision does not match its body")

    report = PlanBundleReport(
        revision=index_revision,
        source_manifest_revision=context[0],
        engine_contract_revision=context[1],
        target_engine_version=context[2],
        target_platform=context[3],
        semantic_blocker_count=semantic_blocker_count,
        operation_count=sum(item.operation_count for item in summaries),
        readiness_counts=dict(readiness),
        plans=tuple(summaries),
        semantic_artifacts=semantic_artifacts,
        semantic_blockers=semantic_blockers,
    )
    return ValidatedPlanBundle(report, tuple(typed_operations))


def _semantic_artifact_summaries(
    index: JsonObject,
) -> tuple[SemanticArtifactSummary, ...]:
    values = _array(index, "semantic_artifacts", context="plan bundle index")
    if len(values) != len(_SEMANTIC_ARTIFACT_SPECS):
        fail_protocol("plan bundle semantic artifact inventory is not exact")
    artifacts: list[SemanticArtifactSummary] = []
    for position, ((artifact_id, filename), raw) in enumerate(
        zip(_SEMANTIC_ARTIFACT_SPECS, values, strict=True)
    ):
        value = require_json_object(
            raw, context=f"semantic artifact {position}"
        )
        _require_exact_fields(
            value,
            ("artifact_id", "filename", "revision", "byte_count"),
            context="semantic artifact",
        )
        observed_id = _text(
            value, "artifact_id", context="semantic artifact"
        )
        if observed_id != artifact_id:
            fail_protocol(
                "plan bundle semantic artifact order is not canonical"
            )
        if _text(value, "filename", context="semantic artifact") != filename:
            fail_protocol(
                "plan bundle semantic artifact filename is not canonical"
            )
        artifacts.append(SemanticArtifactSummary(
            artifact_id=artifact_id,
            filename=filename,
            revision=_sha256_field(
                value, "revision", context="semantic artifact"
            ),
            byte_count=_nonnegative_integer(
                value, "byte_count", context="semantic artifact"
            ),
        ))
    return tuple(artifacts)


def _semantic_blocker_summaries(
    index: JsonObject,
) -> tuple[SemanticBlockerSummary, ...]:
    blockers: list[SemanticBlockerSummary] = []
    previous: tuple[str, str, str] | None = None
    for position, raw in enumerate(
        _array(index, "semantic_blockers", context="plan bundle index")
    ):
        value = require_json_object(
            raw, context=f"semantic blocker class {position}"
        )
        _require_exact_fields(
            value,
            ("category", "target_kind", "import_profile", "count"),
            context="semantic blocker class",
        )
        blocker = SemanticBlockerSummary(
            category=_identity_field(
                value, "category", context="semantic blocker class"
            ),
            target_kind=_identity_field(
                value, "target_kind", context="semantic blocker class"
            ),
            import_profile=_identity_field(
                value, "import_profile", context="semantic blocker class"
            ),
            count=_nonnegative_integer(
                value, "count", context="semantic blocker class"
            ),
        )
        if blocker.count == 0:
            fail_protocol("semantic blocker class count must be positive")
        key = (blocker.category, blocker.target_kind, blocker.import_profile)
        if previous is not None and key <= previous:
            fail_protocol("semantic blocker classes are not unique and sorted")
        previous = key
        blockers.append(blocker)
    return tuple(blockers)


def _typed_operation(plan_id: str, operation: JsonObject) -> PlanOperation:
    return PlanOperation(
        plan_id=plan_id,
        operation_id=_text(operation, "operation_id", context="plan operation"),
        package_identity=_text(
            operation,
            "package_identity",
            context="plan operation",
        ),
        source_identity=_text(
            operation,
            "source_identity",
            context="plan operation",
        ),
        source_format=_text(
            operation, "source_format", context="plan operation"
        ),
        target_family=_text(
            operation, "target_family", context="plan operation"
        ),
        source_path=_text(operation, "source_path", context="plan operation"),
        source_revision=_text(
            operation,
            "source_revision",
            context="plan operation",
        ),
        destination=_text(operation, "destination", context="plan operation"),
        target_class=_text(operation, "target_class", context="plan operation"),
        importer=_text(operation, "importer", context="plan operation"),
        import_profile=_text(
            operation,
            "import_profile",
            context="plan operation",
        ),
        dependencies=tuple(
            _string_array(operation, "dependencies", context="plan operation")
        ),
        readiness=_text(operation, "readiness", context="plan operation"),
        world_owned=_boolean(
            operation, "world_owned", context="plan operation"
        ),
        runtime_bound=_boolean(
            operation,
            "runtime_bound",
            context="plan operation",
        ),
    )


def _validate_plan(  # noqa: PLR0912 - complete envelope validation
    text: str,
    *,
    summary: PlanSummary,
    expected_dependencies: tuple[str, ...],
    revision_by_id: dict[str, str],
    expected_context: tuple[str, str, str, str],
) -> tuple[JsonObject, ...]:
    plan = _parse_document(text, context=f"plan {summary.plan_id}")
    _require_exact_fields(plan, _PLAN_FIELDS, context="plan envelope")
    if _text(plan, "schema", context="plan envelope") != _PLAN_SCHEMA:
        fail_protocol("plan schema is not supported")
    if _text(plan, "plan_id", context="plan envelope") != summary.plan_id:
        fail_protocol("plan identity does not match its index entry")
    revision = _sha256_field(plan, "revision", context="plan envelope")
    if revision != summary.revision:
        fail_protocol("plan revision does not match its index entry")
    if _bundle_context(plan, label="plan envelope") != expected_context:
        fail_protocol("plan context does not match the bundle index")

    dependencies = _array(plan, "dependencies", context="plan envelope")
    if len(dependencies) != len(expected_dependencies):
        fail_protocol("plan dependency count is not canonical")
    for expected_id, raw_dependency in zip(
        expected_dependencies,
        dependencies,
        strict=True,
    ):
        dependency = require_json_object(
            raw_dependency,
            context="plan dependency",
        )
        _require_exact_fields(
            dependency,
            ("plan_id", "revision"),
            context="plan dependency",
        )
        if (
            _text(dependency, "plan_id", context="plan dependency")
            != expected_id
        ):
            fail_protocol("plan dependencies are not canonical")
        if (
            _sha256_field(
                dependency,
                "revision",
                context="plan dependency",
            )
            != revision_by_id[expected_id]
        ):
            fail_protocol("plan dependency revision is stale")

    raw_operations = _array(plan, "operations", context="plan envelope")
    operations = tuple(
        require_json_object(value, context="plan operation")
        for value in raw_operations
    )
    if len(operations) != summary.operation_count:
        fail_protocol("plan operation count disagrees with its index entry")
    outputs = _array(plan, "outputs", context="plan envelope")
    expected_outputs: list[str] = []
    for operation in operations:
        _validate_operation(
            operation,
            plan_id=summary.plan_id,
            source_manifest_revision=expected_context[0],
        )
        expected_outputs.append(
            _text(operation, "destination", context="plan operation")
        )
    if outputs != expected_outputs:
        fail_protocol("plan outputs do not match operation destinations")
    _validate_plan_requirements(plan, summary)

    canonical_plan = _canonical_plan(plan, revision=revision)
    if text != f"{canonical_plan}\n":
        fail_protocol("plan JSON is not canonical")
    if _digest(_canonical_plan(plan, revision="")) != revision:
        fail_protocol("plan revision does not match its canonical body")
    return operations


def _validate_operation(  # noqa: PLR0914 - atomic operation contract
    operation: JsonObject,
    *,
    plan_id: str,
    source_manifest_revision: str,
) -> None:
    _require_exact_fields(
        operation, _OPERATION_FIELDS, context="plan operation"
    )
    operation_id = _text(operation, "operation_id", context="plan operation")
    if _OPERATION_ID.fullmatch(operation_id) is None:
        fail_protocol("plan operation identity is not canonical")
    package_identity = _identity_field(
        operation,
        "package_identity",
        context="plan operation",
    )
    source_identity = _identity_field(
        operation,
        "source_identity",
        context="plan operation",
    )
    source_format = _text(operation, "source_format", context="plan operation")
    target_family = _text(operation, "target_family", context="plan operation")
    source_path = _text(operation, "source_path", context="plan operation")
    _validate_portable_path(source_path)
    source_revision = _sha256_field(
        operation,
        "source_revision",
        context="plan operation",
    )
    destination = _text(operation, "destination", context="plan operation")
    _validate_destination(destination)
    target_class = _identity_field(
        operation,
        "target_class",
        context="plan operation",
    )
    importer = _identity_field(operation, "importer", context="plan operation")
    import_profile = _identity_field(
        operation,
        "import_profile",
        context="plan operation",
    )
    dependencies = _string_array(
        operation,
        "dependencies",
        context="plan operation",
    )
    if dependencies != sorted(set(dependencies)):
        fail_protocol("plan operation dependencies are not unique and sorted")
    if operation_id in dependencies:
        fail_protocol("plan operation depends on itself")
    readiness = _text(operation, "readiness", context="plan operation")
    _boolean(operation, "world_owned", context="plan operation")
    _boolean(operation, "runtime_bound", context="plan operation")

    expected = {
        "json": (
            "structured-data",
            {"requires-editor-factory"},
            "asset-construction-plan",
        ),
        "image": ("texture", {"ready"}, "asset-import-plan"),
        "wav": ("audio", {"ready"}, "asset-import-plan"),
        "hap": ("media", {"ready"}, "asset-import-plan"),
        "fbx": (
            "model",
            {"ready", "requires-conversion"},
            "asset-import-plan",
        ),
    }.get(source_format)
    if expected is None:
        fail_protocol("plan operation source format is unsupported")
    expected_family, expected_readiness, expected_plan = expected
    if target_family != expected_family or readiness not in expected_readiness:
        fail_protocol("plan operation source contract is inconsistent")
    if plan_id != expected_plan:
        fail_protocol("plan operation is assigned to the wrong plan family")
    if source_format == "json" and (
        source_path != "manifest.jsonl"
        or source_revision != source_manifest_revision
    ):
        fail_protocol("construction operation source evidence is not canonical")

    separator = chr(10)
    preimage = separator.join((
        package_identity,
        source_identity,
        source_format,
        target_family,
        source_path,
        source_revision,
        destination,
        target_class,
        importer,
        import_profile,
    ))
    if operation_id != f"operation-{_digest(preimage)[:16]}":
        fail_protocol("plan operation identity does not match its evidence")


def _validate_plan_requirements(plan: JsonObject, summary: PlanSummary) -> None:
    validation = require_json_object(
        plan.get("validation"),
        context="plan validation",
    )
    _require_exact_fields(
        validation,
        ("operation_count", "requirements"),
        context="plan validation",
    )
    if (
        _nonnegative_integer(
            validation,
            "operation_count",
            context="plan validation",
        )
        != summary.operation_count
    ):
        fail_protocol("plan validation operation count is stale")
    requirements = _string_array(
        validation,
        "requirements",
        context="plan validation",
    )
    expected = sorted(
        _BASE_REQUIREMENTS | _FAMILY_REQUIREMENTS[summary.plan_id]
    )
    if requirements != expected:
        fail_protocol("plan validation requirements are not canonical")


def _validate_operation_set(
    operations: list[tuple[str, JsonObject]],
) -> None:
    ids: dict[str, str] = {}
    destinations: set[str] = set()
    by_plan: dict[str, list[str]] = {
        plan_id: [] for plan_id, _file, _deps in _PLAN_SPECS
    }
    dependency_map: dict[str, tuple[str, ...]] = {}
    family_order = {
        plan_id: index
        for index, (plan_id, _file, _deps) in enumerate(_PLAN_SPECS)
    }
    for plan_id, operation in operations:
        operation_id = _text(
            operation, "operation_id", context="plan operation"
        )
        if operation_id in ids:
            fail_protocol("plan bundle contains a duplicate operation identity")
        ids[operation_id] = plan_id
        by_plan[plan_id].append(operation_id)
        destination = _text(operation, "destination", context="plan operation")
        folded = destination.casefold()
        if folded in destinations:
            fail_protocol("plan bundle contains a destination collision")
        destinations.add(folded)
        dependency_map[operation_id] = tuple(
            _string_array(operation, "dependencies", context="plan operation")
        )
    for operation_ids in by_plan.values():
        if operation_ids != sorted(operation_ids):
            fail_protocol("plan operations are not sorted by identity")
    for operation_id, dependencies in dependency_map.items():
        owner = ids[operation_id]
        for dependency in dependencies:
            dependency_owner = ids.get(dependency)
            if dependency_owner is None:
                fail_protocol("plan operation depends on an unknown operation")
            if family_order[dependency_owner] > family_order[owner]:
                fail_protocol("plan operation depends on a later plan family")
    _validate_acyclic_dependencies(dependency_map)


def _validate_acyclic_dependencies(
    dependencies: dict[str, tuple[str, ...]],
) -> None:
    remaining = dict(dependencies)
    completed: set[str] = set()
    while remaining:
        ready = sorted(
            operation_id
            for operation_id, requirements in remaining.items()
            if all(value in completed for value in requirements)
        )
        if not ready:
            fail_protocol("plan operation dependency graph contains a cycle")
        for operation_id in ready:
            del remaining[operation_id]
            completed.add(operation_id)


def _canonical_index(index: JsonObject, *, revision: str) -> str:
    blockers = [
        OrderedDict((
            ("category", blocker.category),
            ("target_kind", blocker.target_kind),
            ("import_profile", blocker.import_profile),
            ("count", blocker.count),
        ))
        for blocker in _semantic_blocker_summaries(index)
    ]
    semantic_artifacts = [
        OrderedDict((
            ("artifact_id", artifact.artifact_id),
            ("filename", artifact.filename),
            ("revision", artifact.revision),
            ("byte_count", artifact.byte_count),
        ))
        for artifact in _semantic_artifact_summaries(index)
    ]
    plans = [
        OrderedDict((
            ("plan_id", _text(entry, "plan_id", context="plan index entry")),
            ("revision", _text(entry, "revision", context="plan index entry")),
            ("filename", _text(entry, "filename", context="plan index entry")),
            (
                "operation_count",
                _nonnegative_integer(
                    entry,
                    "operation_count",
                    context="plan index entry",
                ),
            ),
        ))
        for entry in (
            require_json_object(value, context="plan index entry")
            for value in _array(index, "plans", context="plan bundle index")
        )
    ]
    payload = OrderedDict((
        ("schema", _text(index, "schema", context="plan bundle index")),
        ("revision", revision),
        (
            "source_manifest_revision",
            _text(
                index, "source_manifest_revision", context="plan bundle index"
            ),
        ),
        (
            "engine_contract_revision",
            _text(
                index, "engine_contract_revision", context="plan bundle index"
            ),
        ),
        (
            "target_engine_version",
            _text(index, "target_engine_version", context="plan bundle index"),
        ),
        (
            "target_platform",
            _text(index, "target_platform", context="plan bundle index"),
        ),
        (
            "semantic_blocker_count",
            _nonnegative_integer(
                index,
                "semantic_blocker_count",
                context="plan bundle index",
            ),
        ),
        ("semantic_blockers", blockers),
        ("semantic_artifacts", semantic_artifacts),
        ("plans", plans),
    ))
    return _canonical(payload)


def _canonical_plan(plan: JsonObject, *, revision: str) -> str:
    dependencies = [
        OrderedDict((
            ("plan_id", _text(item, "plan_id", context="plan dependency")),
            ("revision", _text(item, "revision", context="plan dependency")),
        ))
        for item in (
            require_json_object(value, context="plan dependency")
            for value in _array(plan, "dependencies", context="plan envelope")
        )
    ]
    operations = [
        OrderedDict((field, operation[field]) for field in _OPERATION_FIELDS)
        for operation in (
            require_json_object(value, context="plan operation")
            for value in _array(plan, "operations", context="plan envelope")
        )
    ]
    validation = require_json_object(
        plan.get("validation"),
        context="plan validation",
    )
    validation_payload = OrderedDict((
        (
            "operation_count",
            _nonnegative_integer(
                validation,
                "operation_count",
                context="plan validation",
            ),
        ),
        (
            "requirements",
            _string_array(
                validation,
                "requirements",
                context="plan validation",
            ),
        ),
    ))
    payload = OrderedDict((
        ("schema", _text(plan, "schema", context="plan envelope")),
        ("plan_id", _text(plan, "plan_id", context="plan envelope")),
        ("revision", revision),
        (
            "source_manifest_revision",
            _text(plan, "source_manifest_revision", context="plan envelope"),
        ),
        (
            "engine_contract_revision",
            _text(plan, "engine_contract_revision", context="plan envelope"),
        ),
        (
            "target_engine_version",
            _text(plan, "target_engine_version", context="plan envelope"),
        ),
        (
            "target_platform",
            _text(plan, "target_platform", context="plan envelope"),
        ),
        ("dependencies", dependencies),
        ("outputs", _string_array(plan, "outputs", context="plan envelope")),
        ("operations", operations),
        ("validation", validation_payload),
    ))
    return _canonical(payload)


def _parse_document(text: str, *, context: str) -> JsonObject:
    if not text.endswith("\n") or "\r" in text or text.count("\n") != 1:
        fail_protocol(f"{context}: JSON line endings are not canonical")
    try:
        parsed = json.loads(
            text,
            object_pairs_hook=reject_duplicate_json_object,
            parse_constant=lambda _: fail_protocol(
                f"{context}: non-finite JSON number is not supported"
            ),
        )
    except (json.JSONDecodeError, UnicodeError) as error:
        fail_protocol(f"{context}: invalid JSON", cause=error)
    normalized = normalize_json(parsed, context=context)
    return require_json_object(normalized, context=context)


def _bundle_context(
    value: JsonObject,
    *,
    label: str,
) -> tuple[str, str, str, str]:
    source_revision = _sha256_field(
        value,
        "source_manifest_revision",
        context=label,
    )
    contract_revision = _identity_field(
        value,
        "engine_contract_revision",
        context=label,
    )
    engine = _identity_field(value, "target_engine_version", context=label)
    platform = _identity_field(value, "target_platform", context=label)
    if engine != _TARGET_ENGINE_VERSION or platform != _TARGET_PLATFORM:
        fail_protocol("plan bundle targets an unsupported Unreal environment")
    return source_revision, contract_revision, engine, platform


def _require_exact_fields(
    value: JsonObject,
    expected: tuple[str, ...],
    *,
    context: str,
) -> None:
    if tuple(value) != expected:
        fail_protocol(f"{context}: fields are not canonical")


def _text(value: JsonObject, field: str, *, context: str) -> str:
    item = value.get(field)
    if not isinstance(item, str):
        fail_protocol(f"{context}: expected text field {field}")
    return item


def _identity_field(value: JsonObject, field: str, *, context: str) -> str:
    item = _text(value, field, context=context)
    if _IDENTITY.fullmatch(item) is None:
        fail_protocol(f"{context}: identity field {field} is not canonical")
    return item


def _sha256_field(value: JsonObject, field: str, *, context: str) -> str:
    item = _text(value, field, context=context)
    if _SHA256.fullmatch(item) is None:
        fail_protocol(f"{context}: revision field {field} is not canonical")
    return item


def _array(value: JsonObject, field: str, *, context: str) -> list[JsonValue]:
    item = value.get(field)
    if not isinstance(item, list):
        fail_protocol(f"{context}: expected array field {field}")
    return item


def _string_array(value: JsonObject, field: str, *, context: str) -> list[str]:
    items = _array(value, field, context=context)
    if not all(isinstance(item, str) for item in items):
        fail_protocol(f"{context}: array field {field} must contain text")
    return cast("list[str]", items)


def _nonnegative_integer(
    value: JsonObject,
    field: str,
    *,
    context: str,
) -> int:
    item = value.get(field)
    if not isinstance(item, int) or isinstance(item, bool) or item < 0:
        fail_protocol(f"{context}: expected nonnegative integer field {field}")
    return item


def _boolean(value: JsonObject, field: str, *, context: str) -> bool:
    item = value.get(field)
    if not isinstance(item, bool):
        fail_protocol(f"{context}: expected boolean field {field}")
    return item


def _validate_portable_path(value: str) -> None:
    if not value or value.startswith("/") or "\\" in value or ":" in value:
        fail_protocol("plan operation source path is unsafe")
    if any(unicodedata.category(character) == "Cc" for character in value):
        fail_protocol("plan operation source path is unsafe")
    if any(part in {"", ".", ".."} for part in value.split("/")):
        fail_protocol("plan operation source path is unsafe")


def _validate_destination(value: str) -> None:
    prefix = "/Game/Generated/SHAR/"
    if (
        not value.startswith(prefix)
        or len(value) > 240
        or "//" in value
        or any(unicodedata.category(character) == "Cc" for character in value)
    ):
        fail_protocol("plan operation destination is unsafe")
    package, separator, object_name = value.rpartition(".")
    if not separator or "." in package:
        fail_protocol(
            "plan operation destination is not a complete object path"
        )
    _, slash, asset_name = package.rpartition("/")
    if not slash or object_name != asset_name:
        fail_protocol("plan operation object name does not match its package")
    segments = tuple(part for part in package.split("/") if part)
    if not segments or any(
        _UNREAL_NAME.fullmatch(part) is None for part in segments
    ):
        fail_protocol("plan operation destination contains an invalid segment")


def _canonical(value: object) -> str:
    try:
        return json.dumps(
            value,
            ensure_ascii=False,
            allow_nan=False,
            separators=(",", ":"),
        )
    except (TypeError, ValueError, UnicodeError) as error:
        fail_protocol("plan bundle contains noncanonical JSON", cause=error)


def _digest(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()

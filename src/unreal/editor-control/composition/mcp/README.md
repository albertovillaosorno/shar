# Unreal MCP terminal translator

`mcp` is a Python command-line translator for the native Unreal Engine 5.8
MCP server. It is **not an MCP server**, it does not replace Unreal's native
plugins, and it does not contain or copy Epic Games plugin source.

The package turns terminal commands into MCP lifecycle, discovery, and tool-call
messages so terminal-capable AI agents can operate Unreal Editor through a
reviewable and testable command surface.

## Architecture

The translator follows the repository's canonical source taxonomy:

```text
src/unreal/editor-control/
├── domain/mcp/domain/                    Protocol values and invariants
├── application/mcp/application/          Discovery and invocation use cases
├── port-outbound/mcp/port_outbound/      Transport and storage contracts
├── adapter-inbound/mcp/adapter_inbound/  Terminal command adapter
├── adapter-outbound/mcp/adapter_outbound/ HTTP and filesystem adapters
├── contract/mcp/py.typed                  Typed-package marker
└── composition/mcp/                       Distribution metadata and guide
```

The package portions form the `mcp` namespace when the wheel is built. The
repository tree therefore expresses dependency direction directly instead of
hiding domain, application, ports, and adapters below another source root.
The public operator interface remains the `shar-unreal-mcp` command.

Dependencies point inward. Domain code does not import HTTP, command-line,
filesystem, Unreal, or process APIs. The application layer invokes ports. The
outer adapters own transport and operator-facing behavior.

## Unreal plugin boundary

The Unreal project enables:

- `ModelContextProtocol` for the native inbound MCP server; and
- `AllToolsets` for the complete native editor tool catalog.

`MCPClientToolset` is explicitly disabled because it is an outbound adapter for
connecting Unreal to other MCP servers. This package needs the opposite
direction: terminal to Unreal. It remains an available future fallback if a
required workflow cannot be expressed through the official inbound server and
an external repository-owned MCP service becomes the safest interoperable
boundary. Enabling that fallback requires a reviewed ADR, tests, and an explicit
project configuration change; it is not part of the default architecture.

Installed project plugins remain local under
`src/unreal/project/composition/uproject/Plugins/` and
are ignored by Git.

The tracked Unreal project defaults enable automatic server startup, preserve
native tool-search mode, and use `http://127.0.0.1:8000/mcp`. Unreal Editor must
be restarted after plugin or MCP configuration changes because an existing
process cannot load newly enabled plugins retroactively.

## Commands

After installing the package in the repository Python environment:

```text
shar-unreal-mcp doctor
shar-unreal-mcp plan-preflight
shar-unreal-mcp plan-execution-preflight
shar-unreal-mcp plan-capabilities
shar-unreal-mcp plan-apply
shar-unreal-mcp vehicle-material-preflight
shar-unreal-mcp vehicle-material-capabilities
shar-unreal-mcp vehicle-material-apply
shar-unreal-mcp vehicle-physics-preflight
shar-unreal-mcp vehicle-physics-capabilities
shar-unreal-mcp vehicle-physics-apply
shar-unreal-mcp toolsets
shar-unreal-mcp describe EditorToolset.EditorAppToolset
shar-unreal-mcp call \
  EditorToolset.EditorAppToolset \
  EditorToolset.EditorAppToolset.GetCameraTransform --arguments '{}'
shar-unreal-mcp catalog --format markdown
shar-unreal-mcp skills
```

The default endpoint is `http://127.0.0.1:8000/mcp`. Only loopback HTTP
endpoints are accepted. Network commands create one MCP session, perform bounded
work, and close the session.

Generated-plan application is divided into four fail-closed gates:

- `plan-preflight` is local and verifies the exact seven-file bundle, canonical
  revisions, operation identities, dependencies, destinations, and readiness;
- `plan-execution-preflight` remains local, additionally verifies every
  applicable physical source and SHA-256, then compiles only reviewed native
  import routes; and
- `plan-capabilities` repeats both local gates, opens one MCP session, and uses
  only `list_toolsets` plus `describe_toolset` to validate the exact live input
  and output schemas needed for import, save, existence, class, dirty-state
  read-back, and compensating deletion. It never invokes the native `call_tool`
  mutation meta-tool; and
- `plan-apply` refuses incomplete local evidence before constructing transport,
  repeats the live capability audit, verifies every destination is absent, then
  performs serialized import, class read-back, explicit save, and clean-state
  verification. Media operations also verify their deterministic external movie
  payload. Any failure deletes only effects created by that transaction in
  reverse order and verifies their absence.

Verified vehicle FBXs are emitted as ready `shar-fbx-skeletal-v1`
prerequisite operations with dedicated `_Skeletal` destinations. Their owning
car packages remain semantic `CompositeModel` blockers, so this prerequisite
never makes the global plan complete by itself. Physical plan-source
verification resolves `vehicle-assets/...` only below
`.cache/pipeline/vehicle-assets/`, rejects redirected ancestry, and verifies the
bound SHA-256 before general execution.

Vehicle material assets have a separate three-gate transaction. The local
`vehicle-material-preflight` requires the release index to bind exactly one
`vehicle-materials.json`, rechecks the sidecar bytes and SHA-256, validates the
v3/v8 construction contract, and verifies all referenced PNGs as regular,
unlinked files with exact byte counts and content digests. It compiles only the
reviewed simple-unlit subset into content-addressed Texture2D requests,
deduplicated native masters, and per-slot Material Instance requests.

The three vehicle-material commands also accept `--package-id PACKAGE_ID`.
Selection is applied only after the complete release-bound document compiles
successfully; it retains instances for that exact canonical package id plus only
the texture and master dependencies those instances reference. Output keeps the
complete `constructionRevision` and adds a distinct `selectionRevision`, so
scoped execution cannot masquerade as complete-corpus construction evidence.

`vehicle-material-capabilities` opens one MCP session and validates the exact
schemas for `ImportBaseColorTexture2D`, the vehicle master/instance factories,
and AssetTools existence, class, dirty-state, save, and deletion operations.
`vehicle-material-apply` repeats every local and live gate, requires every
output to be absent before the first mutation, creates textures before masters
and masters before instances, reads back each class and dirty state, explicitly
saves every package, and requires clean state after save. Any failure triggers
reverse-order compensation limited to assets created by that transaction.
This transaction still does not assign Material Instances to Skeletal Mesh
slots, promote lit/spheremap/environment families, or make vehicle runtime
presentation ready.

Vehicle physics prerequisites have their own three-gate transaction. The local
`vehicle-physics-prerequisites-preflight` selects only skeletal imports required
by native-ready rigs and verifies just those physical FBXs. Capability audit
validates `ImportSkeletalMesh` plus AssetTools read-back, save, and compensation
schemas. Apply remains create-only and reuses the reviewed import transaction
without weakening global `plan-apply` completeness.

The current release selects 87 vehicle FBXs for 135 ready rigs; the
cylinder-only `icecream` dependency is not imported by this transaction.

Vehicle Physics Asset publication adds a separate three-gate construction
transaction after the Skeletal Mesh import is available. `vehicle-physics-
preflight` remains local: it requires the release index to bind exactly one
`vehicle-physics.json`, rechecks exact bytes and SHA-256, validates the v1/v8
construction contract, and joins every ready `source_fbx` to exactly one
reviewed `skeletal-mesh-fbx-v1` import step. Deterministic outputs are confined
to `/Game/Generated/SHAR/VehiclePhysics/PHYS_<digest>`.

`vehicle-physics-capabilities` opens one MCP session and validates the exact
live schemas for `CreateVehiclePhysicsAsset` plus AssetTools existence, class,
dirty-state, save, and deletion operations without invoking mutation.
`vehicle-physics-apply` repeats those gates, requires each Skeletal Mesh
package to exist as `SkeletalMesh`, requires every Physics Asset destination to
be absent, creates and independently reads back each `PhysicsAsset`, explicitly
saves it, and requires the package to read back clean. Failure compensation
runs in reverse order and can delete only Physics Assets created by that
transaction; imported Skeletal Meshes and Skeletons remain separate dependencies
and are never compensation-owned. This construction boundary does not activate
Chaos simulation, spawn a vehicle, or bind a runtime presentation by itself.

A real Unreal Engine 5.8.1 `sedana` run exercises both transactions against the
release-bound source. It verifies the 327,292-byte FBX and audits both six-tool
live
surfaces, imports and saves the Skeletal Mesh plus Skeleton, creates and saves
the six-shape Physics Asset, and reads all three back clean. A fresh editor
process then loads the same files as `SkeletalMesh`, `Skeleton`, and
`PhysicsAsset` with `dirty=false`, proving that explicit persistence crosses an
editor restart before the session-owned evidence assets are removed.

All four plan commands read `.cache/pipeline/unreal-staging/plans/` by default.
The execution, capability, and
application commands return failure while any emitted operation remains
conversion-blocked, factory-blocked, or lacks a reviewed native route. They
never report or execute a partial subset as a complete plan. Import-manifest v2
may additionally classify normalized packages as
`requires-semantic-conversion`; those packages intentionally emit no operation
until a deterministic domain compiler produces a concrete Unreal target, so
they remain upstream completion blockers rather than MCP mutation work.

Bundle
index v2 carries their aggregate `semantic_blocker_count`; execution preflight
requires that count to be zero before it can report a complete plan, even when
no operations were emitted.

The current reviewed compiler maps decoded images to
`TextureTools.import_file`, PCM WAV files to the project-owned
`SharImportToolset.ImportSoundWave`, verified HAP MOV files to
`SharImportToolset.ImportFileMediaSource`, ready static-mesh FBX operations to
`SharImportToolset.ImportStaticMesh`, and ready skeletal-mesh FBX operations to
`SharImportToolset.ImportSkeletalMesh`. The editor-only SHAR toolset loads after
engine initialization and registers through ToolsetRegistry. WAV, static FBX,
and skeletal FBX use synchronous automated asset-import tasks without
replacement or implicit save. Static FBX pins the import to one combined
`StaticMesh`, preserves authored normals, and disables material, texture,
animation, LOD, collision, Nanite, and lightmap-UV generation.

Skeletal FBX creates one `SkeletalMesh` plus its new `Skeleton` companion,
preserves authored normals, and disables Physics Asset and animation creation
for that transaction.

HAP copies verified bytes transactionally beneath
`Content/Movies/Generated/SHAR/`, creates a `UFileMediaSource`, and stores the
matching `./Movies/Generated/SHAR/...` path. All four project-owned routes leave
package save and independent read-back to `plan-apply`; media rollback deletes
the external payload before the asset, while skeletal rollback owns both the
mesh and Skeleton companion. Material and texture assets remain separate
planned operations. Repository-owned JSON semantic compilers and concrete
factories, world assembly, runtime binding, validation, cook, and packaging
remain explicit
blocked work until their complete native routes exist.

Generic or abstract `DataAsset` creation is not a
substitute for compiling normalized source into the project-owned typed runtime
contract. Source verification keeps portable plan identities separate from
physical
workspace locations: `extracted/...` and `fbx-assets/...` resolve below
`.cache/pipeline/`, while logical `manifest.jsonl` resolves to
`game/manifest/unreal.jsonl`. Reads do not follow links, SHA-256 streams from
stable file descriptors, and physical paths stay out of public reports. If an
import response is lost after the asset appears,
`plan-apply` treats that destination as created and compensates it.

A lost delete
response is accepted only when independent existence read-back proves the asset
is already absent.

The `call` command accepts either the native leaf name or the fully qualified
tool identity shown by `describe` and the generated skills. Qualified names are
validated against the selected toolset and converted to the leaf name required
by Unreal's `call_tool` meta-tool. Before that meta-tool can run, the translator
refreshes the live tool definition and validates the complete argument object.
Required fields, JSON types, nested objects and arrays, enums, constants,
patterns, supported bounds, additional-property policy, and supported
composition assertions fail locally when they do not match.

Unsupported
assertion keywords and ambiguous global lookup fail closed rather than bypassing
validation. This structural gate does not infer defaults or replace Unreal's
semantic and postcondition checks. `raw-call` remains available for top-level
non-mutating protocol tools, but it explicitly rejects the native `call_tool`
meta-tool so it cannot bypass schema validation.

The `skills` command discovers every live toolset and schema, verifies explicit
taxonomy ownership, and safely replaces only `skills/unreal/index.md` and
`skills/unreal/capabilities/**`. Manual workflow skills are preserved under a
lifecycle taxonomy for connection, planning, execution, assurance, maintenance,
and extension. `skills/unreal/workflows/README.md` is the only manual workflow
map, while the generated central index renders the same grouped routing. Each
per-tool file contains five project-evidence fields plus one protected reviewed-
revision token.

Regeneration updates the generated shell while preserving exact
text between valid field markers. It derives review status from the installed
Unreal MCP plugin `VersionName` and live interface digest; `1.0` is normalized
to public SemVer `1.0.0`. The Python package CalVer remains separate and is not
part of skill review identity. Mismatched or legacy guidance is marked **Review
required** without data loss.

The central index records the Unreal MCP version,
revision, and exact status counts. Malformed or unknown markers fail before
cleanup or writes. Existing output ancestors, the generated index, and the
capability tree must also be direct regular filesystem entries; symlinks,
junctions, and reparse boundaries fail before cleanup or replacement.

The generated tree contains one mandatory central index and exactly one skill
per native tool. Paths are derived from native names, such as
`automation/test/toolset/discover-tests.md`. New tools start with `[TODO]` and
`[FILL_ME]` fields; removed tools delete their obsolete generated files.

Use `--output RELATIVE_PATH` only for repository-relative test or review output.
Unsafe absolute or parent-traversing paths fail before any MCP session opens.

The wire adapter sends the canonical loopback `Origin` header and fails closed
unless initialization returns JSON-RPC 2.0, protocol version `2025-11-25`, a
Tools capability, text-typed server metadata, and a visible-ASCII session
identifier. UE 5.8 currently returns empty server name, title, and version
strings, so Unreal readiness is established by the required tool-search
meta-tools and a non-empty Toolset Registry rather than those informational
fields. The `doctor` result includes `toolsetCount` and reports `ready: false`
when the registry is empty. Subsequent requests carry the negotiated protocol
and session headers.

The driven execution gate serializes tool calls. This is intentionally stricter
than the native server, which can track multiple asynchronous requests at once.
Serial execution prevents overlapping editor mutations from producing
nondeterministic state.

Native tool outcomes preserve the complete raw result, concatenated text
fallback, and first-class `structuredContent` JSON value. Programmatic callers
can consume structured results without reparsing the text fallback, while the
CLI continues to print the complete raw MCP result.

HTTP JSON bodies, SSE streams, and session-delete responses are bounded to
64 MiB by default. Declared and streamed overflows fail before the response can
grow without limit. Programmatic transports may select a smaller positive
ceiling for constrained automation or deterministic tests.

When a native tool call exceeds the configured timeout, the translator sends a
`notifications/cancelled` notification with the original JSON-RPC request ID.
UE 5.8 removes the matching active request and invokes the tool's asynchronous
cancellation hook. The command still exits as a timeout; cancellation never
converts incomplete work into success.

Always start with the mandatory
[Unreal MCP capability index](../../../../../skills/unreal/index.md).

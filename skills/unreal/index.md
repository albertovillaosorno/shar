# Unreal MCP skill index

> Read this index first for every Unreal MCP task.

Generated from the live MCP interface; edit only protected fields.

This catalog follows the `docs/` navigation model.
It uses one central index and name-derived taxonomy folders.
Shared sibling prefixes become folders; unique suffixes become files.
Every link opens one focused per-tool skill.

- Unreal MCP version: `1.0.0`
- Interface digest: `c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Manual review revision: `1.0.0/c6e4275ffd125b32daf25b03c2746196b76c1fdd123994bde79239a30149342b`
- Toolsets: **52**
- Capabilities: **830**
- Manual guidance current: **169**
- Manual guidance review required: **661**
- Protocol: `2025-11-25`

## Usage

1. Read this index before selecting a capability.
1. Open the workflow skill for the operation stage.
1. Open the linked capability skill.
1. Fill protected fields only when project evidence exists.
1. Run `describe` against the live editor before every mutation.
1. Verify editor state independently after every mutation.

Regeneration preserves text inside manual-field markers.
Everything outside those markers is refreshed from live MCP metadata.
The protected review revision is never advanced automatically.
A version or interface change marks preserved guidance for review.
The live schema is authoritative when generated files drift.
Regenerate after Unreal Engine or Toolset plugin changes:

```text
shar-unreal-mcp skills
```

## Workflow skills

- [Editor readiness](workflows/editor-readiness.md)
- [Capability selection](workflows/capability-selection.md)
- [Schema and arguments](workflows/schema-and-arguments.md)
- [Read-only operations](workflows/read-only-operations.md)
- [Safe mutations](workflows/safe-mutations.md)
- [Long-running and batch operations](workflows/long-running-and-batch-operations.md)
- [Programmatic tool scripts](workflows/programmatic-tool-scripts.md)
- [Verification and recovery](workflows/verification-and-recovery.md)
- [Manual guidance maintenance](workflows/manual-guidance-maintenance.md)
- [Regeneration and taxonomy](workflows/regeneration-and-taxonomy.md)

## Capability taxonomy

### Core and governance

Editor health, configuration, plugins, logs, tests, and search.

8 toolsets; 65 capabilities.

#### `AutomationTestToolset.AutomationTestToolset`

Capabilities: **7**

- [`AutomationTestToolset.AutomationTestToolset.DiscoverTests`](capabilities/automation/test/toolset/discover-tests.md)
- [`AutomationTestToolset.AutomationTestToolset.GetTestResults`](capabilities/automation/test/toolset/get/test/results.md)
- [`AutomationTestToolset.AutomationTestToolset.GetTestStatus`](capabilities/automation/test/toolset/get/test/status.md)
- [`AutomationTestToolset.AutomationTestToolset.ListTests`](capabilities/automation/test/toolset/list-tests.md)
- [`AutomationTestToolset.AutomationTestToolset.RunTests`](capabilities/automation/test/toolset/run/tests.md)
- [`AutomationTestToolset.AutomationTestToolset.RunTestsByFilter`](capabilities/automation/test/toolset/run/tests-by-filter.md)
- [`AutomationTestToolset.AutomationTestToolset.StopTests`](capabilities/automation/test/toolset/stop-tests.md)

#### `ConfigSettingsToolset.ConfigSettingsToolset`

Capabilities: **8**

- [`ConfigSettingsToolset.ConfigSettingsToolset.GetSectionPropertyValues`](capabilities/config/settings/toolset/get/section/property-values.md)
- [`ConfigSettingsToolset.ConfigSettingsToolset.GetSectionSchema`](capabilities/config/settings/toolset/get/section/schema.md)
- [`ConfigSettingsToolset.ConfigSettingsToolset.ListCategories`](capabilities/config/settings/toolset/list/categories.md)
- [`ConfigSettingsToolset.ConfigSettingsToolset.ListContainers`](capabilities/config/settings/toolset/list/containers.md)
- [`ConfigSettingsToolset.ConfigSettingsToolset.ListSections`](capabilities/config/settings/toolset/list/sections.md)
- [`ConfigSettingsToolset.ConfigSettingsToolset.ResetSectionToDefaults`](capabilities/config/settings/toolset/reset-section-to-defaults.md)
- [`ConfigSettingsToolset.ConfigSettingsToolset.SaveSection`](capabilities/config/settings/toolset/save-section.md)
- [`ConfigSettingsToolset.ConfigSettingsToolset.SetSectionProperties`](capabilities/config/settings/toolset/set-section-properties.md)

#### `EditorToolset.EditorAppToolset`

Capabilities: **21**

- [`EditorToolset.EditorAppToolset.CaptureAssetImage`](capabilities/editor/toolset/app/capture/asset-image.md)
- [`EditorToolset.EditorAppToolset.CaptureEditorImage`](capabilities/editor/toolset/app/capture/editor-image.md)
- [`EditorToolset.EditorAppToolset.CaptureViewport`](capabilities/editor/toolset/app/capture/viewport.md)
- [`EditorToolset.EditorAppToolset.FocusOnActors`](capabilities/editor/toolset/app/focus-on-actors.md)
- [`EditorToolset.EditorAppToolset.GetCameraTransform`](capabilities/editor/toolset/app/get/camera-transform.md)
- [`EditorToolset.EditorAppToolset.GetContentBrowserPath`](capabilities/editor/toolset/app/get/content-browser-path.md)
- [`EditorToolset.EditorAppToolset.GetOpenAssets`](capabilities/editor/toolset/app/get/open-assets.md)
- [`EditorToolset.EditorAppToolset.GetSelectedActors`](capabilities/editor/toolset/app/get/selected/actors.md)
- [`EditorToolset.EditorAppToolset.GetSelectedAssets`](capabilities/editor/toolset/app/get/selected/assets.md)
- [`EditorToolset.EditorAppToolset.GetVisibleActors`](capabilities/editor/toolset/app/get/visible-actors.md)
- [`EditorToolset.EditorAppToolset.IsPIERunning`](capabilities/editor/toolset/app/is-pie-running.md)
- [`EditorToolset.EditorAppToolset.OpenEditorForAsset`](capabilities/editor/toolset/app/open-editor-for-asset.md)
- [`EditorToolset.EditorAppToolset.ScreenCoordsToWorld`](capabilities/editor/toolset/app/screen-coords-to-world.md)
- [`EditorToolset.EditorAppToolset.SearchCVars`](capabilities/editor/toolset/app/search-c-vars.md)
- [`EditorToolset.EditorAppToolset.SelectActors`](capabilities/editor/toolset/app/select/actors.md)
- [`EditorToolset.EditorAppToolset.SelectAssets`](capabilities/editor/toolset/app/select/assets.md)
- [`EditorToolset.EditorAppToolset.SetCameraTransform`](capabilities/editor/toolset/app/set/camera-transform.md)
- [`EditorToolset.EditorAppToolset.SetContentBrowserPath`](capabilities/editor/toolset/app/set/content-browser-path.md)
- [`EditorToolset.EditorAppToolset.StartPIE`](capabilities/editor/toolset/app/start-pie.md)
- [`EditorToolset.EditorAppToolset.StopPIE`](capabilities/editor/toolset/app/stop-pie.md)
- [`EditorToolset.EditorAppToolset.WorldPosToScreenCoords`](capabilities/editor/toolset/app/world-pos-to-screen-coords.md)

#### `EditorToolset.LogsToolset`

Capabilities: **4**

- [`EditorToolset.LogsToolset.GetLogCategories`](capabilities/editor/toolset/logs/get/log/categories.md)
- [`EditorToolset.LogsToolset.GetLogEntries`](capabilities/editor/toolset/logs/get/log/entries.md)
- [`EditorToolset.LogsToolset.GetVerbosity`](capabilities/editor/toolset/logs/get/verbosity.md)
- [`EditorToolset.LogsToolset.SetVerbosity`](capabilities/editor/toolset/logs/set-verbosity.md)

#### `PluginToolset.PluginToolset`

Capabilities: **17**

- [`PluginToolset.PluginToolset.AddPluginDependency`](capabilities/plugin/toolset/add-plugin-dependency.md)
- [`PluginToolset.PluginToolset.CreatePlugin`](capabilities/plugin/toolset/create-plugin.md)
- [`PluginToolset.PluginToolset.GetPluginDependencies`](capabilities/plugin/toolset/get/plugin/dependencies.md)
- [`PluginToolset.PluginToolset.GetPluginDependents`](capabilities/plugin/toolset/get/plugin/dependents.md)
- [`PluginToolset.PluginToolset.GetPluginDescriptor`](capabilities/plugin/toolset/get/plugin/descriptor.md)
- [`PluginToolset.PluginToolset.GetPluginForAsset`](capabilities/plugin/toolset/get/plugin/for-asset.md)
- [`PluginToolset.PluginToolset.GetPluginInfo`](capabilities/plugin/toolset/get/plugin/info.md)
- [`PluginToolset.PluginToolset.GetPluginTemplateDescriptions`](capabilities/plugin/toolset/get/plugin/template-descriptions.md)
- [`PluginToolset.PluginToolset.IsEnabled`](capabilities/plugin/toolset/is/enabled.md)
- [`PluginToolset.PluginToolset.IsPluginCreationAllowed`](capabilities/plugin/toolset/is/plugin/creation-allowed.md)
- [`PluginToolset.PluginToolset.IsPluginModificationAllowed`](capabilities/plugin/toolset/is/plugin/modification-allowed.md)
- [`PluginToolset.PluginToolset.ListDiscoveredPlugins`](capabilities/plugin/toolset/list/discovered-plugins.md)
- [`PluginToolset.PluginToolset.ListEnabledPlugins`](capabilities/plugin/toolset/list/enabled-plugins.md)
- [`PluginToolset.PluginToolset.RemovePluginDependency`](capabilities/plugin/toolset/remove-plugin-dependency.md)
- [`PluginToolset.PluginToolset.SetPluginEnabled`](capabilities/plugin/toolset/set-plugin-enabled.md)
- [`PluginToolset.PluginToolset.UpdatePluginDescriptor`](capabilities/plugin/toolset/update-plugin-descriptor.md)
- [`PluginToolset.PluginToolset.ValidateNewPluginNameAndLocation`](capabilities/plugin/toolset/validate-new-plugin-name-and-location.md)

#### `SemanticSearchToolset.SemanticSearchToolset`

Capabilities: **2**

- [`SemanticSearchToolset.SemanticSearchToolset.FindSimilar`](capabilities/semantic/search/toolset/find-similar.md)
- [`SemanticSearchToolset.SemanticSearchToolset.Search`](capabilities/semantic/search/toolset/search.md)

#### `ToolsetRegistry.AgentSkillToolset`

Capabilities: **4**

- [`ToolsetRegistry.AgentSkillToolset.CreateSkill`](capabilities/toolset/registry/agent/skill/create-skill.md)
- [`ToolsetRegistry.AgentSkillToolset.GetSkills`](capabilities/toolset/registry/agent/skill/get-skills.md)
- [`ToolsetRegistry.AgentSkillToolset.ListSkills`](capabilities/toolset/registry/agent/skill/list-skills.md)
- [`ToolsetRegistry.AgentSkillToolset.UpdateSkill`](capabilities/toolset/registry/agent/skill/update-skill.md)

#### `editor_toolset.toolsets.programmatic.ProgrammaticToolset`

Capabilities: **2**

- [`editor_toolset.toolsets.programmatic.ProgrammaticToolset.execute_tool_script`](capabilities/editor/toolset/programmatic/execute-tool-script.md)
- [`editor_toolset.toolsets.programmatic.ProgrammaticToolset.get_execution_environment`](capabilities/editor/toolset/programmatic/get-execution-environment.md)

### Assets and data

Assets, Blueprints, tables, materials, textures, and meshes.

13 toolsets; 187 capabilities.

#### `editor_toolset.toolsets.asset.AssetTools`

Capabilities: **21**

- [`editor_toolset.toolsets.asset.AssetTools.can_edit_asset`](capabilities/editor/toolset/asset/can-edit-asset.md)
- [`editor_toolset.toolsets.asset.AssetTools.create_folder`](capabilities/editor/toolset/asset/create-folder.md)
- [`editor_toolset.toolsets.asset.AssetTools.delete`](capabilities/editor/toolset/asset/delete.md)
- [`editor_toolset.toolsets.asset.AssetTools.duplicate`](capabilities/editor/toolset/asset/duplicate.md)
- [`editor_toolset.toolsets.asset.AssetTools.exists`](capabilities/editor/toolset/asset/exists.md)
- [`editor_toolset.toolsets.asset.AssetTools.find_assets`](capabilities/editor/toolset/asset/find-assets.md)
- [`editor_toolset.toolsets.asset.AssetTools.get_asset_class`](capabilities/editor/toolset/asset/get/asset/class.md)
- [`editor_toolset.toolsets.asset.AssetTools.get_asset_tags`](capabilities/editor/toolset/asset/get/asset/tags.md)
- [`editor_toolset.toolsets.asset.AssetTools.get_dependencies`](capabilities/editor/toolset/asset/get/dependencies.md)
- [`editor_toolset.toolsets.asset.AssetTools.get_metadata_tags`](capabilities/editor/toolset/asset/get/metadata-tags.md)
- [`editor_toolset.toolsets.asset.AssetTools.get_plugin_content_paths`](capabilities/editor/toolset/asset/get/plugin-content-paths.md)
- [`editor_toolset.toolsets.asset.AssetTools.get_referencers`](capabilities/editor/toolset/asset/get/referencers.md)
- [`editor_toolset.toolsets.asset.AssetTools.is_checked_out`](capabilities/editor/toolset/asset/is/checked-out.md)
- [`editor_toolset.toolsets.asset.AssetTools.is_dirty`](capabilities/editor/toolset/asset/is/dirty.md)
- [`editor_toolset.toolsets.asset.AssetTools.list_folders`](capabilities/editor/toolset/asset/list-folders.md)
- [`editor_toolset.toolsets.asset.AssetTools.load_asset`](capabilities/editor/toolset/asset/load-asset.md)
- [`editor_toolset.toolsets.asset.AssetTools.move`](capabilities/editor/toolset/asset/move.md)
- [`editor_toolset.toolsets.asset.AssetTools.read_file`](capabilities/editor/toolset/asset/read-file.md)
- [`editor_toolset.toolsets.asset.AssetTools.save_assets`](capabilities/editor/toolset/asset/save-assets.md)
- [`editor_toolset.toolsets.asset.AssetTools.update_metadata_tags`](capabilities/editor/toolset/asset/update-metadata-tags.md)
- [`editor_toolset.toolsets.asset.AssetTools.write_file`](capabilities/editor/toolset/asset/write-file.md)

#### `editor_toolset.toolsets.blueprint.BlueprintTools`

Capabilities: **53**

- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_component_bound_event`](capabilities/editor/toolset/blueprint/add/component-bound-event.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_event`](capabilities/editor/toolset/blueprint/add/event.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_event_dispatcher`](capabilities/editor/toolset/blueprint/add/event-dispatcher.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_function_graph`](capabilities/editor/toolset/blueprint/add/function/graph.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_function_param`](capabilities/editor/toolset/blueprint/add/function/param.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_node_pin`](capabilities/editor/toolset/blueprint/add/node-pin.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_object_function_param`](capabilities/editor/toolset/blueprint/add/object/function-param.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_object_variable`](capabilities/editor/toolset/blueprint/add/object/variable.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_struct_function_param`](capabilities/editor/toolset/blueprint/add/struct/function-param.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_struct_variable`](capabilities/editor/toolset/blueprint/add/struct/variable.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.add_variable`](capabilities/editor/toolset/blueprint/add/variable.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.arrange_nodes`](capabilities/editor/toolset/blueprint/arrange-nodes.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.break_pins`](capabilities/editor/toolset/blueprint/break-pins.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.compile_blueprint`](capabilities/editor/toolset/blueprint/compile-blueprint.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.connect_pins`](capabilities/editor/toolset/blueprint/connect-pins.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.create`](capabilities/editor/toolset/blueprint/create.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.create_node`](capabilities/editor/toolset/blueprint/create-node.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.delete_node`](capabilities/editor/toolset/blueprint/delete-node.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.find_node_categories`](capabilities/editor/toolset/blueprint/find/node/categories.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.find_node_types`](capabilities/editor/toolset/blueprint/find/node/types.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.find_nodes`](capabilities/editor/toolset/blueprint/find/nodes.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_connected_subgraph`](capabilities/editor/toolset/blueprint/get/connected-subgraph.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_create_event_function`](capabilities/editor/toolset/blueprint/get/create-event-function.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_default_object`](capabilities/editor/toolset/blueprint/get/default-object.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_graph`](capabilities/editor/toolset/blueprint/get/graph.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_graph_dsl_docs`](capabilities/editor/toolset/blueprint/get/graph-dsl-docs.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_node_infos`](capabilities/editor/toolset/blueprint/get/node/infos.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_node_type_pins`](capabilities/editor/toolset/blueprint/get/node/type-pins.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_parent`](capabilities/editor/toolset/blueprint/get/parent.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_pin_value`](capabilities/editor/toolset/blueprint/get/pin-value.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_variable_category`](capabilities/editor/toolset/blueprint/get/variable/category.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.get_variable_replication`](capabilities/editor/toolset/blueprint/get/variable/replication.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.list_compatible_event_functions`](capabilities/editor/toolset/blueprint/list/compatible-event-functions.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.list_component_events`](capabilities/editor/toolset/blueprint/list/component-events.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.list_event_dispatchers`](capabilities/editor/toolset/blueprint/list/event-dispatchers.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.list_events`](capabilities/editor/toolset/blueprint/list/events.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.list_functions`](capabilities/editor/toolset/blueprint/list/functions.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.list_graphs`](capabilities/editor/toolset/blueprint/list/graphs.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.list_variables`](capabilities/editor/toolset/blueprint/list/variables.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.read_graph_dsl`](capabilities/editor/toolset/blueprint/read-graph-dsl.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.remove_function_graph`](capabilities/editor/toolset/blueprint/remove/function/graph.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.remove_function_param`](capabilities/editor/toolset/blueprint/remove/function/param.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.remove_node_pin`](capabilities/editor/toolset/blueprint/remove/node-pin.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.remove_variable`](capabilities/editor/toolset/blueprint/remove/variable.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.retarget_node_class`](capabilities/editor/toolset/blueprint/retarget-node-class.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.set_create_event_function`](capabilities/editor/toolset/blueprint/set/create-event-function.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.set_node_position`](capabilities/editor/toolset/blueprint/set/node-position.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.set_parent`](capabilities/editor/toolset/blueprint/set/parent.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.set_pin_value`](capabilities/editor/toolset/blueprint/set/pin-value.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.set_variable_category`](capabilities/editor/toolset/blueprint/set/variable/category.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.set_variable_instance_editable`](capabilities/editor/toolset/blueprint/set/variable/instance-editable.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.set_variable_replication`](capabilities/editor/toolset/blueprint/set/variable/replication.md)
- [`editor_toolset.toolsets.blueprint.BlueprintTools.write_graph_dsl`](capabilities/editor/toolset/blueprint/write-graph-dsl.md)

#### `editor_toolset.toolsets.curve_table.CurveTableTools`

Capabilities: **9**

- [`editor_toolset.toolsets.curve_table.CurveTableTools.add_key`](capabilities/editor/toolset/curve/table/add/key.md)
- [`editor_toolset.toolsets.curve_table.CurveTableTools.add_row`](capabilities/editor/toolset/curve/table/add/row.md)
- [`editor_toolset.toolsets.curve_table.CurveTableTools.create`](capabilities/editor/toolset/curve/table/create.md)
- [`editor_toolset.toolsets.curve_table.CurveTableTools.get_keys`](capabilities/editor/toolset/curve/table/get-keys.md)
- [`editor_toolset.toolsets.curve_table.CurveTableTools.import_file`](capabilities/editor/toolset/curve/table/import-file.md)
- [`editor_toolset.toolsets.curve_table.CurveTableTools.list_rows`](capabilities/editor/toolset/curve/table/list-rows.md)
- [`editor_toolset.toolsets.curve_table.CurveTableTools.remove_row`](capabilities/editor/toolset/curve/table/remove-row.md)
- [`editor_toolset.toolsets.curve_table.CurveTableTools.rename_row`](capabilities/editor/toolset/curve/table/rename-row.md)
- [`editor_toolset.toolsets.curve_table.CurveTableTools.set_keys`](capabilities/editor/toolset/curve/table/set-keys.md)

#### `editor_toolset.toolsets.data_asset.DataAssetTools`

Capabilities: **1**

- [`editor_toolset.toolsets.data_asset.DataAssetTools.create`](capabilities/editor/toolset/data/asset/create.md)

#### `editor_toolset.toolsets.data_table.DataTableTools`

Capabilities: **10**

- [`editor_toolset.toolsets.data_table.DataTableTools.add_rows`](capabilities/editor/toolset/data/table/add-rows.md)
- [`editor_toolset.toolsets.data_table.DataTableTools.create`](capabilities/editor/toolset/data/table/create.md)
- [`editor_toolset.toolsets.data_table.DataTableTools.get_rows`](capabilities/editor/toolset/data/table/get/rows.md)
- [`editor_toolset.toolsets.data_table.DataTableTools.get_schema`](capabilities/editor/toolset/data/table/get/schema.md)
- [`editor_toolset.toolsets.data_table.DataTableTools.import_file`](capabilities/editor/toolset/data/table/import-file.md)
- [`editor_toolset.toolsets.data_table.DataTableTools.list_rows`](capabilities/editor/toolset/data/table/list-rows.md)
- [`editor_toolset.toolsets.data_table.DataTableTools.remove_rows`](capabilities/editor/toolset/data/table/remove-rows.md)
- [`editor_toolset.toolsets.data_table.DataTableTools.rename_rows`](capabilities/editor/toolset/data/table/rename-rows.md)
- [`editor_toolset.toolsets.data_table.DataTableTools.search_row_structs`](capabilities/editor/toolset/data/table/search-row-structs.md)
- [`editor_toolset.toolsets.data_table.DataTableTools.set_rows`](capabilities/editor/toolset/data/table/set-rows.md)

#### `editor_toolset.toolsets.material.MaterialTools`

Capabilities: **22**

- [`editor_toolset.toolsets.material.MaterialTools.add_expression`](capabilities/editor/toolset/material/add-expression.md)
- [`editor_toolset.toolsets.material.MaterialTools.connect_expressions`](capabilities/editor/toolset/material/connect/expressions.md)
- [`editor_toolset.toolsets.material.MaterialTools.connect_to_output`](capabilities/editor/toolset/material/connect/to-output.md)
- [`editor_toolset.toolsets.material.MaterialTools.create_function`](capabilities/editor/toolset/material/create/function.md)
- [`editor_toolset.toolsets.material.MaterialTools.create_material`](capabilities/editor/toolset/material/create/material.md)
- [`editor_toolset.toolsets.material.MaterialTools.create_parameter_collection`](capabilities/editor/toolset/material/create/parameter-collection.md)
- [`editor_toolset.toolsets.material.MaterialTools.delete_expression`](capabilities/editor/toolset/material/delete/expression.md)
- [`editor_toolset.toolsets.material.MaterialTools.delete_parameter_group`](capabilities/editor/toolset/material/delete/parameter-group.md)
- [`editor_toolset.toolsets.material.MaterialTools.delete_unused_expressions`](capabilities/editor/toolset/material/delete/unused-expressions.md)
- [`editor_toolset.toolsets.material.MaterialTools.disconnect_expressions`](capabilities/editor/toolset/material/disconnect/expressions.md)
- [`editor_toolset.toolsets.material.MaterialTools.disconnect_from_output`](capabilities/editor/toolset/material/disconnect/from-output.md)
- [`editor_toolset.toolsets.material.MaterialTools.get_expression_input_names`](capabilities/editor/toolset/material/get/expression/input-names.md)
- [`editor_toolset.toolsets.material.MaterialTools.get_expression_inputs`](capabilities/editor/toolset/material/get/expression/inputs.md)
- [`editor_toolset.toolsets.material.MaterialTools.get_expression_output_names`](capabilities/editor/toolset/material/get/expression/output-names.md)
- [`editor_toolset.toolsets.material.MaterialTools.get_expressions`](capabilities/editor/toolset/material/get/expressions.md)
- [`editor_toolset.toolsets.material.MaterialTools.get_property_input`](capabilities/editor/toolset/material/get/property-input.md)
- [`editor_toolset.toolsets.material.MaterialTools.get_referencing_materials`](capabilities/editor/toolset/material/get/referencing-materials.md)
- [`editor_toolset.toolsets.material.MaterialTools.layout_expressions`](capabilities/editor/toolset/material/layout-expressions.md)
- [`editor_toolset.toolsets.material.MaterialTools.list_expression_classes`](capabilities/editor/toolset/material/list/expression-classes.md)
- [`editor_toolset.toolsets.material.MaterialTools.list_parameter_groups`](capabilities/editor/toolset/material/list/parameter-groups.md)
- [`editor_toolset.toolsets.material.MaterialTools.recompile`](capabilities/editor/toolset/material/recompile.md)
- [`editor_toolset.toolsets.material.MaterialTools.rename_parameter_group`](capabilities/editor/toolset/material/rename-parameter-group.md)

#### `editor_toolset.toolsets.material_instance.MaterialInstanceTools`

Capabilities: **13**

- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.clear_parameters`](capabilities/editor/toolset/material/instance/clear-parameters.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.create`](capabilities/editor/toolset/material/instance/create.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.get_scalar_parameter`](capabilities/editor/toolset/material/instance/get/scalar-parameter.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.get_static_switch_parameter`](capabilities/editor/toolset/material/instance/get/static-switch-parameter.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.get_texture_parameter`](capabilities/editor/toolset/material/instance/get/texture-parameter.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.get_vector_parameter`](capabilities/editor/toolset/material/instance/get/vector-parameter.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.list_parameters`](capabilities/editor/toolset/material/instance/list-parameters.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.set_parameter_override`](capabilities/editor/toolset/material/instance/set/parameter-override.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.set_parent`](capabilities/editor/toolset/material/instance/set/parent.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.set_scalar_parameter`](capabilities/editor/toolset/material/instance/set/scalar-parameter.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.set_static_switch_parameter`](capabilities/editor/toolset/material/instance/set/static-switch-parameter.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.set_texture_parameter`](capabilities/editor/toolset/material/instance/set/texture-parameter.md)
- [`editor_toolset.toolsets.material_instance.MaterialInstanceTools.set_vector_parameter`](capabilities/editor/toolset/material/instance/set/vector-parameter.md)

#### `editor_toolset.toolsets.object.ObjectTools`

Capabilities: **6**

- [`editor_toolset.toolsets.object.ObjectTools.get_class`](capabilities/editor/toolset/object/get/class.md)
- [`editor_toolset.toolsets.object.ObjectTools.get_properties`](capabilities/editor/toolset/object/get/properties.md)
- [`editor_toolset.toolsets.object.ObjectTools.list_properties`](capabilities/editor/toolset/object/list-properties.md)
- [`editor_toolset.toolsets.object.ObjectTools.reset_properties`](capabilities/editor/toolset/object/reset-properties.md)
- [`editor_toolset.toolsets.object.ObjectTools.search_subclasses`](capabilities/editor/toolset/object/search-subclasses.md)
- [`editor_toolset.toolsets.object.ObjectTools.set_properties`](capabilities/editor/toolset/object/set-properties.md)

#### `editor_toolset.toolsets.primitive.PrimitiveTools`

Capabilities: **4**

- [`editor_toolset.toolsets.primitive.PrimitiveTools.add_cone`](capabilities/editor/toolset/primitive/add/cone.md)
- [`editor_toolset.toolsets.primitive.PrimitiveTools.add_cube`](capabilities/editor/toolset/primitive/add/cube.md)
- [`editor_toolset.toolsets.primitive.PrimitiveTools.add_cylinder`](capabilities/editor/toolset/primitive/add/cylinder.md)
- [`editor_toolset.toolsets.primitive.PrimitiveTools.add_sphere`](capabilities/editor/toolset/primitive/add/sphere.md)

#### `editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools`

Capabilities: **22**

- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.add_socket`](capabilities/editor/toolset/skeletal/mesh/add-socket.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.assign_physics_asset`](capabilities/editor/toolset/skeletal/mesh/assign-physics-asset.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_bone_children`](capabilities/editor/toolset/skeletal/mesh/get/bone/children.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_bone_names`](capabilities/editor/toolset/skeletal/mesh/get/bone/names.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_bone_parent`](capabilities/editor/toolset/skeletal/mesh/get/bone/parent.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_bounds`](capabilities/editor/toolset/skeletal/mesh/get/bounds.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_lod_count`](capabilities/editor/toolset/skeletal/mesh/get/lod-count.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_material`](capabilities/editor/toolset/skeletal/mesh/get/material.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_material_slots`](capabilities/editor/toolset/skeletal/mesh/get/material-slots.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_morph_target_names`](capabilities/editor/toolset/skeletal/mesh/get/morph-target-names.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_physics_asset`](capabilities/editor/toolset/skeletal/mesh/get/physics-asset.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_section_count`](capabilities/editor/toolset/skeletal/mesh/get/section-count.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_skeleton`](capabilities/editor/toolset/skeletal/mesh/get/skeleton.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_socket_bone`](capabilities/editor/toolset/skeletal/mesh/get/socket/bone.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_socket_names`](capabilities/editor/toolset/skeletal/mesh/get/socket/names.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_socket_transform`](capabilities/editor/toolset/skeletal/mesh/get/socket/transform.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.get_vertex_count`](capabilities/editor/toolset/skeletal/mesh/get/vertex-count.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.import_file`](capabilities/editor/toolset/skeletal/mesh/import-file.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.remove_socket`](capabilities/editor/toolset/skeletal/mesh/remove-socket.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.rename_socket`](capabilities/editor/toolset/skeletal/mesh/rename-socket.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.set_material`](capabilities/editor/toolset/skeletal/mesh/set/material.md)
- [`editor_toolset.toolsets.skeletal_mesh.SkeletalMeshTools.set_socket_transform`](capabilities/editor/toolset/skeletal/mesh/set/socket-transform.md)

#### `editor_toolset.toolsets.static_mesh.StaticMeshTools`

Capabilities: **16**

- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.generate_convex_collisions`](capabilities/editor/toolset/static/mesh/generate/convex-collisions.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.generate_lods`](capabilities/editor/toolset/static/mesh/generate/lods.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.get_bounds`](capabilities/editor/toolset/static/mesh/get/bounds.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.get_lod_count`](capabilities/editor/toolset/static/mesh/get/lod/count.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.get_lod_thresholds`](capabilities/editor/toolset/static/mesh/get/lod/thresholds.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.get_material`](capabilities/editor/toolset/static/mesh/get/material.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.get_material_slots`](capabilities/editor/toolset/static/mesh/get/material-slots.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.get_triangle_count`](capabilities/editor/toolset/static/mesh/get/triangle-count.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.get_vertex_count`](capabilities/editor/toolset/static/mesh/get/vertex-count.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.import_file`](capabilities/editor/toolset/static/mesh/import-file.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.is_nanite_enabled`](capabilities/editor/toolset/static/mesh/is-nanite-enabled.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.remove_collisions`](capabilities/editor/toolset/static/mesh/remove/collisions.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.remove_lods`](capabilities/editor/toolset/static/mesh/remove/lods.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.set_lod_thresholds`](capabilities/editor/toolset/static/mesh/set/lod-thresholds.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.set_material`](capabilities/editor/toolset/static/mesh/set/material.md)
- [`editor_toolset.toolsets.static_mesh.StaticMeshTools.set_nanite_enabled`](capabilities/editor/toolset/static/mesh/set/nanite-enabled.md)

#### `editor_toolset.toolsets.string_table.StringTableTools`

Capabilities: **8**

- [`editor_toolset.toolsets.string_table.StringTableTools.create`](capabilities/editor/toolset/string/table/create.md)
- [`editor_toolset.toolsets.string_table.StringTableTools.get_entry`](capabilities/editor/toolset/string/table/get/entry.md)
- [`editor_toolset.toolsets.string_table.StringTableTools.get_namespace`](capabilities/editor/toolset/string/table/get/namespace.md)
- [`editor_toolset.toolsets.string_table.StringTableTools.get_table_id`](capabilities/editor/toolset/string/table/get/table-id.md)
- [`editor_toolset.toolsets.string_table.StringTableTools.import_file`](capabilities/editor/toolset/string/table/import-file.md)
- [`editor_toolset.toolsets.string_table.StringTableTools.list_keys`](capabilities/editor/toolset/string/table/list-keys.md)
- [`editor_toolset.toolsets.string_table.StringTableTools.remove_entry`](capabilities/editor/toolset/string/table/remove-entry.md)
- [`editor_toolset.toolsets.string_table.StringTableTools.set_entry`](capabilities/editor/toolset/string/table/set-entry.md)

#### `editor_toolset.toolsets.texture.TextureTools`

Capabilities: **2**

- [`editor_toolset.toolsets.texture.TextureTools.get_size`](capabilities/editor/toolset/texture/get-size.md)
- [`editor_toolset.toolsets.texture.TextureTools.import_file`](capabilities/editor/toolset/texture/import-file.md)

### World and UI

Actors, levels, Slate inspection, and UMG authoring.

4 toolsets; 74 capabilities.

#### `SlateInspectorToolset.SlateInspectorToolset`

Capabilities: **14**

- [`SlateInspectorToolset.SlateInspectorToolset.Click`](capabilities/slate/inspector/toolset/click.md)
- [`SlateInspectorToolset.SlateInspectorToolset.Drag`](capabilities/slate/inspector/toolset/drag.md)
- [`SlateInspectorToolset.SlateInspectorToolset.FillForm`](capabilities/slate/inspector/toolset/fill-form.md)
- [`SlateInspectorToolset.SlateInspectorToolset.Hover`](capabilities/slate/inspector/toolset/hover.md)
- [`SlateInspectorToolset.SlateInspectorToolset.ListObservers`](capabilities/slate/inspector/toolset/list-observers.md)
- [`SlateInspectorToolset.SlateInspectorToolset.Observe`](capabilities/slate/inspector/toolset/observe.md)
- [`SlateInspectorToolset.SlateInspectorToolset.PressKey`](capabilities/slate/inspector/toolset/press-key.md)
- [`SlateInspectorToolset.SlateInspectorToolset.Screenshot`](capabilities/slate/inspector/toolset/screenshot.md)
- [`SlateInspectorToolset.SlateInspectorToolset.SelectOption`](capabilities/slate/inspector/toolset/select-option.md)
- [`SlateInspectorToolset.SlateInspectorToolset.Snapshot`](capabilities/slate/inspector/toolset/snapshot.md)
- [`SlateInspectorToolset.SlateInspectorToolset.Type`](capabilities/slate/inspector/toolset/type.md)
- [`SlateInspectorToolset.SlateInspectorToolset.Unobserve`](capabilities/slate/inspector/toolset/unobserve.md)
- [`SlateInspectorToolset.SlateInspectorToolset.WaitFor`](capabilities/slate/inspector/toolset/wait-for.md)
- [`SlateInspectorToolset.SlateInspectorToolset.Windows`](capabilities/slate/inspector/toolset/windows.md)

#### `UMGToolSet.UMGToolSet`

Capabilities: **23**

- [`UMGToolSet.UMGToolSet.AddUIComponent`](capabilities/umg/tool/set/add/ui-component.md)
- [`UMGToolSet.UMGToolSet.AddWidget`](capabilities/umg/tool/set/add/widget.md)
- [`UMGToolSet.UMGToolSet.BindToEventProperty`](capabilities/umg/tool/set/bind-to-event-property.md)
- [`UMGToolSet.UMGToolSet.CompileWidgetBlueprint`](capabilities/umg/tool/set/compile-widget-blueprint.md)
- [`UMGToolSet.UMGToolSet.CreateWidgetBlueprint`](capabilities/umg/tool/set/create-widget-blueprint.md)
- [`UMGToolSet.UMGToolSet.GetNamedSlots`](capabilities/umg/tool/set/get/named-slots.md)
- [`UMGToolSet.UMGToolSet.GetWidgetClassInfo`](capabilities/umg/tool/set/get/widget/class-info.md)
- [`UMGToolSet.UMGToolSet.GetWidgetDescription`](capabilities/umg/tool/set/get/widget/description.md)
- [`UMGToolSet.UMGToolSet.GetWidgetTreeDepth`](capabilities/umg/tool/set/get/widget/tree-depth.md)
- [`UMGToolSet.UMGToolSet.GetWidgets`](capabilities/umg/tool/set/get/widgets.md)
- [`UMGToolSet.UMGToolSet.ListWidgetBlueprints`](capabilities/umg/tool/set/list/widget/blueprints.md)
- [`UMGToolSet.UMGToolSet.ListWidgetClasses`](capabilities/umg/tool/set/list/widget/classes.md)
- [`UMGToolSet.UMGToolSet.MoveUIComponent`](capabilities/umg/tool/set/move/ui-component.md)
- [`UMGToolSet.UMGToolSet.MoveWidget`](capabilities/umg/tool/set/move/widget.md)
- [`UMGToolSet.UMGToolSet.RemoveUIComponent`](capabilities/umg/tool/set/remove/ui-component.md)
- [`UMGToolSet.UMGToolSet.RemoveWidget`](capabilities/umg/tool/set/remove/widget.md)
- [`UMGToolSet.UMGToolSet.RenameWidget`](capabilities/umg/tool/set/rename-widget.md)
- [`UMGToolSet.UMGToolSet.ReplaceWidgetWithChild`](capabilities/umg/tool/set/replace/widget/with/child.md)
- [`UMGToolSet.UMGToolSet.ReplaceWidgetWithNamedSlot`](capabilities/umg/tool/set/replace/widget/with/named-slot.md)
- [`UMGToolSet.UMGToolSet.ReplaceWidgetWithTemplate`](capabilities/umg/tool/set/replace/widget/with/template.md)
- [`UMGToolSet.UMGToolSet.SetNamedSlotContent`](capabilities/umg/tool/set/set-named-slot-content.md)
- [`UMGToolSet.UMGToolSet.ToggleWidgetAsVariable`](capabilities/umg/tool/set/toggle-widget-as-variable.md)
- [`UMGToolSet.UMGToolSet.WrapWidgets`](capabilities/umg/tool/set/wrap-widgets.md)

#### `editor_toolset.toolsets.actor.ActorTools`

Capabilities: **17**

- [`editor_toolset.toolsets.actor.ActorTools.add_component`](capabilities/editor/toolset/actor/add/component.md)
- [`editor_toolset.toolsets.actor.ActorTools.add_tag`](capabilities/editor/toolset/actor/add/tag.md)
- [`editor_toolset.toolsets.actor.ActorTools.get_actor_bounds`](capabilities/editor/toolset/actor/get/actor/bounds.md)
- [`editor_toolset.toolsets.actor.ActorTools.get_actor_transform`](capabilities/editor/toolset/actor/get/actor/transform.md)
- [`editor_toolset.toolsets.actor.ActorTools.get_component_actor`](capabilities/editor/toolset/actor/get/component-actor.md)
- [`editor_toolset.toolsets.actor.ActorTools.get_components`](capabilities/editor/toolset/actor/get/components.md)
- [`editor_toolset.toolsets.actor.ActorTools.get_label`](capabilities/editor/toolset/actor/get/label.md)
- [`editor_toolset.toolsets.actor.ActorTools.get_parent_component`](capabilities/editor/toolset/actor/get/parent-component.md)
- [`editor_toolset.toolsets.actor.ActorTools.get_root_component`](capabilities/editor/toolset/actor/get/root-component.md)
- [`editor_toolset.toolsets.actor.ActorTools.get_tags`](capabilities/editor/toolset/actor/get/tags.md)
- [`editor_toolset.toolsets.actor.ActorTools.has_tag`](capabilities/editor/toolset/actor/has-tag.md)
- [`editor_toolset.toolsets.actor.ActorTools.look_at`](capabilities/editor/toolset/actor/look-at.md)
- [`editor_toolset.toolsets.actor.ActorTools.remove_component`](capabilities/editor/toolset/actor/remove/component.md)
- [`editor_toolset.toolsets.actor.ActorTools.remove_tag`](capabilities/editor/toolset/actor/remove/tag.md)
- [`editor_toolset.toolsets.actor.ActorTools.set_actor_transform`](capabilities/editor/toolset/actor/set/actor-transform.md)
- [`editor_toolset.toolsets.actor.ActorTools.set_label`](capabilities/editor/toolset/actor/set/label.md)
- [`editor_toolset.toolsets.actor.ActorTools.set_parent_component`](capabilities/editor/toolset/actor/set/parent-component.md)

#### `editor_toolset.toolsets.scene.SceneTools`

Capabilities: **20**

- [`editor_toolset.toolsets.scene.SceneTools.add_to_scene_from_asset`](capabilities/editor/toolset/scene/add/to/scene/from/asset.md)
- [`editor_toolset.toolsets.scene.SceneTools.add_to_scene_from_class`](capabilities/editor/toolset/scene/add/to/scene/from/class.md)
- [`editor_toolset.toolsets.scene.SceneTools.can_edit`](capabilities/editor/toolset/scene/can-edit.md)
- [`editor_toolset.toolsets.scene.SceneTools.commit_level_instance`](capabilities/editor/toolset/scene/commit-level-instance.md)
- [`editor_toolset.toolsets.scene.SceneTools.create_level_instance`](capabilities/editor/toolset/scene/create-level-instance.md)
- [`editor_toolset.toolsets.scene.SceneTools.delete_folder`](capabilities/editor/toolset/scene/delete-folder.md)
- [`editor_toolset.toolsets.scene.SceneTools.edit_level_instance`](capabilities/editor/toolset/scene/edit-level-instance.md)
- [`editor_toolset.toolsets.scene.SceneTools.find_actors`](capabilities/editor/toolset/scene/find-actors.md)
- [`editor_toolset.toolsets.scene.SceneTools.get_actors_in_folder`](capabilities/editor/toolset/scene/get/actors-in-folder.md)
- [`editor_toolset.toolsets.scene.SceneTools.get_collision_channels`](capabilities/editor/toolset/scene/get/collision-channels.md)
- [`editor_toolset.toolsets.scene.SceneTools.get_current_level`](capabilities/editor/toolset/scene/get/current-level.md)
- [`editor_toolset.toolsets.scene.SceneTools.get_folders`](capabilities/editor/toolset/scene/get/folders.md)
- [`editor_toolset.toolsets.scene.SceneTools.is_checked_out`](capabilities/editor/toolset/scene/is-checked-out.md)
- [`editor_toolset.toolsets.scene.SceneTools.load_level`](capabilities/editor/toolset/scene/load-level.md)
- [`editor_toolset.toolsets.scene.SceneTools.merge_actors`](capabilities/editor/toolset/scene/merge-actors.md)
- [`editor_toolset.toolsets.scene.SceneTools.remove_from_scene`](capabilities/editor/toolset/scene/remove-from-scene.md)
- [`editor_toolset.toolsets.scene.SceneTools.rename_folder`](capabilities/editor/toolset/scene/rename-folder.md)
- [`editor_toolset.toolsets.scene.SceneTools.save_actor`](capabilities/editor/toolset/scene/save-actor.md)
- [`editor_toolset.toolsets.scene.SceneTools.set_actor_folder`](capabilities/editor/toolset/scene/set-actor-folder.md)
- [`editor_toolset.toolsets.scene.SceneTools.trace_world`](capabilities/editor/toolset/scene/trace-world.md)

### Animation and cinematics

Sequencer, keyframing, bindings, Control Rig, and animation exchange.

8 toolsets; 319 capabilities.

#### `animation_toolset.toolsets.conditions.SequencerConditionTools`

Capabilities: **9**

- [`animation_toolset.toolsets.conditions.SequencerConditionTools.clear_section_condition`](capabilities/animation/toolset/conditions/sequencer/condition/clear/section-condition.md)
- [`animation_toolset.toolsets.conditions.SequencerConditionTools.clear_track_condition`](capabilities/animation/toolset/conditions/sequencer/condition/clear/track/condition.md)
- [`animation_toolset.toolsets.conditions.SequencerConditionTools.clear_track_row_condition`](capabilities/animation/toolset/conditions/sequencer/condition/clear/track/row-condition.md)
- [`animation_toolset.toolsets.conditions.SequencerConditionTools.get_section_condition`](capabilities/animation/toolset/conditions/sequencer/condition/get/section-condition.md)
- [`animation_toolset.toolsets.conditions.SequencerConditionTools.get_track_condition`](capabilities/animation/toolset/conditions/sequencer/condition/get/track/condition.md)
- [`animation_toolset.toolsets.conditions.SequencerConditionTools.get_track_row_condition`](capabilities/animation/toolset/conditions/sequencer/condition/get/track/row-condition.md)
- [`animation_toolset.toolsets.conditions.SequencerConditionTools.set_section_condition`](capabilities/animation/toolset/conditions/sequencer/condition/set/section-condition.md)
- [`animation_toolset.toolsets.conditions.SequencerConditionTools.set_track_condition`](capabilities/animation/toolset/conditions/sequencer/condition/set/track/condition.md)
- [`animation_toolset.toolsets.conditions.SequencerConditionTools.set_track_row_condition`](capabilities/animation/toolset/conditions/sequencer/condition/set/track/row-condition.md)

#### `animation_toolset.toolsets.controlrig.ControlRigTools`

Capabilities: **44**

- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_backward_solve_graph`](capabilities/animation/toolset/controlrig/control/rig/add/backward-solve-graph.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_bone`](capabilities/animation/toolset/controlrig/control/rig/add/bone.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_control`](capabilities/animation/toolset/controlrig/control/rig/add/control.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_element`](capabilities/animation/toolset/controlrig/control/rig/add/element.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_event_graph`](capabilities/animation/toolset/controlrig/control/rig/add/event/graph.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_event_node`](capabilities/animation/toolset/controlrig/control/rig/add/event/node.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_graph`](capabilities/animation/toolset/controlrig/control/rig/add/graph.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_interaction_graph`](capabilities/animation/toolset/controlrig/control/rig/add/interaction-graph.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_null`](capabilities/animation/toolset/controlrig/control/rig/add/null.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_variable`](capabilities/animation/toolset/controlrig/control/rig/add/variable.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.add_variable_node`](capabilities/animation/toolset/controlrig/control/rig/add/variable-node.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.change_variable_type`](capabilities/animation/toolset/controlrig/control/rig/change-variable-type.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.connect_pins`](capabilities/animation/toolset/controlrig/control/rig/connect-pins.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.create`](capabilities/animation/toolset/controlrig/control/rig/create.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.create_node`](capabilities/animation/toolset/controlrig/control/rig/create-node.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.delete_node`](capabilities/animation/toolset/controlrig/control/rig/delete-node.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.disconnect_pins`](capabilities/animation/toolset/controlrig/control/rig/disconnect-pins.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_all_bones`](capabilities/animation/toolset/controlrig/control/rig/get/all/bones.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_all_controls`](capabilities/animation/toolset/controlrig/control/rig/get/all/controls.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_all_nulls`](capabilities/animation/toolset/controlrig/control/rig/get/all/nulls.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_backward_solve_graph`](capabilities/animation/toolset/controlrig/control/rig/get/backward-solve-graph.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_children`](capabilities/animation/toolset/controlrig/control/rig/get/children.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_connected_pins`](capabilities/animation/toolset/controlrig/control/rig/get/connected-pins.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_elements`](capabilities/animation/toolset/controlrig/control/rig/get/elements.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_event_graph`](capabilities/animation/toolset/controlrig/control/rig/get/event-graph.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_forward_solve_graph`](capabilities/animation/toolset/controlrig/control/rig/get/forward-solve-graph.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_global_transform`](capabilities/animation/toolset/controlrig/control/rig/get/global-transform.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_graph`](capabilities/animation/toolset/controlrig/control/rig/get/graph.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_interaction_graph`](capabilities/animation/toolset/controlrig/control/rig/get/interaction-graph.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_local_transform`](capabilities/animation/toolset/controlrig/control/rig/get/local-transform.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_node_position`](capabilities/animation/toolset/controlrig/control/rig/get/node-position.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_parent`](capabilities/animation/toolset/controlrig/control/rig/get/parent.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_pin_value`](capabilities/animation/toolset/controlrig/control/rig/get/pin-value.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.get_variable`](capabilities/animation/toolset/controlrig/control/rig/get/variable.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.import_bones_from_asset`](capabilities/animation/toolset/controlrig/control/rig/import-bones-from-asset.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.list_graphs`](capabilities/animation/toolset/controlrig/control/rig/list/graphs.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.list_nodes`](capabilities/animation/toolset/controlrig/control/rig/list/nodes.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.list_pins`](capabilities/animation/toolset/controlrig/control/rig/list/pins.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.list_variables`](capabilities/animation/toolset/controlrig/control/rig/list/variables.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.remove_variable`](capabilities/animation/toolset/controlrig/control/rig/remove-variable.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.set_global_transform`](capabilities/animation/toolset/controlrig/control/rig/set/global-transform.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.set_local_transform`](capabilities/animation/toolset/controlrig/control/rig/set/local-transform.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.set_node_position`](capabilities/animation/toolset/controlrig/control/rig/set/node-position.md)
- [`animation_toolset.toolsets.controlrig.ControlRigTools.set_pin_value`](capabilities/animation/toolset/controlrig/control/rig/set/pin-value.md)

#### `animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools`

Capabilities: **72**

- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.add_layer_from_selection`](capabilities/animation/toolset/controlrig/sequencer/control/rig/add-layer-from-selection.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.bake_space`](capabilities/animation/toolset/controlrig/sequencer/control/rig/bake/space.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.bake_to_control_rig`](capabilities/animation/toolset/controlrig/sequencer/control/rig/bake/to-control-rig.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.blend_values_on_selected`](capabilities/animation/toolset/controlrig/sequencer/control/rig/blend-values-on-selected.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.clear_selection`](capabilities/animation/toolset/controlrig/sequencer/control/rig/clear-selection.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.collapse_anim_layers`](capabilities/animation/toolset/controlrig/sequencer/control/rig/collapse-anim-layers.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.delete_anim_layer`](capabilities/animation/toolset/controlrig/sequencer/control/rig/delete/anim-layer.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.delete_space`](capabilities/animation/toolset/controlrig/sequencer/control/rig/delete/space.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.duplicate_anim_layer`](capabilities/animation/toolset/controlrig/sequencer/control/rig/duplicate-anim-layer.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.export_fbx_from_rig`](capabilities/animation/toolset/controlrig/sequencer/control/rig/export-fbx-from-rig.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.find_or_create_track`](capabilities/animation/toolset/controlrig/sequencer/control/rig/find-or-create-track.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.frame_selection`](capabilities/animation/toolset/controlrig/sequencer/control/rig/frame-selection.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_actor_transform_at_frame`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/actor-transform-at-frame.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_anim_layers`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/anim/layers.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_anim_mode_gizmo_scale`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/anim/mode/gizmo-scale.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_anim_mode_hide_manips`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/anim/mode/hide-manips.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_anim_mode_hierarchy`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/anim/mode/hierarchy.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_anim_mode_local_spaces`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/anim/mode/local-spaces.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_anim_mode_nulls`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/anim/mode/nulls.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_anim_mode_only_rig_sel`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/anim/mode/only-rig-sel.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_bool`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/bool.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_control_rigs`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/control-rigs.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_controls_info`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/controls/info.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_controls_mask`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/controls/mask.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_euler_transform`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/euler-transform.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_float`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/float.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_int`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/int.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_position`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/position.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_priority_order`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/priority-order.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_rotator`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/rotator.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_scale`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/scale.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_selected_controls`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/selected-controls.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_transform`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/transform.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_vector2d`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/vector-2-d.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.get_world_transform`](capabilities/animation/toolset/controlrig/sequencer/control/rig/get/world-transform.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.hide_all_controls`](capabilities/animation/toolset/controlrig/sequencer/control/rig/hide-all-controls.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.import_fbx_to_rig`](capabilities/animation/toolset/controlrig/sequencer/control/rig/import-fbx-to-rig.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.is_fk_control_rig`](capabilities/animation/toolset/controlrig/sequencer/control/rig/is/fk-control-rig.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.is_layered_control_rig`](capabilities/animation/toolset/controlrig/sequencer/control/rig/is/layered-control-rig.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.key_controls`](capabilities/animation/toolset/controlrig/sequencer/control/rig/key/controls.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.key_controls_at_frames`](capabilities/animation/toolset/controlrig/sequencer/control/rig/key/controls-at-frames.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.load_anim_into_rig`](capabilities/animation/toolset/controlrig/sequencer/control/rig/load-anim-into-rig.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.merge_anim_layers`](capabilities/animation/toolset/controlrig/sequencer/control/rig/merge-anim-layers.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.mirror_selected_controls`](capabilities/animation/toolset/controlrig/sequencer/control/rig/mirror-selected-controls.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.move_space`](capabilities/animation/toolset/controlrig/sequencer/control/rig/move-space.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.reorder_anim_layers`](capabilities/animation/toolset/controlrig/sequencer/control/rig/reorder-anim-layers.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.select_control`](capabilities/animation/toolset/controlrig/sequencer/control/rig/select/control.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.select_mirrored_controls`](capabilities/animation/toolset/controlrig/sequencer/control/rig/select/mirrored-controls.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_anim_mode_gizmo_scale`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/anim/mode/gizmo-scale.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_anim_mode_hide_manips`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/anim/mode/hide-manips.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_anim_mode_hierarchy`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/anim/mode/hierarchy.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_anim_mode_local_spaces`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/anim/mode/local-spaces.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_anim_mode_nulls`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/anim/mode/nulls.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_anim_mode_only_rig_sel`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/anim/mode/only-rig-sel.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_bool`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/bool.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_controls_mask`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/controls-mask.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_euler_transform`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/euler-transform.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_float`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/float.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_int`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/int.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_layered_mode`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/layered-mode.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_position`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/position.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_priority_order`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/priority-order.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_rotator`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/rotator.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_scale`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/scale.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_space`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/space.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_transform`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/transform.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_vector2d`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/vector-2-d.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.set_world_transform`](capabilities/animation/toolset/controlrig/sequencer/control/rig/set/world-transform.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.show_all_controls`](capabilities/animation/toolset/controlrig/sequencer/control/rig/show-all-controls.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.snap_control_rig`](capabilities/animation/toolset/controlrig/sequencer/control/rig/snap-control-rig.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.tween_control_rig`](capabilities/animation/toolset/controlrig/sequencer/control/rig/tween-control-rig.md)
- [`animation_toolset.toolsets.controlrig_sequencer.SequencerControlRigTools.zero_transforms`](capabilities/animation/toolset/controlrig/sequencer/control/rig/zero-transforms.md)

#### `animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools`

Capabilities: **8**

- [`animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.change_actor_template_class`](capabilities/animation/toolset/custom/bindings/sequencer/binding/change-actor-template-class.md)
- [`animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.convert_to_custom_binding`](capabilities/animation/toolset/custom/bindings/sequencer/binding/convert/to/custom-binding.md)
- [`animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.convert_to_possessable`](capabilities/animation/toolset/custom/bindings/sequencer/binding/convert/to/possessable.md)
- [`animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.convert_to_spawnable`](capabilities/animation/toolset/custom/bindings/sequencer/binding/convert/to/spawnable.md)
- [`animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.get_custom_binding_objects`](capabilities/animation/toolset/custom/bindings/sequencer/binding/get/custom/binding/objects.md)
- [`animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.get_custom_binding_type`](capabilities/animation/toolset/custom/bindings/sequencer/binding/get/custom/binding/type.md)
- [`animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.get_custom_bindings_of_type`](capabilities/animation/toolset/custom/bindings/sequencer/binding/get/custom/bindings-of-type.md)
- [`animation_toolset.toolsets.custom_bindings.SequencerCustomBindingTools.save_default_spawnable_state`](capabilities/animation/toolset/custom/bindings/sequencer/binding/save-default-spawnable-state.md)

#### `animation_toolset.toolsets.import_export.SequencerImportExportTools`

Capabilities: **6**

- [`animation_toolset.toolsets.import_export.SequencerImportExportTools.export_anim_sequence`](capabilities/animation/toolset/import/export/sequencer/export/anim-sequence.md)
- [`animation_toolset.toolsets.import_export.SequencerImportExportTools.export_fbx`](capabilities/animation/toolset/import/export/sequencer/export/fbx.md)
- [`animation_toolset.toolsets.import_export.SequencerImportExportTools.get_linked_anim_sequences`](capabilities/animation/toolset/import/export/sequencer/get/linked/anim-sequences.md)
- [`animation_toolset.toolsets.import_export.SequencerImportExportTools.get_linked_level_sequence`](capabilities/animation/toolset/import/export/sequencer/get/linked/level-sequence.md)
- [`animation_toolset.toolsets.import_export.SequencerImportExportTools.import_fbx`](capabilities/animation/toolset/import/export/sequencer/import-fbx.md)
- [`animation_toolset.toolsets.import_export.SequencerImportExportTools.link_anim_sequence`](capabilities/animation/toolset/import/export/sequencer/link-anim-sequence.md)

#### `animation_toolset.toolsets.keyframing.SequencerKeyframingTools`

Capabilities: **22**

- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.add_key_bool`](capabilities/animation/toolset/keyframing/sequencer/add/key/bool.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.add_key_float`](capabilities/animation/toolset/keyframing/sequencer/add/key/float.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.add_key_integer`](capabilities/animation/toolset/keyframing/sequencer/add/key/integer.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.add_key_string`](capabilities/animation/toolset/keyframing/sequencer/add/key/string.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.bake_channel_keys`](capabilities/animation/toolset/keyframing/sequencer/bake-channel-keys.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.close_curve_editor`](capabilities/animation/toolset/keyframing/sequencer/close-curve-editor.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.curve_editor_empty_selection`](capabilities/animation/toolset/keyframing/sequencer/curve/editor/empty-selection.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.curve_editor_select_keys`](capabilities/animation/toolset/keyframing/sequencer/curve/editor/select-keys.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.get_channel_names`](capabilities/animation/toolset/keyframing/sequencer/get/channel-names.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.get_curve_editor_selected_keys`](capabilities/animation/toolset/keyframing/sequencer/get/curve-editor-selected-keys.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.get_default_value`](capabilities/animation/toolset/keyframing/sequencer/get/default-value.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.get_keys`](capabilities/animation/toolset/keyframing/sequencer/get/keys.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.get_keys_by_index`](capabilities/animation/toolset/keyframing/sequencer/get/keys-by-index.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.get_selected_channels`](capabilities/animation/toolset/keyframing/sequencer/get/selected/channels.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.get_selected_key_channels`](capabilities/animation/toolset/keyframing/sequencer/get/selected/key-channels.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.is_curve_editor_open`](capabilities/animation/toolset/keyframing/sequencer/is/curve/editor-open.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.is_curve_shown`](capabilities/animation/toolset/keyframing/sequencer/is/curve/shown.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.open_curve_editor`](capabilities/animation/toolset/keyframing/sequencer/open-curve-editor.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.remove_key_at_frame`](capabilities/animation/toolset/keyframing/sequencer/remove-key-at-frame.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.select_channels`](capabilities/animation/toolset/keyframing/sequencer/select-channels.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.set_default_value`](capabilities/animation/toolset/keyframing/sequencer/set-default-value.md)
- [`animation_toolset.toolsets.keyframing.SequencerKeyframingTools.show_curve`](capabilities/animation/toolset/keyframing/sequencer/show-curve.md)

#### `animation_toolset.toolsets.outliner.SequencerOutlinerTools`

Capabilities: **18**

- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_deactivated_nodes`](capabilities/animation/toolset/outliner/sequencer/get/deactivated-nodes.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_locked_nodes`](capabilities/animation/toolset/outliner/sequencer/get/locked-nodes.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_muted_nodes`](capabilities/animation/toolset/outliner/sequencer/get/muted-nodes.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_node_label`](capabilities/animation/toolset/outliner/sequencer/get/node-label.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_outliner_children`](capabilities/animation/toolset/outliner/sequencer/get/outliner/children.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_outliner_selection`](capabilities/animation/toolset/outliner/sequencer/get/outliner/selection.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_outliner_tree`](capabilities/animation/toolset/outliner/sequencer/get/outliner/tree.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_pinned_nodes`](capabilities/animation/toolset/outliner/sequencer/get/pinned-nodes.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_sections_for_nodes`](capabilities/animation/toolset/outliner/sequencer/get/sections-for-nodes.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.get_soloed_nodes`](capabilities/animation/toolset/outliner/sequencer/get/soloed-nodes.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.is_node_expanded`](capabilities/animation/toolset/outliner/sequencer/is-node-expanded.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.set_node_deactivated`](capabilities/animation/toolset/outliner/sequencer/set/node/deactivated.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.set_node_expanded`](capabilities/animation/toolset/outliner/sequencer/set/node/expanded.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.set_node_locked`](capabilities/animation/toolset/outliner/sequencer/set/node/locked.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.set_node_muted`](capabilities/animation/toolset/outliner/sequencer/set/node/muted.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.set_node_pinned`](capabilities/animation/toolset/outliner/sequencer/set/node/pinned.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.set_node_solo`](capabilities/animation/toolset/outliner/sequencer/set/node/solo.md)
- [`animation_toolset.toolsets.outliner.SequencerOutlinerTools.set_outliner_selection`](capabilities/animation/toolset/outliner/sequencer/set/outliner-selection.md)

#### `animation_toolset.toolsets.sequencer.SequencerTools`

Capabilities: **140**

- [`animation_toolset.toolsets.sequencer.SequencerTools.add_actors`](capabilities/animation/toolset/sequencer/add/actors.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_actors_by_name`](capabilities/animation/toolset/sequencer/add/actors/by-name.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_actors_to_binding`](capabilities/animation/toolset/sequencer/add/actors/to-binding.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_binding_to_folder`](capabilities/animation/toolset/sequencer/add/binding-to-folder.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_event_repeater_section`](capabilities/animation/toolset/sequencer/add/event/repeater-section.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_event_trigger_section`](capabilities/animation/toolset/sequencer/add/event/trigger-section.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_marked_frame`](capabilities/animation/toolset/sequencer/add/marked-frame.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_root_folder`](capabilities/animation/toolset/sequencer/add/root-folder.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_section`](capabilities/animation/toolset/sequencer/add/section.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_spawnable_from_class`](capabilities/animation/toolset/sequencer/add/spawnable/from/class.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_spawnable_from_instance`](capabilities/animation/toolset/sequencer/add/spawnable/from/instance.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_track_to_binding`](capabilities/animation/toolset/sequencer/add/track/to/binding.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_track_to_folder`](capabilities/animation/toolset/sequencer/add/track/to/folder.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.add_track_to_sequence`](capabilities/animation/toolset/sequencer/add/track/to/sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.bake_transform`](capabilities/animation/toolset/sequencer/bake-transform.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.close_sequence`](capabilities/animation/toolset/sequencer/close-sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.copy_bindings`](capabilities/animation/toolset/sequencer/copy/bindings.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.copy_folders`](capabilities/animation/toolset/sequencer/copy/folders.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.copy_sections`](capabilities/animation/toolset/sequencer/copy/sections.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.copy_tracks`](capabilities/animation/toolset/sequencer/copy/tracks.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.create_camera`](capabilities/animation/toolset/sequencer/create/camera.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.create_level_sequence`](capabilities/animation/toolset/sequencer/create/level-sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.delete_all_marked_frames`](capabilities/animation/toolset/sequencer/delete/all-marked-frames.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.delete_marked_frame`](capabilities/animation/toolset/sequencer/delete/marked-frame.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.empty_selection`](capabilities/animation/toolset/sequencer/empty-selection.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.find_binding_by_name`](capabilities/animation/toolset/sequencer/find/binding/by/name.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.find_binding_by_tag`](capabilities/animation/toolset/sequencer/find/binding/by/tag.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.find_bindings_by_tag`](capabilities/animation/toolset/sequencer/find/bindings-by-tag.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.find_marked_frame_by_label`](capabilities/animation/toolset/sequencer/find/marked-frame-by-label.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.find_tracks_by_type`](capabilities/animation/toolset/sequencer/find/tracks-by-type.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.fix_actor_references`](capabilities/animation/toolset/sequencer/fix-actor-references.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.focus_parent_sequence`](capabilities/animation/toolset/sequencer/focus/parent-sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.focus_sub_sequence`](capabilities/animation/toolset/sequencer/focus/sub-sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.force_evaluate`](capabilities/animation/toolset/sequencer/force-evaluate.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_all_binding_tags`](capabilities/animation/toolset/sequencer/get/all-binding-tags.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_binding_id`](capabilities/animation/toolset/sequencer/get/binding/id.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_binding_name`](capabilities/animation/toolset/sequencer/get/binding/name.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_binding_tags`](capabilities/animation/toolset/sequencer/get/binding/tags.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_bindings`](capabilities/animation/toolset/sequencer/get/bindings.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_bound_objects`](capabilities/animation/toolset/sequencer/get/bound-objects.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_child_possessables`](capabilities/animation/toolset/sequencer/get/child-possessables.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_clock_source`](capabilities/animation/toolset/sequencer/get/clock-source.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_current_sequence`](capabilities/animation/toolset/sequencer/get/current-sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_display_rate`](capabilities/animation/toolset/sequencer/get/display-rate.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_evaluation_type`](capabilities/animation/toolset/sequencer/get/evaluation-type.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_focused_sequence`](capabilities/animation/toolset/sequencer/get/focused-sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_folder_contents`](capabilities/animation/toolset/sequencer/get/folder-contents.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_loop_mode`](capabilities/animation/toolset/sequencer/get/loop-mode.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_marked_frames`](capabilities/animation/toolset/sequencer/get/marked-frames.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_playback_range`](capabilities/animation/toolset/sequencer/get/playback/range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_playback_speed`](capabilities/animation/toolset/sequencer/get/playback/speed.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_playhead_frame`](capabilities/animation/toolset/sequencer/get/playhead-frame.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_root_folders`](capabilities/animation/toolset/sequencer/get/root-folders.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_section_blend_type`](capabilities/animation/toolset/sequencer/get/section/blend-type.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_section_completion_mode`](capabilities/animation/toolset/sequencer/get/section/completion-mode.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_section_ease_in`](capabilities/animation/toolset/sequencer/get/section/ease/in.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_section_ease_out`](capabilities/animation/toolset/sequencer/get/section/ease/out.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_section_post_roll_frames`](capabilities/animation/toolset/sequencer/get/section/post-roll-frames.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_section_pre_roll_frames`](capabilities/animation/toolset/sequencer/get/section/pre-roll-frames.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_section_properties`](capabilities/animation/toolset/sequencer/get/section/properties.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_section_range`](capabilities/animation/toolset/sequencer/get/section/range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_section_to_key`](capabilities/animation/toolset/sequencer/get/section/to-key.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_sections`](capabilities/animation/toolset/sequencer/get/sections.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_selected_bindings`](capabilities/animation/toolset/sequencer/get/selected/bindings.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_selected_folders`](capabilities/animation/toolset/sequencer/get/selected/folders.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_selected_sections`](capabilities/animation/toolset/sequencer/get/selected/sections.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_selected_tracks`](capabilities/animation/toolset/sequencer/get/selected/tracks.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_selection_range`](capabilities/animation/toolset/sequencer/get/selection-range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_sequence_lock_state`](capabilities/animation/toolset/sequencer/get/sequence-lock-state.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_sub_sequence_hierarchy`](capabilities/animation/toolset/sequencer/get/sub-sequence-hierarchy.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_tick_resolution`](capabilities/animation/toolset/sequencer/get/tick-resolution.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_track_display_name`](capabilities/animation/toolset/sequencer/get/track/display-name.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_track_filter_names`](capabilities/animation/toolset/sequencer/get/track/filter-names.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_tracks_on_binding`](capabilities/animation/toolset/sequencer/get/tracks/on/binding.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_tracks_on_sequence`](capabilities/animation/toolset/sequencer/get/tracks/on/sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_view_range`](capabilities/animation/toolset/sequencer/get/view-range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.get_work_range`](capabilities/animation/toolset/sequencer/get/work-range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.has_section_end_frame`](capabilities/animation/toolset/sequencer/has/section/end-frame.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.has_section_start_frame`](capabilities/animation/toolset/sequencer/has/section/start-frame.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.is_camera_cut_locked`](capabilities/animation/toolset/sequencer/is/camera-cut-locked.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.is_playback_range_locked`](capabilities/animation/toolset/sequencer/is/playback-range-locked.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.is_playing`](capabilities/animation/toolset/sequencer/is/playing.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.is_sequence_locked`](capabilities/animation/toolset/sequencer/is/sequence-locked.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.is_track_filter_active`](capabilities/animation/toolset/sequencer/is/track-filter-active.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.open_sequence`](capabilities/animation/toolset/sequencer/open-sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.paste_bindings`](capabilities/animation/toolset/sequencer/paste/bindings.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.paste_folders`](capabilities/animation/toolset/sequencer/paste/folders.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.paste_sections`](capabilities/animation/toolset/sequencer/paste/sections.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.paste_tracks`](capabilities/animation/toolset/sequencer/paste/tracks.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.pause`](capabilities/animation/toolset/sequencer/pause.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.play`](capabilities/animation/toolset/sequencer/play.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.play_to`](capabilities/animation/toolset/sequencer/play-to.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.rebind_component`](capabilities/animation/toolset/sequencer/rebind-component.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.refresh_sequence`](capabilities/animation/toolset/sequencer/refresh-sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.remove_actors_from_binding`](capabilities/animation/toolset/sequencer/remove/actors-from-binding.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.remove_all_bindings`](capabilities/animation/toolset/sequencer/remove/all-bindings.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.remove_binding`](capabilities/animation/toolset/sequencer/remove/binding.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.remove_binding_tag`](capabilities/animation/toolset/sequencer/remove/binding-tag.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.remove_invalid_bindings`](capabilities/animation/toolset/sequencer/remove/invalid-bindings.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.remove_root_folder`](capabilities/animation/toolset/sequencer/remove/root-folder.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.remove_section`](capabilities/animation/toolset/sequencer/remove/section.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.remove_track`](capabilities/animation/toolset/sequencer/remove/track.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.remove_track_from_sequence`](capabilities/animation/toolset/sequencer/remove/track-from-sequence.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.replace_binding_with_actors`](capabilities/animation/toolset/sequencer/replace-binding-with-actors.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.select_bindings`](capabilities/animation/toolset/sequencer/select/bindings.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.select_folders`](capabilities/animation/toolset/sequencer/select/folders.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.select_sections`](capabilities/animation/toolset/sequencer/select/sections.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.select_tracks`](capabilities/animation/toolset/sequencer/select/tracks.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_binding_name`](capabilities/animation/toolset/sequencer/set/binding-name.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_byte_track_enum`](capabilities/animation/toolset/sequencer/set/byte-track-enum.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_camera_cut_binding`](capabilities/animation/toolset/sequencer/set/camera/cut-binding.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_camera_lock`](capabilities/animation/toolset/sequencer/set/camera/lock.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_clock_source`](capabilities/animation/toolset/sequencer/set/clock-source.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_display_rate`](capabilities/animation/toolset/sequencer/set/display-rate.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_evaluation_type`](capabilities/animation/toolset/sequencer/set/evaluation-type.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_loop_mode`](capabilities/animation/toolset/sequencer/set/loop-mode.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_playback_range`](capabilities/animation/toolset/sequencer/set/playback/range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_playback_range_locked`](capabilities/animation/toolset/sequencer/set/playback/range-locked.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_playback_speed`](capabilities/animation/toolset/sequencer/set/playback/speed.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_playhead_frame`](capabilities/animation/toolset/sequencer/set/playhead-frame.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_property_name_and_path`](capabilities/animation/toolset/sequencer/set/property-name-and-path.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_animation`](capabilities/animation/toolset/sequencer/set/section/animation.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_blend_type`](capabilities/animation/toolset/sequencer/set/section/blend-type.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_completion_mode`](capabilities/animation/toolset/sequencer/set/section/completion-mode.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_ease_in`](capabilities/animation/toolset/sequencer/set/section/ease/in.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_ease_out`](capabilities/animation/toolset/sequencer/set/section/ease/out.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_end_bounded`](capabilities/animation/toolset/sequencer/set/section/end-bounded.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_post_roll_frames`](capabilities/animation/toolset/sequencer/set/section/post-roll-frames.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_pre_roll_frames`](capabilities/animation/toolset/sequencer/set/section/pre-roll-frames.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_range`](capabilities/animation/toolset/sequencer/set/section/range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_section_start_bounded`](capabilities/animation/toolset/sequencer/set/section/start-bounded.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_selection_range`](capabilities/animation/toolset/sequencer/set/selection-range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_sequence_locked`](capabilities/animation/toolset/sequencer/set/sequence-locked.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_tick_resolution`](capabilities/animation/toolset/sequencer/set/tick-resolution.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_track_display_name`](capabilities/animation/toolset/sequencer/set/track/display-name.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_track_filter_active`](capabilities/animation/toolset/sequencer/set/track/filter-active.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_view_range`](capabilities/animation/toolset/sequencer/set/view-range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.set_work_range`](capabilities/animation/toolset/sequencer/set/work-range.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.tag_binding`](capabilities/animation/toolset/sequencer/tag-binding.md)
- [`animation_toolset.toolsets.sequencer.SequencerTools.untag_binding`](capabilities/animation/toolset/sequencer/untag-binding.md)

### Gameplay and AI

Game Features, tags, abilities, AI graphs, and world conditions.

10 toolsets; 59 capabilities.

#### `DataRegistryToolset.DataRegistryTools`

Capabilities: **7**

- [`DataRegistryToolset.DataRegistryTools.GetItems`](capabilities/data/registry/toolset/get/items.md)
- [`DataRegistryToolset.DataRegistryTools.GetRegistryInfo`](capabilities/data/registry/toolset/get/registry-info.md)
- [`DataRegistryToolset.DataRegistryTools.GetSchema`](capabilities/data/registry/toolset/get/schema.md)
- [`DataRegistryToolset.DataRegistryTools.ListDataSources`](capabilities/data/registry/toolset/list/data-sources.md)
- [`DataRegistryToolset.DataRegistryTools.ListItems`](capabilities/data/registry/toolset/list/items.md)
- [`DataRegistryToolset.DataRegistryTools.ListRegistries`](capabilities/data/registry/toolset/list/registries.md)
- [`DataRegistryToolset.DataRegistryTools.ListRuntimeSources`](capabilities/data/registry/toolset/list/runtime-sources.md)

#### `GASToolsets.AbilitySystemInspectorToolset`

Capabilities: **4**

- [`GASToolsets.AbilitySystemInspectorToolset.GetActiveEffects`](capabilities/gas/toolset/ability/system/inspector/get/active/effects.md)
- [`GASToolsets.AbilitySystemInspectorToolset.GetActiveTags`](capabilities/gas/toolset/ability/system/inspector/get/active/tags.md)
- [`GASToolsets.AbilitySystemInspectorToolset.GetAttributeValues`](capabilities/gas/toolset/ability/system/inspector/get/attribute-values.md)
- [`GASToolsets.AbilitySystemInspectorToolset.GetGrantedAbilities`](capabilities/gas/toolset/ability/system/inspector/get/granted-abilities.md)

#### `GASToolsets.AttributeSetToolset`

Capabilities: **2**

- [`GASToolsets.AttributeSetToolset.FindAttributeSetClasses`](capabilities/gas/toolset/attribute/set/find-attribute-set-classes.md)
- [`GASToolsets.AttributeSetToolset.ListAttributes`](capabilities/gas/toolset/attribute/set/list-attributes.md)

#### `GASToolsets.GameplayCueToolset`

Capabilities: **8**

- [`GASToolsets.GameplayCueToolset.AddCueTag`](capabilities/gas/toolset/gameplay/cue/add-cue-tag.md)
- [`GASToolsets.GameplayCueToolset.CreateCueNotifyAsset`](capabilities/gas/toolset/gameplay/cue/create-cue-notify-asset.md)
- [`GASToolsets.GameplayCueToolset.ExecuteCueOnSelectedActor`](capabilities/gas/toolset/gameplay/cue/execute-cue-on-selected-actor.md)
- [`GASToolsets.GameplayCueToolset.FindCueNotifyAssets`](capabilities/gas/toolset/gameplay/cue/find/cue/notify-assets.md)
- [`GASToolsets.GameplayCueToolset.FindCueTagsWithoutNotifies`](capabilities/gas/toolset/gameplay/cue/find/cue/tags-without-notifies.md)
- [`GASToolsets.GameplayCueToolset.GetCueInfo`](capabilities/gas/toolset/gameplay/cue/get-cue-info.md)
- [`GASToolsets.GameplayCueToolset.ListCues`](capabilities/gas/toolset/gameplay/cue/list-cues.md)
- [`GASToolsets.GameplayCueToolset.RemoveCueTag`](capabilities/gas/toolset/gameplay/cue/remove-cue-tag.md)

#### `GameFeaturesToolset.GameFeaturesToolset`

Capabilities: **7**

- [`GameFeaturesToolset.GameFeaturesToolset.GetGameFeatureState`](capabilities/game/features/toolset/get-game-feature-state.md)
- [`GameFeaturesToolset.GameFeaturesToolset.IsGameFeatureActive`](capabilities/game/features/toolset/is/game/feature/active.md)
- [`GameFeaturesToolset.GameFeaturesToolset.IsGameFeaturePlugin`](capabilities/game/features/toolset/is/game/feature/plugin.md)
- [`GameFeaturesToolset.GameFeaturesToolset.ListDiscoveredGameFeaturePlugins`](capabilities/game/features/toolset/list/discovered-game-feature-plugins.md)
- [`GameFeaturesToolset.GameFeaturesToolset.ListEnabledGameFeaturePlugins`](capabilities/game/features/toolset/list/enabled-game-feature-plugins.md)
- [`GameFeaturesToolset.GameFeaturesToolset.RequestActivateGameFeature`](capabilities/game/features/toolset/request/activate-game-feature.md)
- [`GameFeaturesToolset.GameFeaturesToolset.RequestDeactivateGameFeature`](capabilities/game/features/toolset/request/deactivate-game-feature.md)

#### `GameplayTagsToolset.GameplayTagsToolset`

Capabilities: **6**

- [`GameplayTagsToolset.GameplayTagsToolset.AddTag`](capabilities/gameplay/tags/toolset/add-tag.md)
- [`GameplayTagsToolset.GameplayTagsToolset.FindReferencersByTag`](capabilities/gameplay/tags/toolset/find-referencers-by-tag.md)
- [`GameplayTagsToolset.GameplayTagsToolset.GetTagInfo`](capabilities/gameplay/tags/toolset/get-tag-info.md)
- [`GameplayTagsToolset.GameplayTagsToolset.ListTags`](capabilities/gameplay/tags/toolset/list-tags.md)
- [`GameplayTagsToolset.GameplayTagsToolset.RemoveTag`](capabilities/gameplay/tags/toolset/remove-tag.md)
- [`GameplayTagsToolset.GameplayTagsToolset.RenameTag`](capabilities/gameplay/tags/toolset/rename-tag.md)

#### `WorldConditionsToolset.WorldConditionTools`

Capabilities: **2**

- [`WorldConditionsToolset.WorldConditionTools.GetConditionDescription`](capabilities/world/conditions/toolset/condition/get/condition-description.md)
- [`WorldConditionsToolset.WorldConditionTools.GetQueryDescription`](capabilities/world/conditions/toolset/condition/get/query-description.md)

#### `aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools`

Capabilities: **7**

- [`aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_blackboard`](capabilities/aimodule/toolset/behavior/tree/get/blackboard.md)
- [`aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_children`](capabilities/aimodule/toolset/behavior/tree/get/children.md)
- [`aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_node_depth`](capabilities/aimodule/toolset/behavior/tree/get/node/depth.md)
- [`aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_node_depths`](capabilities/aimodule/toolset/behavior/tree/get/node/depths.md)
- [`aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_root_decorators`](capabilities/aimodule/toolset/behavior/tree/get/root-decorators.md)
- [`aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.get_subtree`](capabilities/aimodule/toolset/behavior/tree/get/subtree.md)
- [`aimodule_toolset.toolsets.behavior_tree.BehaviorTreeTools.list_nodes`](capabilities/aimodule/toolset/behavior/tree/list-nodes.md)

#### `conversation_toolset.toolsets.conversation.ConversationTools`

Capabilities: **7**

- [`conversation_toolset.toolsets.conversation.ConversationTools.get_all_nodes`](capabilities/conversation/toolset/get/all-nodes.md)
- [`conversation_toolset.toolsets.conversation.ConversationTools.get_node_by_guid`](capabilities/conversation/toolset/get/node/by-guid.md)
- [`conversation_toolset.toolsets.conversation.ConversationTools.get_node_connections`](capabilities/conversation/toolset/get/node/connections.md)
- [`conversation_toolset.toolsets.conversation.ConversationTools.get_node_guids`](capabilities/conversation/toolset/get/node/guids.md)
- [`conversation_toolset.toolsets.conversation.ConversationTools.get_sub_nodes`](capabilities/conversation/toolset/get/sub-nodes.md)
- [`conversation_toolset.toolsets.conversation.ConversationTools.list_entry_points`](capabilities/conversation/toolset/list/entry-points.md)
- [`conversation_toolset.toolsets.conversation.ConversationTools.list_speakers`](capabilities/conversation/toolset/list/speakers.md)

#### `state_tree_toolset.toolsets.state_tree.StateTreeTools`

Capabilities: **9**

- [`state_tree_toolset.toolsets.state_tree.StateTreeTools.get_children`](capabilities/state/tree/toolset/get/children.md)
- [`state_tree_toolset.toolsets.state_tree.StateTreeTools.get_editor_data`](capabilities/state/tree/toolset/get/editor-data.md)
- [`state_tree_toolset.toolsets.state_tree.StateTreeTools.get_enter_conditions`](capabilities/state/tree/toolset/get/enter-conditions.md)
- [`state_tree_toolset.toolsets.state_tree.StateTreeTools.get_evaluators`](capabilities/state/tree/toolset/get/evaluators.md)
- [`state_tree_toolset.toolsets.state_tree.StateTreeTools.get_global_tasks`](capabilities/state/tree/toolset/get/global-tasks.md)
- [`state_tree_toolset.toolsets.state_tree.StateTreeTools.get_node_description`](capabilities/state/tree/toolset/get/node-description.md)
- [`state_tree_toolset.toolsets.state_tree.StateTreeTools.get_root_states`](capabilities/state/tree/toolset/get/root-states.md)
- [`state_tree_toolset.toolsets.state_tree.StateTreeTools.get_tasks`](capabilities/state/tree/toolset/get/tasks.md)
- [`state_tree_toolset.toolsets.state_tree.StateTreeTools.get_transitions`](capabilities/state/tree/toolset/get/transitions.md)

### Effects, physics, and procedural

Niagara, PCG, Dataflow, and Physics Asset workflows.

9 toolsets; 126 capabilities.

#### `DataflowAgent.DataflowAgentToolset`

Capabilities: **22**

- [`DataflowAgent.DataflowAgentToolset.AddCommentBox`](capabilities/dataflow/agent/toolset/add/comment-box.md)
- [`DataflowAgent.DataflowAgentToolset.AddNode`](capabilities/dataflow/agent/toolset/add/node.md)
- [`DataflowAgent.DataflowAgentToolset.AddVariable`](capabilities/dataflow/agent/toolset/add/variable.md)
- [`DataflowAgent.DataflowAgentToolset.AssignDataflowTemplate`](capabilities/dataflow/agent/toolset/assign-dataflow-template.md)
- [`DataflowAgent.DataflowAgentToolset.ConnectNodePins`](capabilities/dataflow/agent/toolset/connect-node-pins.md)
- [`DataflowAgent.DataflowAgentToolset.CreateDataflowCompatibleAsset`](capabilities/dataflow/agent/toolset/create/dataflow/compatible/asset.md)
- [`DataflowAgent.DataflowAgentToolset.CreateDataflowCompatibleAssetFromTemplate`](capabilities/dataflow/agent/toolset/create/dataflow/compatible/asset-from-template.md)
- [`DataflowAgent.DataflowAgentToolset.CreateGraph`](capabilities/dataflow/agent/toolset/create/graph.md)
- [`DataflowAgent.DataflowAgentToolset.DisconnectNodePins`](capabilities/dataflow/agent/toolset/disconnect-node-pins.md)
- [`DataflowAgent.DataflowAgentToolset.GetGraphStructure`](capabilities/dataflow/agent/toolset/get/graph-structure.md)
- [`DataflowAgent.DataflowAgentToolset.GetNodeInfo`](capabilities/dataflow/agent/toolset/get/node/info.md)
- [`DataflowAgent.DataflowAgentToolset.GetNodeTypeSchema`](capabilities/dataflow/agent/toolset/get/node/type-schema.md)
- [`DataflowAgent.DataflowAgentToolset.ListDataflowCompatibleAssetTypes`](capabilities/dataflow/agent/toolset/list/dataflow/compatible-asset-types.md)
- [`DataflowAgent.DataflowAgentToolset.ListDataflowTemplatesForAssetClass`](capabilities/dataflow/agent/toolset/list/dataflow/templates-for-asset-class.md)
- [`DataflowAgent.DataflowAgentToolset.ListNodeTypes`](capabilities/dataflow/agent/toolset/list/node-types.md)
- [`DataflowAgent.DataflowAgentToolset.ListVariables`](capabilities/dataflow/agent/toolset/list/variables.md)
- [`DataflowAgent.DataflowAgentToolset.RemoveCommentBox`](capabilities/dataflow/agent/toolset/remove/comment-box.md)
- [`DataflowAgent.DataflowAgentToolset.RemoveNode`](capabilities/dataflow/agent/toolset/remove/node.md)
- [`DataflowAgent.DataflowAgentToolset.RemoveVariable`](capabilities/dataflow/agent/toolset/remove/variable.md)
- [`DataflowAgent.DataflowAgentToolset.RepositionNode`](capabilities/dataflow/agent/toolset/reposition-node.md)
- [`DataflowAgent.DataflowAgentToolset.SetVariable`](capabilities/dataflow/agent/toolset/set-variable.md)
- [`DataflowAgent.DataflowAgentToolset.UpdateNode`](capabilities/dataflow/agent/toolset/update-node.md)

#### `NiagaraToolsets.NiagaraToolset_Assets`

Capabilities: **3**

- [`NiagaraToolsets.NiagaraToolset_Assets.FindNiagaraScripts`](capabilities/niagara/toolset/assets/find-niagara-scripts.md)
- [`NiagaraToolsets.NiagaraToolset_Assets.GetAssetDiscoveryInfo`](capabilities/niagara/toolset/assets/get/asset-discovery-info.md)
- [`NiagaraToolsets.NiagaraToolset_Assets.GetNiagaraScriptDigest`](capabilities/niagara/toolset/assets/get/niagara-script-digest.md)

#### `NiagaraToolsets.NiagaraToolset_Blueprint`

Capabilities: **2**

- [`NiagaraToolsets.NiagaraToolset_Blueprint.ConstructNiagaraBPWrapperFromComponent`](capabilities/niagara/toolset/blueprint/construct/niagara/bp/wrapper/from/component.md)
- [`NiagaraToolsets.NiagaraToolset_Blueprint.ConstructNiagaraBPWrapperFromSystem`](capabilities/niagara/toolset/blueprint/construct/niagara/bp/wrapper/from/system.md)

#### `NiagaraToolsets.NiagaraToolset_Component`

Capabilities: **4**

- [`NiagaraToolsets.NiagaraToolset_Component.GetUserVariables`](capabilities/niagara/toolset/component/get/user-variables.md)
- [`NiagaraToolsets.NiagaraToolset_Component.GetVariable`](capabilities/niagara/toolset/component/get/variable.md)
- [`NiagaraToolsets.NiagaraToolset_Component.SetSystem`](capabilities/niagara/toolset/component/set/system.md)
- [`NiagaraToolsets.NiagaraToolset_Component.SetVariable`](capabilities/niagara/toolset/component/set/variable.md)

#### `NiagaraToolsets.NiagaraToolset_Info`

Capabilities: **1**

- [`NiagaraToolsets.NiagaraToolset_Info.UEnum_Info`](capabilities/niagara/toolset/info/u-enum-info.md)

#### `NiagaraToolsets.NiagaraToolset_System`

Capabilities: **46**

- [`NiagaraToolsets.NiagaraToolset_System.AddEmitter`](capabilities/niagara/toolset/system/add/emitter.md)
- [`NiagaraToolsets.NiagaraToolset_System.AddModule`](capabilities/niagara/toolset/system/add/module.md)
- [`NiagaraToolsets.NiagaraToolset_System.AddRenderer`](capabilities/niagara/toolset/system/add/renderer.md)
- [`NiagaraToolsets.NiagaraToolset_System.AddSetParameterEntry`](capabilities/niagara/toolset/system/add/set/parameter-entry.md)
- [`NiagaraToolsets.NiagaraToolset_System.AddSetParametersModule`](capabilities/niagara/toolset/system/add/set/parameters-module.md)
- [`NiagaraToolsets.NiagaraToolset_System.AddUserVariables`](capabilities/niagara/toolset/system/add/user-variables.md)
- [`NiagaraToolsets.NiagaraToolset_System.ApplyStackIssueFix`](capabilities/niagara/toolset/system/apply-stack-issue-fix.md)
- [`NiagaraToolsets.NiagaraToolset_System.CreateNiagaraSystem`](capabilities/niagara/toolset/system/create-niagara-system.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetAvailableDynamicInputs`](capabilities/niagara/toolset/system/get/available-dynamic-inputs.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetDataInterfaceSchema`](capabilities/niagara/toolset/system/get/data-interface-schema.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetDynamicInputChain`](capabilities/niagara/toolset/system/get/dynamic/input/chain.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetDynamicInputSchema`](capabilities/niagara/toolset/system/get/dynamic/input/schema.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetDynamicInputSchemaFromAsset`](capabilities/niagara/toolset/system/get/dynamic/input/schema-from-asset.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetEmitterData`](capabilities/niagara/toolset/system/get/emitter/data.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetEmitterInputValues`](capabilities/niagara/toolset/system/get/emitter/input-values.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetEmitterSchema`](capabilities/niagara/toolset/system/get/emitter/schema.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetEmitterSummary`](capabilities/niagara/toolset/system/get/emitter/summary.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetEmitterTopology`](capabilities/niagara/toolset/system/get/emitter/topology.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetModuleInputValues`](capabilities/niagara/toolset/system/get/module/input-values.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetModuleSchema`](capabilities/niagara/toolset/system/get/module/schema.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetModuleSchemaFromAsset`](capabilities/niagara/toolset/system/get/module/schema-from-asset.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetModuleTopology`](capabilities/niagara/toolset/system/get/module/topology.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetRendererData`](capabilities/niagara/toolset/system/get/renderer/data.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetRendererSchema`](capabilities/niagara/toolset/system/get/renderer/schema.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetScriptStackInputValues`](capabilities/niagara/toolset/system/get/script/stack/input-values.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetScriptStackTopology`](capabilities/niagara/toolset/system/get/script/stack/topology.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetStackInputData`](capabilities/niagara/toolset/system/get/stack/input/data.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetStackInputSchema`](capabilities/niagara/toolset/system/get/stack/input/schema.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetStackInputTopology`](capabilities/niagara/toolset/system/get/stack/input/topology.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetStackIssues`](capabilities/niagara/toolset/system/get/stack/issues.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetSystemCompileState`](capabilities/niagara/toolset/system/get/system/compile-state.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetSystemData`](capabilities/niagara/toolset/system/get/system/data.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetSystemDependencies`](capabilities/niagara/toolset/system/get/system/dependencies.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetSystemSchema`](capabilities/niagara/toolset/system/get/system/schema.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetSystemSummary`](capabilities/niagara/toolset/system/get/system/summary.md)
- [`NiagaraToolsets.NiagaraToolset_System.GetUserVariables`](capabilities/niagara/toolset/system/get/user-variables.md)
- [`NiagaraToolsets.NiagaraToolset_System.RemoveEmitter`](capabilities/niagara/toolset/system/remove/emitter.md)
- [`NiagaraToolsets.NiagaraToolset_System.RemoveModule`](capabilities/niagara/toolset/system/remove/module.md)
- [`NiagaraToolsets.NiagaraToolset_System.RemoveRenderer`](capabilities/niagara/toolset/system/remove/renderer.md)
- [`NiagaraToolsets.NiagaraToolset_System.RemoveSetParameterEntry`](capabilities/niagara/toolset/system/remove/set-parameter-entry.md)
- [`NiagaraToolsets.NiagaraToolset_System.RemoveUserVariables`](capabilities/niagara/toolset/system/remove/user-variables.md)
- [`NiagaraToolsets.NiagaraToolset_System.SetEmitterData`](capabilities/niagara/toolset/system/set/emitter-data.md)
- [`NiagaraToolsets.NiagaraToolset_System.SetModuleEnabled`](capabilities/niagara/toolset/system/set/module-enabled.md)
- [`NiagaraToolsets.NiagaraToolset_System.SetRendererData`](capabilities/niagara/toolset/system/set/renderer-data.md)
- [`NiagaraToolsets.NiagaraToolset_System.SetStackInputData`](capabilities/niagara/toolset/system/set/stack-input-data.md)
- [`NiagaraToolsets.NiagaraToolset_System.SetSystemData`](capabilities/niagara/toolset/system/set/system-data.md)

#### `PCGToolset.PCGSpatialToolset`

Capabilities: **1**

- [`PCGToolset.PCGSpatialToolset.RunPCGInstantGraph`](capabilities/pcg/toolset/spatial/run-pcg-instant-graph.md)

#### `PCGToolset.PCGToolset`

Capabilities: **30**

- [`PCGToolset.PCGToolset.AddCommentBox`](capabilities/pcg/toolset/add/comment-box.md)
- [`PCGToolset.PCGToolset.AddNode`](capabilities/pcg/toolset/add/node.md)
- [`PCGToolset.PCGToolset.AddSubgraphNode`](capabilities/pcg/toolset/add/subgraph-node.md)
- [`PCGToolset.PCGToolset.ConnectNodePins`](capabilities/pcg/toolset/connect-node-pins.md)
- [`PCGToolset.PCGToolset.CreateGraph`](capabilities/pcg/toolset/create-graph.md)
- [`PCGToolset.PCGToolset.DisconnectNodePins`](capabilities/pcg/toolset/disconnect-node-pins.md)
- [`PCGToolset.PCGToolset.DrawSpline`](capabilities/pcg/toolset/draw-spline.md)
- [`PCGToolset.PCGToolset.ExecuteGraphInstance`](capabilities/pcg/toolset/execute-graph-instance.md)
- [`PCGToolset.PCGToolset.GetGraphDescription`](capabilities/pcg/toolset/get/graph/description.md)
- [`PCGToolset.PCGToolset.GetGraphInstanceParams`](capabilities/pcg/toolset/get/graph/instance-params.md)
- [`PCGToolset.PCGToolset.GetGraphSchema`](capabilities/pcg/toolset/get/graph/schema.md)
- [`PCGToolset.PCGToolset.GetGraphStructure`](capabilities/pcg/toolset/get/graph/structure.md)
- [`PCGToolset.PCGToolset.GetNativeNodeSchema`](capabilities/pcg/toolset/get/native-node-schema.md)
- [`PCGToolset.PCGToolset.GetNodeDataView`](capabilities/pcg/toolset/get/node/data-view.md)
- [`PCGToolset.PCGToolset.GetNodeInfo`](capabilities/pcg/toolset/get/node/info.md)
- [`PCGToolset.PCGToolset.ListAvailableSubgraphs`](capabilities/pcg/toolset/list/available-subgraphs.md)
- [`PCGToolset.PCGToolset.ListGraphInstances`](capabilities/pcg/toolset/list/graph-instances.md)
- [`PCGToolset.PCGToolset.ListNativeNodes`](capabilities/pcg/toolset/list/native-nodes.md)
- [`PCGToolset.PCGToolset.RemoveCommentBox`](capabilities/pcg/toolset/remove/comment-box.md)
- [`PCGToolset.PCGToolset.RemoveGraphParams`](capabilities/pcg/toolset/remove/graph-params.md)
- [`PCGToolset.PCGToolset.RemoveNode`](capabilities/pcg/toolset/remove/node.md)
- [`PCGToolset.PCGToolset.RepositionNode`](capabilities/pcg/toolset/reposition-node.md)
- [`PCGToolset.PCGToolset.ResetGraphInstanceParams`](capabilities/pcg/toolset/reset-graph-instance-params.md)
- [`PCGToolset.PCGToolset.SetGraphDescription`](capabilities/pcg/toolset/set/graph/description.md)
- [`PCGToolset.PCGToolset.SetGraphInstanceParams`](capabilities/pcg/toolset/set/graph/instance-params.md)
- [`PCGToolset.PCGToolset.SetGraphParams`](capabilities/pcg/toolset/set/graph/params.md)
- [`PCGToolset.PCGToolset.SetNodeComment`](capabilities/pcg/toolset/set/node-comment.md)
- [`PCGToolset.PCGToolset.SpawnGraphInstance`](capabilities/pcg/toolset/spawn-graph-instance.md)
- [`PCGToolset.PCGToolset.UpdateCommentBox`](capabilities/pcg/toolset/update/comment-box.md)
- [`PCGToolset.PCGToolset.UpdateNode`](capabilities/pcg/toolset/update/node.md)

#### `PhysicsToolsets.PhysicsAssetToolset`

Capabilities: **17**

- [`PhysicsToolsets.PhysicsAssetToolset.AddBody`](capabilities/physics/toolset/asset/add/body.md)
- [`PhysicsToolsets.PhysicsAssetToolset.AddConstraint`](capabilities/physics/toolset/asset/add/constraint.md)
- [`PhysicsToolsets.PhysicsAssetToolset.CreateFromMesh`](capabilities/physics/toolset/asset/create-from-mesh.md)
- [`PhysicsToolsets.PhysicsAssetToolset.GetBodyMassScale`](capabilities/physics/toolset/asset/get/body/mass-scale.md)
- [`PhysicsToolsets.PhysicsAssetToolset.GetBodyNames`](capabilities/physics/toolset/asset/get/body/names.md)
- [`PhysicsToolsets.PhysicsAssetToolset.GetBodyPhysicsMode`](capabilities/physics/toolset/asset/get/body/physics-mode.md)
- [`PhysicsToolsets.PhysicsAssetToolset.GetBodyShapes`](capabilities/physics/toolset/asset/get/body/shapes.md)
- [`PhysicsToolsets.PhysicsAssetToolset.GetConstraints`](capabilities/physics/toolset/asset/get/constraints.md)
- [`PhysicsToolsets.PhysicsAssetToolset.RemoveBody`](capabilities/physics/toolset/asset/remove/body.md)
- [`PhysicsToolsets.PhysicsAssetToolset.RemoveConstraint`](capabilities/physics/toolset/asset/remove/constraint.md)
- [`PhysicsToolsets.PhysicsAssetToolset.RemoveShape`](capabilities/physics/toolset/asset/remove/shape.md)
- [`PhysicsToolsets.PhysicsAssetToolset.SetBodyMassScale`](capabilities/physics/toolset/asset/set/body/mass-scale.md)
- [`PhysicsToolsets.PhysicsAssetToolset.SetBodyPhysicsMode`](capabilities/physics/toolset/asset/set/body/physics-mode.md)
- [`PhysicsToolsets.PhysicsAssetToolset.SetBox`](capabilities/physics/toolset/asset/set/box.md)
- [`PhysicsToolsets.PhysicsAssetToolset.SetCapsule`](capabilities/physics/toolset/asset/set/capsule.md)
- [`PhysicsToolsets.PhysicsAssetToolset.SetConstraintLimits`](capabilities/physics/toolset/asset/set/constraint-limits.md)
- [`PhysicsToolsets.PhysicsAssetToolset.SetSphere`](capabilities/physics/toolset/asset/set/sphere.md)

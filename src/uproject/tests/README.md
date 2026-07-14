# Unreal Tooling Test Taxonomy

The suite is organized by behavior boundary instead of generic test type.

- `project_descriptor/` owns `.uproject` identity and plugin policy.
- `project_generation/` owns UnrealBuildTool arguments, discovery, and cache repair.
- `solution_repair/` owns generated Visual Studio solution normalization.
- `repository_policy/` owns tracked configuration, headers, and test layout.

New tests belong in the narrowest owning slice. Root-level `test_*.py` modules are
not part of the accepted layout.

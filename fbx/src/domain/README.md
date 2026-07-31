# FBX domain layer

This directory owns pure scene semantics only.

`domain/domain.rs` is the root domain facade. Each concept folder owns its own
concept facade with the same name as the folder:

```text
domain/domain.rs

domain/mesh/mesh.rs
domain/texture/texture.rs
domain/scene/scene.rs
domain/capability/capability.rs
```

Those facade files declare child modules in dependency order and re-export the
stable public surface for that concept. They must not contain structs, enums,
functions, filesystem access, serde decoding, package-index reading, CLI
parsing,
Blender details, or writer syntax.

Real domain code lives in the sibling files inside the same concept folder. For
example, mesh code lives in `asset.rs`, `primitive_group.rs`, `topology.rs`, and
`translator.rs`; `mesh.rs` only lists and re-exports those modules.

The domain may translate already-resolved evidence into domain objects. That is
still pure domain work. The domain must not resolve the evidence itself.
Evidence
resolution belongs to ports and adapters.

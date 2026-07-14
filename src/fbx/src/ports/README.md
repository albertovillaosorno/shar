# FBX ports layer

Ports are contracts owned by the application boundary. They are not adapters and
they are not domain concepts.

`package_index.rs` is intentionally narrow: it resolves one Phase 3 package id
into stable model package evidence. It never translates meshes. It never decides
FBX topology. It never reads local asset routes. A driven adapter may implement
that port using the generated package-index JSONL.

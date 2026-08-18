// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Adapters composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Adapters composition module.
// - Description:
//   - Implements the declared responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Adapters composition module.

/// Concrete outbound filesystem adapters.
pub mod driven {
    pub use crate::std_filesystem::StdFilesystem;
}

/// Inbound local filesystem composition surfaces.
pub mod driving {
    /// Local filesystem operations backed by the standard provider.
    pub mod local {
        pub use crate::local::{
            canonicalize, create_dir_all, file_len, path_kind, read_bytes,
            read_optional_utf8, read_utf8, regular_files, strict_regular_files,
            write_bytes, write_new_text, write_text,
        };
    }
}

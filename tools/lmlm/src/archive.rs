//! Complete reviewed LSPA-v5 archive parsing boundary used by this tool.
//!
//! This module describes archive structure only. Publication containment and
//! hostile-input policy live in [`super::security`].

mod binary;
mod container;
pub(crate) mod diagnostic;
mod entry;
mod error;
mod layout;
mod name;
mod parser;
mod payload;
mod table;
mod validation;

pub use entry::FileEntry;
pub use error::LmlmError;
pub use parser::parse;
pub use payload::entry_bytes;

#[cfg(test)]
mod tests;

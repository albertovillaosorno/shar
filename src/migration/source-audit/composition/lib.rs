//! Read-only deep source-audit function.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application/mod.rs"]
pub mod application;
#[path = "../domain/mod.rs"]
pub mod domain;

pub use application::{DeepSourceAudit, DeepSourceAuditReport};
pub use domain::SourceAuditError;

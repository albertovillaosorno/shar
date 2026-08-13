//! Algorithm domain module.

pub mod model;

pub(crate) use model::{
    ALGORITHM_SCHEMA, AlgorithmDocument, AuthenticatedMetadata, ProtectedTarget, SourceRecord,
    TargetDescriptor, TargetKind,
};
pub use model::{AlgorithmError, Settings};
